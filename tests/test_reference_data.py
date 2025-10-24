"""
Reference test data ported from original poliastro repository.

This module contains validated reference values from authoritative sources
including textbooks (Vallado, Curtis), academic literature, and NASA mission data.
These values serve as ground truth for validating astrora's implementations.

References:
1. Vallado, D. A. "Fundamentals of Astrodynamics and Applications", 4th Ed.
2. Curtis, H. D. "Orbital Mechanics for Engineering Students", 3rd Ed.
3. Original poliastro test suite (archived Oct 2023)
4. NASA/JPL mission data

All units are SI unless otherwise noted:
- Distance: meters (m)
- Velocity: meters per second (m/s)
- Time: seconds (s)
- Angles: radians (rad)
- Gravitational parameter: m³/s²
"""

from typing import Any, Dict

import numpy as np
import pytest

# =============================================================================
# Time and Epoch References
# =============================================================================

# Reference epochs from poliastro conftest.py
REFERENCE_EPOCHS = {
    "earth_perihelion_2020": "2020-01-05 07:47:00",  # TDB scale
    "hyperbolic_epoch": "2015-07-14 07:59",  # TDB scale (New Horizons)
}


# =============================================================================
# Celestial Body Parameters
# =============================================================================

# From poliastro test_bodies.py
CELESTIAL_BODIES = {
    "earth": {
        "gm": 3.986004418e14,  # m³/s²
        "radius": 6_371_000.0,  # m (mean)
        "angular_velocity": 7.292114e-5,  # rad/s
        "j2": 1.08262668e-3,  # dimensionless
        "name": "Earth",
    },
    "jupiter": {
        "gm": 1.26712763e17,  # m³/s²
        "radius": 71_492_000.0,  # m
        "name": "Jupiter",
    },
    "sun": {
        "gm": 1.32712440018e20,  # m³/s²
        "radius": 695_700_000.0,  # m
        "name": "Sun",
    },
    "mars": {
        "gm": 4.282837e13,  # m³/s² (approximate)
        "radius": 3_389_500.0,  # m
        "name": "Mars",
    },
}


# =============================================================================
# Vallado Example 2.4: Orbit Propagation
# =============================================================================

# Source: Vallado, "Fundamentals of Astrodynamics and Applications", 4th Ed.
# Example 2.4: Propagate an Earth orbit for 40 minutes
VALLADO_EXAMPLE_2_4 = {
    "description": "Vallado Example 2.4: Orbit propagation",
    "source": "Vallado 4th Ed., Example 2.4",
    "attractor": "earth",
    "initial_state": {
        "r": np.array([1131.340e3, -2282.343e3, 6672.423e3]),  # m
        "v": np.array([-5.64305e3, 4.30333e3, 2.42879e3]),  # m/s
        "description": "Initial position and velocity in ECI frame",
    },
    "time_of_flight": 40.0 * 60.0,  # 40 minutes in seconds
    "expected_final_state": {
        "r": np.array([-4219.7527e3, 4363.0292e3, -3958.7666e3]),  # m
        "v": np.array([3.689866e3, -1.916735e3, -6.112511e3]),  # m/s
        "description": "Expected final state after 40 minutes",
    },
    "tolerance": {
        "position": 1.0,  # m (textbook precision)
        "velocity": 1e-3,  # m/s
    },
}


# =============================================================================
# Curtis Example 4.3: State Vectors to Orbital Elements
# =============================================================================

# Source: Curtis, "Orbital Mechanics for Engineering Students", 3rd Ed.
# Example 4.3, page 200: Convert r,v to classical orbital elements
CURTIS_EXAMPLE_4_3 = {
    "description": "Curtis Example 4.3: State vectors to orbital elements",
    "source": "Curtis 3rd Ed., Example 4.3, p. 200",
    "attractor": "earth",
    "state": {
        "r": np.array([-6045.0e3, -3490.0e3, 2500.0e3]),  # m
        "v": np.array([-3.457e3, 6.618e3, 2.533e3]),  # m/s
    },
    "expected_elements": {
        "e": 0.1712,  # eccentricity
        "i": 153.25,  # degrees
        "raan": 255.28,  # degrees (Ω)
        "argp": 20.07,  # degrees (ω)
        "nu": 28.45,  # degrees (true anomaly)
        "p": 8530.47e3,  # m (semi-latus rectum)
    },
    "tolerance": {
        "e": 0.001,
        "angles_deg": 0.1,  # 0.1 degree tolerance
        "p": 100.0,  # 100 m
    },
}


# =============================================================================
# Curtis Example 3.5: Hyperbolic Orbit Propagation
# =============================================================================

# Source: Curtis 3rd Ed., Example 3.5
# Hyperbolic escape trajectory from Earth
CURTIS_EXAMPLE_3_5 = {
    "description": "Curtis Example 3.5: Hyperbolic orbit propagation",
    "source": "Curtis 3rd Ed., Example 3.5",
    "attractor": "earth",
    "initial_state": {
        "r": np.array([6_671_000.0, 0.0, 0.0]),  # R_earth + 300 km
        "v": np.array([0.0, 15.0e3, 0.0]),  # 15 km/s (hyperbolic)
    },
    "time_of_flight": 14941.0,  # seconds
    "expected_final_state": {
        "r_magnitude": 163_180e3,  # m
        "v_magnitude": 10.51e3,  # m/s
    },
    "tolerance": {
        "position": 1000.0,  # 1 km
        "velocity": 10.0,  # m/s
    },
}


# =============================================================================
# Curtis Problem 3.15: Parabolic Orbit
# =============================================================================

# Source: Curtis 3rd Ed., Problem 3.15
# Parabolic trajectory (e = 1.0)
CURTIS_PROBLEM_3_15 = {
    "description": "Curtis Problem 3.15: Parabolic orbit",
    "source": "Curtis 3rd Ed., Problem 3.15",
    "attractor": "earth",
    "semi_latus_rectum": 13_200e3,  # m
    "eccentricity": 1.0,  # parabolic
    "test_cases": [
        {
            "time": 0.44485 * 3600.0,  # hours to seconds
            "expected_nu_deg": 90.0,  # true anomaly in degrees
        },
        {
            "time": 36.0 * 3600.0,  # 36 hours
            "expected_r": 304_700e3,  # m
        },
    ],
}


# =============================================================================
# Vallado75: Lambert Problem
# =============================================================================

# Source: Vallado reference material (test case 75)
VALLADO75_LAMBERT = {
    "description": "Vallado test case 75: Lambert problem",
    "source": "Vallado reference implementation, test 75",
    "attractor": "earth",
    "r1": np.array([15945.34e3, 0.0, 0.0]),  # m (initial position)
    "r2": np.array([12214.83399e3, 10249.46731e3, 0.0]),  # m (final position)
    "time_of_flight": 76.0 * 60.0,  # 76 minutes in seconds
    "expected_velocities": {
        "v1": np.array([2.058925e3, 2.915956e3, 0.0]),  # m/s (initial velocity)
        "v2": np.array([-3.451569e3, 0.910301e3, 0.0]),  # m/s (final velocity)
    },
    "tolerance": {
        "velocity": 1.0,  # m/s (reference precision)
    },
}


# =============================================================================
# Curtis52: Lambert Problem
# =============================================================================

# Source: Curtis reference material (test case 52)
CURTIS52_LAMBERT = {
    "description": "Curtis test case 52: Lambert problem",
    "source": "Curtis reference implementation, test 52",
    "attractor": "earth",
    "r1": np.array([5000.0e3, 10000.0e3, 2100.0e3]),  # m
    "r2": np.array([-14600.0e3, 2500.0e3, 7000.0e3]),  # m
    "time_of_flight": 1.0 * 3600.0,  # 1 hour in seconds
    "expected_velocities": {
        "v1": np.array([-5.9925e3, 1.9254e3, 3.2456e3]),  # m/s
        "v2": np.array([-3.3125e3, -4.1966e3, -0.38529e3]),  # m/s
    },
    "tolerance": {
        "velocity": 1.0,  # m/s
    },
}


# =============================================================================
# Curtis53: Lambert Problem
# =============================================================================

# Source: Curtis reference material (test case 53)
# Note: Errata mentions positive j-component
CURTIS53_LAMBERT = {
    "description": "Curtis test case 53: Lambert problem (with errata)",
    "source": "Curtis reference implementation, test 53",
    "attractor": "earth",
    "r1": np.array([273378.0e3, 0.0, 0.0]),  # m (high altitude)
    "r2": np.array([145820.0e3, 12758.0e3, 0.0]),  # m
    "time_of_flight": 13.5 * 3600.0,  # 13.5 hours in seconds
    "expected_velocities": {
        "v1": np.array([-2.4356e3, 0.26741e3, 0.0]),  # m/s (note positive j)
        # v2 not provided in reference
    },
    "tolerance": {
        "velocity": 1.0,  # m/s
    },
    "notes": "Errata notes positive j-component in v1",
}


# =============================================================================
# Hohmann Transfer: LEO to GEO
# =============================================================================

# Standard LEO to GEO Hohmann transfer (from poliastro test_maneuver.py)
HOHMANN_LEO_TO_GEO = {
    "description": "Hohmann transfer from LEO to GEO",
    "source": "poliastro test suite, standard reference",
    "attractor": "earth",
    "initial_altitude": 191.34411e3,  # m (LEO)
    "final_altitude": 35_781.34857e3,  # m (GEO)
    "expected_results": {
        "delta_v_total": 3.935224e3,  # m/s
        "transfer_time": 5.256713 * 3600.0,  # hours to seconds
        "final_eccentricity": 0.0,  # circular (tolerance 1e-14)
    },
    "tolerance": {
        "delta_v": 10.0,  # m/s (10 m/s tolerance)
        "time": 60.0,  # seconds
        "eccentricity": 1e-12,
    },
}


# =============================================================================
# Bielliptic Transfer
# =============================================================================

# Bielliptic transfer example (from poliastro test_maneuver.py)
BIELLIPTIC_TRANSFER = {
    "description": "Bielliptic transfer maneuver",
    "source": "poliastro test suite",
    "attractor": "earth",
    "initial_altitude": 191.34411e3,  # m (LEO)
    "intermediate_altitude": 503_873.0e3,  # m (apogee altitude)
    "final_altitude": 376_310.0e3,  # m (final altitude)
    "expected_results": {
        "delta_v_total": 3.904057e3,  # m/s
        "transfer_time": 593.919803 * 3600.0,  # hours to seconds
        "final_eccentricity": 0.0,  # circular (tolerance 1e-12)
    },
    "tolerance": {
        "delta_v": 10.0,  # m/s
        "time": 100.0,  # seconds
        "eccentricity": 1e-12,
    },
}


# =============================================================================
# Circular Velocity Calculation
# =============================================================================

# From poliastro test_elements.py
CIRCULAR_VELOCITY_TEST = {
    "description": "Circular velocity calculation",
    "source": "poliastro test suite, basic validation",
    "gm": 398600e9,  # m³/s² (398600 km³/s² in original)
    "semi_major_axis": 7000e3,  # m (7000 km)
    "expected_velocity": 7546.0491,  # m/s (7 decimal precision in original)
    "tolerance": 0.001,  # m/s
    "formula": "V = sqrt(GM/a)",
}


# =============================================================================
# New Horizons Hyperbolic Departure
# =============================================================================

# From poliastro conftest.py (Sun-centered hyperbolic orbit)
NEW_HORIZONS_HYPERBOLIC = {
    "description": "New Horizons hyperbolic departure",
    "source": "poliastro conftest.py, New Horizons mission data",
    "epoch": "2015-07-14 07:59",  # TDB
    "attractor": "sun",
    "state": {
        "r": np.array([1.197659243752796e9, -4.443716685978071e9, -1.747610548576734e9]),  # m
        "v": np.array([5.540549267188614e3, -12.51544669134140e3, -4.848892572767733e3]),  # m/s
    },
    "orbit_type": "hyperbolic",
    "notes": "Sun-centered state at Pluto flyby approach",
}


# =============================================================================
# Near-Parabolic Test Cases
# =============================================================================

# From poliastro test_propagation.py
# Used to test numerical stability near parabolic (e ≈ 1.0)
NEAR_PARABOLIC_ECCENTRICITIES = {
    "description": "Near-parabolic orbit test eccentricities",
    "source": "poliastro test_propagation.py",
    "elliptic": [0.9, 0.99, 0.999, 0.9999, 0.99999],
    "hyperbolic": [1.0001, 1.001, 1.01, 1.1],
    "notes": "Used for numerical stability validation against Cowell propagator",
}


# =============================================================================
# Fixture: All Reference Data
# =============================================================================


@pytest.fixture(scope="session")
def all_reference_data() -> Dict[str, Any]:
    """
    Comprehensive fixture providing all reference test data.

    Returns:
        Dictionary containing all reference test cases organized by category.
    """
    return {
        "epochs": REFERENCE_EPOCHS,
        "bodies": CELESTIAL_BODIES,
        "propagation": {
            "vallado_2_4": VALLADO_EXAMPLE_2_4,
            "curtis_3_5": CURTIS_EXAMPLE_3_5,
            "curtis_3_15": CURTIS_PROBLEM_3_15,
        },
        "state_conversion": {
            "curtis_4_3": CURTIS_EXAMPLE_4_3,
        },
        "lambert": {
            "vallado75": VALLADO75_LAMBERT,
            "curtis52": CURTIS52_LAMBERT,
            "curtis53": CURTIS53_LAMBERT,
        },
        "maneuvers": {
            "hohmann_leo_geo": HOHMANN_LEO_TO_GEO,
            "bielliptic": BIELLIPTIC_TRANSFER,
        },
        "basic": {
            "circular_velocity": CIRCULAR_VELOCITY_TEST,
        },
        "mission_data": {
            "new_horizons": NEW_HORIZONS_HYPERBOLIC,
        },
        "stability": {
            "near_parabolic": NEAR_PARABOLIC_ECCENTRICITIES,
        },
    }


# =============================================================================
# Individual Fixtures for Convenience
# =============================================================================


@pytest.fixture(scope="session")
def vallado_2_4():
    """Vallado Example 2.4: Orbit propagation."""
    return VALLADO_EXAMPLE_2_4


@pytest.fixture(scope="session")
def curtis_4_3():
    """Curtis Example 4.3: State vectors to orbital elements."""
    return CURTIS_EXAMPLE_4_3


@pytest.fixture(scope="session")
def curtis_3_5():
    """Curtis Example 3.5: Hyperbolic orbit propagation."""
    return CURTIS_EXAMPLE_3_5


@pytest.fixture(scope="session")
def vallado75_lambert():
    """Vallado test case 75: Lambert problem."""
    return VALLADO75_LAMBERT


@pytest.fixture(scope="session")
def curtis52_lambert():
    """Curtis test case 52: Lambert problem."""
    return CURTIS52_LAMBERT


@pytest.fixture(scope="session")
def curtis53_lambert():
    """Curtis test case 53: Lambert problem."""
    return CURTIS53_LAMBERT


@pytest.fixture(scope="session")
def hohmann_leo_geo():
    """Standard Hohmann transfer from LEO to GEO."""
    return HOHMANN_LEO_TO_GEO


@pytest.fixture(scope="session")
def bielliptic_transfer():
    """Bielliptic transfer reference data."""
    return BIELLIPTIC_TRANSFER


@pytest.fixture(scope="session")
def new_horizons_hyperbolic():
    """New Horizons hyperbolic departure state."""
    return NEW_HORIZONS_HYPERBOLIC


# =============================================================================
# Validation Test: Ensure Data Integrity
# =============================================================================


class TestReferenceDataIntegrity:
    """Validate that reference data is well-formed."""

    def test_all_lambert_cases_have_required_fields(self, all_reference_data):
        """Ensure all Lambert test cases have required fields."""
        lambert_cases = all_reference_data["lambert"]

        for name, case in lambert_cases.items():
            assert "r1" in case, f"{name} missing r1"
            assert "r2" in case, f"{name} missing r2"
            assert "time_of_flight" in case, f"{name} missing time_of_flight"
            assert "attractor" in case, f"{name} missing attractor"
            assert case["r1"].shape == (3,), f"{name} r1 wrong shape"
            assert case["r2"].shape == (3,), f"{name} r2 wrong shape"

    def test_all_bodies_have_gm(self, all_reference_data):
        """Ensure all celestial bodies have gravitational parameter."""
        bodies = all_reference_data["bodies"]

        for name, body in bodies.items():
            assert "gm" in body, f"{name} missing gm"
            assert body["gm"] > 0, f"{name} has non-positive gm"

    def test_propagation_cases_have_states(self, all_reference_data):
        """Ensure propagation test cases have initial states."""
        prop_cases = all_reference_data["propagation"]

        for name, case in prop_cases.items():
            if "initial_state" in case:
                assert "r" in case["initial_state"], f"{name} missing r"
                assert "v" in case["initial_state"], f"{name} missing v"

    def test_tolerances_are_positive(self, all_reference_data):
        """Ensure all tolerance values are positive."""

        def check_tolerances(data):
            if isinstance(data, dict):
                for key, value in data.items():
                    if key == "tolerance" and isinstance(value, dict):
                        for tol_key, tol_val in value.items():
                            assert tol_val > 0, f"Negative tolerance: {tol_key} = {tol_val}"
                    else:
                        check_tolerances(value)

        check_tolerances(all_reference_data)
