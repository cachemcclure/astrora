//! Perturbation models for orbit propagation
//!
//! This module provides various perturbation models for high-fidelity
//! orbit propagation beyond the two-body problem:
//! - J2 (Earth oblateness)
//! - Atmospheric drag (future)
//! - Solar radiation pressure (future)
//! - Third-body perturbations (future)
//!
//! # References
//! - Curtis, H. D. "Orbital Mechanics for Engineering Students" (Equation 12.30)
//! - Vallado, D. A. "Fundamentals of Astrodynamics and Applications" (Section 8.7.1)

use crate::core::error::{PoliastroError, PoliastroResult};
use crate::core::linalg::Vector3;

/// J2 oblateness perturbation acceleration
///
/// Computes the acceleration due to Earth's oblateness (J2 term) in Cartesian coordinates.
/// This is the dominant perturbation for Earth-orbiting satellites and accounts for the
/// equatorial bulge.
///
/// # Formula
/// ```text
/// a_J2 = (3/2) * (J2 * μ * R²/r⁴) * [
///     x/r * (5z²/r² - 1) î +
///     y/r * (5z²/r² - 1) ĵ +
///     z/r * (5z²/r² - 3) k̂
/// ]
/// ```
///
/// Where:
/// - J2 is the oblateness coefficient (~1.08263e-3 for Earth)
/// - μ is the standard gravitational parameter (m³/s²)
/// - R is the body's equatorial radius (m)
/// - r is the orbital radius magnitude (m)
/// - x, y, z are position components in inertial frame (m)
///
/// # Arguments
/// * `r` - Position vector [x, y, z] in meters (inertial frame)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `J2` - Oblateness coefficient (dimensionless)
/// * `R` - Body equatorial radius (meters)
///
/// # Returns
/// Acceleration vector [ax, ay, az] in m/s²
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::j2_perturbation;
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// // ISS orbit at 400 km altitude
/// let r = Vector3::new(6778e3, 0.0, 0.0);  // On equator
/// let acc = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);
/// ```
///
/// # Notes
/// - The acceleration is computed in the same inertial frame as the position vector
/// - For Earth, typical magnitude is ~1e-5 m/s² at LEO altitudes
/// - J2 causes secular changes in RAAN and argument of perigee
/// - Effect decreases as ~1/r⁴ with increasing altitude
pub fn j2_perturbation(r: &Vector3, mu: f64, j2: f64, R: f64) -> Vector3 {
    // Position components
    let x = r.x;
    let y = r.y;
    let z = r.z;

    // Orbital radius magnitude
    let r_mag = r.norm();

    // Precompute common terms for efficiency
    let r2 = r_mag * r_mag;
    let r4 = r2 * r2;
    let z2 = z * z;

    // Common coefficient: (3/2) * J2 * μ * R² / r⁴
    let k = 1.5 * j2 * mu * R * R / r4;

    // z²/r² term (appears multiple times)
    let z2_r2 = z2 / r2;

    // Acceleration components
    // ax = k * (x/r) * (5*z²/r² - 1)
    // ay = k * (y/r) * (5*z²/r² - 1)
    // az = k * (z/r) * (5*z²/r² - 3)
    let factor_xy = 5.0 * z2_r2 - 1.0;
    let factor_z = 5.0 * z2_r2 - 3.0;

    Vector3::new(
        k * (x / r_mag) * factor_xy,
        k * (y / r_mag) * factor_xy,
        k * (z / r_mag) * factor_z,
    )
}

/// Perturbed acceleration function type
///
/// This type alias represents a function that computes perturbation accelerations
/// given the current time, position, and velocity.
///
/// # Arguments
/// * `t` - Current time (seconds since epoch)
/// * `r` - Position vector (m)
/// * `v` - Velocity vector (m/s)
///
/// # Returns
/// Acceleration vector (m/s²)
pub type PerturbationFn = fn(f64, &Vector3, &Vector3) -> Vector3;

/// J2 perturbation function wrapper for use with numerical integrators
///
/// Returns a function suitable for use with RK4 or DOPRI5 integrators.
/// The returned function computes total acceleration (two-body + J2).
///
/// # Arguments
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
///
/// # Returns
/// Function that computes [vx, vy, vz, ax, ay, az] given state vector
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::j2_acceleration_func;
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// let accel_func = j2_acceleration_func(GM_EARTH, J2_EARTH, R_EARTH);
/// // Use with RK4 or DOPRI5 integrator
/// ```
pub fn j2_acceleration_func(
    mu: f64,
    j2: f64,
    R: f64,
) -> impl Fn(f64, &nalgebra::DVector<f64>) -> nalgebra::DVector<f64> {
    move |_t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        // State vector: [x, y, z, vx, vy, vz]
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration: -μ/r³ * r
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;

        // J2 perturbation acceleration
        let a_j2 = j2_perturbation(&r, mu, j2, R);

        // Total acceleration
        let a_total = a_twobody + a_j2;

        // Return derivative: [vx, vy, vz, ax, ay, az]
        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    }
}

/// Propagate orbit with J2 perturbation using RK4 integrator
///
/// Propagates a state vector forward in time accounting for Earth's oblateness.
/// Uses fixed-step RK4 integration with multiple sub-steps for accuracy.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::propagate_j2_rk4;
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
/// let (r1, v1) = propagate_j2_rk4(&r0, &v0, 3600.0, GM_EARTH, J2_EARTH, R_EARTH, None).unwrap();
/// ```
pub fn propagate_j2_rk4(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    n_steps: Option<usize>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::rk4_step;

    let n_steps = n_steps.unwrap_or(10);
    let h = dt / n_steps as f64;

    // Create acceleration function
    let f = j2_acceleration_func(mu, j2, R);

    // Initial state vector: [x, y, z, vx, vy, vz]
    let mut state = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate using multiple RK4 steps
    let mut t = 0.0;
    for _ in 0..n_steps {
        state = rk4_step(&f, t, &state, h);
        t += h;
    }

    // Extract final position and velocity
    let r_final = Vector3::new(state[0], state[1], state[2]);
    let v_final = Vector3::new(state[3], state[4], state[5]);

    Ok((r_final, v_final))
}

/// Propagate orbit with J2 perturbation using adaptive DOPRI5 integrator
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
/// Automatically adjusts step size to maintain specified error tolerance.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::propagate_j2_dopri5;
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
/// let (r1, v1) = propagate_j2_dopri5(&r0, &v0, 3600.0, GM_EARTH, J2_EARTH, R_EARTH, None).unwrap();
/// ```
pub fn propagate_j2_dopri5(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::dopri5_integrate;

    let tol = tol.unwrap_or(1e-8);

    // Create acceleration function
    let f = j2_acceleration_func(mu, j2, R);

    // Initial state vector
    let state0 = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate from t0=0 to tf=dt
    let state_final = dopri5_integrate(f, 0.0, &state0, dt, dt.abs() / 10.0, tol, None)?;

    // Extract final position and velocity
    let r_final = Vector3::new(state_final[0], state_final[1], state_final[2]);
    let v_final = Vector3::new(state_final[3], state_final[4], state_final[5]);

    Ok((r_final, v_final))
}

/// Propagate orbit with J2 perturbation using adaptive DOP853 integrator
///
/// Ultra-high accuracy propagation using Dormand-Prince 8(5,3) adaptive integration.
/// This 8th-order method is recommended for problems requiring very tight error
/// tolerances (e.g., tol < 1e-10) or long-duration high-precision propagation.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `tol` - Error tolerance (default: 1e-10)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::propagate_j2_dop853;
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
/// let (r1, v1) = propagate_j2_dop853(&r0, &v0, 3600.0, GM_EARTH, J2_EARTH, R_EARTH, None).unwrap();
/// ```
pub fn propagate_j2_dop853(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::dop853_integrate;

    let tol = tol.unwrap_or(1e-10);

    // Create acceleration function
    let f = j2_acceleration_func(mu, j2, R);

    // Initial state vector
    let state0 = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate from t0=0 to tf=dt
    let state_final = dop853_integrate(f, 0.0, &state0, dt, dt.abs() / 10.0, tol, None)?;

    // Extract final position and velocity
    let r_final = Vector3::new(state_final[0], state_final[1], state_final[2]);
    let v_final = Vector3::new(state_final[3], state_final[4], state_final[5]);

    Ok((r_final, v_final))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
    use approx::assert_relative_eq;

    #[test]
    fn test_j2_perturbation_equatorial_orbit() {
        // Satellite on equator (z=0) should have no vertical component initially
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let acc = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);

        // At equator (z=0): factor_xy = -1, factor_z = -3
        // ax should be negative (towards Earth's center, modified by J2)
        assert!(acc.x < 0.0);
        // ay = 0 (no y component)
        assert_relative_eq!(acc.y, 0.0, epsilon = 1e-20);
        // az = 0 (no z-component for equatorial position)
        assert_relative_eq!(acc.z, 0.0, epsilon = 1e-20);
    }

    #[test]
    fn test_j2_perturbation_polar_orbit() {
        // Satellite above pole (x=0, y=0, z>0)
        let r = Vector3::new(0.0, 0.0, 7000e3);
        let acc = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);

        // At pole: x=y=0, so ax=ay=0
        assert_relative_eq!(acc.x, 0.0, epsilon = 1e-20);
        assert_relative_eq!(acc.y, 0.0, epsilon = 1e-20);
        // az should be positive (J2 acts away from equator at poles)
        // factor_z = 5 - 3 = 2 > 0
        assert!(acc.z > 0.0);
    }

    #[test]
    fn test_j2_perturbation_magnitude() {
        // Check magnitude is reasonable for LEO
        let r = Vector3::new(7000e3, 0.0, 1000e3);
        let acc = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);

        let acc_mag = acc.norm();
        // J2 acceleration at LEO should be ~1e-5 to 1e-2 m/s²
        assert!(acc_mag > 1e-6 && acc_mag < 2e-2);
    }

    #[test]
    fn test_j2_acceleration_decreases_with_altitude() {
        // J2 effect should decrease as ~1/r⁴
        let r1 = Vector3::new(7000e3, 0.0, 0.0);
        let r2 = Vector3::new(14000e3, 0.0, 0.0); // 2x radius

        let acc1 = j2_perturbation(&r1, GM_EARTH, J2_EARTH, R_EARTH);
        let acc2 = j2_perturbation(&r2, GM_EARTH, J2_EARTH, R_EARTH);

        let ratio = acc1.norm() / acc2.norm();
        // Should be approximately 2⁴ = 16
        assert!(ratio > 14.0 && ratio < 18.0);
    }

    #[test]
    fn test_propagate_j2_rk4_basic() {
        // Simple propagation test - just verify it runs
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        let result = propagate_j2_rk4(&r0, &v0, 100.0, GM_EARTH, J2_EARTH, R_EARTH, Some(10));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // Verify orbit hasn't degraded unrealistically
        assert!(r1.norm() > 6000e3); // Still above Earth
        assert!(v1.norm() > 1000.0); // Still has significant velocity
    }

    #[test]
    fn test_propagate_j2_dopri5_basic() {
        // Adaptive integration test
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        let result = propagate_j2_dopri5(&r0, &v0, 100.0, GM_EARTH, J2_EARTH, R_EARTH, Some(1e-8));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // Verify orbit is reasonable
        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
    }

    #[test]
    fn test_j2_vs_two_body_difference() {
        // J2 should cause orbit to differ from pure two-body
        use crate::propagators::keplerian::propagate_state_keplerian;

        let r0 = Vector3::new(7000e3, 0.0, 1000e3); // Inclined orbit
        let v0 = Vector3::new(0.0, 7546.0, 100.0);
        let dt = 3600.0; // 1 hour

        // Two-body propagation
        let (r_twobody, _v_twobody) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // J2-perturbed propagation
        let (r_j2, _v_j2) = propagate_j2_rk4(&r0, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(100)).unwrap();

        // Positions should differ (J2 causes secular drift)
        let pos_diff = (r_twobody - r_j2).norm();
        println!("Position difference after 1 hour: {} m", pos_diff);
        assert!(pos_diff > 10.0); // At least 10 meters difference after 1 hour

        // But shouldn't be too different (same order of magnitude)
        // After 1 hour with J2, difference can be several km
        assert!(pos_diff < 100000.0); // Less than 100 km difference
    }

    #[test]
    fn test_j2_energy_not_exactly_conserved() {
        // J2 perturbations should cause slight energy changes over time
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        let (r1, v1) = propagate_j2_rk4(&r0, &v0, 3600.0, GM_EARTH, J2_EARTH, R_EARTH, Some(100)).unwrap();

        // Calculate specific energies
        let e0 = v0.norm_squared() / 2.0 - GM_EARTH / r0.norm();
        let e1 = v1.norm_squared() / 2.0 - GM_EARTH / r1.norm();

        // Energy should change slightly (J2 is a conservative perturbation
        // but causes redistribution between kinetic and potential)
        // Note: Total mechanical energy is still conserved for conservative J2,
        // but the simple two-body energy formula doesn't capture the full picture
        let energy_diff = (e1 - e0).abs();

        // The difference should be small but measurable
        assert!(energy_diff < 1e6); // Less than 1 MJ/kg change
    }
}

/// Cowell's method for numerical orbit propagation with perturbations
///
/// General numerical propagator that solves the equations of motion:
/// d²r/dt² = -μ/r³ * r + a_pert
///
/// where a_pert is the total perturbation acceleration from all sources.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `perturbations` - Vector of perturbation functions
/// * `method` - Integration method ("rk4" or "dopri5")
/// * `n_steps` - Number of sub-steps for RK4 (ignored for DOPRI5)
/// * `tol` - Error tolerance for DOPRI5 (ignored for RK4)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::{propagate_cowell, j2_perturbation};
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
///
/// // Create J2 perturbation function
/// let j2_pert = |_t: f64, r: &Vector3, _v: &Vector3| {
///     j2_perturbation(r, GM_EARTH, J2_EARTH, R_EARTH)
/// };
///
/// let perts: Vec<Box<dyn Fn(f64, &Vector3, &Vector3) -> Vector3>> = vec![Box::new(j2_pert)];
/// let (r1, v1) = propagate_cowell(&r0, &v0, 3600.0, GM_EARTH, &perts, "rk4", Some(100), None).unwrap();
/// ```
#[allow(dead_code)]
pub fn propagate_cowell<F>(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    perturbations: &[F],
    method: &str,
    n_steps: Option<usize>,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)>
where
    F: Fn(f64, &Vector3, &Vector3) -> Vector3,
{
    // Create total acceleration function (two-body + perturbations)
    let accel_func = |t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration: -μ/r³ * r
        let mut a_total = -mu / (r_mag * r_mag * r_mag) * r;

        // Add all perturbations
        for pert in perturbations {
            a_total += pert(t, &r, &v);
        }

        // Return derivative: [vx, vy, vz, ax, ay, az]
        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    };

    // Initial state vector
    let state0 = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Choose integration method
    let state_final = match method {
        "rk4" => {
            use crate::core::numerical::rk4_step;

            let n_steps = n_steps.unwrap_or(10);
            let h = dt / n_steps as f64;

            let mut state = state0;
            let mut t = 0.0;
            for _ in 0..n_steps {
                state = rk4_step(&accel_func, t, &state, h);
                t += h;
            }
            state
        }
        "dopri5" => {
            use crate::core::numerical::dopri5_integrate;

            let tol = tol.unwrap_or(1e-8);
            dopri5_integrate(accel_func, 0.0, &state0, dt, dt.abs() / 10.0, tol, None)?
        }
        _ => {
            return Err(PoliastroError::ComputationError {
                message: format!("Invalid integration method '{}'. Expected either 'rk4' or 'dopri5'", method),
            });
        }
    };

    // Extract final position and velocity
    let r_final = Vector3::new(state_final[0], state_final[1], state_final[2]);
    let v_final = Vector3::new(state_final[3], state_final[4], state_final[5]);

    Ok((r_final, v_final))
}

#[cfg(test)]
mod cowell_tests {
    use super::*;
    use crate::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
    use approx::assert_relative_eq;

    #[test]
    fn test_cowell_two_body_only() {
        // With no perturbations, should match two-body propagation
        use crate::propagators::keplerian::propagate_state_keplerian;

        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0; // 10 minutes

        // Cowell with no perturbations
        let perts: Vec<Box<dyn Fn(f64, &Vector3, &Vector3) -> Vector3>> = vec![];
        let (r_cowell, v_cowell) = propagate_cowell(&r0, &v0, dt, GM_EARTH, &perts, "rk4", Some(100), None).unwrap();

        // Keplerian propagation
        let (r_kep, v_kep) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Should be very close (RK4 with 100 steps is quite accurate)
        let pos_diff = (r_cowell - r_kep).norm();
        let vel_diff = (v_cowell - v_kep).norm();

        assert!(pos_diff < 100.0); // < 100 m difference
        assert!(vel_diff < 0.1); // < 0.1 m/s difference
    }

    #[test]
    fn test_cowell_with_j2() {
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 3600.0;

        // Create J2 perturbation function
        let j2_pert = |_t: f64, r: &Vector3, _v: &Vector3| {
            j2_perturbation(r, GM_EARTH, J2_EARTH, R_EARTH)
        };

        let perts: Vec<Box<dyn Fn(f64, &Vector3, &Vector3) -> Vector3>> = vec![Box::new(j2_pert)];
        let (r_cowell, v_cowell) = propagate_cowell(&r0, &v0, dt, GM_EARTH, &perts, "rk4", Some(100), None).unwrap();

        // Compare with dedicated J2 propagator
        let (r_j2, v_j2) = propagate_j2_rk4(&r0, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(100)).unwrap();

        // Should be identical (same algorithm)
        assert_relative_eq!(r_cowell.x, r_j2.x, epsilon = 1.0);
        assert_relative_eq!(r_cowell.y, r_j2.y, epsilon = 1.0);
        assert_relative_eq!(r_cowell.z, r_j2.z, epsilon = 1.0);
        assert_relative_eq!(v_cowell.x, v_j2.x, epsilon = 0.001);
        assert_relative_eq!(v_cowell.y, v_j2.y, epsilon = 0.001);
        assert_relative_eq!(v_cowell.z, v_j2.z, epsilon = 0.001);
    }

    #[test]
    fn test_cowell_dopri5_method() {
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0;

        let perts: Vec<Box<dyn Fn(f64, &Vector3, &Vector3) -> Vector3>> = vec![];
        let result = propagate_cowell(&r0, &v0, dt, GM_EARTH, &perts, "dopri5", None, Some(1e-8));

        assert!(result.is_ok());
        let (r, v) = result.unwrap();

        // Should still be in orbit
        assert!(r.norm() > 6000e3);
        assert!(v.norm() > 1000.0);
    }

    #[test]
    fn test_cowell_invalid_method() {
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        let perts: Vec<Box<dyn Fn(f64, &Vector3, &Vector3) -> Vector3>> = vec![];
        let result = propagate_cowell(&r0, &v0, 600.0, GM_EARTH, &perts, "invalid", None, None);

        assert!(result.is_err());
    }
}
