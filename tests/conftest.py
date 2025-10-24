"""
Pytest configuration and shared fixtures for astrora test suite.

This file provides:
1. Common orbit fixtures (LEO, MEO, GEO, HEO, lunar, interplanetary)
2. Celestial body fixtures with standard gravitational parameters
3. State vector fixtures for various scenarios
4. Time/epoch fixtures
5. Numerical tolerance fixtures
6. Test markers and hooks
7. Reference data fixtures from test_reference_data.py
"""

from typing import Dict

import numpy as np
import pytest

# Import astrora modules
try:
    import astrora._core as core
    from astrora._core import (
        CartesianState,
        Duration,
        Epoch,
        OrbitalElements,
        constants,
    )

    ASTRORA_AVAILABLE = True
except ImportError:
    ASTRORA_AVAILABLE = False
    pytest.skip("astrora not available", allow_module_level=True)

# Register test_reference_data module to make fixtures available
# This is the pytest-recommended way to share fixtures across modules
pytest_plugins = ["tests.test_reference_data"]


# ============================================================================
# Test Configuration Fixtures
# ============================================================================


@pytest.fixture(scope="session")
def numerical_tolerances() -> Dict[str, float]:
    """
    Standard numerical tolerances for different test categories.

    Returns:
        Dictionary of tolerance values for various test types
    """
    return {
        "position_m": 1e-6,  # 1 micrometer position tolerance
        "velocity_m_s": 1e-9,  # 1 nanometer/second velocity tolerance
        "angle_rad": 1e-12,  # Sub-milliarcsecond angular tolerance
        "energy_relative": 1e-10,  # Energy conservation tolerance
        "momentum_relative": 1e-10,  # Angular momentum conservation tolerance
        "time_s": 1e-9,  # 1 nanosecond time tolerance
        "mass_kg": 1e-12,  # Mass tolerance
        # Looser tolerances for integration tests
        "integration_position_m": 1e-3,  # 1 mm for numerical integration
        "integration_velocity_m_s": 1e-6,  # 1 micrometer/s for integration
        # Very loose for validation against external tools
        "validation_position_m": 1.0,  # 1 meter for GMAT/STK comparison
        "validation_velocity_m_s": 1e-3,  # 1 mm/s for external validation
    }


@pytest.fixture(scope="session")
def standard_epochs() -> Dict[str, Epoch]:
    """
    Standard test epochs for various scenarios.

    Returns:
        Dictionary of commonly used epoch values
    """
    return {
        "j2000": Epoch.j2000_epoch(),  # J2000.0 epoch
        "gps_epoch": Epoch.from_midnight_utc(1980, 1, 6),  # GPS epoch
        "unix_epoch": Epoch.from_midnight_utc(1970, 1, 1),  # Unix epoch
        "year_2025": Epoch.from_midnight_utc(2025, 1, 1),
        "year_2030": Epoch.from_midnight_utc(2030, 1, 1),
    }


# ============================================================================
# Celestial Body Fixtures
# ============================================================================


@pytest.fixture(scope="session")
def earth_params() -> Dict[str, float]:
    """Standard Earth parameters for testing."""
    return {
        "gm": constants.GM_EARTH,  # m³/s²
        "radius": constants.R_EARTH,  # m
        "j2": constants.J2_EARTH,
        "angular_velocity": 7.292115e-5,  # rad/s
    }


@pytest.fixture(scope="session")
def sun_params() -> Dict[str, float]:
    """Standard Sun parameters for testing."""
    return {
        "gm": constants.GM_SUN,  # m³/s²
        "radius": constants.R_SUN,  # m
    }


@pytest.fixture(scope="session")
def moon_params() -> Dict[str, float]:
    """Standard Moon parameters for testing."""
    return {
        "gm": constants.GM_MOON,  # m³/s²
        "radius": constants.R_MOON,  # m
    }


# ============================================================================
# Orbit Regime Fixtures - State Vectors
# ============================================================================


@pytest.fixture
def leo_state(earth_params: Dict[str, float]) -> CartesianState:
    """
    Low Earth Orbit (LEO) state vector.

    Circular orbit at 400 km altitude (typical ISS orbit).
    """
    gm = earth_params["gm"]
    r_earth = earth_params["radius"]
    altitude = 400000.0  # 400 km
    r = r_earth + altitude
    v = np.sqrt(gm / r)

    return CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])


@pytest.fixture
def meo_state(earth_params: Dict[str, float]) -> CartesianState:
    """
    Medium Earth Orbit (MEO) state vector.

    Circular orbit at 20,200 km altitude (typical GPS orbit).
    """
    gm = earth_params["gm"]
    r_earth = earth_params["radius"]
    altitude = 20200000.0  # 20,200 km
    r = r_earth + altitude
    v = np.sqrt(gm / r)

    return CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])


@pytest.fixture
def geo_state(earth_params: Dict[str, float]) -> CartesianState:
    """
    Geostationary Orbit (GEO) state vector.

    Circular equatorial orbit at ~35,786 km altitude.
    """
    gm = earth_params["gm"]
    # Geostationary radius
    r = (gm / (earth_params["angular_velocity"] ** 2)) ** (1 / 3)
    v = np.sqrt(gm / r)

    return CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])


@pytest.fixture
def gto_state(earth_params: Dict[str, float]) -> CartesianState:
    """
    Geostationary Transfer Orbit (GTO) state vector.

    Elliptical orbit with perigee at 300 km and apogee at GEO altitude.
    """
    gm = earth_params["gm"]
    r_earth = earth_params["radius"]
    r_perigee = r_earth + 300000.0  # 300 km
    r_apogee = (gm / (earth_params["angular_velocity"] ** 2)) ** (1 / 3)

    a = (r_perigee + r_apogee) / 2
    e = (r_apogee - r_perigee) / (r_apogee + r_perigee)

    # Velocity at perigee
    v = np.sqrt(gm * (2 / r_perigee - 1 / a))

    return CartesianState([r_perigee, 0.0, 0.0], [0.0, v, 0.0])


@pytest.fixture
def heo_state(earth_params: Dict[str, float]) -> CartesianState:
    """
    Highly Elliptical Orbit (HEO) state vector.

    Molniya-type orbit: e=0.74, i=63.4°, period=12h.
    """
    gm = earth_params["gm"]
    r_earth = earth_params["radius"]

    # Molniya orbit parameters
    period = 43200.0  # 12 hours in seconds
    a = (gm * (period / (2 * np.pi)) ** 2) ** (1 / 3)
    e = 0.74
    i = np.radians(63.4)

    # State at perigee
    r_perigee = a * (1 - e)
    v_perigee = np.sqrt(gm * (2 / r_perigee - 1 / a))

    # Apply inclination
    return CartesianState(
        [r_perigee, 0.0, 0.0], [0.0, v_perigee * np.cos(i), v_perigee * np.sin(i)]
    )


@pytest.fixture
def sso_state(earth_params: Dict[str, float]) -> CartesianState:
    """
    Sun-Synchronous Orbit (SSO) state vector.

    Circular orbit at 800 km altitude with 98° inclination.
    """
    gm = earth_params["gm"]
    r_earth = earth_params["radius"]
    altitude = 800000.0  # 800 km
    r = r_earth + altitude
    v = np.sqrt(gm / r)
    i = np.radians(98.0)

    return CartesianState([r, 0.0, 0.0], [0.0, v * np.cos(i), v * np.sin(i)])


@pytest.fixture
def lunar_transfer_state(earth_params: Dict[str, float]) -> CartesianState:
    """
    Lunar transfer orbit state vector.

    Initial state for a Hohmann-like transfer to lunar distance.
    """
    gm = earth_params["gm"]
    r_earth = earth_params["radius"]
    r_moon = 384400000.0  # meters (lunar distance)

    # Hohmann transfer from LEO to lunar distance
    r_departure = r_earth + 200000.0  # 200 km altitude
    a = (r_departure + r_moon) / 2
    v = np.sqrt(gm * (2 / r_departure - 1 / a))

    return CartesianState([r_departure, 0.0, 0.0], [0.0, v, 0.0])


# ============================================================================
# Orbit Regime Fixtures - Orbital Elements
# ============================================================================


@pytest.fixture
def circular_equatorial_elements(earth_params: Dict[str, float]) -> OrbitalElements:
    """Circular equatorial orbit elements (e=0, i=0)."""
    r_earth = earth_params["radius"]
    a = r_earth + 500000.0  # 500 km altitude

    return OrbitalElements(a=a, e=0.0, i=0.0, raan=0.0, argp=0.0, nu=0.0)


@pytest.fixture
def eccentric_inclined_elements(earth_params: Dict[str, float]) -> OrbitalElements:
    """Eccentric inclined orbit elements (e=0.3, i=45°)."""
    r_earth = earth_params["radius"]
    a = r_earth + 10000000.0  # High altitude

    return OrbitalElements(
        a=a,
        e=0.3,
        i=np.radians(45.0),
        raan=np.radians(30.0),
        argp=np.radians(60.0),
        nu=np.radians(90.0),
    )


@pytest.fixture
def polar_orbit_elements(earth_params: Dict[str, float]) -> OrbitalElements:
    """Polar orbit elements (i=90°)."""
    r_earth = earth_params["radius"]
    a = r_earth + 700000.0  # 700 km altitude

    return OrbitalElements(a=a, e=0.001, i=np.radians(90.0), raan=0.0, argp=0.0, nu=0.0)


# ============================================================================
# Test Data Arrays
# ============================================================================


@pytest.fixture
def test_true_anomalies() -> np.ndarray:
    """Array of test true anomaly values covering full orbit."""
    return np.linspace(0, 2 * np.pi, 100)


@pytest.fixture
def test_eccentricities() -> np.ndarray:
    """Array of test eccentricity values (circular to highly elliptical)."""
    return np.array([0.0, 0.1, 0.3, 0.5, 0.7, 0.9, 0.95, 0.99])


@pytest.fixture
def test_inclinations() -> np.ndarray:
    """Array of test inclination values (equatorial to polar)."""
    return np.radians([0, 15, 30, 45, 60, 75, 90, 105, 120, 135, 150, 165, 180])


# ============================================================================
# Propagation Time Fixtures
# ============================================================================


@pytest.fixture
def short_propagation_times() -> np.ndarray:
    """Short propagation times for quick tests (0 to 1 orbit)."""
    return np.linspace(0, 6000, 50)  # Up to ~1.7 hours


@pytest.fixture
def medium_propagation_times() -> np.ndarray:
    """Medium propagation times for validation (0 to 1 day)."""
    return np.linspace(0, 86400, 100)  # Up to 1 day


@pytest.fixture
def long_propagation_times() -> np.ndarray:
    """Long propagation times for stability tests (0 to 7 days)."""
    return np.linspace(0, 604800, 200)  # Up to 7 days


# ============================================================================
# Pytest Hooks
# ============================================================================


def pytest_configure(config):
    """Pytest configuration hook for custom setup."""
    # Register custom markers dynamically if needed
    pass


def pytest_collection_modifyitems(config, items):
    """
    Automatically mark tests based on their module and name.

    This hook automatically applies markers to tests based on patterns:
    - Files starting with 'benchmark_' get the 'benchmark' marker
    - Files containing 'validation' get the 'validation' marker
    - Files containing 'integration' get the 'integration' marker
    - Tests taking > 1 second get the 'slow' marker (via pytest-timeout)
    """
    for item in items:
        # Mark benchmark files
        if "benchmark_" in str(item.fspath):
            item.add_marker(pytest.mark.benchmark)

        # Mark validation tests
        if "validation" in str(item.fspath):
            item.add_marker(pytest.mark.validation)

        # Mark integration tests
        if "integration" in str(item.fspath) or "_integration" in str(item.fspath):
            item.add_marker(pytest.mark.integration)

        # Mark regression tests
        if "regression" in str(item.fspath):
            item.add_marker(pytest.mark.regression)

        # Mark domain-specific tests based on file names
        fspath_str = str(item.fspath)
        if "propagat" in fspath_str:
            item.add_marker(pytest.mark.propagation)
        if "coordinate" in fspath_str or "frame" in fspath_str or "transform" in fspath_str:
            item.add_marker(pytest.mark.coordinates)
        if "maneuver" in fspath_str or "transfer" in fspath_str:
            item.add_marker(pytest.mark.maneuvers)
        if (
            "perturbation" in fspath_str
            or "_j2" in fspath_str
            or "drag" in fspath_str
            or "srp" in fspath_str
        ):
            item.add_marker(pytest.mark.perturbations)
        if "satellite" in fspath_str or "sgp4" in fspath_str or "tle" in fspath_str:
            item.add_marker(pytest.mark.satellite)
        if "plot" in fspath_str or "animation" in fspath_str:
            item.add_marker(pytest.mark.plotting)


def pytest_report_header(config):
    """Add custom header information to pytest output."""
    return [
        f"astrora version: {core.__version__ if hasattr(core, '__version__') else 'unknown'}",
        f"NumPy version: {np.__version__}",
    ]


# ============================================================================
# Benchmark Fixtures (for pytest-benchmark)
# ============================================================================


@pytest.fixture
def benchmark_arrays():
    """Standard array sizes for benchmarking."""
    return {
        "tiny": 10,
        "small": 100,
        "medium": 1000,
        "large": 10000,
        "huge": 100000,
    }
