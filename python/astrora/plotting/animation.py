"""
Orbit animation helpers for matplotlib and plotly.

This module provides animation capabilities for visualizing orbital motion over time,
going beyond what poliastro/hapsira offered with static plotting.

The module supports:
- 2D matplotlib animations with FuncAnimation
- 3D interactive plotly animations with frame controls
- Multiple orbits animating simultaneously
- Customizable frame rates, durations, and visual styles
- Export to GIF, MP4, HTML, and other formats
"""

from typing import List, Optional, Tuple, Union

import numpy as np

try:
    import matplotlib.animation as animation
    import matplotlib.pyplot as plt
    from matplotlib.axes import Axes
    from matplotlib.patches import Circle

    HAS_MATPLOTLIB = True
except ImportError:
    HAS_MATPLOTLIB = False
    animation = None

try:
    import plotly.graph_objects as go

    HAS_PLOTLY = True
except ImportError:
    HAS_PLOTLY = False
    go = None

try:
    from .._core import Duration, Epoch
    from ..bodies import Body
    from ..twobody import Orbit
except ImportError:
    # For standalone testing
    Orbit = None
    Body = None
    Epoch = None
    Duration = None


def animate_orbit(
    orbit: Union["Orbit", List["Orbit"]],
    duration: Optional[float] = None,
    num_frames: int = 100,
    fps: int = 20,
    trail: bool = True,
    ax: Optional[Axes] = None,
    figsize: Tuple[float, float] = (8, 8),
    dark: bool = False,
    show_time: bool = True,
    save_to: Optional[str] = None,
    **kwargs,
) -> animation.FuncAnimation:
    """
    Create an animated 2D visualization of orbit propagation using matplotlib.

    This function animates one or more orbits over time, showing the satellite
    position moving along its trajectory. The animation can be displayed
    interactively or saved to a file (GIF, MP4, etc.).

    Parameters
    ----------
    orbit : Orbit or list of Orbit
        The orbit(s) to animate. Multiple orbits will be animated simultaneously.
    duration : float, optional
        Total time to simulate in seconds. If None, uses one orbital period
        of the first orbit. For multiple orbits, consider setting this explicitly.
    num_frames : int, optional
        Number of animation frames. More frames = smoother but slower animation.
        Default is 100 frames.
    fps : int, optional
        Frames per second for playback. Default is 20 fps.
    trail : bool, optional
        If True, shows the full orbital trail. If False, only shows current position.
        Default is True.
    ax : matplotlib.axes.Axes, optional
        Matplotlib axes to use. If None, creates new figure and axes.
    figsize : tuple, optional
        Figure size as (width, height) in inches. Default is (8, 8).
    dark : bool, optional
        If True, uses dark theme. Default is False.
    show_time : bool, optional
        If True, displays current simulation time as text annotation.
        Default is True.
    save_to : str, optional
        Filename to save animation. Extension determines format:
        - '.gif' for GIF animation
        - '.mp4' for MP4 video (requires ffmpeg)
        - '.html' for HTML5 video
        If None, animation is displayed but not saved.
    **kwargs
        Additional keyword arguments passed to FuncAnimation.

    Returns
    -------
    matplotlib.animation.FuncAnimation
        The animation object. Must be stored in a variable to prevent
        garbage collection (which would stop the animation).

    Examples
    --------
    Animate a single orbit:

    >>> from astrora.plotting import animate_orbit
    >>> from astrora.twobody import Orbit
    >>> from astrora.bodies import Earth
    >>> import numpy as np
    >>>
    >>> r = np.array([7000e3, 0, 0])
    >>> v = np.array([0, 7546, 0])
    >>> orbit = Orbit.from_vectors(Earth, r, v)
    >>>
    >>> anim = animate_orbit(orbit, num_frames=50, fps=15)
    >>> plt.show()

    Animate multiple orbits and save to GIF:

    >>> orbit1 = Orbit.from_classical(Earth, a=7000e3, ecc=0.01, ...)
    >>> orbit2 = Orbit.from_classical(Earth, a=8000e3, ecc=0.05, ...)
    >>>
    >>> anim = animate_orbit(
    ...     [orbit1, orbit2],
    ...     duration=2*orbit1.period,
    ...     num_frames=100,
    ...     save_to='orbits.gif'
    ... )

    Notes
    -----
    - The animation object MUST be stored in a variable (like `anim = animate_orbit(...)`)
      to prevent garbage collection
    - Saving to MP4 requires ffmpeg: `conda install ffmpeg` or `apt install ffmpeg`
    - Higher `num_frames` values create smoother but larger animations
    - Use `trail=False` for clearer visualization with many orbits

    See Also
    --------
    animate_orbit_3d : Create interactive 3D animations with plotly
    StaticOrbitPlotter.animate : Animate orbits using existing plotter
    """
    if not HAS_MATPLOTLIB:
        raise ImportError(
            "Matplotlib is required for 2D animations. " "Install it with: pip install matplotlib"
        )

    # Normalize to list of orbits
    if not isinstance(orbit, list):
        orbits = [orbit]
    else:
        orbits = orbit

    # Determine duration (default to first orbit's period)
    if duration is None:
        if hasattr(orbits[0].period, "value"):
            duration = orbits[0].period.value  # Extract from Quantity
        else:
            duration = orbits[0].period

    # Generate time points
    times = np.linspace(0, duration, num_frames)

    # Propagate all orbits
    orbit_data = []
    for orb in orbits:
        positions, velocities = orb.sample(times)
        # Convert to km for plotting
        if hasattr(positions, "value"):
            positions = positions.value / 1000  # m to km
        else:
            positions = positions / 1000
        orbit_data.append(positions)

    # Create figure if needed
    if ax is None:
        fig, ax = plt.subplots(figsize=figsize)
    else:
        fig = ax.figure

    ax.set_aspect("equal")
    ax.grid(True, alpha=0.3)
    ax.set_xlabel("x (km)", fontsize=12)
    ax.set_ylabel("y (km)", fontsize=12)
    ax.set_title("Orbit Animation", fontsize=14, fontweight="bold")

    # Dark mode
    if dark:
        fig.patch.set_facecolor("#1a1a1a")
        ax.set_facecolor("#1a1a1a")
        ax.spines["bottom"].set_color("white")
        ax.spines["top"].set_color("white")
        ax.spines["left"].set_color("white")
        ax.spines["right"].set_color("white")
        ax.tick_params(colors="white")
        ax.xaxis.label.set_color("white")
        ax.yaxis.label.set_color("white")
        ax.title.set_color("white")
        text_color = "white"
    else:
        text_color = "black"

    # Plot central body
    attractor = orbits[0].attractor
    if hasattr(attractor, "R"):
        if hasattr(attractor.R, "value"):
            body_radius = attractor.R.value / 1000  # m to km
        else:
            body_radius = attractor.R / 1000
    else:
        body_radius = 6371  # Default Earth radius in km

    body_circle = Circle((0, 0), body_radius, color="#4169E1", zorder=10, label=attractor.name)
    ax.add_patch(body_circle)

    # Set axis limits based on max orbit size
    max_r = max(np.max(np.abs(data)) for data in orbit_data)
    margin = max_r * 0.1
    ax.set_xlim(-max_r - margin, max_r + margin)
    ax.set_ylim(-max_r - margin, max_r + margin)

    # Initialize artists for each orbit
    orbit_artists = []
    colors = plt.cm.tab10(np.linspace(0, 1, len(orbits)))

    for i, (orb, data) in enumerate(zip(orbits, orbit_data)):
        color = colors[i]

        # Trail line (shows full orbit if trail=True)
        if trail:
            (trail_line,) = ax.plot([], [], "-", color=color, alpha=0.6, linewidth=1.5, zorder=5)
        else:
            trail_line = None

        # Current position marker
        (position_marker,) = ax.plot(
            [],
            [],
            "o",
            color=color,
            markersize=8,
            zorder=20,
            label=f"Orbit {i+1}" if len(orbits) > 1 else "Satellite",
        )

        orbit_artists.append({"trail": trail_line, "marker": position_marker, "data": data})

    # Time annotation
    if show_time:
        time_text = ax.text(
            0.02,
            0.98,
            "",
            transform=ax.transAxes,
            fontsize=12,
            verticalalignment="top",
            bbox=dict(boxstyle="round", facecolor="wheat" if not dark else "gray", alpha=0.5),
            color=text_color,
        )
    else:
        time_text = None

    ax.legend(loc="upper right")

    def init():
        """Initialize animation."""
        artists = []
        for art in orbit_artists:
            if art["trail"] is not None:
                art["trail"].set_data([], [])
                artists.append(art["trail"])
            art["marker"].set_data([], [])
            artists.append(art["marker"])
        if time_text is not None:
            time_text.set_text("")
            artists.append(time_text)
        return artists

    def animate_frame(frame):
        """Update animation for given frame."""
        artists = []

        for art in orbit_artists:
            data = art["data"]

            # Update trail (show path up to current frame)
            if art["trail"] is not None:
                art["trail"].set_data(data[: frame + 1, 0], data[: frame + 1, 1])
                artists.append(art["trail"])

            # Update position marker
            art["marker"].set_data([data[frame, 0]], [data[frame, 1]])
            artists.append(art["marker"])

        # Update time display
        if time_text is not None:
            current_time = times[frame]
            hours = current_time / 3600
            time_text.set_text(f"Time: {hours:.2f} hours")
            artists.append(time_text)

        return artists

    # Create animation
    anim = animation.FuncAnimation(
        fig,
        animate_frame,
        init_func=init,
        frames=num_frames,
        interval=1000 / fps,
        blit=True,
        **kwargs,
    )

    # Save if requested
    if save_to is not None:
        print(f"Saving animation to {save_to}...")
        if save_to.endswith(".gif"):
            anim.save(save_to, writer="pillow", fps=fps)
        elif save_to.endswith(".mp4"):
            anim.save(save_to, writer="ffmpeg", fps=fps)
        elif save_to.endswith(".html"):
            anim.save(save_to, writer="html", fps=fps)
        else:
            anim.save(save_to, fps=fps)
        print(f"Animation saved successfully!")

    return anim


def animate_orbit_3d(
    orbit: Union["Orbit", List["Orbit"]],
    duration: Optional[float] = None,
    num_frames: int = 100,
    fps: int = 20,
    trail: bool = True,
    dark: bool = False,
    show_time: bool = True,
    save_to: Optional[str] = None,
    include_plotlyjs: str = "cdn",
    **kwargs,
) -> go.Figure:
    """
    Create an interactive 3D animation of orbit propagation using plotly.

    This function creates an animated 3D visualization with interactive controls
    (play/pause, slider, rotation, zoom). The animation can be displayed in
    Jupyter notebooks or saved to an HTML file.

    Parameters
    ----------
    orbit : Orbit or list of Orbit
        The orbit(s) to animate. Multiple orbits will be animated simultaneously.
    duration : float, optional
        Total time to simulate in seconds. If None, uses one orbital period
        of the first orbit.
    num_frames : int, optional
        Number of animation frames. Default is 100.
    fps : int, optional
        Frames per second for playback. Default is 20 fps.
    trail : bool, optional
        If True, shows the full orbital trail in addition to current position.
        Default is True.
    dark : bool, optional
        If True, uses dark theme. Default is False.
    show_time : bool, optional
        If True, displays current simulation time in the title.
        Default is True.
    save_to : str, optional
        Filename to save the interactive HTML animation. If None, animation
        is displayed but not saved. Use '.html' extension.
    include_plotlyjs : str, optional
        How to include plotly.js in saved HTML files. Options:
        - 'cdn' (default): Use CDN link (~3MB smaller, requires internet)
        - True: Embed full library (~3MB larger, works offline)
        - 'directory': Reference external plotly.min.js file
        - False: Don't include (for embedding in existing pages)
        Only used when save_to is specified. Default is 'cdn'.
    **kwargs
        Additional keyword arguments passed to plotly Figure.

    Returns
    -------
    plotly.graph_objects.Figure
        The interactive plotly figure with animation frames.

    Examples
    --------
    Animate a single orbit interactively:

    >>> from astrora.plotting import animate_orbit_3d
    >>> from astrora.twobody import Orbit
    >>> from astrora.bodies import Earth
    >>> import numpy as np
    >>>
    >>> r = np.array([7000e3, 0, 0])
    >>> v = np.array([0, 0, 7546])
    >>> orbit = Orbit.from_vectors(Earth, r, v)
    >>>
    >>> fig = animate_orbit_3d(orbit, num_frames=50)
    >>> fig.show()

    Animate and save to HTML:

    >>> fig = animate_orbit_3d(
    ...     orbit,
    ...     num_frames=100,
    ...     dark=True,
    ...     save_to='orbit_animation.html'
    ... )

    Animate multiple orbits:

    >>> orbit1 = Orbit.from_classical(Earth, a=7000e3, ecc=0.01, ...)
    >>> orbit2 = Orbit.from_classical(Earth, a=8000e3, ecc=0.05, ...)
    >>> fig = animate_orbit_3d([orbit1, orbit2], duration=2*orbit1.period)

    Notes
    -----
    - The resulting animation is interactive: rotate, zoom, pan with mouse
    - Play/pause button and time slider are automatically included
    - Best viewed in Jupyter notebooks or as standalone HTML files
    - Default uses CDN for plotly.js (~3MB smaller files, requires internet)
    - Use include_plotlyjs=True for offline viewing (larger files)
    - File sizes can be large for many frames; consider reducing num_frames

    See Also
    --------
    animate_orbit : Create 2D animations with matplotlib
    OrbitPlotter3D.animate : Animate orbits using existing 3D plotter
    """
    if not HAS_PLOTLY:
        raise ImportError(
            "Plotly is required for 3D animations. " "Install it with: pip install plotly"
        )

    # Normalize to list of orbits
    if not isinstance(orbit, list):
        orbits = [orbit]
    else:
        orbits = orbit

    # Determine duration
    if duration is None:
        if hasattr(orbits[0].period, "value"):
            duration = orbits[0].period.value
        else:
            duration = orbits[0].period

    # Generate time points
    times = np.linspace(0, duration, num_frames)

    # Propagate all orbits
    orbit_data = []
    for orb in orbits:
        positions, velocities = orb.sample(times)
        # Convert to km
        if hasattr(positions, "value"):
            positions = positions.value / 1000
        else:
            positions = positions / 1000
        orbit_data.append(positions)

    # Create frames
    frames = []
    colors = ["#FF6B6B", "#4ECDC4", "#45B7D1", "#FFA07A", "#98D8C8", "#F7DC6F"]

    for frame_idx in range(num_frames):
        frame_data = []

        # Add central body (Earth sphere)
        attractor = orbits[0].attractor
        if hasattr(attractor, "R"):
            if hasattr(attractor.R, "value"):
                body_radius = attractor.R.value / 1000
            else:
                body_radius = attractor.R / 1000
        else:
            body_radius = 6371

        # Create sphere for attractor
        u = np.linspace(0, 2 * np.pi, 30)
        v = np.linspace(0, np.pi, 20)
        x_sphere = body_radius * np.outer(np.cos(u), np.sin(v))
        y_sphere = body_radius * np.outer(np.sin(u), np.sin(v))
        z_sphere = body_radius * np.outer(np.ones(np.size(u)), np.cos(v))

        frame_data.append(
            go.Surface(
                x=x_sphere,
                y=y_sphere,
                z=z_sphere,
                colorscale="Blues",
                showscale=False,
                opacity=0.7,
                name=attractor.name,
            )
        )

        # Add each orbit
        for orbit_idx, data in enumerate(orbit_data):
            color = colors[orbit_idx % len(colors)]

            # Full orbital trail (if requested)
            if trail:
                frame_data.append(
                    go.Scatter3d(
                        x=data[:, 0],
                        y=data[:, 1],
                        z=data[:, 2],
                        mode="lines",
                        line=dict(color=color, width=2),
                        opacity=0.5,
                        name=f"Orbit {orbit_idx + 1} trail",
                    )
                )

            # Current position marker
            frame_data.append(
                go.Scatter3d(
                    x=[data[frame_idx, 0]],
                    y=[data[frame_idx, 1]],
                    z=[data[frame_idx, 2]],
                    mode="markers",
                    marker=dict(size=8, color=color),
                    name=f"Satellite {orbit_idx + 1}",
                )
            )

        frames.append(go.Frame(data=frame_data, name=str(frame_idx)))

    # Create initial figure with first frame
    fig = go.Figure(data=frames[0].data, frames=frames)

    # Configure layout
    template = "plotly_dark" if dark else "plotly_white"

    # Calculate axis range
    max_r = max(np.max(np.abs(data)) for data in orbit_data)
    margin = max_r * 0.1
    axis_range = [-max_r - margin, max_r + margin]

    title_text = "Orbit Animation"
    if show_time:
        title_text += " (Time: 0.00 hours)"

    fig.update_layout(
        template=template,
        title=title_text,
        scene=dict(
            xaxis=dict(title="x (km)", range=axis_range),
            yaxis=dict(title="y (km)", range=axis_range),
            zaxis=dict(title="z (km)", range=axis_range),
            aspectmode="cube",
        ),
        updatemenus=[
            {
                "type": "buttons",
                "showactive": False,
                "buttons": [
                    {
                        "label": "Play",
                        "method": "animate",
                        "args": [
                            None,
                            {
                                "frame": {"duration": 1000 / fps, "redraw": True},
                                "fromcurrent": True,
                                "mode": "immediate",
                                "transition": {"duration": 0},
                            },
                        ],
                    },
                    {
                        "label": "Pause",
                        "method": "animate",
                        "args": [
                            [None],
                            {
                                "frame": {"duration": 0, "redraw": False},
                                "mode": "immediate",
                                "transition": {"duration": 0},
                            },
                        ],
                    },
                ],
                "x": 0.1,
                "y": 0,
            }
        ],
        sliders=[
            {
                "active": 0,
                "steps": [
                    {
                        "args": [
                            [f.name],
                            {
                                "frame": {"duration": 0, "redraw": True},
                                "mode": "immediate",
                                "transition": {"duration": 0},
                            },
                        ],
                        "label": f"{times[int(f.name)]/3600:.2f}h" if show_time else str(i),
                        "method": "animate",
                    }
                    for i, f in enumerate(frames)
                ],
                "x": 0.1,
                "len": 0.9,
                "xanchor": "left",
                "y": 0,
                "yanchor": "top",
            }
        ],
    )

    # Save if requested
    if save_to is not None:
        print(f"Saving animation to {save_to}...")
        # Use specified include_plotlyjs option (default: 'cdn' for smaller files)
        fig.write_html(save_to, include_plotlyjs=include_plotlyjs)
        if include_plotlyjs == "cdn":
            print(f"Animation saved successfully! (Using CDN - requires internet to view)")
        elif include_plotlyjs is True:
            print(f"Animation saved successfully! (Standalone - works offline)")
        else:
            print(f"Animation saved successfully!")

    return fig


__all__ = [
    "animate_orbit",
    "animate_orbit_3d",
]
