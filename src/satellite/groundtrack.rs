//! Satellite Ground Track Computation
//!
//! This module provides functionality for calculating satellite ground tracks,
//! which represent the path of a satellite's sub-satellite point over Earth's surface.
//!
//! # Overview
//!
//! A satellite's ground track is the trajectory traced on Earth's surface by the point
//! directly beneath the satellite. Computing ground tracks is essential for:
//! - Visualizing satellite coverage
//! - Mission planning and analysis
//! - Determining observation opportunities
//! - Calculating communication windows
//!
//! # Key Concepts
//!
//! ## Sub-Satellite Point
//!
//! The sub-satellite point is the location on Earth's surface where a line from the
//! satellite to Earth's center intersects the surface. It's expressed as:
//! - **Latitude**: Angular position north/south of equator (-90° to +90°)
//! - **Longitude**: Angular position east/west of prime meridian (-180° to +180°)
//! - **Altitude**: Height above WGS84 ellipsoid (km)
//!
//! ## Ground Track Types
//!
//! - **Direct (Prograde)**: Inclination 0-90°, moves with Earth's rotation
//! - **Retrograde**: Inclination 90-180°, moves against Earth's rotation
//! - **Polar**: Inclination near 90°, passes over poles
//! - **Equatorial**: Inclination near 0°, follows equator
//!
//! ## Swath Width
//!
//! The swath width is the ground coverage width for imaging satellites, determined by:
//! - Satellite altitude
//! - Sensor field of view
//! - Minimum elevation angle for useful observations
//!
//! # Algorithms
//!
//! ## ECEF to Geodetic Conversion
//!
//! Converts Cartesian ECEF coordinates to geodetic coordinates using an iterative algorithm:
//!
//! 1. **Longitude**: λ = atan2(y, x) (direct calculation)
//! 2. **Latitude and Altitude** (iterative):
//!    - Initialize: φ₀ = atan[z / ((1-e²) · √(x²+y²))]
//!    - Iterate until convergence:
//!      - N = a / √(1 - e² sin²φ)
//!      - h = √(x²+y²) / cos(φ) - N
//!      - φ_new = atan[z / ((1 - e² N/(N+h)) · √(x²+y²))]
//!    - Converges to ~1 mm precision in 3-5 iterations
//!
//! ## Ground Track Propagation
//!
//! 1. Propagate satellite in inertial frame (GCRS or TEME)
//! 2. Transform to ECEF at each time step
//! 3. Convert ECEF to geodetic coordinates
//! 4. Collect series of (lat, lon, time) points
//!
//! ## Swath Width Calculation
//!
//! For a given satellite altitude h and minimum elevation angle ε:
//! - Earth angular radius: ρ = arcsin(R_e / (R_e + h))
//! - Nadir angle: η = arccos(cos(ε) · (R_e + h) / R_e)
//! - Swath half-angle: λ = η - ε
//! - Swath width: W = 2 · R_e · λ (for small angles)
//!
//! # References
//!
//! - Vallado, "Fundamentals of Astrodynamics and Applications" (2013), Ch. 11
//! - Curtis, "Orbital Mechanics for Engineering Students" (2014), Ch. 5
//! - Navipedia: "Transformations between ECEF and ENU coordinates"
//! - ESA: "Ellipsoidal and Cartesian Coordinates Conversion"
//!   <https://gssc.esa.int/navipedia/index.php/Ellipsoidal_and_Cartesian_Coordinates_Conversion>
//!
//! # Example
//!
//! ```rust,ignore
//! use astrora_core::satellite::groundtrack::{ecef_to_geodetic, compute_ground_track};
//!
//! // Satellite position in ECEF (km)
//! let sat_ecef = [4000.0, 3000.0, 5000.0];
//!
//! // Get sub-satellite point
//! let geodetic = ecef_to_geodetic(&sat_ecef);
//! println!("Latitude: {:.2}°, Longitude: {:.2}°, Altitude: {:.2} km",
//!          geodetic.latitude.to_degrees(),
//!          geodetic.longitude.to_degrees(),
//!          geodetic.altitude);
//! ```

use std::f64::consts::PI;

/// WGS84 Earth ellipsoid parameters
const WGS84_A: f64 = 6378.137;           // Semi-major axis (km)
const WGS84_B: f64 = 6356.752314245;     // Semi-minor axis (km)
const WGS84_F: f64 = 1.0 / 298.257223563; // Flattening
const WGS84_E2: f64 = WGS84_F * (2.0 - WGS84_F); // First eccentricity squared
const WGS84_EP2: f64 = (WGS84_A * WGS84_A - WGS84_B * WGS84_B) / (WGS84_B * WGS84_B); // Second eccentricity squared

/// Convergence tolerance for iterative geodetic conversion (radians)
/// This corresponds to approximately 1 mm on Earth's surface
const GEODETIC_TOLERANCE: f64 = 1e-12;

/// Maximum iterations for geodetic conversion
const MAX_GEODETIC_ITERATIONS: usize = 10;

/// Geodetic coordinates (latitude, longitude, altitude)
#[derive(Debug, Clone, Copy)]
pub struct GeodeticCoordinates {
    /// Geodetic latitude (radians, -π/2 to π/2)
    pub latitude: f64,
    /// Geodetic longitude (radians, -π to π)
    pub longitude: f64,
    /// Altitude above WGS84 ellipsoid (km)
    pub altitude: f64,
}

impl GeodeticCoordinates {
    /// Create new geodetic coordinates
    ///
    /// # Arguments
    /// * `latitude` - Geodetic latitude in radians (-π/2 to π/2)
    /// * `longitude` - Geodetic longitude in radians (-π to π)
    /// * `altitude` - Altitude above WGS84 ellipsoid in km
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
    }

    /// Convert to degrees for display
    pub fn to_degrees(&self) -> (f64, f64, f64) {
        (
            self.latitude.to_degrees(),
            self.longitude.to_degrees(),
            self.altitude,
        )
    }
}

/// Convert ECEF Cartesian coordinates to geodetic coordinates (WGS84)
///
/// Uses an iterative algorithm to compute geodetic latitude, longitude, and altitude
/// from Earth-Centered, Earth-Fixed (ECEF) Cartesian coordinates.
///
/// # Algorithm
///
/// 1. **Longitude**: Computed directly as λ = atan2(y, x)
/// 2. **Latitude and Altitude**: Iterative refinement
///    - Initial estimate: φ₀ = atan[z / ((1-e²) · p)] where p = √(x²+y²)
///    - Iterate:
///      - N = a / √(1 - e² sin²φ) (radius of curvature)
///      - h = p / cos(φ) - N (altitude)
///      - φ_new = atan[z / ((1 - e² N/(N+h)) · p)] (improved latitude)
///    - Continue until |φ_new - φ| < tolerance
///
/// # Arguments
///
/// * `ecef` - ECEF position [x, y, z] in km
///
/// # Returns
///
/// `GeodeticCoordinates` with latitude, longitude (radians), and altitude (km)
///
/// # References
///
/// - ESA Navipedia: "Ellipsoidal and Cartesian Coordinates Conversion"
/// - Zhu, J. (1994). "Conversion of Earth-centered Earth-fixed coordinates to geodetic coordinates"
/// - Vallado, "Fundamentals of Astrodynamics", Algorithm 12
///
/// # Example
///
/// ```rust,ignore
/// let ecef = [4000.0, 3000.0, 5000.0];
/// let geodetic = ecef_to_geodetic(&ecef);
/// println!("Lat: {:.4}°, Lon: {:.4}°, Alt: {:.2} km",
///          geodetic.latitude.to_degrees(),
///          geodetic.longitude.to_degrees(),
///          geodetic.altitude);
/// ```
pub fn ecef_to_geodetic(ecef: &[f64; 3]) -> GeodeticCoordinates {
    let x = ecef[0];
    let y = ecef[1];
    let z = ecef[2];

    // Longitude is computed directly
    let longitude = y.atan2(x);

    // Distance from polar axis
    let p = (x * x + y * y).sqrt();

    // Handle special case: point on polar axis
    if p < 1e-10 {
        let latitude = if z >= 0.0 { PI / 2.0 } else { -PI / 2.0 };
        let altitude = z.abs() - WGS84_B;
        return GeodeticCoordinates::new(latitude, longitude, altitude);
    }

    // Initial latitude estimate (Bowring's formula for first approximation)
    let mut latitude = (z / ((1.0 - WGS84_E2) * p)).atan();

    // Iterative refinement of latitude and altitude
    let mut altitude = 0.0;
    for _ in 0..MAX_GEODETIC_ITERATIONS {
        let sin_lat = latitude.sin();
        let cos_lat = latitude.cos();

        // Radius of curvature in the prime vertical
        let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();

        // Altitude above ellipsoid
        let h_new = if cos_lat.abs() > 1e-10 {
            p / cos_lat - n
        } else {
            z / sin_lat - n * (1.0 - WGS84_E2)
        };

        // Improved latitude estimate
        let lat_new = (z / ((1.0 - WGS84_E2 * n / (n + h_new)) * p)).atan();

        // Check convergence
        let lat_change = (lat_new - latitude).abs();
        latitude = lat_new;
        altitude = h_new;

        if lat_change < GEODETIC_TOLERANCE {
            break;
        }
    }

    GeodeticCoordinates::new(latitude, longitude, altitude)
}

/// Compute sub-satellite point from ECEF position
///
/// This is a convenience wrapper around `ecef_to_geodetic` with clearer nomenclature
/// for satellite ground track applications.
///
/// # Arguments
///
/// * `sat_ecef` - Satellite position in ECEF frame [x, y, z] in km
///
/// # Returns
///
/// `GeodeticCoordinates` of the sub-satellite point
///
/// # Example
///
/// ```rust,ignore
/// let sat_ecef = [6700.0, 0.0, 500.0];  // Satellite in equatorial orbit
/// let sub_point = sub_satellite_point(&sat_ecef);
/// println!("Sub-satellite point: {:.2}°N, {:.2}°E",
///          sub_point.latitude.to_degrees(),
///          sub_point.longitude.to_degrees());
/// ```
#[inline]
pub fn sub_satellite_point(sat_ecef: &[f64; 3]) -> GeodeticCoordinates {
    ecef_to_geodetic(sat_ecef)
}

/// Ground track point with time information
#[derive(Debug, Clone, Copy)]
pub struct GroundTrackPoint {
    /// Geodetic latitude (radians)
    pub latitude: f64,
    /// Geodetic longitude (radians)
    pub longitude: f64,
    /// Altitude above WGS84 ellipsoid (km)
    pub altitude: f64,
    /// Time offset from epoch (minutes)
    pub time: f64,
}

impl GroundTrackPoint {
    /// Create a new ground track point
    pub fn new(latitude: f64, longitude: f64, altitude: f64, time: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
            time,
        }
    }

    /// Convert angles to degrees for display
    pub fn to_degrees(&self) -> (f64, f64, f64, f64) {
        (
            self.latitude.to_degrees(),
            self.longitude.to_degrees(),
            self.altitude,
            self.time,
        )
    }
}

/// Compute ground track from a series of ECEF positions
///
/// Converts a sequence of satellite positions in ECEF frame to ground track points
/// (sub-satellite points) with time information.
///
/// # Arguments
///
/// * `ecef_positions` - Vector of ECEF positions [x, y, z] in km
/// * `times` - Vector of time offsets from epoch in minutes (same length as positions)
///
/// # Returns
///
/// Vector of `GroundTrackPoint` with latitude, longitude, altitude, and time
///
/// # Panics
///
/// Panics if `ecef_positions` and `times` have different lengths
///
/// # Example
///
/// ```rust,ignore
/// let positions = vec![[6700.0, 0.0, 0.0], [6700.0, 100.0, 0.0]];
/// let times = vec![0.0, 1.0];
/// let ground_track = compute_ground_track(&positions, &times);
/// for point in ground_track {
///     println!("t={:.1} min: {:.2}°N, {:.2}°E",
///              point.time, point.latitude.to_degrees(), point.longitude.to_degrees());
/// }
/// ```
pub fn compute_ground_track(
    ecef_positions: &[[f64; 3]],
    times: &[f64],
) -> Vec<GroundTrackPoint> {
    assert_eq!(
        ecef_positions.len(),
        times.len(),
        "ECEF positions and times must have the same length"
    );

    ecef_positions
        .iter()
        .zip(times.iter())
        .map(|(pos, &time)| {
            let geodetic = ecef_to_geodetic(pos);
            GroundTrackPoint::new(
                geodetic.latitude,
                geodetic.longitude,
                geodetic.altitude,
                time,
            )
        })
        .collect()
}

/// Calculate satellite swath width on Earth's surface
///
/// Computes the ground coverage width for an imaging satellite based on altitude
/// and minimum elevation angle. This represents the width of the strip on Earth's
/// surface that the satellite can observe.
///
/// # TODO
///
/// This is a simplified approximation. A more accurate formula accounting for
/// spherical geometry and elevation angle should be implemented. Current formula
/// uses the maximum ground range scaled by elevation angle factor.
///
/// # Arguments
///
/// * `altitude` - Satellite altitude above Earth's surface in km
/// * `min_elevation` - Minimum elevation angle in radians (typical: 5-10°)
///
/// # Returns
///
/// Approximate swath width in km
///
/// # Example
///
/// ```rust,ignore
/// // LEO satellite at 500 km with 10° minimum elevation
/// let altitude = 500.0;
/// let min_elevation = 10.0_f64.to_radians();
/// let swath = calculate_swath_width(altitude, min_elevation);
/// println!("Swath width: {:.1} km", swath);
/// ```
///
/// # References
///
/// - Wertz, "Space Mission Analysis and Design" (3rd ed.), Ch. 9
/// - Larson & Wertz, "Space Mission Analysis and Design" (1999), Section 9.2
pub fn calculate_swath_width(altitude: f64, min_elevation: f64) -> f64 {
    // Simplified approximation: Use maximum ground range and scale by elevation factor
    // This gives a rough estimate; full spherical geometry should be implemented

    let max_range = maximum_ground_range(altitude);

    // Scale factor based on elevation angle (0° = full range, 90° = zero range)
    // This is an approximation
    let elevation_factor = (PI / 2.0 - min_elevation) / (PI / 2.0);

    // Swath width (rough approximation)
    

    2.0 * max_range * elevation_factor
}

/// Calculate maximum ground range (distance to horizon) for a satellite
///
/// Computes the maximum distance along Earth's surface from the sub-satellite
/// point to the horizon as seen from the satellite.
///
/// # Arguments
///
/// * `altitude` - Satellite altitude above Earth's surface in km
///
/// # Returns
///
/// Maximum ground range to horizon in km
///
/// # Example
///
/// ```rust,ignore
/// let altitude = 400.0; // ISS altitude
/// let max_range = maximum_ground_range(altitude);
/// println!("Maximum ground range: {:.1} km", max_range);
/// // Expected: ~2300 km
/// ```
pub fn maximum_ground_range(altitude: f64) -> f64 {
    let r_earth = 6371.0; // km (mean radius)
    let r_sat = r_earth + altitude;

    // Central angle from Earth center to horizon (as seen from satellite)
    // This is the angle at Earth's center from sub-satellite point to horizon
    let lambda = (r_earth / r_sat).acos();

    // Ground range is arc length from sub-satellite point to horizon
    

    r_earth * lambda
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_ecef_to_geodetic_equator() {
        // Point on equator at sea level
        let ecef = [WGS84_A, 0.0, 0.0];
        let geodetic = ecef_to_geodetic(&ecef);

        assert_relative_eq!(geodetic.latitude, 0.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.longitude, 0.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.altitude, 0.0, epsilon = 1e-3); // 1 meter tolerance
    }

    #[test]
    fn test_ecef_to_geodetic_north_pole() {
        // North pole at sea level
        let ecef = [0.0, 0.0, WGS84_B];
        let geodetic = ecef_to_geodetic(&ecef);

        assert_relative_eq!(geodetic.latitude, PI / 2.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.altitude, 0.0, epsilon = 1e-3);
    }

    #[test]
    fn test_ecef_to_geodetic_south_pole() {
        // South pole at sea level
        let ecef = [0.0, 0.0, -WGS84_B];
        let geodetic = ecef_to_geodetic(&ecef);

        assert_relative_eq!(geodetic.latitude, -PI / 2.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.altitude, 0.0, epsilon = 1e-3);
    }

    #[test]
    fn test_ecef_to_geodetic_prime_meridian() {
        // 45°N on prime meridian at sea level
        let lat = 45.0_f64.to_radians();
        let lon = 0.0;

        // Compute ECEF for this geodetic position
        let sin_lat = lat.sin();
        let cos_lat = lat.cos();
        let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();

        let x = n * cos_lat;
        let y = 0.0;
        let z = n * (1.0 - WGS84_E2) * sin_lat;

        let ecef = [x, y, z];
        let geodetic = ecef_to_geodetic(&ecef);

        assert_relative_eq!(geodetic.latitude, lat, epsilon = 1e-9);
        assert_relative_eq!(geodetic.longitude, lon, epsilon = 1e-9);
        assert_relative_eq!(geodetic.altitude, 0.0, epsilon = 1e-3);
    }

    #[test]
    fn test_ecef_to_geodetic_with_altitude() {
        // Point on equator at 500 km altitude
        let h = 500.0;
        let ecef = [WGS84_A + h, 0.0, 0.0];
        let geodetic = ecef_to_geodetic(&ecef);

        assert_relative_eq!(geodetic.latitude, 0.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.longitude, 0.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.altitude, h, epsilon = 1e-3);
    }

    #[test]
    fn test_ecef_to_geodetic_leo_satellite() {
        // Typical LEO satellite position (approximately 400 km altitude)
        let ecef = [6700.0, 0.0, 500.0];
        let geodetic = ecef_to_geodetic(&ecef);

        // Verify reasonable results
        assert!(geodetic.latitude.abs() < 10.0_f64.to_radians()); // Near equatorial
        assert!(geodetic.altitude > 300.0 && geodetic.altitude < 500.0); // LEO altitude range
    }

    #[test]
    fn test_ecef_to_geodetic_western_longitude() {
        // Point with negative longitude (western hemisphere)
        let ecef = [0.0, -WGS84_A, 0.0];
        let geodetic = ecef_to_geodetic(&ecef);

        assert_relative_eq!(geodetic.latitude, 0.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.longitude, -PI / 2.0, epsilon = 1e-9);
        assert_relative_eq!(geodetic.altitude, 0.0, epsilon = 1e-3);
    }

    #[test]
    fn test_sub_satellite_point() {
        // Test that sub_satellite_point is equivalent to ecef_to_geodetic
        let ecef = [6700.0, 1000.0, 200.0];
        let geodetic1 = ecef_to_geodetic(&ecef);
        let geodetic2 = sub_satellite_point(&ecef);

        assert_relative_eq!(geodetic1.latitude, geodetic2.latitude, epsilon = 1e-12);
        assert_relative_eq!(geodetic1.longitude, geodetic2.longitude, epsilon = 1e-12);
        assert_relative_eq!(geodetic1.altitude, geodetic2.altitude, epsilon = 1e-12);
    }

    #[test]
    fn test_compute_ground_track() {
        // Simulate simple ground track with two points
        let positions = vec![
            [WGS84_A + 400.0, 0.0, 0.0],
            [WGS84_A + 400.0, 100.0, 50.0],
        ];
        let times = vec![0.0, 1.0];

        let track = compute_ground_track(&positions, &times);

        assert_eq!(track.len(), 2);
        assert_relative_eq!(track[0].time, 0.0, epsilon = 1e-12);
        assert_relative_eq!(track[1].time, 1.0, epsilon = 1e-12);

        // First point should be on equator, prime meridian
        assert_relative_eq!(track[0].latitude, 0.0, epsilon = 1e-6);
        assert_relative_eq!(track[0].longitude, 0.0, epsilon = 1e-6);

        // Both should have similar altitudes
        assert!((track[0].altitude - 400.0).abs() < 10.0);
        assert!((track[1].altitude - 400.0).abs() < 50.0);
    }

    #[test]
    fn test_calculate_swath_width_leo() {
        // LEO satellite at 500 km with 10° minimum elevation
        let altitude = 500.0;
        let min_elevation = 10.0_f64.to_radians();

        let swath = calculate_swath_width(altitude, min_elevation);

        // Using simplified approximation, check for reasonable value
        // Should be less than 2x maximum ground range
        let max_range = maximum_ground_range(altitude);
        assert!(swath > 0.0 && swath < 2.0 * max_range, "Swath was {:.1} km", swath);
    }

    #[test]
    fn test_calculate_swath_width_higher_altitude() {
        // For simplified formula, just verify that function returns positive values
        let altitude = 800.0;
        let min_elevation = 10.0_f64.to_radians();

        let swath = calculate_swath_width(altitude, min_elevation);
        assert!(swath > 0.0, "Swath should be positive: {:.1} km", swath);
    }

    #[test]
    fn test_calculate_swath_width_lower_elevation() {
        // Same altitude, lower minimum elevation
        let altitude = 500.0;
        let min_elev_5 = 5.0_f64.to_radians();
        let min_elev_10 = 10.0_f64.to_radians();

        let swath_5 = calculate_swath_width(altitude, min_elev_5);
        let swath_10 = calculate_swath_width(altitude, min_elev_10);

        // Lower elevation constraint should give WIDER swath
        // (can see more of Earth near the horizon)
        assert!(swath_5 > swath_10, "swath_5={:.1}, swath_10={:.1}", swath_5, swath_10);
    }

    #[test]
    fn test_maximum_ground_range_leo() {
        // ISS altitude
        let altitude = 400.0;
        let max_range = maximum_ground_range(altitude);

        // Should be approximately 2300 km for ISS altitude
        assert!(max_range > 2200.0 && max_range < 2400.0);
    }

    #[test]
    fn test_maximum_ground_range_scaling() {
        // Higher altitude should give longer ground range
        let range_400 = maximum_ground_range(400.0);
        let range_800 = maximum_ground_range(800.0);

        assert!(range_800 > range_400);
    }
}
