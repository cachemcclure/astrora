# Astrora Testing Infrastructure

This document describes the testing infrastructure, conventions, and best practices for the astrora project.

## Table of Contents

- [Quick Start](#quick-start)
- [Test Organization](#test-organization)
- [Test Markers](#test-markers)
- [Shared Fixtures](#shared-fixtures)
- [Test Utilities](#test-utilities)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Benchmarking](#benchmarking)

## Quick Start

```bash
# Install test dependencies
uv pip install -e ".[dev]"

# Run all tests
pytest

# Run only fast tests (skip slow tests)
pytest -m "not slow"

# Run only unit tests
pytest -m unit

# Run benchmarks
pytest -m benchmark --benchmark-only

# Run with coverage
pytest --cov=astrora --cov-report=html
```

## Test Organization

The test suite is organized into several categories:

### Test Types

- **Unit Tests** (`test_*.py`): Fast, isolated tests of individual functions/classes
- **Integration Tests** (`test_*_integration.py`): Tests involving multiple components
- **Validation Tests** (`test_*_validation.py`): Compare against reference implementations (GMAT, STK, etc.)
- **Regression Tests** (`test_regression.py`): Ensure specific bugs don't reappear
- **Benchmarks** (`benchmark_*.py`): Performance measurements using pytest-benchmark

### File Naming Conventions

- `test_<module>.py` - Unit tests for a specific module
- `test_<module>_integration.py` - Integration tests
- `test_<reference>_validation.py` - Validation against external tools
- `benchmark_<feature>.py` - Performance benchmarks
- `conftest.py` - Shared fixtures and pytest configuration
- `test_utils.py` - Common test utility functions

## Test Markers

Markers are used to categorize and selectively run tests. Markers are automatically applied based on file names (see `conftest.py`), but can also be manually added.

### Test Type Markers

```python
@pytest.mark.unit
def test_fast_calculation():
    """Fast, isolated unit test."""
    pass

@pytest.mark.integration
def test_multiple_components():
    """Test involving multiple system components."""
    pass

@pytest.mark.validation
def test_against_gmat():
    """Validation against external reference (GMAT)."""
    pass

@pytest.mark.regression
def test_bug_123_fixed():
    """Ensure bug #123 doesn't return."""
    pass
```

### Performance Markers

```python
@pytest.mark.benchmark
def test_propagation_speed(benchmark):
    """Performance benchmark using pytest-benchmark."""
    benchmark(propagate_orbit, state, times)

@pytest.mark.slow
def test_long_propagation():
    """Test that takes > 1 second."""
    pass

@pytest.mark.very_slow
def test_year_long_propagation():
    """Test that takes > 10 seconds."""
    pass
```

### Domain-Specific Markers

```python
@pytest.mark.propagation
def test_kepler_propagator():
    """Orbit propagation test."""
    pass

@pytest.mark.coordinates
def test_frame_transformation():
    """Coordinate transformation test."""
    pass

@pytest.mark.maneuvers
def test_hohmann_transfer():
    """Orbital maneuver test."""
    pass

@pytest.mark.perturbations
def test_j2_effect():
    """Perturbation model test."""
    pass

@pytest.mark.satellite
def test_sgp4_propagation():
    """Satellite-specific test (TLE, SGP4, etc.)."""
    pass

@pytest.mark.plotting
def test_orbit_visualization():
    """Visualization test."""
    pass
```

### Accuracy Level Markers

```python
@pytest.mark.high_precision
def test_accurate_integration():
    """Test requiring tight numerical tolerances."""
    pass

@pytest.mark.numerical
def test_with_tolerances():
    """Test with numerical approximations."""
    pass
```

### External Dependency Markers

```python
@pytest.mark.requires_ephemerides
def test_planetary_positions():
    """Test requiring JPL ephemerides files."""
    pass

@pytest.mark.requires_internet
def test_download_tle():
    """Test requiring internet connectivity."""
    pass
```

## Running Tests

### Basic Usage

```bash
# Run all tests
pytest

# Run with verbose output
pytest -v

# Run specific test file
pytest tests/test_propagator.py

# Run specific test function
pytest tests/test_propagator.py::test_kepler_propagator

# Run specific test class
pytest tests/test_propagator.py::TestKeplerPropagator
```

### Filtering by Markers

```bash
# Run only unit tests
pytest -m unit

# Run integration and validation tests
pytest -m "integration or validation"

# Skip slow tests
pytest -m "not slow"

# Skip slow and very_slow tests
pytest -m "not (slow or very_slow)"

# Run only propagation-related tests
pytest -m propagation

# Run benchmarks only
pytest -m benchmark --benchmark-only
```

### Coverage Reports

```bash
# Basic coverage report in terminal
pytest --cov=astrora

# Generate HTML coverage report
pytest --cov=astrora --cov-report=html
# Open htmlcov/index.html in browser

# Generate XML coverage report (for CI)
pytest --cov=astrora --cov-report=xml
```

### Parallel Execution

```bash
# Run tests in parallel (using pytest-xdist)
pytest -n auto

# Run with 4 workers
pytest -n 4
```

## Shared Fixtures

The `conftest.py` file provides many shared fixtures to reduce code duplication:

### Celestial Body Parameters

```python
def test_earth_orbit(earth_params):
    """Test using Earth parameters."""
    gm = earth_params["gm"]
    radius = earth_params["radius"]
    # ... test code
```

Available fixtures: `earth_params`, `sun_params`, `moon_params`

### Standard Orbit Regimes

```python
def test_leo_propagation(leo_state):
    """Test using LEO orbit state."""
    propagated = propagate(leo_state, 3600.0)
    # ... test code
```

Available orbit fixtures:
- `leo_state` - Low Earth Orbit (400 km altitude)
- `meo_state` - Medium Earth Orbit (GPS altitude)
- `geo_state` - Geostationary Orbit
- `gto_state` - Geostationary Transfer Orbit
- `heo_state` - Highly Elliptical Orbit (Molniya)
- `sso_state` - Sun-Synchronous Orbit
- `lunar_transfer_state` - Lunar transfer orbit

### Orbital Elements Fixtures

```python
def test_circular_orbit(circular_equatorial_elements):
    """Test using circular equatorial orbit."""
    # ... test code
```

Available element fixtures:
- `circular_equatorial_elements` - e=0, i=0
- `eccentric_inclined_elements` - e=0.3, i=45°
- `polar_orbit_elements` - i=90°

### Test Data Arrays

```python
def test_over_full_orbit(test_true_anomalies):
    """Test at multiple points in orbit."""
    for nu in test_true_anomalies:
        # ... test each anomaly
```

Available array fixtures:
- `test_true_anomalies` - 100 points from 0 to 2π
- `test_eccentricities` - [0.0, 0.1, 0.3, 0.5, 0.7, 0.9, 0.95, 0.99]
- `test_inclinations` - Angles from 0° to 180°

### Propagation Times

```python
def test_short_propagation(short_propagation_times):
    """Test over ~1 orbit."""
    for t in short_propagation_times:
        # ... test each time
```

Available time fixtures:
- `short_propagation_times` - 0 to 6000s (~1.7 hours)
- `medium_propagation_times` - 0 to 86400s (1 day)
- `long_propagation_times` - 0 to 604800s (7 days)

### Numerical Tolerances

```python
def test_with_standard_tolerances(numerical_tolerances):
    """Test using standard tolerance levels."""
    pos_tol = numerical_tolerances["position_m"]
    vel_tol = numerical_tolerances["velocity_m_s"]
    # ... test code
```

Available tolerance keys:
- `position_m` - 1 μm (1e-6 m)
- `velocity_m_s` - 1 nm/s (1e-9 m/s)
- `angle_rad` - 1e-12 rad
- `energy_relative` - 1e-10
- `momentum_relative` - 1e-10
- `integration_position_m` - 1 mm (1e-3 m)
- `validation_position_m` - 1 m

## Test Utilities

The `test_utils.py` module provides helper functions:

### State Comparisons

```python
from tests.test_utils import assert_states_equal

def test_roundtrip_conversion():
    state1 = CartesianState(...)
    elements = state1.to_elements()
    state2 = elements.to_state()

    assert_states_equal(
        state1, state2,
        position_tol=1e-6,
        velocity_tol=1e-9,
        context="roundtrip conversion"
    )
```

### Conservation Law Checks

```python
from tests.test_utils import (
    assert_energy_conserved,
    assert_angular_momentum_conserved
)

def test_energy_conservation():
    initial = CartesianState(...)
    final = propagate(initial, 86400.0)

    assert_energy_conserved(
        initial, final, gm=EARTH_GM,
        rtol=1e-10,
        context="24-hour propagation"
    )
```

### Test Data Generators

```python
from tests.test_utils import generate_test_orbits

def test_many_orbits():
    orbits = generate_test_orbits(n_orbits=100, seed=42)
    for orbit in orbits:
        # ... test each orbit
```

## Writing Tests

### Test Structure

```python
"""
Module docstring explaining what is being tested.
"""

import pytest
import numpy as np
from astrora._core import CartesianState, propagate
from tests.test_utils import assert_states_equal


class TestFeatureName:
    """Group related tests in a class."""

    def test_basic_case(self):
        """Test the simplest case."""
        # Arrange
        state = CartesianState(...)

        # Act
        result = propagate(state, 3600.0)

        # Assert
        assert result.x > 0

    @pytest.mark.slow
    def test_edge_case(self):
        """Test an edge case that takes longer."""
        # ... test code
```

### Parameterized Tests

```python
@pytest.mark.parametrize("eccentricity", [0.0, 0.1, 0.5, 0.9])
def test_various_eccentricities(eccentricity):
    """Test with different eccentricity values."""
    # ... test code

@pytest.mark.parametrize("altitude,regime", [
    (400e3, "LEO"),
    (20200e3, "MEO"),
    (35786e3, "GEO"),
])
def test_orbit_regimes(altitude, regime):
    """Test different orbit regimes."""
    # ... test code
```

### Using Fixtures

```python
def test_with_fixture(leo_state, earth_params, numerical_tolerances):
    """Test using multiple fixtures."""
    gm = earth_params["gm"]
    pos_tol = numerical_tolerances["position_m"]

    result = propagate(leo_state, 5400.0)  # One orbit

    assert abs(result.x - leo_state.x) < pos_tol
```

### Best Practices

1. **Use descriptive test names** that explain what is being tested
2. **Add docstrings** to tests explaining the test purpose
3. **Use appropriate markers** to categorize tests
4. **Use shared fixtures** instead of duplicating setup code
5. **Test one concept per test** - keep tests focused
6. **Include context in assertions** for better error messages
7. **Use appropriate tolerances** from the fixtures
8. **Add regression tests** when fixing bugs

## Benchmarking

Benchmarks use `pytest-benchmark` for statistical performance measurement.

### Basic Benchmark

```python
@pytest.mark.benchmark
def test_propagation_speed(benchmark):
    """Benchmark orbit propagation."""
    state = CartesianState(...)
    times = np.linspace(0, 86400, 1000)

    benchmark(propagate_batch, state, times)
```

### Running Benchmarks

```bash
# Run all benchmarks
pytest -m benchmark --benchmark-only

# Run with verbose statistics
pytest -m benchmark --benchmark-only --benchmark-verbose

# Save baseline results
pytest -m benchmark --benchmark-only --benchmark-save=baseline

# Compare against baseline
pytest -m benchmark --benchmark-only --benchmark-compare=baseline

# Generate histogram
pytest -m benchmark --benchmark-only --benchmark-histogram
```

### Benchmark Output

```
-----------------------------------------------------------------------------
Name (time in ms)              Min       Max      Mean    StdDev    Median
-----------------------------------------------------------------------------
test_propagation_speed      10.234    12.456    10.567    0.234    10.456
-----------------------------------------------------------------------------
```

## Continuous Integration

The test suite is designed to work with CI systems:

```bash
# Fast CI run (skip slow tests)
pytest -m "not slow" --cov=astrora --cov-report=xml

# Full validation run (including slow tests)
pytest --cov=astrora --cov-report=xml

# Benchmark regression check
pytest -m benchmark --benchmark-only --benchmark-compare=baseline
```

## Test Coverage Goals

- **Rust code**: >90% line coverage
- **Python code**: >85% line coverage
- **Critical paths**: 100% coverage (propagators, coordinate transforms)
- **Edge cases**: Explicitly tested for numerical stability

## Getting Help

- Read test docstrings for examples
- Check existing tests in the same category
- Review `conftest.py` for available fixtures
- Review `test_utils.py` for helper functions
- Ask questions in GitHub issues or discussions

## Further Reading

- [pytest Documentation](https://docs.pytest.org/)
- [pytest-benchmark Documentation](https://pytest-benchmark.readthedocs.io/)
- [pytest-cov Documentation](https://pytest-cov.readthedocs.io/)
- [pytest-xdist Documentation](https://pytest-xdist.readthedocs.io/)
