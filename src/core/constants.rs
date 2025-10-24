//! Physical and astronomical constants
//!
//! This module provides comprehensive physical and astronomical constants for astrodynamics calculations.
//! Unless otherwise specified, gravitational parameters are from JPL DE440/441 ephemeris.
//! Radii and shape parameters are from IAU Working Group on Cartographic Coordinates and Rotational Elements.
//!
//! # References
//! - JPL DE440/441: <https://ssd.jpl.nasa.gov/astro_par.html>
//! - IAU 2015 Resolution B3: <https://arxiv.org/abs/1510.07674>
//! - IAU Working Group on Cartographic Coordinates

// =============================================================================
// FUNDAMENTAL PHYSICAL CONSTANTS
// =============================================================================

/// Gravitational constant (m³ kg⁻¹ s⁻²)
///
/// CODATA 2018 recommended value.
/// Reference: <https://physics.nist.gov/cgi-bin/cuu/Value?bg>
pub const G: f64 = 6.674_30e-11;

/// Speed of light in vacuum (m/s)
///
/// Exact value by definition in SI units.
pub const C: f64 = 299_792_458.0;

/// Astronomical unit (m)
///
/// IAU 2012 Resolution B2: exact value by definition.
/// Reference: <https://www.iau.org/static/resolutions/IAU2012_English.pdf>
pub const AU: f64 = 149_597_870_700.0;

// =============================================================================
// SOLAR CONSTANTS
// =============================================================================

/// Sun's standard gravitational parameter (m³/s²)
///
/// Heliocentric gravitational constant from JPL DE440.
/// Value: 1.32712440041279419 × 10²⁰ m³/s²
pub const GM_SUN: f64 = 1.327_124_400_412_794_2e20;

/// Sun's nominal equatorial radius (m)
///
/// IAU 2015 Resolution B3 nominal value.
pub const R_SUN: f64 = 6.957e8;

/// Sun's mean radius (m)
///
/// Equivalent to equatorial radius for the Sun (assumed spherical).
pub const R_MEAN_SUN: f64 = 6.957e8;

/// Sun's J2 gravitational coefficient (dimensionless)
///
/// Solar oblateness coefficient.
pub const J2_SUN: f64 = 2.0e-7;

/// Total Solar Irradiance (TSI) at 1 AU (W/m²)
///
/// Also known as the solar constant. This is the total electromagnetic radiation
/// per unit area received from the Sun at a distance of 1 AU (at the top of Earth's
/// atmosphere), measured perpendicular to the incoming sunlight.
///
/// Value from NASA SORCE mission and widely adopted in astrodynamics.
/// Reference: Kopp, G., & Lean, J. L. (2011). A new, lower value of total solar
/// irradiance: Evidence and climate significance. Geophysical Research Letters, 38(1).
/// DOI: 10.1029/2010GL045777
pub const SOLAR_IRRADIANCE: f64 = 1361.0;

/// Solar radiation pressure at 1 AU for a perfectly absorbing surface (N/m²)
///
/// Calculated as P = E/c, where E is the solar irradiance and c is the speed of light.
/// For reflective surfaces, multiply by the reflectivity coefficient CR:
/// - CR = 1.0 for perfectly absorbing (black body)
/// - CR = 2.0 for perfectly reflecting (specular reflection)
/// - CR ≈ 1.2-1.5 for typical spacecraft surfaces
///
/// Value: 1361 W/m² / 299,792,458 m/s ≈ 4.5398 × 10⁻⁶ N/m²
pub const SOLAR_RADIATION_PRESSURE: f64 = 4.539_807_335_646_84e-6;

// =============================================================================
// MERCURY
// =============================================================================

/// Mercury's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 22,031.87 km³/s²
pub const GM_MERCURY: f64 = 2.203_187e13;

/// Mercury's equatorial radius (m)
pub const R_MERCURY: f64 = 2_440_530.0;

/// Mercury's polar radius (m)
pub const R_POLAR_MERCURY: f64 = 2_438_260.0;

/// Mercury's mean radius (m)
pub const R_MEAN_MERCURY: f64 = 2_439_400.0;

// =============================================================================
// VENUS
// =============================================================================

/// Venus's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 324,858.59 km³/s²
pub const GM_VENUS: f64 = 3.248_585_9e14;

/// Venus's equatorial radius (m)
pub const R_VENUS: f64 = 6_051_800.0;

/// Venus's polar radius (m)
pub const R_POLAR_VENUS: f64 = 6_051_800.0;

/// Venus's mean radius (m)
pub const R_MEAN_VENUS: f64 = 6_051_800.0;

/// Venus's J2 gravitational coefficient (dimensionless)
pub const J2_VENUS: f64 = 4.458e-6;

/// Venus's J3 gravitational coefficient (dimensionless)
pub const J3_VENUS: f64 = -2.1e-6;

// =============================================================================
// EARTH
// =============================================================================

/// Earth's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 398,600.44 km³/s²
/// IAU 2015 Resolution B3 nominal terrestrial mass parameter.
pub const GM_EARTH: f64 = 3.986_004_4e14;

/// Earth's equatorial radius (m)
///
/// WGS84 value, also IAU 2015 Resolution B3 nominal value.
pub const R_EARTH: f64 = 6_378_137.0;

/// Earth's polar radius (m)
///
/// WGS84 value, also IAU 2015 Resolution B3 nominal value.
pub const R_POLAR_EARTH: f64 = 6_356_752.314_245;

/// Earth's mean radius (m)
///
/// Volumetric mean radius.
pub const R_MEAN_EARTH: f64 = 6_371_008.4;

/// Earth's J2 gravitational coefficient (dimensionless)
///
/// Primary oblateness term, critical for orbital perturbations.
/// WGS84 value: 1.082626683×10⁻³
pub const J2_EARTH: f64 = 1.082_626_683e-3;

/// Earth's J3 gravitational coefficient (dimensionless)
///
/// Pear-shaped distortion term.
pub const J3_EARTH: f64 = -2.532_435_346e-6;

/// Earth's J4 gravitational coefficient (dimensionless)
///
/// Higher-order oblateness term. Represents the fourth zonal harmonic
/// in Earth's gravity field expansion. This coefficient is about 650× smaller
/// than J2 but becomes important for high-precision orbit propagation.
///
/// Value from satellite observations. Note: WGS84 reference ellipsoid gives
/// a different value (-2.37×10⁻⁶), but this observed value better represents
/// the actual gravitational field for orbit propagation.
pub const J4_EARTH: f64 = -1.649e-6;

/// Earth's J5 gravitational coefficient (dimensionless)
///
/// Odd zonal harmonic representing asymmetry about the equator.
/// This coefficient is non-zero for the actual Earth due to mass distribution
/// irregularities, though it is zero for the idealized WGS84 reference ellipsoid.
///
/// Value from satellite observations.
pub const J5_EARTH: f64 = -0.21e-6;

/// Earth's J6 gravitational coefficient (dimensionless)
///
/// Higher-order even zonal harmonic. This coefficient is about 1,800× smaller
/// than J2 and is used in high-precision orbit propagation over extended periods.
///
/// Value from satellite observations. Note: WGS84 reference ellipsoid gives
/// a different value (6.08×10⁻⁹).
pub const J6_EARTH: f64 = 0.646e-6;

/// Earth's atmospheric scale height at sea level (m)
///
/// Reference height for exponential atmosphere model.
pub const H0_EARTH: f64 = 8_500.0;

/// Earth's atmospheric density at sea level (kg/m³)
///
/// Reference density for exponential atmosphere model.
pub const RHO0_EARTH: f64 = 1.225;

// =============================================================================
// MOON
// =============================================================================

/// Moon's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 4,902.80 km³/s²
pub const GM_MOON: f64 = 4.902_80e12;

/// Moon's equatorial radius (m)
pub const R_MOON: f64 = 1_738_100.0;

/// Moon's polar radius (m)
pub const R_POLAR_MOON: f64 = 1_736_000.0;

/// Moon's mean radius (m)
pub const R_MEAN_MOON: f64 = 1_737_400.0;

// =============================================================================
// GALILEAN MOONS (JUPITER'S MAJOR SATELLITES)
// =============================================================================

/// Io's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 5,959.91547 km³/s²
/// Reference: gm_de440.tpc NAIF kernel
pub const GM_IO: f64 = 5.959_915_466_180_539e12;

/// Io's mean radius (m)
///
/// From JPL Planetary Satellite Physical Parameters.
/// Reference: JUP365 ephemeris
pub const R_IO: f64 = 1_821_490.0;

/// Europa's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 3,202.71210 km³/s²
/// Reference: gm_de440.tpc NAIF kernel
pub const GM_EUROPA: f64 = 3.202_712_099_607_295e12;

/// Europa's mean radius (m)
///
/// From JPL Planetary Satellite Physical Parameters.
/// Reference: JUP365 ephemeris
pub const R_EUROPA: f64 = 1_560_800.0;

/// Ganymede's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 9,887.83275 km³/s²
/// Ganymede is the largest moon in the Solar System.
/// Reference: gm_de440.tpc NAIF kernel
pub const GM_GANYMEDE: f64 = 9.887_832_752_719_638e12;

/// Ganymede's mean radius (m)
///
/// From JPL Planetary Satellite Physical Parameters.
/// Ganymede is the largest moon in the Solar System (larger than Mercury).
/// Reference: JUP365 ephemeris
pub const R_GANYMEDE: f64 = 2_631_200.0;

/// Callisto's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 7,179.28340 km³/s²
/// Reference: gm_de440.tpc NAIF kernel
pub const GM_CALLISTO: f64 = 7.179_283_402_579_837e12;

/// Callisto's mean radius (m)
///
/// From JPL Planetary Satellite Physical Parameters.
/// Reference: JUP365 ephemeris
pub const R_CALLISTO: f64 = 2_410_300.0;

// =============================================================================
// SATURNIAN MOONS
// =============================================================================

/// Titan's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 8,978.13710 km³/s²
/// Titan is Saturn's largest moon and the only moon with a substantial atmosphere.
/// Reference: gm_de440.tpc NAIF kernel
pub const GM_TITAN: f64 = 8.978_137_095_521_046e12;

/// Titan's mean radius (m)
///
/// From JPL Planetary Satellite Physical Parameters.
/// Titan is the second-largest moon in the Solar System.
/// Reference: SAT441 ephemeris
pub const R_TITAN: f64 = 2_574_760.0;

// =============================================================================
// NEPTUNIAN MOONS
// =============================================================================

/// Triton's standard gravitational parameter (m³/s²)
///
/// From JPL DE440: 1,428.49546 km³/s²
/// Triton is Neptune's largest moon and orbits retrograde.
/// Reference: gm_de440.tpc NAIF kernel
pub const GM_TRITON: f64 = 1.428_495_462_910_464e12;

/// Triton's mean radius (m)
///
/// From JPL Planetary Satellite Physical Parameters.
/// Reference: NEP097 ephemeris
pub const R_TRITON: f64 = 1_352_600.0;

// =============================================================================
// MARS
// =============================================================================

/// Mars's standard gravitational parameter (m³/s²)
///
/// Mars system from JPL DE440: 42,828.38 km³/s²
pub const GM_MARS: f64 = 4.282_838e13;

/// Mars's equatorial radius (m)
pub const R_MARS: f64 = 3_396_200.0;

/// Mars's polar radius (m)
pub const R_POLAR_MARS: f64 = 3_376_200.0;

/// Mars's mean radius (m)
pub const R_MEAN_MARS: f64 = 3_389_500.0;

/// Mars's J2 gravitational coefficient (dimensionless)
pub const J2_MARS: f64 = 1.960_45e-3;

/// Mars's J3 gravitational coefficient (dimensionless)
pub const J3_MARS: f64 = 3.14e-5;

// =============================================================================
// JUPITER
// =============================================================================

/// Jupiter system's standard gravitational parameter (m³/s²)
///
/// Jupiter system from JPL DE440: 126,712,764.10 km³/s²
/// IAU 2015 Resolution B3 nominal jovian mass parameter.
pub const GM_JUPITER: f64 = 1.267_127_641e17;

/// Jupiter's equatorial radius (m)
///
/// IAU 2015 Resolution B3 nominal jovian equatorial radius.
pub const R_JUPITER: f64 = 7.149_2e7;

/// Jupiter's polar radius (m)
///
/// IAU 2015 Resolution B3 nominal jovian polar radius.
pub const R_POLAR_JUPITER: f64 = 6.685_4e7;

/// Jupiter's mean radius (m)
pub const R_MEAN_JUPITER: f64 = 6.991_1e7;

// =============================================================================
// SATURN
// =============================================================================

/// Saturn system's standard gravitational parameter (m³/s²)
///
/// Saturn system from JPL DE440: 37,940,584.84 km³/s²
pub const GM_SATURN: f64 = 3.794_058_484e16;

/// Saturn's equatorial radius (m)
pub const R_SATURN: f64 = 6.026_8e7;

/// Saturn's polar radius (m)
pub const R_POLAR_SATURN: f64 = 5.431_4e7;

/// Saturn's mean radius (m)
pub const R_MEAN_SATURN: f64 = 5.823_2e7;

// =============================================================================
// URANUS
// =============================================================================

/// Uranus system's standard gravitational parameter (m³/s²)
///
/// Uranus system from JPL DE440: 5,794,556.40 km³/s²
pub const GM_URANUS: f64 = 5.794_556_4e15;

/// Uranus's equatorial radius (m)
pub const R_URANUS: f64 = 2.559_2e7;

/// Uranus's polar radius (m)
pub const R_POLAR_URANUS: f64 = 2.497_3e7;

/// Uranus's mean radius (m)
pub const R_MEAN_URANUS: f64 = 2.536_2e7;

// =============================================================================
// NEPTUNE
// =============================================================================

/// Neptune system's standard gravitational parameter (m³/s²)
///
/// Neptune system from JPL DE440: 6,836,527.10 km³/s²
pub const GM_NEPTUNE: f64 = 6.836_527_1e15;

/// Neptune's equatorial radius (m)
pub const R_NEPTUNE: f64 = 2.476_4e7;

/// Neptune's polar radius (m)
pub const R_POLAR_NEPTUNE: f64 = 2.431_5e7;

/// Neptune's mean radius (m)
pub const R_MEAN_NEPTUNE: f64 = 2.462_2e7;

// =============================================================================
// PLUTO
// =============================================================================

/// Pluto system's standard gravitational parameter (m³/s²)
///
/// Pluto system from JPL DE440: 975.50 km³/s²
pub const GM_PLUTO: f64 = 9.755e11;

/// Pluto's equatorial radius (m)
pub const R_PLUTO: f64 = 1_188_300.0;

/// Pluto's polar radius (m)
pub const R_POLAR_PLUTO: f64 = 1_188_300.0;

/// Pluto's mean radius (m)
pub const R_MEAN_PLUTO: f64 = 1_188_300.0;

// =============================================================================
// PLANETARY ORBITAL PARAMETERS
// =============================================================================

/// Mercury's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 0.38709927 AU
pub const A_MERCURY: f64 = 5.790_922_654_152_439e10;

/// Venus's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 0.72333566 AU
pub const A_VENUS: f64 = 1.082_094_745_373_792e11;

/// Earth's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 1.00000261 AU (Earth-Moon barycenter)
pub const A_EARTH: f64 = 1.495_982_611_504_425e11;

/// Mars's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 1.52371034 AU
pub const A_MARS: f64 = 2.279_438_224_275_731e11;

/// Jupiter's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 5.20288700 AU
pub const A_JUPITER: f64 = 7.783_408_166_927_108e11;

/// Saturn's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 9.53667594 AU
pub const A_SATURN: f64 = 1.426_666_414_179_921e12;

/// Uranus's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 19.18916464 AU
pub const A_URANUS: f64 = 2.870_658_170_655_732e12;

/// Neptune's semi-major axis (m)
///
/// From JPL planetary ephemerides, valid 1800 AD - 2050 AD.
/// Value: 30.06992276 AU
pub const A_NEPTUNE: f64 = 4.498_396_417_009_467e12;

// =============================================================================
// SPHERE OF INFLUENCE RADII
// =============================================================================

/// Mercury's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
/// where a is the semi-major axis of Mercury's orbit.
///
/// This is the approximate radius within which Mercury's gravity dominates
/// over the Sun's gravity for orbital mechanics calculations.
/// Used in patched conic approximations for interplanetary trajectories.
pub const R_SOI_MERCURY: f64 = 1.124_096_653_358_193e8;

/// Venus's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
pub const R_SOI_VENUS: f64 = 6.162_804_293_403_314e8;

/// Earth's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
///
/// For reference, the Moon's orbital radius is ~384,400 km,
/// well within Earth's sphere of influence (~924,600 km).
pub const R_SOI_EARTH: f64 = 9.246_492_066_976_895e8;

/// Mars's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
pub const R_SOI_MARS: f64 = 5.772_392_211_457_512e8;

/// Jupiter's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
///
/// Jupiter has a large sphere of influence (~48.2 million km) due to
/// its large mass and distance from the Sun.
pub const R_SOI_JUPITER: f64 = 4.820_957_444_909_551e10;

/// Saturn's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
///
/// Saturn has the largest sphere of influence of any planet (~54.6 million km)
/// due to its great distance from the Sun, despite having less mass than Jupiter.
pub const R_SOI_SATURN: f64 = 5.455_058_247_395_433e10;

/// Uranus's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
pub const R_SOI_URANUS: f64 = 5.176_365_156_171_003e10;

/// Neptune's sphere of influence radius (m)
///
/// Calculated using: r_SOI = a × (GM_planet / GM_sun)^(2/5)
pub const R_SOI_NEPTUNE: f64 = 8.666_171_649_697_98e10;

// =============================================================================
// CONVERSION FACTORS
// =============================================================================

/// Kilometers to meters conversion factor
pub const KM_TO_M: f64 = 1_000.0;

/// Meters to kilometers conversion factor
pub const M_TO_KM: f64 = 1.0 / 1_000.0;

/// Degrees to radians conversion factor
pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

/// Radians to degrees conversion factor
pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

/// Days to seconds conversion factor
pub const DAY_TO_SEC: f64 = 86_400.0;

/// Seconds to days conversion factor
pub const SEC_TO_DAY: f64 = 1.0 / 86_400.0;

// =============================================================================
// TIME REFERENCE EPOCHS
// =============================================================================

/// J2000 epoch in Julian Date (Terrestrial Time)
///
/// January 1, 2000, 12:00:00 TT
/// Reference epoch for astronomical calculations.
pub const J2000_TT: f64 = 2_451_545.0;

/// J2000 epoch in Modified Julian Date
///
/// MJD = JD - 2400000.5
pub const J2000_MJD: f64 = 51_544.5;

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Convert kilometers to meters
#[inline]
pub fn km_to_m(km: f64) -> f64 {
    km * KM_TO_M
}

/// Convert meters to kilometers
#[inline]
pub fn m_to_km(m: f64) -> f64 {
    m * M_TO_KM
}

/// Convert degrees to radians
#[inline]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * DEG_TO_RAD
}

/// Convert radians to degrees
#[inline]
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * RAD_TO_DEG
}

/// Convert days to seconds
#[inline]
pub fn days_to_sec(days: f64) -> f64 {
    days * DAY_TO_SEC
}

/// Convert seconds to days
#[inline]
pub fn sec_to_days(sec: f64) -> f64 {
    sec * SEC_TO_DAY
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_km_to_m() {
        assert_relative_eq!(km_to_m(1.0), 1000.0);
        assert_relative_eq!(km_to_m(10.5), 10500.0);
        assert_relative_eq!(km_to_m(0.001), 1.0);
    }

    #[test]
    fn test_m_to_km() {
        assert_relative_eq!(m_to_km(1000.0), 1.0);
        assert_relative_eq!(m_to_km(10500.0), 10.5);
        assert_relative_eq!(m_to_km(1.0), 0.001);
    }

    #[test]
    fn test_deg_to_rad() {
        assert_relative_eq!(deg_to_rad(180.0), std::f64::consts::PI);
        assert_relative_eq!(deg_to_rad(90.0), std::f64::consts::PI / 2.0);
        assert_relative_eq!(deg_to_rad(0.0), 0.0);
        assert_relative_eq!(deg_to_rad(360.0), 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_rad_to_deg() {
        assert_relative_eq!(rad_to_deg(std::f64::consts::PI), 180.0);
        assert_relative_eq!(rad_to_deg(std::f64::consts::PI / 2.0), 90.0);
        assert_relative_eq!(rad_to_deg(0.0), 0.0);
        assert_relative_eq!(rad_to_deg(2.0 * std::f64::consts::PI), 360.0);
    }

    #[test]
    fn test_days_to_sec() {
        assert_relative_eq!(days_to_sec(1.0), 86400.0);
        assert_relative_eq!(days_to_sec(0.5), 43200.0);
        assert_relative_eq!(days_to_sec(7.0), 604800.0);
    }

    #[test]
    fn test_sec_to_days() {
        assert_relative_eq!(sec_to_days(86400.0), 1.0);
        assert_relative_eq!(sec_to_days(43200.0), 0.5);
        assert_relative_eq!(sec_to_days(604800.0), 7.0);
    }

    #[test]
    fn test_conversion_roundtrips() {
        // Test that conversions are inverses of each other
        let km = 123.456;
        assert_relative_eq!(m_to_km(km_to_m(km)), km);

        let deg = 45.678;
        assert_relative_eq!(rad_to_deg(deg_to_rad(deg)), deg);

        let days = 3.14159;
        assert_relative_eq!(sec_to_days(days_to_sec(days)), days);
    }

    #[test]
    fn test_constant_values() {
        // Test some key constant values
        assert!(GM_EARTH > 0.0);
        assert!(R_EARTH > 0.0);
        assert!(AU > 0.0);
        assert!(C > 0.0);

        // Earth should be oblate (equatorial > polar)
        assert!(R_EARTH > R_POLAR_EARTH);

        // J2 should be the dominant term
        assert!(J2_EARTH.abs() > J3_EARTH.abs());
        assert!(J2_EARTH.abs() > J4_EARTH.abs());
    }
}
