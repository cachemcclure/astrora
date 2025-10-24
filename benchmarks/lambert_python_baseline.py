"""
Pure Python/NumPy baseline implementation of Lambert's problem solver.

This serves as a performance baseline for comparing against the Rust implementation.
Uses the universal variable formulation with Newton-Raphson iteration.

References:
- Curtis, H. D. (2013). Orbital Mechanics for Engineering Students. Ch. 5
- Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications. Ch. 7
"""

from typing import Tuple

import numpy as np


def stumpff_c(z: float) -> float:
    """
    Stumpff function C(z).

    Generalizes cosine for elliptic (z > 0), parabolic (z ≈ 0),
    and hyperbolic (z < 0) orbits.
    """
    TOL = 1e-6

    if z > TOL:
        # Elliptic
        sqrt_z = np.sqrt(z)
        return (1.0 - np.cos(sqrt_z)) / z
    elif z < -TOL:
        # Hyperbolic
        sqrt_neg_z = np.sqrt(-z)
        return (np.cosh(sqrt_neg_z) - 1.0) / (-z)
    else:
        # Parabolic - series expansion
        return 0.5 - z / 24.0 + z * z / 720.0


def stumpff_s(z: float) -> float:
    """
    Stumpff function S(z).

    Generalizes sine for elliptic (z > 0), parabolic (z ≈ 0),
    and hyperbolic (z < 0) orbits.
    """
    TOL = 1e-6

    if z > TOL:
        # Elliptic
        sqrt_z = np.sqrt(z)
        return (sqrt_z - np.sin(sqrt_z)) / (z * sqrt_z)
    elif z < -TOL:
        # Hyperbolic
        sqrt_neg_z = np.sqrt(-z)
        return (np.sinh(sqrt_neg_z) - sqrt_neg_z) / ((-z) * sqrt_neg_z)
    else:
        # Parabolic - series expansion
        return 1.0 / 6.0 - z / 120.0 + z * z / 5040.0


def lambert_universal_variable(
    r1: np.ndarray,
    r2: np.ndarray,
    tof: float,
    mu: float,
    short_way: bool = True,
    max_iter: int = 100,
    tol: float = 1e-8,
) -> Tuple[np.ndarray, np.ndarray]:
    """
    Solve Lambert's problem using universal variable formulation.

    Parameters
    ----------
    r1 : np.ndarray
        Initial position vector (m)
    r2 : np.ndarray
        Final position vector (m)
    tof : float
        Time of flight (s)
    mu : float
        Gravitational parameter (m³/s²)
    short_way : bool
        True for short-way transfer, False for long-way
    max_iter : int
        Maximum Newton-Raphson iterations
    tol : float
        Convergence tolerance

    Returns
    -------
    v1, v2 : Tuple[np.ndarray, np.ndarray]
        Initial and final velocity vectors (m/s)

    Raises
    ------
    ValueError
        If solver fails to converge or inputs are invalid
    """
    # Calculate position magnitudes
    r1_mag = np.linalg.norm(r1)
    r2_mag = np.linalg.norm(r2)

    if r1_mag < 1.0 or r2_mag < 1.0:
        raise ValueError("Position magnitudes must be > 1 m")

    if tof <= 0.0:
        raise ValueError("Time of flight must be positive")

    if mu <= 0.0:
        raise ValueError("Gravitational parameter must be positive")

    # Calculate geometric parameters
    cos_dnu = np.dot(r1, r2) / (r1_mag * r2_mag)
    cross = np.cross(r1, r2)
    cross_mag = np.linalg.norm(cross)

    # Determine transfer direction
    if short_way:
        sin_dnu = cross_mag / (r1_mag * r2_mag)
    else:
        sin_dnu = -cross_mag / (r1_mag * r2_mag)

    # Calculate A parameter (Curtis Eq. 5.35)
    a_param = np.sqrt(r1_mag * r2_mag * (1.0 + cos_dnu))

    if abs(a_param) < 1e-6:
        raise ValueError("Position vectors are nearly opposite")

    # Initial guess for universal variable z
    z = 0.0

    # Newton-Raphson iteration
    converged = False

    for iteration in range(max_iter):
        c2 = stumpff_c(z)
        c3 = stumpff_s(z)

        # Calculate y(z) - Curtis Eq. 5.38
        y = r1_mag + r2_mag + a_param * (z * c3 - 1.0) / np.sqrt(c2)

        if y <= 0.0:
            # Negative y, adjust z
            z += 0.1
            continue

        # Calculate chi(z) - universal anomaly
        chi = np.sqrt(y / c2)

        # Time of flight equation - Curtis Eq. 5.40
        tof_calc = (chi**3 * c3 + a_param * np.sqrt(y)) / np.sqrt(mu)

        # Check convergence
        error = tof - tof_calc
        if abs(error) < tol:
            converged = True
            break

        # Newton-Raphson derivative - Curtis Eq. 5.43
        if abs(z) < 1e-6:
            # Near-parabolic case
            dt_dz = (chi**3 / 40.0 + a_param / 8.0) / np.sqrt(mu)
        else:
            # General case - numerical derivative for simplicity
            dz = 1e-8
            c2_plus = stumpff_c(z + dz)
            c3_plus = stumpff_s(z + dz)
            y_plus = r1_mag + r2_mag + a_param * ((z + dz) * c3_plus - 1.0) / np.sqrt(c2_plus)
            chi_plus = np.sqrt(y_plus / c2_plus) if y_plus > 0 else chi
            tof_plus = (chi_plus**3 * c3_plus + a_param * np.sqrt(y_plus)) / np.sqrt(mu)
            dt_dz = (tof_plus - tof_calc) / dz

        if abs(dt_dz) < 1e-15:
            raise ValueError("Derivative too small, cannot continue")

        # Newton-Raphson update
        z_new = z + error / dt_dz

        # Prevent oscillation
        if iteration > 10 and abs(z_new - z) < 1e-12:
            converged = True
            break

        z = z_new

    if not converged:
        raise ValueError(f"Lambert solver failed to converge after {max_iter} iterations")

    # Calculate final velocities using Lagrange coefficients
    c2 = stumpff_c(z)
    c3 = stumpff_s(z)
    y = r1_mag + r2_mag + a_param * (z * c3 - 1.0) / np.sqrt(c2)

    # Lagrange coefficients - Curtis Eq. 5.28-5.31
    f = 1.0 - y / r1_mag
    g = a_param * np.sqrt(y) / np.sqrt(mu)
    g_dot = 1.0 - y / r2_mag

    # Velocities
    v1 = (r2 - f * r1) / g
    v2 = (g_dot * r2 - r1) / g

    return v1, v2


def lambert_batch(
    r1: np.ndarray,
    r2: np.ndarray,
    tofs: np.ndarray,
    mu: float,
    short_way: bool = True,
) -> Tuple[np.ndarray, np.ndarray]:
    """
    Solve Lambert's problem for a batch of time-of-flight values.

    This is the Python baseline for porkchop plot generation.

    Parameters
    ----------
    r1 : np.ndarray
        Initial position vector (m)
    r2 : np.ndarray
        Final position vector (m)
    tofs : np.ndarray
        Array of time of flight values (s)
    mu : float
        Gravitational parameter (m³/s²)
    short_way : bool
        True for short-way transfer

    Returns
    -------
    v1s, v2s : Tuple[np.ndarray, np.ndarray]
        Arrays of initial and final velocity vectors (m/s)
    """
    n = len(tofs)
    v1s = np.zeros((n, 3))
    v2s = np.zeros((n, 3))

    for i, tof in enumerate(tofs):
        try:
            v1s[i], v2s[i] = lambert_universal_variable(r1, r2, tof, mu, short_way)
        except ValueError:
            # Failed to converge, set to NaN
            v1s[i] = np.nan
            v2s[i] = np.nan

    return v1s, v2s


# Optional: Try to import poliastro/hapsira for comparison
try:
    from poliastro.iod import izzo as poliastro_lambert

    POLIASTRO_AVAILABLE = True
except ImportError:
    try:
        from hapsira.iod import izzo as poliastro_lambert

        POLIASTRO_AVAILABLE = True
    except ImportError:
        POLIASTRO_AVAILABLE = False


def lambert_poliastro(
    r1: np.ndarray,
    r2: np.ndarray,
    tof: float,
    mu: float,
    short_way: bool = True,
) -> Tuple[np.ndarray, np.ndarray]:
    """
    Wrapper for poliastro/hapsira Lambert solver (if available).

    This provides a comparison point to established Python implementations.
    """
    if not POLIASTRO_AVAILABLE:
        raise ImportError("poliastro or hapsira not installed")

    # poliastro uses different parameter ordering
    M = 0  # Number of revolutions
    v1, v2 = poliastro_lambert(mu, r1, r2, tof, M=M, prograde=short_way)

    return np.array(v1), np.array(v2)


if __name__ == "__main__":
    # Quick validation test
    EARTH_MU = 3.986004418e14

    # LEO quarter-orbit transfer
    r_leo = 7000e3
    r1 = np.array([r_leo, 0.0, 0.0])
    r2 = np.array([0.0, r_leo, 0.0])

    period = 2.0 * np.pi * (r_leo**3 / EARTH_MU) ** 0.5
    tof = period / 4.0

    print("Python Lambert Baseline Implementation")
    print("=" * 60)
    print(f"Test: LEO quarter-orbit transfer")
    print(f"r1 = {r1 / 1e3} km")
    print(f"r2 = {r2 / 1e3} km")
    print(f"TOF = {tof:.2f} s ({tof / 3600:.4f} hours)")
    print()

    v1, v2 = lambert_universal_variable(r1, r2, tof, EARTH_MU)

    print(f"v1 = {v1 / 1e3} km/s (magnitude: {np.linalg.norm(v1) / 1e3:.4f} km/s)")
    print(f"v2 = {v2 / 1e3} km/s (magnitude: {np.linalg.norm(v2) / 1e3:.4f} km/s)")
    print()

    # Verify it's roughly circular velocity
    v_circular = np.sqrt(EARTH_MU / r_leo) / 1e3
    print(f"Circular velocity at {r_leo / 1e3:.0f} km: {v_circular:.4f} km/s")
    print(f"Ratio v1/v_circ: {np.linalg.norm(v1) / 1e3 / v_circular:.4f}")
    print()

    if POLIASTRO_AVAILABLE:
        print("Comparing with poliastro/hapsira...")
        v1_p, v2_p = lambert_poliastro(r1, r2, tof, EARTH_MU)
        print(f"poliastro v1 = {v1_p / 1e3} km/s")
        print(f"Difference: {np.linalg.norm(v1 - v1_p):.6e} m/s")
    else:
        print("poliastro/hapsira not available for comparison")
