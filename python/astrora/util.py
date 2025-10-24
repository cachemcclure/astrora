"""
Utility functions for orbital mechanics calculations.

This module provides various helper functions for working with orbits,
angles, time ranges, and other common operations.
"""

from typing import Optional, Union

import numpy as np
from astropy import units as u
from astropy.time import Time, TimeDelta


def time_range(
    start: Union[Time, str],
    *,
    periods: int = 50,
    spacing: Optional[Union[TimeDelta, u.Quantity]] = None,
    end: Optional[Union[Time, str]] = None,
    format: Optional[str] = None,
    scale: Optional[str] = None,
) -> Time:
    """
    Generate a range of astronomical times.

    This function creates an array of Time objects, useful for propagating
    orbits or generating ephemerides over a time span.

    Parameters
    ----------
    start : Time or str
        Start time. If str, format and scale parameters apply.
    periods : int, optional
        Number of time points to generate (default: 50).
        Ignored if both spacing and end are provided.
    spacing : TimeDelta or Quantity, optional
        Time spacing between consecutive points.
        If Quantity, must have time units (e.g., u.hour, u.day).
        If not provided and end is given, spacing is computed automatically.
    end : Time or str, optional
        End time. If str, format and scale parameters apply.
        If provided with spacing, periods is ignored.
    format : str, optional
        Format string for parsing start/end if they are strings
        (e.g., 'iso', 'jd', 'mjd'). See astropy.time.Time formats.
    scale : str, optional
        Time scale (e.g., 'utc', 'tt', 'tdb'). Default: 'utc'.

    Returns
    -------
    Time
        Array of Time objects spanning the requested range

    Raises
    ------
    ValueError
        If neither spacing nor end is provided

    Examples
    --------
    >>> from astrora.util import time_range
    >>> from astropy.time import Time
    >>> from astropy import units as u
    >>>
    >>> # Generate 100 time points over 1 day
    >>> start = Time('2024-01-01 00:00:00', scale='utc')
    >>> times = time_range(start, end=Time('2024-01-02 00:00:00'), periods=100)
    >>> len(times)
    100
    >>>
    >>> # Generate times with 1-hour spacing for 24 hours
    >>> times = time_range(start, spacing=1*u.hour, periods=24)
    >>> len(times)
    24
    >>>
    >>> # Automatically determine spacing from start and end
    >>> times = time_range(
    ...     Time('2024-01-01'),
    ...     end=Time('2024-01-10'),
    ...     periods=10
    ... )

    Notes
    -----
    This function is compatible with poliastro/hapsira's time_range API.

    If both spacing and end are provided, the function generates times
    from start to end with the given spacing, and periods is ignored.

    The default number of periods (50) matches poliastro's default.
    """
    # Parse start time
    if isinstance(start, str):
        start = Time(start, format=format, scale=scale or "utc")

    # Parse end time if provided
    if end is not None and isinstance(end, str):
        end = Time(end, format=format, scale=scale or "utc")

    # Convert spacing to TimeDelta if it's a Quantity
    if spacing is not None and isinstance(spacing, u.Quantity):
        spacing = TimeDelta(spacing.to(u.s).value, format="sec")

    # Case 1: spacing and end both provided -> ignore periods
    if spacing is not None and end is not None:
        # Generate times from start to end with given spacing
        duration = (end - start).sec  # Total duration in seconds
        spacing_sec = spacing.sec
        num_points = int(duration / spacing_sec) + 1
        offsets = np.arange(num_points) * spacing_sec
        return start + TimeDelta(offsets, format="sec")

    # Case 2: only end provided -> compute spacing from periods
    elif end is not None:
        # Generate evenly spaced times from start to end
        duration = (end - start).sec
        offsets = np.linspace(0, duration, periods)
        return start + TimeDelta(offsets, format="sec")

    # Case 3: only spacing provided -> use periods
    elif spacing is not None:
        spacing_sec = spacing.sec
        offsets = np.arange(periods) * spacing_sec
        return start + TimeDelta(offsets, format="sec")

    # Case 4: neither spacing nor end provided -> error
    else:
        raise ValueError(
            "Must provide either 'spacing', 'end', or both. "
            "Cannot determine time range with only 'start' and 'periods'."
        )


def norm(vector: Union[np.ndarray, u.Quantity]) -> Union[float, u.Quantity]:
    """
    Calculate the norm (magnitude) of a vector, respecting units.

    This function computes the Euclidean norm of a vector and preserves
    astropy units if present.

    Parameters
    ----------
    vector : array_like or Quantity
        Input vector. Can be 1D or 2D (for multiple vectors).
        If Quantity, units are preserved in the output.

    Returns
    -------
    float or Quantity
        Norm of the vector. If input has units, output has the same units.
        For 2D arrays, returns norm of each row.

    Examples
    --------
    >>> from astrora.util import norm
    >>> from astropy import units as u
    >>> import numpy as np
    >>>
    >>> # Simple vector norm
    >>> v = np.array([3, 4, 0])
    >>> norm(v)
    5.0
    >>>
    >>> # Vector with units
    >>> r = [6378, 0, 0] << u.km
    >>> norm(r)
    <Quantity 6378. km>
    >>>
    >>> # Multiple vectors (2D array)
    >>> vectors = np.array([[3, 4, 0], [0, 5, 12]])
    >>> norm(vectors)
    array([5., 13.])

    Notes
    -----
    This function is equivalent to np.linalg.norm but preserves astropy units.
    It's compatible with poliastro/hapsira's util.norm function.
    """
    if isinstance(vector, u.Quantity):
        # Extract value, compute norm, reattach units
        value = vector.value
        if value.ndim == 1:
            magnitude = np.linalg.norm(value)
        else:
            # For 2D arrays, compute norm along last axis (each row)
            magnitude = np.linalg.norm(value, axis=-1)
        return magnitude * vector.unit
    else:
        # No units, just compute norm
        if vector.ndim == 1:
            return np.linalg.norm(vector)
        else:
            return np.linalg.norm(vector, axis=-1)


def wrap_angle(
    angle: Union[float, u.Quantity],
    limit: Union[float, u.Quantity] = 180.0,
) -> Union[float, u.Quantity]:
    """
    Wrap an angle to a specified range.

    This function wraps angles to the range [-limit, +limit). By default,
    wraps to [-180°, +180°) or [-π, +π) depending on input units.

    Parameters
    ----------
    angle : float or Quantity
        Input angle. If float, assumed to be in degrees.
        If Quantity, must have angle units.
    limit : float or Quantity, optional
        Upper limit for wrapping (default: 180 degrees).
        The angle will be wrapped to [-limit, +limit).
        If float, assumed to be in degrees.
        If Quantity, must have angle units.

    Returns
    -------
    float or Quantity
        Wrapped angle in the range [-limit, +limit).
        If input is Quantity, output preserves the input units.

    Examples
    --------
    >>> from astrora.util import wrap_angle
    >>> from astropy import units as u
    >>> import numpy as np
    >>>
    >>> # Wrap 370° to [-180°, +180°)
    >>> wrap_angle(370)
    10.0
    >>>
    >>> # Wrap -190° to [-180°, +180°)
    >>> wrap_angle(-190)
    170.0
    >>>
    >>> # Wrap with units (radians)
    >>> wrap_angle(7*u.rad, limit=np.pi*u.rad)
    <Quantity 0.71681469 rad>
    >>>
    >>> # Wrap to custom range [-90°, +90°)
    >>> wrap_angle(100*u.deg, limit=90*u.deg)
    <Quantity -80. deg>

    Notes
    -----
    This function is compatible with poliastro/hapsira's util.wrap_angle.

    The wrapping formula is: ((angle + limit) % (2 * limit)) - limit
    """
    if isinstance(angle, u.Quantity):
        # Both angle and limit should be Quantities with angle units
        if not isinstance(limit, u.Quantity):
            # Assume limit is in same units as angle if not specified
            limit = limit * angle.unit

        # Ensure both are in same units
        limit_val = limit.to(angle.unit).value
        angle_val = angle.value

        # Wrap using modulo arithmetic
        wrapped = ((angle_val + limit_val) % (2 * limit_val)) - limit_val

        return wrapped * angle.unit
    else:
        # No units, assume degrees
        if isinstance(limit, u.Quantity):
            limit = limit.to(u.deg).value

        wrapped = ((angle + limit) % (2 * limit)) - limit
        return wrapped


def alinspace(
    start: Union[float, u.Quantity],
    stop: Union[float, u.Quantity],
    num: int = 50,
) -> Union[np.ndarray, u.Quantity]:
    """
    Return evenly spaced angular values over a specified interval.

    This function is similar to np.linspace but handles angular wrapping
    correctly, allowing for ranges that span more than 2π radians.

    Parameters
    ----------
    start : float or Quantity
        Starting angle. If float, assumed to be in radians.
        If Quantity, must have angle units.
    stop : float or Quantity
        Ending angle. If float, assumed to be in radians.
        If Quantity, must have angle units (converted to match start).
    num : int, optional
        Number of samples to generate (default: 50).
        Must be >= 2.

    Returns
    -------
    ndarray or Quantity
        Array of evenly spaced angles from start to stop (inclusive).
        If inputs are Quantity, output preserves units.

    Examples
    --------
    >>> from astrora.util import alinspace
    >>> from astropy import units as u
    >>> import numpy as np
    >>>
    >>> # Generate 5 angles from 0 to π radians
    >>> angles = alinspace(0, np.pi, num=5)
    >>> angles
    array([0.        , 0.78539816, 1.57079633, 2.35619449, 3.14159265])
    >>>
    >>> # With units (degrees)
    >>> angles = alinspace(0*u.deg, 90*u.deg, num=4)
    >>> angles
    <Quantity [ 0., 30., 60., 90.] deg>
    >>>
    >>> # Across multiple revolutions (0 to 720 degrees)
    >>> angles = alinspace(0*u.deg, 720*u.deg, num=9)
    >>> len(angles)
    9

    Notes
    -----
    This function is compatible with poliastro/hapsira's util.alinspace.

    Unlike wrap_angle, this function does NOT wrap the output angles.
    It generates linearly spaced values that can exceed 2π if the range
    between start and stop is large.
    """
    if num < 2:
        raise ValueError("num must be at least 2")

    if isinstance(start, u.Quantity):
        # Work with Quantities
        if not isinstance(stop, u.Quantity):
            # Assume stop has same units as start
            stop = stop * start.unit
        else:
            # Convert stop to same units as start
            stop = stop.to(start.unit)

        # Generate linearly spaced values
        values = np.linspace(start.value, stop.value, num)
        return values * start.unit
    else:
        # No units, work with floats
        if isinstance(stop, u.Quantity):
            # Convert to radians if stop has units but start doesn't
            stop = stop.to(u.rad).value

        return np.linspace(start, stop, num)


def find_closest_value(
    value: Union[float, u.Quantity],
    values: Union[np.ndarray, u.Quantity],
) -> int:
    """
    Find the index of the closest value in an array.

    Parameters
    ----------
    value : float or Quantity
        Target value to find
    values : array_like or Quantity
        Array of values to search

    Returns
    -------
    int
        Index of the closest value in the array

    Examples
    --------
    >>> from astrora.util import find_closest_value
    >>> from astropy import units as u
    >>> import numpy as np
    >>>
    >>> # Find closest value in array
    >>> arr = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
    >>> find_closest_value(3.7, arr)
    3
    >>>
    >>> # With units
    >>> times = np.array([0, 100, 200, 300]) << u.s
    >>> find_closest_value(250*u.s, times)
    2

    Notes
    -----
    This function is compatible with poliastro/hapsira's util.find_closest_value.
    """
    if isinstance(value, u.Quantity) and isinstance(values, u.Quantity):
        # Convert to same units
        value_val = value.to(values.unit).value
        values_val = values.value
    elif isinstance(values, u.Quantity):
        # value is float, values is Quantity - assume same units
        value_val = value
        values_val = values.value
    elif isinstance(value, u.Quantity):
        # value is Quantity, values is array
        value_val = value.value
        values_val = values
    else:
        # Both are regular arrays
        value_val = value
        values_val = values

    # Find index of minimum distance
    idx = np.argmin(np.abs(values_val - value_val))
    return int(idx)


# Export all utility functions
__all__ = [
    "time_range",
    "norm",
    "wrap_angle",
    "alinspace",
    "find_closest_value",
]
