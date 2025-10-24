//! Eclipse Detection and Solar Lighting Conditions
//!
//! This module provides algorithms for determining satellite eclipse conditions,
//! solar beta angle calculations, and sun-synchronous orbit analysis.
//!
//! # Overview
//!
//! Satellites in Earth orbit periodically pass through Earth's shadow, experiencing
//! eclipses that affect:
//! - **Solar panel power generation** - Zero power during umbra
//! - **Thermal conditions** - Rapid temperature changes
//! - **Attitude control** - Star tracker availability
//! - **Battery cycling** - Charge/discharge patterns
//!
//! # Shadow Regions
//!
//! Earth's shadow consists of two regions:
//! - **Umbra**: Complete shadow where the Sun is fully blocked by Earth
//! - **Penumbra**: Partial shadow where the Sun is partially blocked
//!
//! # Shadow Models
//!
//! This implementation uses the **conical shadow model** which:
//! - Treats Earth's shadow as a cone (not cylinder)
//! - Accounts for the Sun's finite angular size
//! - Provides accurate penumbra calculations
//! - Works for all orbit altitudes (LEO, MEO, GEO)
//!
//! ## Algorithm
//!
//! Following Vallado's Algorithm 34:
//!
//! 1. **Check night side**: dot(r_sat, r_sun) < 0
//! 2. **Calculate shadow angles**:
//!    - Umbra angle: α_u = arctan((R_☉ - R_⊕) / |r_sun|)
//!    - Penumbra angle: α_p = arctan((R_☉ + R_⊕) / |r_sun|)
//! 3. **Calculate satellite angle**:
//!    - θ_sat = arcsin(R_⊕ / |r_sat|)
//! 4. **Calculate shadow angle**:
//!    - θ_shadow = angle between (-r_sun) and r_sat
//! 5. **Determine state**:
//!    - Umbra: θ_shadow < α_u + θ_sat
//!    - Penumbra: θ_shadow < α_p + θ_sat
//!    - Sunlit: otherwise
//!
//! # Beta Angle
//!
//! The **solar beta angle** (β) is the angle between a satellite's orbital
//! plane and the geocentric Sun vector. It determines eclipse duration:
//!
//! - **β = 0°**: Maximum eclipse duration (orbit plane contains Sun)
//! - **β = 90°**: No eclipses (orbit plane perpendicular to Sun)
//! - **|β| > ~70°**: Continuous sunlight for typical LEO satellites
//!
//! Formula:
//! ```text
//! β = arcsin[sin(i)·cos(Ω - λ_☉)]
//! ```
//! where:
//! - i = orbital inclination
//! - Ω = right ascension of ascending node (RAAN)
//! - λ_☉ = ecliptic longitude of Sun
//!
//! For more precise calculations (Vallado):
//! ```text
//! β = arcsin[cos(Γ)·sin(Ω)·sin(i) - sin(Γ)·cos(ε)·cos(Ω)·sin(i) + sin(Γ)·sin(ε)·cos(i)]
//! ```
//! where:
//! - Γ = ecliptic true solar longitude
//! - ε = obliquity of ecliptic (≈ 23.45°)
//!
//! # Sun-Synchronous Orbits
//!
//! A **sun-synchronous orbit** maintains a constant angle between the orbital
//! plane and the Sun direction by matching the orbital precession rate to
//! Earth's orbital rate around the Sun (≈0.9856°/day).
//!
//! Required RAAN rate:
//! ```text
//! dΩ/dt = 0.9856° / day
//! ```
//!
//! For a circular orbit with J2 perturbations:
//! ```text
//! dΩ/dt = -3/2 · (R_⊕/a)² · J2 · n · cos(i)
//! ```
//!
//! Solving for inclination:
//! ```text
//! i = arccos(-dΩ/dt / (3/2 · (R_⊕/a)² · J2 · n))
//! ```
//!
//! # References
//!
//! - **Vallado, D. A.** (2013). Fundamentals of Astrodynamics and Applications (4th ed.).
//!   Algorithm 34: Eclipse shadow calculation
//! - **Curtis, H. D.** (2013). Orbital Mechanics for Engineering Students (3rd ed.).
//!   Chapter 3: Orbital elements, Section 12: Eclipse prediction
//! - **Wertz, J. R., & Larson, W. J.** (1999). Space Mission Analysis and Design (3rd ed.).
//!   Chapter 5: Spacecraft thermal control
//! - <https://en.wikipedia.org/wiki/Beta_angle>
//! - <https://en.wikipedia.org/wiki/Sun-synchronous_orbit>

use nalgebra::Vector3;
use std::f64::consts::PI;

use crate::core::PoliastroResult;

/// Eclipse state for a satellite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EclipseState {
    /// Satellite is in full sunlight
    Sunlit,
    /// Satellite is in partial shadow (penumbra)
    Penumbra,
    /// Satellite is in full shadow (umbra)
    Umbra,
}

/// Constants for eclipse calculations
pub mod eclipse_constants {
    /// Mean radius of the Sun (m)
    pub const SUN_RADIUS: f64 = 695_700_000.0;

    /// Mean Earth-Sun distance (1 AU in meters)
    pub const EARTH_SUN_DISTANCE: f64 = 1.496e11;

    /// Earth's mean equatorial radius (m)
    pub const EARTH_RADIUS: f64 = 6_378_137.0;

    /// Obliquity of the ecliptic (Earth's axial tilt) in radians
    /// ε ≈ 23.4392811° (J2000 epoch)
    pub const OBLIQUITY_ECLIPTIC: f64 = 0.409_092_804_222; // 23.4392811° in radians

    /// Earth's orbital angular rate around the Sun (rad/s)
    /// ≈ 0.9856°/day = 1.991e-7 rad/s
    pub const EARTH_ORBITAL_RATE: f64 = 1.991e-7;
}

/// Determine eclipse state for a satellite position
///
/// Uses the conical shadow model (Vallado Algorithm 34) to determine whether
/// a satellite is in sunlight, penumbra, or umbra.
///
/// # Arguments
///
/// * `r_sat` - Satellite position vector in ECI frame (m)
/// * `r_sun` - Sun position vector from Earth in ECI frame (m)
///
/// # Returns
///
/// The eclipse state (Sunlit, Penumbra, or Umbra)
///
/// # Algorithm
///
/// 1. Check if satellite is on night side: dot(r_sat, r_sun) < 0
/// 2. Calculate umbra and penumbra angles
/// 3. Calculate satellite's angular radius from Earth
/// 4. Determine shadow state based on angular comparisons
///
/// # Example
///
/// ```rust,ignore
/// use nalgebra::Vector3;
/// use astrora_core::satellite::eclipse::compute_eclipse_state;
///
/// // Satellite at 400 km altitude (LEO)
/// let r_sat = Vector3::new(6778e3, 0.0, 0.0);
/// // Sun at 1 AU
/// let r_sun = Vector3::new(1.496e11, 0.0, 0.0);
///
/// let state = compute_eclipse_state(&r_sat, &r_sun);
/// // state will be Sunlit (satellite and sun on same side)
/// ```
pub fn compute_eclipse_state(r_sat: &Vector3<f64>, r_sun: &Vector3<f64>) -> EclipseState {
    use eclipse_constants::*;

    // Step 1: Check if satellite is on night side of Earth
    // If dot product is positive, satellite and Sun are on same side of Earth
    let dot_product = r_sat.dot(r_sun);
    if dot_product > 0.0 {
        return EclipseState::Sunlit;
    }

    // Step 2: Calculate shadow angles
    let sun_distance = r_sun.magnitude();

    // Umbra angle: half-angle of the umbral cone
    // α_u = arctan((R_sun - R_earth) / d_sun)
    let alpha_umbra = ((SUN_RADIUS - EARTH_RADIUS) / sun_distance).atan();

    // Penumbra angle: half-angle of the penumbral cone
    // α_p = arctan((R_sun + R_earth) / d_sun)
    let alpha_penumbra = ((SUN_RADIUS + EARTH_RADIUS) / sun_distance).atan();

    // Step 3: Calculate satellite angular radius from Earth
    let sat_distance = r_sat.magnitude();
    let sat_angle = (EARTH_RADIUS / sat_distance).asin();

    // Step 4: Calculate angle between satellite and shadow axis
    // Shadow axis points opposite to Sun direction
    let shadow_axis = -r_sun.normalize();
    let sat_direction = r_sat.normalize();

    // Angle between satellite direction and shadow axis
    let cos_theta = shadow_axis.dot(&sat_direction).clamp(-1.0, 1.0);
    let theta_shadow = cos_theta.acos();

    // Step 5: Determine eclipse state
    // Satellite is in umbra if its angular separation from shadow axis
    // is less than (umbra angle + satellite angular size)
    if theta_shadow < alpha_umbra + sat_angle {
        EclipseState::Umbra
    } else if theta_shadow < alpha_penumbra + sat_angle {
        EclipseState::Penumbra
    } else {
        EclipseState::Sunlit
    }
}

/// Calculate solar beta angle for a satellite orbit
///
/// The beta angle (β) is the angle between the orbital plane and the Sun vector.
/// It determines eclipse duration and thermal conditions.
///
/// # Arguments
///
/// * `inclination` - Orbital inclination (radians)
/// * `raan` - Right Ascension of Ascending Node (radians)
/// * `solar_longitude` - Ecliptic longitude of the Sun (radians)
///
/// # Returns
///
/// Beta angle in radians, range [-π/2, +π/2]
///
/// # Formula
///
/// Simplified:
/// ```text
/// β = arcsin[sin(i)·sin(Ω - λ_☉)]
/// ```
///
/// For small beta angles and circular orbits, this approximation is sufficient.
///
/// # Example
///
/// ```rust,ignore
/// use astrora_core::satellite::eclipse::solar_beta_angle;
/// use std::f64::consts::PI;
///
/// // ISS-like orbit: 51.6° inclination, RAAN = 90°
/// let i = 51.6_f64.to_radians();
/// let raan = (90.0_f64).to_radians();
/// let solar_lon = 0.0; // Sun at vernal equinox
///
/// let beta = solar_beta_angle(i, raan, solar_lon);
/// println!("Beta angle: {:.2}°", beta.to_degrees());
/// ```
pub fn solar_beta_angle(
    inclination: f64,
    raan: f64,
    solar_longitude: f64,
) -> f64 {
    // β = arcsin[sin(i)·sin(Ω - λ_☉)]
    let sin_beta = inclination.sin() * (raan - solar_longitude).sin();
    sin_beta.clamp(-1.0, 1.0).asin()
}

/// Calculate precise solar beta angle using full Vallado formula
///
/// This is more accurate than the simplified version, accounting for
/// Earth's axial tilt (obliquity of ecliptic).
///
/// # Arguments
///
/// * `inclination` - Orbital inclination (radians)
/// * `raan` - Right Ascension of Ascending Node (radians)
/// * `solar_longitude` - True ecliptic longitude of Sun (radians, Γ)
///
/// # Returns
///
/// Beta angle in radians, range [-π/2, +π/2]
///
/// # Formula (Vallado)
///
/// ```text
/// β = arcsin[cos(Γ)·sin(Ω)·sin(i) - sin(Γ)·cos(ε)·cos(Ω)·sin(i) + sin(Γ)·sin(ε)·cos(i)]
/// ```
/// where ε ≈ 23.44° is the obliquity of the ecliptic
///
/// # Example
///
/// ```rust,ignore
/// use astrora_core::satellite::eclipse::solar_beta_angle_precise;
///
/// let i = 51.6_f64.to_radians();
/// let raan = (90.0_f64).to_radians();
/// let solar_lon = 0.0;
///
/// let beta = solar_beta_angle_precise(i, raan, solar_lon);
/// ```
pub fn solar_beta_angle_precise(
    inclination: f64,
    raan: f64,
    solar_longitude: f64,
) -> f64 {
    use eclipse_constants::OBLIQUITY_ECLIPTIC;

    let eps = OBLIQUITY_ECLIPTIC;
    let gamma = solar_longitude;
    let omega = raan;
    let i = inclination;

    // β = arcsin[cos(Γ)·sin(Ω)·sin(i) - sin(Γ)·cos(ε)·cos(Ω)·sin(i) + sin(Γ)·sin(ε)·cos(i)]
    let term1 = gamma.cos() * omega.sin() * i.sin();
    let term2 = -gamma.sin() * eps.cos() * omega.cos() * i.sin();
    let term3 = gamma.sin() * eps.sin() * i.cos();

    let sin_beta = term1 + term2 + term3;
    sin_beta.clamp(-1.0, 1.0).asin()
}

/// Calculate required inclination for a sun-synchronous orbit
///
/// A sun-synchronous orbit precesses at the same rate as Earth orbits the Sun,
/// maintaining a constant angle between the orbital plane and Sun direction.
///
/// # Arguments
///
/// * `semi_major_axis` - Semi-major axis of the orbit (m)
/// * `eccentricity` - Orbital eccentricity (0 for circular)
/// * `j2` - Earth's J2 zonal harmonic coefficient (default: 1.08263e-3)
/// * `earth_radius` - Earth's equatorial radius (m, default: 6378137.0)
/// * `mu` - Earth's gravitational parameter (m³/s², default: 3.986004418e14)
///
/// # Returns
///
/// Required inclination in radians for sun-synchronous orbit
///
/// # Formula
///
/// Required RAAN rate for sun-synchronous:
/// ```text
/// dΩ/dt = 0.9856°/day ≈ 1.991e-7 rad/s
/// ```
///
/// J2 perturbation RAAN rate:
/// ```text
/// dΩ/dt = -3/2 · (R_e/a)² · J2 · n · cos(i)
/// ```
///
/// Where n is the mean motion:
/// ```text
/// n = sqrt(μ/a³)
/// ```
///
/// Solving for i:
/// ```text
/// i = arccos(-dΩ/dt / (3/2 · (R_e/a)² · J2 · n))
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use astrora_core::satellite::eclipse::sun_synchronous_inclination;
/// use astrora_core::core::constants::{EARTH_MU, EARTH_RADIUS, EARTH_J2};
///
/// // Typical sun-sync orbit at 600 km
/// let altitude = 600e3; // m
/// let a = EARTH_RADIUS + altitude;
///
/// let inclination = sun_synchronous_inclination(a, 0.0, EARTH_J2, EARTH_RADIUS, EARTH_MU)?;
/// println!("Sun-sync inclination: {:.2}°", inclination.to_degrees());
/// // Output: ~97.8° (retrograde orbit)
/// ```
pub fn sun_synchronous_inclination(
    semi_major_axis: f64,
    eccentricity: f64,
    j2: f64,
    earth_radius: f64,
    mu: f64,
) -> PoliastroResult<f64> {
    use crate::core::PoliastroError;
    use eclipse_constants::EARTH_ORBITAL_RATE;

    // Required RAAN rate for sun-synchronous orbit (rad/s)
    let required_raan_rate = EARTH_ORBITAL_RATE;

    // Mean motion: n = sqrt(μ/a³)
    let n = (mu / semi_major_axis.powi(3)).sqrt();

    // For near-circular orbits, we can use the simplified formula
    // dΩ/dt = -3/2 · (R_e/a)² · J2 · n · cos(i)
    //
    // Solving for cos(i):
    // cos(i) = -dΩ/dt / (3/2 · (R_e/a)² · J2 · n)

    let factor = 1.5 * (earth_radius / semi_major_axis).powi(2) * j2 * n;
    let cos_i = -required_raan_rate / factor;

    // Check if solution is valid
    if cos_i.abs() > 1.0 {
        return Err(PoliastroError::invalid_parameter(
            "semi_major_axis",
            semi_major_axis / 1000.0,
            format!(
                "Sun-synchronous orbit not possible at this altitude. \
                Required cos(i) = {cos_i:.4} is outside [-1, 1]. \
                Try altitude between 200-6000 km."
            )
        ));
    }

    // Return inclination
    Ok(cos_i.acos())
}

/// Calculate eclipse duration for a circular orbit
///
/// Estimates the maximum eclipse duration per orbit based on beta angle
/// and orbit altitude.
///
/// # Arguments
///
/// * `semi_major_axis` - Semi-major axis of the orbit (m)
/// * `beta_angle` - Solar beta angle (radians)
/// * `mu` - Earth's gravitational parameter (m³/s²)
///
/// # Returns
///
/// Eclipse duration in seconds (0 if no eclipse)
///
/// # Formula
///
/// For circular orbit, the eclipse fraction is approximately:
/// ```text
/// f_eclipse = (1/π) · arccos[sqrt(h² - R²) / (R·|sin(β)|)]
/// ```
/// where:
/// - h = semi-major axis
/// - R = Earth radius
/// - β = beta angle
///
/// Then: eclipse_duration = f_eclipse · orbital_period
///
/// For |β| > β_critical, no eclipse occurs, where:
/// ```text
/// β_critical = arcsin(R/h)
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use astrora_core::satellite::eclipse::eclipse_duration;
/// use astrora_core::core::constants::{EARTH_MU, EARTH_RADIUS};
///
/// // ISS at 400 km altitude
/// let a = EARTH_RADIUS + 400e3;
/// let beta = 0.0; // Maximum eclipse
///
/// let duration = eclipse_duration(a, beta, EARTH_MU)?;
/// println!("Eclipse duration: {:.1} minutes", duration / 60.0);
/// // Output: ~37 minutes
/// ```
pub fn eclipse_duration(
    semi_major_axis: f64,
    beta_angle: f64,
    mu: f64,
) -> PoliastroResult<f64> {
    use eclipse_constants::EARTH_RADIUS;

    // Critical beta angle: above this, no eclipse occurs
    let beta_critical = (EARTH_RADIUS / semi_major_axis).asin();

    // No eclipse if |β| > β_critical
    if beta_angle.abs() > beta_critical {
        return Ok(0.0);
    }

    // Orbital period: T = 2π√(a³/μ)
    let period = 2.0 * PI * (semi_major_axis.powi(3) / mu).sqrt();

    // For circular orbits, the eclipse arc angle is:
    // θ = arccos[sqrt(a² - R²) / (a·sin(π/2 - |β|))]
    //   = arccos[sqrt(a² - R²) / (a·cos(|β|))]

    let numerator = (semi_major_axis.powi(2) - EARTH_RADIUS.powi(2)).sqrt();
    let denominator = semi_major_axis * beta_angle.abs().cos();

    if denominator < 1e-10 {
        // β very close to 90°, no eclipse
        return Ok(0.0);
    }

    let cos_half_eclipse_angle = (numerator / denominator).clamp(-1.0, 1.0);
    let half_eclipse_angle = cos_half_eclipse_angle.acos();

    // Eclipse fraction of orbit
    let eclipse_fraction = (2.0 * half_eclipse_angle) / (2.0 * PI);

    // Eclipse duration
    Ok(eclipse_fraction * period)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_sunlit_satellite_same_side_as_sun() {
        // Satellite and sun on same side of Earth
        let r_sat = Vector3::new(7000e3, 0.0, 0.0);
        let r_sun = Vector3::new(1.496e11, 0.0, 0.0);

        let state = compute_eclipse_state(&r_sat, &r_sun);
        assert_eq!(state, EclipseState::Sunlit);
    }

    #[test]
    fn test_umbra_satellite_in_shadow() {
        // Satellite directly in Earth's shadow
        // Sun at +X, satellite at -X (opposite side of Earth)
        let r_sat = Vector3::new(-7000e3, 0.0, 0.0);
        let r_sun = Vector3::new(1.496e11, 0.0, 0.0);

        let state = compute_eclipse_state(&r_sat, &r_sun);
        assert_eq!(state, EclipseState::Umbra);
    }

    #[test]
    fn test_beta_angle_zero_maximum_eclipse() {
        // Orbit plane contains the Sun (β = 0)
        // i = 90° (polar), Ω = 0°, λ_☉ = 0° (Sun at vernal equinox)
        let i = PI / 2.0;
        let raan = 0.0;
        let solar_lon = 0.0;

        let beta = solar_beta_angle(i, raan, solar_lon);
        assert_relative_eq!(beta, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_beta_angle_90_no_eclipse() {
        // Orbit plane perpendicular to Sun (β = 90°)
        // i = 90° (polar), Ω = 90°, λ_☉ = 0°
        let i = PI / 2.0;
        let raan = PI / 2.0;
        let solar_lon = 0.0;

        let beta = solar_beta_angle(i, raan, solar_lon);
        assert_relative_eq!(beta.abs(), PI / 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_sun_synchronous_inclination_typical_leo() {
        // Typical sun-sync orbit at 600 km
        use crate::core::constants::{GM_EARTH, R_EARTH, J2_EARTH};

        let altitude = 600e3; // 600 km
        let a = R_EARTH + altitude;

        let inclination = sun_synchronous_inclination(a, 0.0, J2_EARTH, R_EARTH, GM_EARTH).unwrap();

        // Should be around 97-98° (retrograde)
        assert_relative_eq!(inclination.to_degrees(), 97.8, epsilon = 0.5);
    }

    #[test]
    fn test_sun_synchronous_inclination_800km() {
        use crate::core::constants::{GM_EARTH, R_EARTH, J2_EARTH};

        let altitude = 800e3; // 800 km
        let a = R_EARTH + altitude;

        let inclination = sun_synchronous_inclination(a, 0.0, J2_EARTH, R_EARTH, GM_EARTH).unwrap();

        // Should be slightly higher than 600 km case
        assert!((97.0..99.0).contains(&inclination.to_degrees()));
    }

    #[test]
    fn test_eclipse_duration_iss_orbit() {
        use crate::core::constants::{GM_EARTH, R_EARTH};

        // ISS at 400 km altitude
        let a = R_EARTH + 400e3;
        let beta = 0.0; // Maximum eclipse (orbit plane contains Sun)

        let duration = eclipse_duration(a, beta, GM_EARTH).unwrap();

        // For ISS, maximum eclipse is about 37 minutes
        // Orbital period ~90 min, eclipse fraction ~0.41
        assert!((30.0 * 60.0..40.0 * 60.0).contains(&duration));
    }

    #[test]
    fn test_eclipse_duration_no_eclipse_high_beta() {
        use crate::core::constants::{GM_EARTH, R_EARTH};

        // ISS at 400 km altitude
        let a = R_EARTH + 400e3;
        let beta = 80.0_f64.to_radians(); // High beta angle

        let duration = eclipse_duration(a, beta, GM_EARTH).unwrap();

        // Should be very short or zero eclipse
        assert!(duration < 5.0 * 60.0); // Less than 5 minutes
    }

    #[test]
    fn test_beta_angle_precise_vs_simple() {
        // Compare precise and simple formulas
        let i = 51.6_f64.to_radians();
        let raan = (90.0_f64).to_radians();
        let solar_lon = 0.0;

        let beta_simple = solar_beta_angle(i, raan, solar_lon);
        let beta_precise = solar_beta_angle_precise(i, raan, solar_lon);

        // Should be close for moderate inclinations
        assert_relative_eq!(beta_simple, beta_precise, epsilon = 0.1);
    }
}
