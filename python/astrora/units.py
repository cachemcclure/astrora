"""
Unit handling utilities for astropy integration.

This module provides helper functions for converting between astropy Quantity
objects and raw numerical values used by the Rust backend. It enables the
high-level Python API to accept quantities with units while maintaining
backward compatibility with raw floats/arrays.
"""

from typing import Union

import numpy as np
from astropy import units as u

# Common unit types for validation
LENGTH_UNITS = u.m  # Base SI unit for distance
VELOCITY_UNITS = u.m / u.s  # Base SI unit for velocity
ANGLE_UNITS = u.rad  # Base SI unit for angles
TIME_UNITS = u.s  # Base SI unit for time
DIMENSIONLESS = u.dimensionless_unscaled  # For eccentricity, etc.


def _is_quantity(value) -> bool:
    """
    Check if a value is an astropy Quantity object.

    Parameters
    ----------
    value : any
        Value to check

    Returns
    -------
    bool
        True if value is a Quantity, False otherwise
    """
    return hasattr(value, "unit") and hasattr(value, "value")


def _to_si_value(quantity, target_unit: u.Unit, name: str = "value") -> Union[float, np.ndarray]:
    """
    Convert a Quantity to SI base unit value (scalar or array).

    Handles both raw values (pass through) and Quantity objects (convert).

    Parameters
    ----------
    quantity : float, np.ndarray, or Quantity
        Input value with or without units
    target_unit : astropy.units.Unit
        Target SI unit for conversion (e.g., u.m, u.m/u.s)
    name : str, optional
        Name of the parameter for error messages

    Returns
    -------
    float or np.ndarray
        Value in SI units (dimensionless)

    Raises
    ------
    ValueError
        If quantity has incompatible units

    Examples
    --------
    >>> import numpy as np
    >>> from astropy import units as u
    >>>
    >>> # Raw value passes through
    >>> _to_si_value(1000.0, u.m, "distance")
    1000.0
    >>>
    >>> # Quantity converts to SI
    >>> _to_si_value(1 << u.km, u.m, "distance")
    1000.0
    >>>
    >>> # Array with units
    >>> r = [7000, 0, 0] << u.km
    >>> _to_si_value(r, u.m, "position")
    array([7000000., 0., 0.])
    """
    # If it's a Quantity, validate and convert
    if _is_quantity(quantity):
        try:
            # Convert to target unit and extract value
            converted = quantity.to(target_unit)
            return converted.value
        except u.UnitConversionError as e:
            raise ValueError(
                f"Cannot convert {name} with units {quantity.unit} to {target_unit}. "
                f"Expected units compatible with {target_unit}."
            ) from e

    # Raw value - return as-is (assume SI units)
    return quantity


def to_si_length(value, name: str = "length") -> float:
    """
    Convert length/distance to meters (SI).

    Parameters
    ----------
    value : float or Quantity
        Length value (meters if raw, or with units)
    name : str
        Parameter name for error messages

    Returns
    -------
    float
        Length in meters
    """
    result = _to_si_value(value, u.m, name)
    return float(result) if np.isscalar(result) else result


def to_si_velocity(value, name: str = "velocity") -> float:
    """
    Convert velocity to m/s (SI).

    Parameters
    ----------
    value : float or Quantity
        Velocity value (m/s if raw, or with units)
    name : str
        Parameter name for error messages

    Returns
    -------
    float
        Velocity in m/s
    """
    result = _to_si_value(value, u.m / u.s, name)
    return float(result) if np.isscalar(result) else result


def to_si_angle(value, name: str = "angle") -> float:
    """
    Convert angle to radians (SI).

    Parameters
    ----------
    value : float or Quantity
        Angle value (radians if raw, or with units like degrees)
    name : str
        Parameter name for error messages

    Returns
    -------
    float
        Angle in radians
    """
    result = _to_si_value(value, u.rad, name)
    return float(result) if np.isscalar(result) else result


def to_si_time(value, name: str = "time") -> float:
    """
    Convert time to seconds (SI).

    Parameters
    ----------
    value : float or Quantity
        Time value (seconds if raw, or with units)
    name : str
        Parameter name for error messages

    Returns
    -------
    float
        Time in seconds
    """
    result = _to_si_value(value, u.s, name)
    return float(result) if np.isscalar(result) else result


def to_dimensionless(value, name: str = "value") -> float:
    """
    Convert dimensionless quantity to float.

    Parameters
    ----------
    value : float or Quantity
        Dimensionless value (e.g., eccentricity)
    name : str
        Parameter name for error messages

    Returns
    -------
    float
        Dimensionless value as float
    """
    if _is_quantity(value):
        # Check that it's dimensionless
        if value.unit.physical_type != "dimensionless":
            raise ValueError(f"{name} must be dimensionless, got units: {value.unit}")
        return float(value.value)

    return float(value)


def to_si_vector(vector, target_unit: u.Unit, name: str = "vector") -> np.ndarray:
    """
    Convert 3D vector (position or velocity) to SI units.

    Parameters
    ----------
    vector : array-like or Quantity
        3-element vector with or without units
    target_unit : astropy.units.Unit
        Target SI unit (u.m or u.m/u.s)
    name : str
        Parameter name for error messages

    Returns
    -------
    np.ndarray
        3-element array in SI units (meters or m/s)

    Raises
    ------
    ValueError
        If vector is not 3-dimensional or has incompatible units

    Examples
    --------
    >>> from astropy import units as u
    >>>
    >>> # Raw array
    >>> to_si_vector([7000e3, 0, 0], u.m)
    array([7000000., 0., 0.])
    >>>
    >>> # With units
    >>> r = [7000, 0, 0] << u.km
    >>> to_si_vector(r, u.m, "position")
    array([7000000., 0., 0.])
    """
    # Convert to SI
    result = _to_si_value(vector, target_unit, name)

    # Ensure it's a numpy array
    result = np.asarray(result, dtype=np.float64)

    # Validate shape
    if result.shape != (3,):
        raise ValueError(f"{name} must be a 3-element vector, got shape: {result.shape}")

    return result


# Convenience functions for common conversions
def to_si_position(r) -> np.ndarray:
    """Convert position vector to meters."""
    return to_si_vector(r, u.m, "position")


def to_si_velocity_vector(v) -> np.ndarray:
    """Convert velocity vector to m/s."""
    return to_si_vector(v, u.m / u.s, "velocity")


# Functions for returning Quantity objects from properties
def as_quantity_length(value: float, preferred_unit=u.km) -> u.Quantity:
    """
    Convert scalar length from SI (meters) to Quantity with preferred unit.

    Parameters
    ----------
    value : float
        Length in meters (SI)
    preferred_unit : astropy.units.Unit
        Preferred display unit (default: km)

    Returns
    -------
    Quantity
        Length as Quantity with preferred unit
    """
    return (value * u.m).to(preferred_unit)


def as_quantity_velocity(value: float, preferred_unit=u.km / u.s) -> u.Quantity:
    """
    Convert scalar velocity from SI (m/s) to Quantity with preferred unit.

    Parameters
    ----------
    value : float
        Velocity in m/s (SI)
    preferred_unit : astropy.units.Unit
        Preferred display unit (default: km/s)

    Returns
    -------
    Quantity
        Velocity as Quantity with preferred unit
    """
    return (value * u.m / u.s).to(preferred_unit)


def as_quantity_angle(value: float, preferred_unit=u.rad) -> u.Quantity:
    """
    Convert scalar angle from SI (radians) to Quantity with preferred unit.

    Parameters
    ----------
    value : float
        Angle in radians (SI)
    preferred_unit : astropy.units.Unit
        Preferred display unit (default: radians)

    Returns
    -------
    Quantity
        Angle as Quantity with preferred unit
    """
    return (value * u.rad).to(preferred_unit)


def as_quantity_time(value: float, preferred_unit=u.s) -> u.Quantity:
    """
    Convert scalar time from SI (seconds) to Quantity with preferred unit.

    Parameters
    ----------
    value : float
        Time in seconds (SI)
    preferred_unit : astropy.units.Unit
        Preferred display unit (default: seconds)

    Returns
    -------
    Quantity
        Time as Quantity with preferred unit
    """
    return (value * u.s).to(preferred_unit)


def as_quantity_vector(vector: np.ndarray, unit_type: str) -> u.Quantity:
    """
    Convert vector from SI units to Quantity with appropriate unit.

    Parameters
    ----------
    vector : np.ndarray
        3-element vector in SI units
    unit_type : str
        Type of vector: 'position' (km) or 'velocity' (km/s)

    Returns
    -------
    Quantity
        Vector as Quantity with appropriate units
    """
    if unit_type == "position":
        return vector * u.m
    elif unit_type == "velocity":
        return vector * u.m / u.s
    else:
        raise ValueError(f"Unknown unit_type: {unit_type}")
