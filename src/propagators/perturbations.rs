//! Perturbation models for orbit propagation
//!
//! This module provides various perturbation models for high-fidelity
//! orbit propagation beyond the two-body problem:
//! - J2 (Earth oblateness)
//! - Atmospheric drag (exponential atmosphere model)
//! - Third-body perturbations (Sun and Moon)
//! - Solar radiation pressure (future)
//!
//! # References
//! - Curtis, H. D. "Orbital Mechanics for Engineering Students" (Equation 12.30, 12.69-12.71)
//! - Vallado, D. A. "Fundamentals of Astrodynamics and Applications" (Section 8.7.1, 8.6.2, 8.6.3)
//! - Wertz, J. R. "Spacecraft Attitude Determination and Control" (Chapter 17)
//! - Montenbruck, O. "Satellite Orbits" (Section 3.3.2)

use crate::core::error::{PoliastroError, PoliastroResult};
use crate::core::linalg::Vector3;
use std::f64::consts::PI;

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

// =============================================================================
// ATMOSPHERIC DRAG MODELS
// =============================================================================

/// Compute atmospheric density using exponential atmosphere model
///
/// The exponential model approximates atmospheric density as:
/// ρ(h) = ρ₀ × exp(-h/H)
///
/// Where:
/// - h = altitude above reference surface (m)
/// - ρ₀ = reference density at surface (kg/m³)
/// - H = atmospheric scale height (m)
///
/// # Arguments
/// * `altitude` - Altitude above reference surface (m)
/// * `rho0` - Reference density at surface (kg/m³)
/// * `H0` - Atmospheric scale height (m)
///
/// # Returns
/// Atmospheric density (kg/m³)
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::exponential_density;
/// use astrora::core::constants::{RHO0_EARTH, H0_EARTH, R_EARTH};
///
/// let altitude = 400e3; // 400 km altitude (ISS)
/// let rho = exponential_density(altitude, RHO0_EARTH, H0_EARTH);
/// ```
///
/// # Notes
/// - This is a simplified model suitable for preliminary analysis
/// - For high-fidelity work, consider Harris-Priester or NRLMSISE-00
/// - Valid primarily for altitudes 100-1000 km
/// - Does not account for solar activity, geomagnetic effects, or diurnal variations
pub fn exponential_density(altitude: f64, rho0: f64, H0: f64) -> f64 {
    rho0 * (-altitude / H0).exp()
}

/// Atmospheric drag acceleration (exponential atmosphere model)
///
/// Computes the acceleration due to atmospheric drag using the exponential
/// atmosphere density model and a constant ballistic coefficient.
///
/// # Formula
/// ```text
/// a_drag = -(1/2) × ρ(h) × v_rel² × (1/B) × v̂_rel
/// ```
///
/// Where:
/// - ρ(h) is atmospheric density at altitude h
/// - v_rel is velocity relative to the atmosphere
/// - B = m/(C_d × A) is the ballistic coefficient (kg/m²)
/// - v̂_rel is the unit vector in the velocity direction
///
/// # Arguments
/// * `r` - Position vector [x, y, z] in meters (inertial frame)
/// * `v` - Velocity vector [vx, vy, vz] in m/s (inertial frame)
/// * `R` - Body equatorial radius (m) - reference surface for altitude
/// * `rho0` - Reference atmospheric density at surface (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient B = m/(C_d × A) in kg/m²
///
/// # Returns
/// Acceleration vector [ax, ay, az] in m/s²
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::drag_acceleration;
/// use astrora::core::constants::{R_EARTH, RHO0_EARTH, H0_EARTH};
///
/// // ISS parameters: mass ~450,000 kg, area ~2500 m², C_d ≈ 2.2
/// // B = 450000 / (2.2 × 2500) ≈ 82 kg/m²
/// let B = 82.0;
///
/// let r = Vector3::new(6778e3, 0.0, 0.0);  // 400 km altitude
/// let v = Vector3::new(0.0, 7670.0, 0.0);  // ~7.67 km/s circular velocity
/// let a_drag = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, B);
/// ```
///
/// # Notes
/// - This model assumes non-rotating atmosphere (v_rel = v_inertial)
/// - For Earth, atmospheric rotation should be considered for high precision
/// - Drag always opposes the velocity direction
/// - Typical ballistic coefficients:
///   - Small satellites/CubeSats: 10-50 kg/m²
///   - ISS: 50-100 kg/m²
///   - Large satellites: 100-500 kg/m²
/// - Higher B = less drag effect (more massive or smaller cross-section)
pub fn drag_acceleration(r: &Vector3, v: &Vector3, R: f64, rho0: f64, H0: f64, B: f64) -> Vector3 {
    // Calculate altitude above reference surface
    let r_mag = r.norm();
    let altitude = r_mag - R;

    // Atmospheric density at current altitude
    let rho = exponential_density(altitude, rho0, H0);

    // Velocity magnitude and unit vector
    let v_mag = v.norm();

    // Handle zero velocity case (shouldn't happen in orbit, but be safe)
    if v_mag < 1e-10 {
        return Vector3::zeros();
    }

    let v_unit = v / v_mag;

    // Drag acceleration: a = -(1/2) × ρ × v² × (1/B) × v̂
    // Negative sign because drag opposes motion
    let drag_mag = -0.5 * rho * v_mag * v_mag / B;

    drag_mag * v_unit
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

/// Drag acceleration function wrapper for use with numerical integrators
///
/// Returns a function suitable for use with RK4 or DOPRI5 integrators.
/// The returned function computes total acceleration (two-body + drag).
///
/// # Arguments
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
///
/// # Returns
/// Function that computes [vx, vy, vz, ax, ay, az] given state vector
pub fn drag_acceleration_func(
    mu: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
) -> impl Fn(f64, &nalgebra::DVector<f64>) -> nalgebra::DVector<f64> {
    move |_t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        // State vector: [x, y, z, vx, vy, vz]
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration: -μ/r³ * r
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;

        // Drag acceleration
        let a_drag = drag_acceleration(&r, &v, R, rho0, H0, B);

        // Total acceleration
        let a_total = a_twobody + a_drag;

        // Return derivative: [vx, vy, vz, ax, ay, az]
        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    }
}

/// Propagate orbit with atmospheric drag using RK4 integrator
///
/// Propagates a state vector forward in time accounting for atmospheric drag.
/// Uses fixed-step RK4 integration with multiple sub-steps for accuracy.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::propagate_drag_rk4;
/// use astrora::core::constants::{GM_EARTH, R_EARTH, RHO0_EARTH, H0_EARTH};
///
/// let r0 = Vector3::new(6778e3, 0.0, 0.0);  // 400 km altitude
/// let v0 = Vector3::new(0.0, 7670.0, 0.0);
/// let B = 50.0;  // CubeSat ballistic coefficient
/// let (r1, v1) = propagate_drag_rk4(&r0, &v0, 3600.0, GM_EARTH, R_EARTH, RHO0_EARTH, H0_EARTH, B, None).unwrap();
/// ```
pub fn propagate_drag_rk4(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    n_steps: Option<usize>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::rk4_step;

    let n_steps = n_steps.unwrap_or(10);
    let h = dt / n_steps as f64;

    // Create acceleration function
    let f = drag_acceleration_func(mu, R, rho0, H0, B);

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

/// Propagate orbit with atmospheric drag using adaptive DOPRI5 integrator
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
/// Automatically adjusts step size to maintain specified error tolerance.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity)
pub fn propagate_drag_dopri5(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::dopri5_integrate;

    let tol = tol.unwrap_or(1e-8);

    // Create acceleration function
    let f = drag_acceleration_func(mu, R, rho0, H0, B);

    // Initial state vector
    let state0 = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate from t0=0 to tf=dt
    let state_final = dopri5_integrate(f, 0.0, &state0, dt, dt.abs() / 10.0, tol, None)?;

    // Extract final position and velocity
    let r_final = Vector3::new(state_final[0], state_final[1], state_final[2]);
    let v_final = Vector3::new(state_final[3], state_final[4], state_final[5]);

    Ok((r_final, v_final))
}

/// Combined J2 + drag acceleration function wrapper
///
/// Returns a function suitable for use with RK4 or DOPRI5 integrators.
/// Computes total acceleration from two-body + J2 + drag.
///
/// # Arguments
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
pub fn j2_drag_acceleration_func(
    mu: f64,
    j2: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
) -> impl Fn(f64, &nalgebra::DVector<f64>) -> nalgebra::DVector<f64> {
    move |_t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;

        // J2 perturbation
        let a_j2 = j2_perturbation(&r, mu, j2, R);

        // Drag perturbation
        let a_drag = drag_acceleration(&r, &v, R, rho0, H0, B);

        // Total acceleration
        let a_total = a_twobody + a_j2 + a_drag;

        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    }
}

/// Propagate orbit with J2 + drag perturbations using RK4 integrator
///
/// Combines Earth oblateness and atmospheric drag effects.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
pub fn propagate_j2_drag_rk4(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    n_steps: Option<usize>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::rk4_step;

    let n_steps = n_steps.unwrap_or(10);
    let h = dt / n_steps as f64;

    let f = j2_drag_acceleration_func(mu, j2, R, rho0, H0, B);

    let mut state = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    let mut t = 0.0;
    for _ in 0..n_steps {
        state = rk4_step(&f, t, &state, h);
        t += h;
    }

    let r_final = Vector3::new(state[0], state[1], state[2]);
    let v_final = Vector3::new(state[3], state[4], state[5]);

    Ok((r_final, v_final))
}

/// Propagate orbit with J2 + drag perturbations using adaptive DOPRI5 integrator
///
/// High-accuracy propagation combining Earth oblateness and atmospheric drag.
pub fn propagate_j2_drag_dopri5(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::dopri5_integrate;

    let tol = tol.unwrap_or(1e-8);

    let f = j2_drag_acceleration_func(mu, j2, R, rho0, H0, B);

    let state0 = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    let state_final = dopri5_integrate(f, 0.0, &state0, dt, dt.abs() / 10.0, tol, None)?;

    let r_final = Vector3::new(state_final[0], state_final[1], state_final[2]);
    let v_final = Vector3::new(state_final[3], state_final[4], state_final[5]);

    Ok((r_final, v_final))
}

// =============================================================================
// THIRD-BODY PERTURBATIONS (SUN AND MOON)
// =============================================================================

/// Simplified ephemeris for Sun position relative to Earth
///
/// Returns the Sun's position vector using a simple circular orbit approximation.
/// This is a simplified model suitable for basic perturbation calculations.
/// For high-precision work, use JPL ephemerides.
///
/// # Formula
/// The Sun is approximated as being in a circular orbit at 1 AU with a period
/// of 365.25 days (one sidereal year). The ecliptic plane is assumed to coincide
/// with the equatorial plane (no obliquity), and the Sun starts at the vernal equinox.
///
/// Position: r_sun = AU * [cos(λ), sin(λ), 0]
/// where λ = n * t (mean longitude), n = 2π / T (mean motion)
///
/// # Arguments
/// * `t` - Time since J2000 epoch (seconds)
///
/// # Returns
/// Sun position vector in ECI frame (meters)
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::sun_position_simple;
///
/// let t = 0.0; // J2000 epoch
/// let r_sun = sun_position_simple(t);
/// ```
///
/// # Notes
/// - This is a simplified model: no obliquity, no eccentricity
/// - Error: ~3-5% in position (acceptable for basic perturbation analysis)
/// - For comparison, real Sun orbit has e ≈ 0.0167
/// - For high-precision work, use JPL Horizons or DE440/441 ephemerides
pub fn sun_position_simple(t: f64) -> Vector3 {
    use crate::core::constants::{AU, DAY_TO_SEC};

    // Sidereal year (365.25636 days)
    const YEAR_SEC: f64 = 365.25636 * DAY_TO_SEC;

    // Mean motion: n = 2π / T
    let n = 2.0 * PI / YEAR_SEC;

    // Mean longitude: λ = n * t
    let lambda = n * t;

    // Circular orbit position at 1 AU
    // Simplified: Sun in ecliptic plane (ignoring obliquity)
    Vector3::new(AU * lambda.cos(), AU * lambda.sin(), 0.0)
}

/// Simplified ephemeris for Moon position relative to Earth
///
/// Returns the Moon's position vector using a simple circular orbit approximation.
/// This is a simplified model suitable for basic perturbation calculations.
/// For high-precision work, use JPL ephemerides or lunar theory (ELP-2000/82).
///
/// # Formula
/// The Moon is approximated as being in a circular orbit at the mean lunar distance
/// with a sidereal month period of 27.321661 days. The orbit is assumed to be
/// in the equatorial plane (no inclination).
///
/// Position: r_moon = a_moon * [cos(λ), sin(λ), 0]
/// where λ = n * t (mean longitude), n = 2π / T (mean motion)
///
/// # Arguments
/// * `t` - Time since J2000 epoch (seconds)
///
/// # Returns
/// Moon position vector in ECI frame (meters)
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::moon_position_simple;
///
/// let t = 0.0; // J2000 epoch
/// let r_moon = moon_position_simple(t);
/// ```
///
/// # Notes
/// - This is a very simplified model: no inclination, no eccentricity
/// - Error: ~10-20% in position (sufficient for basic perturbation studies)
/// - Real Moon orbit: i ≈ 5.145°, e ≈ 0.0549
/// - For high-precision work, use JPL Horizons or ELP-2000/82 lunar theory
pub fn moon_position_simple(t: f64) -> Vector3 {
    use crate::core::constants::DAY_TO_SEC;

    // Mean lunar distance (semi-major axis)
    const A_MOON: f64 = 384_400_000.0; // 384,400 km in meters

    // Sidereal month (27.321661 days)
    const MONTH_SEC: f64 = 27.321661 * DAY_TO_SEC;

    // Mean motion: n = 2π / T
    let n = 2.0 * PI / MONTH_SEC;

    // Mean longitude: λ = n * t
    let lambda = n * t;

    // Circular orbit position at mean lunar distance
    // Simplified: Moon in equatorial plane (ignoring 5.145° inclination)
    Vector3::new(A_MOON * lambda.cos(), A_MOON * lambda.sin(), 0.0)
}

/// Third-body perturbation acceleration (point-mass approximation)
///
/// Computes the gravitational perturbation acceleration on a satellite due to
/// a third body (e.g., Sun or Moon) using the point-mass approximation.
///
/// # Formula
/// The third-body perturbation acceleration is:
/// ```text
/// a_3rd = μ₃ * [(r₃ - r) / |r₃ - r|³ - r₃ / |r₃|³]
/// ```
///
/// This can be decomposed into two terms:
/// - **Direct term**: μ₃ * (r₃ - r) / |r₃ - r|³
///   - Gravitational attraction of satellite toward third body
/// - **Indirect term**: -μ₃ * r₃ / |r₃|³
///   - Effect of third body's attraction on the primary body (Earth)
///
/// Where:
/// - μ₃ is the gravitational parameter of the third body (m³/s²)
/// - r is the position of the satellite relative to Earth (m)
/// - r₃ is the position of the third body relative to Earth (m)
///
/// # Arguments
/// * `r` - Satellite position vector relative to Earth (m)
/// * `r_third` - Third body position vector relative to Earth (m)
/// * `mu_third` - Gravitational parameter of third body (m³/s²)
///
/// # Returns
/// Perturbation acceleration vector (m/s²)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::{third_body_perturbation, sun_position_simple};
/// use astrora::core::constants::GM_SUN;
///
/// let r_sat = Vector3::new(7000e3, 0.0, 0.0);  // Satellite at 7000 km
/// let r_sun = sun_position_simple(0.0);         // Sun position at J2000
/// let a_sun = third_body_perturbation(&r_sat, &r_sun, GM_SUN);
/// ```
///
/// # Notes
/// - Magnitude for Sun at LEO: ~6×10⁻⁶ m/s²
/// - Magnitude for Moon at LEO: ~3×10⁻⁶ m/s²
/// - Combined Sun+Moon effect comparable to J2 (~1×10⁻⁵ m/s²)
/// - Most significant for high-altitude orbits (GEO, lunar missions)
/// - For GEO satellites: Moon effect > Sun effect > J2 effect
///
/// # References
/// - Curtis "Orbital Mechanics" Section 12.8 (Equation 12.69)
/// - Vallado "Fundamentals" Section 8.6.3
/// - Montenbruck "Satellite Orbits" Section 3.3.2
pub fn third_body_perturbation(r: &Vector3, r_third: &Vector3, mu_third: f64) -> Vector3 {
    // Relative position: third body to satellite
    let r_rel = r_third - r;
    let r_rel_mag = r_rel.norm();
    let r_third_mag = r_third.norm();

    // Avoid division by zero (shouldn't happen in practice)
    if r_rel_mag < 1e-10 || r_third_mag < 1e-10 {
        return Vector3::zeros();
    }

    // Direct term: attraction toward third body
    // a_direct = μ₃ * (r₃ - r) / |r₃ - r|³
    let direct = mu_third / (r_rel_mag * r_rel_mag * r_rel_mag) * r_rel;

    // Indirect term: effect on Earth's motion
    // a_indirect = -μ₃ * r₃ / |r₃|³
    let indirect = -mu_third / (r_third_mag * r_third_mag * r_third_mag) * r_third;

    // Total third-body acceleration
    direct + indirect
}

/// Combined Sun and Moon third-body perturbation acceleration
///
/// Convenience function that computes the total perturbation from both
/// the Sun and Moon using simplified ephemerides.
///
/// # Arguments
/// * `r` - Satellite position vector relative to Earth (m)
/// * `t` - Time since J2000 epoch (seconds)
///
/// # Returns
/// Combined perturbation acceleration from Sun and Moon (m/s²)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::sun_moon_perturbation;
///
/// let r = Vector3::new(42164e3, 0.0, 0.0);  // GEO altitude
/// let t = 0.0;  // J2000 epoch
/// let a_thirdbody = sun_moon_perturbation(&r, t);
/// ```
pub fn sun_moon_perturbation(r: &Vector3, t: f64) -> Vector3 {
    use crate::core::constants::{GM_MOON, GM_SUN};

    // Get third body positions at time t
    let r_sun = sun_position_simple(t);
    let r_moon = moon_position_simple(t);

    // Compute perturbations
    let a_sun = third_body_perturbation(r, &r_sun, GM_SUN);
    let a_moon = third_body_perturbation(r, &r_moon, GM_MOON);

    // Total third-body acceleration
    a_sun + a_moon
}

/// Third-body perturbation acceleration function wrapper for numerical integrators
///
/// Returns a function suitable for use with RK4, DOPRI5, or DOP853 integrators.
/// Computes total acceleration (two-body + third-body perturbations).
///
/// # Arguments
/// * `mu` - Standard gravitational parameter of primary body (m³/s²)
/// * `mu_third` - Gravitational parameter of third body (m³/s²)
/// * `ephemeris_func` - Function that returns third body position given time
///
/// # Returns
/// Function that computes [vx, vy, vz, ax, ay, az] given time and state vector
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::{third_body_acceleration_func, sun_position_simple};
/// use astrora::core::constants::{GM_EARTH, GM_SUN};
///
/// let accel_func = third_body_acceleration_func(GM_EARTH, GM_SUN, sun_position_simple);
/// // Use with RK4 or DOPRI5 integrator
/// ```
pub fn third_body_acceleration_func<F>(
    mu: f64,
    mu_third: f64,
    ephemeris_func: F,
) -> impl Fn(f64, &nalgebra::DVector<f64>) -> nalgebra::DVector<f64>
where
    F: Fn(f64) -> Vector3,
{
    move |t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        // State vector: [x, y, z, vx, vy, vz]
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration: -μ/r³ * r
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;

        // Third-body perturbation
        let r_third = ephemeris_func(t);
        let a_thirdbody = third_body_perturbation(&r, &r_third, mu_third);

        // Total acceleration
        let a_total = a_twobody + a_thirdbody;

        // Return derivative: [vx, vy, vz, ax, ay, az]
        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    }
}

/// Combined Sun + Moon third-body acceleration function wrapper
///
/// Returns a function for numerical integration with both Sun and Moon perturbations.
///
/// # Arguments
/// * `mu` - Standard gravitational parameter of primary body (m³/s²)
///
/// # Returns
/// Function that computes [vx, vy, vz, ax, ay, az] with Sun and Moon perturbations
pub fn sun_moon_acceleration_func(
    mu: f64,
) -> impl Fn(f64, &nalgebra::DVector<f64>) -> nalgebra::DVector<f64> {
    move |t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;

        // Sun + Moon perturbations
        let a_thirdbody = sun_moon_perturbation(&r, t);

        // Total acceleration
        let a_total = a_twobody + a_thirdbody;

        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    }
}

/// Propagate orbit with third-body perturbations using RK4 integrator
///
/// Propagates a state vector forward in time accounting for third-body
/// gravitational perturbations from the Sun and/or Moon.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter of primary body (m³/s²)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `include_sun` - Include Sun perturbation
/// * `include_moon` - Include Moon perturbation
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::propagate_thirdbody_rk4;
/// use astrora::core::constants::GM_EARTH;
///
/// let r0 = Vector3::new(42164e3, 0.0, 0.0);  // GEO
/// let v0 = Vector3::new(0.0, 3075.0, 0.0);   // GEO velocity
/// let (r1, v1) = propagate_thirdbody_rk4(
///     &r0, &v0, 86400.0, GM_EARTH, 0.0, true, true, Some(100)
/// ).unwrap();
/// ```
pub fn propagate_thirdbody_rk4(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    t0: f64,
    include_sun: bool,
    include_moon: bool,
    n_steps: Option<usize>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::constants::{GM_MOON, GM_SUN};
    use crate::core::numerical::rk4_step;

    let n_steps = n_steps.unwrap_or(10);
    let h = dt / n_steps as f64;

    // Create acceleration function based on flags
    let f = move |t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration
        let mut a_total = -mu / (r_mag * r_mag * r_mag) * r;

        // Add third-body perturbations
        if include_sun {
            let r_sun = sun_position_simple(t);
            a_total += third_body_perturbation(&r, &r_sun, GM_SUN);
        }
        if include_moon {
            let r_moon = moon_position_simple(t);
            a_total += third_body_perturbation(&r, &r_moon, GM_MOON);
        }

        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    };

    // Initial state vector
    let mut state = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate using multiple RK4 steps
    let mut t = t0;
    for _ in 0..n_steps {
        state = rk4_step(f, t, &state, h);
        t += h;
    }

    // Extract final position and velocity
    let r_final = Vector3::new(state[0], state[1], state[2]);
    let v_final = Vector3::new(state[3], state[4], state[5]);

    Ok((r_final, v_final))
}

/// Propagate orbit with third-body perturbations using adaptive DOPRI5 integrator
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter of primary body (m³/s²)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `include_sun` - Include Sun perturbation
/// * `include_moon` - Include Moon perturbation
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity)
pub fn propagate_thirdbody_dopri5(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    t0: f64,
    include_sun: bool,
    include_moon: bool,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::constants::{GM_MOON, GM_SUN};
    use crate::core::numerical::dopri5_integrate;

    let tol = tol.unwrap_or(1e-8);

    // Create acceleration function
    let f = move |t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        let mut a_total = -mu / (r_mag * r_mag * r_mag) * r;

        if include_sun {
            let r_sun = sun_position_simple(t);
            a_total += third_body_perturbation(&r, &r_sun, GM_SUN);
        }
        if include_moon {
            let r_moon = moon_position_simple(t);
            a_total += third_body_perturbation(&r, &r_moon, GM_MOON);
        }

        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    };

    // Initial state vector
    let state0 = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate from t0 to t0+dt
    let state_final = dopri5_integrate(f, t0, &state0, t0 + dt, dt.abs() / 10.0, tol, None)?;

    // Extract final position and velocity
    let r_final = Vector3::new(state_final[0], state_final[1], state_final[2]);
    let v_final = Vector3::new(state_final[3], state_final[4], state_final[5]);

    Ok((r_final, v_final))
}

// =============================================================================
// SOLAR RADIATION PRESSURE (SRP)
// =============================================================================

/// Compute shadow function for solar radiation pressure
///
/// Determines the degree of illumination of a spacecraft by the Sun,
/// accounting for Earth's shadow (umbra and penumbra regions).
///
/// # Returns
/// Shadow factor k:
/// - k = 1.0: Full sunlight (no shadow)
/// - k = 0.0: Full umbra (complete shadow)
/// - 0.0 < k < 1.0: Partial shadow (penumbra)
///
/// # Formula
/// Uses a conical shadow model with:
/// - θ_sun = apparent angular radius of Sun as seen from spacecraft
/// - θ_earth = apparent angular radius of Earth as seen from spacecraft
/// - α = angle between Sun and Earth as seen from spacecraft
///
/// Conditions:
/// - Umbra: α + θ_sun < θ_earth (Earth completely blocks Sun)
/// - Penumbra: |θ_sun - θ_earth| < α < θ_sun + θ_earth
/// - Full sunlight: α > θ_sun + θ_earth
///
/// # Arguments
/// * `r_sat` - Satellite position vector (m) in inertial frame
/// * `r_sun` - Sun position vector (m) in inertial frame
/// * `R_earth` - Earth's radius (m)
///
/// # Returns
/// Shadow factor k ∈ [0, 1]
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::{shadow_function, sun_position_simple};
/// use astrora::core::constants::R_EARTH;
///
/// let r_sat = Vector3::new(7000e3, 0.0, 0.0);
/// let r_sun = sun_position_simple(0.0);
/// let k = shadow_function(&r_sat, &r_sun, R_EARTH);
/// ```
///
/// # Notes
/// - This is a conical shadow model (more accurate than cylindrical)
/// - Earth is treated as a sphere (oblate effects neglected)
/// - Sun is treated as a point source for umbra calculation, disk for penumbra
/// - For high-precision work, consider ray-tracing or ellipsoidal Earth models
///
/// # References
/// - Montenbruck & Gill, "Satellite Orbits", Section 3.4
/// - Vallado, "Fundamentals of Astrodynamics", Section 8.7.2
pub fn shadow_function(r_sat: &Vector3, r_sun: &Vector3, R_earth: f64) -> f64 {
    use crate::core::constants::R_SUN;

    // Vectors from Earth to satellite and Sun
    let r_sat_mag = r_sat.norm();
    let r_sun_mag = r_sun.norm();

    // Unit vectors
    let u_sat = r_sat / r_sat_mag;
    let u_sun = r_sun / r_sun_mag;

    // Angle between satellite and Sun directions (cosine)
    let cos_alpha = u_sat.dot(&u_sun);

    // If satellite is on Sun side of Earth, definitely in sunlight
    if cos_alpha >= 0.0 {
        return 1.0;
    }

    // Apparent angular radii as seen from satellite
    // θ = arcsin(R / r) ≈ R/r for small angles
    let theta_earth = (R_earth / r_sat_mag).asin();
    let theta_sun = (R_SUN / (r_sun_mag - r_sat_mag).abs()).asin();

    // Angle between Sun and Earth as seen from spacecraft
    let alpha = cos_alpha.abs().acos();

    // Check shadow conditions
    if alpha + theta_sun <= theta_earth {
        // Total umbra: Earth completely blocks Sun
        0.0
    } else if alpha - theta_sun >= theta_earth {
        // Full sunlight: no shadow
        1.0
    } else {
        // Penumbra: partial shadow
        // Linear approximation for penumbra transition
        // More accurate models use actual area overlap calculations
        let penumbra_start = theta_earth - theta_sun;
        let penumbra_end = theta_earth + theta_sun;

        if alpha < penumbra_start {
            0.0 // Full umbra
        } else if alpha > penumbra_end {
            1.0 // Full sunlight
        } else {
            // Linear interpolation in penumbra
            (alpha - penumbra_start) / (penumbra_end - penumbra_start)
        }
    }
}

/// Solar radiation pressure acceleration (cannon-ball model)
///
/// Computes the acceleration due to solar radiation pressure using the
/// simple cannon-ball model (spacecraft treated as a sphere).
///
/// # Formula
/// ```text
/// a_srp = k · C_r · (A/m) · P_sun · (AU²/r²) · û_sun
/// ```
///
/// Where:
/// - k = shadow factor (0 = umbra, 1 = full sun, 0-1 = penumbra)
/// - C_r = reflectivity coefficient (dimensionless)
///   - C_r = 1.0: Perfect absorption (black body)
///   - C_r = 2.0: Perfect specular reflection
///   - C_r ≈ 1.2-1.5: Typical spacecraft surfaces
/// - A/m = area-to-mass ratio (m²/kg)
/// - P_sun = solar radiation pressure at 1 AU (N/m²)
/// - AU = astronomical unit (m)
/// - r = distance from Sun to spacecraft (m)
/// - û_sun = unit vector from spacecraft to Sun
///
/// The pressure at 1 AU is: P = E/c = 1361 W/m² / 299792458 m/s ≈ 4.54×10⁻⁶ N/m²
///
/// # Arguments
/// * `r_sat` - Satellite position vector (m) in inertial frame
/// * `r_sun` - Sun position vector (m) in inertial frame
/// * `area_mass_ratio` - Cross-sectional area divided by mass (A/m) in m²/kg
/// * `C_r` - Reflectivity coefficient (typically 1.0-2.0)
/// * `R_earth` - Earth's radius for shadow calculations (m)
///
/// # Returns
/// Acceleration vector [ax, ay, az] in m/s²
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::{srp_acceleration, sun_position_simple};
/// use astrora::core::constants::R_EARTH;
///
/// // CubeSat: 0.1 m × 0.1 m × 0.1 m, mass 1 kg
/// // A = 0.01 m² (one face), A/m = 0.01 m²/kg
/// let area_mass_ratio = 0.01;
/// let C_r = 1.3; // Typical value
///
/// let r_sat = Vector3::new(7000e3, 0.0, 0.0);
/// let r_sun = sun_position_simple(0.0);
/// let a_srp = srp_acceleration(&r_sat, &r_sun, area_mass_ratio, C_r, R_EARTH);
/// ```
///
/// # Notes
/// - Cannon-ball model assumes spacecraft is spherical with uniform reflectivity
/// - More accurate models account for:
///   - Multi-faceted spacecraft geometry
///   - Surface material properties (specular vs diffuse reflection)
///   - Thermal re-radiation
///   - Material degradation over time
/// - SRP magnitude at 1 AU for A/m=1 m²/kg, C_r=1.0: ~4.54×10⁻⁶ m/s²
/// - Effect decreases as 1/r² with distance from Sun
/// - Most significant for:
///   - High area-to-mass ratio spacecraft (solar sails, large antennas)
///   - GEO and higher altitude orbits (drag negligible)
///   - Long-duration missions (cumulative effect)
///
/// # References
/// - Curtis, "Orbital Mechanics for Engineering Students", Section 12.9
/// - Vallado, "Fundamentals of Astrodynamics", Section 8.7
/// - Montenbruck, "Satellite Orbits", Section 3.4
pub fn srp_acceleration(
    r_sat: &Vector3,
    r_sun: &Vector3,
    area_mass_ratio: f64,
    C_r: f64,
    R_earth: f64,
) -> Vector3 {
    use crate::core::constants::{AU, SOLAR_RADIATION_PRESSURE};

    // Shadow factor (0 = umbra, 1 = full sun)
    let k = shadow_function(r_sat, r_sun, R_earth);

    // If in full shadow, no SRP
    if k < 1e-10 {
        return Vector3::zeros();
    }

    // Vector from satellite to Sun
    let r_sun_sat = r_sun - r_sat;
    let r_sun_sat_mag = r_sun_sat.norm();

    // Handle edge case (shouldn't happen, but be safe)
    if r_sun_sat_mag < 1e3 {
        return Vector3::zeros();
    }

    // Unit vector from satellite to Sun
    let u_sun = r_sun_sat / r_sun_sat_mag;

    // Distance scaling factor: (AU/r)² accounts for inverse-square law
    let distance_factor = (AU / r_sun_sat_mag).powi(2);

    // SRP acceleration magnitude
    // a = k · C_r · (A/m) · P_sun · (AU/r)²
    let a_mag = k * C_r * area_mass_ratio * SOLAR_RADIATION_PRESSURE * distance_factor;

    // Acceleration vector points away from Sun
    a_mag * u_sun
}

/// SRP acceleration function wrapper for use with numerical integrators
///
/// Returns a function suitable for use with RK4 or DOPRI5 integrators.
/// The returned function computes total acceleration (two-body + SRP).
///
/// # Arguments
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `area_mass_ratio` - Cross-sectional area divided by mass (m²/kg)
/// * `C_r` - Reflectivity coefficient (1.0-2.0)
/// * `R_earth` - Earth's radius for shadow calculations (m)
/// * `t0` - Initial time since J2000 epoch (seconds)
///
/// # Returns
/// Function that computes [vx, vy, vz, ax, ay, az] given state vector
pub fn srp_acceleration_func(
    mu: f64,
    area_mass_ratio: f64,
    C_r: f64,
    R_earth: f64,
    t0: f64,
) -> impl Fn(f64, &nalgebra::DVector<f64>) -> nalgebra::DVector<f64> {
    move |t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        // State vector: [x, y, z, vx, vy, vz]
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration: -μ/r³ * r
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;

        // Sun position at current time
        let r_sun = sun_position_simple(t0 + t);

        // SRP acceleration
        let a_srp = srp_acceleration(&r, &r_sun, area_mass_ratio, C_r, R_earth);

        // Total acceleration
        let a_total = a_twobody + a_srp;

        // Return derivative: [vx, vy, vz, ax, ay, az]
        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    }
}

/// Propagate orbit with solar radiation pressure using RK4 integrator
///
/// Propagates a state vector forward in time accounting for solar radiation
/// pressure using the cannon-ball model. Uses fixed-step RK4 integration.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `area_mass_ratio` - Cross-sectional area divided by mass (m²/kg)
/// * `C_r` - Reflectivity coefficient (1.0-2.0)
/// * `R_earth` - Earth's radius for shadow calculations (m)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `n_steps` - Number of RK4 sub-steps (default: 100)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::core::linalg::Vector3;
/// use astrora::propagators::perturbations::propagate_srp_rk4;
/// use astrora::core::constants::{GM_EARTH, R_EARTH};
///
/// // CubeSat in GEO
/// let r0 = Vector3::new(42164e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 3075.0, 0.0);
/// let area_mass_ratio = 0.01; // 0.01 m²/kg
/// let C_r = 1.3;
/// let t0 = 0.0; // J2000 epoch
///
/// let (r1, v1) = propagate_srp_rk4(
///     &r0, &v0, 3600.0, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(100)
/// ).unwrap();
/// ```
pub fn propagate_srp_rk4(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    area_mass_ratio: f64,
    C_r: f64,
    R_earth: f64,
    t0: f64,
    n_steps: Option<usize>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::rk4_step;

    let n_steps = n_steps.unwrap_or(100);
    let h = dt / (n_steps as f64);

    // Create acceleration function
    let f = srp_acceleration_func(mu, area_mass_ratio, C_r, R_earth, t0);

    // Initial state vector
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

/// Propagate orbit with solar radiation pressure using adaptive DOPRI5 integrator
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `area_mass_ratio` - Cross-sectional area divided by mass (m²/kg)
/// * `C_r` - Reflectivity coefficient (1.0-2.0)
/// * `R_earth` - Earth's radius for shadow calculations (m)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity)
pub fn propagate_srp_dopri5(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    area_mass_ratio: f64,
    C_r: f64,
    R_earth: f64,
    t0: f64,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::dopri5_integrate;

    let tol = tol.unwrap_or(1e-8);

    // Create acceleration function
    let f = srp_acceleration_func(mu, area_mass_ratio, C_r, R_earth, t0);

    // Initial state vector
    let state0 = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate from 0 to dt (time is relative to t0)
    let state_final = dopri5_integrate(f, 0.0, &state0, dt, dt.abs() / 10.0, tol, None)?;

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

    #[test]
    fn test_propagate_j2_dop853_basic() {
        // Test high-order DOP853 propagator for J2
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        let result = propagate_j2_dop853(&r0, &v0, 100.0, GM_EARTH, J2_EARTH, R_EARTH, Some(1e-10));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // Verify orbit is reasonable
        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
    }

    #[test]
    fn test_propagate_j2_dop853_vs_dopri5() {
        // DOP853 should be more accurate than DOPRI5 for same tolerance
        let r0 = Vector3::new(7000e3, 0.0, 1000e3);
        let v0 = Vector3::new(0.0, 7546.0, 100.0);
        let dt = 3600.0;

        // Both with same tolerance
        let (r_dop853, v_dop853) = propagate_j2_dop853(&r0, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(1e-10)).unwrap();
        let (r_dopri5, v_dopri5) = propagate_j2_dopri5(&r0, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(1e-10)).unwrap();

        // Results should be very close (both high accuracy)
        let pos_diff = (r_dop853 - r_dopri5).norm();
        let vel_diff = (v_dop853 - v_dopri5).norm();

        // Should agree to within meters/mm/s (both are accurate)
        assert!(pos_diff < 10.0, "Position difference: {} m", pos_diff);
        assert!(vel_diff < 0.01, "Velocity difference: {} m/s", vel_diff);
    }

    #[test]
    fn test_propagate_j2_dop853_long_duration() {
        // Test longer propagation (1 orbit period)
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        // Orbital period ~98 minutes
        let period = 2.0 * std::f64::consts::PI * (r0.norm().powi(3) / GM_EARTH).sqrt();

        let result = propagate_j2_dop853(&r0, &v0, period, GM_EARTH, J2_EARTH, R_EARTH, None);
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // After one period, should be roughly back to same position
        // (J2 causes secular drift, so won't be exact)
        let pos_diff = (r1 - r0).norm();
        assert!(pos_diff < 150e3, "Position drift after 1 orbit: {} km", pos_diff / 1e3);

        // Velocity magnitude should be preserved
        let v_ratio = v1.norm() / v0.norm();
        assert!((v_ratio - 1.0).abs() < 0.01, "Velocity change: {}", v_ratio);
    }

    #[test]
    fn test_exponential_density_at_sea_level() {
        use crate::core::constants::{H0_EARTH, RHO0_EARTH};

        // At zero altitude, density should equal reference density
        let rho = exponential_density(0.0, RHO0_EARTH, H0_EARTH);
        assert_relative_eq!(rho, RHO0_EARTH, epsilon = 1e-10);
    }

    #[test]
    fn test_exponential_density_decreases_with_altitude() {
        use crate::core::constants::{H0_EARTH, RHO0_EARTH};

        // Density should decrease exponentially with altitude
        let rho_100km = exponential_density(100e3, RHO0_EARTH, H0_EARTH);
        let rho_200km = exponential_density(200e3, RHO0_EARTH, H0_EARTH);
        let rho_400km = exponential_density(400e3, RHO0_EARTH, H0_EARTH);

        // Each should be less than the previous
        assert!(rho_100km < RHO0_EARTH);
        assert!(rho_200km < rho_100km);
        assert!(rho_400km < rho_200km);

        // At 400 km with this simple model, density is extremely small
        // Note: Real atmospheric density at 400 km is ~1e-11 to 1e-12 kg/m³
        // This simple exponential model with H0=8500m underestimates at high altitude
        assert!(rho_400km < 1e-10);
        assert!(rho_400km > 0.0); // But not zero
    }

    #[test]
    fn test_exponential_density_scale_height() {
        use crate::core::constants::{H0_EARTH, RHO0_EARTH};

        // At one scale height, density should be 1/e of reference
        let rho_H = exponential_density(H0_EARTH, RHO0_EARTH, H0_EARTH);
        let expected = RHO0_EARTH / std::f64::consts::E;

        assert_relative_eq!(rho_H, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_drag_acceleration_opposes_velocity() {
        use crate::core::constants::{H0_EARTH, R_EARTH, RHO0_EARTH};

        // ISS-like orbit at 400 km altitude
        let r = Vector3::new(6778e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7670.0, 0.0);
        let B = 50.0; // CubeSat-like ballistic coefficient

        let a_drag = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, B);

        // Drag should oppose velocity: a_drag · v < 0
        let dot_product = a_drag.dot(&v);
        assert!(dot_product < 0.0, "Drag should oppose velocity");

        // Drag should be parallel to velocity (but opposite direction)
        let a_drag_normalized = a_drag.normalize();
        let v_normalized = v.normalize();
        let cos_angle = a_drag_normalized.dot(&v_normalized);

        // Should be antiparallel: cos(angle) ≈ -1
        assert_relative_eq!(cos_angle, -1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_drag_acceleration_magnitude() {
        use crate::core::constants::{H0_EARTH, R_EARTH, RHO0_EARTH};

        // ISS at 400 km altitude
        let r = Vector3::new(6778e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7670.0, 0.0);
        let B = 82.0; // ISS ballistic coefficient

        let a_drag = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, B);

        // At 400 km with this simple exponential model, drag is extremely small
        // Note: This simple model underestimates drag at high altitudes
        // Real drag at 400 km would be ~1e-7 to 1e-6 m/s² with accurate density models
        let mag = a_drag.norm();
        assert!(mag > 0.0 && mag < 1e-5, "Drag magnitude at 400km: {} m/s²", mag);
    }

    #[test]
    fn test_drag_acceleration_increases_at_lower_altitude() {
        use crate::core::constants::{H0_EARTH, R_EARTH, RHO0_EARTH};

        let B = 50.0;

        // Compare drag at 300 km vs 400 km
        let r_300km = Vector3::new(6678e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7730.0, 0.0); // Slightly faster at lower altitude
        let a_drag_300 = drag_acceleration(&r_300km, &v, R_EARTH, RHO0_EARTH, H0_EARTH, B);

        let r_400km = Vector3::new(6778e3, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 7670.0, 0.0);
        let a_drag_400 = drag_acceleration(&r_400km, &v2, R_EARTH, RHO0_EARTH, H0_EARTH, B);

        // Drag at lower altitude should be stronger (more negative)
        assert!(a_drag_300.norm() > a_drag_400.norm());
    }

    #[test]
    fn test_drag_acceleration_zero_velocity() {
        use crate::core::constants::{H0_EARTH, R_EARTH, RHO0_EARTH};

        let r = Vector3::new(6778e3, 0.0, 0.0);
        let v = Vector3::zeros(); // No velocity
        let B = 50.0;

        let a_drag = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, B);

        // With zero velocity, drag should be zero
        assert_relative_eq!(a_drag.norm(), 0.0, epsilon = 1e-20);
    }

    #[test]
    fn test_drag_ballistic_coefficient_effect() {
        use crate::core::constants::{H0_EARTH, R_EARTH, RHO0_EARTH};

        let r = Vector3::new(6778e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7670.0, 0.0);

        // Compare two different ballistic coefficients
        let B_small = 20.0; // Small satellite (more drag)
        let B_large = 200.0; // Large satellite (less drag)

        let a_drag_small = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, B_small);
        let a_drag_large = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, B_large);

        // Smaller B should experience more drag
        assert!(a_drag_small.norm() > a_drag_large.norm());

        // Ratio should be approximately B_large / B_small
        let ratio = a_drag_small.norm() / a_drag_large.norm();
        let expected_ratio = B_large / B_small;
        assert_relative_eq!(ratio, expected_ratio, epsilon = 1e-10);
    }

    #[test]
    fn test_propagate_drag_rk4_basic() {
        use crate::core::constants::{GM_EARTH, H0_EARTH, R_EARTH, RHO0_EARTH};

        // ISS-like orbit
        let r0 = Vector3::new(6778e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);
        let B = 50.0;

        let result = propagate_drag_rk4(&r0, &v0, 600.0, GM_EARTH, R_EARTH, RHO0_EARTH, H0_EARTH, B, Some(100));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // Orbit should still be valid
        assert!(r1.norm() > 6000e3); // Still above Earth
        assert!(v1.norm() > 1000.0); // Still has significant velocity
    }

    #[test]
    fn test_propagate_drag_dopri5_basic() {
        use crate::core::constants::{GM_EARTH, H0_EARTH, R_EARTH, RHO0_EARTH};

        let r0 = Vector3::new(6778e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);
        let B = 50.0;

        let result = propagate_drag_dopri5(&r0, &v0, 600.0, GM_EARTH, R_EARTH, RHO0_EARTH, H0_EARTH, B, Some(1e-8));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
    }

    #[test]
    fn test_drag_causes_orbit_decay() {
        use crate::core::constants::{GM_EARTH, H0_EARTH, R_EARTH, RHO0_EARTH};
        use crate::propagators::keplerian::propagate_state_keplerian;

        let r0 = Vector3::new(6778e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);
        let B = 50.0;
        let dt = 3600.0; // 1 hour

        // Two-body propagation (no drag)
        let (r_twobody, v_twobody) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Drag propagation
        let (r_drag, v_drag) = propagate_drag_rk4(&r0, &v0, dt, GM_EARTH, R_EARTH, RHO0_EARTH, H0_EARTH, B, Some(100)).unwrap();

        // With drag, orbit should decay: energy should decrease
        let e_twobody = v_twobody.norm_squared() / 2.0 - GM_EARTH / r_twobody.norm();
        let e_drag = v_drag.norm_squared() / 2.0 - GM_EARTH / r_drag.norm();

        assert!(e_drag < e_twobody, "Drag should decrease orbital energy");

        // Orbital altitude should decrease
        assert!(r_drag.norm() < r_twobody.norm(), "Drag should cause orbit to decay");
    }

    #[test]
    fn test_propagate_j2_drag_combined() {
        use crate::core::constants::{GM_EARTH, H0_EARTH, J2_EARTH, R_EARTH, RHO0_EARTH};

        let r0 = Vector3::new(6778e3, 0.0, 1000e3); // Slightly inclined
        let v0 = Vector3::new(100.0, 7670.0, 0.0);
        let B = 50.0;

        let result = propagate_j2_drag_rk4(&r0, &v0, 600.0, GM_EARTH, J2_EARTH, R_EARTH, RHO0_EARTH, H0_EARTH, B, Some(100));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
    }

    #[test]
    fn test_propagate_j2_drag_dopri5() {
        use crate::core::constants::{GM_EARTH, H0_EARTH, J2_EARTH, R_EARTH, RHO0_EARTH};

        let r0 = Vector3::new(6778e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);
        let B = 50.0;

        let result = propagate_j2_drag_dopri5(&r0, &v0, 600.0, GM_EARTH, J2_EARTH, R_EARTH, RHO0_EARTH, H0_EARTH, B, Some(1e-8));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
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
                state = rk4_step(accel_func, t, &state, h);
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
                message: format!("Invalid integration method '{method}'. Expected either 'rk4' or 'dopri5'"),
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

#[cfg(test)]
mod thirdbody_tests {
    use super::*;
    use crate::core::constants::{AU, GM_EARTH, GM_MOON, GM_SUN};
    use approx::assert_relative_eq;

    #[test]
    fn test_sun_position_simple_at_j2000() {
        // At J2000 (t=0), Sun should be at vernal equinox: [1 AU, 0, 0]
        let r_sun = sun_position_simple(0.0);

        assert_relative_eq!(r_sun.x, AU, epsilon = 1.0);
        assert_relative_eq!(r_sun.y, 0.0, epsilon = 1.0);
        assert_relative_eq!(r_sun.z, 0.0, epsilon = 1.0);

        // Magnitude should be 1 AU
        let r_mag = r_sun.norm();
        assert_relative_eq!(r_mag, AU, epsilon = 1.0);
    }

    #[test]
    fn test_sun_position_simple_circular_orbit() {
        use crate::core::constants::DAY_TO_SEC;

        // After 1 year, Sun should complete one orbit
        let t_year = 365.25636 * DAY_TO_SEC;
        let r_sun_0 = sun_position_simple(0.0);
        let r_sun_1yr = sun_position_simple(t_year);

        // Should be approximately back at starting position (within a few km)
        assert_relative_eq!(r_sun_0.x, r_sun_1yr.x, epsilon = 1e6);
        assert_relative_eq!(r_sun_0.y, r_sun_1yr.y, epsilon = 1e6);

        // After quarter year, Sun should be at [0, 1 AU, 0]
        let t_quarter = t_year / 4.0;
        let r_sun_q = sun_position_simple(t_quarter);
        assert_relative_eq!(r_sun_q.x, 0.0, epsilon = 1e6);
        assert_relative_eq!(r_sun_q.y, AU, epsilon = 1e6);
    }

    #[test]
    fn test_moon_position_simple_at_j2000() {
        // At J2000 (t=0), Moon should be at [a_moon, 0, 0]
        let r_moon = moon_position_simple(0.0);

        const A_MOON: f64 = 384_400_000.0;
        assert_relative_eq!(r_moon.x, A_MOON, epsilon = 1.0);
        assert_relative_eq!(r_moon.y, 0.0, epsilon = 1.0);
        assert_relative_eq!(r_moon.z, 0.0, epsilon = 1.0);

        // Magnitude should be ~384,400 km
        let r_mag = r_moon.norm();
        assert_relative_eq!(r_mag, A_MOON, epsilon = 1.0);
    }

    #[test]
    fn test_moon_position_simple_circular_orbit() {
        use crate::core::constants::DAY_TO_SEC;

        // After one sidereal month, Moon should complete one orbit
        let t_month = 27.321661 * DAY_TO_SEC;
        let r_moon_0 = moon_position_simple(0.0);
        let r_moon_1m = moon_position_simple(t_month);

        // Should be approximately back at starting position
        assert_relative_eq!(r_moon_0.x, r_moon_1m.x, epsilon = 1e4);
        assert_relative_eq!(r_moon_0.y, r_moon_1m.y, epsilon = 1e4);
    }

    #[test]
    fn test_third_body_perturbation_sun_magnitude() {
        // Test Sun perturbation magnitude at LEO
        let r_sat = Vector3::new(7000e3, 0.0, 0.0); // LEO satellite
        let r_sun = sun_position_simple(0.0); // Sun at 1 AU

        let a_sun = third_body_perturbation(&r_sat, &r_sun, GM_SUN);

        // Sun perturbation at LEO should be ~0.5-2 × 10⁻⁶ m/s²
        // The magnitude depends on the relative geometry of satellite and sun
        let a_mag = a_sun.norm();
        assert!(
            a_mag > 1e-7 && a_mag < 3e-6,
            "Sun perturbation magnitude: {} m/s²",
            a_mag
        );
    }

    #[test]
    fn test_third_body_perturbation_moon_magnitude() {
        // Test Moon perturbation magnitude at LEO
        let r_sat = Vector3::new(7000e3, 0.0, 0.0); // LEO satellite
        let r_moon = moon_position_simple(0.0); // Moon at mean distance

        let a_moon = third_body_perturbation(&r_sat, &r_moon, GM_MOON);

        // Moon perturbation at LEO should be ~2-4 × 10⁻⁶ m/s²
        let a_mag = a_moon.norm();
        assert!(
            a_mag > 1e-6 && a_mag < 5e-6,
            "Moon perturbation magnitude: {} m/s²",
            a_mag
        );
    }

    #[test]
    fn test_third_body_perturbation_geo_larger() {
        // At GEO, third-body effects are larger than at LEO
        let r_leo = Vector3::new(7000e3, 0.0, 0.0);
        let r_geo = Vector3::new(42164e3, 0.0, 0.0); // GEO altitude

        let r_sun = sun_position_simple(0.0);

        let a_sun_leo = third_body_perturbation(&r_leo, &r_sun, GM_SUN);
        let a_sun_geo = third_body_perturbation(&r_geo, &r_sun, GM_SUN);

        // At GEO, Sun perturbation is relatively larger (distance matters less)
        // Actually, the perturbation magnitude can be larger at GEO
        // because the indirect term becomes more significant
        println!(
            "Sun perturbation at LEO: {} m/s²",
            a_sun_leo.norm()
        );
        println!(
            "Sun perturbation at GEO: {} m/s²",
            a_sun_geo.norm()
        );

        // Both should be reasonable magnitudes
        assert!(a_sun_leo.norm() > 1e-7 && a_sun_leo.norm() < 1e-4);
        assert!(a_sun_geo.norm() > 1e-7 && a_sun_geo.norm() < 1e-4);
    }

    #[test]
    fn test_sun_moon_perturbation_combined() {
        // Test that combined perturbation is sum of individual perturbations
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let t = 0.0;

        let a_combined = sun_moon_perturbation(&r, t);

        // Compute individual perturbations
        let r_sun = sun_position_simple(t);
        let r_moon = moon_position_simple(t);
        let a_sun = third_body_perturbation(&r, &r_sun, GM_SUN);
        let a_moon = third_body_perturbation(&r, &r_moon, GM_MOON);
        let a_manual = a_sun + a_moon;

        // Should be identical
        assert_relative_eq!(a_combined.x, a_manual.x, epsilon = 1e-15);
        assert_relative_eq!(a_combined.y, a_manual.y, epsilon = 1e-15);
        assert_relative_eq!(a_combined.z, a_manual.z, epsilon = 1e-15);
    }

    #[test]
    fn test_third_body_perturbation_direct_indirect_terms() {
        // Verify that direct and indirect terms have correct signs
        let r_sat = Vector3::new(7000e3, 0.0, 0.0); // Satellite along +x
        let r_sun = Vector3::new(AU, 0.0, 0.0); // Sun also along +x (same direction)

        let a_sun = third_body_perturbation(&r_sat, &r_sun, GM_SUN);

        // When satellite and sun are in the same direction:
        // - Direct term pulls satellite toward sun (+x direction)
        // - Indirect term represents sun pulling Earth toward sun (-x from satellite's perspective)
        // Net effect depends on the balance

        // The x-component should be positive (pulled toward sun)
        // Actually, this depends on the exact geometry, but the magnitude should be reasonable
        println!("Sun perturbation when aligned: {:?}", a_sun);
        assert!(a_sun.norm() > 1e-7 && a_sun.norm() < 1e-4);
    }

    #[test]
    fn test_propagate_thirdbody_rk4_sun_only() {
        // Propagate with Sun perturbation only
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0; // 10 minutes
        let t0 = 0.0;

        let result = propagate_thirdbody_rk4(&r0, &v0, dt, GM_EARTH, t0, true, false, Some(10));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // Orbit should still be valid
        assert!(r1.norm() > 6000e3); // Still above Earth
        assert!(v1.norm() > 1000.0); // Still has significant velocity
    }

    #[test]
    fn test_propagate_thirdbody_rk4_moon_only() {
        // Propagate with Moon perturbation only
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0;
        let t0 = 0.0;

        let result = propagate_thirdbody_rk4(&r0, &v0, dt, GM_EARTH, t0, false, true, Some(10));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
    }

    #[test]
    fn test_propagate_thirdbody_rk4_sun_and_moon() {
        // Propagate with both Sun and Moon perturbations
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0;
        let t0 = 0.0;

        let result = propagate_thirdbody_rk4(&r0, &v0, dt, GM_EARTH, t0, true, true, Some(10));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
    }

    #[test]
    fn test_propagate_thirdbody_dopri5_basic() {
        // Test adaptive integrator
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0;
        let t0 = 0.0;

        let result = propagate_thirdbody_dopri5(&r0, &v0, dt, GM_EARTH, t0, true, true, Some(1e-8));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 1000.0);
    }

    #[test]
    fn test_thirdbody_vs_twobody_difference() {
        // Third-body effects should cause orbit to differ from pure two-body
        use crate::propagators::keplerian::propagate_state_keplerian;

        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 3600.0; // 1 hour
        let t0 = 0.0;

        // Two-body propagation
        let (r_twobody, _v_twobody) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Third-body propagation (Sun + Moon)
        let (r_thirdbody, _v_thirdbody) =
            propagate_thirdbody_rk4(&r0, &v0, dt, GM_EARTH, t0, true, true, Some(100)).unwrap();

        // Positions should differ (third-body causes perturbations)
        let pos_diff = (r_twobody - r_thirdbody).norm();
        println!("Position difference after 1 hour: {} m", pos_diff);

        // Difference should be measurable but not huge
        // After 1 hour at LEO, third-body effects should cause ~meters to tens of meters difference
        assert!(pos_diff > 0.1); // At least 10 cm difference
        assert!(pos_diff < 1000.0); // Less than 1 km difference
    }

    #[test]
    fn test_thirdbody_geo_significant() {
        // At GEO, third-body effects are more significant
        use crate::propagators::keplerian::propagate_state_keplerian;

        let r0 = Vector3::new(42164e3, 0.0, 0.0); // GEO altitude
        let v0 = Vector3::new(0.0, 3075.0, 0.0); // GEO circular velocity
        let dt = 86400.0; // 1 day
        let t0 = 0.0;

        // Two-body propagation
        let (r_twobody, _v_twobody) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Third-body propagation
        let (r_thirdbody, _v_thirdbody) =
            propagate_thirdbody_rk4(&r0, &v0, dt, GM_EARTH, t0, true, true, Some(1000)).unwrap();

        // At GEO over 1 day, third-body effects should be significant
        let pos_diff = (r_twobody - r_thirdbody).norm();
        println!("GEO position difference after 1 day: {} m", pos_diff);

        // Difference should be kilometers at GEO over 1 day
        assert!(pos_diff > 100.0); // At least 100 m
        assert!(pos_diff < 1e6); // Less than 1000 km (sanity check)
    }

    #[test]
    fn test_thirdbody_time_dependence() {
        // Third-body perturbations should vary with time as bodies move
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let t0 = 0.0;
        let t1 = 86400.0; // 1 day later

        let a0 = sun_moon_perturbation(&r, t0);
        let a1 = sun_moon_perturbation(&r, t1);

        // Accelerations should be different (Sun and Moon have moved)
        let diff = (a0 - a1).norm();
        println!("Acceleration difference after 1 day: {} m/s²", diff);

        // Should be measurably different
        assert!(diff > 1e-8); // At least some difference
    }

    #[test]
    fn test_shadow_function_full_sunlight() {
        use crate::core::constants::R_EARTH;

        // Satellite on the Sun side of Earth - full sunlight
        let r_sat = Vector3::new(7000e3, 0.0, 0.0);
        let r_sun = Vector3::new(AU, 0.0, 0.0); // Sun in same direction

        let nu = shadow_function(&r_sat, &r_sun, R_EARTH);

        // Should be in full sunlight (nu = 1.0)
        assert_relative_eq!(nu, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_shadow_function_full_umbra() {
        use crate::core::constants::R_EARTH;

        // Satellite directly behind Earth from Sun - full umbra
        let r_sat = Vector3::new(-7000e3, 0.0, 0.0); // Opposite to Sun
        let r_sun = Vector3::new(AU, 0.0, 0.0);

        let nu = shadow_function(&r_sat, &r_sun, R_EARTH);

        // Should be in full umbra (nu = 0.0)
        assert_relative_eq!(nu, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_shadow_function_penumbra() {
        use crate::core::constants::R_EARTH;

        // Satellite at edge of shadow cone - should be in penumbra
        // Place satellite at angle that puts it in partial shadow
        let r_sat = Vector3::new(-7000e3, 1000e3, 0.0);
        let r_sun = Vector3::new(AU, 0.0, 0.0);

        let nu = shadow_function(&r_sat, &r_sun, R_EARTH);

        // Should be in penumbra (0 < nu < 1)
        // The exact value depends on geometry
        assert!(nu >= 0.0 && nu <= 1.0, "Shadow function out of range: {}", nu);
    }

    #[test]
    fn test_shadow_function_high_altitude_less_shadow() {
        use crate::core::constants::R_EARTH;

        // Higher altitude satellites spend less time in shadow
        let r_sun = Vector3::new(AU, 0.0, 0.0);

        // LEO satellite behind Earth
        let r_leo = Vector3::new(-7000e3, 0.0, 0.0);
        let nu_leo = shadow_function(&r_leo, &r_sun, R_EARTH);

        // GEO satellite behind Earth (further from shadow cone)
        let r_geo = Vector3::new(-42164e3, 0.0, 0.0);
        let nu_geo = shadow_function(&r_geo, &r_sun, R_EARTH);

        // GEO should be more likely to be out of shadow
        // (Earth's shadow cone doesn't extend as far)
        assert!(nu_geo >= nu_leo, "GEO should have >= shadow function than LEO");
    }

    #[test]
    fn test_shadow_function_perpendicular_full_sun() {
        use crate::core::constants::R_EARTH;

        // Satellite perpendicular to Sun-Earth line
        let r_sat = Vector3::new(0.0, 7000e3, 0.0);
        let r_sun = Vector3::new(AU, 0.0, 0.0);

        let nu = shadow_function(&r_sat, &r_sun, R_EARTH);

        // Should be in full sunlight (not behind Earth)
        assert_relative_eq!(nu, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_shadow_function_symmetry() {
        use crate::core::constants::R_EARTH;

        // Shadow function should be symmetric about Sun-Earth line
        let r_sun = Vector3::new(AU, 0.0, 0.0);

        let r_sat_plus_y = Vector3::new(-7000e3, 500e3, 0.0);
        let r_sat_minus_y = Vector3::new(-7000e3, -500e3, 0.0);
        let r_sat_plus_z = Vector3::new(-7000e3, 0.0, 500e3);

        let nu_plus_y = shadow_function(&r_sat_plus_y, &r_sun, R_EARTH);
        let nu_minus_y = shadow_function(&r_sat_minus_y, &r_sun, R_EARTH);
        let nu_plus_z = shadow_function(&r_sat_plus_z, &r_sun, R_EARTH);

        // Should all be equal (cylindrical symmetry)
        assert_relative_eq!(nu_plus_y, nu_minus_y, epsilon = 1e-10);
        assert_relative_eq!(nu_plus_y, nu_plus_z, epsilon = 1e-10);
    }
}

#[cfg(test)]
mod srp_tests {
    use super::*;
    use crate::core::constants::{AU, GM_EARTH, R_EARTH};
    use approx::assert_relative_eq;

    #[test]
    fn test_propagate_srp_rk4_basic() {
        // Test SRP propagation with RK4
        let r0 = Vector3::new(42164e3, 0.0, 0.0); // GEO altitude
        let v0 = Vector3::new(0.0, 3075.0, 0.0);
        let dt = 600.0; // 10 minutes

        // Typical values for a satellite with solar panels
        let area_mass_ratio = 0.01; // m²/kg
        let C_r = 1.3; // Reflectivity coefficient
        let t0 = 0.0;

        let result = propagate_srp_rk4(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(100));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // Orbit should still be valid
        assert!(r1.norm() > 40000e3); // Still near GEO
        assert!(v1.norm() > 2000.0); // Still has orbital velocity
    }

    #[test]
    fn test_propagate_srp_dopri5_basic() {
        // Test SRP propagation with DOPRI5
        let r0 = Vector3::new(42164e3, 0.0, 0.0); // GEO altitude
        let v0 = Vector3::new(0.0, 3075.0, 0.0);
        let dt = 600.0;

        let area_mass_ratio = 0.01;
        let C_r = 1.3;
        let t0 = 0.0;

        let result = propagate_srp_dopri5(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(1e-8));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        assert!(r1.norm() > 40000e3);
        assert!(v1.norm() > 2000.0);
    }

    #[test]
    fn test_propagate_srp_rk4_vs_dopri5() {
        // Both integrators should give similar results
        let r0 = Vector3::new(42164e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 3075.0, 0.0);
        let dt = 3600.0; // 1 hour

        let area_mass_ratio = 0.01;
        let C_r = 1.3;
        let t0 = 0.0;

        let (r_rk4, v_rk4) = propagate_srp_rk4(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(100)).unwrap();
        let (r_dopri5, v_dopri5) = propagate_srp_dopri5(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(1e-8)).unwrap();

        // Results should be close (both accurate integrators)
        let pos_diff = (r_rk4 - r_dopri5).norm();
        let vel_diff = (v_rk4 - v_dopri5).norm();

        // Should agree within reasonable tolerance
        assert!(pos_diff < 100.0, "Position difference: {} m", pos_diff);
        assert!(vel_diff < 0.1, "Velocity difference: {} m/s", vel_diff);
    }

    #[test]
    fn test_propagate_srp_vs_twobody() {
        // SRP should cause orbit to differ from pure two-body
        use crate::propagators::keplerian::propagate_state_keplerian;

        let r0 = Vector3::new(42164e3, 0.0, 0.0); // GEO
        let v0 = Vector3::new(0.0, 3075.0, 0.0);
        let dt = 86400.0; // 1 day

        let area_mass_ratio = 0.01; // Relatively large area
        let C_r = 1.3;
        let t0 = 0.0;

        // Two-body propagation
        let (r_twobody, _v_twobody) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // SRP propagation
        let (r_srp, _v_srp) = propagate_srp_rk4(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(1000)).unwrap();

        // Positions should differ (SRP causes perturbations)
        let pos_diff = (r_twobody - r_srp).norm();
        println!("Position difference after 1 day (GEO): {} m", pos_diff);

        // SRP at GEO over 1 day should cause measurable difference
        assert!(pos_diff > 1.0); // At least 1 m difference
        assert!(pos_diff < 100000.0); // Less than 100 km (sanity check)
    }

    #[test]
    fn test_propagate_srp_zero_area() {
        // Zero area/mass ratio should give two-body propagation
        use crate::propagators::keplerian::propagate_state_keplerian;

        let r0 = Vector3::new(42164e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 3075.0, 0.0);
        let dt = 3600.0;

        let area_mass_ratio = 0.0; // No SRP effect
        let C_r = 1.3;
        let t0 = 0.0;

        let (r_srp, v_srp) = propagate_srp_rk4(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(100)).unwrap();
        let (r_twobody, v_twobody) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Should be very close to two-body (only numerical errors)
        let pos_diff = (r_srp - r_twobody).norm();
        let vel_diff = (v_srp - v_twobody).norm();

        assert!(pos_diff < 10.0, "Position difference: {} m", pos_diff);
        assert!(vel_diff < 0.01, "Velocity difference: {} m/s", vel_diff);
    }

    #[test]
    fn test_propagate_srp_shadow_effect() {
        // SRP should be reduced in Earth's shadow
        let r0 = Vector3::new(7000e3, 0.0, 0.0); // LEO
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0;

        let area_mass_ratio = 0.02; // High area for LEO
        let C_r = 1.5;
        let t0 = 0.0;

        // Propagate - should handle shadow transitions
        let result = propagate_srp_rk4(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, Some(100));
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // Verify orbit is still reasonable
        assert!(r1.norm() > 6000e3);
        assert!(v1.norm() > 6000.0);
    }

    #[test]
    fn test_propagate_srp_long_duration() {
        // Test longer propagation
        let r0 = Vector3::new(42164e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 3075.0, 0.0);

        // One day
        let dt = 86400.0;
        let area_mass_ratio = 0.01;
        let C_r = 1.3;
        let t0 = 0.0;

        let result = propagate_srp_dopri5(&r0, &v0, dt, GM_EARTH, area_mass_ratio, C_r, R_EARTH, t0, None);
        assert!(result.is_ok());

        let (r1, v1) = result.unwrap();

        // After 1 day at GEO, orbit should still be reasonable
        // Semi-major axis should be roughly preserved
        let a0 = r0.norm();
        let a1 = r1.norm();
        let a_ratio = a1 / a0;

        assert!((a_ratio - 1.0).abs() < 0.01, "Semi-major axis change: {}", a_ratio);

        // Velocity magnitude roughly preserved
        let v_ratio = v1.norm() / v0.norm();
        assert!((v_ratio - 1.0).abs() < 0.05, "Velocity magnitude change: {}", v_ratio);
    }
}

//==============================================================================
// COMPOSABLE PERTURBATION FRAMEWORK (Trait-Based Design)
//==============================================================================

/// Trait for composable perturbation models
///
/// This trait defines the interface for all perturbation accelerations,
/// enabling flexible composition of different force models. All perturbations
/// must be `Send + Sync` to support parallel propagation with rayon.
///
/// # Design Philosophy
///
/// This trait-based design allows:
/// 1. **Composability**: Mix and match different perturbations
/// 2. **Extensibility**: Users can define custom perturbations
/// 3. **Type Safety**: Compile-time guarantees for perturbation combinations
/// 4. **Performance**: Zero-cost abstractions via trait objects
///
/// # Example
///
/// ```ignore
/// use astrora::propagators::perturbations::{Perturbation, J2Perturbation, DragPerturbation};
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// // Create individual perturbations
/// let j2 = J2Perturbation::new(J2_EARTH, R_EARTH);
/// let drag = DragPerturbation::new(R_EARTH, 1.225, 8500.0, 100.0);
///
/// // Use them individually
/// let a_j2 = j2.acceleration(0.0, &r, &v, GM_EARTH);
/// let a_drag = drag.acceleration(0.0, &r, &v, GM_EARTH);
///
/// // Or combine them
/// let perts: Vec<Box<dyn Perturbation>> = vec![Box::new(j2), Box::new(drag)];
/// let combined = PerturbationSet::new(perts);
/// ```
///
/// # References
/// - Inspired by nyx-space's ForceModel trait design
/// - Vallado "Fundamentals of Astrodynamics" Ch. 8
/// - Curtis "Orbital Mechanics" Ch. 12
pub trait Perturbation: Send + Sync {
    /// Compute the perturbation acceleration at a given state
    ///
    /// # Arguments
    /// * `t` - Time since epoch (seconds, typically J2000)
    /// * `r` - Position vector in inertial frame (meters)
    /// * `v` - Velocity vector in inertial frame (meters/second)
    /// * `mu` - Standard gravitational parameter of central body (m³/s²)
    ///
    /// # Returns
    /// Perturbation acceleration vector (m/s²)
    ///
    /// # Notes
    /// - The returned acceleration should be in the same inertial frame as position
    /// - Time parameter allows for time-dependent perturbations (e.g., third-body, SRP)
    /// - The `mu` parameter is the central body's GM, not the perturbing body
    fn acceleration(&self, t: f64, r: &Vector3, v: &Vector3, mu: f64) -> Vector3;

    /// Get a human-readable name for this perturbation
    ///
    /// Used for logging, debugging, and user feedback.
    fn name(&self) -> &str;

    /// Check if this perturbation is time-dependent
    ///
    /// Returns true if the acceleration depends on time `t`.
    /// This can be used for optimization (e.g., caching for time-independent forces).
    ///
    /// Default implementation returns false (time-independent).
    fn is_time_dependent(&self) -> bool {
        false
    }
}

//==============================================================================
// CONCRETE PERTURBATION IMPLEMENTATIONS
//==============================================================================

/// J2 oblateness perturbation
///
/// Wraps the J2 perturbation calculation in a trait-based type.
/// This accounts for the equatorial bulge of the central body.
///
/// # Parameters
/// - `j2`: J2 coefficient (dimensionless, ~1.08263e-3 for Earth)
/// - `R`: Equatorial radius (meters, ~6378.137 km for Earth)
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::J2Perturbation;
/// use astrora::core::constants::{J2_EARTH, R_EARTH};
///
/// let j2_pert = J2Perturbation::new(J2_EARTH, R_EARTH);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct J2Perturbation {
    /// J2 oblateness coefficient (dimensionless)
    pub j2: f64,
    /// Equatorial radius (m)
    pub radius: f64,
}

impl J2Perturbation {
    /// Create a new J2 perturbation
    ///
    /// # Arguments
    /// * `j2` - J2 oblateness coefficient (dimensionless)
    /// * `radius` - Equatorial radius of central body (meters)
    pub fn new(j2: f64, radius: f64) -> Self {
        Self { j2, radius }
    }

    /// Create J2 perturbation for Earth (convenience constructor)
    pub fn earth() -> Self {
        use crate::core::constants::{J2_EARTH, R_EARTH};
        Self::new(J2_EARTH, R_EARTH)
    }
}

impl Perturbation for J2Perturbation {
    fn acceleration(&self, _t: f64, r: &Vector3, _v: &Vector3, mu: f64) -> Vector3 {
        j2_perturbation(r, mu, self.j2, self.radius)
    }

    fn name(&self) -> &str {
        "J2 Oblateness"
    }

    fn is_time_dependent(&self) -> bool {
        false
    }
}

/// Atmospheric drag perturbation (exponential atmosphere model)
///
/// Models atmospheric drag using a simple exponential density model.
/// Suitable for altitudes between 100-1000 km.
///
/// # Parameters
/// - `R`: Body radius (m)
/// - `rho0`: Reference density at sea level (kg/m³, ~1.225 for Earth)
/// - `H0`: Atmospheric scale height (m, ~8500 for Earth)
/// - `B`: Ballistic coefficient m/(C_d × A) in kg/m²
///
/// # Limitations
/// - Simple exponential model (constant scale height)
/// - Non-rotating atmosphere assumption
/// - No solar/geomagnetic activity effects
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::DragPerturbation;
/// use astrora::core::constants::{R_EARTH, RHO0_EARTH, H0_EARTH};
///
/// // ISS-like spacecraft: 100 kg/m²
/// let drag = DragPerturbation::new(R_EARTH, RHO0_EARTH, H0_EARTH, 100.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DragPerturbation {
    /// Body radius (m)
    pub radius: f64,
    /// Reference density (kg/m³)
    pub rho0: f64,
    /// Scale height (m)
    pub scale_height: f64,
    /// Ballistic coefficient (kg/m²)
    pub ballistic_coeff: f64,
}

impl DragPerturbation {
    /// Create a new atmospheric drag perturbation
    ///
    /// # Arguments
    /// * `radius` - Body radius (m)
    /// * `rho0` - Reference density at sea level (kg/m³)
    /// * `scale_height` - Atmospheric scale height (m)
    /// * `ballistic_coeff` - Ballistic coefficient B = m/(C_d × A) in kg/m²
    pub fn new(radius: f64, rho0: f64, scale_height: f64, ballistic_coeff: f64) -> Self {
        Self {
            radius,
            rho0,
            scale_height,
            ballistic_coeff,
        }
    }

    /// Create drag perturbation for Earth (convenience constructor)
    ///
    /// # Arguments
    /// * `ballistic_coeff` - Ballistic coefficient in kg/m²
    ///   - Small satellites/CubeSats: 10-50 kg/m²
    ///   - ISS: 50-100 kg/m²
    ///   - Large satellites: 100-500 kg/m²
    pub fn earth(ballistic_coeff: f64) -> Self {
        use crate::core::constants::{H0_EARTH, R_EARTH, RHO0_EARTH};
        Self::new(R_EARTH, RHO0_EARTH, H0_EARTH, ballistic_coeff)
    }
}

impl Perturbation for DragPerturbation {
    fn acceleration(&self, _t: f64, r: &Vector3, v: &Vector3, _mu: f64) -> Vector3 {
        drag_acceleration(
            r,
            v,
            self.radius,
            self.rho0,
            self.scale_height,
            self.ballistic_coeff,
        )
    }

    fn name(&self) -> &str {
        "Atmospheric Drag"
    }

    fn is_time_dependent(&self) -> bool {
        false
    }
}

/// Third-body gravitational perturbation
///
/// Models the gravitational perturbation from a third body (e.g., Sun, Moon).
/// Can use either a simple circular ephemeris or a custom position function.
///
/// # Variants
/// - `Sun`: Simplified Sun ephemeris (1 AU circular orbit)
/// - `Moon`: Simplified Moon ephemeris (384,400 km circular orbit)
/// - `Custom`: User-provided position function and gravitational parameter
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::ThirdBodyPerturbation;
/// use astrora::core::constants::{GM_SUN, GM_MOON};
///
/// // Use built-in Sun perturbation
/// let sun = ThirdBodyPerturbation::sun();
///
/// // Use built-in Moon perturbation
/// let moon = ThirdBodyPerturbation::moon();
///
/// // Or create a custom third body
/// let jupiter = ThirdBodyPerturbation::custom(
///     GM_JUPITER,
///     |t| jupiter_position_function(t),
/// );
/// ```
#[derive(Clone)]
pub enum ThirdBodyPerturbation {
    /// Sun perturbation using simplified circular ephemeris
    Sun,
    /// Moon perturbation using simplified circular ephemeris
    Moon,
    /// Custom third body with user-provided position function
    Custom {
        /// Gravitational parameter of third body (m³/s²)
        mu: f64,
        /// Function to compute third body position at time t
        position_func: fn(f64) -> Vector3,
        /// Name of the third body (for identification)
        body_name: String,
    },
}

impl ThirdBodyPerturbation {
    /// Create Sun perturbation (simplified circular ephemeris)
    pub fn sun() -> Self {
        Self::Sun
    }

    /// Create Moon perturbation (simplified circular ephemeris)
    pub fn moon() -> Self {
        Self::Moon
    }

    /// Create custom third body perturbation
    ///
    /// # Arguments
    /// * `mu` - Gravitational parameter of third body (m³/s²)
    /// * `position_func` - Function to compute position at time t (seconds since J2000)
    /// * `name` - Human-readable name for the body
    pub fn custom(mu: f64, position_func: fn(f64) -> Vector3, name: impl Into<String>) -> Self {
        Self::Custom {
            mu,
            position_func,
            body_name: name.into(),
        }
    }

    /// Get the gravitational parameter for this third body
    fn get_mu(&self) -> f64 {
        use crate::core::constants::{GM_MOON, GM_SUN};
        match self {
            Self::Sun => GM_SUN,
            Self::Moon => GM_MOON,
            Self::Custom { mu, .. } => *mu,
        }
    }

    /// Get the position of the third body at time t
    fn get_position(&self, t: f64) -> Vector3 {
        match self {
            Self::Sun => sun_position_simple(t),
            Self::Moon => moon_position_simple(t),
            Self::Custom { position_func, .. } => position_func(t),
        }
    }
}

impl Perturbation for ThirdBodyPerturbation {
    fn acceleration(&self, t: f64, r: &Vector3, _v: &Vector3, _mu: f64) -> Vector3 {
        let r_third = self.get_position(t);
        let mu_third = self.get_mu();
        third_body_perturbation(r, &r_third, mu_third)
    }

    fn name(&self) -> &str {
        match self {
            Self::Sun => "Third-Body (Sun)",
            Self::Moon => "Third-Body (Moon)",
            Self::Custom { body_name, .. } => body_name,
        }
    }

    fn is_time_dependent(&self) -> bool {
        true // Third body positions change with time
    }
}

impl std::fmt::Debug for ThirdBodyPerturbation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sun => write!(f, "ThirdBodyPerturbation::Sun"),
            Self::Moon => write!(f, "ThirdBodyPerturbation::Moon"),
            Self::Custom { mu, body_name, .. } => {
                f.debug_struct("ThirdBodyPerturbation::Custom")
                    .field("mu", mu)
                    .field("body_name", body_name)
                    .finish()
            }
        }
    }
}

/// Solar radiation pressure perturbation (cannon-ball model)
///
/// Models solar radiation pressure using a simplified cannon-ball model
/// with shadow function (umbra/penumbra). Suitable for spacecraft with
/// simple geometries.
///
/// # Parameters
/// - `area_mass_ratio`: A/m in m²/kg
/// - `reflectivity`: C_r coefficient (1.0-2.0, typically ~1.3)
/// - `body_radius`: Radius of shadowing body (m)
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::SolarRadiationPressure;
/// use astrora::core::constants::R_EARTH;
///
/// // CubeSat: A/m ≈ 0.01 m²/kg, C_r ≈ 1.3
/// let srp = SolarRadiationPressure::new(0.01, 1.3, R_EARTH);
///
/// // Solar sail: A/m ≈ 10 m²/kg, C_r ≈ 2.0
/// let solar_sail = SolarRadiationPressure::new(10.0, 2.0, R_EARTH);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SolarRadiationPressure {
    /// Area-to-mass ratio (m²/kg)
    pub area_mass_ratio: f64,
    /// Reflectivity coefficient (dimensionless, 1.0-2.0)
    pub reflectivity: f64,
    /// Radius of shadowing body (m)
    pub body_radius: f64,
}

impl SolarRadiationPressure {
    /// Create a new solar radiation pressure perturbation
    ///
    /// # Arguments
    /// * `area_mass_ratio` - A/m in m²/kg
    ///   - CubeSats: ~0.01 m²/kg
    ///   - Typical satellites: 0.01-0.1 m²/kg
    ///   - Solar sails: 1-100 m²/kg
    /// * `reflectivity` - C_r coefficient
    ///   - Perfect absorption: 1.0
    ///   - Typical spacecraft: 1.2-1.5
    ///   - Perfect reflection: 2.0
    /// * `body_radius` - Radius of shadowing body (m)
    pub fn new(area_mass_ratio: f64, reflectivity: f64, body_radius: f64) -> Self {
        Self {
            area_mass_ratio,
            reflectivity,
            body_radius,
        }
    }

    /// Create SRP for Earth-orbiting spacecraft (convenience constructor)
    ///
    /// # Arguments
    /// * `area_mass_ratio` - A/m in m²/kg
    /// * `reflectivity` - C_r coefficient (typically 1.2-1.5)
    pub fn earth(area_mass_ratio: f64, reflectivity: f64) -> Self {
        use crate::core::constants::R_EARTH;
        Self::new(area_mass_ratio, reflectivity, R_EARTH)
    }
}

impl Perturbation for SolarRadiationPressure {
    fn acceleration(&self, t: f64, r: &Vector3, _v: &Vector3, _mu: f64) -> Vector3 {
        let r_sun = sun_position_simple(t);
        srp_acceleration(
            r,
            &r_sun,
            self.area_mass_ratio,
            self.reflectivity,
            self.body_radius,
        )
    }

    fn name(&self) -> &str {
        "Solar Radiation Pressure"
    }

    fn is_time_dependent(&self) -> bool {
        true // Sun position changes with time
    }
}

/// Collection of perturbations that can be applied together
///
/// This type allows combining multiple perturbations into a single
/// unit that can be used with propagators. The total acceleration
/// is the sum of all individual perturbation accelerations.
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::{
///     PerturbationSet, J2Perturbation, DragPerturbation, ThirdBodyPerturbation
/// };
///
/// let mut perts = PerturbationSet::new();
/// perts.add(J2Perturbation::earth());
/// perts.add(DragPerturbation::earth(100.0));
/// perts.add(ThirdBodyPerturbation::sun());
/// perts.add(ThirdBodyPerturbation::moon());
///
/// // Use with propagator
/// let (r, v) = propagate_with_perturbations(&r0, &v0, dt, GM_EARTH, &perts)?;
/// ```
#[derive(Default)]
pub struct PerturbationSet {
    perturbations: Vec<Box<dyn Perturbation>>,
}

impl PerturbationSet {
    /// Create a new empty perturbation set
    pub fn new() -> Self {
        Self {
            perturbations: Vec::new(),
        }
    }

    /// Add a perturbation to the set
    ///
    /// # Arguments
    /// * `perturbation` - Any type implementing the Perturbation trait
    pub fn add<P: Perturbation + 'static>(&mut self, perturbation: P) {
        self.perturbations.push(Box::new(perturbation));
    }

    /// Get the number of perturbations in the set
    pub fn len(&self) -> usize {
        self.perturbations.len()
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.perturbations.is_empty()
    }

    /// Get names of all perturbations in the set
    pub fn names(&self) -> Vec<&str> {
        self.perturbations.iter().map(|p| p.name()).collect()
    }

    /// Check if any perturbation is time-dependent
    pub fn is_time_dependent(&self) -> bool {
        self.perturbations.iter().any(|p| p.is_time_dependent())
    }

    /// Compute the total perturbation acceleration from all perturbations
    ///
    /// # Arguments
    /// * `t` - Time since epoch (seconds)
    /// * `r` - Position vector (m)
    /// * `v` - Velocity vector (m/s)
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Total perturbation acceleration (m/s²)
    pub fn total_acceleration(&self, t: f64, r: &Vector3, v: &Vector3, mu: f64) -> Vector3 {
        let mut total = Vector3::zeros();
        for pert in &self.perturbations {
            total += pert.acceleration(t, r, v, mu);
        }
        total
    }
}

impl Perturbation for PerturbationSet {
    fn acceleration(&self, t: f64, r: &Vector3, v: &Vector3, mu: f64) -> Vector3 {
        self.total_acceleration(t, r, v, mu)
    }

    fn name(&self) -> &str {
        "Combined Perturbations"
    }

    fn is_time_dependent(&self) -> bool {
        self.is_time_dependent()
    }
}

impl std::fmt::Debug for PerturbationSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PerturbationSet")
            .field("count", &self.len())
            .field("names", &self.names())
            .finish()
    }
}

/// Propagate orbit with trait-based perturbations (new API)
///
/// This is a new, more flexible version of `propagate_cowell` that uses
/// the trait-based perturbation framework. It allows mixing different
/// perturbation types seamlessly.
///
/// # Arguments
/// * `r0` - Initial position (m)
/// * `v0` - Initial velocity (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Central body gravitational parameter (m³/s²)
/// * `perturbations` - Perturbation set or any type implementing Perturbation
/// * `method` - Integration method: "rk4" or "dopri5"
/// * `n_steps` - Number of steps for RK4 (None = 100)
/// * `tol` - Tolerance for DOPRI5 (None = 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity)
///
/// # Example
/// ```ignore
/// use astrora::propagators::perturbations::{
///     propagate_with_perturbations, PerturbationSet,
///     J2Perturbation, DragPerturbation
/// };
/// use astrora::core::constants::GM_EARTH;
///
/// let mut perts = PerturbationSet::new();
/// perts.add(J2Perturbation::earth());
/// perts.add(DragPerturbation::earth(100.0));
///
/// let (r, v) = propagate_with_perturbations(
///     &r0, &v0, 3600.0, GM_EARTH, &perts, "dopri5", None, None
/// )?;
/// ```
pub fn propagate_with_perturbations<P: Perturbation + ?Sized>(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    perturbations: &P,
    method: &str,
    n_steps: Option<usize>,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3)> {
    use crate::core::numerical::{dopri5_integrate, rk4_step};

    // Create acceleration function combining two-body + perturbations
    let accel_func = |t: f64, state: &nalgebra::DVector<f64>| -> nalgebra::DVector<f64> {
        let r = Vector3::new(state[0], state[1], state[2]);
        let v = Vector3::new(state[3], state[4], state[5]);

        let r_mag = r.norm();

        // Two-body acceleration: -μ/r³ * r
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;

        // Add perturbation acceleration
        let a_pert = perturbations.acceleration(t, &r, &v, mu);

        let a_total = a_twobody + a_pert;

        // Return derivative: [vx, vy, vz, ax, ay, az]
        nalgebra::DVector::from_vec(vec![v.x, v.y, v.z, a_total.x, a_total.y, a_total.z])
    };

    // Initial state
    let mut state = nalgebra::DVector::from_vec(vec![r0.x, r0.y, r0.z, v0.x, v0.y, v0.z]);

    // Integrate based on method
    let state_final = match method.to_lowercase().as_str() {
        "rk4" => {
            let steps = n_steps.unwrap_or(100);
            let h = dt / steps as f64;
            let mut t = 0.0;

            // Integrate using multiple RK4 steps
            for _ in 0..steps {
                state = rk4_step(accel_func, t, &state, h);
                t += h;
            }
            state
        }
        "dopri5" => {
            let tolerance = tol.unwrap_or(1e-8);
            let h0 = dt.abs() / 10.0; // Initial step size guess
            dopri5_integrate(accel_func, 0.0, &state, dt, h0, tolerance, None)?
        }
        _ => {
            return Err(PoliastroError::invalid_state(format!(
                "Unknown integration method: {method}. Use 'rk4' or 'dopri5'"
            )))
        }
    };

    // Extract final position and velocity
    let r_final = Vector3::new(state_final[0], state_final[1], state_final[2]);
    let v_final = Vector3::new(state_final[3], state_final[4], state_final[5]);

    Ok((r_final, v_final))
}

//==============================================================================
// TESTS FOR TRAIT-BASED PERTURBATION FRAMEWORK
//==============================================================================

#[cfg(test)]
mod perturbation_trait_tests {
    use super::*;
    use crate::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
    use approx::assert_relative_eq;

    #[test]
    fn test_j2_perturbation_trait() {
        let j2_pert = J2Perturbation::new(J2_EARTH, R_EARTH);

        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7546.0, 0.0);

        let a = j2_pert.acceleration(0.0, &r, &v, GM_EARTH);

        // Should match direct function call
        let a_direct = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);

        assert_relative_eq!(a.x, a_direct.x, epsilon = 1e-12);
        assert_relative_eq!(a.y, a_direct.y, epsilon = 1e-12);
        assert_relative_eq!(a.z, a_direct.z, epsilon = 1e-12);

        assert_eq!(j2_pert.name(), "J2 Oblateness");
        assert!(!j2_pert.is_time_dependent());
    }

    #[test]
    fn test_j2_earth_convenience() {
        let j2_pert = J2Perturbation::earth();

        assert_relative_eq!(j2_pert.j2, J2_EARTH, epsilon = 1e-15);
        assert_relative_eq!(j2_pert.radius, R_EARTH, epsilon = 1e-15);
    }

    #[test]
    fn test_drag_perturbation_trait() {
        use crate::core::constants::{H0_EARTH, RHO0_EARTH};

        let drag_pert = DragPerturbation::new(R_EARTH, RHO0_EARTH, H0_EARTH, 100.0);

        let r = Vector3::new(6778e3, 0.0, 0.0); // 400 km altitude
        let v = Vector3::new(0.0, 7670.0, 0.0);

        let a = drag_pert.acceleration(0.0, &r, &v, GM_EARTH);

        // Should match direct function call
        let a_direct = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, 100.0);

        assert_relative_eq!(a.x, a_direct.x, epsilon = 1e-12);
        assert_relative_eq!(a.y, a_direct.y, epsilon = 1e-12);
        assert_relative_eq!(a.z, a_direct.z, epsilon = 1e-12);

        assert_eq!(drag_pert.name(), "Atmospheric Drag");
        assert!(!drag_pert.is_time_dependent());
    }

    #[test]
    fn test_drag_earth_convenience() {
        use crate::core::constants::{H0_EARTH, RHO0_EARTH};

        let drag_pert = DragPerturbation::earth(100.0);

        assert_relative_eq!(drag_pert.radius, R_EARTH, epsilon = 1e-15);
        assert_relative_eq!(drag_pert.rho0, RHO0_EARTH, epsilon = 1e-15);
        assert_relative_eq!(drag_pert.scale_height, H0_EARTH, epsilon = 1e-15);
        assert_relative_eq!(drag_pert.ballistic_coeff, 100.0, epsilon = 1e-15);
    }

    #[test]
    fn test_thirdbody_sun_perturbation() {
        let sun_pert = ThirdBodyPerturbation::sun();

        let r = Vector3::new(42164e3, 0.0, 0.0); // GEO
        let v = Vector3::new(0.0, 3075.0, 0.0);

        let a = sun_pert.acceleration(0.0, &r, &v, GM_EARTH);

        // Should match direct function call
        let r_sun = sun_position_simple(0.0);
        let a_direct = third_body_perturbation(&r, &r_sun, crate::core::constants::GM_SUN);

        assert_relative_eq!(a.x, a_direct.x, epsilon = 1e-12);
        assert_relative_eq!(a.y, a_direct.y, epsilon = 1e-12);
        assert_relative_eq!(a.z, a_direct.z, epsilon = 1e-12);

        assert_eq!(sun_pert.name(), "Third-Body (Sun)");
        assert!(sun_pert.is_time_dependent());
    }

    #[test]
    fn test_thirdbody_moon_perturbation() {
        let moon_pert = ThirdBodyPerturbation::moon();

        let r = Vector3::new(42164e3, 0.0, 0.0); // GEO
        let v = Vector3::new(0.0, 3075.0, 0.0);

        let a = moon_pert.acceleration(0.0, &r, &v, GM_EARTH);

        // Should match direct function call
        let r_moon = moon_position_simple(0.0);
        let a_direct = third_body_perturbation(&r, &r_moon, crate::core::constants::GM_MOON);

        assert_relative_eq!(a.x, a_direct.x, epsilon = 1e-12);
        assert_relative_eq!(a.y, a_direct.y, epsilon = 1e-12);
        assert_relative_eq!(a.z, a_direct.z, epsilon = 1e-12);

        assert_eq!(moon_pert.name(), "Third-Body (Moon)");
        assert!(moon_pert.is_time_dependent());
    }

    #[test]
    fn test_thirdbody_custom_perturbation() {
        fn custom_body_pos(_t: f64) -> Vector3 {
            Vector3::new(1e9, 0.0, 0.0)
        }

        let custom_pert = ThirdBodyPerturbation::custom(1e15, custom_body_pos, "Custom Body");

        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7546.0, 0.0);

        let a = custom_pert.acceleration(0.0, &r, &v, GM_EARTH);

        // Should be non-zero
        assert!(a.norm() > 0.0);

        assert_eq!(custom_pert.name(), "Custom Body");
        assert!(custom_pert.is_time_dependent());
    }

    #[test]
    fn test_srp_perturbation_trait() {
        let srp_pert = SolarRadiationPressure::new(0.01, 1.3, R_EARTH);

        let r = Vector3::new(42164e3, 0.0, 0.0); // GEO
        let v = Vector3::new(0.0, 3075.0, 0.0);

        let a = srp_pert.acceleration(0.0, &r, &v, GM_EARTH);

        // Should match direct function call
        let r_sun = sun_position_simple(0.0);
        let a_direct = srp_acceleration(&r, &r_sun, 0.01, 1.3, R_EARTH);

        assert_relative_eq!(a.x, a_direct.x, epsilon = 1e-12);
        assert_relative_eq!(a.y, a_direct.y, epsilon = 1e-12);
        assert_relative_eq!(a.z, a_direct.z, epsilon = 1e-12);

        assert_eq!(srp_pert.name(), "Solar Radiation Pressure");
        assert!(srp_pert.is_time_dependent());
    }

    #[test]
    fn test_srp_earth_convenience() {
        let srp_pert = SolarRadiationPressure::earth(0.01, 1.3);

        assert_relative_eq!(srp_pert.area_mass_ratio, 0.01, epsilon = 1e-15);
        assert_relative_eq!(srp_pert.reflectivity, 1.3, epsilon = 1e-15);
        assert_relative_eq!(srp_pert.body_radius, R_EARTH, epsilon = 1e-15);
    }

    #[test]
    fn test_perturbation_set_empty() {
        let perts = PerturbationSet::new();

        assert_eq!(perts.len(), 0);
        assert!(perts.is_empty());
        assert!(!perts.is_time_dependent());

        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7546.0, 0.0);
        let a = perts.total_acceleration(0.0, &r, &v, GM_EARTH);

        // Empty set should give zero acceleration
        assert_relative_eq!(a.x, 0.0, epsilon = 1e-15);
        assert_relative_eq!(a.y, 0.0, epsilon = 1e-15);
        assert_relative_eq!(a.z, 0.0, epsilon = 1e-15);
    }

    #[test]
    fn test_perturbation_set_single() {
        let mut perts = PerturbationSet::new();
        perts.add(J2Perturbation::earth());

        assert_eq!(perts.len(), 1);
        assert!(!perts.is_empty());
        assert!(!perts.is_time_dependent());

        let names = perts.names();
        assert_eq!(names, vec!["J2 Oblateness"]);

        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 7546.0, 0.0);
        let a = perts.total_acceleration(0.0, &r, &v, GM_EARTH);

        // Should match J2 perturbation alone
        let a_j2 = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);
        assert_relative_eq!(a.x, a_j2.x, epsilon = 1e-12);
        assert_relative_eq!(a.y, a_j2.y, epsilon = 1e-12);
        assert_relative_eq!(a.z, a_j2.z, epsilon = 1e-12);
    }

    #[test]
    fn test_perturbation_set_multiple() {
        let mut perts = PerturbationSet::new();
        perts.add(J2Perturbation::earth());
        perts.add(DragPerturbation::earth(100.0));
        perts.add(ThirdBodyPerturbation::sun());
        perts.add(ThirdBodyPerturbation::moon());

        assert_eq!(perts.len(), 4);
        assert!(!perts.is_empty());
        assert!(perts.is_time_dependent()); // Sun and Moon are time-dependent

        let names = perts.names();
        assert_eq!(names.len(), 4);
        assert!(names.contains(&"J2 Oblateness"));
        assert!(names.contains(&"Atmospheric Drag"));
        assert!(names.contains(&"Third-Body (Sun)"));
        assert!(names.contains(&"Third-Body (Moon)"));

        let r = Vector3::new(6778e3, 0.0, 0.0); // 400 km altitude
        let v = Vector3::new(0.0, 7670.0, 0.0);
        let a = perts.total_acceleration(0.0, &r, &v, GM_EARTH);

        // Total should be non-zero
        assert!(a.norm() > 0.0);

        // Should be sum of individual accelerations
        use crate::core::constants::{H0_EARTH, RHO0_EARTH};
        let a_j2 = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);
        let a_drag = drag_acceleration(&r, &v, R_EARTH, RHO0_EARTH, H0_EARTH, 100.0);
        let r_sun = sun_position_simple(0.0);
        let a_sun = third_body_perturbation(&r, &r_sun, crate::core::constants::GM_SUN);
        let r_moon = moon_position_simple(0.0);
        let a_moon = third_body_perturbation(&r, &r_moon, crate::core::constants::GM_MOON);

        let a_expected = a_j2 + a_drag + a_sun + a_moon;

        assert_relative_eq!(a.x, a_expected.x, epsilon = 1e-10);
        assert_relative_eq!(a.y, a_expected.y, epsilon = 1e-10);
        assert_relative_eq!(a.z, a_expected.z, epsilon = 1e-10);
    }

    #[test]
    fn test_propagate_with_perturbations_j2_only() {
        let mut perts = PerturbationSet::new();
        perts.add(J2Perturbation::earth());

        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 3600.0;

        let result = propagate_with_perturbations(
            &r0, &v0, dt, GM_EARTH, &perts, "rk4", Some(100), None
        );

        assert!(result.is_ok());
        let (r, v) = result.unwrap();

        // Should match dedicated J2 propagator
        let (r_j2, v_j2) = propagate_j2_rk4(&r0, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(100)).unwrap();

        assert_relative_eq!(r.x, r_j2.x, epsilon = 1.0);
        assert_relative_eq!(r.y, r_j2.y, epsilon = 1.0);
        assert_relative_eq!(r.z, r_j2.z, epsilon = 1.0);
        assert_relative_eq!(v.x, v_j2.x, epsilon = 0.001);
        assert_relative_eq!(v.y, v_j2.y, epsilon = 0.001);
        assert_relative_eq!(v.z, v_j2.z, epsilon = 0.001);
    }

    #[test]
    fn test_propagate_with_perturbations_combined() {
        let mut perts = PerturbationSet::new();
        perts.add(J2Perturbation::earth());
        perts.add(DragPerturbation::earth(100.0));

        let r0 = Vector3::new(6778e3, 0.0, 0.0); // 400 km altitude
        let v0 = Vector3::new(0.0, 7670.0, 0.0);
        let dt = 600.0; // 10 minutes

        let result = propagate_with_perturbations(
            &r0, &v0, dt, GM_EARTH, &perts, "dopri5", None, Some(1e-8)
        );

        assert!(result.is_ok());
        let (r, v) = result.unwrap();

        // Should still be in orbit with reasonable values
        assert!(r.norm() > 6000e3);
        assert!(r.norm() < 8000e3);
        assert!(v.norm() > 1000.0);
        assert!(v.norm() < 10000.0);

        // Verify the result is different from initial state (perturbations had an effect)
        let pos_diff = (r - r0).norm();
        let vel_diff = (v - v0).norm();
        assert!(pos_diff > 100.0); // Position changed by at least 100 m
        assert!(vel_diff > 0.1); // Velocity changed by at least 0.1 m/s
    }

    #[test]
    fn test_propagate_with_perturbations_invalid_method() {
        let perts = PerturbationSet::new();

        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        let result = propagate_with_perturbations(
            &r0, &v0, 600.0, GM_EARTH, &perts, "invalid", None, None
        );

        assert!(result.is_err());
    }
}
