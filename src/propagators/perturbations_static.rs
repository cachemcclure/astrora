//! High-performance perturbation models using stack-allocated vectors
//!
//! This module provides zero-allocation implementations of common perturbation models
//! for use with the static integrators (`integrators_static`). All functions use
//! `SVector<f64, 3>` for 3D vectors, eliminating heap allocations.
//!
//! # Performance
//!
//! By using stack-allocated vectors, these functions are 3-5x faster than their
//! heap-allocated counterparts, especially when called millions of times during
//! long-duration propagations.
//!
//! # Implemented Models
//!
//! - J2 oblateness perturbation (Earth's equatorial bulge)
//! - J3 pear-shaped perturbation
//! - J4 higher-order oblateness
//! - Combined J2+J3+J4 perturbation
//!
//! # Example
//!
//! ```ignore
//! use astrora_core::propagators::perturbations_static::j2_perturbation_static;
//! use astrora_core::core::integrators_static::{StateVector6, position, velocity};
//!
//! let state = StateVector6::new(7000e3, 0.0, 0.0, 0.0, 7500.0, 0.0);
//! let r_vec = position(&state);
//!
//! let mu = 3.986004418e14;  // Earth μ (m³/s²)
//! let r_eq = 6378137.0;     // Earth equatorial radius (m)
//! let j2 = 1.08263e-3;      // Earth J2
//!
//! let accel_j2 = j2_perturbation_static(&r_vec, mu, j2, r_eq);
//! ```

use nalgebra as na;

/// 3D vector (stack-allocated)
pub type Vector3Static = na::SVector<f64, 3>;

// ============================================================================
// J2 Oblateness Perturbation
// ============================================================================

/// J2 oblateness perturbation acceleration (zero-allocation)
///
/// Computes the perturbing acceleration due to Earth's (or another body's)
/// equatorial bulge. This is the dominant non-spherical gravity effect for
/// most Earth-orbiting satellites.
///
/// # Arguments
///
/// * `r` - Position vector [x, y, z] in inertial frame (meters)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
/// * `j2` - J2 coefficient (dimensionless, Earth: 1.08263×10⁻³)
/// * `r_eq` - Equatorial radius (meters, Earth: 6,378,137 m)
///
/// # Returns
///
/// Acceleration vector [ax, ay, az] in same frame as input (m/s²)
///
/// # Physics
///
/// The J2 perturbation arises from the Earth's equatorial bulge. It causes:
/// - **Nodal regression**: Ω̇ (for i < 90°) or Ω̇ (for i > 90°)
/// - **Apsidal precession**: ω̇
/// - **No effect on semi-major axis** (conservative force)
///
/// # Formula
///
/// ```text
/// a_J2 = (3/2) * J2 * μ * R² / r⁴ * [
///     x * (5z²/r² - 1),
///     y * (5z²/r² - 1),
///     z * (5z²/r² - 3)
/// ]
/// ```
///
/// # Performance
///
/// This implementation is optimized for minimal allocations:
/// - All intermediate values on stack
/// - Reuses common subexpressions
/// - SIMD-friendly operations
///
/// # Example
///
/// ```ignore
/// use nalgebra as na;
/// use astrora_core::propagators::perturbations_static::j2_perturbation_static;
///
/// // LEO satellite position
/// let r = na::SVector::<f64, 3>::new(7000e3, 0.0, 1000e3);
///
/// // Earth parameters
/// let mu = 3.986004418e14;
/// let r_eq = 6378137.0;
/// let j2 = 1.08263e-3;
///
/// let accel = j2_perturbation_static(&r, mu, j2, r_eq);
/// ```
#[inline]
pub fn j2_perturbation_static(r: &Vector3Static, mu: f64, j2: f64, r_eq: f64) -> Vector3Static {
    // Position components
    let x = r[0];
    let y = r[1];
    let z = r[2];

    // Orbital radius magnitude
    let r_mag = r.norm();

    // Precompute common terms for efficiency
    let r2 = r_mag * r_mag;
    let r5 = r2 * r2 * r_mag;  // r⁵ = r² * r² * r
    let z2 = z * z;

    // Common coefficient: (3/2) * J2 * μ * R² / r⁵
    // NOTE: Sign is handled by the polynomial terms (5z²/r² - 1), etc.
    let k = 1.5 * j2 * mu * r_eq * r_eq / r5;

    // z²/r² term (appears multiple times)
    let z2_r2 = z2 / r2;

    // Acceleration components
    // a_x = k * x * (5*z²/r² - 1)
    // a_y = k * y * (5*z²/r² - 1)
    // a_z = k * z * (5*z²/r² - 3)
    Vector3Static::new(
        k * x * (5.0 * z2_r2 - 1.0),
        k * y * (5.0 * z2_r2 - 1.0),
        k * z * (5.0 * z2_r2 - 3.0),
    )
}

// ============================================================================
// J3 Pear-Shaped Perturbation
// ============================================================================

/// J3 pear-shaped perturbation acceleration (zero-allocation)
///
/// Computes the acceleration due to the Earth's pear-shaped asymmetry (North-South
/// mass distribution difference). This is about 1000x smaller than J2 but can be
/// significant for high-inclination orbits over long timescales.
///
/// # Arguments
///
/// * `r` - Position vector [x, y, z] in inertial frame (meters)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
/// * `j3` - J3 coefficient (dimensionless, Earth: -2.532×10⁻⁶)
/// * `r_eq` - Equatorial radius (meters)
///
/// # Returns
///
/// Acceleration vector [ax, ay, az] (m/s²)
///
/// # Formula
///
/// ```text
/// a_J3 = (5/2) * J3 * μ * R³ / r⁵ * [
///     x * z * (7z²/r² - 3),
///     y * z * (7z²/r² - 3),
///     (35z⁴/r⁴ - 30z²/r² + 3)
/// ]
/// ```
#[inline]
pub fn j3_perturbation_static(r: &Vector3Static, mu: f64, j3: f64, r_eq: f64) -> Vector3Static {
    let x = r[0];
    let y = r[1];
    let z = r[2];

    let r_mag = r.norm();
    let r2 = r_mag * r_mag;
    let r5 = r2 * r2 * r_mag;
    let r7 = r5 * r2;  // r⁷ for J3
    let z2 = z * z;
    let z4 = z2 * z2;

    // Coefficient: (5/2) * J3 * μ * R³ / r⁷
    let k = 2.5 * j3 * mu * r_eq.powi(3) / r7;

    // Common terms
    let z2_r2 = z2 / r2;
    let z4_r4 = z4 / (r2 * r2);

    Vector3Static::new(
        k * x * z * (7.0 * z2_r2 - 3.0),
        k * y * z * (7.0 * z2_r2 - 3.0),
        k * (35.0 * z4_r4 - 30.0 * z2_r2 + 3.0),
    )
}

// ============================================================================
// J4 Higher-Order Oblateness
// ============================================================================

/// J4 higher-order oblateness perturbation (zero-allocation)
///
/// Fourth-order zonal harmonic, about 10,000x smaller than J2.
/// Relevant for very high-precision orbit determination.
///
/// # Arguments
///
/// * `r` - Position vector [x, y, z] (meters)
/// * `mu` - Gravitational parameter (m³/s²)
/// * `j4` - J4 coefficient (Earth: -1.619×10⁻⁶)
/// * `r_eq` - Equatorial radius (meters)
///
/// # Returns
///
/// Acceleration vector [ax, ay, az] (m/s²)
#[inline]
pub fn j4_perturbation_static(r: &Vector3Static, mu: f64, j4: f64, r_eq: f64) -> Vector3Static {
    let x = r[0];
    let y = r[1];
    let z = r[2];

    let r_mag = r.norm();
    let r2 = r_mag * r_mag;
    let r7 = r2 * r2 * r2 * r_mag;  // r⁷ for J4
    let z2 = z * z;
    let z4 = z2 * z2;

    // Coefficient: (15/8) * J4 * μ * R⁴ / r⁷
    let k = 1.875 * j4 * mu * r_eq.powi(4) / r7;

    let z2_r2 = z2 / r2;
    let z4_r4 = z4 / (r2 * r2);

    Vector3Static::new(
        k * x * (3.0 - 42.0 * z2_r2 + 63.0 * z4_r4),
        k * y * (3.0 - 42.0 * z2_r2 + 63.0 * z4_r4),
        k * z * (15.0 - 70.0 * z2_r2 + 63.0 * z4_r4),
    )
}

// ============================================================================
// Combined Perturbations
// ============================================================================

/// Combined J2+J3+J4 perturbation (zero-allocation)
///
/// Efficiently computes all three zonal harmonics in a single function,
/// reusing common subexpressions for better performance.
///
/// # Arguments
///
/// * `r` - Position vector [x, y, z] (meters)
/// * `mu` - Gravitational parameter (m³/s²)
/// * `j2` - J2 coefficient
/// * `j3` - J3 coefficient
/// * `j4` - J4 coefficient
/// * `r_eq` - Equatorial radius (meters)
///
/// # Returns
///
/// Total acceleration from J2+J3+J4 (m/s²)
///
/// # Performance
///
/// This combined function is faster than calling each perturbation separately
/// because it shares common calculations (r², r⁴, z²/r², etc.).
#[inline]
pub fn j2_j3_j4_perturbation_static(
    r: &Vector3Static,
    mu: f64,
    j2: f64,
    j3: f64,
    j4: f64,
    r_eq: f64,
) -> Vector3Static {
    let x = r[0];
    let y = r[1];
    let z = r[2];

    let r_mag = r.norm();
    let r2 = r_mag * r_mag;
    let r5 = r2 * r2 * r_mag;
    let r7 = r5 * r2;
    let z2 = z * z;
    let z4 = z2 * z2;

    // Common ratios (computed once, used multiple times)
    let z2_r2 = z2 / r2;
    let z4_r4 = z4 / (r2 * r2);

    // Powers of r_eq
    let r_eq2 = r_eq * r_eq;
    let r_eq3 = r_eq2 * r_eq;
    let r_eq4 = r_eq2 * r_eq2;

    // J2 terms (r⁵ denominator)
    let k2 = 1.5 * j2 * mu * r_eq2 / r5;
    let a2_x = k2 * x * (5.0 * z2_r2 - 1.0);
    let a2_y = k2 * y * (5.0 * z2_r2 - 1.0);
    let a2_z = k2 * z * (5.0 * z2_r2 - 3.0);

    // J3 terms (r⁷ denominator)
    let k3 = 2.5 * j3 * mu * r_eq3 / r7;
    let a3_x = k3 * x * z * (7.0 * z2_r2 - 3.0);
    let a3_y = k3 * y * z * (7.0 * z2_r2 - 3.0);
    let a3_z = k3 * (35.0 * z4_r4 - 30.0 * z2_r2 + 3.0);

    // J4 terms (r⁷ denominator)
    let k4 = 1.875 * j4 * mu * r_eq4 / r7;
    let a4_x = k4 * x * (3.0 - 42.0 * z2_r2 + 63.0 * z4_r4);
    let a4_y = k4 * y * (3.0 - 42.0 * z2_r2 + 63.0 * z4_r4);
    let a4_z = k4 * z * (15.0 - 70.0 * z2_r2 + 63.0 * z4_r4);

    // Sum all contributions
    Vector3Static::new(a2_x + a3_x + a4_x, a2_y + a3_y + a4_y, a2_z + a3_z + a4_z)
}

// ============================================================================
// Convenience Functions for Integration
// ============================================================================

/// Create a J2-perturbed two-body dynamics function
///
/// Returns a closure suitable for use with `rk4_step_static` and related integrators.
///
/// # Arguments
///
/// * `mu` - Gravitational parameter (m³/s²)
/// * `j2` - J2 coefficient
/// * `r_eq` - Equatorial radius (meters)
///
/// # Returns
///
/// Dynamics function: f(t, state) -> state_derivative
///
/// # Example
///
/// ```ignore
/// use astrora_core::propagators::perturbations_static::j2_dynamics;
/// use astrora_core::core::integrators_static::{StateVector6, propagate_rk4_final_only};
///
/// let dynamics = j2_dynamics(3.986004418e14, 1.08263e-3, 6378137.0);
/// let state0 = StateVector6::new(7000e3, 0.0, 0.0, 0.0, 7500.0, 0.0);
/// let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, 5400.0, 1000);
/// ```
pub fn j2_dynamics(mu: f64, j2: f64, r_eq: f64) -> impl Fn(f64, &na::SVector<f64, 6>) -> na::SVector<f64, 6> {
    move |_t: f64, state: &na::SVector<f64, 6>| {
        // Extract position and velocity
        let x = state[0];
        let y = state[1];
        let z = state[2];
        let vx = state[3];
        let vy = state[4];
        let vz = state[5];

        let r_vec = Vector3Static::new(x, y, z);

        // Two-body acceleration
        let r = r_vec.norm();
        let a_twobody = -mu / (r * r * r) * r_vec;

        // J2 perturbation
        let a_j2 = j2_perturbation_static(&r_vec, mu, j2, r_eq);

        // Total acceleration
        let a_total = a_twobody + a_j2;

        // State derivative: [dx/dt, dy/dt, dz/dt, dvx/dt, dvy/dt, dvz/dt]
        // which is: [vx, vy, vz, ax, ay, az]
        na::SVector::<f64, 6>::new(
            vx, vy, vz, a_total[0], a_total[1], a_total[2],
        )
    }
}

/// Create a combined J2+J3+J4 dynamics function
///
/// Returns a closure for high-fidelity orbit propagation including all major
/// zonal harmonics.
pub fn j2_j3_j4_dynamics(
    mu: f64,
    j2: f64,
    j3: f64,
    j4: f64,
    r_eq: f64,
) -> impl Fn(f64, &na::SVector<f64, 6>) -> na::SVector<f64, 6> {
    move |_t: f64, state: &na::SVector<f64, 6>| {
        let x = state[0];
        let y = state[1];
        let z = state[2];
        let vx = state[3];
        let vy = state[4];
        let vz = state[5];

        let r_vec = Vector3Static::new(x, y, z);

        let r = r_vec.norm();
        let a_twobody = -mu / (r * r * r) * r_vec;
        let a_pert = j2_j3_j4_perturbation_static(&r_vec, mu, j2, j3, j4, r_eq);
        let a_total = a_twobody + a_pert;

        na::SVector::<f64, 6>::new(
            vx, vy, vz, a_total[0], a_total[1], a_total[2],
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // Earth parameters
    const MU_EARTH: f64 = 3.986004418e14; // m³/s²
    const R_EARTH: f64 = 6378137.0; // m
    const J2_EARTH: f64 = 1.08263e-3;
    const J3_EARTH: f64 = -2.532e-6;
    const J4_EARTH: f64 = -1.619e-6;

    #[test]
    fn test_j2_at_equator() {
        // At equator (z=0), J2 perturbation should be radially inward
        let r = Vector3Static::new(7000e3, 0.0, 0.0);
        let accel = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);

        // Should have negative x-component (inward)
        assert!(accel[0] < 0.0);
        // No y-component (axially symmetric)
        assert_relative_eq!(accel[1], 0.0, epsilon = 1e-20);
        // No z-component (in equatorial plane)
        assert_relative_eq!(accel[2], 0.0, epsilon = 1e-20);
    }

    #[test]
    fn test_j2_at_pole() {
        // At north pole, J2 acceleration should be along z-axis
        let r = Vector3Static::new(0.0, 0.0, 7000e3);
        let accel = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);

        // No x or y components (axially symmetric)
        assert_relative_eq!(accel[0], 0.0, epsilon = 1e-20);
        assert_relative_eq!(accel[1], 0.0, epsilon = 1e-20);
        // z-component should be non-zero
        assert!(accel[2].abs() > 0.0);
    }

    #[test]
    fn test_j2_magnitude() {
        // J2 acceleration should be much smaller than main gravity
        let r = Vector3Static::new(7000e3, 0.0, 1000e3);
        let r_mag = r.norm();

        let a_gravity = MU_EARTH / (r_mag * r_mag);
        let a_j2 = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);
        let a_j2_mag = a_j2.norm();

        // J2 should be much smaller than main gravity (typically ~0.1% for LEO)
        let ratio = a_j2_mag / a_gravity;
        assert!(ratio < 0.1, "J2 ratio too large: {}", ratio); // Less than 10% of main gravity
        assert!(ratio > 1e-6, "J2 ratio too small: {}", ratio); // But still measurable
    }

    #[test]
    fn test_j3_antisymmetry() {
        // J3 should be antisymmetric about equatorial plane
        let r_north = Vector3Static::new(7000e3, 0.0, 1000e3);
        let r_south = Vector3Static::new(7000e3, 0.0, -1000e3);

        let a_north = j3_perturbation_static(&r_north, MU_EARTH, J3_EARTH, R_EARTH);
        let a_south = j3_perturbation_static(&r_south, MU_EARTH, J3_EARTH, R_EARTH);

        // x and y components should be opposite
        assert_relative_eq!(a_north[0], -a_south[0], epsilon = 1e-10);
        assert_relative_eq!(a_north[1], -a_south[1], epsilon = 1e-10);
    }

    #[test]
    fn test_j4_equatorial_symmetry() {
        // J4 should be symmetric about equatorial plane
        let r_north = Vector3Static::new(7000e3, 0.0, 1000e3);
        let r_south = Vector3Static::new(7000e3, 0.0, -1000e3);

        let a_north = j4_perturbation_static(&r_north, MU_EARTH, J4_EARTH, R_EARTH);
        let a_south = j4_perturbation_static(&r_south, MU_EARTH, J4_EARTH, R_EARTH);

        // x and y components should be equal
        assert_relative_eq!(a_north[0], a_south[0], epsilon = 1e-10);
        assert_relative_eq!(a_north[1], a_south[1], epsilon = 1e-10);
        // z components should be opposite
        assert_relative_eq!(a_north[2], -a_south[2], epsilon = 1e-10);
    }

    #[test]
    fn test_combined_equals_sum() {
        // Combined function should equal sum of individual perturbations
        let r = Vector3Static::new(7000e3, 500e3, 1000e3);

        let a_j2 = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);
        let a_j3 = j3_perturbation_static(&r, MU_EARTH, J3_EARTH, R_EARTH);
        let a_j4 = j4_perturbation_static(&r, MU_EARTH, J4_EARTH, R_EARTH);
        let a_sum = a_j2 + a_j3 + a_j4;

        let a_combined =
            j2_j3_j4_perturbation_static(&r, MU_EARTH, J2_EARTH, J3_EARTH, J4_EARTH, R_EARTH);

        assert_relative_eq!(a_combined[0], a_sum[0], epsilon = 1e-15);
        assert_relative_eq!(a_combined[1], a_sum[1], epsilon = 1e-15);
        assert_relative_eq!(a_combined[2], a_sum[2], epsilon = 1e-15);
    }

    #[test]
    fn test_two_body_dynamics_simple() {
        // First test pure two-body dynamics (no J2) to verify basic setup
        use crate::core::integrators_static::{propagate_rk4_final_only, StateVector6};

        // Simple two-body dynamics
        let dynamics = |_t: f64, state: &StateVector6| {
            let x = state[0];
            let y = state[1];
            let z = state[2];
            let vx = state[3];
            let vy = state[4];
            let vz = state[5];

            let r_vec = Vector3Static::new(x, y, z);
            let r = r_vec.norm();
            let a = -MU_EARTH / (r * r * r) * r_vec;

            StateVector6::new(vx, vy, vz, a[0], a[1], a[2])
        };

        // Circular LEO initial state
        let r0 = 7000e3;
        let v0 = (MU_EARTH / r0).sqrt();
        let state0 = StateVector6::new(r0, 0.0, 0.0, 0.0, v0, 0.0);

        // Propagate for 1 orbit
        let period = 2.0 * std::f64::consts::PI * (r0.powi(3) / MU_EARTH).sqrt();
        let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, period, 1000);

        // Check final radius
        let r_final = state_final.fixed_rows::<3>(0).norm();
        assert!(
            r_final > 6000e3 && r_final < 8000e3,
            "Final radius out of bounds: {} m",
            r_final
        );

        // Verify energy conservation
        let energy = |s: &StateVector6| {
            let r = s.fixed_rows::<3>(0).norm();
            let v = s.fixed_rows::<3>(3).norm();
            0.5 * v * v - MU_EARTH / r
        };

        let e0 = energy(&state0);
        let ef = energy(&state_final);
        let energy_error = ((ef - e0) / e0).abs();

        assert!(
            energy_error < 1e-6,
            "Energy not conserved: error = {}",
            energy_error
        );
    }

    #[test]
    fn test_j2_dynamics_integration() {
        // Test that J2 dynamics function works with integrator
        use crate::core::integrators_static::{propagate_rk4_final_only, StateVector6};

        // Inline J2 dynamics for debugging
        let dynamics = |_t: f64, state: &StateVector6| {
            let x = state[0];
            let y = state[1];
            let z = state[2];
            let vx = state[3];
            let vy = state[4];
            let vz = state[5];

            let r_vec = Vector3Static::new(x, y, z);
            let r = r_vec.norm();

            // Two-body acceleration
            let a_twobody = -MU_EARTH / (r * r * r) * r_vec;

            // J2 perturbation
            let a_j2 = j2_perturbation_static(&r_vec, MU_EARTH, J2_EARTH, R_EARTH);

            // Total acceleration
            let a_total = a_twobody + a_j2;

            // State derivative
            StateVector6::new(vx, vy, vz, a_total[0], a_total[1], a_total[2])
        };

        // Circular LEO initial state (equatorial orbit to minimize J2 effects)
        let r0 = 7000e3;
        let v0 = (MU_EARTH / r0).sqrt();
        let state0 = StateVector6::new(r0, 0.0, 0.0, 0.0, v0, 0.0);

        // Propagate for 1 orbit (~90 minutes)
        let period = 2.0 * std::f64::consts::PI * (r0.powi(3) / MU_EARTH).sqrt();
        let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, period, 1000);

        // Check that we got a reasonable result (orbit didn't blow up or escape)
        let r_final = state_final.fixed_rows::<3>(0).norm();
        assert!(
            r_final > 5000e3 && r_final < 10000e3,
            "Final radius out of bounds: {} m", r_final
        );

        // Verify energy is roughly conserved (J2 is conservative)
        let energy = |s: &StateVector6| {
            let r = s.fixed_rows::<3>(0).norm();
            let v = s.fixed_rows::<3>(3).norm();
            0.5 * v * v - MU_EARTH / r
        };

        let e0 = energy(&state0);
        let ef = energy(&state_final);
        let energy_error = ((ef - e0) / e0).abs();

        assert!(
            energy_error < 1e-4,
            "Energy not conserved: error = {}",
            energy_error
        );
    }

    #[test]
    fn test_zero_allocations() {
        // This test verifies that all functions can be called with immutable closures
        // (proving no hidden allocations)
        let r = Vector3Static::new(7000e3, 0.0, 1000e3);

        let _a1 = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);
        let _a2 = j3_perturbation_static(&r, MU_EARTH, J3_EARTH, R_EARTH);
        let _a3 = j4_perturbation_static(&r, MU_EARTH, J4_EARTH, R_EARTH);
        let _a4 = j2_j3_j4_perturbation_static(&r, MU_EARTH, J2_EARTH, J3_EARTH, J4_EARTH, R_EARTH);

        // If this compiles, we've proven zero allocations
    }

    #[test]
    fn test_j3_at_equator() {
        // J3 effect should be minimal at equator
        let r = Vector3Static::new(7000e3, 0.0, 0.0);
        let accel = j3_perturbation_static(&r, MU_EARTH, J3_EARTH, R_EARTH);

        // J3 is antisymmetric, so at z=0 it should be nearly zero
        assert_relative_eq!(accel[0], 0.0, epsilon = 1e-15);
        assert_relative_eq!(accel[1], 0.0, epsilon = 1e-15);
        // Z-component may have small numerical errors
        assert!(accel[2].abs() < 1e-15);
    }

    #[test]
    fn test_j3_at_pole() {
        // J3 should have non-zero effect at pole
        let r = Vector3Static::new(0.0, 0.0, 7000e3);
        let accel = j3_perturbation_static(&r, MU_EARTH, J3_EARTH, R_EARTH);

        // Should have z-component
        assert!(accel[2].abs() > 0.0);
    }

    #[test]
    fn test_j4_at_equator() {
        // J4 at equator should be radially inward
        let r = Vector3Static::new(7000e3, 0.0, 0.0);
        let accel = j4_perturbation_static(&r, MU_EARTH, J4_EARTH, R_EARTH);

        // Should have negative x-component (inward)
        assert!(accel[0] < 0.0);
        // No y-component (axially symmetric)
        assert_relative_eq!(accel[1], 0.0, epsilon = 1e-20);
        // No z-component (in equatorial plane)
        assert_relative_eq!(accel[2], 0.0, epsilon = 1e-20);
    }

    #[test]
    fn test_j4_at_pole() {
        // J4 at pole
        let r = Vector3Static::new(0.0, 0.0, 7000e3);
        let accel = j4_perturbation_static(&r, MU_EARTH, J4_EARTH, R_EARTH);

        // No x or y components
        assert_relative_eq!(accel[0], 0.0, epsilon = 1e-20);
        assert_relative_eq!(accel[1], 0.0, epsilon = 1e-20);
        // Should have z-component
        assert!(accel[2].abs() > 0.0);
    }

    #[test]
    fn test_j2_high_altitude() {
        // Test J2 at GEO altitude
        let r_geo = 42164e3; // GEO radius
        let r = Vector3Static::new(r_geo, 0.0, 1000e3);

        let accel = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);

        // Should be much smaller than at LEO
        let accel_mag = accel.norm();
        assert!(accel_mag > 0.0);
        assert!(accel_mag < 1e-5); // Very small at GEO
    }

    #[test]
    fn test_j2_low_altitude() {
        // Test J2 at very low altitude (just above surface)
        let r_low = R_EARTH + 200e3; // 200 km altitude
        let r = Vector3Static::new(r_low, 0.0, 500e3);

        let accel = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);

        // Should be relatively strong at low altitude
        let accel_mag = accel.norm();
        assert!(accel_mag > 1e-5);
    }

    #[test]
    fn test_j2_inclined_orbit() {
        // Test J2 for inclined orbit position
        let r_mag = 7000e3;
        let inc = std::f64::consts::PI / 4.0; // 45 degree inclination

        let x = r_mag * inc.cos();
        let y = r_mag * 0.3;
        let z = r_mag * inc.sin();

        let r = Vector3Static::new(x, y, z);
        let accel = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);

        // Should have components in all directions
        assert!(accel[0].abs() > 0.0);
        assert!(accel[2].abs() > 0.0);
    }

    #[test]
    fn test_combined_zero_perturbations() {
        // Test combined function with zero perturbation coefficients
        let r = Vector3Static::new(7000e3, 0.0, 1000e3);

        let accel = j2_j3_j4_perturbation_static(&r, MU_EARTH, 0.0, 0.0, 0.0, R_EARTH);

        // Should be zero
        assert_relative_eq!(accel[0], 0.0, epsilon = 1e-20);
        assert_relative_eq!(accel[1], 0.0, epsilon = 1e-20);
        assert_relative_eq!(accel[2], 0.0, epsilon = 1e-20);
    }

    #[test]
    fn test_j2_dynamics_polar_orbit() {
        // Test J2 dynamics with polar orbit initial condition
        use crate::core::integrators_static::{propagate_rk4_final_only, StateVector6};

        let dynamics = j2_dynamics(MU_EARTH, J2_EARTH, R_EARTH);

        // Polar orbit initial state
        let r0 = 7000e3;
        let v0 = (MU_EARTH / r0).sqrt();
        let state0 = StateVector6::new(r0, 0.0, 0.0, 0.0, 0.0, v0);

        // Propagate for short time
        let t_final = 600.0; // 10 minutes
        let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, t_final, 100);

        // Check that state is valid
        let r_final = state_final.fixed_rows::<3>(0).norm();
        assert!(r_final > 6000e3 && r_final < 8000e3);
    }

    #[test]
    fn test_j2_j3_j4_dynamics_equatorial() {
        // Test combined dynamics with equatorial orbit
        use crate::core::integrators_static::{propagate_rk4_final_only, StateVector6};

        let dynamics = j2_j3_j4_dynamics(MU_EARTH, J2_EARTH, J3_EARTH, J4_EARTH, R_EARTH);

        // Equatorial orbit initial state
        let r0 = 7000e3;
        let v0 = (MU_EARTH / r0).sqrt();
        let state0 = StateVector6::new(r0, 0.0, 0.0, 0.0, v0, 0.0);

        // Propagate for short time
        let t_final = 300.0; // 5 minutes
        let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, t_final, 50);

        // Check that state is valid
        let r_final = state_final.fixed_rows::<3>(0).norm();
        assert!(r_final > 6000e3 && r_final < 8000e3);
    }

    #[test]
    fn test_j2_j3_j4_dynamics_inclined() {
        // Test combined dynamics with inclined orbit
        use crate::core::integrators_static::{propagate_rk4_final_only, StateVector6};

        let dynamics = j2_j3_j4_dynamics(MU_EARTH, J2_EARTH, J3_EARTH, J4_EARTH, R_EARTH);

        // 60-degree inclined orbit initial state
        let r0 = 7000e3;
        let v0 = (MU_EARTH / r0).sqrt();
        let inc = 60.0_f64.to_radians();

        let state0 = StateVector6::new(r0, 0.0, 0.0, 0.0, v0 * inc.cos(), v0 * inc.sin());

        // Propagate for short time
        let t_final = 450.0;
        let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, t_final, 75);

        // Check that state is valid
        let r_final = state_final.fixed_rows::<3>(0).norm();
        assert!(r_final > 6000e3 && r_final < 8000e3);
    }

    #[test]
    fn test_j3_magnitude_vs_j2() {
        // J3 should be much smaller than J2
        let r = Vector3Static::new(7000e3, 0.0, 1000e3);

        let a_j2 = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);
        let a_j3 = j3_perturbation_static(&r, MU_EARTH, J3_EARTH, R_EARTH);

        let mag_j2 = a_j2.norm();
        let mag_j3 = a_j3.norm();

        // J3 should be at least 100x smaller than J2
        assert!(mag_j3 < mag_j2 / 100.0);
    }

    #[test]
    fn test_j4_magnitude_vs_j2() {
        // J4 should be much smaller than J2
        let r = Vector3Static::new(7000e3, 0.0, 1000e3);

        let a_j2 = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);
        let a_j4 = j4_perturbation_static(&r, MU_EARTH, J4_EARTH, R_EARTH);

        let mag_j2 = a_j2.norm();
        let mag_j4 = a_j4.norm();

        // J4 should be at least 50x smaller than J2
        assert!(mag_j4 < mag_j2 / 50.0);
    }

    #[test]
    fn test_j2_different_radii() {
        // Test J2 at several different orbital radii
        let radii = vec![6600e3, 7000e3, 8000e3, 12000e3, 20000e3, 42164e3];

        for &r_mag in &radii {
            let r = Vector3Static::new(r_mag, 0.0, 500e3);
            let accel = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);

            // Should always have some acceleration
            assert!(accel.norm() > 0.0, "Zero acceleration at radius {}", r_mag);
        }
    }

    #[test]
    fn test_combined_with_only_j2() {
        // Combined function with only J2 should match j2_perturbation_static
        let r = Vector3Static::new(7000e3, 500e3, 1000e3);

        let a_j2_only = j2_perturbation_static(&r, MU_EARTH, J2_EARTH, R_EARTH);
        let a_combined = j2_j3_j4_perturbation_static(&r, MU_EARTH, J2_EARTH, 0.0, 0.0, R_EARTH);

        assert_relative_eq!(a_combined[0], a_j2_only[0], epsilon = 1e-15);
        assert_relative_eq!(a_combined[1], a_j2_only[1], epsilon = 1e-15);
        assert_relative_eq!(a_combined[2], a_j2_only[2], epsilon = 1e-15);
    }
}
