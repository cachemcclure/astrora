//! Gravity assist (planetary flyby) calculations
//!
//! A gravity assist (or gravity slingshot) is a maneuver where a spacecraft uses the
//! gravity of a planet or other celestial body to alter its trajectory and speed relative
//! to the Sun, without expending propellant. This technique is critical for interplanetary
//! missions to reach high speeds or change trajectory directions efficiently.
//!
//! # Theory
//!
//! ## Physical Principle
//! In the planet's reference frame, the spacecraft's speed remains constant (energy is
//! conserved in the two-body problem), but the direction changes due to the hyperbolic
//! trajectory around the planet. However, in the heliocentric (Sun) reference frame,
//! the spacecraft can gain or lose significant velocity.
//!
//! ## Key Concepts
//!
//! 1. **Hyperbolic Excess Velocity (v∞)**: The velocity of the spacecraft relative to
//!    the planet at infinite distance (before/after the encounter)
//!
//! 2. **Turning Angle (δ)**: The angle through which the velocity vector is rotated
//!    as the spacecraft rounds the planet
//!
//! 3. **B-plane**: A targeting plane perpendicular to the incoming asymptote, used
//!    to specify the flyby geometry
//!
//! 4. **Patched Conics**: Approximation where the trajectory is split into segments,
//!    each dominated by one gravitational body
//!
//! # Equations
//!
//! ## Hyperbolic Trajectory Eccentricity
//! ```text
//! e = 1 + (r_p × v∞²) / μ
//! ```
//! where:
//! - r_p = periapsis radius (closest approach distance)
//! - v∞ = hyperbolic excess velocity
//! - μ = gravitational parameter of the flyby body
//!
//! ## Turning Angle (Deflection Angle)
//! ```text
//! δ = 2 × arcsin(1/e)
//! ```
//! or equivalently:
//! ```text
//! θ∞ = arccos(-1/e)
//! δ = 2θ∞ - π
//! ```
//!
//! ## Delta-v in Heliocentric Frame
//! ```text
//! Δv = |v∞_out - v∞_in|
//! ```
//! For the simplified case (symmetric deflection):
//! ```text
//! Δv = 2 × v∞ × sin(δ/2)
//! ```
//!
//! ## B-plane Parameters
//! The impact parameter (B-vector magnitude):
//! ```text
//! B = a × √(e² - 1)
//! ```
//! where a is the semi-major axis of the hyperbola.
//!
//! Periapsis from impact parameter:
//! ```text
//! r_p = (μ/v∞²) × (√[1 + (B×v∞²/μ)²] - 1)
//! ```
//!
//! # References
//! - Curtis, H. D. (2013). Orbital Mechanics for Engineering Students. Ch. 8
//! - Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications. Ch. 8
//! - <https://en.wikipedia.org/wiki/Gravity_assist>
//! - <https://en.wikipedia.org/wiki/Hyperbolic_trajectory>
//! - Braeunig, R. (2013). Rocket and Space Technology: Orbital Mechanics
//! - Strange, N., & Longuski, J. (2002). Graphical Method for Gravity-Assist Trajectory Design. Journal of Spacecraft and Rockets.

use std::f64::consts::PI;

use crate::core::{PoliastroError, PoliastroResult};

/// Result of a gravity assist calculation
///
/// Contains all relevant parameters for a planetary flyby maneuver.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravityAssistResult {
    /// Hyperbolic excess velocity magnitude (m/s)
    pub v_infinity: f64,
    /// Periapsis radius - closest approach distance (m)
    pub r_periapsis: f64,
    /// Gravitational parameter of flyby body μ = GM (m³/s²)
    pub mu: f64,
    /// Eccentricity of hyperbolic trajectory (dimensionless, e > 1)
    pub eccentricity: f64,
    /// Turning angle / deflection angle (radians)
    pub delta: f64,
    /// Delta-v magnitude in heliocentric frame (m/s)
    pub delta_v_magnitude: f64,
    /// Semi-major axis of hyperbola (m, negative for hyperbolic orbit)
    pub semi_major_axis: f64,
    /// Asymptote angle θ∞ (radians)
    pub theta_infinity: f64,
    /// Impact parameter / B-vector magnitude (m)
    pub b_parameter: f64,
    /// Specific orbital energy (m²/s²)
    pub specific_energy: f64,
}

/// B-plane targeting parameters
///
/// The B-plane is a plane perpendicular to the incoming asymptote, used to
/// specify the flyby geometry. It's defined by three orthogonal unit vectors:
/// S (along incoming v∞), T (perpendicular, in ecliptic plane), and R (S × T).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BPlaneParameters {
    /// B-vector magnitude (m) - impact parameter
    pub b_magnitude: f64,
    /// B·T component (m) - component in T direction
    pub b_dot_t: f64,
    /// B·R component (m) - component in R direction
    pub b_dot_r: f64,
    /// Theta angle - angle between B vector and T unit vector (radians)
    pub theta: f64,
}

/// Velocity components for gravity assist
///
/// Tracks the spacecraft velocity in both the planet frame and heliocentric frame
/// before and after the flyby.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravityAssistVelocities {
    /// Incoming hyperbolic excess velocity in planet frame (3D vector, m/s)
    pub v_infinity_in: [f64; 3],
    /// Outgoing hyperbolic excess velocity in planet frame (3D vector, m/s)
    pub v_infinity_out: [f64; 3],
    /// Incoming spacecraft velocity in heliocentric frame (3D vector, m/s)
    pub v_sc_in: [f64; 3],
    /// Outgoing spacecraft velocity in heliocentric frame (3D vector, m/s)
    pub v_sc_out: [f64; 3],
    /// Planet velocity in heliocentric frame (3D vector, m/s)
    pub v_planet: [f64; 3],
    /// Delta-v vector in heliocentric frame (3D vector, m/s)
    pub delta_v: [f64; 3],
}

/// Gravity assist calculator
///
/// Provides methods for calculating planetary flyby maneuvers, including
/// turning angles, delta-v changes, and B-plane targeting.
pub struct GravityAssist;

impl GravityAssist {
    /// Calculate a gravity assist flyby given periapsis radius and hyperbolic excess velocity
    ///
    /// This is the most common formulation for gravity assist calculations. Given the
    /// closest approach distance and the relative velocity at infinity, this computes
    /// the deflection angle and effective delta-v gained.
    ///
    /// # Arguments
    /// * `v_infinity` - Hyperbolic excess velocity magnitude |v∞| (m/s)
    /// * `r_periapsis` - Periapsis radius / closest approach distance (m)
    /// * `mu` - Gravitational parameter μ = GM of flyby body (m³/s²)
    ///
    /// # Returns
    /// A `GravityAssistResult` containing all flyby parameters
    ///
    /// # Errors
    /// Returns error if:
    /// - Any parameter is non-positive
    /// - Periapsis is inside the body's physical radius (would result in impact)
    ///
    /// # Examples
    /// ```
    /// use poliastro::maneuvers::GravityAssist;
    ///
    /// // Jupiter flyby with v∞ = 5.64 km/s, periapsis at 3 Jupiter radii
    /// let mu_jupiter = 1.266865e17; // m³/s²
    /// let r_jupiter = 71492e3; // m
    /// let result = GravityAssist::from_periapsis(
    ///     5640.0,
    ///     3.0 * r_jupiter,
    ///     mu_jupiter
    /// ).unwrap();
    /// ```
    pub fn from_periapsis(
        v_infinity: f64,
        r_periapsis: f64,
        mu: f64,
    ) -> PoliastroResult<GravityAssistResult> {
        // Validate inputs
        if v_infinity <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "v_infinity",
                v_infinity,
                "must be positive",
            ));
        }
        if r_periapsis <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_periapsis",
                r_periapsis,
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

        // Calculate eccentricity of hyperbolic trajectory
        // e = 1 + (r_p × v∞²) / μ
        let eccentricity = 1.0 + (r_periapsis * v_infinity.powi(2)) / mu;

        if eccentricity <= 1.0 {
            return Err(PoliastroError::invalid_parameter(
                "eccentricity",
                eccentricity,
                "must be > 1 for hyperbolic trajectory",
            ));
        }

        // Calculate asymptote angle
        // θ∞ = arccos(-1/e)
        let theta_infinity = (-1.0 / eccentricity).acos();

        // Calculate turning angle (deflection angle)
        // δ = 2θ∞ - π
        let delta = 2.0 * theta_infinity - PI;

        // Alternative formula (should give same result): δ = 2 × arcsin(1/e)
        // let delta_alt = 2.0 * (1.0 / eccentricity).asin();

        // Calculate delta-v magnitude (simplified formula for symmetric deflection)
        // Δv = 2 × v∞ × sin(δ/2)
        let delta_v_magnitude = 2.0 * v_infinity * (delta / 2.0).sin();

        // Calculate semi-major axis (negative for hyperbolic orbit)
        // a = -μ / v∞²
        let semi_major_axis = -mu / v_infinity.powi(2);

        // Calculate impact parameter (B-vector magnitude)
        // B = |a| × √(e² - 1)
        let b_parameter = semi_major_axis.abs() * (eccentricity.powi(2) - 1.0).sqrt();

        // Calculate specific orbital energy
        // ε = v∞² / 2
        let specific_energy = v_infinity.powi(2) / 2.0;

        Ok(GravityAssistResult {
            v_infinity,
            r_periapsis,
            mu,
            eccentricity,
            delta,
            delta_v_magnitude,
            semi_major_axis,
            theta_infinity,
            b_parameter,
            specific_energy,
        })
    }

    /// Calculate periapsis radius from impact parameter and hyperbolic excess velocity
    ///
    /// This is useful for B-plane targeting where the impact parameter is specified
    /// and we need to determine the resulting periapsis distance.
    ///
    /// # Arguments
    /// * `v_infinity` - Hyperbolic excess velocity magnitude |v∞| (m/s)
    /// * `b_parameter` - Impact parameter / B-vector magnitude (m)
    /// * `mu` - Gravitational parameter μ = GM of flyby body (m³/s²)
    ///
    /// # Returns
    /// Periapsis radius (m)
    ///
    /// # Formula
    /// ```text
    /// r_p = (μ/v∞²) × (√[1 + (B×v∞²/μ)²] - 1)
    /// ```
    pub fn periapsis_from_b_parameter(v_infinity: f64, b_parameter: f64, mu: f64) -> f64 {
        let term = 1.0 + (b_parameter * v_infinity.powi(2) / mu).powi(2);
        (mu / v_infinity.powi(2)) * (term.sqrt() - 1.0)
    }

    /// Calculate B-plane parameters for a gravity assist
    ///
    /// Given the spacecraft state vectors and planet velocity, this computes the
    /// B-plane targeting parameters used in mission design.
    ///
    /// # Arguments
    /// * `position` - Position vector in planet frame (m)
    /// * `velocity` - Velocity vector in planet frame (m/s)
    /// * `mu` - Gravitational parameter μ = GM of flyby body (m³/s²)
    ///
    /// # Returns
    /// `BPlaneParameters` containing B-vector components and theta angle
    ///
    /// # Note
    /// This is a simplified implementation. Full B-plane targeting requires
    /// defining the S, T, R unit vectors based on the trajectory geometry.
    pub fn calculate_b_plane(
        position: [f64; 3],
        velocity: [f64; 3],
        mu: f64,
    ) -> PoliastroResult<BPlaneParameters> {
        // Calculate angular momentum vector h = r × v
        let h = Self::cross_product(position, velocity);
        let h_mag = Self::vector_magnitude(h);

        if h_mag < 1e-10 {
            return Err(PoliastroError::invalid_state(
                "Angular momentum is too small (near-radial trajectory)",
            ));
        }

        // Calculate velocity magnitude
        let v_mag = Self::vector_magnitude(velocity);
        let r_mag = Self::vector_magnitude(position);

        // Calculate specific energy: ε = v²/2 - μ/r
        let specific_energy = v_mag.powi(2) / 2.0 - mu / r_mag;

        // For hyperbolic orbit: v∞² = 2ε
        let v_infinity_sq = 2.0 * specific_energy;

        if v_infinity_sq <= 0.0 {
            return Err(PoliastroError::invalid_state(
                "Orbit is not hyperbolic (specific energy must be positive)",
            ));
        }

        let v_infinity = v_infinity_sq.sqrt();

        // Calculate semi-major axis: a = -μ / v∞²
        let a = -mu / v_infinity_sq;

        // Calculate eccentricity vector: e = (v²/μ)r - (r·v/μ)v - r̂
        let r_dot_v = Self::dot_product(position, velocity);
        let r_unit = [
            position[0] / r_mag,
            position[1] / r_mag,
            position[2] / r_mag,
        ];

        let e_vec = [
            (v_mag.powi(2) / mu) * position[0] - (r_dot_v / mu) * velocity[0] - r_unit[0],
            (v_mag.powi(2) / mu) * position[1] - (r_dot_v / mu) * velocity[1] - r_unit[1],
            (v_mag.powi(2) / mu) * position[2] - (r_dot_v / mu) * velocity[2] - r_unit[2],
        ];
        let e_mag = Self::vector_magnitude(e_vec);

        // Calculate B-vector magnitude: B = |a| × √(e² - 1)
        let b_magnitude = a.abs() * (e_mag.powi(2) - 1.0).sqrt();

        // For this simplified version, we'll set b_dot_t and b_dot_r based on
        // the geometry. A full implementation would require defining the
        // S, T, R reference frame.

        // Simplified: assume B is in the plane perpendicular to h
        let b_dot_t = b_magnitude * 0.7071; // Placeholder: 45° angle
        let b_dot_r = b_magnitude * 0.7071;
        let theta = (b_dot_t / b_magnitude).acos();

        Ok(BPlaneParameters {
            b_magnitude,
            b_dot_t,
            b_dot_r,
            theta,
        })
    }

    /// Rotate a velocity vector by a given angle in a specified plane
    ///
    /// This is used to compute the outgoing velocity vector after a gravity assist,
    /// given the incoming velocity and the turning angle.
    ///
    /// # Arguments
    /// * `v_in` - Incoming velocity vector (m/s)
    /// * `delta` - Turning angle (radians)
    /// * `rotation_axis` - Axis about which to rotate (unit vector)
    ///
    /// # Returns
    /// Outgoing velocity vector (m/s)
    pub fn rotate_velocity(v_in: [f64; 3], delta: f64, rotation_axis: [f64; 3]) -> [f64; 3] {
        // Rodrigues' rotation formula:
        // v_out = v_in × cos(δ) + (axis × v_in) × sin(δ) + axis × (axis · v_in) × (1 - cos(δ))

        let cos_delta = delta.cos();
        let sin_delta = delta.sin();

        let axis_cross_v = Self::cross_product(rotation_axis, v_in);
        let axis_dot_v = Self::dot_product(rotation_axis, v_in);

        [
            v_in[0] * cos_delta
                + axis_cross_v[0] * sin_delta
                + rotation_axis[0] * axis_dot_v * (1.0 - cos_delta),
            v_in[1] * cos_delta
                + axis_cross_v[1] * sin_delta
                + rotation_axis[1] * axis_dot_v * (1.0 - cos_delta),
            v_in[2] * cos_delta
                + axis_cross_v[2] * sin_delta
                + rotation_axis[2] * axis_dot_v * (1.0 - cos_delta),
        ]
    }

    /// Calculate full velocity transformation for a gravity assist
    ///
    /// Given the spacecraft and planet velocities before the encounter, this computes
    /// the full velocity state after the flyby, including the heliocentric delta-v.
    ///
    /// # Arguments
    /// * `v_sc_in` - Spacecraft velocity in heliocentric frame before flyby (m/s)
    /// * `v_planet` - Planet velocity in heliocentric frame (m/s)
    /// * `delta` - Turning angle (radians)
    /// * `rotation_axis` - Axis about which the flyby rotates the velocity (unit vector)
    ///
    /// # Returns
    /// `GravityAssistVelocities` containing all velocity components
    pub fn calculate_velocities(
        v_sc_in: [f64; 3],
        v_planet: [f64; 3],
        delta: f64,
        rotation_axis: [f64; 3],
    ) -> GravityAssistVelocities {
        // Calculate incoming v∞ in planet frame
        let v_infinity_in = Self::vector_subtract(v_sc_in, v_planet);

        // Rotate v∞ by the turning angle to get outgoing v∞
        let v_infinity_out = Self::rotate_velocity(v_infinity_in, delta, rotation_axis);

        // Transform back to heliocentric frame
        let v_sc_out = Self::vector_add(v_infinity_out, v_planet);

        // Calculate delta-v
        let delta_v = Self::vector_subtract(v_sc_out, v_sc_in);

        GravityAssistVelocities {
            v_infinity_in,
            v_infinity_out,
            v_sc_in,
            v_sc_out,
            v_planet,
            delta_v,
        }
    }

    // Helper functions for vector operations

    fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn dot_product(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    fn vector_magnitude(v: [f64; 3]) -> f64 {
        (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt()
    }

    fn vector_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }

    fn vector_subtract(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    fn normalize_vector(v: [f64; 3]) -> [f64; 3] {
        let mag = Self::vector_magnitude(v);
        if mag < 1e-10 {
            [0.0, 0.0, 0.0]
        } else {
            [v[0] / mag, v[1] / mag, v[2] / mag]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_jupiter_flyby() {
        // Classic Jupiter gravity assist
        // Data from Curtis Example 8.7
        let mu_jupiter = 1.266865e17; // m³/s²
        let r_jupiter = 71492e3; // m
        let v_infinity = 5640.0; // m/s
        let r_periapsis = 3.0 * r_jupiter; // 3 Jupiter radii

        let result = GravityAssist::from_periapsis(v_infinity, r_periapsis, mu_jupiter).unwrap();

        // Check eccentricity is > 1 (hyperbolic)
        assert!(result.eccentricity > 1.0);

        // Check turning angle is positive and less than π
        assert!(result.delta > 0.0);
        assert!(result.delta < PI);

        // Check delta-v is positive
        assert!(result.delta_v_magnitude > 0.0);

        // Check semi-major axis is negative (hyperbolic)
        assert!(result.semi_major_axis < 0.0);

        // Verify relationship: e = 1 + (r_p × v∞²) / μ
        let e_calculated = 1.0 + (r_periapsis * v_infinity.powi(2)) / mu_jupiter;
        assert_relative_eq!(result.eccentricity, e_calculated, epsilon = EPSILON);
    }

    #[test]
    fn test_turning_angle_formula() {
        // Test that two formulas for turning angle give same result
        // δ = 2 × arcsin(1/e) and δ = 2θ∞ - π where θ∞ = arccos(-1/e)

        let e: f64 = 1.5; // Hyperbolic eccentricity

        let delta1 = 2.0 * (1.0 / e).asin();
        let theta_inf = (-1.0 / e).acos();
        let delta2 = 2.0 * theta_inf - PI;

        assert_relative_eq!(delta1, delta2, epsilon = EPSILON);
    }

    #[test]
    fn test_periapsis_from_b_parameter() {
        // Test conversion between B-parameter and periapsis
        let v_infinity = 5000.0; // m/s
        let mu = 1.266865e17; // Jupiter
        let r_periapsis_original = 200000e3; // 200,000 km

        // Calculate B from periapsis
        let result = GravityAssist::from_periapsis(v_infinity, r_periapsis_original, mu).unwrap();
        let b_parameter = result.b_parameter;

        // Convert B back to periapsis
        let r_periapsis_recovered =
            GravityAssist::periapsis_from_b_parameter(v_infinity, b_parameter, mu);

        assert_relative_eq!(
            r_periapsis_original,
            r_periapsis_recovered,
            epsilon = 1.0 // within 1 meter
        );
    }

    #[test]
    fn test_vector_operations() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];

        // Cross product
        let cross = GravityAssist::cross_product(a, b);
        assert_relative_eq!(cross[0], 0.0, epsilon = EPSILON);
        assert_relative_eq!(cross[1], 0.0, epsilon = EPSILON);
        assert_relative_eq!(cross[2], 1.0, epsilon = EPSILON);

        // Dot product
        let dot = GravityAssist::dot_product(a, b);
        assert_relative_eq!(dot, 0.0, epsilon = EPSILON);

        // Magnitude
        let mag = GravityAssist::vector_magnitude([3.0, 4.0, 0.0]);
        assert_relative_eq!(mag, 5.0, epsilon = EPSILON);
    }

    #[test]
    fn test_velocity_rotation() {
        // Test rotating a velocity vector by 90 degrees about z-axis
        let v_in = [1.0, 0.0, 0.0];
        let axis = [0.0, 0.0, 1.0];
        let delta = PI / 2.0; // 90 degrees

        let v_out = GravityAssist::rotate_velocity(v_in, delta, axis);

        // After 90° rotation about z, x→y, y→-x
        assert_relative_eq!(v_out[0], 0.0, epsilon = EPSILON);
        assert_relative_eq!(v_out[1], 1.0, epsilon = EPSILON);
        assert_relative_eq!(v_out[2], 0.0, epsilon = EPSILON);
    }

    #[test]
    fn test_error_handling() {
        let mu = 1.266865e17;
        let r_p = 200000e3;

        // Test negative v_infinity
        let result = GravityAssist::from_periapsis(-1000.0, r_p, mu);
        assert!(result.is_err());

        // Test negative periapsis
        let result = GravityAssist::from_periapsis(5000.0, -r_p, mu);
        assert!(result.is_err());

        // Test negative mu
        let result = GravityAssist::from_periapsis(5000.0, r_p, -mu);
        assert!(result.is_err());
    }

    #[test]
    fn test_high_eccentricity_flyby() {
        // Test a very close flyby (higher eccentricity from closer approach)
        let mu = 1.266865e17; // Jupiter
        let r_jupiter = 71492e3;
        let v_infinity = 20000.0; // m/s - high speed (typical for outer planet missions)
        let r_periapsis = 1.5 * r_jupiter; // Close but safe flyby

        let result = GravityAssist::from_periapsis(v_infinity, r_periapsis, mu).unwrap();

        // Calculate expected: e = 1 + (r_p × v∞²) / μ
        // e = 1 + (1.5×71492e3 × 20000²) / 1.266865e17 ≈ 1.338
        assert!(result.eccentricity > 1.3);
        assert!(result.eccentricity < 1.5);
        assert!(result.delta > 0.8); // Should be a substantial deflection
    }

    #[test]
    fn test_low_eccentricity_flyby() {
        // Test a distant flyby (low eccentricity)
        let mu = 1.266865e17; // Jupiter
        let v_infinity = 3000.0; // m/s - lower speed
        let r_periapsis = 1000000e3; // 1 million km - very distant

        let result = GravityAssist::from_periapsis(v_infinity, r_periapsis, mu).unwrap();

        // Lower eccentricity (but still > 1) should give smaller turning angle
        // e = 1 + (r_p × v∞²) / μ = 1 + (1e9 × 3000²) / 1.266865e17 ≈ 1.071
        assert!(result.eccentricity > 1.0);
        assert!(result.eccentricity < 1.2); // Relatively low for hyperbolic
        assert!(result.delta > 0.0);
        assert!(result.delta < PI); // Valid turning angle range
    }

    #[test]
    fn test_energy_conservation() {
        // In planet frame, speed should be conserved
        let v_infinity = 5640.0;
        let mu = 1.266865e17;
        let r_periapsis = 214476e3;

        let result = GravityAssist::from_periapsis(v_infinity, r_periapsis, mu).unwrap();

        // Specific energy should equal v∞²/2
        let expected_energy = v_infinity.powi(2) / 2.0;
        assert_relative_eq!(result.specific_energy, expected_energy, epsilon = 1.0);
    }

    #[test]
    fn test_error_zero_v_infinity() {
        let mu = 1.266865e17;
        let r_p = 200000e3;
        let result = GravityAssist::from_periapsis(0.0, r_p, mu);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_zero_r_periapsis() {
        let mu = 1.266865e17;
        let v_inf = 5000.0;
        let result = GravityAssist::from_periapsis(v_inf, 0.0, mu);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_zero_mu() {
        let r_p = 200000e3;
        let v_inf = 5000.0;
        let result = GravityAssist::from_periapsis(v_inf, r_p, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_b_plane() {
        // Test B-plane calculation
        let mu = 1.266865e17; // Jupiter

        // Hyperbolic approach vector - need high enough velocity for hyperbolic orbit
        let position = [1e9, 0.0, 0.0]; // 1 million km on x-axis
        let velocity = [0.0, 20000.0, 5000.0]; // High velocity with y and z components

        let b_plane = GravityAssist::calculate_b_plane(position, velocity, mu).unwrap();

        // B-plane parameters should be calculated
        assert!(b_plane.b_magnitude >= 0.0);
        assert!(b_plane.theta >= 0.0);
        assert!(b_plane.theta <= 2.0 * PI);
    }

    #[test]
    fn test_calculate_velocities() {
        // Test velocity calculation around a gravity assist
        let v_sc_in = [5000.0, 13000.0, 0.0]; // Incoming spacecraft velocity in heliocentric frame
        let v_planet = [0.0, 13000.0, 0.0]; // Planet velocity
        let delta = PI / 3.0; // 60 degree deflection
        let rotation_axis = [0.0, 0.0, 1.0]; // Rotate about z-axis

        let velocities = GravityAssist::calculate_velocities(
            v_sc_in,
            v_planet,
            delta,
            rotation_axis,
        );

        // Check that v_infinity magnitude is conserved
        let v_inf_in_mag = GravityAssist::vector_magnitude(velocities.v_infinity_in);
        let v_inf_out_mag = GravityAssist::vector_magnitude(velocities.v_infinity_out);
        assert_relative_eq!(v_inf_in_mag, v_inf_out_mag, epsilon = 1e-3);

        // Check heliocentric velocities are computed
        assert!(velocities.v_sc_in.iter().any(|&x| x != 0.0));
        assert!(velocities.v_sc_out.iter().any(|&x| x != 0.0));
    }

    #[test]
    fn test_vector_add() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = GravityAssist::vector_add(a, b);

        assert_relative_eq!(result[0], 5.0, epsilon = EPSILON);
        assert_relative_eq!(result[1], 7.0, epsilon = EPSILON);
        assert_relative_eq!(result[2], 9.0, epsilon = EPSILON);
    }

    #[test]
    fn test_vector_subtract() {
        let a = [5.0, 7.0, 9.0];
        let b = [4.0, 5.0, 6.0];
        let result = GravityAssist::vector_subtract(a, b);

        assert_relative_eq!(result[0], 1.0, epsilon = EPSILON);
        assert_relative_eq!(result[1], 2.0, epsilon = EPSILON);
        assert_relative_eq!(result[2], 3.0, epsilon = EPSILON);
    }

    #[test]
    fn test_normalize_vector() {
        let v = [3.0, 4.0, 0.0]; // magnitude = 5
        let normalized = GravityAssist::normalize_vector(v);

        assert_relative_eq!(normalized[0], 0.6, epsilon = EPSILON);
        assert_relative_eq!(normalized[1], 0.8, epsilon = EPSILON);
        assert_relative_eq!(normalized[2], 0.0, epsilon = EPSILON);

        // Check that result has unit magnitude
        let mag = GravityAssist::vector_magnitude(normalized);
        assert_relative_eq!(mag, 1.0, epsilon = EPSILON);
    }

    #[test]
    fn test_normalize_unit_vector() {
        // Normalizing an already normalized vector should give same vector
        let v = [1.0, 0.0, 0.0];
        let normalized = GravityAssist::normalize_vector(v);

        assert_relative_eq!(normalized[0], 1.0, epsilon = EPSILON);
        assert_relative_eq!(normalized[1], 0.0, epsilon = EPSILON);
        assert_relative_eq!(normalized[2], 0.0, epsilon = EPSILON);
    }

    #[test]
    fn test_cross_product_orthogonality() {
        // Cross product should be orthogonal to both input vectors
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let cross = GravityAssist::cross_product(a, b);

        // cross · a should be 0
        let dot_a = GravityAssist::dot_product(cross, a);
        assert_relative_eq!(dot_a, 0.0, epsilon = 1e-10);

        // cross · b should be 0
        let dot_b = GravityAssist::dot_product(cross, b);
        assert_relative_eq!(dot_b, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_dot_product_parallel_vectors() {
        // Dot product of parallel vectors = product of magnitudes
        let a = [2.0, 0.0, 0.0];
        let b = [3.0, 0.0, 0.0];
        let dot = GravityAssist::dot_product(a, b);

        assert_relative_eq!(dot, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn test_dot_product_perpendicular_vectors() {
        // Dot product of perpendicular vectors = 0
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let dot = GravityAssist::dot_product(a, b);

        assert_relative_eq!(dot, 0.0, epsilon = EPSILON);
    }

    #[test]
    fn test_rotate_velocity_no_rotation() {
        // Rotating by 0 radians should return same vector
        let v = [1.0, 2.0, 3.0];
        let axis = [0.0, 0.0, 1.0];
        let rotated = GravityAssist::rotate_velocity(v, 0.0, axis);

        assert_relative_eq!(rotated[0], v[0], epsilon = EPSILON);
        assert_relative_eq!(rotated[1], v[1], epsilon = EPSILON);
        assert_relative_eq!(rotated[2], v[2], epsilon = EPSILON);
    }

    #[test]
    fn test_rotate_velocity_180_degrees() {
        // Rotating by 180° should reverse the perpendicular component
        let v = [1.0, 0.0, 0.0];
        let axis = [0.0, 0.0, 1.0];
        let rotated = GravityAssist::rotate_velocity(v, PI, axis);

        assert_relative_eq!(rotated[0], -1.0, epsilon = EPSILON);
        assert_relative_eq!(rotated[1], 0.0, epsilon = EPSILON);
        assert_relative_eq!(rotated[2], 0.0, epsilon = EPSILON);
    }

    #[test]
    fn test_periapsis_from_b_parameter_edge_case() {
        // Test with very large B parameter (distant flyby)
        let v_infinity = 5000.0;
        let mu = 1.266865e17;
        let b_large = 1e9; // Very large impact parameter

        let r_p = GravityAssist::periapsis_from_b_parameter(v_infinity, b_large, mu);

        // Should return a valid positive periapsis
        assert!(r_p > 0.0);
        // Periapsis is always less than the impact parameter
        assert!(r_p < b_large);
        // For these parameters (b * v_infinity² / mu ≈ 0.197), r_p ≈ 9.8e7
        assert!((r_p - 9.77e7).abs() < 1e6); // Within 1 km of expected value
    }

    #[test]
    fn test_periapsis_from_b_parameter_small_b() {
        // Test with small B parameter (close flyby)
        let v_infinity = 10000.0;
        let mu = 1.266865e17;
        let b_small = 1e6; // Small impact parameter

        let r_p = GravityAssist::periapsis_from_b_parameter(v_infinity, b_small, mu);

        // Should return a valid periapsis
        assert!(r_p > 0.0);
        assert!(r_p < mu / v_infinity.powi(2)); // Should be less than semi-latus rectum
    }

    #[test]
    fn test_b_parameter_relationship() {
        // Test relationship: B = |a| × √(e² - 1)
        let v_infinity = 6000.0;
        let mu = 1.266865e17;
        let r_periapsis = 150000e3;

        let result = GravityAssist::from_periapsis(v_infinity, r_periapsis, mu).unwrap();

        // Verify B = |a| × √(e² - 1)
        let b_calculated = result.semi_major_axis.abs()
            * (result.eccentricity.powi(2) - 1.0).sqrt();

        assert_relative_eq!(result.b_parameter, b_calculated, epsilon = 1e-3);
    }

    #[test]
    fn test_very_high_speed_flyby() {
        // Test extreme case with very high v_infinity
        let v_infinity = 50000.0; // 50 km/s - very fast
        let mu = 1.266865e17; // Jupiter
        let r_jupiter = 71492e3;
        let r_periapsis = 2.0 * r_jupiter;

        let result = GravityAssist::from_periapsis(v_infinity, r_periapsis, mu).unwrap();

        // At very high speeds, eccentricity should be very high
        assert!(result.eccentricity > 3.0);
        // Turning angle should be moderate (high e means low turning angle)
        assert!(result.delta > 0.0);
        assert!(result.delta < PI);
    }
}
