//! Keplerian (two-body) orbit propagator
//!
//! This module provides propagators for unperturbed two-body motion using:
//! 1. Mean anomaly propagation method
//! 2. Lagrange coefficients (f and g functions)
//!
//! The Keplerian propagator assumes only gravitational attraction from
//! a central body with no perturbations (ideal two-body problem).

use crate::core::anomaly::{mean_to_true_anomaly, true_to_mean_anomaly};
use crate::core::elements::{coe_to_rv, rv_to_coe, OrbitalElements};
use crate::core::error::{PoliastroError, PoliastroResult};
use crate::core::linalg::Vector3;
use crate::core::time::Duration;
use rayon::prelude::*;
use std::f64::consts::PI;

/// Propagate orbital elements forward in time using mean anomaly propagation
///
/// This is the classical Keplerian propagation method:
/// 1. Calculate mean motion: n = √(μ/a³)
/// 2. Convert initial true anomaly to mean anomaly
/// 3. Propagate mean anomaly: M = M₀ + n·Δt
/// 4. Convert back to true anomaly
/// 5. Update orbital elements with new true anomaly
///
/// # Arguments
/// * `elements` - Initial orbital elements (a, e, i, Ω, ω, ν)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// New orbital elements at time t₀ + Δt
///
/// # Errors
/// Returns error if:
/// - Orbit is not elliptical (e >= 1) - only elliptical orbits are supported
/// - Semi-major axis is invalid
///
/// # Example
/// ```
/// use astrora_core::propagators::keplerian::propagate_keplerian;
/// use astrora_core::core::elements::OrbitalElements;
/// use astrora_core::core::constants::GM_EARTH;
///
/// let elements = OrbitalElements::new(7000e3, 0.01, 0.0, 0.0, 0.0, 0.0);
/// let dt = 3600.0; // 1 hour
/// let new_elements = propagate_keplerian(&elements, dt, GM_EARTH).unwrap();
/// ```
pub fn propagate_keplerian(
    elements: &OrbitalElements,
    dt: f64,
    mu: f64,
) -> PoliastroResult<OrbitalElements> {
    // Validate orbit type (only elliptical orbits have finite periods)
    if elements.e >= 1.0 {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "keplerian propagation".into(),
            orbit_type: if elements.e > 1.0 {
                "hyperbolic (not yet implemented)"
            } else {
                "parabolic (not yet implemented)"
            }
            .into(),
        });
    }

    if elements.a <= 0.0 {
        return Err(PoliastroError::InvalidParameter {
            parameter: "semi_major_axis".into(),
            value: elements.a,
            constraint: "must be positive for elliptical orbits".into(),
        });
    }

    // Step 1: Calculate mean motion (radians per second)
    // n = √(μ/a³)
    let n = (mu / elements.a.powi(3)).sqrt();

    // Step 2: Convert initial true anomaly to mean anomaly
    let M0 = true_to_mean_anomaly(elements.nu, elements.e)?;

    // Step 3: Propagate mean anomaly
    // M = M₀ + n·Δt
    let M = (M0 + n * dt).rem_euclid(2.0 * PI);

    // Step 4: Convert new mean anomaly to true anomaly
    let nu_new = mean_to_true_anomaly(M, elements.e, None, None)?;

    // Step 5: Create new orbital elements with updated true anomaly
    // All other elements (a, e, i, Ω, ω) remain constant for Keplerian motion
    Ok(OrbitalElements::new(
        elements.a,
        elements.e,
        elements.i,
        elements.raan,
        elements.argp,
        nu_new,
    ))
}

/// Propagate orbital elements using Duration
///
/// Convenience wrapper around `propagate_keplerian` that accepts a Duration instead of seconds
///
/// # Arguments
/// * `elements` - Initial orbital elements
/// * `duration` - Time duration to propagate
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// New orbital elements at time t₀ + duration
pub fn propagate_keplerian_duration(
    elements: &OrbitalElements,
    duration: &Duration,
    mu: f64,
) -> PoliastroResult<OrbitalElements> {
    let dt = duration.to_seconds();
    propagate_keplerian(elements, dt, mu)
}

/// Propagate Cartesian state vectors forward in time
///
/// Converts state vectors to orbital elements, propagates, and converts back.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// Tuple of (position, velocity) at time t₀ + Δt
///
/// # Example
/// ```
/// use astrora_core::propagators::keplerian::propagate_state_keplerian;
/// use astrora_core::core::linalg::Vector3;
/// use astrora_core::core::constants::GM_EARTH;
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
/// let dt = 3600.0;
/// let (r, v) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();
/// ```
pub fn propagate_state_keplerian(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
) -> PoliastroResult<(Vector3, Vector3)> {
    // Convert state to orbital elements
    let elements = rv_to_coe(r0, v0, mu, 1e-8)?;

    // Propagate elements
    let new_elements = propagate_keplerian(&elements, dt, mu)?;

    // Convert back to state vectors
    Ok(coe_to_rv(&new_elements, mu))
}

// ============================================================================
// Lagrange Coefficients (f and g functions)
// ============================================================================

/// Calculate change in true anomaly from time of flight
///
/// Internal helper function for Lagrange coefficient propagation
fn calculate_delta_nu(
    r0_mag: f64,
    v0_mag: f64,
    vr0: f64,
    dt: f64,
    mu: f64,
) -> PoliastroResult<f64> {
    // Calculate specific angular momentum magnitude
    // For perpendicular case: h = r × v = r·v·sin(90°) = r·v
    // For general case, we need the cross product magnitude
    // But we can also derive from h² = μ·p where p = a(1-e²)

    // Calculate semi-major axis from energy
    let energy = 0.5 * v0_mag * v0_mag - mu / r0_mag;

    if energy.abs() < 1e-10 {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "lagrange propagation".into(),
            orbit_type: "parabolic".into(),
        });
    }

    let a = -mu / (2.0 * energy);

    if a < 0.0 {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "lagrange propagation".into(),
            orbit_type: "hyperbolic".into(),
        });
    }

    // Calculate mean motion
    let n = (mu / a.powi(3)).sqrt();

    // For now, use simple mean anomaly propagation to get Δν
    // This is equivalent to the full method but more straightforward
    let delta_M = n * dt;

    // For small angles, Δν ≈ ΔM for circular orbits
    // For general case, we need to know eccentricity
    // Calculate eccentricity from state vectors
    let h_vec_sq = r0_mag * r0_mag * (v0_mag * v0_mag - vr0 * vr0);
    let h_mag = h_vec_sq.sqrt();

    if h_mag < 1e-10 {
        return Err(PoliastroError::InvalidStateVector {
            reason: "zero angular momentum (radial trajectory)".into(),
        });
    }

    let p = h_mag * h_mag / mu;
    let e = ((1.0 - p / a).abs()).sqrt();

    // Calculate initial true anomaly from orbit equation
    // r = p / (1 + e·cos(ν))
    let cos_nu0 = (p / r0_mag - 1.0) / e;
    let cos_nu0 = cos_nu0.clamp(-1.0, 1.0);

    // Determine quadrant from radial velocity
    let nu0 = if vr0 >= 0.0 {
        cos_nu0.acos()
    } else {
        2.0 * PI - cos_nu0.acos()
    };

    // Convert to mean anomaly, propagate, convert back
    let M0 = true_to_mean_anomaly(nu0, e)?;
    let M = M0 + delta_M;
    let nu = mean_to_true_anomaly(M, e, None, None)?;

    Ok(nu - nu0)
}

/// Calculate Lagrange coefficients for orbit propagation
///
/// Computes the f, g, fdot, gdot coefficients that relate position and velocity
/// at time t to initial conditions at time t₀:
///
/// r(t) = f·r₀ + g·v₀
/// v(t) = fdot·r₀ + gdot·v₀
///
/// # Arguments
/// * `r0_mag` - Initial position magnitude (m)
/// * `r_mag` - Current position magnitude (m)
/// * `h_mag` - Specific angular momentum magnitude (m²/s)
/// * `delta_nu` - Change in true anomaly (radians)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// Tuple of (f, g, fdot, gdot)
///
/// # References
/// - Vallado, "Fundamentals of Astrodynamics and Applications", 4th ed., Algorithm 11
/// - Curtis, "Orbital Mechanics for Engineering Students", Chapter 3
fn lagrange_coefficients(
    r0_mag: f64,
    r_mag: f64,
    h_mag: f64,
    delta_nu: f64,
    mu: f64,
) -> (f64, f64, f64, f64) {
    let cos_dnu = delta_nu.cos();
    let sin_dnu = delta_nu.sin();

    // Lagrange coefficients
    let f = 1.0 - (mu * r_mag / (h_mag * h_mag)) * (1.0 - cos_dnu);
    let g = (r_mag * r0_mag / h_mag) * sin_dnu;

    // Time derivatives
    let fdot = (mu / h_mag) * ((1.0 - cos_dnu) / sin_dnu)
        * ((mu / (h_mag * h_mag)) * (1.0 - cos_dnu) - 1.0 / r0_mag - 1.0 / r_mag);
    let gdot = 1.0 - (mu * r0_mag / (h_mag * h_mag)) * (1.0 - cos_dnu);

    (f, g, fdot, gdot)
}

/// Propagate orbit using Lagrange coefficients (f and g functions)
///
/// This method propagates the orbit without converting to orbital elements,
/// using the Lagrange coefficients that relate final and initial state vectors.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// Tuple of (position, velocity) at time t₀ + Δt
///
/// # Algorithm
/// 1. Calculate specific angular momentum h = r₀ × v₀
/// 2. Determine change in true anomaly Δν from time of flight
/// 3. Calculate new orbital radius r from orbit equation
/// 4. Compute Lagrange coefficients f, g, ḟ, ġ
/// 5. Apply: r = f·r₀ + g·v₀, v = ḟ·r₀ + ġ·v₀
///
/// # Note
/// This method is more direct than mean anomaly propagation as it doesn't
/// require full conversion to orbital elements, but both methods give identical results.
///
/// # Example
/// ```
/// use astrora_core::propagators::keplerian::propagate_lagrange;
/// use astrora_core::core::linalg::Vector3;
/// use astrora_core::core::constants::GM_EARTH;
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
/// let dt = 3600.0;
/// let (r, v) = propagate_lagrange(&r0, &v0, dt, GM_EARTH).unwrap();
/// ```
pub fn propagate_lagrange(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
) -> PoliastroResult<(Vector3, Vector3)> {
    // Calculate magnitudes
    let r0_mag = r0.norm();
    let v0_mag = v0.norm();

    // Calculate radial velocity component
    let vr0 = r0.dot(v0) / r0_mag;

    // Calculate specific angular momentum
    let h_vec = r0.cross(v0);
    let h_mag = h_vec.norm();

    if h_mag < 1e-10 {
        return Err(PoliastroError::InvalidStateVector {
            reason: "zero angular momentum (radial trajectory)".into(),
        });
    }

    // Calculate change in true anomaly
    let delta_nu = calculate_delta_nu(r0_mag, v0_mag, vr0, dt, mu)?;

    // Calculate new orbital radius using orbit equation
    // r = (h²/μ) / [1 + ((h²/μr₀) - 1)cos(Δν) - (h·vᵣ₀/μ)sin(Δν)]
    let cos_dnu = delta_nu.cos();
    let sin_dnu = delta_nu.sin();

    let h_sq_over_mu = h_mag * h_mag / mu;
    let term1 = h_sq_over_mu / r0_mag - 1.0;
    let term2 = h_mag * vr0 / mu;

    let r_mag = h_sq_over_mu / (1.0 + term1 * cos_dnu - term2 * sin_dnu);

    // Calculate Lagrange coefficients
    let (f, g, fdot, gdot) = lagrange_coefficients(r0_mag, r_mag, h_mag, delta_nu, mu);

    // Propagate position and velocity
    let r = f * r0 + g * v0;
    let v = fdot * r0 + gdot * v0;

    Ok((r, v))
}

// ============================================================================
// Batch Operations
// ============================================================================

/// Batch propagation of multiple state vectors
///
/// Efficiently propagates multiple orbits in a single call to minimize Python-Rust
/// boundary crossing overhead. This function demonstrates the critical performance
/// pattern: batch operations are 10-20x faster than sequential calls.
///
/// # Arguments
/// * `states` - 2D array where each row is [x, y, z, vx, vy, vz] in meters and m/s
/// * `time_steps` - Either a single time step for all states, or one per state (in seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// 2D array with same shape containing propagated states
///
/// # Errors
/// Returns error if:
/// - State array doesn't have exactly 6 columns
/// - Number of time steps doesn't match number of states (when array is provided)
/// - Any individual propagation fails (invalid orbit type, etc.)
///
/// # Example
/// ```
/// use astrora_core::propagators::keplerian::batch_propagate_states;
/// use ndarray::array;
/// use astrora_core::core::constants::GM_EARTH;
///
/// // Two orbits to propagate
/// let states = array![
///     [7000e3, 0.0, 0.0, 0.0, 7546.0, 0.0],
///     [8000e3, 0.0, 0.0, 0.0, 7000.0, 0.0]
/// ];
/// let dt = 3600.0; // 1 hour for both
/// let result = batch_propagate_states(states.view(), &[dt], GM_EARTH).unwrap();
/// ```
pub fn batch_propagate_states(
    states: ndarray::ArrayView2<f64>,
    time_steps: &[f64],
    mu: f64,
) -> PoliastroResult<ndarray::Array2<f64>> {
    use ndarray::Array2;

    if states.ncols() != 6 {
        return Err(PoliastroError::InvalidParameter {
            parameter: "state_dimensions".into(),
            value: states.ncols() as f64,
            constraint: "must be 6 (x, y, z, vx, vy, vz)".into(),
        });
    }

    let n_states = states.nrows();

    // Handle time steps - either single value or one per state
    let dt_vec: Vec<f64> = if time_steps.len() == 1 {
        // Single time step for all states
        vec![time_steps[0]; n_states]
    } else if time_steps.len() == n_states {
        // One time step per state
        time_steps.to_vec()
    } else {
        return Err(PoliastroError::InvalidParameter {
            parameter: "time_steps".into(),
            value: time_steps.len() as f64,
            constraint: format!(
                "must be 1 or match number of states ({n_states})"
            ),
        });
    };

    // Parallel propagation using rayon
    // Process all states in parallel, collecting results as rows
    let result_rows: Vec<[f64; 6]> = (0..n_states)
        .into_par_iter()
        .map(|i| {
            let state_row = states.row(i);
            let r0 = Vector3::new(state_row[0], state_row[1], state_row[2]);
            let v0 = Vector3::new(state_row[3], state_row[4], state_row[5]);
            let dt = dt_vec[i];

            // Propagate this state
            let (r, v) = propagate_state_keplerian(&r0, &v0, dt, mu).map_err(|e| {
                PoliastroError::PropagationFailed {
                    context: format!("batch propagation at index {i}"),
                    source: Box::new(e),
                }
            })?;

            // Return as row array
            Ok([r.x, r.y, r.z, v.x, v.y, v.z])
        })
        .collect::<PoliastroResult<Vec<[f64; 6]>>>()?;

    // Convert Vec of rows to Array2
    let result = Array2::from_shape_fn((n_states, 6), |(i, j)| result_rows[i][j]);

    Ok(result)
}

/// Batch propagation using Lagrange coefficients
///
/// Alternative batch propagation method using f and g functions.
/// Useful for cross-validation and performance comparison.
///
/// # Arguments
/// * `states` - 2D array where each row is [x, y, z, vx, vy, vz]
/// * `time_steps` - Either a single time step or one per state (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// 2D array with same shape containing propagated states
pub fn batch_propagate_lagrange(
    states: ndarray::ArrayView2<f64>,
    time_steps: &[f64],
    mu: f64,
) -> PoliastroResult<ndarray::Array2<f64>> {
    use ndarray::Array2;

    if states.ncols() != 6 {
        return Err(PoliastroError::InvalidParameter {
            parameter: "state_dimensions".into(),
            value: states.ncols() as f64,
            constraint: "must be 6 (x, y, z, vx, vy, vz)".into(),
        });
    }

    let n_states = states.nrows();

    let dt_vec: Vec<f64> = if time_steps.len() == 1 {
        vec![time_steps[0]; n_states]
    } else if time_steps.len() == n_states {
        time_steps.to_vec()
    } else {
        return Err(PoliastroError::InvalidParameter {
            parameter: "time_steps".into(),
            value: time_steps.len() as f64,
            constraint: format!(
                "must be 1 or match number of states ({n_states})"
            ),
        });
    };

    // Parallel propagation using rayon (Lagrange method)
    let result_rows: Vec<[f64; 6]> = (0..n_states)
        .into_par_iter()
        .map(|i| {
            let state_row = states.row(i);
            let r0 = Vector3::new(state_row[0], state_row[1], state_row[2]);
            let v0 = Vector3::new(state_row[3], state_row[4], state_row[5]);
            let dt = dt_vec[i];

            let (r, v) = propagate_lagrange(&r0, &v0, dt, mu).map_err(|e| {
                PoliastroError::PropagationFailed {
                    context: format!("batch lagrange propagation at index {i}"),
                    source: Box::new(e),
                }
            })?;

            Ok([r.x, r.y, r.z, v.x, v.y, v.z])
        })
        .collect::<PoliastroResult<Vec<[f64; 6]>>>()?;

    // Convert Vec of rows to Array2
    let result = Array2::from_shape_fn((n_states, 6), |(i, j)| result_rows[i][j]);

    Ok(result)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::GM_EARTH;
    use approx::assert_relative_eq;

    #[test]
    fn test_propagate_circular_orbit() {
        // Circular orbit at 7000 km radius
        let r = 7000e3;
        let v_circ = (GM_EARTH / r).sqrt();

        let elements = OrbitalElements::new(r, 0.0, 0.0, 0.0, 0.0, 0.0);

        // Propagate for 1/4 of orbital period (should be 90° around)
        let period = elements.period(GM_EARTH).unwrap();
        let dt = period / 4.0;

        let new_elements = propagate_keplerian(&elements, dt, GM_EARTH).unwrap();

        // After 1/4 period, true anomaly should be π/2
        assert_relative_eq!(new_elements.nu, PI / 2.0, epsilon = 1e-6);

        // All other elements should remain constant
        assert_relative_eq!(new_elements.a, r, epsilon = 1.0);
        assert_relative_eq!(new_elements.e, 0.0, epsilon = 1e-8);
    }

    #[test]
    fn test_propagate_full_orbit() {
        // Elliptical orbit
        let a = 8000e3;
        let e = 0.1;
        let elements = OrbitalElements::new(a, e, 0.0, 0.0, 0.0, 0.0);

        // Propagate for one full period
        let period = elements.period(GM_EARTH).unwrap();
        let new_elements = propagate_keplerian(&elements, period, GM_EARTH).unwrap();

        // Should return to original position (true anomaly ≈ 0)
        // Account for 2π wrap-around
        let nu_diff = (new_elements.nu - elements.nu).abs();
        assert!(nu_diff < 1e-6 || (2.0 * PI - nu_diff).abs() < 1e-6);

        // All orbital elements should remain the same
        assert_relative_eq!(new_elements.a, a, epsilon = 1.0);
        assert_relative_eq!(new_elements.e, e, epsilon = 1e-8);
    }

    #[test]
    fn test_propagate_state_circular() {
        // Circular orbit
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v_circ = (GM_EARTH / 7000e3).sqrt();
        let v0 = Vector3::new(0.0, v_circ, 0.0);

        // Propagate for 1 hour
        let dt = 3600.0;
        let (r, v) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Orbit radius should remain constant
        assert_relative_eq!(r.norm(), r0.norm(), epsilon = 1.0);
        assert_relative_eq!(v.norm(), v0.norm(), epsilon = 0.1);

        // Total energy should be conserved
        let energy0 = 0.5 * v0.norm_squared() - GM_EARTH / r0.norm();
        let energy = 0.5 * v.norm_squared() - GM_EARTH / r.norm();
        assert_relative_eq!(energy, energy0, epsilon = 1.0);
    }

    #[test]
    fn test_propagate_state_elliptical() {
        // Elliptical orbit
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 8000.0, 0.0);

        // Propagate for 2 hours
        let dt = 2.0 * 3600.0;
        let (r, v) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Energy conservation
        let energy0 = 0.5 * v0.norm_squared() - GM_EARTH / r0.norm();
        let energy = 0.5 * v.norm_squared() - GM_EARTH / r.norm();
        assert_relative_eq!(energy, energy0, epsilon = 1.0);

        // Angular momentum conservation
        let h0 = r0.cross(&v0);
        let h = r.cross(&v);
        assert_relative_eq!(h.x, h0.x, epsilon = 1e-3);
        assert_relative_eq!(h.y, h0.y, epsilon = 1e-3);
        assert_relative_eq!(h.z, h0.z, epsilon = 1e-3);
    }

    #[test]
    fn test_propagate_lagrange_vs_mean_anomaly() {
        // Test that Lagrange method gives same result as mean anomaly method
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v_circ = (GM_EARTH / 7000e3).sqrt();
        let v0 = Vector3::new(0.0, v_circ, 0.0);
        let dt = 3600.0;

        // Method 1: Mean anomaly propagation
        let (r1, v1) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        // Method 2: Lagrange coefficients
        let (r2, v2) = propagate_lagrange(&r0, &v0, dt, GM_EARTH).unwrap();

        // Results should be very close
        assert_relative_eq!(r1.x, r2.x, epsilon = 1.0);
        assert_relative_eq!(r1.y, r2.y, epsilon = 1.0);
        assert_relative_eq!(r1.z, r2.z, epsilon = 1.0);

        assert_relative_eq!(v1.x, v2.x, epsilon = 1e-3);
        assert_relative_eq!(v1.y, v2.y, epsilon = 1e-3);
        assert_relative_eq!(v1.z, v2.z, epsilon = 1e-3);
    }

    #[test]
    fn test_propagate_backward_in_time() {
        // Test propagating backward (negative dt)
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v_circ = (GM_EARTH / 7000e3).sqrt();
        let v0 = Vector3::new(0.0, v_circ, 0.0);

        let dt = 3600.0;

        // Propagate forward then backward
        let (r_fwd, v_fwd) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();
        let (r_back, v_back) = propagate_state_keplerian(&r_fwd, &v_fwd, -dt, GM_EARTH).unwrap();

        // Should return to original state
        assert_relative_eq!(r_back.x, r0.x, epsilon = 10.0);
        assert_relative_eq!(r_back.y, r0.y, epsilon = 10.0);
        assert_relative_eq!(r_back.z, r0.z, epsilon = 10.0);

        assert_relative_eq!(v_back.x, v0.x, epsilon = 0.1);
        assert_relative_eq!(v_back.y, v0.y, epsilon = 0.1);
        assert_relative_eq!(v_back.z, v0.z, epsilon = 0.1);
    }

    #[test]
    fn test_propagate_rejects_hyperbolic() {
        // Hyperbolic orbit (e > 1)
        let elements = OrbitalElements::new(8000e3, 1.5, 0.0, 0.0, 0.0, 0.0);
        let dt = 3600.0;

        let result = propagate_keplerian(&elements, dt, GM_EARTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_propagate_rejects_parabolic() {
        // Parabolic orbit (e = 1)
        let elements = OrbitalElements::new(8000e3, 1.0, 0.0, 0.0, 0.0, 0.0);
        let dt = 3600.0;

        let result = propagate_keplerian(&elements, dt, GM_EARTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_energy_conservation() {
        // Test that energy is conserved during propagation
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 8000.0, 0.0);

        let energy0 = 0.5 * v0.norm_squared() - GM_EARTH / r0.norm();

        // Propagate multiple times
        for _ in 0..10 {
            let dt = 1000.0; // Multiple 1000-second steps
            let (r, v) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

            let energy = 0.5 * v.norm_squared() - GM_EARTH / r.norm();
            assert_relative_eq!(energy, energy0, epsilon = 10.0);
        }
    }

    #[test]
    fn test_angular_momentum_conservation() {
        // Test that angular momentum is conserved
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 8000.0, 0.0);

        let h0 = r0.cross(&v0);

        // Propagate and check h conservation
        let dt = 5000.0;
        let (r, v) = propagate_state_keplerian(&r0, &v0, dt, GM_EARTH).unwrap();

        let h = r.cross(&v);

        assert_relative_eq!(h.norm(), h0.norm(), epsilon = 1e-3);
        assert_relative_eq!(h.x, h0.x, epsilon = 1e-3);
        assert_relative_eq!(h.y, h0.y, epsilon = 1e-3);
        assert_relative_eq!(h.z, h0.z, epsilon = 1e-3);
    }

    #[test]
    fn test_batch_propagate_single_time_step() {
        use ndarray::array;

        // Create two circular orbits at different altitudes
        let r1 = 7000e3;
        let r2 = 8000e3;
        let v1 = (GM_EARTH / r1).sqrt();
        let v2 = (GM_EARTH / r2).sqrt();

        let states = array![
            [r1, 0.0, 0.0, 0.0, v1, 0.0],
            [r2, 0.0, 0.0, 0.0, v2, 0.0]
        ];

        let dt = 3600.0; // 1 hour
        let result = batch_propagate_states(states.view(), &[dt], GM_EARTH).unwrap();

        // Check shape
        assert_eq!(result.shape(), &[2, 6]);

        // Both orbits should maintain their radius (circular orbits)
        let r1_new = (result[[0, 0]].powi(2) + result[[0, 1]].powi(2) + result[[0, 2]].powi(2)).sqrt();
        let r2_new = (result[[1, 0]].powi(2) + result[[1, 1]].powi(2) + result[[1, 2]].powi(2)).sqrt();

        assert_relative_eq!(r1_new, r1, epsilon = 1.0);
        assert_relative_eq!(r2_new, r2, epsilon = 1.0);
    }

    #[test]
    fn test_batch_propagate_multiple_time_steps() {
        use ndarray::array;

        // Two orbits with different time steps
        let r = 7000e3;
        let v = (GM_EARTH / r).sqrt();

        let states = array![
            [r, 0.0, 0.0, 0.0, v, 0.0],
            [r, 0.0, 0.0, 0.0, v, 0.0]
        ];

        // Different time steps
        let dt = vec![1800.0, 3600.0]; // 30 min and 1 hour
        let result = batch_propagate_states(states.view(), &dt, GM_EARTH).unwrap();

        // Second orbit should have traveled farther
        // Both should still be circular
        let r1_new = (result[[0, 0]].powi(2) + result[[0, 1]].powi(2) + result[[0, 2]].powi(2)).sqrt();
        let r2_new = (result[[1, 0]].powi(2) + result[[1, 1]].powi(2) + result[[1, 2]].powi(2)).sqrt();

        assert_relative_eq!(r1_new, r, epsilon = 1.0);
        assert_relative_eq!(r2_new, r, epsilon = 1.0);
    }

    #[test]
    fn test_batch_propagate_invalid_dimensions() {
        use ndarray::array;

        // Wrong number of columns
        let states = array![
            [7000e3, 0.0, 0.0, 0.0, 7546.0], // Only 5 columns
        ];

        let dt = 3600.0;
        let result = batch_propagate_states(states.view(), &[dt], GM_EARTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_propagate_wrong_time_steps() {
        use ndarray::array;

        let states = array![
            [7000e3, 0.0, 0.0, 0.0, 7546.0, 0.0],
            [8000e3, 0.0, 0.0, 0.0, 7000.0, 0.0]
        ];

        // Wrong number of time steps (2 states but 3 time steps)
        let dt = vec![1000.0, 2000.0, 3000.0];
        let result = batch_propagate_states(states.view(), &dt, GM_EARTH);
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_propagate_energy_conservation() {
        use ndarray::array;

        // Elliptical orbit
        let r0 = 7000e3;
        let v0 = 8000.0;

        let states = array![
            [r0, 0.0, 0.0, 0.0, v0, 0.0],
            [r0, 0.0, 0.0, 0.0, v0, 0.0]
        ];

        let dt = 3600.0;
        let result = batch_propagate_states(states.view(), &[dt], GM_EARTH).unwrap();

        // Check energy conservation for both
        for i in 0..2 {
            let r_mag = (result[[i, 0]].powi(2) + result[[i, 1]].powi(2) + result[[i, 2]].powi(2)).sqrt();
            let v_mag_sq = result[[i, 3]].powi(2) + result[[i, 4]].powi(2) + result[[i, 5]].powi(2);

            let energy_initial = 0.5 * v0 * v0 - GM_EARTH / r0;
            let energy_final = 0.5 * v_mag_sq - GM_EARTH / r_mag;

            assert_relative_eq!(energy_final, energy_initial, epsilon = 10.0);
        }
    }

    #[test]
    fn test_batch_propagate_lagrange() {
        use ndarray::array;

        let r = 7000e3;
        let v = (GM_EARTH / r).sqrt();

        let states = array![
            [r, 0.0, 0.0, 0.0, v, 0.0],
        ];

        let dt = 3600.0;

        // Both methods should give same result
        let result1 = batch_propagate_states(states.view(), &[dt], GM_EARTH).unwrap();
        let result2 = batch_propagate_lagrange(states.view(), &[dt], GM_EARTH).unwrap();

        for i in 0..6 {
            assert_relative_eq!(result1[[0, i]], result2[[0, i]], epsilon = 1.0);
        }
    }
}
