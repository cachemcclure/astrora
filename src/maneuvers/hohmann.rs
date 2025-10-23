//! Hohmann transfer orbit calculations
//!
//! A Hohmann transfer is the most efficient two-impulse maneuver for transferring
//! between two coplanar circular orbits. It uses an elliptical transfer orbit that
//! is tangent to both the initial and final orbits.
//!
//! # Theory
//!
//! The transfer consists of two impulsive burns:
//! 1. **First burn (ΔV₁)**: At the initial orbit, increase velocity to enter the
//!    elliptical transfer orbit
//! 2. **Second burn (ΔV₂)**: At the transfer orbit apoapsis/periapsis, increase or
//!    decrease velocity to circularize at the final orbit
//!
//! ## Assumptions
//! - Initial and final orbits are circular
//! - Orbits are coplanar (no inclination change)
//! - Maneuvers are impulsive (instantaneous velocity changes)
//! - Two-body problem (no perturbations)
//!
//! # Equations
//!
//! For a transfer from radius r₁ to r₂:
//!
//! **First burn (ΔV₁):**
//! ```text
//! ΔV₁ = √(μ/r₁) × [√(2r₂/(r₁+r₂)) - 1]
//! ```
//!
//! **Second burn (ΔV₂):**
//! ```text
//! ΔV₂ = √(μ/r₂) × [1 - √(2r₁/(r₁+r₂))]
//! ```
//!
//! **Transfer time (half period of ellipse):**
//! ```text
//! t_transfer = π × √((r₁+r₂)³/(8μ))
//! ```
//!
//! # References
//! - Hohmann, W. (1925). Die Erreichbarkeit der Himmelskörper.
//! - Curtis, H. D. (2013). Orbital Mechanics for Engineering Students. Ch. 6.2
//! - Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications. Ch. 6.2
//! - <https://orbital-mechanics.space/orbital-maneuvers/hohmann-transfer.html>
//! - <https://en.wikipedia.org/wiki/Hohmann_transfer_orbit>

use std::f64::consts::PI;

use crate::core::{PoliastroError, PoliastroResult};

/// Result of a Hohmann transfer calculation
///
/// Contains all relevant parameters for a Hohmann transfer between two
/// circular coplanar orbits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HohmannTransferResult {
    /// Initial orbit radius (m)
    pub r_initial: f64,
    /// Final orbit radius (m)
    pub r_final: f64,
    /// Gravitational parameter μ = GM (m³/s²)
    pub mu: f64,
    /// First burn delta-v (m/s) - to enter transfer orbit
    pub delta_v1: f64,
    /// Second burn delta-v (m/s) - to circularize at final orbit
    pub delta_v2: f64,
    /// Total delta-v required (m/s)
    pub delta_v_total: f64,
    /// Transfer time (seconds) - half period of elliptical transfer orbit
    pub transfer_time: f64,
    /// Transfer orbit semi-major axis (m)
    pub transfer_sma: f64,
    /// Transfer orbit eccentricity (dimensionless)
    pub transfer_eccentricity: f64,
    /// Initial circular orbit velocity (m/s)
    pub v_initial: f64,
    /// Final circular orbit velocity (m/s)
    pub v_final: f64,
    /// Velocity at periapsis of transfer orbit (m/s)
    pub v_transfer_periapsis: f64,
    /// Velocity at apoapsis of transfer orbit (m/s)
    pub v_transfer_apoapsis: f64,
}

/// Hohmann transfer calculator
///
/// Provides methods for calculating optimal two-impulse transfers between
/// circular coplanar orbits.
pub struct HohmannTransfer;

impl HohmannTransfer {
    /// Calculate a Hohmann transfer between two circular orbits
    ///
    /// # Arguments
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// A `HohmannTransferResult` containing all transfer parameters
    ///
    /// # Errors
    /// Returns `PoliastroError` if:
    /// - Either radius is non-positive
    /// - Initial and final radii are equal (no transfer needed)
    /// - Gravitational parameter is non-positive
    ///
    /// # Examples
    /// ```
    /// use astrora::maneuvers::HohmannTransfer;
    /// use astrora::core::constants::earth;
    ///
    /// // Transfer from LEO (400 km) to GEO (35,786 km)
    /// let r_leo = earth::MEAN_RADIUS + 400e3;  // m
    /// let r_geo = earth::MEAN_RADIUS + 35_786e3;  // m
    ///
    /// let result = HohmannTransfer::calculate(r_leo, r_geo, earth::MU).unwrap();
    ///
    /// println!("ΔV₁: {:.1} m/s", result.delta_v1);
    /// println!("ΔV₂: {:.1} m/s", result.delta_v2);
    /// println!("Total ΔV: {:.1} m/s", result.delta_v_total);
    /// println!("Transfer time: {:.1} hours", result.transfer_time / 3600.0);
    /// ```
    pub fn calculate(r_initial: f64, r_final: f64, mu: f64) -> PoliastroResult<HohmannTransferResult> {
        // Validation
        if r_initial <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_initial",
                r_initial,
                "must be positive"
            ));
        }
        if r_final <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_final",
                r_final,
                "must be positive"
            ));
        }
        if mu <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "mu",
                mu,
                "must be positive"
            ));
        }
        if (r_initial - r_final).abs() < 1e-6 {
            return Err(PoliastroError::invalid_parameter(
                "r_initial, r_final",
                r_initial,
                "radii must be different - no transfer needed"
            ));
        }

        // Calculate circular orbit velocities
        // v_circular = √(μ/r)
        let v_initial = (mu / r_initial).sqrt();
        let v_final = (mu / r_final).sqrt();

        // Transfer orbit parameters
        let transfer_sma = (r_initial + r_final) / 2.0;

        // Calculate transfer orbit eccentricity
        // For an ellipse with periapsis r_p and apoapsis r_a:
        // a = (r_p + r_a)/2, e = (r_a - r_p)/(r_a + r_p)
        let r_min = r_initial.min(r_final);
        let r_max = r_initial.max(r_final);
        let transfer_eccentricity = (r_max - r_min) / (r_max + r_min);

        // Calculate transfer orbit velocities using vis-viva equation
        // v = √(μ(2/r - 1/a))
        let v_transfer_periapsis = (mu * (2.0 / r_min - 1.0 / transfer_sma)).sqrt();
        let v_transfer_apoapsis = (mu * (2.0 / r_max - 1.0 / transfer_sma)).sqrt();

        // Calculate delta-v values
        let (delta_v1, delta_v2) = if r_initial < r_final {
            // Ascending transfer (LEO to GEO)
            // First burn: increase velocity at periapsis (initial orbit)
            // ΔV₁ = √(μ/r₁) × [√(2r₂/(r₁+r₂)) - 1]
            let dv1 = v_transfer_periapsis - v_initial;

            // Second burn: increase velocity at apoapsis (final orbit)
            // ΔV₂ = √(μ/r₂) × [1 - √(2r₁/(r₁+r₂))]
            let dv2 = v_final - v_transfer_apoapsis;

            (dv1, dv2)
        } else {
            // Descending transfer (GEO to LEO)
            // First burn: decrease velocity at apoapsis (initial orbit)
            let dv1 = v_initial - v_transfer_apoapsis;

            // Second burn: decrease velocity at periapsis (final orbit)
            let dv2 = v_transfer_periapsis - v_final;

            (dv1, dv2)
        };

        let delta_v_total = delta_v1 + delta_v2;

        // Calculate transfer time (half the period of the elliptical orbit)
        // t = π × √(a³/μ) = π × √((r₁+r₂)³/(8μ))
        let transfer_time = PI * (transfer_sma.powi(3) / mu).sqrt();

        Ok(HohmannTransferResult {
            r_initial,
            r_final,
            mu,
            delta_v1,
            delta_v2,
            delta_v_total,
            transfer_time,
            transfer_sma,
            transfer_eccentricity,
            v_initial,
            v_final,
            v_transfer_periapsis,
            v_transfer_apoapsis,
        })
    }

    /// Calculate the optimal phase angle for a Hohmann transfer
    ///
    /// For a successful rendezvous, the target must be at a specific angular
    /// position when the transfer begins. This calculates the required phase angle.
    ///
    /// # Arguments
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// Phase angle in radians (0 to 2π). The target should be this angle ahead
    /// of the chaser at the time of the first burn.
    ///
    /// # Errors
    /// Returns `PoliastroError` if the transfer calculation fails
    ///
    /// # Theory
    /// During the transfer time, the target continues to orbit. For a successful
    /// rendezvous, the target must travel from its current position to the
    /// intercept point during the transfer time.
    ///
    /// The phase angle θ is calculated as:
    /// ```text
    /// θ = π - n_final × t_transfer
    /// ```
    /// where n_final is the mean motion of the final orbit and t_transfer is
    /// the Hohmann transfer time.
    pub fn phase_angle(r_initial: f64, r_final: f64, mu: f64) -> PoliastroResult<f64> {
        let result = Self::calculate(r_initial, r_final, mu)?;

        // Mean motion of final orbit: n = √(μ/r³)
        let n_final = (mu / r_final.powi(3)).sqrt();

        // During transfer time, target moves through angle: θ_target = n × t
        let theta_target = n_final * result.transfer_time;

        // Phase angle: target should be π - θ_target ahead of chaser
        // (π because we rendezvous at the opposite side of the orbit)
        let mut phase_angle = PI - theta_target;

        // Normalize to [0, 2π]
        while phase_angle < 0.0 {
            phase_angle += 2.0 * PI;
        }
        while phase_angle >= 2.0 * PI {
            phase_angle -= 2.0 * PI;
        }

        Ok(phase_angle)
    }

    /// Calculate the synodic period between two orbits
    ///
    /// The synodic period is the time between successive alignments of two
    /// objects in different orbits. This determines how often transfer
    /// opportunities occur.
    ///
    /// # Arguments
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// Synodic period in seconds
    ///
    /// # Errors
    /// Returns `PoliastroError` if any radius is non-positive
    ///
    /// # Theory
    /// The synodic period T_syn is given by:
    /// ```text
    /// 1/T_syn = |1/T₁ - 1/T₂|
    /// ```
    /// where T₁ and T₂ are the orbital periods.
    pub fn synodic_period(r_initial: f64, r_final: f64, mu: f64) -> PoliastroResult<f64> {
        if r_initial <= 0.0 || r_final <= 0.0 || mu <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_initial, r_final, mu",
                0.0,
                "all parameters must be positive"
            ));
        }

        // Orbital periods: T = 2π√(r³/μ)
        let t_initial = 2.0 * PI * (r_initial.powi(3) / mu).sqrt();
        let t_final = 2.0 * PI * (r_final.powi(3) / mu).sqrt();

        // Synodic period
        let synodic_period = 1.0 / ((1.0 / t_initial - 1.0 / t_final).abs());

        Ok(synodic_period)
    }

    /// Calculate time until next optimal transfer window
    ///
    /// Given the current phase angle between two objects, calculate how long
    /// until the next optimal transfer opportunity.
    ///
    /// # Arguments
    /// * `current_phase` - Current phase angle (radians, 0 to 2π)
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// Time until next transfer window in seconds
    ///
    /// # Errors
    /// Returns `PoliastroError` if calculation fails
    pub fn time_to_transfer_window(
        current_phase: f64,
        r_initial: f64,
        r_final: f64,
        mu: f64,
    ) -> PoliastroResult<f64> {
        let optimal_phase = Self::phase_angle(r_initial, r_final, mu)?;
        let synodic = Self::synodic_period(r_initial, r_final, mu)?;

        // Angular difference to next window
        let mut delta_phase = optimal_phase - current_phase;

        // Normalize to [0, 2π]
        while delta_phase < 0.0 {
            delta_phase += 2.0 * PI;
        }
        while delta_phase >= 2.0 * PI {
            delta_phase -= 2.0 * PI;
        }

        // Time = (angle / 2π) × synodic_period
        let wait_time = (delta_phase / (2.0 * PI)) * synodic;

        Ok(wait_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // Earth parameters for testing
    const EARTH_MU: f64 = 3.986004418e14; // m³/s²
    const EARTH_RADIUS: f64 = 6.371e6; // m

    #[test]
    fn test_leo_to_geo_transfer() {
        // LEO at 400 km altitude to GEO at 35,786 km
        let r_leo = EARTH_RADIUS + 400e3;
        let r_geo = EARTH_RADIUS + 35_786e3;

        let result = HohmannTransfer::calculate(r_leo, r_geo, EARTH_MU).unwrap();

        // Verify basic properties
        assert!(result.delta_v1 > 0.0, "First burn should be positive");
        assert!(result.delta_v2 > 0.0, "Second burn should be positive");
        assert_relative_eq!(result.delta_v_total, result.delta_v1 + result.delta_v2, epsilon = 1e-6);

        // Expected values (from standard references)
        // ΔV₁ ≈ 2,427 m/s, ΔV₂ ≈ 1,469 m/s, Total ≈ 3,896 m/s
        assert_relative_eq!(result.delta_v1, 2427.0, epsilon = 50.0);
        assert_relative_eq!(result.delta_v2, 1469.0, epsilon = 50.0);
        assert_relative_eq!(result.delta_v_total, 3896.0, epsilon = 100.0);

        // Transfer time should be about 5.25 hours
        let expected_time_hours = 5.25;
        assert_relative_eq!(
            result.transfer_time / 3600.0,
            expected_time_hours,
            epsilon = 0.1
        );
    }

    #[test]
    fn test_geo_to_leo_transfer() {
        // Descending transfer (GEO to LEO)
        let r_leo = EARTH_RADIUS + 400e3;
        let r_geo = EARTH_RADIUS + 35_786e3;

        let result = HohmannTransfer::calculate(r_geo, r_leo, EARTH_MU).unwrap();

        // Delta-v should be same magnitude as ascending transfer
        assert!(result.delta_v1 > 0.0);
        assert!(result.delta_v2 > 0.0);

        // Total ΔV should be same for both directions
        let ascending = HohmannTransfer::calculate(r_leo, r_geo, EARTH_MU).unwrap();
        assert_relative_eq!(result.delta_v_total, ascending.delta_v_total, epsilon = 1e-6);
    }

    #[test]
    fn test_transfer_orbit_properties() {
        let r1 = EARTH_RADIUS + 400e3;
        let r2 = EARTH_RADIUS + 10_000e3;

        let result = HohmannTransfer::calculate(r1, r2, EARTH_MU).unwrap();

        // Semi-major axis should be average of radii
        assert_relative_eq!(result.transfer_sma, (r1 + r2) / 2.0, epsilon = 1e-6);

        // Eccentricity should be in valid range
        assert!(result.transfer_eccentricity >= 0.0);
        assert!(result.transfer_eccentricity < 1.0);

        // Verify vis-viva equation at periapsis
        let expected_v_peri = (EARTH_MU * (2.0 / r1 - 1.0 / result.transfer_sma)).sqrt();
        assert_relative_eq!(result.v_transfer_periapsis, expected_v_peri, epsilon = 1e-6);
    }

    #[test]
    fn test_small_altitude_change() {
        // Small transfer (100 km altitude change)
        let r1 = EARTH_RADIUS + 400e3;
        let r2 = EARTH_RADIUS + 500e3;

        let result = HohmannTransfer::calculate(r1, r2, EARTH_MU).unwrap();

        // Delta-v should be relatively small
        assert!(result.delta_v_total < 100.0); // Less than 100 m/s
        assert!(result.transfer_time < 3600.0); // Less than 1 hour
    }

    #[test]
    fn test_error_handling() {
        // Negative radius
        assert!(HohmannTransfer::calculate(-1.0, 1e7, EARTH_MU).is_err());

        // Zero radius
        assert!(HohmannTransfer::calculate(0.0, 1e7, EARTH_MU).is_err());

        // Equal radii
        assert!(HohmannTransfer::calculate(1e7, 1e7, EARTH_MU).is_err());

        // Negative mu
        assert!(HohmannTransfer::calculate(7e6, 1e7, -1.0).is_err());
    }

    #[test]
    fn test_phase_angle_calculation() {
        let r_leo = EARTH_RADIUS + 400e3;
        let r_geo = EARTH_RADIUS + 35_786e3;

        let phase = HohmannTransfer::phase_angle(r_leo, r_geo, EARTH_MU).unwrap();

        // Phase angle should be in valid range
        assert!(phase >= 0.0);
        assert!(phase < 2.0 * PI);

        // For LEO to GEO, phase angle should be around 1.75 radians (100.4°)
        // This is the angle the target needs to be ahead of the chaser at departure
        assert_relative_eq!(phase, 1.75, epsilon = 0.01);
    }

    #[test]
    fn test_synodic_period() {
        let r1 = EARTH_RADIUS + 400e3;
        let r2 = EARTH_RADIUS + 800e3;

        let synodic = HohmannTransfer::synodic_period(r1, r2, EARTH_MU).unwrap();

        // Synodic period should be positive
        assert!(synodic > 0.0);

        // For these close orbits, synodic period should be larger than individual periods
        let t1 = 2.0 * PI * (r1.powi(3) / EARTH_MU).sqrt();
        let t2 = 2.0 * PI * (r2.powi(3) / EARTH_MU).sqrt();
        assert!(synodic > t1);
        assert!(synodic > t2);
    }

    #[test]
    fn test_time_to_transfer_window() {
        let r_leo = EARTH_RADIUS + 400e3;
        let r_geo = EARTH_RADIUS + 35_786e3;

        // Test with current phase = 0 (worst case)
        let wait_time = HohmannTransfer::time_to_transfer_window(0.0, r_leo, r_geo, EARTH_MU).unwrap();

        // Wait time should be positive and less than synodic period
        assert!(wait_time >= 0.0);
        let synodic = HohmannTransfer::synodic_period(r_leo, r_geo, EARTH_MU).unwrap();
        assert!(wait_time <= synodic);
    }

    #[test]
    fn test_energy_conservation() {
        // Verify that specific orbital energy is conserved in transfer
        let r1 = EARTH_RADIUS + 400e3;
        let r2 = EARTH_RADIUS + 10_000e3;

        let result = HohmannTransfer::calculate(r1, r2, EARTH_MU).unwrap();

        // Specific energy of transfer orbit at periapsis
        let e_peri = 0.5 * result.v_transfer_periapsis.powi(2) - EARTH_MU / r1;

        // Specific energy of transfer orbit at apoapsis
        let e_apo = 0.5 * result.v_transfer_apoapsis.powi(2) - EARTH_MU / r2;

        // Should be equal (energy conservation)
        assert_relative_eq!(e_peri, e_apo, epsilon = 1e-3);

        // Should also equal -μ/(2a)
        let e_expected = -EARTH_MU / (2.0 * result.transfer_sma);
        assert_relative_eq!(e_peri, e_expected, epsilon = 1e-3);
    }

    #[test]
    fn test_earth_mars_transfer() {
        // Interplanetary transfer test
        // Earth orbit: 1 AU, Mars orbit: 1.524 AU
        const AU: f64 = 1.495978707e11; // meters
        const SUN_MU: f64 = 1.32712440018e20; // m³/s²

        let r_earth = 1.0 * AU;
        let r_mars = 1.524 * AU;

        let result = HohmannTransfer::calculate(r_earth, r_mars, SUN_MU).unwrap();

        // Transfer time should be about 259 days
        let expected_days = 259.0;
        let actual_days = result.transfer_time / 86400.0;
        assert_relative_eq!(actual_days, expected_days, epsilon = 10.0);

        // Total delta-v should be about 5.7 km/s relative to circular orbits
        assert_relative_eq!(result.delta_v_total / 1000.0, 5.7, epsilon = 0.5);
    }
}
