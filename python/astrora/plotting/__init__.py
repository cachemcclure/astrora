"""
Plotting and visualization module

Tools for visualizing orbits, trajectories, and mission design plots.

This module provides poliastro-compatible plotting classes and functions:

- **StaticOrbitPlotter**: 2D matplotlib-based orbit plotting
- **OrbitPlotter3D**: 3D interactive Plotly-based visualization
- **plot_porkchop**: Launch window analysis with porkchop plots
- **plot_ground_track**: Satellite ground track visualization
- **animate_orbit**: 2D orbit propagation animations (matplotlib)
- **animate_orbit_3d**: 3D interactive orbit animations (plotly)

Examples
--------
2D static plotting:
>>> from astrora.plotting import StaticOrbitPlotter
>>> from astrora.twobody import Orbit
>>> from astrora.bodies import Earth
>>> plotter = StaticOrbitPlotter()
>>> plotter.plot(orbit, label="ISS")
>>> plotter.show()

3D interactive plotting:
>>> from astrora.plotting import OrbitPlotter3D
>>> plotter = OrbitPlotter3D()
>>> plotter.plot(orbit, label="ISS")
>>> plotter.show()

Ground track plotting:
>>> from astrora.plotting import plot_ground_track
>>> plot_ground_track(orbit, duration=orbit.period)

Orbit animation:
>>> from astrora.plotting import animate_orbit
>>> anim = animate_orbit(orbit, num_frames=100, fps=20)
>>> plt.show()
"""

from .animation import animate_orbit, animate_orbit_3d
from .groundtrack import plot_ground_track, plot_ground_track_3d
from .interactive import OrbitPlotter3D
from .porkchop import plot_porkchop, plot_porkchop_simple
from .static import StaticOrbitPlotter

__all__ = [
    "StaticOrbitPlotter",
    "OrbitPlotter3D",
    "plot_porkchop",
    "plot_porkchop_simple",
    "plot_ground_track",
    "plot_ground_track_3d",
    "animate_orbit",
    "animate_orbit_3d",
]
