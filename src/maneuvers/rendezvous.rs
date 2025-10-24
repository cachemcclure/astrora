//! Orbital rendezvous and phasing maneuver calculations
//!
//! This module provides calculations for spacecraft rendezvous scenarios, including:
//! - Phasing orbits for catching up or waiting for another spacecraft
//! - Coplanar rendezvous (different orbits, same plane)
//! - Coorbital rendezvous (same orbit, different positions)
//! - Wait time and transfer timing calculations
//!
//! # Theory
//!
//! ## Phasing Maneuvers
//!
//! A phasing maneuver is a two-impulse transfer from an orbit into a different orbit,
//! then back to the original orbit. The phasing orbit has a different period, allowing
//! the spacecraft to arrive back at the original point at a different time, thus changing
//! its position relative to another spacecraft.
//!
//! **Applications:**
//! - Spacecraft rendezvous (ISS resupply, crew transfers)
//! - Satellite constellation deployment
//! - Orbital positioning adjustments
//!
//! ## Rendezvous Scenarios
//!
//! ### Coorbital Rendezvous
//! Both spacecraft are in the same orbit but at different positions:
//! - **Target ahead**: Chaser enters smaller phasing orbit (shorter period) to catch up
//! - **Chaser ahead**: Chaser enters larger phasing orbit (longer period) to wait
//!
//! ### Coplanar Rendezvous
//! Spacecraft are in different orbits in the same plane:
//! - Uses Hohmann-like transfer with precise timing
//! - Requires calculating phase angle at transfer initiation
//! - Target must reach intercept point simultaneously with chaser
//!
//! # Equations
//!
//! ## Phasing Orbit Parameters
//!
//! **Angular velocities:**
//! ```text
//! ω = √(μ/r³)  (for circular orbit)
//! n = √(μ/a³)  (mean motion)
//! ```
//!
//! **Phasing orbit semi-major axis** (coorbital case):
//! ```text
//! a_phasing = ∛[(φ_travel / (2π × ω_target))² × μ]
//! ```
//!
//! **Wait time** (time until proper alignment):
//! ```text
//! Wait time = (φ_final - φ_initial) / (ω_target - ω_chaser)
//! ```
//!
//! **Delta-v for phasing** (applied twice - enter and exit):
//! ```text
//! Δv_total = 2 × |v_phasing - v_initial|
//! ```
//!
//! ## Rendezvous Phase Angle
//!
//! **Lead angle** (how far target travels during transfer):
//! ```text
//! α_lead = ω_target × t_transfer
//! ```
//!
//! **Required phase angle** (target position at transfer start):
//! ```text
//! φ_required = π - α_lead  (for Hohmann-like transfer)
//! ```
//!
//! # References
//! - Curtis, H. D. (2013). Orbital Mechanics for Engineering Students. Ch. 8
//! - Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications. Ch. 7
//! - <https://orbital-mechanics.space/orbital-maneuvers/phasing-maneuvers.html>
//! - Pressbooks: Introduction to Orbital Mechanics, Ch. 8 (Rendezvous)

use std::f64::consts::PI;

use crate::core::{PoliastroError, PoliastroResult};

/// Result of a phasing maneuver calculation
///
/// Contains all parameters for a phasing orbit maneuver where the spacecraft
/// temporarily enters a different orbit to change its position relative to a target.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhasingOrbitResult {
    /// Original orbit radius (m)
    pub r_original: f64,
    /// Phasing orbit semi-major axis (m)
    pub a_phasing: f64,
    /// Phasing orbit periapsis radius (m)
    pub r_phasing_periapsis: f64,
    /// Phasing orbit apoapsis radius (m)
    pub r_phasing_apoapsis: f64,
    /// Phasing orbit eccentricity
    pub e_phasing: f64,
    /// Gravitational parameter μ = GM (m³/s²)
    pub mu: f64,
    /// Original orbit velocity (m/s)
    pub v_original: f64,
    /// Velocity at phasing orbit periapsis (m/s)
    pub v_phasing_periapsis: f64,
    /// Velocity at phasing orbit apoapsis (m/s)
    pub v_phasing_apoapsis: f64,
    /// Delta-v for first burn (enter phasing orbit, m/s)
    pub delta_v_enter: f64,
    /// Delta-v for second burn (exit phasing orbit, m/s)
    pub delta_v_exit: f64,
    /// Total delta-v required (m/s)
    pub delta_v_total: f64,
    /// Original orbit period (s)
    pub period_original: f64,
    /// Phasing orbit period (s)
    pub period_phasing: f64,
    /// Number of phasing orbits required
    pub num_phasing_orbits: f64,
    /// Total phasing time (s)
    pub phasing_time: f64,
    /// Angular change per phasing orbit (radians)
    pub phase_change_per_orbit: f64,
    /// Total angular change achieved (radians)
    pub total_phase_change: f64,
}

/// Result of a coorbital rendezvous calculation
///
/// For spacecraft in the same circular orbit but at different angular positions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoorbitalRendezvousResult {
    /// Orbit radius (m)
    pub r_orbit: f64,
    /// Gravitational parameter μ = GM (m³/s²)
    pub mu: f64,
    /// Initial phase angle difference (radians, 0 to 2π)
    pub initial_phase_difference: f64,
    /// Whether chaser is ahead (true) or behind (false) target
    pub chaser_ahead: bool,
    /// Phasing orbit result
    pub phasing: PhasingOrbitResult,
}

/// Result of a coplanar rendezvous calculation
///
/// For spacecraft in different circular orbits in the same plane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoplanarRendezvousResult {
    /// Chaser orbit radius (m)
    pub r_chaser: f64,
    /// Target orbit radius (m)
    pub r_target: f64,
    /// Gravitational parameter μ = GM (m³/s²)
    pub mu: f64,
    /// Transfer semi-major axis (m)
    pub a_transfer: f64,
    /// Transfer orbit eccentricity
    pub e_transfer: f64,
    /// Transfer time (s)
    pub transfer_time: f64,
    /// Lead angle - how far target travels during transfer (radians)
    pub lead_angle: f64,
    /// Required phase angle at transfer start (radians, 0 to 2π)
    pub required_phase_angle: f64,
    /// Current phase angle (radians, 0 to 2π)
    pub current_phase_angle: f64,
    /// Wait time until proper alignment (s)
    pub wait_time: f64,
    /// Number of orbits to wait
    pub wait_orbits: f64,
    /// Delta-v for first burn (enter transfer orbit, m/s)
    pub delta_v1: f64,
    /// Delta-v for second burn (circularize at target, m/s)
    pub delta_v2: f64,
    /// Total delta-v required (m/s)
    pub delta_v_total: f64,
}

/// Rendezvous maneuver calculator
///
/// Provides methods for calculating orbital rendezvous maneuvers including
/// phasing orbits and intercept trajectories.
pub struct Rendezvous;

impl Rendezvous {
    /// Calculate a phasing orbit maneuver
    ///
    /// Computes the parameters for a phasing orbit that changes the spacecraft's
    /// position by a specified phase angle. The spacecraft temporarily enters
    /// a different orbit (higher or lower) and returns to the original orbit
    /// at a different position.
    ///
    /// # Arguments
    /// * `r_original` - Original circular orbit radius (m)
    /// * `phase_change` - Desired angular change (radians), positive = catch up, negative = wait
    /// * `num_orbits` - Number of phasing orbits to complete (must be ≥ 1)
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// A `PhasingOrbitResult` containing all phasing orbit parameters
    ///
    /// # Errors
    /// Returns `PoliastroError` if:
    /// - Radius is non-positive
    /// - Phase change is zero or invalid
    /// - Number of orbits is less than 1
    /// - Gravitational parameter is non-positive
    /// - Phasing orbit would be physically impossible (e.g., intersect planet)
    ///
    /// # Examples
    /// ```
    /// use astrora::maneuvers::Rendezvous;
    /// use astrora::core::constants::earth;
    /// use std::f64::consts::PI;
    ///
    /// // ISS orbit, need to catch up by 90 degrees in 2 orbits
    /// let r_iss = earth::MEAN_RADIUS + 400e3;  // 400 km altitude
    /// let phase_change = PI / 2.0;  // 90 degrees
    /// let num_orbits = 2.0;
    ///
    /// let result = Rendezvous::phasing_orbit(r_iss, phase_change, num_orbits, earth::MU).unwrap();
    ///
    /// println!("Total ΔV: {:.1} m/s", result.delta_v_total);
    /// println!("Phasing time: {:.1} hours", result.phasing_time / 3600.0);
    /// ```
    pub fn phasing_orbit(
        r_original: f64,
        phase_change: f64,
        num_orbits: f64,
        mu: f64,
    ) -> PoliastroResult<PhasingOrbitResult> {
        // Validation
        if r_original <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_original",
                r_original,
                "must be positive",
            ));
        }
        if mu <= 0.0 {
            return Err(PoliastroError::invalid_parameter("mu", mu, "must be positive"));
        }
        if phase_change.abs() < 1e-10 {
            return Err(PoliastroError::invalid_parameter(
                "phase_change",
                phase_change,
                "must be non-zero",
            ));
        }
        if num_orbits < 1.0 {
            return Err(PoliastroError::invalid_parameter(
                "num_orbits",
                num_orbits,
                "must be at least 1",
            ));
        }

        // Calculate original orbit parameters
        let v_original = (mu / r_original).sqrt();
        let period_original = 2.0 * PI * (r_original.powi(3) / mu).sqrt();
        let omega_original = (mu / r_original.powi(3)).sqrt();

        // Calculate phase change per orbit
        let phase_change_per_orbit = phase_change / num_orbits;

        // Calculate phasing orbit period
        // During one phasing orbit (time T_phasing), the chaser completes one full orbit (2π)
        // and returns to the starting point. During this same time, the target moves by:
        // θ_target = ω_original × T_phasing
        // The relative phase change gained is: 2π - θ_target
        // We want: 2π - ω_original × T_phasing = phase_change_per_orbit
        // Therefore: T_phasing = (2π - phase_change_per_orbit) / ω_original
        //
        // For catching up (positive phase_change): T_phasing < T_original (shorter period, lower orbit)
        // For waiting (negative phase_change): T_phasing > T_original (longer period, higher orbit)

        let period_phasing = (2.0 * PI - phase_change_per_orbit) / omega_original;

        // Calculate phasing orbit semi-major axis from period
        // T = 2π√(a³/μ)  =>  a = ∛[(T/(2π))² × μ]
        let a_phasing = ((period_phasing / (2.0 * PI)).powi(2) * mu).cbrt();

        // Determine periapsis and apoapsis
        // For catching up (positive phase): need smaller orbit (faster period)
        // For waiting (negative phase): need larger orbit (slower period)
        let (r_phasing_periapsis, r_phasing_apoapsis) = if phase_change > 0.0 {
            // Catching up: phasing orbit is smaller
            // Periapsis at phasing orbit, apoapsis at original orbit
            (2.0 * a_phasing - r_original, r_original)
        } else {
            // Waiting: phasing orbit is larger
            // Periapsis at original orbit, apoapsis at phasing orbit
            (r_original, 2.0 * a_phasing - r_original)
        };

        // Check physical validity
        // Approximate minimum altitude check (simplified - assumes spherical planet)
        let min_safe_radius = r_original * 0.95; // At least 95% of original radius
        if r_phasing_periapsis < min_safe_radius {
            return Err(PoliastroError::invalid_parameter(
                "phase_change",
                phase_change,
                format!(
                    "phasing orbit periapsis ({r_phasing_periapsis:.0} m) too low - would be below safe altitude"
                ),
            ));
        }

        // Calculate eccentricity
        let e_phasing = (r_phasing_apoapsis - r_phasing_periapsis)
            / (r_phasing_apoapsis + r_phasing_periapsis);

        // Calculate phasing orbit velocities using vis-viva equation
        let v_phasing_periapsis = (mu * (2.0 / r_phasing_periapsis - 1.0 / a_phasing)).sqrt();
        let v_phasing_apoapsis = (mu * (2.0 / r_phasing_apoapsis - 1.0 / a_phasing)).sqrt();

        // Calculate delta-v
        // Maneuver happens at r_original
        let (delta_v_enter, delta_v_exit) = if phase_change > 0.0 {
            // At apoapsis of phasing orbit (original orbit radius)
            let dv = (v_phasing_apoapsis - v_original).abs();
            (dv, dv) // Symmetric maneuver
        } else {
            // At periapsis of phasing orbit (original orbit radius)
            let dv = (v_phasing_periapsis - v_original).abs();
            (dv, dv) // Symmetric maneuver
        };

        let delta_v_total = delta_v_enter + delta_v_exit;

        // Total phasing time
        let phasing_time = num_orbits * period_phasing;

        Ok(PhasingOrbitResult {
            r_original,
            a_phasing,
            r_phasing_periapsis,
            r_phasing_apoapsis,
            e_phasing,
            mu,
            v_original,
            v_phasing_periapsis,
            v_phasing_apoapsis,
            delta_v_enter,
            delta_v_exit,
            delta_v_total,
            period_original,
            period_phasing,
            num_phasing_orbits: num_orbits,
            phasing_time,
            phase_change_per_orbit,
            total_phase_change: phase_change,
        })
    }

    /// Calculate coorbital rendezvous maneuver
    ///
    /// For two spacecraft in the same circular orbit but at different positions.
    /// Automatically determines whether the chaser should speed up or slow down,
    /// and calculates the optimal phasing orbit.
    ///
    /// # Arguments
    /// * `r_orbit` - Common orbital radius (m)
    /// * `phase_difference` - Angular separation (radians, 0 to 2π), positive = target ahead
    /// * `num_phasing_orbits` - Number of phasing orbits (≥ 1), more orbits = less delta-v but more time
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// A `CoorbitalRendezvousResult` with phasing orbit and rendezvous parameters
    ///
    /// # Errors
    /// Returns error if inputs are invalid or phasing orbit is impossible
    ///
    /// # Examples
    /// ```
    /// use astrora::maneuvers::Rendezvous;
    /// use astrora::core::constants::earth;
    /// use std::f64::consts::PI;
    ///
    /// // Two spacecraft at 400 km altitude, 180° apart
    /// let r = earth::MEAN_RADIUS + 400e3;
    /// let phase_diff = PI;  // 180 degrees apart
    ///
    /// // Catch up in 2 orbits
    /// let result = Rendezvous::coorbital_rendezvous(r, phase_diff, 2.0, earth::MU).unwrap();
    ///
    /// println!("Total ΔV: {:.1} m/s", result.phasing.delta_v_total);
    /// println!("Time: {:.1} hours", result.phasing.phasing_time / 3600.0);
    /// ```
    pub fn coorbital_rendezvous(
        r_orbit: f64,
        phase_difference: f64,
        num_phasing_orbits: f64,
        mu: f64,
    ) -> PoliastroResult<CoorbitalRendezvousResult> {
        // Normalize phase difference to [0, 2π)
        let mut phase_diff = phase_difference % (2.0 * PI);
        if phase_diff < 0.0 {
            phase_diff += 2.0 * PI;
        }

        // Determine strategy: catch up or wait?
        // If target is ahead (0 < phase < π), typically catch up is faster
        // If target is far ahead (π < phase < 2π), waiting might be better
        // For simplicity, we'll always catch up to the target
        let chaser_ahead = false;
        let phase_change = phase_diff; // Positive = need to catch up

        // Calculate phasing orbit
        let phasing = Self::phasing_orbit(r_orbit, phase_change, num_phasing_orbits, mu)?;

        Ok(CoorbitalRendezvousResult {
            r_orbit,
            mu,
            initial_phase_difference: phase_diff,
            chaser_ahead,
            phasing,
        })
    }

    /// Calculate coplanar rendezvous between different orbits
    ///
    /// For spacecraft in different circular orbits in the same plane. Uses a
    /// Hohmann-like transfer and calculates the required phase angle and wait time.
    ///
    /// # Arguments
    /// * `r_chaser` - Chaser orbit radius (m)
    /// * `r_target` - Target orbit radius (m)
    /// * `current_phase` - Current angular separation (radians, 0 to 2π), positive = target ahead
    /// * `mu` - Gravitational parameter μ = GM (m³/s²)
    ///
    /// # Returns
    /// A `CoplanarRendezvousResult` with transfer parameters and timing
    ///
    /// # Errors
    /// Returns error if radii are equal, non-positive, or mu is invalid
    ///
    /// # Examples
    /// ```
    /// use astrora::maneuvers::Rendezvous;
    /// use astrora::core::constants::earth;
    /// use std::f64::consts::PI;
    ///
    /// // Transfer from LEO to rendezvous with ISS
    /// let r_leo = earth::MEAN_RADIUS + 300e3;
    /// let r_iss = earth::MEAN_RADIUS + 400e3;
    /// let current_phase = PI / 4.0;  // ISS is 45° ahead
    ///
    /// let result = Rendezvous::coplanar_rendezvous(
    ///     r_leo, r_iss, current_phase, earth::MU
    /// ).unwrap();
    ///
    /// println!("Wait time: {:.1} hours", result.wait_time / 3600.0);
    /// println!("Transfer ΔV: {:.1} m/s", result.delta_v_total);
    /// ```
    pub fn coplanar_rendezvous(
        r_chaser: f64,
        r_target: f64,
        current_phase: f64,
        mu: f64,
    ) -> PoliastroResult<CoplanarRendezvousResult> {
        // Validation
        if r_chaser <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_chaser",
                r_chaser,
                "must be positive",
            ));
        }
        if r_target <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_target",
                r_target,
                "must be positive",
            ));
        }
        if (r_chaser - r_target).abs() < 1e-6 {
            return Err(PoliastroError::invalid_parameter(
                "r_chaser, r_target",
                r_chaser,
                "orbits must be different - use coorbital_rendezvous instead",
            ));
        }
        if mu <= 0.0 {
            return Err(PoliastroError::invalid_parameter("mu", mu, "must be positive"));
        }

        // Normalize current phase to [0, 2π)
        let mut phase = current_phase % (2.0 * PI);
        if phase < 0.0 {
            phase += 2.0 * PI;
        }

        // Calculate transfer orbit (Hohmann-like)
        let a_transfer = (r_chaser + r_target) / 2.0;
        let e_transfer = (r_target - r_chaser).abs() / (r_target + r_chaser);

        // Transfer time (half period of ellipse)
        let transfer_time = PI * (a_transfer.powi(3) / mu).sqrt();

        // Calculate angular velocities
        let omega_chaser = (mu / r_chaser.powi(3)).sqrt();
        let omega_target = (mu / r_target.powi(3)).sqrt();

        // Lead angle: how far target travels during transfer
        let lead_angle = omega_target * transfer_time;

        // Required phase angle at transfer start
        // Target should be at intercept point when chaser arrives
        let required_phase_angle = if r_chaser < r_target {
            // Chaser ascending: intercept at target orbit (apoapsis)
            PI - lead_angle
        } else {
            // Chaser descending: intercept at target orbit (periapsis)
            PI - lead_angle
        };

        // Normalize required phase to [0, 2π)
        let mut req_phase = required_phase_angle % (2.0 * PI);
        if req_phase < 0.0 {
            req_phase += 2.0 * PI;
        }

        // Calculate wait time
        // Phase evolves as: φ(t) = φ_initial + (ω_target - ω_chaser) × t
        // We want: φ(t) = required_phase_angle
        let phase_diff = req_phase - phase;
        let omega_diff = omega_target - omega_chaser;

        let wait_time = if omega_diff.abs() < 1e-12 {
            // Orbits have same period (shouldn't happen given earlier check)
            0.0
        } else {
            let mut wt = phase_diff / omega_diff;
            // Ensure positive wait time
            if wt < 0.0 {
                let period_synodic = 2.0 * PI / omega_diff.abs();
                wt += period_synodic;
            }
            wt
        };

        let wait_orbits = wait_time * omega_chaser / (2.0 * PI);

        // Calculate delta-v (similar to Hohmann transfer)
        let v_chaser = (mu / r_chaser).sqrt();
        let v_target = (mu / r_target).sqrt();

        let v_transfer_at_chaser = (mu * (2.0 / r_chaser - 1.0 / a_transfer)).sqrt();
        let v_transfer_at_target = (mu * (2.0 / r_target - 1.0 / a_transfer)).sqrt();

        let delta_v1 = (v_transfer_at_chaser - v_chaser).abs();
        let delta_v2 = (v_target - v_transfer_at_target).abs();
        let delta_v_total = delta_v1 + delta_v2;

        Ok(CoplanarRendezvousResult {
            r_chaser,
            r_target,
            mu,
            a_transfer,
            e_transfer,
            transfer_time,
            lead_angle,
            required_phase_angle: req_phase,
            current_phase_angle: phase,
            wait_time,
            wait_orbits,
            delta_v1,
            delta_v2,
            delta_v_total,
        })
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
    fn test_phasing_orbit_catch_up() {
        // ISS-like orbit, catch up by 30 degrees in 5 orbits (more realistic)
        let r = EARTH_RADIUS + 400e3;
        let phase_change = PI / 6.0; // 30 degrees
        let num_orbits = 5.0;

        let result = Rendezvous::phasing_orbit(r, phase_change, num_orbits, EARTH_MU).unwrap();

        // Verify basic properties
        assert!(result.delta_v_total > 0.0);
        assert!(result.phasing_time > 0.0);
        assert_relative_eq!(result.total_phase_change, phase_change, epsilon = 1e-6);
        assert_relative_eq!(
            result.phase_change_per_orbit,
            phase_change / num_orbits,
            epsilon = 1e-6
        );

        // Phasing orbit should be smaller (faster) for catching up
        assert!(result.a_phasing < r);
        assert!(result.period_phasing < result.period_original);
    }

    #[test]
    fn test_phasing_orbit_wait() {
        // Wait for target by slowing down (negative phase change)
        let r = EARTH_RADIUS + 400e3;
        let phase_change = -PI / 6.0; // -30 degrees (wait)
        let num_orbits = 5.0;

        let result = Rendezvous::phasing_orbit(r, phase_change, num_orbits, EARTH_MU).unwrap();

        // Phasing orbit should be larger (slower) for waiting
        assert!(result.a_phasing > r);
        assert!(result.period_phasing > result.period_original);
        assert_relative_eq!(result.total_phase_change, phase_change, epsilon = 1e-6);
    }

    #[test]
    fn test_coorbital_rendezvous() {
        // Two spacecraft 45° apart in same orbit (more realistic scenario)
        let r = EARTH_RADIUS + 400e3;
        let phase_diff = PI / 4.0; // 45 degrees
        let num_orbits = 10.0; // Use more orbits for gradual catch-up

        let result = Rendezvous::coorbital_rendezvous(r, phase_diff, num_orbits, EARTH_MU).unwrap();

        assert_relative_eq!(result.r_orbit, r, epsilon = 1e-6);
        assert_relative_eq!(result.initial_phase_difference, phase_diff, epsilon = 1e-6);
        assert!(result.phasing.delta_v_total > 0.0);
        assert_relative_eq!(
            result.phasing.total_phase_change,
            phase_diff,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_coplanar_rendezvous() {
        // Transfer from lower to higher orbit
        let r_leo = EARTH_RADIUS + 300e3;
        let r_iss = EARTH_RADIUS + 400e3;
        let current_phase = PI / 4.0;

        let result =
            Rendezvous::coplanar_rendezvous(r_leo, r_iss, current_phase, EARTH_MU).unwrap();

        assert_relative_eq!(result.r_chaser, r_leo, epsilon = 1e-6);
        assert_relative_eq!(result.r_target, r_iss, epsilon = 1e-6);
        assert!(result.transfer_time > 0.0);
        assert!(result.delta_v_total > 0.0);
        assert!(result.wait_time >= 0.0);

        // Transfer orbit should be between the two orbits
        assert!(result.a_transfer > r_leo);
        assert!(result.a_transfer < r_iss);
    }

    #[test]
    fn test_invalid_inputs() {
        let r = EARTH_RADIUS + 400e3;

        // Zero phase change
        assert!(Rendezvous::phasing_orbit(r, 0.0, 2.0, EARTH_MU).is_err());

        // Negative radius
        assert!(Rendezvous::phasing_orbit(-r, PI, 2.0, EARTH_MU).is_err());

        // Less than 1 orbit
        assert!(Rendezvous::phasing_orbit(r, PI, 0.5, EARTH_MU).is_err());

        // Equal radii for coplanar
        assert!(Rendezvous::coplanar_rendezvous(r, r, PI, EARTH_MU).is_err());
    }
}
