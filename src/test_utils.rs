/// Common test utilities and property-based testing support for Astrora
///
/// This module provides shared testing infrastructure for unit tests across the codebase,
/// including:
/// - Floating-point comparison utilities with appropriate tolerances
/// - Property-based testing strategies for orbital mechanics
/// - Test data generators for common scenarios
/// - Invariant checkers for physical conservation laws
///
/// # Usage
///
/// ```rust,ignore
/// #[cfg(test)]
/// mod tests {
///     use crate::test_utils::*;
///
///     #[test]
///     fn test_orbital_calculation() {
///         proptest!(|(state in orbital_state_strategy())| {
///             // Test that energy is conserved
///             assert_energy_conserved(&state, tolerance_energy());
///         });
///     }
/// }
/// ```

use approx::{assert_relative_eq, relative_eq};
use nalgebra::{Vector3, Vector6};
use proptest::prelude::*;

// ============================================================================
// Tolerance Constants
// ============================================================================

/// Standard position tolerance in meters for orbital calculations
pub const POSITION_TOLERANCE_M: f64 = 1.0; // 1 meter

/// Standard velocity tolerance in m/s for orbital calculations
pub const VELOCITY_TOLERANCE_MS: f64 = 0.001; // 1 mm/s

/// Standard energy tolerance (specific energy in J/kg)
pub const ENERGY_TOLERANCE_JKG: f64 = 1.0; // 1 J/kg

/// Standard angular momentum tolerance (m²/s)
pub const ANGULAR_MOMENTUM_TOLERANCE: f64 = 1e-6; // 1e-6 m²/s

/// Standard angle tolerance in radians
pub const ANGLE_TOLERANCE_RAD: f64 = 1e-10; // ~1e-8 degrees

/// Relative tolerance for floating-point comparisons
pub const RELATIVE_TOLERANCE: f64 = 1e-12;

// ============================================================================
// Physical Constants for Testing
// ============================================================================

/// Earth's gravitational parameter (km³/s²) - for test reference
pub const GM_EARTH_TEST: f64 = 398600.4418;

/// Earth's radius (km) - for test reference
pub const R_EARTH_TEST: f64 = 6378.137;

/// Minimum safe orbital radius (km) - above atmosphere
pub const MIN_ORBITAL_RADIUS: f64 = 6578.0; // ~200 km altitude

/// Maximum practical orbital radius (km) - below GEO escape
pub const MAX_ORBITAL_RADIUS: f64 = 50000.0;

// ============================================================================
// Floating-Point Comparison Utilities
// ============================================================================

/// Compare two floating-point values with relative tolerance
pub fn assert_float_eq(a: f64, b: f64, epsilon: f64) {
    assert_relative_eq!(a, b, epsilon = epsilon, max_relative = epsilon);
}

/// Compare two vectors with element-wise relative tolerance
pub fn assert_vector3_eq(a: &Vector3<f64>, b: &Vector3<f64>, epsilon: f64) {
    for i in 0..3 {
        assert_relative_eq!(a[i], b[i], epsilon = epsilon, max_relative = epsilon);
    }
}

/// Compare two state vectors (6D) with element-wise relative tolerance
pub fn assert_vector6_eq(a: &Vector6<f64>, b: &Vector6<f64>, epsilon: f64) {
    for i in 0..6 {
        assert_relative_eq!(a[i], b[i], epsilon = epsilon, max_relative = epsilon);
    }
}

/// Check if two floats are approximately equal (returns bool)
pub fn floats_equal(a: f64, b: f64, epsilon: f64) -> bool {
    relative_eq!(a, b, epsilon = epsilon, max_relative = epsilon)
}

// ============================================================================
// Property-Based Testing Strategies
// ============================================================================

/// Strategy for generating valid orbital radii (km)
pub fn orbital_radius_strategy() -> impl Strategy<Value = f64> {
    (MIN_ORBITAL_RADIUS..MAX_ORBITAL_RADIUS)
}

/// Strategy for generating valid eccentricities (elliptical orbits)
pub fn eccentricity_elliptical_strategy() -> impl Strategy<Value = f64> {
    (0.0..0.9) // Avoid near-parabolic orbits
}

/// Strategy for generating valid eccentricities (all conics, excluding parabolic)
pub fn eccentricity_all_strategy() -> impl Strategy<Value = f64> {
    prop_oneof![
        (0.0..0.95), // Elliptical
        (1.05..3.0), // Hyperbolic
    ]
}

/// Strategy for generating valid inclinations in radians
pub fn inclination_strategy() -> impl Strategy<Value = f64> {
    (0.0..std::f64::consts::PI)
}

/// Strategy for generating valid angles (0 to 2π)
pub fn angle_strategy() -> impl Strategy<Value = f64> {
    (0.0..std::f64::consts::TAU)
}

/// Strategy for generating positive time intervals in seconds
pub fn time_interval_strategy() -> impl Strategy<Value = f64> {
    (1.0..86400.0) // 1 second to 1 day
}

/// Strategy for generating small positive time steps (for numerical integration)
pub fn small_timestep_strategy() -> impl Strategy<Value = f64> {
    (1.0..600.0) // 1 second to 10 minutes
}

/// Strategy for generating reasonable velocity magnitudes (km/s)
pub fn velocity_magnitude_strategy() -> impl Strategy<Value = f64> {
    (1.0..15.0) // Typical orbital velocities
}

/// Strategy for generating 3D position vectors in orbital regime
pub fn position_vector_strategy() -> impl Strategy<Value = Vector3<f64>> {
    (orbital_radius_strategy(), angle_strategy(), angle_strategy()).prop_map(|(r, theta, phi)| {
        Vector3::new(
            r * theta.sin() * phi.cos(),
            r * theta.sin() * phi.sin(),
            r * theta.cos(),
        )
    })
}

/// Strategy for generating 3D velocity vectors with reasonable magnitudes
pub fn velocity_vector_strategy() -> impl Strategy<Value = Vector3<f64>> {
    (
        velocity_magnitude_strategy(),
        angle_strategy(),
        angle_strategy(),
    )
        .prop_map(|(v, theta, phi)| {
            Vector3::new(
                v * theta.sin() * phi.cos(),
                v * theta.sin() * phi.sin(),
                v * theta.cos(),
            )
        })
}

/// Strategy for generating complete orbital states (position + velocity)
pub fn orbital_state_strategy() -> impl Strategy<Value = (Vector3<f64>, Vector3<f64>)> {
    // Generate physically valid orbital states
    (
        orbital_radius_strategy(),
        eccentricity_elliptical_strategy(),
        inclination_strategy(),
        angle_strategy(), // RAAN
        angle_strategy(), // arg of periapsis
        angle_strategy(), // true anomaly
    )
        .prop_map(|(a, e, inc, raan, omega, nu)| {
            // Convert orbital elements to state vectors
            // This is a simplified conversion for testing purposes
            let p = a * (1.0 - e * e);
            let r_mag = p / (1.0 + e * nu.cos());
            let h = (GM_EARTH_TEST * p).sqrt();

            // Position in orbital plane
            let r_pqw = Vector3::new(r_mag * nu.cos(), r_mag * nu.sin(), 0.0);

            // Velocity in orbital plane
            let v_pqw = Vector3::new(
                -(GM_EARTH_TEST / h) * nu.sin(),
                (GM_EARTH_TEST / h) * (e + nu.cos()),
                0.0,
            );

            // Rotation matrices (simplified for testing)
            let cos_raan = raan.cos();
            let sin_raan = raan.sin();
            let cos_inc = inc.cos();
            let sin_inc = inc.sin();
            let cos_omega = omega.cos();
            let sin_omega = omega.sin();

            // Transform to inertial frame (simplified)
            let r = Vector3::new(
                (cos_raan * cos_omega - sin_raan * sin_omega * cos_inc) * r_pqw.x
                    - (cos_raan * sin_omega + sin_raan * cos_omega * cos_inc) * r_pqw.y,
                (sin_raan * cos_omega + cos_raan * sin_omega * cos_inc) * r_pqw.x
                    - (sin_raan * sin_omega - cos_raan * cos_omega * cos_inc) * r_pqw.y,
                sin_inc * sin_omega * r_pqw.x + sin_inc * cos_omega * r_pqw.y,
            );

            let v = Vector3::new(
                (cos_raan * cos_omega - sin_raan * sin_omega * cos_inc) * v_pqw.x
                    - (cos_raan * sin_omega + sin_raan * cos_omega * cos_inc) * v_pqw.y,
                (sin_raan * cos_omega + cos_raan * sin_omega * cos_inc) * v_pqw.x
                    - (sin_raan * sin_omega - cos_raan * cos_omega * cos_inc) * v_pqw.y,
                sin_inc * sin_omega * v_pqw.x + sin_inc * cos_omega * v_pqw.y,
            );

            (r, v)
        })
}

// ============================================================================
// Physical Invariant Checkers
// ============================================================================

/// Check that specific orbital energy is conserved
///
/// Specific orbital energy: ε = v²/2 - μ/r
pub fn check_energy_conserved(
    r0: &Vector3<f64>,
    v0: &Vector3<f64>,
    r1: &Vector3<f64>,
    v1: &Vector3<f64>,
    mu: f64,
    tolerance: f64,
) -> bool {
    let energy0 = v0.norm_squared() / 2.0 - mu / r0.norm();
    let energy1 = v1.norm_squared() / 2.0 - mu / r1.norm();

    floats_equal(energy0, energy1, tolerance)
}

/// Check that angular momentum is conserved
///
/// Angular momentum: h = r × v
pub fn check_angular_momentum_conserved(
    r0: &Vector3<f64>,
    v0: &Vector3<f64>,
    r1: &Vector3<f64>,
    v1: &Vector3<f64>,
    tolerance: f64,
) -> bool {
    let h0 = r0.cross(v0);
    let h1 = r1.cross(v1);

    let diff = (h0 - h1).norm();
    diff < tolerance
}

/// Check that semi-major axis is consistent with orbital energy
///
/// For elliptical orbits: a = -μ/(2ε)
pub fn check_semimajor_axis_energy_consistency(
    r: &Vector3<f64>,
    v: &Vector3<f64>,
    a_expected: f64,
    mu: f64,
    tolerance: f64,
) -> bool {
    let energy = v.norm_squared() / 2.0 - mu / r.norm();

    // For elliptical orbits (energy < 0)
    if energy < 0.0 {
        let a_from_energy = -mu / (2.0 * energy);
        floats_equal(a_from_energy, a_expected, tolerance)
    } else {
        // Parabolic or hyperbolic - skip this check
        true
    }
}

/// Check that eccentricity vector magnitude matches expected eccentricity
pub fn check_eccentricity_vector(
    r: &Vector3<f64>,
    v: &Vector3<f64>,
    e_expected: f64,
    mu: f64,
    tolerance: f64,
) -> bool {
    let h = r.cross(v);
    let e_vec = v.cross(&h) / mu - r.normalize();
    let e_calc = e_vec.norm();

    floats_equal(e_calc, e_expected, tolerance)
}

// ============================================================================
// Test Data Generators
// ============================================================================

/// Generate a circular orbit at given radius
pub fn circular_orbit(r_mag: f64, mu: f64) -> (Vector3<f64>, Vector3<f64>) {
    let r = Vector3::new(r_mag, 0.0, 0.0);
    let v_mag = (mu / r_mag).sqrt();
    let v = Vector3::new(0.0, v_mag, 0.0);
    (r, v)
}

/// Generate an equatorial circular orbit
pub fn equatorial_circular_orbit(altitude_km: f64) -> (Vector3<f64>, Vector3<f64>) {
    let r_mag = R_EARTH_TEST + altitude_km;
    circular_orbit(r_mag, GM_EARTH_TEST)
}

/// Generate a polar circular orbit
pub fn polar_circular_orbit(altitude_km: f64) -> (Vector3<f64>, Vector3<f64>) {
    let r_mag = R_EARTH_TEST + altitude_km;
    let v_mag = (GM_EARTH_TEST / r_mag).sqrt();
    let r = Vector3::new(r_mag, 0.0, 0.0);
    let v = Vector3::new(0.0, 0.0, v_mag); // Velocity in Z direction
    (r, v)
}

/// Generate a highly eccentric orbit (e.g., Molniya)
pub fn eccentric_orbit(
    perigee_altitude_km: f64,
    apogee_altitude_km: f64,
) -> (Vector3<f64>, Vector3<f64>) {
    let r_p = R_EARTH_TEST + perigee_altitude_km;
    let r_a = R_EARTH_TEST + apogee_altitude_km;
    let a = (r_p + r_a) / 2.0;
    let e = (r_a - r_p) / (r_a + r_p);

    // Start at periapsis
    let r = Vector3::new(r_p, 0.0, 0.0);
    let v_mag = ((2.0 * GM_EARTH_TEST / r_p) - (GM_EARTH_TEST / a)).sqrt();
    let v = Vector3::new(0.0, v_mag, 0.0);
    (r, v)
}

// ============================================================================
// Orbit Classification Helpers
// ============================================================================

/// Determine if an orbit is circular (e < threshold)
pub fn is_circular(e: f64, threshold: f64) -> bool {
    e < threshold
}

/// Determine if an orbit is elliptical
pub fn is_elliptical(e: f64) -> bool {
    e >= 0.0 && e < 1.0
}

/// Determine if an orbit is parabolic (within tolerance)
pub fn is_parabolic(e: f64, tolerance: f64) -> bool {
    (e - 1.0).abs() < tolerance
}

/// Determine if an orbit is hyperbolic
pub fn is_hyperbolic(e: f64) -> bool {
    e > 1.0
}

/// Determine if an orbit is equatorial (i ≈ 0 or π)
pub fn is_equatorial(inclination_rad: f64, tolerance: f64) -> bool {
    inclination_rad < tolerance || (inclination_rad - std::f64::consts::PI).abs() < tolerance
}

/// Determine if an orbit is polar (i ≈ π/2)
pub fn is_polar(inclination_rad: f64, tolerance: f64) -> bool {
    (inclination_rad - std::f64::consts::PI / 2.0).abs() < tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_comparison() {
        assert_float_eq(1.0, 1.0 + 1e-13, RELATIVE_TOLERANCE);
        assert!(floats_equal(1.0, 1.0 + 1e-13, RELATIVE_TOLERANCE));
    }

    #[test]
    fn test_circular_orbit_energy() {
        let (r, v) = circular_orbit(7000.0, GM_EARTH_TEST);
        let energy = v.norm_squared() / 2.0 - GM_EARTH_TEST / r.norm();
        let expected_energy = -GM_EARTH_TEST / (2.0 * 7000.0);
        assert_float_eq(energy, expected_energy, RELATIVE_TOLERANCE);
    }

    #[test]
    fn test_angular_momentum_circular() {
        let (r, v) = circular_orbit(7000.0, GM_EARTH_TEST);
        let h = r.cross(&v);
        let expected_h = (GM_EARTH_TEST * 7000.0).sqrt();
        assert_float_eq(h.norm(), expected_h, RELATIVE_TOLERANCE);
    }

    #[test]
    fn test_orbit_classification() {
        assert!(is_circular(0.001, 0.01));
        assert!(is_elliptical(0.5));
        assert!(is_hyperbolic(1.5));
        assert!(is_parabolic(1.0, 0.1));
        assert!(is_equatorial(0.001, 0.01));
        assert!(is_polar(std::f64::consts::PI / 2.0, 0.01));
    }

    proptest! {
        #[test]
        fn test_circular_orbit_property(radius in 6500.0..50000.0) {
            let (r, v) = circular_orbit(radius, GM_EARTH_TEST);

            // Check that r and v are perpendicular
            let dot = r.dot(&v);
            assert!(dot.abs() < 1e-10);

            // Check circular velocity formula
            let v_expected = (GM_EARTH_TEST / radius).sqrt();
            assert!(floats_equal(v.norm(), v_expected, 1e-12));
        }

        #[test]
        fn test_energy_conservation_property(
            (r0, v0) in orbital_state_strategy(),
            dt in small_timestep_strategy()
        ) {
            // Property: Energy should be constant (we're not propagating, just checking calculation)
            let energy = v0.norm_squared() / 2.0 - GM_EARTH_TEST / r0.norm();

            // Energy for elliptical orbits should be negative
            if r0.norm() < MAX_ORBITAL_RADIUS {
                assert!(energy < 0.0);
            }
        }

        #[test]
        fn test_angular_momentum_property((r, v) in orbital_state_strategy()) {
            let h = r.cross(&v);

            // Angular momentum should be non-zero for non-rectilinear orbits
            assert!(h.norm() > 1e-6);

            // h should be perpendicular to both r and v
            assert!(h.dot(&r).abs() < 1e-6);
            assert!(h.dot(&v).abs() < 1e-6);
        }
    }
}
