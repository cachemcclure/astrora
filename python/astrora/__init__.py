"""
Astrora - Rust-backed astrodynamics library

A modern, high-performance orbital mechanics library combining Python's
ease of use with Rust's computational performance.

Astrora is the spiritual successor to poliastro, leveraging the mature Rust
astrodynamics ecosystem (2024-2025) to provide 10-100x performance improvements
while maintaining API compatibility where practical.

Key Features
------------
- **High Performance**: Rust-powered propagators and coordinate transformations
- **Familiar API**: Compatible with poliastro/hapsira workflows
- **Modern Stack**: Built with PyO3, hifitime, nalgebra, and rayon
- **Comprehensive**: Two-body propagation, Lambert solvers, maneuvers, plotting
- **Unit Support**: Full astropy.units and astropy.time integration

Modules
-------
bodies
    Celestial bodies with physical and gravitational properties
    (Sun, Earth, Moon, Mars, Jupiter, etc.)
twobody
    Two-body orbit representation and propagation
time
    High-precision time handling with astropy integration (hifitime backend)
coordinates
    Coordinate frame transformations (ICRS, GCRS, ITRS, TEME)
    with astropy.coordinates integration
units
    Unit conversion utilities for astropy.units integration
maneuver
    Orbital maneuvers (Hohmann, bi-elliptic, Lambert transfers)
plotting
    Orbit visualization (2D static, 3D interactive, animations, ground tracks)
util
    Utility functions (time ranges, vector operations, angle wrapping)

Quick Start
-----------
Create and visualize an orbit:

>>> from astrora import Orbit, bodies, plotting
>>> import numpy as np
>>>
>>> # Define orbit from position and velocity
>>> r = np.array([7000e3, 0, 0])  # 7000 km altitude (meters)
>>> v = np.array([0, 7546, 0])     # Circular velocity (m/s)
>>> orbit = Orbit.from_vectors(bodies.Earth, r, v)
>>>
>>> # Access orbital elements
>>> print(f"Period: {orbit.period / 3600:.2f} hours")
>>> print(f"Eccentricity: {orbit.ecc:.4f}")
>>> print(f"Inclination: {np.rad2deg(orbit.inc):.1f} degrees")
>>>
>>> # Propagate the orbit
>>> from astrora._core import Duration
>>> future_orbit = orbit.propagate(Duration.from_hours(2))
>>>
>>> # Visualize in 3D
>>> plotter = plotting.OrbitPlotter3D()
>>> plotter.plot(orbit, label="ISS-like orbit")
>>> plotter.show()

With astropy units (poliastro-style):

>>> from astropy import units as u
>>> from astropy.time import Time
>>>
>>> # Create orbit with units
>>> r = [6800, 0, 0] * u.km
>>> v = [0, 7.66, 0] * u.km / u.s
>>> epoch = Time('2024-01-01 12:00:00', scale='utc')
>>> orbit = Orbit.from_vectors(bodies.Earth, r, v, epoch)
>>>
>>> # Access properties with units
>>> print(orbit.a.to(u.km))  # Semi-major axis
>>> print(orbit.period.to(u.minute))  # Orbital period

Classical orbital elements:

>>> # Create orbit from Keplerian elements
>>> orbit = Orbit.from_classical(
...     bodies.Earth,
...     a=8000 * u.km,           # Semi-major axis
...     ecc=0.1,                 # Eccentricity
...     inc=28.5 * u.deg,        # Inclination
...     raan=0 * u.deg,          # Right ascension of ascending node
...     argp=90 * u.deg,         # Argument of periapsis
...     nu=0 * u.deg,            # True anomaly
...     epoch=epoch
... )

Orbital maneuvers:

>>> from astrora import Maneuver
>>>
>>> # Hohmann transfer to GEO
>>> maneuver = Maneuver.hohmann(orbit, 42164e3)
>>> print(f"Total Δv: {maneuver.get_total_cost():.1f} m/s")
>>>
>>> # Lambert problem (rendezvous)
>>> orbit1 = Orbit.from_vectors(bodies.Earth, r1, v1, epoch1)
>>> orbit2 = Orbit.from_vectors(bodies.Earth, r2, v2, epoch2)
>>> maneuver = Maneuver.lambert(orbit1, orbit2)

Ground track visualization:

>>> from astrora.plotting import plot_ground_track
>>> plot_ground_track(orbit, duration=orbit.period, dt=30)

Performance
-----------
Astrora provides significant performance improvements over pure Python:

- Numerical propagation: 10-50x faster
- Lambert problem (batch): 50-100x faster with parallel processing
- Coordinate transformations (batch): 20-80x faster
- Overall workflows: 5-10x typical improvement

Best practices for maximum performance:
- Use batch operations when possible
- Minimize Python-Rust boundary crossings
- Leverage parallel Lambert solvers for porkchop plots

See Also
--------
poliastro : Original pure-Python astrodynamics library (archived Oct 2023)
hapsira : Active fork of poliastro with continued development
Nyx-space : Mission-proven Rust astrodynamics toolkit
"""

__version__ = "0.1.0"

# Import Rust core module
try:
    from astrora._core import __version__ as _core_version

    # Verify core module version matches
    if _core_version != __version__:
        import warnings

        warnings.warn(
            f"Version mismatch: Python package is {__version__}, "
            f"but Rust core is {_core_version}",
            RuntimeWarning,
        )
except ImportError:
    import warnings

    warnings.warn(
        "Rust core module not found. Please build the extension with 'maturin develop'",
        ImportWarning,
    )

# High-level API
from astrora import bodies, coordinates, time, twobody, units
from astrora.maneuver import Maneuver
from astrora.twobody import Orbit

__all__ = [
    "__version__",
    "bodies",
    "twobody",
    "time",
    "units",
    "coordinates",
    "Orbit",
    "Maneuver",
]
