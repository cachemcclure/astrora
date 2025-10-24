"""
Ground track plotting for satellite orbits.

This module provides functions for visualizing satellite ground tracks
(sub-satellite points) on Earth's surface.
"""

from typing import Optional

import matplotlib.pyplot as plt
import numpy as np
from astropy import units as u
from matplotlib.axes import Axes

try:
    from .._core import compute_ground_track, ecef_to_geodetic
    from ..twobody import Orbit
except ImportError:
    Orbit = None
    ecef_to_geodetic = None
    compute_ground_track = None


def plot_ground_track(
    orbit: "Orbit",
    duration: float,
    dt: float = 60.0,
    ax: Optional[Axes] = None,
    show_map: bool = True,
    color: Optional[str] = None,
    label: Optional[str] = None,
    **kwargs,
) -> Axes:
    """
    Plot satellite ground track on a 2D map projection.

    The ground track shows the sub-satellite point (nadir point) as the
    satellite orbits. This is useful for understanding coverage, visibility,
    and orbit characteristics.

    Parameters
    ----------
    orbit : Orbit
        The orbit to plot
    duration : float
        Duration to plot in seconds
    dt : float, optional
        Time step in seconds. Default is 60 seconds.
    ax : matplotlib.axes.Axes, optional
        Axes to plot on. If None, creates new figure.
    show_map : bool, optional
        If True, shows Earth map background. Default is True.
    color : str, optional
        Color for the ground track line
    label : str, optional
        Label for the ground track (for legend)
    **kwargs
        Additional arguments passed to plot()

    Returns
    -------
    ax : matplotlib.axes.Axes
        The axes containing the plot

    Examples
    --------
    >>> from astrora.twobody import Orbit
    >>> from astrora.bodies import Earth
    >>> from astrora.plotting import plot_ground_track
    >>> import numpy as np
    >>>
    >>> # ISS orbit
    >>> r = np.array([6800e3, 0, 0])
    >>> v = np.array([0, 7.546e3, 0])
    >>> orbit = Orbit.from_vectors(Earth, r, v)
    >>>
    >>> # Plot one orbit
    >>> plot_ground_track(orbit, duration=orbit.period, dt=30)
    >>> plt.show()

    Notes
    -----
    Ground tracks depend on:
    - Orbital inclination: Determines latitude coverage
    - Orbital period vs. Earth rotation: Determines track shift
    - Eccentricity: Affects ground speed variations

    For polar orbits (inc ~ 90°), ground tracks cover all latitudes.
    For equatorial orbits (inc ~ 0°), ground tracks stay near the equator.
    For sun-synchronous orbits, ground tracks repeat daily.

    References
    ----------
    - Vallado, "Fundamentals of Astrodynamics" (2013), Ch. 11
    - Wertz & Larson, "Space Mission Analysis and Design" (1999), Ch. 5
    """
    if ecef_to_geodetic is None:
        raise ImportError(
            "Ground track functions not available. " "Make sure astrora._core is properly compiled."
        )

    if ax is None:
        fig, ax = plt.subplots(figsize=(14, 8))

    # Generate time points
    # Handle Quantity objects
    if hasattr(duration, "unit"):
        duration_seconds = duration.to(u.s).value
    else:
        duration_seconds = duration

    times = np.arange(0, duration_seconds, dt)

    # Sample orbit positions (GCRF/inertial frame)
    positions, _ = orbit.sample(times)

    # Convert to km
    if hasattr(positions, "unit"):
        positions_km = positions.to(u.km).value
    else:
        positions_km = positions / 1000.0

    # Note: Full implementation requires GCRF -> ITRF (ECEF) transformation
    # For now, we approximate using inertial positions
    # TODO: Add proper coordinate frame transformation using ITRF implementation

    # Convert ECEF positions to geodetic coordinates
    latitudes = []
    longitudes = []

    for pos in positions_km:
        # Direct conversion (approximation - assumes ECEF ≈ GCRF for short durations)
        result = ecef_to_geodetic(pos * 1000.0)  # Convert back to meters
        latitudes.append(result["latitude_deg"])
        longitudes.append(result["longitude_deg"])

    latitudes = np.array(latitudes)
    longitudes = np.array(longitudes)

    # Handle longitude wrapping at ±180°
    # Split track when longitude jumps > 180° (date line crossing)
    lon_diff = np.diff(longitudes)
    breaks = np.where(np.abs(lon_diff) > 180)[0]

    # Plot segments separately to avoid lines across the map
    plot_kwargs = {}
    if color is not None:
        plot_kwargs["color"] = color
    if label is not None:
        plot_kwargs["label"] = label
    plot_kwargs.update(kwargs)

    if len(breaks) == 0:
        # No date line crossings
        ax.plot(longitudes, latitudes, **plot_kwargs)
    else:
        # Plot segments between breaks
        breaks = np.concatenate(([0], breaks + 1, [len(longitudes)]))
        for i in range(len(breaks) - 1):
            start, end = breaks[i], breaks[i + 1]
            segment_kwargs = plot_kwargs.copy()
            # Only label the first segment
            if i > 0:
                segment_kwargs.pop("label", None)
            ax.plot(longitudes[start:end], latitudes[start:end], **segment_kwargs)

    # Mark start and end points
    ax.plot(longitudes[0], latitudes[0], "go", markersize=8, label="Start", zorder=10)
    ax.plot(longitudes[-1], latitudes[-1], "ro", markersize=8, label="End", zorder=10)

    # Configure axes
    ax.set_xlim(-180, 180)
    ax.set_ylim(-90, 90)
    ax.set_xlabel("Longitude (°)", fontsize=12)
    ax.set_ylabel("Latitude (°)", fontsize=12)
    ax.set_title("Satellite Ground Track", fontsize=14, fontweight="bold")
    ax.grid(True, alpha=0.3)
    ax.set_aspect("equal", adjustable="box")

    # Add map background if requested
    if show_map:
        try:
            # Try to use cartopy for nice map background
            import cartopy.crs as ccrs
            import cartopy.feature as cfeature

            # Note: This requires the axes to be created with cartopy projection
            # For now, just add gridlines
            ax.set_xticks(np.arange(-180, 181, 30))
            ax.set_yticks(np.arange(-90, 91, 30))
            ax.axhline(y=0, color="k", linestyle="--", alpha=0.3, linewidth=0.5)
            ax.axvline(x=0, color="k", linestyle="--", alpha=0.3, linewidth=0.5)

        except ImportError:
            # Fallback to simple gridlines
            ax.set_xticks(np.arange(-180, 181, 30))
            ax.set_yticks(np.arange(-90, 91, 30))
            ax.axhline(y=0, color="k", linestyle="--", alpha=0.3, linewidth=0.5)
            ax.axvline(x=0, color="k", linestyle="--", alpha=0.3, linewidth=0.5)

    ax.legend(loc="upper right")
    plt.tight_layout()

    return ax


def plot_ground_track_3d(
    orbit: "Orbit", duration: float, dt: float = 60.0, show_earth: bool = True, **kwargs
) -> None:
    """
    Plot satellite ground track on a 3D Earth globe.

    This creates an interactive 3D visualization showing the satellite
    path over Earth's surface.

    Parameters
    ----------
    orbit : Orbit
        The orbit to plot
    duration : float
        Duration to plot in seconds
    dt : float, optional
        Time step in seconds. Default is 60 seconds.
    show_earth : bool, optional
        If True, shows Earth sphere. Default is True.
    **kwargs
        Additional arguments for customization

    Notes
    -----
    Requires plotly for 3D visualization.

    Examples
    --------
    >>> plot_ground_track_3d(orbit, duration=orbit.period)
    """
    try:
        import plotly.graph_objects as go
    except ImportError:
        raise ImportError(
            "Plotly is required for 3D ground tracks. " "Install with: pip install plotly"
        )

    # Generate time points
    # Handle Quantity objects
    if hasattr(duration, "unit"):
        duration_seconds = duration.to(u.s).value
    else:
        duration_seconds = duration

    times = np.arange(0, duration_seconds, dt)

    # Sample orbit positions
    positions, _ = orbit.sample(times)

    # Convert to km
    if hasattr(positions, "unit"):
        positions_km = positions.to(u.km).value
    else:
        positions_km = positions / 1000.0

    # Create figure
    fig = go.Figure()

    # Add Earth sphere if requested
    if show_earth:
        R_earth = 6378.137  # km

        # Create sphere
        theta = np.linspace(0, 2 * np.pi, 50)
        phi = np.linspace(0, np.pi, 50)
        x = R_earth * np.outer(np.cos(theta), np.sin(phi))
        y = R_earth * np.outer(np.sin(theta), np.sin(phi))
        z = R_earth * np.outer(np.ones(np.size(theta)), np.cos(phi))

        fig.add_trace(
            go.Surface(
                x=x,
                y=y,
                z=z,
                colorscale=[[0, "#4d69bb"], [1, "#4d69bb"]],
                showscale=False,
                name="Earth",
                hoverinfo="name",
                opacity=0.9,
            )
        )

    # Add orbit path
    fig.add_trace(
        go.Scatter3d(
            x=positions_km[:, 0],
            y=positions_km[:, 1],
            z=positions_km[:, 2],
            mode="lines",
            line=dict(color="red", width=3),
            name="Orbit",
        )
    )

    # Add start/end markers
    fig.add_trace(
        go.Scatter3d(
            x=[positions_km[0, 0]],
            y=[positions_km[0, 1]],
            z=[positions_km[0, 2]],
            mode="markers",
            marker=dict(size=8, color="green"),
            name="Start",
        )
    )

    fig.add_trace(
        go.Scatter3d(
            x=[positions_km[-1, 0]],
            y=[positions_km[-1, 1]],
            z=[positions_km[-1, 2]],
            mode="markers",
            marker=dict(size=8, color="red"),
            name="End",
        )
    )

    # Configure layout
    fig.update_layout(
        scene=dict(
            xaxis_title="x (km)",
            yaxis_title="y (km)",
            zaxis_title="z (km)",
            aspectmode="data",
        ),
        title="3D Ground Track Visualization",
        showlegend=True,
    )

    fig.show()
