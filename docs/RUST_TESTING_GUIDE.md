# Rust Testing Guide for Astrora

## Overview

This guide documents the comprehensive testing framework for Astrora's Rust codebase. The framework follows best practices for scientific computing libraries, including unit tests, property-based tests, and numerical accuracy validation.

**Testing Statistics (as of Phase 12 completion):**
- **Total Tests**: 465+ Rust unit tests
- **Test Files**: 34 modules with test coverage
- **Success Rate**: 458 passing tests (98.3%)
- **Property-Based Tests**: Integrated with `proptest` crate
- **Test Utilities**: Comprehensive shared testing infrastructure

---

## Table of Contents

1. [Test Infrastructure](#test-infrastructure)
2. [Running Tests](#running-tests)
3. [Test Utilities Module](#test-utilities-module)
4. [Property-Based Testing](#property-based-testing)
5. [Best Practices](#best-practices)
6. [Writing New Tests](#writing-new-tests)
7. [Numerical Accuracy Testing](#numerical-accuracy-testing)
8. [Continuous Integration](#continuous-integration)

---

## Test Infrastructure

### Dependencies

The test framework uses the following crates (specified in `Cargo.toml` under `[dev-dependencies]`):

```toml
[dev-dependencies]
approx = "0.5"       # Floating-point comparison
proptest = "1.0"     # Property-based testing
criterion = "0.5"    # Benchmarking (separate)
```

### Test Organization

Tests are organized using Rust's built-in test framework with the following structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    // Unit tests
    #[test]
    fn test_specific_case() {
        // ...
    }

    // Property-based tests
    proptest! {
        #[test]
        fn test_property(input in strategy()) {
            // ...
        }
    }
}
```

---

## Running Tests

### Basic Test Execution

```bash
# Run all tests (Rust library tests)
cargo test --lib

# Run tests from a specific module
cargo test --lib core::linalg

# Run a specific test
cargo test --lib test_circular_orbit_energy

# Run tests with output visible
cargo test --lib -- --nocapture

# Run tests in release mode (faster, but less debug info)
cargo test --lib --release
```

### Test Filtering

```bash
# Run only tests containing "orbit" in the name
cargo test --lib orbit

# Run only property-based tests (by convention, they often include "property")
cargo test --lib property

# Skip slow tests (requires explicit marking)
cargo test --lib -- --skip slow
```

### Parallel Execution

By default, Rust runs tests in parallel. To control parallelism:

```bash
# Run tests sequentially (useful for debugging)
cargo test --lib -- --test-threads=1

# Run with specific thread count
cargo test --lib -- --test-threads=4
```

---

## Test Utilities Module

The `src/test_utils.rs` module provides shared testing infrastructure for all tests. It is only compiled when testing (`#[cfg(test)]`).

### Key Components

#### 1. **Tolerance Constants**

Pre-defined tolerances for common comparisons:

```rust
use crate::test_utils::*;

// Standard tolerances
POSITION_TOLERANCE_M         // 1 meter for orbital positions
VELOCITY_TOLERANCE_MS        // 0.001 m/s for velocities
ENERGY_TOLERANCE_JKG         // 1 J/kg for specific energy
ANGULAR_MOMENTUM_TOLERANCE   // 1e-6 m²/s
ANGLE_TOLERANCE_RAD          // 1e-10 radians
RELATIVE_TOLERANCE           // 1e-12 for relative comparisons
```

#### 2. **Floating-Point Comparison**

Utilities for comparing floating-point values with appropriate tolerances:

```rust
// Compare two floats
assert_float_eq(computed, expected, RELATIVE_TOLERANCE);

// Compare vectors
assert_vector3_eq(&vec1, &vec2, RELATIVE_TOLERANCE);

// Boolean check (returns true/false)
if floats_equal(a, b, epsilon) {
    // ...
}
```

#### 3. **Property-Based Testing Strategies**

Pre-built strategies for generating test data:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_my_property(
        radius in orbital_radius_strategy(),           // 6578-50000 km
        ecc in eccentricity_elliptical_strategy(),     // 0.0-0.9
        inc in inclination_strategy(),                 // 0-π radians
        angle in angle_strategy(),                     // 0-2π radians
        dt in time_interval_strategy(),                // 1-86400 seconds
    ) {
        // Test implementation
    }
}
```

Available strategies:
- `orbital_radius_strategy()` - Valid orbital radii
- `eccentricity_elliptical_strategy()` - Elliptical eccentricities (0-0.9)
- `eccentricity_all_strategy()` - All eccentricities (elliptical + hyperbolic)
- `inclination_strategy()` - Valid inclinations (0-π)
- `angle_strategy()` - Full circle angles (0-2π)
- `time_interval_strategy()` - Reasonable time intervals
- `small_timestep_strategy()` - Small timesteps for integration
- `velocity_magnitude_strategy()` - Orbital velocity magnitudes
- `position_vector_strategy()` - 3D position vectors in orbital regime
- `velocity_vector_strategy()` - 3D velocity vectors
- `orbital_state_strategy()` - Complete (position, velocity) states

#### 4. **Physical Invariant Checkers**

Functions to verify conservation laws:

```rust
// Check energy conservation
assert!(check_energy_conserved(&r0, &v0, &r1, &v1, mu, tolerance));

// Check angular momentum conservation
assert!(check_angular_momentum_conserved(&r0, &v0, &r1, &v1, tolerance));

// Check semi-major axis consistency
assert!(check_semimajor_axis_energy_consistency(&r, &v, a, mu, tolerance));

// Check eccentricity vector
assert!(check_eccentricity_vector(&r, &v, e, mu, tolerance));
```

#### 5. **Test Data Generators**

Pre-built orbital state generators:

```rust
// Standard orbits
let (r, v) = circular_orbit(7000.0, GM_EARTH);
let (r, v) = equatorial_circular_orbit(400.0);  // 400 km altitude
let (r, v) = polar_circular_orbit(800.0);
let (r, v) = eccentric_orbit(200.0, 35786.0);  // GTO-like

// Classification helpers
assert!(is_circular(e, 0.01));
assert!(is_elliptical(e));
assert!(is_hyperbolic(e));
assert!(is_equatorial(inc, 0.01));
assert!(is_polar(inc, 0.01));
```

---

## Property-Based Testing

Property-based testing verifies that certain mathematical properties hold for arbitrary (randomly generated) inputs, rather than testing specific examples.

### Philosophy

**Traditional Unit Test:**
```rust
#[test]
fn test_normalize_specific() {
    let v = Vector3::new(3.0, 4.0, 0.0);
    let normalized = normalize(&v);
    assert_relative_eq!(normalized.norm(), 1.0, epsilon = 1e-10);
}
```

**Property-Based Test:**
```rust
proptest! {
    #[test]
    fn test_normalize_yields_unit_vector(v in vector3_strategy()) {
        // This runs 100 times (default) with random vectors
        let normalized = normalize(&v);
        let norm = normalized.norm();
        assert!((norm - 1.0).abs() < 1e-10);
    }
}
```

### Benefits

1. **Exhaustive Coverage**: Tests thousands of cases automatically
2. **Edge Case Discovery**: Finds corner cases you didn't think of
3. **Regression Tracking**: Failed cases are saved for future runs
4. **Specification Verification**: Tests mathematical properties, not implementations

### Example: Linear Algebra Properties

From `src/core/linalg.rs`:

```rust
proptest! {
    // Property: Cross product is orthogonal to both inputs
    #[test]
    fn test_cross_product_orthogonality(
        a in vector3_strategy(),
        b in vector3_strategy()
    ) {
        let c = cross(&a, &b);
        let tolerance = a.norm() * b.norm() * 1e-9;
        assert!(c.dot(&a).abs() < tolerance);
        assert!(c.dot(&b).abs() < tolerance);
    }

    // Property: a × b = -(b × a)
    #[test]
    fn test_cross_product_anticommutativity(
        a in vector3_strategy(),
        b in vector3_strategy()
    ) {
        let ab = cross(&a, &b);
        let ba = cross(&b, &a);
        let diff = (ab + ba).norm();
        assert!(diff < 1e-10);
    }

    // Property: skew(a) * b = a × b
    #[test]
    fn test_skew_symmetric_cross_product_equivalence(
        a in vector3_strategy(),
        b in vector3_strategy()
    ) {
        let skew_a = skew_symmetric(&a);
        let result1 = skew_a * b;
        let result2 = cross(&a, &b);
        let diff = (result1 - result2).norm();
        assert!(diff < 1e-10);
    }
}
```

### Configuring Proptest

```rust
// Custom configuration
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1000,              // Run 1000 test cases (default: 256)
        max_shrink_iters: 10000,  // Shrinking iterations
        ..ProptestConfig::default()
    })]

    #[test]
    fn my_intensive_test(x in 0..1000i32) {
        // ...
    }
}
```

### Shrinking

When a property test fails, proptest automatically "shrinks" the failing input to find the minimal failing case:

```
Test failed: c·a = 48, tolerance = 36.97
minimal failing input: a = [[-282835.38, -860927.85, -738887.20]],
                       b = [[303915.74, 248051.93, 572977.28]]
```

This makes debugging much easier than a random failure.

---

## Best Practices

### 1. Use Appropriate Tolerances

**Problem**: Scientific computing involves floating-point arithmetic, which introduces rounding errors that accumulate.

**Solution**: Use relative tolerances that scale with the magnitude of values:

```rust
// ❌ BAD: Absolute tolerance doesn't scale
assert!(result < 1e-10);  // Fails for large values

// ✅ GOOD: Relative tolerance
let tolerance = expected.abs() * 1e-12;
assert!((result - expected).abs() < tolerance);

// ✅ BETTER: Use approx crate
use approx::assert_relative_eq;
assert_relative_eq!(result, expected, epsilon = 1e-12);
```

**For orbital mechanics:**
- Position: ~1 meter for LEO (1e-3 to 1e-6 relative)
- Velocity: ~0.001 m/s (1e-6 to 1e-9 relative)
- Energy: ~1 J/kg (1e-9 to 1e-12 relative)
- Angles: ~1e-10 radians (1e-12 to 1e-15 relative)

### 2. Test Physical Invariants

Always verify conservation laws for orbital mechanics:

```rust
#[test]
fn test_propagation_conserves_energy() {
    let (r0, v0) = circular_orbit(7000e3, GM_EARTH);
    let (r1, v1) = propagate_orbit(&r0, &v0, 3600.0);

    assert!(check_energy_conserved(&r0, &v0, &r1, &v1,
                                   GM_EARTH, ENERGY_TOLERANCE_JKG));
}
```

### 3. Structure Tests by Regime

Orbital mechanics behaves differently in different regimes:

```rust
mod circular_orbit_tests {
    // Tests specific to circular orbits
}

mod eccentric_orbit_tests {
    // Tests for elliptical orbits with e > 0.01
}

mod hyperbolic_orbit_tests {
    // Tests for escape trajectories
}
```

### 4. Use Descriptive Test Names

```rust
// ❌ BAD
#[test]
fn test_1() { ... }

// ✅ GOOD
#[test]
fn test_hohmann_transfer_leo_to_geo_delta_v_within_tolerance() { ... }
```

### 5. Document Expected Behavior

```rust
#[test]
fn test_circular_orbit_remains_circular_after_full_period() {
    // Curtis Example 2.3: Circular orbit at 7000 km should return
    // to the same position after one full period with <1m error
    let (r0, v0) = circular_orbit(7000e3, GM_EARTH);
    let period = orbital_period(7000e3, GM_EARTH);

    let (r1, v1) = propagate_orbit(&r0, &v0, period);

    let position_error = (r1 - r0).norm();
    assert!(position_error < 1.0, "Position error: {} m", position_error);
}
```

### 6. Test Edge Cases Explicitly

```rust
#[test]
fn test_zero_eccentricity_orbit() {
    let e = 0.0;
    // Test degenerate case
}

#[test]
fn test_parabolic_orbit_near_escape() {
    let e = 1.0;  // Exactly parabolic
    // Test boundary condition
}

#[test]
fn test_retrograde_orbit() {
    let inc = std::f64::consts::PI;  // 180 degrees
    // Test special case
}
```

---

## Writing New Tests

### Template for Module Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::test_utils::*;  // If using shared utilities

    // ========================================================================
    // Unit Tests
    // ========================================================================

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_relative_eq!(result, expected, epsilon = 1e-12);
    }

    #[test]
    fn test_edge_case() {
        // Test boundary conditions
    }

    #[test]
    fn test_error_handling() {
        // Test that invalid inputs produce expected errors
        let result = function_under_test(invalid_input);
        assert!(result.is_err());
    }

    // ========================================================================
    // Property-Based Tests
    // ========================================================================

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_mathematical_property(
            x in -1000.0..1000.0_f64,
            y in -1000.0..1000.0_f64
        ) {
            // Test that property holds for all generated inputs
            let result = function_under_test(x, y);
            assert!(verify_property(result));
        }
    }
}
```

### Example: Adding Tests to a New Module

Suppose you're adding a new module `src/core/my_module.rs`:

```rust
// src/core/my_module.rs

/// Calculate orbital angular momentum
pub fn angular_momentum(r: &Vector3, v: &Vector3) -> Vector3 {
    r.cross(v)
}

/// Calculate specific orbital energy
pub fn specific_energy(r: &Vector3, v: &Vector3, mu: f64) -> f64 {
    v.norm_squared() / 2.0 - mu / r.norm()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::test_utils::*;
    use nalgebra::Vector3;
    use proptest::prelude::*;

    // Unit tests
    #[test]
    fn test_circular_orbit_angular_momentum() {
        let (r, v) = circular_orbit(7000e3, GM_EARTH_TEST);
        let h = angular_momentum(&r, &v);

        // Angular momentum should be perpendicular to r and v
        assert!(h.dot(&r).abs() < 1e-6);
        assert!(h.dot(&v).abs() < 1e-6);

        // Magnitude should be r * v for circular orbit
        let expected_h = 7000e3 * (GM_EARTH_TEST / 7000e3).sqrt();
        assert_relative_eq!(h.norm(), expected_h, epsilon = 1e-9);
    }

    #[test]
    fn test_circular_orbit_energy_negative() {
        let (r, v) = circular_orbit(7000e3, GM_EARTH_TEST);
        let energy = specific_energy(&r, &v, GM_EARTH_TEST);

        // Elliptical orbits have negative energy
        assert!(energy < 0.0);

        // Specific energy: ε = -μ/(2a) for circular orbit (a = r)
        let expected = -GM_EARTH_TEST / (2.0 * 7000e3);
        assert_relative_eq!(energy, expected, epsilon = 1e-12);
    }

    // Property-based tests
    proptest! {
        #[test]
        fn test_angular_momentum_perpendicularity(
            (r, v) in orbital_state_strategy()
        ) {
            let h = angular_momentum(&r, &v);

            // h should always be perpendicular to r and v
            let dot_r = h.dot(&r).abs();
            let dot_v = h.dot(&v).abs();

            assert!(dot_r < 1e-6, "h·r = {}", dot_r);
            assert!(dot_v < 1e-6, "h·v = {}", dot_v);
        }

        #[test]
        fn test_energy_bound_states_negative(
            (r, v) in orbital_state_strategy()
        ) {
            let energy = specific_energy(&r, &v, GM_EARTH_TEST);

            // For orbits (not escape), energy should be negative
            let escape_velocity = (2.0 * GM_EARTH_TEST / r.norm()).sqrt();

            if v.norm() < escape_velocity {
                assert!(energy < 0.0);
            }
        }
    }
}
```

---

## Numerical Accuracy Testing

### Validate Against Reference Implementations

Always compare results with established references:

```rust
#[test]
fn test_against_vallado_example_2_3() {
    // Data from Vallado, "Fundamentals of Astrodynamics", Example 2.3
    let r0 = Vector3::new(1131.340e3, -2282.343e3, 6672.423e3);
    let v0 = Vector3::new(-5643.05, 4303.33, 2428.79);

    // Expected result from textbook
    let expected_a = 36127.343e3;  // Semi-major axis (m)

    let elements = cartesian_to_orbital(&r0, &v0, GM_EARTH);

    assert_relative_eq!(elements.a, expected_a, epsilon = 1e-3);
}
```

### Long-Term Stability Tests

For numerical integrators, test stability over many orbits:

```rust
#[test]
fn test_rk4_long_term_stability() {
    let (r0, v0) = circular_orbit(7000e3, GM_EARTH);
    let period = orbital_period(7000e3, GM_EARTH);

    // Propagate for 100 orbits
    let (r_final, v_final) = propagate_orbit(&r0, &v0, 100.0 * period);

    // Energy drift should be minimal
    let e0 = specific_energy(&r0, &v0, GM_EARTH);
    let ef = specific_energy(&r_final, &v_final, GM_EARTH);
    let relative_error = ((ef - e0) / e0).abs();

    assert!(relative_error < 1e-6, "Energy drift: {}", relative_error);
}
```

### Convergence Tests

For iterative methods:

```rust
#[test]
fn test_kepler_solver_convergence() {
    let M = std::f64::consts::PI / 4.0;  // Mean anomaly
    let e = 0.1;  // Eccentricity

    // Should converge in < 10 iterations
    let (E, iterations) = solve_kepler(M, e, 1e-12);

    assert!(iterations < 10);

    // Verify Kepler's equation
    let residual = (E - e * E.sin() - M).abs();
    assert!(residual < 1e-12);
}
```

---

## Continuous Integration

### GitHub Actions Example

```yaml
name: Rust Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --lib --verbose
      - name: Run property tests (extended)
        run: PROPTEST_CASES=1000 cargo test --lib --release
```

### Coverage Reporting

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate coverage report
cargo llvm-cov --lib --html

# Open coverage report
open target/llvm-cov/html/index.html
```

**Coverage Goals:**
- **Rust code**: >90% line coverage
- **Critical paths**: 100% coverage (propagators, coordinate transforms)

---

## Summary

The Astrora Rust testing framework provides:

1. **465+ unit tests** covering all major modules
2. **Property-based testing** with `proptest` for exhaustive validation
3. **Shared test utilities** (`src/test_utils.rs`) for common operations
4. **Numerical accuracy validation** against reference implementations
5. **Physical invariant checking** (energy, angular momentum conservation)
6. **Best practices** for floating-point comparison and tolerance selection

**Quick Reference:**

```bash
# Run all tests
cargo test --lib

# Run with property test output
cargo test --lib -- --nocapture

# Run specific module
cargo test --lib core::linalg

# Run in release mode (faster)
cargo test --lib --release

# Run with extended property testing
PROPTEST_CASES=1000 cargo test --lib
```

For more examples, see:
- `src/test_utils.rs` - Shared testing infrastructure
- `src/core/linalg.rs` - Property-based testing examples
- `tests/test_reference_validation.py` - Python integration tests

---

**Last Updated**: 2025-10-23 (Phase 12 - Testing Infrastructure)
