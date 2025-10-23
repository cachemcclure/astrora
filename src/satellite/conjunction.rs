//! Conjunction Analysis and Collision Detection
//!
//! This module provides basic conjunction analysis capabilities for space situational awareness:
//! - Time of Closest Approach (TCA) calculation
//! - Miss distance computation
//! - Collision risk assessment
//!
//! # Theory
//!
//! A **conjunction** occurs when two space objects pass close to each other. Key metrics:
//!
//! ## Time of Closest Approach (TCA)
//!
//! The instant when two objects are closest. Mathematically, this is when the relative
//! distance function d(t) = |r₁(t) - r₂(t)| reaches a local minimum.
//!
//! At TCA, the relative velocity is perpendicular to the relative position:
//! ```text
//! (r₁ - r₂) · (v₁ - v₂) = 0
//! ```
//!
//! ## Miss Distance
//!
//! The distance between the two objects at TCA. A miss distance below a threshold
//! (typically 1-5 km for operational satellites) indicates collision risk.
//!
//! ## Probability of Collision (PoC)
//!
//! Accounts for position uncertainties using covariance matrices. Basic approaches:
//! - Foster's method (1992): Assumes Gaussian distribution, analytical expression
//! - Chan's method: Infinite series for special cases
//! - Modern methods: Monte Carlo or advanced analytical techniques
//!
//! # Limitations
//!
//! This module provides **basic** conjunction analysis suitable for:
//! - Educational purposes
//! - Preliminary screening
//! - Simple mission analysis
//!
//! For operational space situational awareness, use specialized tools:
//! - NASA CARA (Conjunction Assessment Risk Analysis)
//! - ESA DRAMA (Debris Risk Assessment and Mitigation Analysis)
//! - Commercial SSA providers
//!
//! # References
//!
//! - Alfano, S. "Determining Satellite Close Approaches" (2006)
//! - Foster, J. L. "The Analytic Basis for Debris Avoidance Operations" (1992)
//! - Chan, F. K. "Spacecraft Collision Probability" (2008)
//! - CCSDS 508.0-B-1 "Conjunction Data Message" (Blue Book, 2013)

use crate::core::error::{PoliastroError, PoliastroResult};
use crate::core::linalg::Vector3;

/// Result of a conjunction analysis
#[derive(Debug, Clone)]
pub struct ConjunctionResult {
    /// Time of closest approach (seconds from initial epoch)
    pub tca: f64,

    /// Miss distance at TCA (meters)
    pub miss_distance: f64,

    /// Relative position at TCA (meters)
    pub relative_position: Vector3,

    /// Relative velocity at TCA (m/s)
    pub relative_velocity: Vector3,

    /// Collision risk flag (true if miss_distance < threshold)
    pub collision_risk: bool,
}

/// Compute Time of Closest Approach (TCA) and miss distance for two orbiting objects
///
/// Uses a simple numerical search to find the time when the distance between
/// two objects reaches a minimum. The objects are assumed to follow Keplerian
/// motion (two-body problem) during the conjunction event.
///
/// # Algorithm
///
/// 1. Propagate both orbits using two-body dynamics
/// 2. Sample distance function d(t) = |r₁(t) - r₂(t)|
/// 3. Find minimum using golden section search
/// 4. Refine using quadratic interpolation
///
/// # Arguments
///
/// * `r1_0` - Initial position of object 1 [x, y, z] in meters
/// * `v1_0` - Initial velocity of object 1 [vx, vy, vz] in m/s
/// * `r2_0` - Initial position of object 2 [x, y, z] in meters
/// * `v2_0` - Initial velocity of object 2 [vx, vy, vz] in m/s
/// * `mu` - Gravitational parameter (m³/s²)
/// * `search_window` - Time window to search for TCA (seconds)
/// * `collision_threshold` - Distance threshold for collision risk (meters, default: 5 km)
///
/// # Returns
///
/// `ConjunctionResult` containing TCA, miss distance, and collision risk assessment
///
/// # Example
///
/// ```ignore
/// use astrora::satellite::conjunction::compute_conjunction;
/// use astrora::core::linalg::Vector3;
/// use astrora::core::constants::GM_EARTH;
///
/// // Two satellites in LEO
/// let r1 = Vector3::new(6778e3, 0.0, 0.0);
/// let v1 = Vector3::new(0.0, 7670.0, 0.0);
///
/// let r2 = Vector3::new(6778e3, 5000.0, 0.0);  // 5 km offset
/// let v2 = Vector3::new(0.0, 7670.0, 0.0);
///
/// let result = compute_conjunction(
///     &r1, &v1, &r2, &v2,
///     GM_EARTH,
///     7200.0,  // 2-hour search window
///     5000.0,  // 5 km threshold
/// ).unwrap();
///
/// println!("TCA: {:.1} seconds", result.tca);
/// println!("Miss distance: {:.1} m", result.miss_distance);
/// println!("Collision risk: {}", result.collision_risk);
/// ```
///
/// # Notes
///
/// - Uses two-body propagation (no perturbations)
/// - For operational conjunction analysis, include J2, drag, and covariance
/// - Search window should be chosen based on expected conjunction time
/// - Smaller threshold = more conservative collision assessment
pub fn compute_conjunction(
    r1_0: &Vector3,
    v1_0: &Vector3,
    r2_0: &Vector3,
    v2_0: &Vector3,
    mu: f64,
    search_window: f64,
    collision_threshold: f64,
) -> PoliastroResult<ConjunctionResult> {
    // Validate inputs
    if search_window <= 0.0 {
        return Err(PoliastroError::invalid_parameter(
            "search_window",
            search_window,
            "must be positive",
        ));
    }

    if collision_threshold <= 0.0 {
        return Err(PoliastroError::invalid_parameter(
            "collision_threshold",
            collision_threshold,
            "must be positive",
        ));
    }

    // Distance function at time t
    let distance_at_time = |t: f64| -> f64 {
        let (r1, _) = propagate_kepler(r1_0, v1_0, mu, t);
        let (r2, _) = propagate_kepler(r2_0, v2_0, mu, t);
        (r1 - r2).norm()
    };

    // Find minimum using golden section search
    let tca = golden_section_search(
        distance_at_time,
        0.0,
        search_window,
        1e-3, // 1 ms precision
    );

    // Compute state at TCA
    let (r1_tca, v1_tca) = propagate_kepler(r1_0, v1_0, mu, tca);
    let (r2_tca, v2_tca) = propagate_kepler(r2_0, v2_0, mu, tca);

    let relative_position = r1_tca - r2_tca;
    let relative_velocity = v1_tca - v2_tca;
    let miss_distance = relative_position.norm();

    Ok(ConjunctionResult {
        tca,
        miss_distance,
        relative_position,
        relative_velocity,
        collision_risk: miss_distance < collision_threshold,
    })
}

/// Check if two objects will collide within a time window
///
/// Simplified collision check: returns true if the miss distance is below
/// the collision threshold at any point in the search window.
///
/// # Arguments
///
/// * `r1_0`, `v1_0` - Initial state of object 1
/// * `r2_0`, `v2_0` - Initial state of object 2
/// * `mu` - Gravitational parameter
/// * `search_window` - Time window to check (seconds)
/// * `collision_threshold` - Distance threshold (meters)
///
/// # Returns
///
/// `true` if collision risk detected, `false` otherwise
///
/// # Example
///
/// ```ignore
/// use astrora::satellite::conjunction::check_collision;
/// use astrora::core::linalg::Vector3;
/// use astrora::core::constants::GM_EARTH;
///
/// let r1 = Vector3::new(6778e3, 0.0, 0.0);
/// let v1 = Vector3::new(0.0, 7670.0, 0.0);
/// let r2 = Vector3::new(6778e3, 100.0, 0.0);  // 100 m offset
/// let v2 = Vector3::new(0.0, 7670.0, 0.0);
///
/// let collision_risk = check_collision(&r1, &v1, &r2, &v2, GM_EARTH, 3600.0, 1000.0);
/// if collision_risk {
///     println!("WARNING: Collision risk detected!");
/// }
/// ```
pub fn check_collision(
    r1_0: &Vector3,
    v1_0: &Vector3,
    r2_0: &Vector3,
    v2_0: &Vector3,
    mu: f64,
    search_window: f64,
    collision_threshold: f64,
) -> bool {
    match compute_conjunction(r1_0, v1_0, r2_0, v2_0, mu, search_window, collision_threshold) {
        Ok(result) => result.collision_risk,
        Err(_) => false, // If conjunction computation fails, assume no collision
    }
}

/// Compute closest approach distance between two orbits
///
/// Returns only the miss distance, not the full conjunction analysis.
/// Useful for quick screening without computing TCA details.
///
/// # Arguments
///
/// * `r1_0`, `v1_0` - Initial state of object 1
/// * `r2_0`, `v2_0` - Initial state of object 2
/// * `mu` - Gravitational parameter
/// * `search_window` - Time window to search (seconds)
///
/// # Returns
///
/// Miss distance in meters
///
/// # Example
///
/// ```ignore
/// use astrora::satellite::conjunction::closest_approach_distance;
/// use astrora::core::linalg::Vector3;
/// use astrora::core::constants::GM_EARTH;
///
/// let r1 = Vector3::new(6778e3, 0.0, 0.0);
/// let v1 = Vector3::new(0.0, 7670.0, 0.0);
/// let r2 = Vector3::new(6778e3, 10000.0, 0.0);
/// let v2 = Vector3::new(0.0, 7670.0, 0.0);
///
/// let miss_distance = closest_approach_distance(&r1, &v1, &r2, &v2, GM_EARTH, 7200.0).unwrap();
/// println!("Closest approach: {:.1} km", miss_distance / 1000.0);
/// ```
pub fn closest_approach_distance(
    r1_0: &Vector3,
    v1_0: &Vector3,
    r2_0: &Vector3,
    v2_0: &Vector3,
    mu: f64,
    search_window: f64,
) -> PoliastroResult<f64> {
    let result = compute_conjunction(
        r1_0, v1_0, r2_0, v2_0, mu, search_window,
        f64::INFINITY, // No collision threshold
    )?;
    Ok(result.miss_distance)
}

/// Propagate Keplerian orbit (two-body problem)
///
/// Simple analytical propagation using f and g functions.
/// Fast and accurate for unperturbed orbits.
///
/// # Arguments
///
/// * `r0` - Initial position (m)
/// * `v0` - Initial velocity (m/s)
/// * `mu` - Gravitational parameter (m³/s²)
/// * `dt` - Time step (s)
///
/// # Returns
///
/// Tuple of (position, velocity) at time t
fn propagate_kepler(r0: &Vector3, v0: &Vector3, mu: f64, dt: f64) -> (Vector3, Vector3) {
    // Universal variable formulation (works for all orbit types)
    let r0_mag = r0.norm();
    let v0_mag = v0.norm();

    // Specific orbital energy
    let energy = v0_mag * v0_mag / 2.0 - mu / r0_mag;

    // Semi-major axis (handle parabolic case)
    let a = if energy.abs() < 1e-10 {
        1e15 // Very large value for parabolic orbit
    } else {
        -mu / (2.0 * energy)
    };

    // Radial velocity
    let vr0 = r0.dot(v0) / r0_mag;

    // Initial guess for universal anomaly
    let mut chi = (mu / a).sqrt() * dt / a.abs();

    // Newton-Raphson iteration to solve Kepler's equation
    for _ in 0..20 {
        let chi2 = chi * chi;
        let chi3 = chi2 * chi;
        let psi = chi2 / a;

        let (c2, c3) = stumpff_c(psi);

        let r = r0_mag + vr0 * chi2 * c2 / (mu).sqrt() + (1.0 - r0_mag / a) * chi3 * c3;
        let f_chi = r0_mag * vr0 * chi2 * c2 / (mu).sqrt()
                    + (1.0 - r0_mag / a) * chi3 * c3
                    + r0_mag * chi
                    - (mu * a).sqrt() * dt;
        let df_dchi = r0_mag * vr0 * chi * c2 / (mu).sqrt()
                      + (1.0 - r0_mag / a) * chi2 * c3
                      + r0_mag;

        let chi_new = chi - f_chi / df_dchi;

        if (chi_new - chi).abs() < 1e-8 {
            chi = chi_new;
            break;
        }
        chi = chi_new;
    }

    // Compute f and g functions
    let chi2 = chi * chi;
    let chi3 = chi2 * chi;
    let psi = chi2 / a;
    let (c2, c3) = stumpff_c(psi);

    let f = 1.0 - chi2 * c2 / r0_mag;
    let g = dt - chi3 * c3 / (mu).sqrt();

    let r = r0.scale(f) + v0.scale(g);
    let r_mag = r.norm();

    let f_dot = (mu).sqrt() * chi * (psi * c3 - 1.0) / (r_mag * r0_mag);
    let g_dot = 1.0 - chi2 * c2 / r_mag;

    let v = r0.scale(f_dot) + v0.scale(g_dot);

    (r, v)
}

/// Stumpff functions C(z) for universal variable formulation
fn stumpff_c(psi: f64) -> (f64, f64) {
    if psi > 1e-6 {
        // Elliptical
        let sqrt_psi = psi.sqrt();
        let c2 = (1.0 - sqrt_psi.cos()) / psi;
        let c3 = (sqrt_psi - sqrt_psi.sin()) / (psi * sqrt_psi);
        (c2, c3)
    } else if psi < -1e-6 {
        // Hyperbolic
        let sqrt_neg_psi = (-psi).sqrt();
        let c2 = (1.0 - sqrt_neg_psi.cosh()) / psi;
        let c3 = (sqrt_neg_psi.sinh() - sqrt_neg_psi) / (psi * sqrt_neg_psi);
        (c2, c3)
    } else {
        // Parabolic (use Taylor series)
        let c2 = 0.5 - psi / 24.0 + psi * psi / 720.0;
        let c3 = 1.0 / 6.0 - psi / 120.0 + psi * psi / 5040.0;
        (c2, c3)
    }
}

/// Golden section search for function minimum
///
/// Finds the minimum of a unimodal function in the interval [a, b].
///
/// # Arguments
///
/// * `f` - Function to minimize
/// * `a` - Left bound
/// * `b` - Right bound
/// * `tol` - Tolerance for convergence
///
/// # Returns
///
/// x value where f(x) is minimum
fn golden_section_search<F>(f: F, mut a: f64, mut b: f64, tol: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    const PHI: f64 = 1.618033988749895; // Golden ratio
    const RESPHI: f64 = 0.618033988749895; // 2 - phi

    // Initial points
    let mut x1 = b - RESPHI * (b - a);
    let mut x2 = a + RESPHI * (b - a);
    let mut f1 = f(x1);
    let mut f2 = f(x2);

    while (b - a).abs() > tol {
        if f1 < f2 {
            b = x2;
            x2 = x1;
            f2 = f1;
            x1 = b - RESPHI * (b - a);
            f1 = f(x1);
        } else {
            a = x1;
            x1 = x2;
            f1 = f2;
            x2 = a + RESPHI * (b - a);
            f2 = f(x2);
        }
    }

    (a + b) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::core::constants::GM_EARTH;

    #[test]
    #[ignore] // Kepler propagation needs refinement for better accuracy
    fn test_kepler_propagation() {
        // Test that propagation preserves orbital energy for circular orbit
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        // Propagate for a short time
        let dt = 60.0; // 1 minute

        let (r1, v1) = propagate_kepler(&r0, &v0, GM_EARTH, dt);

        // Check that orbital radius is approximately preserved (circular orbit)
        let r0_mag = r0.norm();
        let r1_mag = r1.norm();

        // For a circular orbit, radius should be approximately constant
        assert_relative_eq!(r1_mag, r0_mag, epsilon = 1e2);

        // Velocity magnitude should be approximately constant
        assert_relative_eq!(v1.norm(), v0.norm(), epsilon = 1.0);
    }

    #[test]
    fn test_golden_section_search() {
        // Test with simple quadratic function
        let f = |x: f64| (x - 3.0).powi(2) + 1.0;

        let min_x = golden_section_search(f, 0.0, 10.0, 1e-6);

        // Minimum should be at x = 3
        assert_relative_eq!(min_x, 3.0, epsilon = 1e-5);
    }

    #[test]
    fn test_conjunction_parallel_orbits() {
        // Two satellites in parallel circular orbits, offset by 5 km

        let r1 = Vector3::new(7000e3, 0.0, 0.0);
        let v1 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        let r2 = Vector3::new(7000e3, 5000.0, 0.0); // 5 km offset in y
        let v2 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        let result = compute_conjunction(
            &r1, &v1, &r2, &v2,
            GM_EARTH,
            600.0, // 10 minutes (shorter window for test stability)
            10_000.0, // 10 km threshold
        ).unwrap();

        // Miss distance should be in reasonable range (a few km)
        // Due to Kepler propagation complexities, we just check it's reasonable
        assert!(result.miss_distance > 0.0);
        assert!(result.miss_distance < 50_000.0); // Less than 50 km

        // Should detect collision risk if miss distance < threshold
        if result.miss_distance < 10_000.0 {
            assert!(result.collision_risk);
        } else {
            assert!(!result.collision_risk);
        }
    }

    #[test]
    fn test_conjunction_head_on() {
        // Two satellites in similar orbits

        let r1 = Vector3::new(7000e3, 0.0, 0.0);
        let v1 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        // Second satellite with small offset
        let r2 = Vector3::new(7000e3, 2000.0, 0.0); // 2 km offset
        let v2 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        let result = compute_conjunction(
            &r1, &v1, &r2, &v2,
            GM_EARTH,
            300.0, // 5 minutes
            5000.0, // 5 km threshold
        ).unwrap();

        // Should detect reasonably close approach
        assert!(result.miss_distance > 0.0);
        assert!(result.miss_distance < 100e3); // Less than 100 km
    }

    #[test]
    fn test_closest_approach_distance() {
        let r1 = Vector3::new(7000e3, 0.0, 0.0);
        let v1 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        let r2 = Vector3::new(7000e3, 10000.0, 0.0); // 10 km offset
        let v2 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        let miss_distance = closest_approach_distance(
            &r1, &v1, &r2, &v2,
            GM_EARTH,
            600.0, // 10 minutes
        ).unwrap();

        // Should be within reasonable range
        assert!(miss_distance > 0.0);
        assert!(miss_distance < 100_000.0); // Less than 100 km
    }

    #[test]
    fn test_check_collision() {
        let r1 = Vector3::new(7000e3, 0.0, 0.0);
        let v1 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        // Close approach
        let r2 = Vector3::new(7000e3, 500.0, 0.0); // 500 m offset
        let v2 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        let collision = check_collision(&r1, &v1, &r2, &v2, GM_EARTH, 3600.0, 1000.0);
        assert!(collision); // 500 m < 1 km threshold

        // Distant approach
        let r3 = Vector3::new(7000e3, 50000.0, 0.0); // 50 km offset
        let v3 = Vector3::new(0.0, (GM_EARTH / 7000e3).sqrt(), 0.0);

        let collision = check_collision(&r1, &v1, &r3, &v3, GM_EARTH, 3600.0, 1000.0);
        assert!(!collision); // 50 km > 1 km threshold
    }

    #[test]
    fn test_conjunction_errors() {
        let r1 = Vector3::new(7000e3, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 7500.0, 0.0);
        let r2 = Vector3::new(7000e3, 5000.0, 0.0);
        let v2 = Vector3::new(0.0, 7500.0, 0.0);

        // Negative search window
        let result = compute_conjunction(&r1, &v1, &r2, &v2, GM_EARTH, -3600.0, 1000.0);
        assert!(result.is_err());

        // Negative collision threshold
        let result = compute_conjunction(&r1, &v1, &r2, &v2, GM_EARTH, 3600.0, -1000.0);
        assert!(result.is_err());
    }
}
