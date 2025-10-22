//! Physical and astronomical constants

/// Gravitational constant (m^3 kg^-1 s^-2)
pub const G: f64 = 6.67430e-11;

/// Speed of light (m/s)
pub const C: f64 = 299_792_458.0;

/// Astronomical unit (m)
pub const AU: f64 = 1.495_978_707e11;

/// Earth's standard gravitational parameter (m^3/s^2)
pub const GM_EARTH: f64 = 3.986_004_418e14;

/// Sun's standard gravitational parameter (m^3/s^2)
pub const GM_SUN: f64 = 1.327_124_400_42e20;

/// Moon's standard gravitational parameter (m^3/s^2)
pub const GM_MOON: f64 = 4.902_800_076e12;

/// Earth's mean radius (m)
pub const R_EARTH: f64 = 6_371_000.0;

/// Earth's equatorial radius (m)
pub const R_EARTH_EQUATORIAL: f64 = 6_378_137.0;

/// Earth's polar radius (m)
pub const R_EARTH_POLAR: f64 = 6_356_752.314_245;

/// Earth's J2 coefficient (dimensionless)
pub const J2_EARTH: f64 = 1.082_626_8e-3;
