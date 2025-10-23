"""
Astrora - Rust-backed astrodynamics library

A modern, high-performance orbital mechanics library combining Python's
ease of use with Rust's computational performance.
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
            RuntimeWarning
        )
except ImportError as e:
    import warnings
    warnings.warn(
        "Rust core module not found. Please build the extension with 'maturin develop'",
        ImportWarning
    )

# High-level API
from astrora import bodies
from astrora import twobody
from astrora import time
from astrora import units
from astrora import coordinates
from astrora.twobody import Orbit
from astrora.maneuver import Maneuver

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
