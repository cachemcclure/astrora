"""
Astropy Coordinates Integration

This module provides seamless integration between astrora's Rust-backed coordinate
frames and astropy.coordinates. It enables bidirectional conversion between:

- astrora frames (ICRS, GCRS, ITRS, TEME) ↔ astropy coordinate frames
- Support for SkyCoord for convenient coordinate manipulation
- Preservation of position, velocity, and time information

Examples
--------
Convert astrora frame to astropy:

    >>> from astrora._core import GCRS
    >>> from astrora.coordinates import to_astropy_coord
    >>> from astropy.time import Time
    >>> import numpy as np
    >>>
    >>> # Create astrora GCRS frame
    >>> pos = np.array([7000e3, 0.0, 0.0])  # 7000 km altitude
    >>> vel = np.array([0.0, 7500.0, 0.0])  # ~7.5 km/s
    >>> frame = GCRS(pos, vel)
    >>>
    >>> # Convert to astropy GCRS
    >>> obstime = Time('2024-01-01 12:00:00')
    >>> astropy_gcrs = to_astropy_coord(frame, obstime=obstime)

Convert astropy coordinate to astrora:

    >>> from astropy.coordinates import GCRS as AstropyGCRS
    >>> from astropy import units as u
    >>> from astrora.coordinates import from_astropy_coord
    >>>
    >>> # Create astropy GCRS
    >>> astropy_gcrs = AstropyGCRS(
    ...     x=7000*u.km, y=0*u.km, z=0*u.km,
    ...     v_x=0*u.km/u.s, v_y=7.5*u.km/u.s, v_z=0*u.km/u.s,
    ...     representation_type='cartesian',
    ...     differential_type='cartesian',
    ...     obstime=Time('2024-01-01 12:00:00')
    ... )
    >>>
    >>> # Convert to astrora GCRS
    >>> astrora_gcrs = from_astropy_coord(astropy_gcrs)

Use SkyCoord for high-level operations:

    >>> from astrora.coordinates import to_skycoord
    >>> from astrora._core import ICRS
    >>>
    >>> # Create astrora ICRS frame
    >>> pos = np.array([1.496e11, 0.0, 0.0])  # 1 AU
    >>> vel = np.array([0.0, 29780.0, 0.0])   # Earth's orbital velocity
    >>> frame = ICRS(pos, vel)
    >>>
    >>> # Convert to SkyCoord
    >>> sc = to_skycoord(frame)
    >>> print(sc.distance)  # Distance from origin
    >>> print(sc.ra, sc.dec)  # Sky position
"""

from typing import Union

import numpy as np

try:
    from astropy import units as u
    from astropy.coordinates import (
        GCRS as AstropyGCRS,
    )
    from astropy.coordinates import (
        ICRS as AstropyICRS,
    )
    from astropy.coordinates import (
        ITRS as AstropyITRS,
    )
    from astropy.coordinates import (
        SkyCoord,
    )
    from astropy.time import Time

    ASTROPY_AVAILABLE = True
except ImportError:
    ASTROPY_AVAILABLE = False

    # Provide helpful error messages
    class _AstropyNotAvailable:
        def __init__(self, *args, **kwargs):
            raise ImportError(
                "astropy is required for coordinate integration. "
                "Install it with: pip install astropy"
            )

    SkyCoord = _AstropyNotAvailable
    AstropyICRS = _AstropyNotAvailable
    AstropyGCRS = _AstropyNotAvailable
    AstropyITRS = _AstropyNotAvailable
    Time = _AstropyNotAvailable
    u = None

from astrora._core import (
    GCRS as AstroraGCRS,
)
from astrora._core import (
    ICRS as AstroraICRS,
)
from astrora._core import (
    ITRS as AstroraITRS,
)
from astrora._core import (
    TEME as AstroraTEME,
)


def _check_astropy():
    """Check if astropy is available and raise helpful error if not."""
    if not ASTROPY_AVAILABLE:
        raise ImportError(
            "astropy is required for coordinate integration. "
            "Install it with: pip install astropy"
        )


def to_astropy_coord(
    frame: Union[AstroraICRS, AstroraGCRS, AstroraITRS, AstroraTEME],
    obstime: Union[Time, None] = None,
):
    """
    Convert astrora coordinate frame to astropy coordinate frame.

    Parameters
    ----------
    frame : ICRS, GCRS, ITRS, or TEME
        Astrora coordinate frame object with position and velocity.
    obstime : astropy.time.Time, optional
        Observation time for the coordinate. Required for GCRS and ITRS frames.
        If not provided, uses J2000.0 epoch.

    Returns
    -------
    astropy.coordinates frame
        Corresponding astropy coordinate frame (ICRS, GCRS, or ITRS) with
        position and velocity information.

    Notes
    -----
    - Positions are converted from meters to kilometers (astropy convention)
    - Velocities are converted from m/s to km/s
    - TEME frame is converted to GCRS (approximate, for astropy compatibility)
    - All conversions preserve full precision

    Examples
    --------
    >>> from astrora._core import GCRS
    >>> from astropy.time import Time
    >>> import numpy as np
    >>>
    >>> pos = np.array([7000e3, 0.0, 0.0])  # meters
    >>> vel = np.array([0.0, 7500.0, 0.0])  # m/s
    >>> frame = GCRS(pos, vel)
    >>>
    >>> obstime = Time('2024-01-01 12:00:00')
    >>> astropy_frame = to_astropy_coord(frame, obstime=obstime)
    >>> print(astropy_frame.cartesian.xyz)  # km
    """
    _check_astropy()

    # Get position and velocity from astrora frame
    position = frame.position  # meters
    velocity = frame.velocity  # m/s

    # Convert to astropy units (km and km/s)
    x, y, z = position[0] / 1000.0, position[1] / 1000.0, position[2] / 1000.0
    vx, vy, vz = velocity[0] / 1000.0, velocity[1] / 1000.0, velocity[2] / 1000.0

    # Set default obstime to J2000.0 if not provided
    if obstime is None:
        obstime = Time("J2000.0", scale="tt")

    # Determine frame type and create corresponding astropy frame
    if isinstance(frame, AstroraICRS):
        # ICRS doesn't require obstime but we include it for consistency
        return AstropyICRS(
            x=x * u.km,
            y=y * u.km,
            z=z * u.km,
            v_x=vx * u.km / u.s,
            v_y=vy * u.km / u.s,
            v_z=vz * u.km / u.s,
            representation_type="cartesian",
            differential_type="cartesian",
        )

    elif isinstance(frame, AstroraGCRS):
        return AstropyGCRS(
            x=x * u.km,
            y=y * u.km,
            z=z * u.km,
            v_x=vx * u.km / u.s,
            v_y=vy * u.km / u.s,
            v_z=vz * u.km / u.s,
            representation_type="cartesian",
            differential_type="cartesian",
            obstime=obstime,
        )

    elif isinstance(frame, AstroraITRS):
        return AstropyITRS(
            x=x * u.km,
            y=y * u.km,
            z=z * u.km,
            v_x=vx * u.km / u.s,
            v_y=vy * u.km / u.s,
            v_z=vz * u.km / u.s,
            representation_type="cartesian",
            differential_type="cartesian",
            obstime=obstime,
        )

    elif isinstance(frame, AstroraTEME):
        # TEME is not a standard astropy frame
        # Convert to GCRS as approximation (user should do proper conversion)
        # Note: For precise work, user should convert TEME → GCRS in astrora first
        import warnings

        warnings.warn(
            "TEME frame is not natively supported in astropy. "
            "Converting to GCRS as approximation. For precise transformations, "
            "use astrora's TEME.to_gcrs() method before converting to astropy.",
            UserWarning,
        )
        return AstropyGCRS(
            x=x * u.km,
            y=y * u.km,
            z=z * u.km,
            v_x=vx * u.km / u.s,
            v_y=vy * u.km / u.s,
            v_z=vz * u.km / u.s,
            representation_type="cartesian",
            differential_type="cartesian",
            obstime=obstime,
        )

    else:
        raise TypeError(f"Unsupported frame type: {type(frame)}")


def from_astropy_coord(astropy_frame) -> Union[AstroraICRS, AstroraGCRS, AstroraITRS]:
    """
    Convert astropy coordinate frame to astrora coordinate frame.

    Parameters
    ----------
    astropy_frame : astropy.coordinates frame
        Astropy coordinate frame (ICRS, GCRS, or ITRS) with Cartesian
        representation and velocity information.

    Returns
    -------
    ICRS, GCRS, or ITRS
        Corresponding astrora coordinate frame with position (meters) and
        velocity (m/s).

    Notes
    -----
    - Positions are converted from kilometers to meters
    - Velocities are converted from km/s to m/s
    - Requires Cartesian representation with velocity differential
    - Frame type is automatically detected from astropy frame

    Examples
    --------
    >>> from astropy.coordinates import GCRS as AstropyGCRS
    >>> from astropy import units as u
    >>> from astropy.time import Time
    >>>
    >>> astropy_gcrs = AstropyGCRS(
    ...     x=7000*u.km, y=0*u.km, z=0*u.km,
    ...     v_x=0*u.km/u.s, v_y=7.5*u.km/u.s, v_z=0*u.km/u.s,
    ...     representation_type='cartesian',
    ...     differential_type='cartesian',
    ...     obstime=Time('2024-01-01')
    ... )
    >>>
    >>> astrora_frame = from_astropy_coord(astropy_gcrs)
    >>> print(astrora_frame.position)  # meters
    """
    _check_astropy()

    # Get Cartesian representation
    cartesian = astropy_frame.cartesian

    # Extract position (convert km → meters)
    x = cartesian.x.to(u.km).value * 1000.0
    y = cartesian.y.to(u.km).value * 1000.0
    z = cartesian.z.to(u.km).value * 1000.0
    position = np.array([x, y, z])

    # Extract velocity (convert km/s → m/s)
    # Check if velocity is available
    if cartesian.differentials:
        velocity_data = cartesian.differentials["s"]  # 's' is the standard key for velocity
        vx = velocity_data.d_x.to(u.km / u.s).value * 1000.0
        vy = velocity_data.d_y.to(u.km / u.s).value * 1000.0
        vz = velocity_data.d_z.to(u.km / u.s).value * 1000.0
        velocity = np.array([vx, vy, vz])
    else:
        # No velocity information, use zero velocity
        velocity = np.array([0.0, 0.0, 0.0])

    # Determine frame type and create corresponding astrora frame
    frame_name = astropy_frame.__class__.__name__

    # Extract obstime if available (needed for GCRS and ITRS)
    if hasattr(astropy_frame, "obstime") and astropy_frame.obstime is not None:
        from astrora.time import astropy_time_to_epoch

        epoch = astropy_time_to_epoch(astropy_frame.obstime)
    else:
        # Default to J2000.0 for frames that don't require obstime
        from astrora._core import Epoch

        epoch = Epoch.j2000_epoch()

    if isinstance(astropy_frame, AstropyICRS):
        return AstroraICRS(position, velocity)

    elif isinstance(astropy_frame, AstropyGCRS):
        return AstroraGCRS(position, velocity, epoch)

    elif isinstance(astropy_frame, AstropyITRS):
        return AstroraITRS(position, velocity, epoch)

    else:
        # Try to convert to GCRS first
        try:
            gcrs_frame = astropy_frame.transform_to(AstropyGCRS(obstime=astropy_frame.obstime))
            return from_astropy_coord(gcrs_frame)
        except Exception as e:
            raise TypeError(
                f"Unsupported astropy frame type: {frame_name}. "
                f"Supported types are ICRS, GCRS, and ITRS. "
                f"Automatic conversion to GCRS failed: {e}"
            )


def to_skycoord(
    frame: Union[AstroraICRS, AstroraGCRS, AstroraITRS, AstroraTEME],
    obstime: Union[Time, None] = None,
) -> SkyCoord:
    """
    Convert astrora coordinate frame to astropy SkyCoord.

    This is a convenience wrapper around `to_astropy_coord()` that returns
    a SkyCoord object for easy manipulation and visualization.

    Parameters
    ----------
    frame : ICRS, GCRS, ITRS, or TEME
        Astrora coordinate frame object with position and velocity.
    obstime : astropy.time.Time, optional
        Observation time for the coordinate. Required for GCRS and ITRS frames.
        If not provided, uses J2000.0 epoch.

    Returns
    -------
    astropy.coordinates.SkyCoord
        SkyCoord object containing position, velocity, and frame information.

    Examples
    --------
    >>> from astrora._core import ICRS
    >>> import numpy as np
    >>>
    >>> pos = np.array([1.496e11, 0.0, 0.0])  # 1 AU
    >>> vel = np.array([0.0, 29780.0, 0.0])
    >>> frame = ICRS(pos, vel)
    >>>
    >>> sc = to_skycoord(frame)
    >>> print(f"Distance: {sc.distance}")
    >>> print(f"RA: {sc.ra}, Dec: {sc.dec}")
    >>> print(f"Proper motion: {sc.pm_ra_cosdec}, {sc.pm_dec}")
    """
    _check_astropy()

    astropy_frame = to_astropy_coord(frame, obstime=obstime)
    return SkyCoord(astropy_frame)


def from_skycoord(skycoord: SkyCoord) -> Union[AstroraICRS, AstroraGCRS, AstroraITRS]:
    """
    Convert astropy SkyCoord to astrora coordinate frame.

    This is a convenience wrapper around `from_astropy_coord()` that accepts
    SkyCoord objects.

    Parameters
    ----------
    skycoord : astropy.coordinates.SkyCoord
        SkyCoord object with position and (optionally) velocity information.

    Returns
    -------
    ICRS, GCRS, or ITRS
        Corresponding astrora coordinate frame.

    Examples
    --------
    >>> from astropy.coordinates import SkyCoord
    >>> from astropy import units as u
    >>>
    >>> sc = SkyCoord(
    ...     x=7000*u.km, y=0*u.km, z=0*u.km,
    ...     v_x=0*u.km/u.s, v_y=7.5*u.km/u.s, v_z=0*u.km/u.s,
    ...     representation_type='cartesian',
    ...     frame='gcrs'
    ... )
    >>>
    >>> astrora_frame = from_skycoord(sc)
    """
    _check_astropy()

    # SkyCoord wraps a frame, extract it
    return from_astropy_coord(skycoord.frame)


__all__ = [
    "to_astropy_coord",
    "from_astropy_coord",
    "to_skycoord",
    "from_skycoord",
]
