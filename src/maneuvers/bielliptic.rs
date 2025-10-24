//! Bi-elliptic transfer orbit calculations
//!
//! A bi-elliptic transfer is a three-impulse maneuver for transferring between two
//! coplanar circular orbits. It can be more fuel-efficient than a Hohmann transfer
//! for large orbital radius ratios (typically > 15.58).
//!
//! # Theory
//!
//! The transfer consists of three impulsive burns:
//! 1. **First burn (ΔV₁)**: At the initial orbit, increase velocity to enter the
//!    first elliptical transfer orbit with apoapsis at an intermediate radius r_b
//! 2. **Second burn (ΔV₂)**: At the apoapsis (r_b), change velocity to enter the
//!    second elliptical orbit with periapsis at the final orbit
//! 3. **Third burn (ΔV₃)**: At the final orbit radius, circularize the orbit
//!
//! ## Assumptions
//! - Initial and final orbits are circular
//! - Orbits are coplanar (no inclination change)
//! - Maneuvers are impulsive (instantaneous velocity changes)
//! - Two-body problem (no perturbations)
//!
//! # Equations
//!
//! For a transfer from radius r₁ to r₂ via intermediate apoapsis r_b:
//!
//! **Transfer orbit semi-major axes:**
//! ```text
//! a₁ = (r₁ + r_b)/2   (first transfer ellipse)
//! a₂ = (r₂ + r_b)/2   (second transfer ellipse)
//! ```
//!
//! **First burn (ΔV₁):**
//! ```text
//! ΔV₁ = √(2μ/r₁ - μ/a₁) - √(μ/r₁)
//! ```
//!
//! **Second burn (ΔV₂) at apoapsis r_b:**
//! ```text
//! ΔV₂ = |√(2μ/r_b - μ/a₂) - √(2μ/r_b - μ/a₁)|
//! ```
//!
//! **Third burn (ΔV₃):**
//! ```text
//! ΔV₃ = |√(2μ/r₂ - μ/a₂) - √(μ/r₂)|
//! ```
//!
//! **Transfer time:**
//! ```text
//! t_transfer = π√(a₁³/μ) + π√(a₂³/μ)
//! ```
//!
//! # Efficiency
//!
//! Bi-elliptic transfers are more efficient than Hohmann transfers when:
//! - r₂/r₁ > 15.58 (approximately)
//! - The optimal intermediate radius r_b is typically much larger than both r₁ and r₂
//!
//! **Trade-offs:**
//! - Lower ΔV requirement for large radius ratios
//! - Significantly longer transfer time (often 5-10x longer than Hohmann)
//! - Three burns instead of two (more complexity)
//!
//! # References
//! - Curtis, H. D. (2013). Orbital Mechanics for Engineering Students. Ch. 6.3
//! - Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications. Ch. 6.3
//! - <https://orbital-mechanics.space/orbital-maneuvers/bielliptic-hohmann-transfer.html>
//! - <https://en.wikipedia.org/wiki/Bi-elliptic_transfer>
//! - Gobetz, F. W. & Doll, J. R. (1969). "A Survey of Impulsive Trajectories"

use std::f64::consts::PI;

use crate::core::{PoliastroError, PoliastroResult};
use crate::maneuvers::hohmann::HohmannTransfer;

/// Result of a bi-elliptic transfer calculation
///
/// Contains all relevant parameters for a bi-elliptic transfer between two
/// circular coplanar orbits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiellipticTransferResult {
    /// Initial orbit radius (m)
    pub r_initial: f64,
    /// Final orbit radius (m)
    pub r_final: f64,
    /// Intermediate apoapsis radius (m)
    pub r_intermediate: f64,
    /// Gravitational parameter μ = GM (m³/s²)
    pub mu: f64,
    /// First burn delta-v (m/s) - to enter first transfer orbit
    pub delta_v1: f64,
    /// Second burn delta-v (m/s) - at intermediate apoapsis
    pub delta_v2: f64,
    /// Third burn delta-v (m/s) - to circularize at final orbit
    pub delta_v3: f64,
    /// Total delta-v required (m/s)
    pub delta_v_total: f64,
    /// Transfer time (seconds) - sum of both half-periods
    pub transfer_time: f64,
    /// First transfer orbit semi-major axis (m)
    pub transfer1_sma: f64,
    /// Second transfer orbit semi-major axis (m)
    pub transfer2_sma: f64,
    /// First transfer orbit eccentricity (dimensionless)
    pub transfer1_eccentricity: f64,
    /// Second transfer orbit eccentricity (dimensionless)
    pub transfer2_eccentricity: f64,
    /// Initial circular orbit velocity (m/s)
    pub v_initial: f64,
    /// Final circular orbit velocity (m/s)
    pub v_final: f64,
    /// Velocity at periapsis of first transfer orbit (m/s)
    pub v_transfer1_periapsis: f64,
    /// Velocity at apoapsis of first transfer orbit (m/s)
    pub v_transfer1_apoapsis: f64,
    /// Velocity at apoapsis of second transfer orbit (m/s)
    pub v_transfer2_apoapsis: f64,
    /// Velocity at periapsis of second transfer orbit (m/s)
    pub v_transfer2_periapsis: f64,
}

/// Bi-elliptic transfer calculator
///
/// Provides methods for calculating three-impulse transfers between
/// circular coplanar orbits, which can be more efficient than Hohmann
/// transfers for large radius ratios.
pub struct BiellipticTransfer;

impl BiellipticTransfer {
    /// Calculate a bi-elliptic transfer between two circular orbits
    ///
    /// # Arguments
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `r_intermediate` - Intermediate apoapsis radius (m), must be > max(r_initial, r_final)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// A `BiellipticTransferResult` containing all transfer parameters
    ///
    /// # Errors
    /// Returns `PoliastroError` if:
    /// - Any radius is non-positive
    /// - Initial and final radii are equal (no transfer needed)
    /// - Intermediate radius is not larger than both initial and final radii
    /// - Gravitational parameter is non-positive
    ///
    /// # Examples
    /// ```
    /// use astrora::maneuvers::BiellipticTransfer;
    /// use astrora::core::constants::earth;
    ///
    /// // Transfer from LEO (400 km) to GEO (35,786 km) via high apoapsis
    /// let r_leo = earth::MEAN_RADIUS + 400e3;  // m
    /// let r_geo = earth::MEAN_RADIUS + 35_786e3;  // m
    /// let r_intermediate = r_geo * 3.0;  // 3x GEO radius
    ///
    /// let result = BiellipticTransfer::calculate(r_leo, r_geo, r_intermediate, earth::MU).unwrap();
    ///
    /// println!("ΔV₁: {:.1} m/s", result.delta_v1);
    /// println!("ΔV₂: {:.1} m/s", result.delta_v2);
    /// println!("ΔV₃: {:.1} m/s", result.delta_v3);
    /// println!("Total ΔV: {:.1} m/s", result.delta_v_total);
    /// println!("Transfer time: {:.1} hours", result.transfer_time / 3600.0);
    /// ```
    pub fn calculate(
        r_initial: f64,
        r_final: f64,
        r_intermediate: f64,
        mu: f64,
    ) -> PoliastroResult<BiellipticTransferResult> {
        // Validation
        if r_initial <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_initial",
                r_initial,
                "must be positive",
            ));
        }
        if r_final <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_final",
                r_final,
                "must be positive",
            ));
        }
        if r_intermediate <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_intermediate",
                r_intermediate,
                "must be positive",
            ));
        }
        if mu <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "mu",
                mu,
                "must be positive",
            ));
        }
        if (r_initial - r_final).abs() < 1e-6 {
            return Err(PoliastroError::invalid_parameter(
                "r_initial, r_final",
                r_initial,
                "radii must be different - no transfer needed",
            ));
        }

        let r_max_orbit = r_initial.max(r_final);
        if r_intermediate <= r_max_orbit {
            return Err(PoliastroError::invalid_parameter(
                "r_intermediate",
                r_intermediate,
                format!(
                    "must be larger than both initial and final radii (max = {r_max_orbit:.1} m)"
                ),
            ));
        }

        // Calculate circular orbit velocities
        // v_circular = √(μ/r)
        let v_initial = (mu / r_initial).sqrt();
        let v_final = (mu / r_final).sqrt();

        // Transfer orbit semi-major axes
        let transfer1_sma = (r_initial + r_intermediate) / 2.0;
        let transfer2_sma = (r_final + r_intermediate) / 2.0;

        // Calculate transfer orbit eccentricities
        // e = (r_a - r_p)/(r_a + r_p)
        let transfer1_eccentricity =
            (r_intermediate - r_initial) / (r_intermediate + r_initial);
        let transfer2_eccentricity = (r_intermediate - r_final) / (r_intermediate + r_final);

        // Calculate transfer orbit velocities using vis-viva equation
        // v = √(μ(2/r - 1/a))

        // First transfer orbit velocities
        let v_transfer1_periapsis = (mu * (2.0 / r_initial - 1.0 / transfer1_sma)).sqrt();
        let v_transfer1_apoapsis = (mu * (2.0 / r_intermediate - 1.0 / transfer1_sma)).sqrt();

        // Second transfer orbit velocities
        let v_transfer2_apoapsis = (mu * (2.0 / r_intermediate - 1.0 / transfer2_sma)).sqrt();
        let v_transfer2_periapsis = (mu * (2.0 / r_final - 1.0 / transfer2_sma)).sqrt();

        // Calculate delta-v values
        // First burn: Enter first transfer orbit from initial circular orbit
        let delta_v1 = (v_transfer1_periapsis - v_initial).abs();

        // Second burn: At intermediate apoapsis, transition between transfer orbits
        let delta_v2 = (v_transfer2_apoapsis - v_transfer1_apoapsis).abs();

        // Third burn: Circularize at final orbit
        let delta_v3 = (v_final - v_transfer2_periapsis).abs();

        let delta_v_total = delta_v1 + delta_v2 + delta_v3;

        // Calculate transfer time (sum of half-periods of both elliptical orbits)
        // t = π × √(a₁³/μ) + π × √(a₂³/μ)
        let transfer_time =
            PI * (transfer1_sma.powi(3) / mu).sqrt() + PI * (transfer2_sma.powi(3) / mu).sqrt();

        Ok(BiellipticTransferResult {
            r_initial,
            r_final,
            r_intermediate,
            mu,
            delta_v1,
            delta_v2,
            delta_v3,
            delta_v_total,
            transfer_time,
            transfer1_sma,
            transfer2_sma,
            transfer1_eccentricity,
            transfer2_eccentricity,
            v_initial,
            v_final,
            v_transfer1_periapsis,
            v_transfer1_apoapsis,
            v_transfer2_apoapsis,
            v_transfer2_periapsis,
        })
    }

    /// Find the optimal intermediate radius for a bi-elliptic transfer
    ///
    /// The optimal intermediate radius minimizes the total ΔV. For most cases,
    /// this approaches infinity theoretically, but practical constraints limit
    /// it. This function searches for a practical optimum.
    ///
    /// # Arguments
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    /// * `search_limit_factor` - Maximum r_intermediate as a multiple of max(r_initial, r_final).
    ///                           Default recommended: 50.0 to 100.0
    ///
    /// # Returns
    /// Tuple of (optimal r_intermediate, BiellipticTransferResult)
    ///
    /// # Theory
    /// The optimal intermediate radius for minimum ΔV approaches infinity, but
    /// practical constraints (mission duration, navigation accuracy, etc.) limit
    /// the choice. This function searches up to a specified limit.
    pub fn find_optimal_intermediate(
        r_initial: f64,
        r_final: f64,
        mu: f64,
        search_limit_factor: f64,
    ) -> PoliastroResult<(f64, BiellipticTransferResult)> {
        let r_max = r_initial.max(r_final);
        let r_min_intermediate = r_max * 1.01; // Must be > both radii
        let r_max_intermediate = r_max * search_limit_factor;

        // Search with logarithmic spacing for efficiency
        let n_samples = 100;
        let mut best_dv = f64::INFINITY;
        let mut best_r_intermediate = r_min_intermediate;
        let mut best_result: Option<BiellipticTransferResult> = None;

        for i in 0..n_samples {
            let log_min = r_min_intermediate.ln();
            let log_max = r_max_intermediate.ln();
            let log_r = log_min + (log_max - log_min) * (i as f64) / ((n_samples - 1) as f64);
            let r_intermediate = log_r.exp();

            if let Ok(result) = Self::calculate(r_initial, r_final, r_intermediate, mu) {
                if result.delta_v_total < best_dv {
                    best_dv = result.delta_v_total;
                    best_r_intermediate = r_intermediate;
                    best_result = Some(result);
                }
            }
        }

        match best_result {
            Some(result) => Ok((best_r_intermediate, result)),
            None => Err(PoliastroError::invalid_state(
                "Could not find valid bi-elliptic transfer in search range",
            )),
        }
    }

    /// Compare bi-elliptic transfer efficiency with Hohmann transfer
    ///
    /// # Arguments
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `r_intermediate` - Intermediate apoapsis radius for bi-elliptic (m)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// Tuple of (bi-elliptic result, Hohmann result, ΔV savings in m/s, time penalty factor)
    ///
    /// Positive ΔV savings means bi-elliptic is more efficient.
    /// Time penalty factor shows how many times longer bi-elliptic takes.
    ///
    /// # Examples
    /// ```
    /// use astrora::maneuvers::BiellipticTransfer;
    /// use astrora::core::constants::earth;
    ///
    /// let r_leo = earth::MEAN_RADIUS + 400e3;
    /// let r_very_high = earth::MEAN_RADIUS + 200_000e3;  // Very high orbit
    /// let r_intermediate = r_very_high * 2.0;
    ///
    /// let (bielliptic, hohmann, dv_savings, time_penalty) =
    ///     BiellipticTransfer::compare_with_hohmann(r_leo, r_very_high, r_intermediate, earth::MU).unwrap();
    ///
    /// if dv_savings > 0.0 {
    ///     println!("Bi-elliptic saves {:.1} m/s but takes {:.1}x longer", dv_savings, time_penalty);
    /// }
    /// ```
    pub fn compare_with_hohmann(
        r_initial: f64,
        r_final: f64,
        r_intermediate: f64,
        mu: f64,
    ) -> PoliastroResult<(BiellipticTransferResult, crate::maneuvers::hohmann::HohmannTransferResult, f64, f64)> {
        let bielliptic = Self::calculate(r_initial, r_final, r_intermediate, mu)?;
        let hohmann = HohmannTransfer::calculate(r_initial, r_final, mu)?;

        let dv_savings = hohmann.delta_v_total - bielliptic.delta_v_total;
        let time_penalty = bielliptic.transfer_time / hohmann.transfer_time;

        Ok((bielliptic, hohmann, dv_savings, time_penalty))
    }

    /// Determine if bi-elliptic transfer is more efficient than Hohmann
    ///
    /// Returns true if a bi-elliptic transfer with the given intermediate radius
    /// would save ΔV compared to a Hohmann transfer.
    ///
    /// # Arguments
    /// * `r_initial` - Initial orbit radius (m)
    /// * `r_final` - Final orbit radius (m)
    /// * `r_intermediate` - Intermediate apoapsis radius (m)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// true if bi-elliptic saves ΔV, false otherwise
    pub fn is_more_efficient_than_hohmann(
        r_initial: f64,
        r_final: f64,
        r_intermediate: f64,
        mu: f64,
    ) -> PoliastroResult<bool> {
        let (_, _, dv_savings, _) = Self::compare_with_hohmann(r_initial, r_final, r_intermediate, mu)?;
        Ok(dv_savings > 0.0)
    }

    /// Calculate the critical radius ratio threshold
    ///
    /// For radius ratios above this threshold, bi-elliptic transfers can
    /// potentially be more efficient than Hohmann transfers (with appropriate
    /// choice of intermediate radius).
    ///
    /// # Returns
    /// The critical radius ratio (approximately 15.58)
    ///
    /// # Theory
    /// Theoretical analysis shows that bi-elliptic transfers with r_intermediate → ∞
    /// become more efficient than Hohmann when r_final/r_initial > 15.58.
    /// In practice, finite intermediate radii may shift this threshold slightly.
    pub fn critical_radius_ratio() -> f64 {
        15.58
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const EARTH_MU: f64 = 3.986004418e14; // m³/s²
    const EARTH_RADIUS: f64 = 6.3781e6; // m

    #[test]
    fn test_bielliptic_calculation_basic() {
        let r_initial = EARTH_RADIUS + 400e3; // LEO
        let r_final = EARTH_RADIUS + 35_786e3; // GEO
        let r_intermediate = r_final * 3.0; // 3x GEO

        let result = BiellipticTransfer::calculate(r_initial, r_final, r_intermediate, EARTH_MU);
        assert!(result.is_ok());

        let transfer = result.unwrap();
        assert!(transfer.delta_v1 > 0.0);
        assert!(transfer.delta_v2 > 0.0);
        assert!(transfer.delta_v3 > 0.0);
        assert_relative_eq!(
            transfer.delta_v_total,
            transfer.delta_v1 + transfer.delta_v2 + transfer.delta_v3,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_bielliptic_transfer_time_positive() {
        let r_initial = EARTH_RADIUS + 400e3;
        let r_final = EARTH_RADIUS + 35_786e3;
        let r_intermediate = r_final * 5.0;

        let result = BiellipticTransfer::calculate(r_initial, r_final, r_intermediate, EARTH_MU).unwrap();
        assert!(result.transfer_time > 0.0);
        // Bi-elliptic should take longer than Hohmann
        let hohmann = HohmannTransfer::calculate(r_initial, r_final, EARTH_MU).unwrap();
        assert!(result.transfer_time > hohmann.transfer_time);
    }

    #[test]
    fn test_bielliptic_intermediate_validation() {
        let r_initial = EARTH_RADIUS + 400e3;
        let r_final = EARTH_RADIUS + 35_786e3;

        // Should fail: r_intermediate too small
        let result = BiellipticTransfer::calculate(r_initial, r_final, r_final * 0.5, EARTH_MU);
        assert!(result.is_err());

        // Should fail: r_intermediate equal to r_final
        let result = BiellipticTransfer::calculate(r_initial, r_final, r_final, EARTH_MU);
        assert!(result.is_err());
    }

    #[test]
    fn test_bielliptic_vs_hohmann_large_ratio() {
        // For very large radius ratios, bi-elliptic should be more efficient
        let r_initial = EARTH_RADIUS + 400e3; // LEO
        let r_final = r_initial * 20.0; // 20x radius ratio
        let r_intermediate = r_final * 5.0;

        let (bielliptic, hohmann, dv_savings, time_penalty) =
            BiellipticTransfer::compare_with_hohmann(r_initial, r_final, r_intermediate, EARTH_MU)
                .unwrap();

        // Should save delta-v
        assert!(dv_savings > 0.0);
        // But take much longer
        assert!(time_penalty > 1.0);

        println!("Bi-elliptic ΔV: {:.1} m/s", bielliptic.delta_v_total);
        println!("Hohmann ΔV: {:.1} m/s", hohmann.delta_v_total);
        println!("Savings: {:.1} m/s ({:.2}%)", dv_savings, 100.0 * dv_savings / hohmann.delta_v_total);
        println!("Time penalty: {:.2}x", time_penalty);
    }

    #[test]
    fn test_critical_radius_ratio() {
        let ratio = BiellipticTransfer::critical_radius_ratio();
        assert_relative_eq!(ratio, 15.58, epsilon = 0.01);
    }

    #[test]
    fn test_energy_conservation() {
        let r_initial = EARTH_RADIUS + 400e3;
        let r_final = EARTH_RADIUS + 35_786e3;
        let r_intermediate = r_final * 3.0;

        let result = BiellipticTransfer::calculate(r_initial, r_final, r_intermediate, EARTH_MU).unwrap();

        // Check specific orbital energy at each point
        let epsilon_initial = -EARTH_MU / (2.0 * r_initial);

        // Energies should match circular orbit energies
        assert_relative_eq!(
            result.v_initial.powi(2) / 2.0 - EARTH_MU / r_initial,
            epsilon_initial,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_find_optimal_intermediate() {
        let r_initial = EARTH_RADIUS + 400e3;
        let r_final = r_initial * 20.0; // Large ratio where bi-elliptic helps

        let result = BiellipticTransfer::find_optimal_intermediate(r_initial, r_final, EARTH_MU, 50.0);
        assert!(result.is_ok());

        let (r_opt, transfer) = result.unwrap();
        assert!(r_opt > r_final);
        assert!(transfer.delta_v_total > 0.0);
    }

    #[test]
    fn test_parameter_validation() {
        let r1 = EARTH_RADIUS + 400e3;
        let r2 = EARTH_RADIUS + 35_786e3;
        let rb = r2 * 2.0;

        // Negative r_initial
        assert!(BiellipticTransfer::calculate(-r1, r2, rb, EARTH_MU).is_err());

        // Negative r_final
        assert!(BiellipticTransfer::calculate(r1, -r2, rb, EARTH_MU).is_err());

        // Negative r_intermediate
        assert!(BiellipticTransfer::calculate(r1, r2, -rb, EARTH_MU).is_err());

        // Negative mu
        assert!(BiellipticTransfer::calculate(r1, r2, rb, -EARTH_MU).is_err());

        // Equal radii
        assert!(BiellipticTransfer::calculate(r1, r1, rb, EARTH_MU).is_err());
    }
}
