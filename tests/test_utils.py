"""
Test utility functions for astrora test suite.

This module provides common helper functions used across multiple test files:
- Numerical comparison utilities with context-aware tolerances
- State vector and orbital element comparisons
- Conservation law checkers
- Test data generators
- Pretty printing for test failures
"""

from typing import Tuple

import astrora._core as core
import numpy as np

# ============================================================================
# Numerical Comparison Utilities
# ============================================================================


def assert_allclose_with_context(
    actual: np.ndarray,
    desired: np.ndarray,
    rtol: float = 1e-7,
    atol: float = 0,
    context: str = "",
    err_msg: str = "",
) -> None:
    """
    Enhanced version of np.testing.assert_allclose with better error messages.

    Args:
        actual: Actual array values
        desired: Expected array values
        rtol: Relative tolerance
        atol: Absolute tolerance
        context: Context string to add to error message (e.g., "position", "velocity")
        err_msg: Additional error message
    """
    try:
        np.testing.assert_allclose(actual, desired, rtol=rtol, atol=atol, err_msg=err_msg)
    except AssertionError as e:
        if context:
            max_diff = np.max(np.abs(actual - desired))
            rel_diff = np.max(np.abs((actual - desired) / (desired + 1e-100)))
            raise AssertionError(
                f"{context} comparison failed:\n"
                f"  Max absolute difference: {max_diff:.3e}\n"
                f"  Max relative difference: {rel_diff:.3e}\n"
                f"  Tolerances: rtol={rtol:.3e}, atol={atol:.3e}\n"
                f"  Original error: {e}"
            )
        else:
            raise


def assert_states_equal(
    state1: "core.CartesianState",
    state2: "core.CartesianState",
    position_tol: float = 1e-6,
    velocity_tol: float = 1e-9,
    context: str = "",
) -> None:
    """
    Assert that two CartesianState objects are equal within tolerances.

    Args:
        state1: First state
        state2: Second state
        position_tol: Position tolerance in meters
        velocity_tol: Velocity tolerance in m/s
        context: Context string for error messages
    """
    pos1 = np.array(state1.position)
    pos2 = np.array(state2.position)
    vel1 = np.array(state1.velocity)
    vel2 = np.array(state2.velocity)

    assert_allclose_with_context(
        pos1, pos2, atol=position_tol, context=f"{context} position" if context else "position"
    )
    assert_allclose_with_context(
        vel1, vel2, atol=velocity_tol, context=f"{context} velocity" if context else "velocity"
    )


def assert_elements_equal(
    elem1: "core.OrbitalElements",
    elem2: "core.OrbitalElements",
    atol_m: float = 1.0,
    atol_rad: float = 1e-9,
    context: str = "",
) -> None:
    """
    Assert that two OrbitalElements objects are equal within tolerances.

    Args:
        elem1: First orbital elements
        elem2: Second orbital elements
        atol_m: Absolute tolerance for semi-major axis (meters)
        atol_rad: Absolute tolerance for angles (radians)
        context: Context string for error messages
    """
    assert_allclose_with_context(
        np.array([elem1.a]),
        np.array([elem2.a]),
        atol=atol_m,
        context=f"{context} semi-major axis" if context else "semi-major axis",
    )

    assert_allclose_with_context(
        np.array([elem1.e]),
        np.array([elem2.e]),
        atol=1e-9,
        context=f"{context} eccentricity" if context else "eccentricity",
    )

    # Angular elements
    angles1 = np.array([elem1.i, elem1.raan, elem1.argp, elem1.nu])
    angles2 = np.array([elem2.i, elem2.raan, elem2.argp, elem2.nu])

    assert_allclose_with_context(
        angles1, angles2, atol=atol_rad, context=f"{context} angles" if context else "angles"
    )


# ============================================================================
# Conservation Law Checkers
# ============================================================================


def compute_specific_energy(state: "core.CartesianState", gm: float) -> float:
    """
    Compute specific orbital energy (energy per unit mass).

    Args:
        state: Cartesian state
        gm: Gravitational parameter

    Returns:
        Specific energy in J/kg (or m²/s²)
    """
    pos = np.array(state.position)
    vel = np.array(state.velocity)
    r = np.linalg.norm(pos)
    v_squared = np.dot(vel, vel)
    return 0.5 * v_squared - gm / r


def compute_specific_angular_momentum(state: "core.CartesianState") -> np.ndarray:
    """
    Compute specific angular momentum vector.

    Args:
        state: Cartesian state

    Returns:
        Angular momentum vector [hx, hy, hz] in m²/s
    """
    r = np.array(state.position)
    v = np.array(state.velocity)
    return np.cross(r, v)


def assert_energy_conserved(
    state_initial: "core.CartesianState",
    state_final: "core.CartesianState",
    gm: float,
    rtol: float = 1e-10,
    context: str = "",
) -> None:
    """
    Assert that orbital energy is conserved between two states.

    Args:
        state_initial: Initial state
        state_final: Final state
        gm: Gravitational parameter
        rtol: Relative tolerance
        context: Context string for error messages
    """
    e_initial = compute_specific_energy(state_initial, gm)
    e_final = compute_specific_energy(state_final, gm)

    rel_error = abs((e_final - e_initial) / e_initial)

    if rel_error > rtol:
        raise AssertionError(
            f"{context + ': ' if context else ''}Energy not conserved!\n"
            f"  Initial energy: {e_initial:.12e} J/kg\n"
            f"  Final energy:   {e_final:.12e} J/kg\n"
            f"  Difference:     {e_final - e_initial:.12e} J/kg\n"
            f"  Relative error: {rel_error:.12e}\n"
            f"  Tolerance:      {rtol:.12e}"
        )


def assert_angular_momentum_conserved(
    state_initial: "core.CartesianState",
    state_final: "core.CartesianState",
    rtol: float = 1e-10,
    context: str = "",
) -> None:
    """
    Assert that angular momentum is conserved between two states.

    Args:
        state_initial: Initial state
        state_final: Final state
        rtol: Relative tolerance
        context: Context string for error messages
    """
    h_initial = compute_specific_angular_momentum(state_initial)
    h_final = compute_specific_angular_momentum(state_final)

    h_mag_initial = np.linalg.norm(h_initial)
    h_mag_final = np.linalg.norm(h_final)

    rel_error = abs((h_mag_final - h_mag_initial) / h_mag_initial)

    if rel_error > rtol:
        raise AssertionError(
            f"{context + ': ' if context else ''}Angular momentum not conserved!\n"
            f"  Initial |h|: {h_mag_initial:.12e} m²/s\n"
            f"  Final |h|:   {h_mag_final:.12e} m²/s\n"
            f"  Difference:  {h_mag_final - h_mag_initial:.12e} m²/s\n"
            f"  Relative error: {rel_error:.12e}\n"
            f"  Tolerance:      {rtol:.12e}"
        )


# ============================================================================
# Test Data Generators
# ============================================================================


def generate_test_orbits(n_orbits: int = 10, seed: int = 42) -> list:
    """
    Generate a list of diverse test orbits for property-based testing.

    Args:
        n_orbits: Number of orbits to generate
        seed: Random seed for reproducibility

    Returns:
        List of OrbitalElements objects
    """
    np.random.seed(seed)
    orbits = []

    for _ in range(n_orbits):
        # Random orbital elements
        a = np.random.uniform(6.6e6, 1.0e8)  # 200 km to very high altitude
        e = np.random.uniform(0.0, 0.95)  # Low to high eccentricity
        i = np.random.uniform(0.0, np.pi)  # All inclinations
        raan = np.random.uniform(0.0, 2 * np.pi)
        omega = np.random.uniform(0.0, 2 * np.pi)
        nu = np.random.uniform(0.0, 2 * np.pi)

        orbits.append(
            core.OrbitalElements(
                semi_major_axis=a,
                eccentricity=e,
                inclination=i,
                raan=raan,
                argument_of_periapsis=omega,
                true_anomaly=nu,
                gm=core.constants.GM_EARTH,
            )
        )

    return orbits


def generate_anomaly_test_cases(
    eccentricity: float, n_points: int = 100
) -> Tuple[np.ndarray, np.ndarray, np.ndarray]:
    """
    Generate test cases for anomaly conversions.

    Args:
        eccentricity: Orbit eccentricity
        n_points: Number of test points

    Returns:
        Tuple of (true_anomalies, eccentric_anomalies, mean_anomalies)
    """
    # Generate true anomalies
    true_anomalies = np.linspace(0, 2 * np.pi, n_points)

    # Convert to eccentric and mean anomalies using astrora
    eccentric_anomalies = np.array(
        [core.true_to_eccentric_anomaly(nu, eccentricity) for nu in true_anomalies]
    )

    mean_anomalies = np.array(
        [core.eccentric_to_mean_anomaly(E, eccentricity) for E in eccentric_anomalies]
    )

    return true_anomalies, eccentric_anomalies, mean_anomalies


# ============================================================================
# Coordinate System Utilities
# ============================================================================


def compute_position_difference_magnitude(
    state1: "core.CartesianState", state2: "core.CartesianState"
) -> float:
    """
    Compute the magnitude of position difference between two states.

    Args:
        state1: First state
        state2: Second state

    Returns:
        Position difference magnitude in meters
    """
    pos1 = np.array(state1.position)
    pos2 = np.array(state2.position)
    return np.linalg.norm(pos1 - pos2)


def compute_velocity_difference_magnitude(
    state1: "core.CartesianState", state2: "core.CartesianState"
) -> float:
    """
    Compute the magnitude of velocity difference between two states.

    Args:
        state1: First state
        state2: Second state

    Returns:
        Velocity difference magnitude in m/s
    """
    vel1 = np.array(state1.velocity)
    vel2 = np.array(state2.velocity)
    return np.linalg.norm(vel1 - vel2)


# ============================================================================
# Orbit Classification Utilities
# ============================================================================


def classify_orbit_regime(semi_major_axis: float, earth_radius: float = 6378137.0) -> str:
    """
    Classify orbit by altitude regime.

    Args:
        semi_major_axis: Semi-major axis in meters
        earth_radius: Earth radius in meters

    Returns:
        String classification: "LEO", "MEO", "GEO", "HEO", etc.
    """
    altitude = semi_major_axis - earth_radius

    if altitude < 2000000:  # < 2000 km
        return "LEO"
    elif altitude < 35000000:  # < 35,000 km
        return "MEO"
    elif 35000000 <= altitude <= 37000000:  # ~GEO altitude
        return "GEO"
    else:
        return "HEO"


def is_circular_orbit(eccentricity: float, tol: float = 1e-6) -> bool:
    """Check if orbit is circular."""
    return eccentricity < tol


def is_equatorial_orbit(inclination: float, tol: float = 1e-6) -> bool:
    """Check if orbit is equatorial."""
    return abs(inclination) < tol or abs(inclination - np.pi) < tol


def is_polar_orbit(inclination: float, tol: float = 1e-6) -> bool:
    """Check if orbit is polar."""
    return abs(inclination - np.pi / 2) < tol


# ============================================================================
# Pretty Printing for Test Output
# ============================================================================


def format_state_vector(state: "core.CartesianState", name: str = "State") -> str:
    """
    Format a CartesianState for pretty printing in test output.

    Args:
        state: State to format
        name: Name/label for the state

    Returns:
        Formatted string representation
    """
    pos = np.array(state.position)
    vel = np.array(state.velocity)
    r = np.linalg.norm(pos)
    v = np.linalg.norm(vel)

    return (
        f"{name}:\n"
        f"  Position: [{pos[0]:14.6f}, {pos[1]:14.6f}, {pos[2]:14.6f}] m\n"
        f"  Velocity: [{vel[0]:14.6f}, {vel[1]:14.6f}, {vel[2]:14.6f}] m/s\n"
        f"  |r| = {r:14.6f} m, |v| = {v:14.6f} m/s"
    )


def format_orbital_elements(elem: "core.OrbitalElements", name: str = "Elements") -> str:
    """
    Format OrbitalElements for pretty printing in test output.

    Args:
        elem: Elements to format
        name: Name/label for the elements

    Returns:
        Formatted string representation
    """
    return (
        f"{name}:\n"
        f"  a = {elem.a:14.6f} m\n"
        f"  e = {elem.e:14.12f}\n"
        f"  i = {np.degrees(elem.i):14.6f}°\n"
        f"  Ω = {np.degrees(elem.raan):14.6f}°\n"
        f"  ω = {np.degrees(elem.argp):14.6f}°\n"
        f"  ν = {np.degrees(elem.nu):14.6f}°"
    )


# ============================================================================
# Skip Decorators for Conditional Tests
# ============================================================================


def skip_if_no_ephemerides():
    """Skip test if JPL ephemerides are not available."""
    import pytest

    try:
        from jplephem.spk import SPK

        # Try to load a common ephemeris file
        return pytest.mark.skipif(False, reason="")
    except (ImportError, FileNotFoundError):
        return pytest.mark.skip(reason="JPL ephemerides not available")


def skip_if_slow(reason: str = "Test is too slow for regular runs"):
    """Skip test if running quick test suite."""
    import pytest

    return pytest.mark.slow
