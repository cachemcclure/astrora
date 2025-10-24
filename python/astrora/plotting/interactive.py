"""
Interactive 3D orbit plotting with Plotly.

This module provides poliastro-compatible 3D orbit visualization using Plotly,
enabling interactive rotation, zoom, and exploration of orbital mechanics.
"""

from typing import Optional

import numpy as np
from astropy import units as u

try:
    import plotly.graph_objects as go
    from plotly.subplots import make_subplots

    HAS_PLOTLY = True
except ImportError:
    HAS_PLOTLY = False
    go = None

try:
    from .._core import Epoch
    from ..bodies import Body
    from ..twobody import Orbit
except ImportError:
    # For standalone testing
    Orbit = None
    Body = None
    Epoch = None


class OrbitPlotter3D:
    """
    Interactive 3D orbit plotter using Plotly.

    This class provides a poliastro-compatible API for plotting orbits,
    trajectories, and celestial bodies in 3D with interactive controls.
    Works best in Jupyter notebooks but can also generate HTML files.

    Parameters
    ----------
    dark : bool, optional
        If True, uses dark theme. Default is False.

    Attributes
    ----------
    fig : plotly.graph_objects.Figure
        The Plotly figure object
    attractor : Body
        The central body being used for the plot

    Examples
    --------
    >>> from astrora.plotting import OrbitPlotter3D
    >>> from astrora.twobody import Orbit
    >>> from astrora.bodies import Earth
    >>> import numpy as np
    >>>
    >>> # Create an orbit
    >>> r = np.array([7000e3, 0, 0])  # meters
    >>> v = np.array([0, 0, 7546])
    >>> orbit = Orbit.from_vectors(Earth, r, v)
    >>>
    >>> # Plot it in 3D
    >>> plotter = OrbitPlotter3D()
    >>> plotter.plot(orbit, label="ISS")
    >>> plotter.show()

    Notes
    -----
    Requires plotly to be installed:
        pip install plotly
    or with uv:
        uv pip install plotly
    """

    def __init__(self, dark: bool = False):
        """Initialize the 3D orbit plotter."""
        if not HAS_PLOTLY:
            raise ImportError(
                "Plotly is required for 3D plotting. " "Install it with: pip install plotly"
            )

        self.fig = go.Figure()
        self._attractor: Optional[Body] = None
        self._dark = dark
        self._orbit_count = 0

        # Configure layout
        template = "plotly_dark" if dark else "plotly_white"

        self.fig.update_layout(
            scene=dict(
                xaxis_title="x (km)",
                yaxis_title="y (km)",
                zaxis_title="z (km)",
                aspectmode="data",
                xaxis=dict(
                    showbackground=True,
                    backgroundcolor=(
                        "rgba(230, 230, 230, 0.5)" if not dark else "rgba(50, 50, 50, 0.5)"
                    ),
                ),
                yaxis=dict(
                    showbackground=True,
                    backgroundcolor=(
                        "rgba(230, 230, 230, 0.5)" if not dark else "rgba(50, 50, 50, 0.5)"
                    ),
                ),
                zaxis=dict(
                    showbackground=True,
                    backgroundcolor=(
                        "rgba(230, 230, 230, 0.5)" if not dark else "rgba(50, 50, 50, 0.5)"
                    ),
                ),
            ),
            template=template,
            showlegend=True,
            legend=dict(
                x=0.02,
                y=0.98,
                xanchor="left",
                yanchor="top",
                bgcolor="rgba(255, 255, 255, 0.8)" if not dark else "rgba(0, 0, 0, 0.8)",
            ),
            margin=dict(l=0, r=0, t=30, b=0),
        )

    @property
    def attractor(self) -> Optional[Body]:
        """Get the current attractor (central body)."""
        return self._attractor

    def set_attractor(self, attractor: Body) -> None:
        """
        Set the central attractor body and draw it as a sphere.

        Parameters
        ----------
        attractor : Body
            The central body (e.g., Earth, Sun, Mars)
        """
        self._attractor = attractor

        # Draw attractor as a sphere
        if hasattr(attractor, "R"):
            radius_km = attractor.R / 1000.0

            # Create sphere mesh
            u_sphere = np.linspace(0, 2 * np.pi, 30)
            v_sphere = np.linspace(0, np.pi, 20)
            x_sphere = radius_km * np.outer(np.cos(u_sphere), np.sin(v_sphere))
            y_sphere = radius_km * np.outer(np.sin(u_sphere), np.sin(v_sphere))
            z_sphere = radius_km * np.outer(np.ones(np.size(u_sphere)), np.cos(v_sphere))

            # Determine color based on body
            body_colors = {
                "Sun": "#FDB813",
                "Mercury": "#8C7853",
                "Venus": "#FFC649",
                "Earth": "#4d69bb",
                "Moon": "#999999",
                "Mars": "#cd5c5c",
                "Jupiter": "#c88b3a",
                "Saturn": "#fad5a5",
                "Uranus": "#4fd0e7",
                "Neptune": "#4166f5",
                "Pluto": "#ba8c6e",
            }
            color = body_colors.get(attractor.name, "#3d59ab")

            self.fig.add_trace(
                go.Surface(
                    x=x_sphere,
                    y=y_sphere,
                    z=z_sphere,
                    colorscale=[[0, color], [1, color]],
                    showscale=False,
                    name=attractor.name,
                    hoverinfo="name",
                    lighting=dict(ambient=0.6, diffuse=0.5, specular=0.3),
                    opacity=0.9,
                )
            )

    def plot(
        self,
        orbit: "Orbit",
        *,
        label: Optional[str] = None,
        color: Optional[str] = None,
        trail: bool = False,
        num_points: int = 150,
    ) -> None:
        """
        Plot an orbit in 3D.

        Parameters
        ----------
        orbit : Orbit
            The orbit object to plot
        label : str, optional
            Label for the orbit (appears in legend)
        color : str, optional
            Color for the orbit line. If None, uses default color cycle.
        trail : bool, optional
            If True, creates a fading trail effect. Default is False.
        num_points : int, optional
            Number of points to use for orbit trajectory. Default is 150.

        Examples
        --------
        >>> plotter = OrbitPlotter3D()
        >>> plotter.plot(orbit, label="ISS", color="red")
        >>> plotter.show()
        """
        # Set attractor if not already set
        if self._attractor is None:
            self.set_attractor(orbit.attractor)

        # Generate orbit points
        if orbit.ecc < 1.0:
            # Closed orbit - sample full period
            times = np.linspace(0, orbit.period, num_points)
        else:
            # Open orbit (parabolic/hyperbolic) - sample a range
            if hasattr(orbit, "period") and orbit.period > 0:
                t_range = 3 * orbit.period
            else:
                t_range = 2 * np.pi * np.sqrt(orbit.p**3 / orbit.attractor.mu)
            times = np.linspace(-t_range, t_range, num_points)

        # Sample positions
        positions, _ = orbit.sample(times)

        # Convert to km for display
        if hasattr(positions, "unit"):
            x = positions[:, 0].to(u.km).value
            y = positions[:, 1].to(u.km).value
            z = positions[:, 2].to(u.km).value
        else:
            # Raw values in meters
            x = positions[:, 0] / 1000.0
            y = positions[:, 1] / 1000.0
            z = positions[:, 2] / 1000.0

        # Default colors cycle
        default_colors = [
            "#1f77b4",
            "#ff7f0e",
            "#2ca02c",
            "#d62728",
            "#9467bd",
            "#8c564b",
            "#e377c2",
            "#7f7f7f",
            "#bcbd22",
            "#17becf",
        ]
        line_color = (
            color if color is not None else default_colors[self._orbit_count % len(default_colors)]
        )
        self._orbit_count += 1

        # Plot trajectory
        if trail:
            # Create gradient effect for trail
            segments = 10
            for i in range(segments):
                start = i * (len(x) // segments)
                end = (i + 1) * (len(x) // segments)
                opacity = 0.2 + 0.8 * (i / segments)

                self.fig.add_trace(
                    go.Scatter3d(
                        x=x[start:end],
                        y=y[start:end],
                        z=z[start:end],
                        mode="lines",
                        line=dict(color=line_color, width=3),
                        opacity=opacity,
                        name=label if i == segments - 1 else None,
                        showlegend=(i == segments - 1 and label is not None),
                        hoverinfo="text",
                        text=f'{label or "Orbit"}',
                    )
                )
        else:
            self.fig.add_trace(
                go.Scatter3d(
                    x=x,
                    y=y,
                    z=z,
                    mode="lines",
                    line=dict(color=line_color, width=3),
                    name=label,
                    hoverinfo="text",
                    text=label or "Orbit",
                )
            )

        # Plot current position
        if hasattr(orbit.r, "unit"):
            pos_x = orbit.r[0].to(u.km).value
            pos_y = orbit.r[1].to(u.km).value
            pos_z = orbit.r[2].to(u.km).value
        else:
            pos_x = orbit.r[0] / 1000.0
            pos_y = orbit.r[1] / 1000.0
            pos_z = orbit.r[2] / 1000.0

        self.fig.add_trace(
            go.Scatter3d(
                x=[pos_x],
                y=[pos_y],
                z=[pos_z],
                mode="markers",
                marker=dict(
                    size=8,
                    color=line_color,
                    line=dict(color="white" if not self._dark else "black", width=2),
                ),
                name=f"{label} position" if label else "Current position",
                showlegend=False,
                hoverinfo="text",
                text=f'{label or "Orbit"} - Current position',
            )
        )

    def plot_trajectory(
        self,
        coordinates: np.ndarray,
        *,
        label: Optional[str] = None,
        color: Optional[str] = None,
        trail: bool = False,
    ) -> None:
        """
        Plot a precomputed 3D trajectory.

        Parameters
        ----------
        coordinates : np.ndarray
            Array of position vectors, shape (N, 3) in meters or (N, 3) Quantity
        label : str, optional
            Label for the trajectory
        color : str, optional
            Color for the trajectory line
        trail : bool, optional
            If True, creates a fading trail effect
        """
        if self._attractor is None:
            raise ValueError("Must set attractor before plotting trajectory")

        # Convert to km
        if hasattr(coordinates, "unit"):
            x = coordinates[:, 0].to(u.km).value
            y = coordinates[:, 1].to(u.km).value
            z = coordinates[:, 2].to(u.km).value
        else:
            x = coordinates[:, 0] / 1000.0
            y = coordinates[:, 1] / 1000.0
            z = coordinates[:, 2] / 1000.0

        # Default color
        default_colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728"]
        line_color = (
            color if color is not None else default_colors[self._orbit_count % len(default_colors)]
        )
        self._orbit_count += 1

        # Plot trajectory
        if trail:
            segments = 10
            for i in range(segments):
                start = i * (len(x) // segments)
                end = (i + 1) * (len(x) // segments)
                opacity = 0.2 + 0.8 * (i / segments)

                self.fig.add_trace(
                    go.Scatter3d(
                        x=x[start:end],
                        y=y[start:end],
                        z=z[start:end],
                        mode="lines",
                        line=dict(color=line_color, width=3),
                        opacity=opacity,
                        name=label if i == segments - 1 else None,
                        showlegend=(i == segments - 1 and label is not None),
                    )
                )
        else:
            self.fig.add_trace(
                go.Scatter3d(
                    x=x,
                    y=y,
                    z=z,
                    mode="lines",
                    line=dict(color=line_color, width=3),
                    name=label,
                )
            )

        # Mark final position
        self.fig.add_trace(
            go.Scatter3d(
                x=[x[-1]],
                y=[y[-1]],
                z=[z[-1]],
                mode="markers",
                marker=dict(
                    size=8,
                    color=line_color,
                    line=dict(color="white" if not self._dark else "black", width=2),
                ),
                name=f"{label} final" if label else "Final position",
                showlegend=False,
            )
        )

    def show(self) -> None:
        """
        Display the interactive 3D plot.

        In Jupyter notebooks, this will render inline. Otherwise, it will
        open in a web browser.
        """
        self.fig.show()

    def savefig(self, filename: str, **kwargs) -> None:
        """
        Save the plot to a file.

        Supports multiple formats including HTML, PNG, JPEG, SVG, and PDF.

        Parameters
        ----------
        filename : str
            Output filename. The extension determines the format:
            - .html: Interactive HTML file
            - .png, .jpg, .svg, .pdf: Static image (requires kaleido)
        **kwargs
            Additional arguments passed to plotly's write_html() or write_image()

        Examples
        --------
        >>> plotter.savefig("orbit.html")  # Interactive HTML
        >>> plotter.savefig("orbit.png", width=800, height=600)  # Static image
        """
        import os

        _, ext = os.path.splitext(filename)

        if ext == ".html":
            self.fig.write_html(filename, **kwargs)
        else:
            # For static images, requires kaleido
            try:
                self.fig.write_image(filename, **kwargs)
            except ImportError:
                raise ImportError(
                    f"Saving to {ext} format requires kaleido. "
                    "Install it with: pip install kaleido"
                )
