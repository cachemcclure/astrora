//! Classical orbital elements

/// Classical (Keplerian) orbital elements
#[derive(Debug, Clone, Copy)]
pub struct OrbitalElements {
    /// Semi-major axis (m)
    pub a: f64,
    /// Eccentricity (dimensionless)
    pub e: f64,
    /// Inclination (radians)
    pub i: f64,
    /// Right ascension of ascending node (radians)
    pub raan: f64,
    /// Argument of periapsis (radians)
    pub arg_pe: f64,
    /// True anomaly (radians)
    pub nu: f64,
}

impl OrbitalElements {
    /// Create new orbital elements
    pub fn new(a: f64, e: f64, i: f64, raan: f64, arg_pe: f64, nu: f64) -> Self {
        Self { a, e, i, raan, arg_pe, nu }
    }

    /// Calculate orbital period (seconds)
    pub fn period(&self, mu: f64) -> f64 {
        use std::f64::consts::PI;
        2.0 * PI * (self.a.powi(3) / mu).sqrt()
    }

    /// Calculate periapsis distance (m)
    pub fn periapsis(&self) -> f64 {
        self.a * (1.0 - self.e)
    }

    /// Calculate apoapsis distance (m)
    pub fn apoapsis(&self) -> f64 {
        self.a * (1.0 + self.e)
    }
}
