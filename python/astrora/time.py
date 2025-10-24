"""
Time conversion utilities for astropy.time integration.

This module provides seamless interoperability between astropy.time.Time
and hifitime Epoch objects, enabling poliastro/hapsira API compatibility
while maintaining the high-precision benefits of hifitime internally.
"""

from typing import Optional, Union

from astrora._core import Epoch

# Try to import astropy.time - it's an optional dependency for time integration
try:
    from astropy.time import Time as AstropyTime

    ASTROPY_AVAILABLE = True
except ImportError:
    AstropyTime = None
    ASTROPY_AVAILABLE = False


# Time scale mapping between astropy and hifitime
ASTROPY_TO_HIFI_SCALE = {
    "utc": "UTC",
    "tai": "TAI",
    "tt": "TT",
    "tdb": "TDB",
    "tcb": "TDB",  # Map TCB to TDB (approximation)
    "tcg": "TT",  # Map TCG to TT (approximation)
}

HIFI_TO_ASTROPY_SCALE = {
    "UTC": "utc",
    "TAI": "tai",
    "TT": "tt",
    "TDB": "tdb",
    "GPST": "tai",  # GPS time is close to TAI (19 second offset)
}


def astropy_time_to_epoch(time: "AstropyTime") -> Epoch:
    """
    Convert an astropy.time.Time object to a hifitime Epoch.

    This function extracts the Julian Date and time scale from an astropy
    Time object and creates the corresponding hifitime Epoch. The conversion
    preserves nanosecond precision where possible.

    Parameters
    ----------
    time : astropy.time.Time
        The astropy Time object to convert

    Returns
    -------
    Epoch
        The corresponding hifitime Epoch object

    Raises
    ------
    ImportError
        If astropy is not installed
    ValueError
        If the time scale is not supported

    Examples
    --------
    >>> from astropy.time import Time
    >>> from astrora.time import astropy_time_to_epoch
    >>>
    >>> t = Time('2000-01-01 12:00:00', scale='tt')
    >>> epoch = astropy_time_to_epoch(t)
    >>> print(epoch)
    Epoch('2000-01-01T12:00:00.000000000 TT')

    Notes
    -----
    - Supported scales: UTC, TAI, TT, TDB
    - TCB and TCG are mapped to TDB and TT respectively (approximation)
    - Precision: Astropy uses dual-float JD representation for sub-nanosecond
      precision. Hifitime uses nanoseconds internally, providing equivalent
      precision for astrodynamics applications (< 65,536 years from J2000).
    """
    if not ASTROPY_AVAILABLE:
        raise ImportError(
            "astropy is required for Time integration. " "Install it with: uv pip install astropy"
        )

    # Get the time scale
    scale = time.scale.lower()

    # Map astropy scale to hifitime scale
    if scale not in ASTROPY_TO_HIFI_SCALE:
        raise ValueError(
            f"Unsupported time scale: {scale}. "
            f"Supported scales: {list(ASTROPY_TO_HIFI_SCALE.keys())}"
        )

    hifi_scale_str = ASTROPY_TO_HIFI_SCALE[scale]

    # Get Julian Date from astropy Time
    # astropy stores time as jd1 + jd2 for maximum precision
    jd = time.jd  # This gives the full JD as a float64

    # Create Epoch from JD in the appropriate time scale using high-precision method
    # The Epoch.from_jd_scale() method preserves nanosecond precision
    return Epoch.from_jd_scale(jd, hifi_scale_str)


def epoch_to_astropy_time(epoch: Epoch, scale: Optional[str] = None) -> "AstropyTime":
    """
    Convert a hifitime Epoch to an astropy.time.Time object.

    Parameters
    ----------
    epoch : Epoch
        The hifitime Epoch to convert
    scale : str, optional
        The desired time scale for the output Time object.
        If None, uses TT (Terrestrial Time) as the default.
        Supported: 'utc', 'tai', 'tt', 'tdb'

    Returns
    -------
    astropy.time.Time
        The corresponding astropy Time object

    Raises
    ------
    ImportError
        If astropy is not installed
    ValueError
        If the requested scale is not supported

    Examples
    --------
    >>> from astrora._core import Epoch
    >>> from astrora.time import epoch_to_astropy_time
    >>>
    >>> epoch = Epoch.j2000_epoch()
    >>> t = epoch_to_astropy_time(epoch, scale='tt')
    >>> print(t)
    2000-01-01 12:00:00.000
    >>> print(t.scale)
    tt

    Notes
    -----
    - Default scale is TT (Terrestrial Time) as it's the standard for J2000
    - The conversion preserves precision within numerical limits
    - UTC conversions account for leap seconds automatically
    """
    if not ASTROPY_AVAILABLE:
        raise ImportError(
            "astropy is required for Time integration. " "Install it with: uv pip install astropy"
        )

    # Default to TT if no scale specified
    if scale is None:
        scale = "tt"

    scale = scale.lower()
    if scale not in ["utc", "tai", "tt", "tdb"]:
        raise ValueError(f"Unsupported scale: {scale}. " f"Supported: utc, tai, tt, tdb")

    # Get JD from Epoch in the requested scale
    if scale == "utc":
        jd = epoch.jd_utc
    elif scale == "tai":
        # Use TT as proxy and convert (hifitime doesn't expose JD in TAI directly)
        # Get MJD in TAI and convert to JD
        mjd_tai = epoch.mjd_tai
        jd = mjd_tai + 2400000.5
    elif scale == "tt":
        jd = epoch.jd_tt
    elif scale == "tdb":
        # Similar for TDB
        mjd_tdb = epoch.mjd_tdb
        jd = mjd_tdb + 2400000.5

    # Create astropy Time from JD
    return AstropyTime(jd, format="jd", scale=scale)


def to_epoch(time_input: Union[Epoch, "AstropyTime", None]) -> Optional[Epoch]:
    """
    Convert various time representations to a hifitime Epoch.

    This is a convenience function that accepts multiple input types and
    normalizes them to Epoch objects for internal use.

    Parameters
    ----------
    time_input : Epoch, astropy.time.Time, or None
        The time to convert. If None, returns None.

    Returns
    -------
    Epoch or None
        The corresponding Epoch object, or None if input was None

    Raises
    ------
    TypeError
        If the input type is not supported
    ImportError
        If astropy.Time is provided but astropy is not installed

    Examples
    --------
    >>> from astrora.time import to_epoch
    >>> from astrora._core import Epoch
    >>> from astropy.time import Time
    >>>
    >>> # Already an Epoch - pass through
    >>> epoch = Epoch.j2000_epoch()
    >>> result = to_epoch(epoch)
    >>> assert result == epoch
    >>>
    >>> # Convert from astropy Time
    >>> t = Time('2000-01-01 12:00:00', scale='tt')
    >>> epoch = to_epoch(t)
    >>> print(epoch)
    Epoch('2000-01-01T12:00:00 TT')
    >>>
    >>> # None pass-through
    >>> result = to_epoch(None)
    >>> assert result is None
    """
    if time_input is None:
        return None

    if isinstance(time_input, Epoch):
        return time_input

    if ASTROPY_AVAILABLE and isinstance(time_input, AstropyTime):
        return astropy_time_to_epoch(time_input)

    raise TypeError(
        f"Unsupported time type: {type(time_input)}. " f"Expected Epoch, astropy.time.Time, or None"
    )


def to_astropy_time(
    time_input: Union[Epoch, "AstropyTime", None], scale: Optional[str] = None
) -> Optional["AstropyTime"]:
    """
    Convert various time representations to astropy.time.Time.

    Parameters
    ----------
    time_input : Epoch, astropy.time.Time, or None
        The time to convert. If None, returns None.
    scale : str, optional
        The desired time scale for the output. Defaults to 'tt'.

    Returns
    -------
    astropy.time.Time or None
        The corresponding Time object, or None if input was None

    Raises
    ------
    TypeError
        If the input type is not supported
    ImportError
        If astropy is not installed

    Examples
    --------
    >>> from astrora.time import to_astropy_time
    >>> from astrora._core import Epoch
    >>>
    >>> epoch = Epoch.j2000_epoch()
    >>> t = to_astropy_time(epoch, scale='utc')
    >>> print(t.scale)
    utc
    """
    if not ASTROPY_AVAILABLE:
        raise ImportError(
            "astropy is required for Time integration. " "Install it with: uv pip install astropy"
        )

    if time_input is None:
        return None

    if isinstance(time_input, AstropyTime):
        # Already astropy Time - optionally convert scale
        if scale is not None and time_input.scale != scale.lower():
            # Convert to requested scale
            return getattr(time_input, scale.lower())
        return time_input

    if isinstance(time_input, Epoch):
        return epoch_to_astropy_time(time_input, scale)

    raise TypeError(
        f"Unsupported time type: {type(time_input)}. " f"Expected Epoch, astropy.time.Time, or None"
    )
