//! Plane change maneuver calculations
//!
//! This module provides calculations for orbital plane change maneuvers including:
//! - Pure plane changes (inclination changes only)
//! - Combined plane changes with apside raise/lower
//! - Optimal maneuver location determination
//!
//! # Theory
//!
//! Plane change maneuvers modify the orientation of the orbital plane by changing
//! the inclination, right ascension of ascending node (RAAN), or both. These are
//! among the most expensive maneuvers in terms of ΔV requirements.
//!
//! ## Cost Characteristics
//!
//! Plane changes are very expensive:
//! - A 60° plane change requires ΔV equal to the spacecraft's current velocity
//! - Cost is proportional to velocity, so maneuvers are best performed at:
//!   - Apoapsis (slowest point) for elliptical orbits
//!   - Higher altitudes for circular orbit transfers
//! - Combined with other maneuvers (e.g., Hohmann transfers) when possible
//!
//! # Pure Plane Change
//!
//! For a simple plane change with no change in orbit size:
//!
//! ```text
//! Δv = 2v·sin(δ/2)
//! ```
//!
//! where:
//! - v is the spacecraft velocity
//! - δ is the dihedral angle (angle between orbital planes)
//!
//! This is the minimum ΔV for a pure rotational plane change.
//!
//! # Combined Plane Change
//!
//! When combining plane changes with orbit changes, the general formula is:
//!
//! ```text
//! Δv² = (v_r2 - v_r1)² + v_⊥2² + v_⊥1² - 2v_⊥1·v_⊥2·cos(δ)
//! ```
//!
//! At apoapsis (v_r = 0), this simplifies to:
//!
//! ```text
//! Δv = √(v1² + v2² - 2v1·v2·cos(δ))
//! ```
//!
//! This is the law of cosines applied to the velocity triangle.
//!
//! # Optimal Maneuver Location
//!
//! For elliptical orbits:
//! - Perform plane changes at apoapsis (lowest velocity)
//! - Can save 20-50% ΔV compared to periapsis maneuvers
//!
//! For transfers between circular orbits:
//! - Split plane change between transfer orbit entry and exit
//! - Optimal split: typically 2-3° at low altitude, remainder at high altitude
//! - Can save 1-5% compared to single-burn at destination
//!
//! # References
//! - Curtis, H. D. (2013). Orbital Mechanics for Engineering Students. Ch. 6.5
//! - Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications. Ch. 6.5
//! - <https://orbital-mechanics.space/orbital-maneuvers/plane-change-maneuvers.html>
//! - <https://en.wikipedia.org/wiki/Orbital_inclination_change>

use std::f64::consts::PI;

use crate::core::{PoliastroError, PoliastroResult};

/// Result of a pure plane change calculation
///
/// Contains all relevant parameters for a simple plane change maneuver
/// that only changes the orbital plane orientation without changing orbit size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaneChangeResult {
    /// Velocity before maneuver (m/s)
    pub velocity: f64,
    /// Dihedral angle - angle between orbital planes (radians)
    pub delta_angle: f64,
    /// Required delta-v for the plane change (m/s)
    pub delta_v: f64,
}

/// Result of a combined plane change with orbit change
///
/// Contains all relevant parameters for a maneuver that simultaneously
/// changes both the orbital plane and orbit size/shape.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CombinedPlaneChangeResult {
    /// Initial velocity (m/s)
    pub v_initial: f64,
    /// Final velocity (m/s)
    pub v_final: f64,
    /// Plane change angle (radians)
    pub delta_angle: f64,
    /// Required delta-v (m/s)
    pub delta_v: f64,
    /// ΔV for orbit change only (no plane change) (m/s)
    pub delta_v_orbit_only: f64,
    /// ΔV for plane change only (no orbit change) (m/s)
    pub delta_v_plane_only: f64,
    /// Extra ΔV cost compared to orbit change alone (m/s)
    pub delta_v_penalty: f64,
}

/// Result of optimal plane change location calculation
///
/// For combined transfers with plane changes, determines the optimal
/// way to split the plane change between different maneuver points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptimalPlaneChangeResult {
    /// Total plane change required (radians)
    pub total_angle: f64,
    /// Plane change at first maneuver point (radians)
    pub angle_at_first: f64,
    /// Plane change at second maneuver point (radians)
    pub angle_at_second: f64,
    /// Velocity at first maneuver point (m/s)
    pub v_first: f64,
    /// Velocity at second maneuver point (m/s)
    pub v_second: f64,
    /// Total delta-v with optimal split (m/s)
    pub delta_v_total: f64,
    /// ΔV at first maneuver (m/s)
    pub delta_v_first: f64,
    /// ΔV at second maneuver (m/s)
    pub delta_v_second: f64,
    /// ΔV saved compared to single burn at high altitude (m/s)
    pub delta_v_saved: f64,
    /// ΔV saved compared to single burn at low altitude (m/s)
    pub delta_v_saved_vs_low: f64,
}

/// Plane change maneuver calculator
///
/// Provides methods for calculating various types of plane change maneuvers.
pub struct PlaneChange;

impl PlaneChange {
    /// Calculate a pure plane change maneuver
    ///
    /// Computes the ΔV required to change the orbital plane by a given angle
    /// without changing the orbit size or shape. This is the most basic
    /// plane change maneuver.
    ///
    /// # Arguments
    /// * `velocity` - Current orbital velocity (m/s)
    /// * `delta_angle` - Dihedral angle to change (radians)
    ///
    /// # Returns
    /// A `PlaneChangeResult` containing the maneuver parameters
    ///
    /// # Example
    /// ```
    /// use astrora::maneuvers::planechange::PlaneChange;
    ///
    /// // LEO satellite at 7.8 km/s changing inclination by 5°
    /// let result = PlaneChange::pure_plane_change(7800.0, 5.0_f64.to_radians()).unwrap();
    /// println!("ΔV required: {:.1} m/s", result.delta_v);
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// - velocity is not positive
    /// - delta_angle is not in range [0, π]
    pub fn pure_plane_change(velocity: f64, delta_angle: f64) -> PoliastroResult<PlaneChangeResult> {
        // Validate inputs
        if velocity <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "velocity",
                velocity,
                "must be positive",
            ));
        }
        if !(0.0..=PI).contains(&delta_angle) {
            return Err(PoliastroError::out_of_range(
                "delta_angle",
                delta_angle,
                0.0,
                PI,
            ));
        }

        // Calculate ΔV using: Δv = 2v·sin(δ/2)
        let delta_v = 2.0 * velocity * (delta_angle / 2.0).sin();

        Ok(PlaneChangeResult {
            velocity,
            delta_angle,
            delta_v,
        })
    }

    /// Calculate a combined plane change with orbit change
    ///
    /// Computes the ΔV required for a maneuver that simultaneously changes
    /// both the orbital plane and the orbit velocity. This is typically more
    /// efficient than performing separate maneuvers.
    ///
    /// This uses the simplified formula valid at apoapsis (or when radial
    /// velocity is zero):
    ///
    /// Δv = √(v1² + v2² - 2v1·v2·cos(δ))
    ///
    /// # Arguments
    /// * `v_initial` - Initial orbital velocity (m/s)
    /// * `v_final` - Final orbital velocity (m/s)
    /// * `delta_angle` - Plane change angle (radians)
    ///
    /// # Returns
    /// A `CombinedPlaneChangeResult` containing all maneuver parameters
    ///
    /// # Example
    /// ```
    /// use astrora::maneuvers::planechange::PlaneChange;
    ///
    /// // Combined Hohmann transfer + plane change
    /// // From LEO (7.8 km/s) to transfer orbit (10.0 km/s) with 5° plane change
    /// let result = PlaneChange::combined_plane_change(
    ///     7800.0, 10000.0, 5.0_f64.to_radians()
    /// ).unwrap();
    /// println!("Total ΔV: {:.1} m/s", result.delta_v);
    /// println!("Penalty for plane change: {:.1} m/s", result.delta_v_penalty);
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// - velocities are not positive
    /// - delta_angle is not in range [0, π]
    pub fn combined_plane_change(
        v_initial: f64,
        v_final: f64,
        delta_angle: f64,
    ) -> PoliastroResult<CombinedPlaneChangeResult> {
        // Validate inputs
        if v_initial <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "v_initial",
                v_initial,
                "must be positive",
            ));
        }
        if v_final <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "v_final",
                v_final,
                "must be positive",
            ));
        }
        if !(0.0..=PI).contains(&delta_angle) {
            return Err(PoliastroError::out_of_range(
                "delta_angle",
                delta_angle,
                0.0,
                PI,
            ));
        }

        // Combined ΔV using law of cosines
        let delta_v = (v_initial.powi(2) + v_final.powi(2)
            - 2.0 * v_initial * v_final * delta_angle.cos())
        .sqrt();

        // ΔV for orbit change only (coplanar)
        let delta_v_orbit_only = (v_final - v_initial).abs();

        // ΔV for plane change only (using average velocity)
        let v_avg = (v_initial + v_final) / 2.0;
        let delta_v_plane_only = 2.0 * v_avg * (delta_angle / 2.0).sin();

        // Extra cost penalty
        let delta_v_penalty = delta_v - delta_v_orbit_only;

        Ok(CombinedPlaneChangeResult {
            v_initial,
            v_final,
            delta_angle,
            delta_v,
            delta_v_orbit_only,
            delta_v_plane_only,
            delta_v_penalty,
        })
    }

    /// Calculate optimal plane change split for a transfer
    ///
    /// For transfers between circular orbits with different inclinations,
    /// determines the optimal way to split the plane change between the
    /// two burn points. This typically results in a small plane change at
    /// the low-altitude burn and the remainder at the high-altitude burn.
    ///
    /// The optimization minimizes total ΔV by finding the best split angle.
    ///
    /// # Arguments
    /// * `v_low` - Velocity at lower altitude (m/s)
    /// * `v_high` - Velocity at higher altitude (m/s)
    /// * `v_transfer_low` - Transfer orbit velocity at low altitude (m/s)
    /// * `v_transfer_high` - Transfer orbit velocity at high altitude (m/s)
    /// * `total_angle` - Total plane change required (radians)
    ///
    /// # Returns
    /// An `OptimalPlaneChangeResult` with optimal split and ΔV breakdown
    ///
    /// # Example
    /// ```
    /// use astrora::maneuvers::planechange::PlaneChange;
    ///
    /// // LEO to GEO transfer with 28.5° plane change
    /// let result = PlaneChange::optimal_plane_change_location(
    ///     7800.0,  // LEO velocity
    ///     3074.0,  // GEO velocity
    ///     10200.0, // Transfer velocity at LEO
    ///     1470.0,  // Transfer velocity at GEO
    ///     28.5_f64.to_radians()
    /// ).unwrap();
    /// println!("Split: {:.1}° at LEO, {:.1}° at GEO",
    ///     result.angle_at_first.to_degrees(),
    ///     result.angle_at_second.to_degrees());
    /// println!("ΔV saved: {:.1} m/s", result.delta_v_saved);
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// - velocities are not positive
    /// - total_angle is not in range [0, π]
    pub fn optimal_plane_change_location(
        v_low: f64,
        v_high: f64,
        v_transfer_low: f64,
        v_transfer_high: f64,
        total_angle: f64,
    ) -> PoliastroResult<OptimalPlaneChangeResult> {
        // Validate inputs
        if v_low <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "v_low",
                v_low,
                "must be positive",
            ));
        }
        if v_high <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "v_high",
                v_high,
                "must be positive",
            ));
        }
        if v_transfer_low <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "v_transfer_low",
                v_transfer_low,
                "must be positive",
            ));
        }
        if v_transfer_high <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "v_transfer_high",
                v_transfer_high,
                "must be positive",
            ));
        }
        if !(0.0..=PI).contains(&total_angle) {
            return Err(PoliastroError::out_of_range(
                "total_angle",
                total_angle,
                0.0,
                PI,
            ));
        }

        // Calculate ΔV for all plane change at low altitude (worst case)
        let dv_low_all = Self::combined_plane_change(v_low, v_transfer_low, total_angle)?;
        let dv_high_coplanar =
            Self::combined_plane_change(v_transfer_high, v_high, 0.0)?;
        let delta_v_all_low = dv_low_all.delta_v + dv_high_coplanar.delta_v;

        // Calculate ΔV for all plane change at high altitude (better)
        let dv_low_coplanar = Self::combined_plane_change(v_low, v_transfer_low, 0.0)?;
        let dv_high_all =
            Self::combined_plane_change(v_transfer_high, v_high, total_angle)?;
        let delta_v_all_high = dv_low_coplanar.delta_v + dv_high_all.delta_v;

        // Search for optimal split using golden section search
        // We'll use a simpler grid search for robustness
        let n_points = 100;
        let mut best_angle_low = 0.0;
        let mut best_delta_v = delta_v_all_high;

        for i in 0..=n_points {
            let angle_low = (i as f64 / n_points as f64) * total_angle;
            let angle_high = total_angle - angle_low;

            // Calculate ΔV for this split
            let dv_low = Self::combined_plane_change(v_low, v_transfer_low, angle_low)?;
            let dv_high =
                Self::combined_plane_change(v_transfer_high, v_high, angle_high)?;
            let total_dv = dv_low.delta_v + dv_high.delta_v;

            if total_dv < best_delta_v {
                best_delta_v = total_dv;
                best_angle_low = angle_low;
            }
        }

        let best_angle_high = total_angle - best_angle_low;

        // Calculate final results with optimal split
        let dv_first = Self::combined_plane_change(v_low, v_transfer_low, best_angle_low)?;
        let dv_second =
            Self::combined_plane_change(v_transfer_high, v_high, best_angle_high)?;

        Ok(OptimalPlaneChangeResult {
            total_angle,
            angle_at_first: best_angle_low,
            angle_at_second: best_angle_high,
            v_first: v_low,
            v_second: v_high,
            delta_v_total: best_delta_v,
            delta_v_first: dv_first.delta_v,
            delta_v_second: dv_second.delta_v,
            delta_v_saved: delta_v_all_high - best_delta_v,
            delta_v_saved_vs_low: delta_v_all_low - best_delta_v,
        })
    }

    /// Calculate plane change penalty for a given velocity and angle
    ///
    /// Helper function that returns just the ΔV penalty incurred by adding
    /// a plane change to an existing maneuver.
    ///
    /// # Arguments
    /// * `velocity` - Orbital velocity (m/s)
    /// * `delta_angle` - Plane change angle (radians)
    ///
    /// # Returns
    /// Additional ΔV cost beyond coplanar maneuver (m/s)
    pub fn plane_change_penalty(velocity: f64, delta_angle: f64) -> PoliastroResult<f64> {
        if velocity <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "velocity",
                velocity,
                "must be positive",
            ));
        }
        if !(0.0..=PI).contains(&delta_angle) {
            return Err(PoliastroError::out_of_range(
                "delta_angle",
                delta_angle,
                0.0,
                PI,
            ));
        }

        // For a pure plane change, all ΔV is penalty
        let penalty = 2.0 * velocity * (delta_angle / 2.0).sin();
        Ok(penalty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // Earth's standard gravitational parameter
    const MU_EARTH: f64 = 3.986004418e14; // m³/s²

    #[test]
    fn test_pure_plane_change_basic() {
        // Test a simple 5° plane change at LEO velocity
        let v = 7800.0; // m/s (typical LEO)
        let angle = 5.0_f64.to_radians();

        let result = PlaneChange::pure_plane_change(v, angle).unwrap();

        // Expected: Δv = 2 * 7800 * sin(2.5°) ≈ 680 m/s
        assert_relative_eq!(result.delta_v, 680.0, epsilon = 10.0);
        assert_eq!(result.velocity, v);
        assert_eq!(result.delta_angle, angle);
    }

    #[test]
    fn test_pure_plane_change_45_degrees() {
        // Test a 45° plane change (expensive!)
        let v = 7800.0;
        let angle = 45.0_f64.to_radians();

        let result = PlaneChange::pure_plane_change(v, angle).unwrap();

        // Expected: Δv = 2 * 7800 * sin(22.5°) ≈ 5969 m/s
        assert_relative_eq!(result.delta_v, 5969.0, epsilon = 10.0);
    }

    #[test]
    fn test_pure_plane_change_180_degrees() {
        // Test a 180° plane change (orbit reversal)
        let v = 7800.0;
        let angle = PI;

        let result = PlaneChange::pure_plane_change(v, angle).unwrap();

        // Expected: Δv = 2 * 7800 * sin(90°) = 15600 m/s
        assert_relative_eq!(result.delta_v, 2.0 * v, epsilon = 0.1);
    }

    #[test]
    fn test_pure_plane_change_zero() {
        // Test zero plane change
        let v = 7800.0;
        let angle = 0.0;

        let result = PlaneChange::pure_plane_change(v, angle).unwrap();

        assert_relative_eq!(result.delta_v, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_combined_plane_change_hohmann_leo_to_geo() {
        // Test combined Hohmann transfer + plane change
        // LEO (400 km) to GEO (35,786 km) with 28.5° plane change

        let r_leo = 6378.137e3 + 400e3; // m
        let r_geo = 42164e3; // m

        let v_leo = (MU_EARTH / r_leo).sqrt();
        let a_transfer = (r_leo + r_geo) / 2.0;
        let v_transfer_leo = ((2.0 * MU_EARTH / r_leo) - (MU_EARTH / a_transfer)).sqrt();

        let angle = 28.5_f64.to_radians();

        let result =
            PlaneChange::combined_plane_change(v_leo, v_transfer_leo, angle).unwrap();

        // With 28.5° plane change, total ΔV should be significantly higher
        // than coplanar Hohmann (which would be ~2.4 km/s first burn)
        assert!(result.delta_v > result.delta_v_orbit_only);
        assert!(result.delta_v_penalty > 0.0);

        // The penalty should be substantial for 28.5°
        assert!(result.delta_v_penalty > 1000.0); // At least 1 km/s penalty
    }

    #[test]
    fn test_combined_plane_change_coplanar() {
        // Test that coplanar (0° plane change) gives simple difference
        let v1 = 7800.0;
        let v2 = 10000.0;
        let angle = 0.0;

        let result = PlaneChange::combined_plane_change(v1, v2, angle).unwrap();

        assert_relative_eq!(result.delta_v, (v2 - v1).abs(), epsilon = 0.1);
        assert_relative_eq!(result.delta_v_penalty, 0.0, epsilon = 1.0);
    }

    #[test]
    fn test_combined_plane_change_90_degrees() {
        // Test 90° plane change (perpendicular orbits)
        let v1 = 7800.0;
        let v2 = 7800.0; // Same speed, different plane
        let angle = PI / 2.0;

        let result = PlaneChange::combined_plane_change(v1, v2, angle).unwrap();

        // For perpendicular planes with same speed:
        // Δv = √(v² + v² - 2v²cos(90°)) = √(2v²) = v√2
        let expected = v1 * 2.0_f64.sqrt();
        assert_relative_eq!(result.delta_v, expected, epsilon = 0.1);
    }

    #[test]
    fn test_optimal_plane_change_location_leo_to_geo() {
        // Test optimal split for LEO to GEO with plane change
        let r_leo = 6378.137e3 + 400e3;
        let r_geo = 42164e3;

        let v_leo = (MU_EARTH / r_leo).sqrt();
        let v_geo = (MU_EARTH / r_geo).sqrt();

        let a_transfer = (r_leo + r_geo) / 2.0;
        let v_transfer_leo = ((2.0 * MU_EARTH / r_leo) - (MU_EARTH / a_transfer)).sqrt();
        let v_transfer_geo = ((2.0 * MU_EARTH / r_geo) - (MU_EARTH / a_transfer)).sqrt();

        let angle = 28.5_f64.to_radians();

        let result = PlaneChange::optimal_plane_change_location(
            v_leo,
            v_geo,
            v_transfer_leo,
            v_transfer_geo,
            angle,
        )
        .unwrap();

        // The optimal split should favor doing most at GEO (high altitude)
        assert!(result.angle_at_second > result.angle_at_first);

        // Should have some savings
        assert!(result.delta_v_saved > 0.0);
        assert!(result.delta_v_saved_vs_low > result.delta_v_saved);

        // Total should be sum of parts
        assert_relative_eq!(
            result.delta_v_total,
            result.delta_v_first + result.delta_v_second,
            epsilon = 0.1
        );
    }

    #[test]
    fn test_optimal_plane_change_small_angle() {
        // Test that small angles favor split maneuvers
        let r_leo = 6378.137e3 + 400e3;
        let r_geo = 42164e3;

        let v_leo = (MU_EARTH / r_leo).sqrt();
        let v_geo = (MU_EARTH / r_geo).sqrt();

        let a_transfer = (r_leo + r_geo) / 2.0;
        let v_transfer_leo = ((2.0 * MU_EARTH / r_leo) - (MU_EARTH / a_transfer)).sqrt();
        let v_transfer_geo = ((2.0 * MU_EARTH / r_geo) - (MU_EARTH / a_transfer)).sqrt();

        let angle = 5.0_f64.to_radians(); // Small angle

        let result = PlaneChange::optimal_plane_change_location(
            v_leo,
            v_geo,
            v_transfer_leo,
            v_transfer_geo,
            angle,
        )
        .unwrap();

        // For small angles, optimal might be a more even split
        // (documented ~2-3° at low altitude for such cases)
        assert!(result.angle_at_first > 0.0);
        assert!(result.delta_v_saved >= 0.0);
    }

    #[test]
    fn test_plane_change_penalty() {
        let v = 7800.0;
        let angle = 10.0_f64.to_radians();

        let penalty = PlaneChange::plane_change_penalty(v, angle).unwrap();

        // Should match pure plane change ΔV
        let pure = PlaneChange::pure_plane_change(v, angle).unwrap();
        assert_relative_eq!(penalty, pure.delta_v, epsilon = 0.1);
    }

    #[test]
    fn test_error_negative_velocity() {
        let result = PlaneChange::pure_plane_change(-100.0, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_invalid_angle() {
        let result = PlaneChange::pure_plane_change(7800.0, 4.0); // > π
        assert!(result.is_err());
    }

    #[test]
    fn test_error_negative_angle() {
        let result = PlaneChange::pure_plane_change(7800.0, -0.1);
        assert!(result.is_err());
    }
}
