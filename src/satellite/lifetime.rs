//! Satellite Lifetime Estimation
//!
//! This module provides functions for estimating the orbital lifetime of satellites
//! subject to atmospheric drag. The lifetime is defined as the time until the satellite
//! reaches a terminal altitude (typically the Karman line at 100 km).
//!
//! # Theory
//!
//! Atmospheric drag causes orbital decay, gradually reducing the semi-major axis
//! and altitude of a satellite's orbit. The drag force is given by:
//!
//! ```text
//! F_drag = -(1/2) * ρ * v² * Cd * A
//! ```
//!
//! Where:
//! - ρ: atmospheric density (exponential model: ρ = ρ₀ * exp(-(h - h₀)/H))
//! - v: velocity relative to atmosphere
//! - Cd: drag coefficient (~2.2 for satellites)
//! - A: cross-sectional area
//!
//! The **ballistic coefficient** B = Cd * A / m characterizes the satellite's
//! susceptibility to drag:
//! - High B (large area, low mass): rapid decay
//! - Low B (small area, high mass): slow decay
//!
//! # Accuracy
//!
//! Lifetime predictions are inherently uncertain due to:
//! - Atmospheric density variations (solar activity, geomagnetic storms)
//! - Drag coefficient uncertainty
//! - Long-term solar activity predictions
//!
//! Even sophisticated models claim only ±10% accuracy for predictions beyond a few orbits.
//!
//! # References
//! - Vallado, "Fundamentals of Astrodynamics" Section 8.7
//! - King-Hele, "Satellite Orbits in an Atmosphere" (1987)
//! - Curtis, "Orbital Mechanics for Engineering Students" Section 12.7
//! - Acta Astronautica 225 (2024) 601-610 (decay time estimates)

use crate::core::error::{PoliastroError, PoliastroResult};
use crate::core::linalg::Vector3;
use crate::propagators::perturbations::{drag_acceleration, j2_perturbation};
use crate::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};

/// Default terminal altitude for reentry (Karman line) in meters
pub const DEFAULT_TERMINAL_ALTITUDE: f64 = 100_000.0; // 100 km

/// Default atmospheric density at sea level (kg/m³)
pub const DEFAULT_RHO0: f64 = 1.225;

/// Default reference altitude for atmospheric model (m)
pub const DEFAULT_H0: f64 = 0.0;

/// Default scale height for exponential atmosphere (m)
/// Typical value for LEO altitudes (200-600 km)
pub const DEFAULT_SCALE_HEIGHT: f64 = 8500.0;

/// Typical drag coefficient for satellites
pub const TYPICAL_DRAG_COEFFICIENT: f64 = 2.2;

/// Estimate satellite lifetime with atmospheric drag
///
/// Propagates the orbit forward in time using RK4 integration with drag and J2 perturbations
/// until the altitude drops below the terminal altitude. Returns the estimated lifetime in seconds.
///
/// # Arguments
///
/// * `r0` - Initial position vector [x, y, z] in meters (inertial frame)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (inertial frame)
/// * `ballistic_coeff` - Ballistic coefficient Cd*A/m in m²/kg (typical: 0.001-0.1)
/// * `terminal_altitude` - Altitude below which satellite is considered reentered (m, default: 100 km)
/// * `max_time` - Maximum propagation time in seconds (to prevent infinite loops)
/// * `time_step` - Integration time step in seconds (adaptive internally)
///
/// # Returns
///
/// Estimated lifetime in seconds (time until altitude < terminal_altitude)
///
/// # Errors
///
/// Returns error if:
/// - Initial altitude is already below terminal altitude
/// - Orbit doesn't decay within max_time (likely numerical issue)
/// - Invalid input parameters
///
/// # Example
///
/// ```ignore
/// use astrora::satellite::lifetime::estimate_lifetime;
/// use astrora::core::linalg::Vector3;
///
/// // LEO satellite at 400 km altitude
/// let r0 = Vector3::new(6778e3, 0.0, 0.0);  // Circular orbit on equator
/// let v0 = Vector3::new(0.0, 7670.0, 0.0);  // Circular velocity
///
/// // CubeSat: Cd = 2.2, A = 0.01 m² (10cm × 10cm), m = 1 kg
/// let B = 2.2 * 0.01 / 1.0; // = 0.022 m²/kg
///
/// let lifetime_days = estimate_lifetime(
///     &r0, &v0,
///     B,
///     100e3,  // 100 km terminal altitude
///     365.25 * 86400.0, // Max 1 year
///     600.0   // 10-minute time steps
/// ).unwrap() / 86400.0;
///
/// println!("Estimated lifetime: {:.1} days", lifetime_days);
/// ```
///
/// # Notes
///
/// - Uses exponential atmospheric model (suitable for LEO, not high accuracy)
/// - Includes J2 perturbation for realistic orbit evolution
/// - Time step is adaptive: larger steps at higher altitudes
/// - For very low ballistic coefficients, may take hours to compute
/// - Does not account for solar activity variations (assumes nominal conditions)
pub fn estimate_lifetime(
    r0: &Vector3,
    v0: &Vector3,
    ballistic_coeff: f64,
    terminal_altitude: f64,
    max_time: f64,
    initial_time_step: f64,
) -> PoliastroResult<f64> {
    // Validate inputs
    if ballistic_coeff <= 0.0 {
        return Err(PoliastroError::invalid_parameter(
            "ballistic_coeff",
            ballistic_coeff,
            "must be positive",
        ));
    }

    if terminal_altitude < 0.0 {
        return Err(PoliastroError::invalid_parameter(
            "terminal_altitude",
            terminal_altitude,
            "must be non-negative",
        ));
    }

    if max_time <= 0.0 {
        return Err(PoliastroError::invalid_parameter(
            "max_time",
            max_time,
            "must be positive",
        ));
    }

    // Check initial altitude
    let r0_mag = r0.norm();
    let h0 = r0_mag - R_EARTH;

    if h0 < terminal_altitude {
        return Err(PoliastroError::invalid_state(
            format!(
                "Initial altitude ({:.1} km) is already below terminal altitude ({:.1} km)",
                h0 / 1000.0,
                terminal_altitude / 1000.0
            ),
        ));
    }

    // Initialize state
    let mut r = *r0;
    let mut v = *v0;
    let mut time = 0.0;
    let mut dt = initial_time_step;

    // Atmospheric parameters
    let rho0 = DEFAULT_RHO0;
    let h_ref = DEFAULT_H0;
    let H = DEFAULT_SCALE_HEIGHT;

    // Adaptive time stepping based on altitude
    // Higher altitudes = slower decay = can use larger time steps
    let get_adaptive_dt = |altitude: f64| -> f64 {
        if altitude > 600_000.0 {
            // Above 600 km: very slow decay, use large steps
            86400.0 // 1 day
        } else if altitude > 400_000.0 {
            // 400-600 km: moderate decay
            3600.0 // 1 hour
        } else if altitude > 200_000.0 {
            // 200-400 km: faster decay
            600.0 // 10 minutes
        } else if altitude > 150_000.0 {
            // 150-200 km: rapid decay
            60.0 // 1 minute
        } else {
            // Below 150 km: very rapid decay
            10.0 // 10 seconds
        }
    };

    // Propagate until terminal altitude or max time
    while time < max_time {
        // Current altitude
        let r_mag = r.norm();
        let altitude = r_mag - R_EARTH;

        // Check if we've reached terminal altitude
        if altitude < terminal_altitude {
            return Ok(time);
        }

        // Adaptive time step based on altitude
        dt = get_adaptive_dt(altitude).min(max_time - time);

        // RK4 integration step with drag + J2
        // k1 = f(t, y)
        let a_drag1 = drag_acceleration(&r, &v, R_EARTH, rho0, h_ref, ballistic_coeff);
        let a_j2_1 = j2_perturbation(&r, GM_EARTH, J2_EARTH, R_EARTH);
        let a_gravity1 = r.scale(-GM_EARTH / (r_mag * r_mag * r_mag));
        let a1 = a_gravity1 + a_drag1 + a_j2_1;

        let k1_r = v;
        let k1_v = a1;

        // k2 = f(t + dt/2, y + k1*dt/2)
        let r2 = r + k1_r.scale(dt / 2.0);
        let v2 = v + k1_v.scale(dt / 2.0);
        let r2_mag = r2.norm();
        let a_drag2 = drag_acceleration(&r2, &v2, R_EARTH, rho0, h_ref, ballistic_coeff);
        let a_j2_2 = j2_perturbation(&r2, GM_EARTH, J2_EARTH, R_EARTH);
        let a_gravity2 = r2.scale(-GM_EARTH / (r2_mag * r2_mag * r2_mag));
        let a2 = a_gravity2 + a_drag2 + a_j2_2;

        let k2_r = v2;
        let k2_v = a2;

        // k3 = f(t + dt/2, y + k2*dt/2)
        let r3 = r + k2_r.scale(dt / 2.0);
        let v3 = v + k2_v.scale(dt / 2.0);
        let r3_mag = r3.norm();
        let a_drag3 = drag_acceleration(&r3, &v3, R_EARTH, rho0, h_ref, ballistic_coeff);
        let a_j2_3 = j2_perturbation(&r3, GM_EARTH, J2_EARTH, R_EARTH);
        let a_gravity3 = r3.scale(-GM_EARTH / (r3_mag * r3_mag * r3_mag));
        let a3 = a_gravity3 + a_drag3 + a_j2_3;

        let k3_r = v3;
        let k3_v = a3;

        // k4 = f(t + dt, y + k3*dt)
        let r4 = r + k3_r.scale(dt);
        let v4 = v + k3_v.scale(dt);
        let r4_mag = r4.norm();
        let a_drag4 = drag_acceleration(&r4, &v4, R_EARTH, rho0, h_ref, ballistic_coeff);
        let a_j2_4 = j2_perturbation(&r4, GM_EARTH, J2_EARTH, R_EARTH);
        let a_gravity4 = r4.scale(-GM_EARTH / (r4_mag * r4_mag * r4_mag));
        let a4 = a_gravity4 + a_drag4 + a_j2_4;

        let k4_r = v4;
        let k4_v = a4;

        // Update state: y_{n+1} = y_n + (dt/6) * (k1 + 2*k2 + 2*k3 + k4)
        r += (k1_r + k2_r.scale(2.0) + k3_r.scale(2.0) + k4_r).scale(dt / 6.0);
        v += (k1_v + k2_v.scale(2.0) + k3_v.scale(2.0) + k4_v).scale(dt / 6.0);
        time += dt;
    }

    // If we got here, satellite didn't decay within max_time
    Err(PoliastroError::convergence_failure(
        "lifetime estimation",
        0,
        terminal_altitude,
    ))
}

/// Estimate decay rate at a given altitude
///
/// Computes the instantaneous rate of altitude loss (dh/dt) for a circular orbit
/// at the given altitude with the specified ballistic coefficient.
///
/// This is useful for:
/// - Quick estimates of decay without full propagation
/// - Understanding sensitivity to ballistic coefficient
/// - Validating lifetime estimates
///
/// # Arguments
///
/// * `altitude` - Orbital altitude in meters
/// * `ballistic_coeff` - Ballistic coefficient Cd*A/m in m²/kg
///
/// # Returns
///
/// Decay rate dh/dt in meters/day (negative value)
///
/// # Example
///
/// ```ignore
/// use astrora::satellite::lifetime::estimate_decay_rate;
///
/// // ISS-like satellite at 400 km
/// let decay_rate = estimate_decay_rate(400e3, 0.005);
/// println!("Decay rate: {:.1} m/day", decay_rate);
/// ```
///
/// # Notes
///
/// - Assumes circular orbit for simplicity
/// - Uses exponential atmospheric model
/// - Does not account for J2 or other perturbations
/// - More accurate at lower altitudes where drag dominates
pub fn estimate_decay_rate(altitude: f64, ballistic_coeff: f64) -> f64 {
    // Circular orbit velocity at this altitude
    let r = R_EARTH + altitude;
    let v_circ = (GM_EARTH / r).sqrt();

    // Atmospheric density at this altitude
    let rho = DEFAULT_RHO0 * (-(altitude - DEFAULT_H0) / DEFAULT_SCALE_HEIGHT).exp();

    // Drag acceleration magnitude (circular orbit, velocity = orbital velocity)
    let a_drag_mag = 0.5 * rho * v_circ * v_circ * ballistic_coeff;

    // For circular orbit, da/dt = 2*a*a_drag/v
    // And h = a - R, so dh/dt ≈ da/dt = 2*a*a_drag/v
    let a = r;
    let da_dt = 2.0 * a * a_drag_mag / v_circ;

    // Convert to meters/day
    -da_dt * 86400.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_decay_rate_higher_altitude_slower() {
        // Higher altitude should have slower decay
        let B = 0.01; // m²/kg

        let rate_200km = estimate_decay_rate(200_000.0, B);
        let rate_400km = estimate_decay_rate(400_000.0, B);
        let rate_600km = estimate_decay_rate(600_000.0, B);

        // Decay rates should be negative
        assert!(rate_200km < 0.0);
        assert!(rate_400km < 0.0);
        assert!(rate_600km < 0.0);

        // Lower altitude = faster decay (more negative)
        assert!(rate_200km.abs() > rate_400km.abs());
        assert!(rate_400km.abs() > rate_600km.abs());
    }

    #[test]
    fn test_decay_rate_higher_ballistic_coeff_faster() {
        // Higher ballistic coefficient = faster decay
        let altitude = 400_000.0;

        let rate_low_B = estimate_decay_rate(altitude, 0.001);
        let rate_high_B = estimate_decay_rate(altitude, 0.1);

        // Both negative
        assert!(rate_low_B < 0.0);
        assert!(rate_high_B < 0.0);

        // Higher B = faster decay
        assert!(rate_high_B.abs() > rate_low_B.abs());
    }

    #[test]
    fn test_decay_rate_scaling() {
        // Decay rate should scale linearly with ballistic coefficient
        let altitude = 400_000.0;
        let B1 = 0.01;
        let B2 = 0.02;

        let rate1 = estimate_decay_rate(altitude, B1);
        let rate2 = estimate_decay_rate(altitude, B2);

        // Should be approximately 2x
        assert_relative_eq!(rate2 / rate1, 2.0, epsilon = 0.01);
    }

    #[test]
    #[ignore] // Slow test requiring significant integration time
    fn test_lifetime_estimation_basic() {
        // Test basic lifetime estimation for a satellite that will decay quickly

        // Very low orbit (150 km) with extremely high ballistic coefficient
        let r0 = Vector3::new(R_EARTH + 150_000.0, 0.0, 0.0);
        let v_circ = (GM_EARTH / r0.norm()).sqrt();
        let v0 = Vector3::new(0.0, v_circ, 0.0);

        // Extremely high ballistic coefficient for very fast decay
        let B = 0.5; // Very large area, small mass (e.g., balloon satellite)

        // Estimate lifetime
        let lifetime = estimate_lifetime(
            &r0,
            &v0,
            B,
            100_000.0,      // 100 km terminal
            10.0 * 86400.0, // Max 10 days
            10.0,           // 10 second initial step
        ).unwrap();

        // Should decay within 10 days
        assert!(lifetime > 0.0);
        assert!(lifetime < 10.0 * 86400.0);

        // Should be on the order of hours to days
        let lifetime_hours = lifetime / 3600.0;
        assert!(lifetime_hours > 0.1); // At least a few minutes
        assert!(lifetime_hours < 240.0); // Less than 10 days
    }

    #[test]
    #[ignore] // Slow test requiring significant integration time
    fn test_lifetime_higher_orbit_longer() {
        // Higher initial altitude should give longer lifetime

        let B = 0.1; // Higher B for faster testing

        // 140 km orbit (very low, will decay quickly)
        let r1 = Vector3::new(R_EARTH + 140_000.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, (GM_EARTH / r1.norm()).sqrt(), 0.0);

        // 160 km orbit (slightly higher)
        let r2 = Vector3::new(R_EARTH + 160_000.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, (GM_EARTH / r2.norm()).sqrt(), 0.0);

        let lifetime1 = estimate_lifetime(&r1, &v1, B, 100_000.0, 10.0 * 86400.0, 10.0).unwrap();
        let lifetime2 = estimate_lifetime(&r2, &v2, B, 100_000.0, 20.0 * 86400.0, 10.0).unwrap();

        // Higher orbit should last longer
        assert!(lifetime2 > lifetime1);

        // Should both be reasonable (hours to days)
        assert!(lifetime1 < 10.0 * 86400.0);
        assert!(lifetime2 < 20.0 * 86400.0);
    }

    #[test]
    fn test_lifetime_errors() {
        let r0 = Vector3::new(R_EARTH + 400_000.0, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);

        // Negative ballistic coefficient
        let result = estimate_lifetime(&r0, &v0, -0.01, 100_000.0, 86400.0, 60.0);
        assert!(result.is_err());

        // Negative terminal altitude
        let result = estimate_lifetime(&r0, &v0, 0.01, -100_000.0, 86400.0, 60.0);
        assert!(result.is_err());

        // Initial altitude below terminal
        let r_low = Vector3::new(R_EARTH + 50_000.0, 0.0, 0.0);
        let result = estimate_lifetime(&r_low, &v0, 0.01, 100_000.0, 86400.0, 60.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_lifetime_zero_ballistic_coeff_error() {
        let r0 = Vector3::new(R_EARTH + 400_000.0, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);

        let result = estimate_lifetime(&r0, &v0, 0.0, 100_000.0, 86400.0, 60.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("ballistic_coeff"));
    }

    #[test]
    fn test_lifetime_zero_max_time_error() {
        let r0 = Vector3::new(R_EARTH + 400_000.0, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);

        let result = estimate_lifetime(&r0, &v0, 0.01, 100_000.0, 0.0, 60.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("max_time"));
    }

    #[test]
    fn test_lifetime_altitude_just_below_terminal() {
        // Test edge case where initial altitude is just below terminal altitude
        let r0 = Vector3::new(R_EARTH + 99_000.0, 0.0, 0.0); // 99 km < 100 km terminal
        let v0 = Vector3::new(0.0, 7670.0, 0.0);

        let result = estimate_lifetime(&r0, &v0, 0.01, 100_000.0, 86400.0, 60.0);
        // Should error because altitude is below terminal
        assert!(result.is_err());
    }

    #[test]
    fn test_decay_rate_very_low_altitude() {
        // Test decay rate at very low altitude (high density)
        let B = 0.01;
        let rate = estimate_decay_rate(100_000.0, B); // 100 km

        // Should be strongly negative (fast decay)
        assert!(rate < -0.01); // At least 10 m/s² decay rate magnitude
    }

    #[test]
    fn test_decay_rate_very_high_altitude() {
        // Test decay rate at very high altitude (low density)
        let B = 0.01;
        let rate = estimate_decay_rate(1_000_000.0, B); // 1000 km

        // Should be very small (slow decay)
        assert!(rate > -1e-6); // Very small magnitude
        assert!(rate < 0.0);   // But still negative
    }

    #[test]
    fn test_decay_rate_zero_ballistic_coeff() {
        // Zero ballistic coefficient should give zero decay rate
        let rate = estimate_decay_rate(400_000.0, 0.0);
        assert_relative_eq!(rate, 0.0, epsilon = 1e-15);
    }

    #[test]
    fn test_decay_rate_altitude_scaling() {
        // Test exponential scaling with altitude
        let B = 0.01;

        let rate_300 = estimate_decay_rate(300_000.0, B);
        let rate_400 = estimate_decay_rate(400_000.0, B);
        let rate_500 = estimate_decay_rate(500_000.0, B);

        // All should be negative
        assert!(rate_300 < 0.0);
        assert!(rate_400 < 0.0);
        assert!(rate_500 < 0.0);

        // Each 100 km increase should reduce decay rate exponentially
        // rate should scale as exp(-Δh / H) where H ≈ 8500 m
        let ratio_1 = rate_300 / rate_400;
        let ratio_2 = rate_400 / rate_500;

        // Each ratio should be significantly greater than 1 (faster decay at lower altitude)
        // exp(100km / 8.5km) ≈ exp(11.76) ≈ 127,000
        assert!(ratio_1 > 10.0);
        assert!(ratio_2 > 10.0);
    }

    #[test]
    fn test_decay_rate_ballistic_coeff_range() {
        // Test with a range of realistic ballistic coefficients
        let altitude = 400_000.0;

        // CubeSat: small area, low mass
        let B_cubesat = 0.001; // m²/kg
        let rate_cubesat = estimate_decay_rate(altitude, B_cubesat);

        // Large satellite with solar panels: large area, medium mass
        let B_solar = 0.05; // m²/kg
        let rate_solar = estimate_decay_rate(altitude, B_solar);

        // Balloon satellite: very large area, low mass
        let B_balloon = 0.2; // m²/kg
        let rate_balloon = estimate_decay_rate(altitude, B_balloon);

        // All should be negative
        assert!(rate_cubesat < 0.0);
        assert!(rate_solar < 0.0);
        assert!(rate_balloon < 0.0);

        // Should scale with B
        assert!(rate_balloon.abs() > rate_solar.abs());
        assert!(rate_solar.abs() > rate_cubesat.abs());
    }

    #[test]
    fn test_decay_rate_consistency() {
        // Decay rate should be smooth and continuous
        let B = 0.01;

        let h1 = 350_000.0;
        let h2 = 351_000.0; // 1 km higher

        let rate1 = estimate_decay_rate(h1, B);
        let rate2 = estimate_decay_rate(h2, B);

        // Both should be negative
        assert!(rate1 < 0.0);
        assert!(rate2 < 0.0);

        // Lower altitude should have faster (more negative) decay
        assert!(rate1.abs() > rate2.abs());

        // Rates should be relatively close (continuity)
        // With scale height of 8.5 km, 1 km change is exp(1/8.5) ≈ 1.125 (12.5% difference)
        let relative_diff = (rate1 - rate2).abs() / rate1.abs();
        assert!(relative_diff < 0.15); // Less than 15% difference for 1 km change
    }

    #[test]
    fn test_constants() {
        // Verify module constants are reasonable
        assert_eq!(DEFAULT_TERMINAL_ALTITUDE, 100_000.0);
        assert_eq!(DEFAULT_RHO0, 1.225);
        assert_eq!(DEFAULT_H0, 0.0);
        assert_eq!(DEFAULT_SCALE_HEIGHT, 8500.0);
        assert_eq!(TYPICAL_DRAG_COEFFICIENT, 2.2);

        // Scale height should be positive
        assert!(DEFAULT_SCALE_HEIGHT > 0.0);

        // Sea level density should be positive
        assert!(DEFAULT_RHO0 > 0.0);

        // Terminal altitude should be reasonable
        assert!(DEFAULT_TERMINAL_ALTITUDE > 0.0);
        assert!(DEFAULT_TERMINAL_ALTITUDE < 200_000.0); // Should be in lower atmosphere
    }

    #[test]
    fn test_decay_rate_iss_altitude() {
        // Test at ISS altitude (~408 km)
        let iss_altitude = 408_000.0;
        let B_iss = 0.0001; // ISS is massive with small cross-section

        let rate = estimate_decay_rate(iss_altitude, B_iss);

        // Should have slow but non-zero decay
        assert!(rate < 0.0);
        assert!(rate > -1e-4); // Very slow decay
    }

    #[test]
    fn test_decay_rate_geo_altitude() {
        // Test at GEO altitude (~36,000 km)
        let geo_altitude = 36_000_000.0;
        let B = 0.01;

        let rate = estimate_decay_rate(geo_altitude, B);

        // At GEO, atmospheric density is negligible, decay should be essentially zero
        assert_relative_eq!(rate, 0.0, epsilon = 1e-15);
    }

    #[test]
    #[ignore] // Fast test with extreme parameters for quick decay
    fn test_lifetime_extreme_parameters() {
        // Test with extreme parameters for very fast decay
        // Very low orbit with huge ballistic coefficient

        let r0 = Vector3::new(R_EARTH + 110_000.0, 0.0, 0.0); // Just above terminal
        let v_circ = (GM_EARTH / r0.norm()).sqrt();
        let v0 = Vector3::new(0.0, v_circ, 0.0);

        // Enormous ballistic coefficient for instant decay
        let B = 1.0; // Extremely large

        let lifetime = estimate_lifetime(
            &r0,
            &v0,
            B,
            100_000.0,  // Terminal at 100 km
            3600.0,     // Max 1 hour
            1.0,        // 1 second time step
        );

        // Should succeed and give short lifetime
        assert!(lifetime.is_ok());
        let time = lifetime.unwrap();
        assert!(time > 0.0);
        assert!(time < 3600.0); // Less than 1 hour
    }

    #[test]
    fn test_lifetime_max_time_exceeded() {
        // Test that we get an error when max_time is too short
        let r0 = Vector3::new(R_EARTH + 400_000.0, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);

        // Very small max_time, satellite won't decay
        let result = estimate_lifetime(&r0, &v0, 0.001, 100_000.0, 1.0, 1.0);

        // Should error due to exceeding max_time
        assert!(result.is_err());
    }

    #[test]
    fn test_lifetime_negative_terminal_altitude() {
        // Test error handling for negative terminal altitude
        let r0 = Vector3::new(R_EARTH + 400_000.0, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);

        let result = estimate_lifetime(&r0, &v0, 0.01, -1000.0, 86400.0, 600.0);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("terminal_altitude"));
    }

    #[test]
    fn test_lifetime_validation_checks() {
        // Test all validation checks in estimate_lifetime
        let r0 = Vector3::new(R_EARTH + 400_000.0, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7670.0, 0.0);

        // Negative ballistic coefficient
        let result = estimate_lifetime(&r0, &v0, -0.01, 100_000.0, 86400.0, 600.0);
        assert!(result.is_err());

        // Zero ballistic coefficient (already tested separately)
        // Negative max_time
        let result = estimate_lifetime(&r0, &v0, 0.01, 100_000.0, -100.0, 600.0);
        assert!(result.is_err());
    }
}
