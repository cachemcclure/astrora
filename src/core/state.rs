//! State vector representations
//!
//! This module provides Cartesian state vector representations and
//! calculations for orbital properties from position and velocity.

use crate::core::linalg::Vector3;
use crate::core::error::{PoliastroError, PoliastroResult};
use pyo3::prelude::*;

/// Cartesian state vector (position and velocity)
#[pyclass(name = "CartesianState", module = "astrora._core")]
#[derive(Debug, Clone, Copy)]
pub struct CartesianState {
    /// Position vector [x, y, z] in meters
    pub position: Vector3,
    /// Velocity vector [vx, vy, vz] in meters per second
    pub velocity: Vector3,
}

impl CartesianState {
    /// Create a new Cartesian state
    pub fn new(position: Vector3, velocity: Vector3) -> Self {
        Self { position, velocity }
    }

    /// Get the position vector
    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    /// Get the velocity vector
    pub fn velocity(&self) -> &Vector3 {
        &self.velocity
    }

    /// Calculate orbital energy per unit mass (m²/s²)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Specific orbital energy ε = v²/2 - μ/r
    ///
    /// # Note
    /// - Negative energy indicates elliptical orbit (bound)
    /// - Zero energy indicates parabolic orbit (marginally bound)
    /// - Positive energy indicates hyperbolic orbit (unbound)
    pub fn specific_energy(&self, mu: f64) -> f64 {
        let r = self.position.norm();
        let v = self.velocity.norm();
        0.5 * v * v - mu / r
    }

    /// Calculate specific angular momentum vector (m²/s)
    ///
    /// # Returns
    /// Angular momentum per unit mass **h** = **r** × **v**
    ///
    /// # Note
    /// The angular momentum vector is perpendicular to the orbital plane
    /// and points in the direction determined by the right-hand rule.
    pub fn specific_angular_momentum(&self) -> Vector3 {
        self.position.cross(&self.velocity)
    }

    /// Calculate semi-major axis (m)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Semi-major axis a = -μ/(2ε)
    ///
    /// # Errors
    /// Returns `PoliastroError::UnsupportedOrbitType` if:
    /// - The orbit is parabolic (energy ≈ 0)
    /// - The orbit is hyperbolic (energy > 0) - semi-major axis is negative
    ///
    /// # Note
    /// Only elliptical orbits have positive semi-major axes.
    pub fn semi_major_axis(&self, mu: f64) -> PoliastroResult<f64> {
        let energy = self.specific_energy(mu);

        // Check for parabolic orbit (energy ≈ 0)
        if energy.abs() < 1e-10 {
            return Err(PoliastroError::UnsupportedOrbitType {
                operation: "semi_major_axis calculation".into(),
                orbit_type: "parabolic".into(),
            });
        }

        let a = -mu / (2.0 * energy);

        // Check for hyperbolic orbit (a < 0)
        if a < 0.0 {
            return Err(PoliastroError::UnsupportedOrbitType {
                operation: "semi_major_axis calculation".into(),
                orbit_type: "hyperbolic".into(),
            });
        }

        Ok(a)
    }

    /// Calculate orbital period (s)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Orbital period T = 2π√(a³/μ) for elliptical orbits
    ///
    /// # Errors
    /// Returns `PoliastroError::UnsupportedOrbitType` if:
    /// - The orbit is not elliptical (e >= 1)
    /// - The semi-major axis cannot be computed
    ///
    /// # Note
    /// Only elliptical orbits have a finite period.
    /// Parabolic and hyperbolic orbits do not return to their starting point.
    pub fn period(&self, mu: f64) -> PoliastroResult<f64> {
        use std::f64::consts::PI;

        let a = self.semi_major_axis(mu)?;
        let period = 2.0 * PI * (a.powi(3) / mu).sqrt();

        Ok(period)
    }

    /// Calculate eccentricity vector
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Eccentricity vector **e** = [(**v**² - μ/r)**r** - (**r**·**v**)**v**] / μ
    ///
    /// # Note
    /// The eccentricity vector:
    /// - Points from apoapsis to periapsis
    /// - Has magnitude equal to the orbital eccentricity
    /// - Is dimensionless
    /// - Lies in the orbital plane
    ///
    /// # Example
    /// ```
    /// use astrora_core::core::{state::CartesianState, linalg::Vector3};
    ///
    /// // Circular orbit example
    /// let pos = Vector3::new(7000e3, 0.0, 0.0);  // 7000 km altitude
    /// let vel = Vector3::new(0.0, 7546.0, 0.0);   // Circular orbit velocity
    /// let state = CartesianState::new(pos, vel);
    ///
    /// let mu = 3.986004418e14;  // Earth's GM
    /// let ecc_vec = state.eccentricity_vector(mu);
    /// let ecc = ecc_vec.norm();  // Should be ≈ 0 for circular orbit
    /// ```
    pub fn eccentricity_vector(&self, mu: f64) -> Vector3 {
        let r_mag = self.position.norm();
        let v_mag_sq = self.velocity.norm_squared();
        let r_dot_v = self.position.dot(&self.velocity);

        // **e** = [(v² - μ/r)**r** - (**r**·**v**)**v**] / μ
        let term1 = (v_mag_sq - mu / r_mag) * self.position;
        let term2 = r_dot_v * self.velocity;

        (term1 - term2) / mu
    }

    /// Calculate eccentricity magnitude
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Orbital eccentricity e = ||**e**|| (dimensionless)
    ///
    /// # Note
    /// Orbit types based on eccentricity:
    /// - e = 0: Circular orbit
    /// - 0 < e < 1: Elliptical orbit
    /// - e = 1: Parabolic trajectory
    /// - e > 1: Hyperbolic trajectory
    ///
    /// # Example
    /// ```
    /// use astrora_core::core::{state::CartesianState, linalg::Vector3};
    ///
    /// let pos = Vector3::new(7000e3, 0.0, 0.0);
    /// let vel = Vector3::new(0.0, 7546.0, 0.0);
    /// let state = CartesianState::new(pos, vel);
    ///
    /// let mu = 3.986004418e14;
    /// let ecc = state.eccentricity(mu);
    /// println!("Eccentricity: {:.6}", ecc);  // Should be ≈ 0
    /// ```
    pub fn eccentricity(&self, mu: f64) -> f64 {
        self.eccentricity_vector(mu).norm()
    }

    /// Get the orbit type based on eccentricity
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// String description of the orbit type
    pub fn orbit_type(&self, mu: f64) -> &'static str {
        let e = self.eccentricity(mu);

        if e < 1e-8 {
            "circular"
        } else if e < 1.0 {
            "elliptical"
        } else if (e - 1.0).abs() < 1e-8 {
            "parabolic"
        } else {
            "hyperbolic"
        }
    }
}

// =============================================================================
// Python Bindings
// =============================================================================

#[pymethods]
impl CartesianState {
    /// Create a new Cartesian state from position and velocity arrays
    ///
    /// # Arguments
    /// * `position` - Position vector [x, y, z] in meters
    /// * `velocity` - Velocity vector [vx, vy, vz] in meters/second
    ///
    /// # Returns
    /// CartesianState object
    #[new]
    pub fn py_new(position: [f64; 3], velocity: [f64; 3]) -> Self {
        Self::new(
            Vector3::new(position[0], position[1], position[2]),
            Vector3::new(velocity[0], velocity[1], velocity[2]),
        )
    }

    /// Get position vector as list [x, y, z]
    #[getter]
    pub fn get_position(&self) -> [f64; 3] {
        [self.position.x, self.position.y, self.position.z]
    }

    /// Get velocity vector as list [vx, vy, vz]
    #[getter]
    pub fn get_velocity(&self) -> [f64; 3] {
        [self.velocity.x, self.velocity.y, self.velocity.z]
    }

    /// Calculate specific orbital energy (m²/s²)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Specific orbital energy
    #[pyo3(name = "specific_energy")]
    pub fn py_specific_energy(&self, mu: f64) -> f64 {
        self.specific_energy(mu)
    }

    /// Calculate specific angular momentum vector (m²/s)
    ///
    /// # Returns
    /// Angular momentum vector [hx, hy, hz]
    #[pyo3(name = "specific_angular_momentum")]
    pub fn py_specific_angular_momentum(&self) -> [f64; 3] {
        let h = self.specific_angular_momentum();
        [h.x, h.y, h.z]
    }

    /// Calculate semi-major axis (m)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Semi-major axis in meters
    ///
    /// # Raises
    /// ValueError: If orbit is parabolic or hyperbolic
    #[pyo3(name = "semi_major_axis")]
    pub fn py_semi_major_axis(&self, mu: f64) -> PyResult<f64> {
        self.semi_major_axis(mu).map_err(Into::into)
    }

    /// Calculate orbital period (s)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Orbital period in seconds
    ///
    /// # Raises
    /// ValueError: If orbit is not elliptical
    #[pyo3(name = "period")]
    pub fn py_period(&self, mu: f64) -> PyResult<f64> {
        self.period(mu).map_err(Into::into)
    }

    /// Calculate eccentricity vector
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Eccentricity vector [ex, ey, ez]
    #[pyo3(name = "eccentricity_vector")]
    pub fn py_eccentricity_vector(&self, mu: f64) -> [f64; 3] {
        let e = self.eccentricity_vector(mu);
        [e.x, e.y, e.z]
    }

    /// Calculate eccentricity magnitude
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Orbital eccentricity (dimensionless)
    #[pyo3(name = "eccentricity")]
    pub fn py_eccentricity(&self, mu: f64) -> f64 {
        self.eccentricity(mu)
    }

    /// Get orbit type classification
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Orbit type: "circular", "elliptical", "parabolic", or "hyperbolic"
    #[pyo3(name = "orbit_type")]
    pub fn py_orbit_type(&self, mu: f64) -> &'static str {
        self.orbit_type(mu)
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "CartesianState(position=[{:.3e}, {:.3e}, {:.3e}] m, velocity=[{:.3e}, {:.3e}, {:.3e}] m/s)",
            self.position.x, self.position.y, self.position.z,
            self.velocity.x, self.velocity.y, self.velocity.z
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::core::constants::GM_EARTH;

    #[test]
    fn test_specific_energy_circular_orbit() {
        // Circular orbit at 7000 km radius
        let r = 7000e3;
        let v = (GM_EARTH / r).sqrt();  // Circular orbit velocity

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v, 0.0);
        let state = CartesianState::new(pos, vel);

        let energy = state.specific_energy(GM_EARTH);
        let expected_energy = -GM_EARTH / (2.0 * r);

        assert_relative_eq!(energy, expected_energy, epsilon = 1e-3);
    }

    #[test]
    fn test_specific_angular_momentum() {
        let pos = Vector3::new(7000e3, 0.0, 0.0);
        let vel = Vector3::new(0.0, 7546.0, 0.0);
        let state = CartesianState::new(pos, vel);

        let h = state.specific_angular_momentum();

        // For this configuration, h should be in the +z direction
        assert_relative_eq!(h.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(h.y, 0.0, epsilon = 1e-6);
        assert!(h.z > 0.0);

        // Magnitude should be r * v for this perpendicular case
        let expected_h = 7000e3 * 7546.0;
        assert_relative_eq!(h.norm(), expected_h, epsilon = 1e-3);
    }

    #[test]
    fn test_semi_major_axis_circular_orbit() {
        let r = 7000e3;
        let v = (GM_EARTH / r).sqrt();

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v, 0.0);
        let state = CartesianState::new(pos, vel);

        let a = state.semi_major_axis(GM_EARTH).unwrap();

        // For circular orbit, semi-major axis equals radius
        assert_relative_eq!(a, r, epsilon = 1e-3);
    }

    #[test]
    fn test_period_circular_orbit() {
        use std::f64::consts::PI;

        let r = 7000e3;
        let v = (GM_EARTH / r).sqrt();

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v, 0.0);
        let state = CartesianState::new(pos, vel);

        let period = state.period(GM_EARTH).unwrap();
        let expected_period = 2.0 * PI * (r.powi(3) / GM_EARTH).sqrt();

        assert_relative_eq!(period, expected_period, epsilon = 1e-3);

        // Should be approximately 97.1 minutes for 7000 km radius orbit
        assert_relative_eq!(period / 60.0, 97.14, epsilon = 0.1);
    }

    #[test]
    fn test_eccentricity_circular_orbit() {
        let r = 7000e3;
        let v = (GM_EARTH / r).sqrt();

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v, 0.0);
        let state = CartesianState::new(pos, vel);

        let ecc = state.eccentricity(GM_EARTH);

        // Circular orbit should have eccentricity ≈ 0
        assert!(ecc < 1e-6);
    }

    #[test]
    fn test_eccentricity_elliptical_orbit() {
        // Create an elliptical orbit with known eccentricity
        // Using perifocal frame: position at periapsis
        let e = 0.5;  // Target eccentricity
        let a = 10000e3;  // Semi-major axis: 10,000 km

        let r_p = a * (1.0 - e);  // Periapsis distance
        let v_p = ((GM_EARTH / a) * (1.0 + e) / (1.0 - e)).sqrt();  // Periapsis velocity

        let pos = Vector3::new(r_p, 0.0, 0.0);
        let vel = Vector3::new(0.0, v_p, 0.0);
        let state = CartesianState::new(pos, vel);

        let ecc = state.eccentricity(GM_EARTH);

        assert_relative_eq!(ecc, 0.5, epsilon = 1e-6);
    }

    #[test]
    fn test_eccentricity_vector_direction() {
        // At periapsis, eccentricity vector should point in direction of position
        let e_target = 0.3;
        let a = 8000e3;

        let r_p = a * (1.0 - e_target);
        let v_p = ((GM_EARTH / a) * (1.0 + e_target) / (1.0 - e_target)).sqrt();

        let pos = Vector3::new(r_p, 0.0, 0.0);
        let vel = Vector3::new(0.0, v_p, 0.0);
        let state = CartesianState::new(pos, vel);

        let ecc_vec = state.eccentricity_vector(GM_EARTH);

        // Eccentricity vector should be in the +x direction (same as position at periapsis)
        assert!(ecc_vec.x > 0.0);
        assert_relative_eq!(ecc_vec.y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(ecc_vec.z, 0.0, epsilon = 1e-6);

        // Magnitude should match target eccentricity
        assert_relative_eq!(ecc_vec.norm(), e_target, epsilon = 1e-6);
    }

    #[test]
    fn test_orbit_type_classification() {
        // Circular orbit
        let r = 7000e3;
        let v = (GM_EARTH / r).sqrt();
        let state_circular = CartesianState::new(
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v, 0.0)
        );
        assert_eq!(state_circular.orbit_type(GM_EARTH), "circular");

        // Elliptical orbit
        let a = 10000e3;
        let e = 0.5;
        let r_p = a * (1.0 - e);
        let v_p = ((GM_EARTH / a) * (1.0 + e) / (1.0 - e)).sqrt();
        let state_elliptical = CartesianState::new(
            Vector3::new(r_p, 0.0, 0.0),
            Vector3::new(0.0, v_p, 0.0)
        );
        assert_eq!(state_elliptical.orbit_type(GM_EARTH), "elliptical");

        // Hyperbolic orbit (escape velocity)
        let v_escape = (2.0 * GM_EARTH / r).sqrt() * 1.5;  // 1.5x escape velocity
        let state_hyperbolic = CartesianState::new(
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v_escape, 0.0)
        );
        assert_eq!(state_hyperbolic.orbit_type(GM_EARTH), "hyperbolic");
    }

    #[test]
    fn test_hyperbolic_orbit_error() {
        // Create hyperbolic orbit
        let r = 7000e3;
        let v_escape = (2.0 * GM_EARTH / r).sqrt() * 1.5;

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v_escape, 0.0);
        let state = CartesianState::new(pos, vel);

        // Semi-major axis should return error for hyperbolic orbit
        let result = state.semi_major_axis(GM_EARTH);
        assert!(result.is_err());

        // Period should also return error
        let period_result = state.period(GM_EARTH);
        assert!(period_result.is_err());
    }

    #[test]
    fn test_position_velocity_getters() {
        // Test position() and velocity() getter methods
        let pos = Vector3::new(7000e3, 1000e3, 2000e3);
        let vel = Vector3::new(1000.0, 7000.0, 500.0);
        let state = CartesianState::new(pos, vel);

        // Test getters
        assert_eq!(state.position(), &pos);
        assert_eq!(state.velocity(), &vel);
        assert_eq!(state.position().x, 7000e3);
        assert_eq!(state.velocity().y, 7000.0);
    }

    #[test]
    fn test_orbit_type_parabolic() {
        // Test parabolic orbit classification (e ≈ 1.0)
        // For parabolic orbit: v² = 2μ/r
        let r = 7000e3;
        let v_parabolic = (2.0 * GM_EARTH / r).sqrt();

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v_parabolic, 0.0);
        let state = CartesianState::new(pos, vel);

        let orbit_type = state.orbit_type(GM_EARTH);
        assert_eq!(orbit_type, "parabolic");
    }

    #[test]
    fn test_semi_major_axis_parabolic_error() {
        // Test that parabolic orbits return error for semi_major_axis
        // Parabolic orbit has energy ≈ 0
        let r = 7000e3;
        let v_parabolic = (2.0 * GM_EARTH / r).sqrt();

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v_parabolic, 0.0);
        let state = CartesianState::new(pos, vel);

        let result = state.semi_major_axis(GM_EARTH);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("parabolic"));
    }

    #[test]
    fn test_period_parabolic_error() {
        // Test that parabolic orbits return error for period
        let r = 7000e3;
        let v_parabolic = (2.0 * GM_EARTH / r).sqrt();

        let pos = Vector3::new(r, 0.0, 0.0);
        let vel = Vector3::new(0.0, v_parabolic, 0.0);
        let state = CartesianState::new(pos, vel);

        let result = state.period(GM_EARTH);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("parabolic"));
    }

    #[test]
    fn test_orbit_type_all_classifications() {
        // Test all orbit type classifications
        let r = 7000e3;

        // Circular (e ≈ 0)
        let v_circ = (GM_EARTH / r).sqrt();
        let state_circ = CartesianState::new(
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v_circ, 0.0),
        );
        assert_eq!(state_circ.orbit_type(GM_EARTH), "circular");

        // Elliptical (0 < e < 1)
        let v_ellip = v_circ * 1.2; // Slightly higher velocity
        let state_ellip = CartesianState::new(
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v_ellip, 0.0),
        );
        assert_eq!(state_ellip.orbit_type(GM_EARTH), "elliptical");

        // Parabolic (e ≈ 1)
        let v_para = (2.0 * GM_EARTH / r).sqrt();
        let state_para = CartesianState::new(
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v_para, 0.0),
        );
        assert_eq!(state_para.orbit_type(GM_EARTH), "parabolic");

        // Hyperbolic (e > 1)
        let v_hyp = v_para * 1.1; // Above escape velocity
        let state_hyp = CartesianState::new(
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v_hyp, 0.0),
        );
        assert_eq!(state_hyp.orbit_type(GM_EARTH), "hyperbolic");
    }
}
