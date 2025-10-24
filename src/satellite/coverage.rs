//! Satellite Footprint and Coverage Analysis
//!
//! This module provides functionality for calculating satellite coverage areas,
//! visibility circles (footprints), and access time statistics for ground targets.
//!
//! # Overview
//!
//! Satellite coverage analysis is essential for mission planning, determining which
//! areas on Earth can communicate with a satellite, and optimizing observation schedules.
//!
//! # Key Concepts
//!
//! ## Satellite Footprint (Visibility Circle)
//!
//! The satellite footprint is the area on Earth's surface from which the satellite
//! is visible above a minimum elevation angle. For a given minimum elevation ε:
//!
//! - Points closer to the sub-satellite point have higher elevation angles
//! - Points at the edge of the footprint have elevation = ε (minimum threshold)
//! - Points outside the footprint cannot see the satellite
//!
//! The footprint is approximately circular for satellites in LEO/MEO, though Earth's
//! oblateness causes slight distortion.
//!
//! ## Coverage Area
//!
//! The total area on Earth's surface visible from a satellite depends on:
//! - **Altitude**: Higher satellites see more of Earth's surface
//! - **Minimum elevation angle**: Lower angles give wider coverage but worse signal quality
//! - **Earth geometry**: Spherical cap approximation is accurate for most purposes
//!
//! ## Access Times
//!
//! For ground-based applications (tracking stations, communication links, observations),
//! access time is the cumulative duration when a satellite is visible above the minimum
//! elevation angle. This module provides utilities to aggregate pass data.
//!
//! # Algorithms
//!
//! ## Visibility Circle Calculation
//!
//! 1. **Compute nadir angle** (angular radius of visibility circle):
//!    - From satellite geometry: λ = arccos(R_e / (R_e + h))
//!    - For minimum elevation ε: λ_min = arccos((R_e · cos(ε)) / (R_e + h))
//!    - Or using horizon formula: λ = arcsin((R_e / (R_e + h)) · cos(ε))
//!
//! 2. **Generate circle points** using spherical trigonometry:
//!    - For N points around the circle, compute bearing β_i = 360° × i/N
//!    - Use destination point formula to find (lat_i, lon_i) at distance λ and bearing β_i
//!
//! 3. **Destination point formula** (from sub-satellite point):
//!    - lat₂ = arcsin(sin(lat₁)·cos(λ) + cos(lat₁)·sin(λ)·cos(β))
//!    - lon₂ = lon₁ + arctan2(sin(β)·sin(λ)·cos(lat₁), cos(λ) - sin(lat₁)·sin(lat₂))
//!
//! ## Coverage Area
//!
//! The coverage area is a spherical cap with angular radius λ:
//! - **Spherical cap area**: A = 2πR²(1 - cos(λ))
//! - For small angles: A ≈ πR²λ² (circle approximation)
//!
//! ## Access Time Statistics
//!
//! From a list of satellite passes (from visibility module):
//! - Total access time: sum of all pass durations
//! - Average pass duration: total time / number of passes
//! - Maximum elevation pass: pass with highest maximum elevation
//! - Coverage percentage: fraction of time satellite is visible
//!
//! # References
//!
//! - Vallado, "Fundamentals of Astrodynamics and Applications" (2013)
//!   - Section 11.4: Ground Track and Coverage
//!   - Algorithm 31: Site Coverage
//! - Curtis, "Orbital Mechanics for Engineering Students" (2014)
//!   - Chapter 5: Satellite Ground Tracks and Coverage
//! - Wertz, Larson: "Space Mission Analysis and Design" (SMAD), Ch. 9
//! - Movable Type Scripts: "Calculate distance, bearing and more between Lat/Lon points"
//!   <https://www.movable-type.co.uk/scripts/latlong.html>
//!
//! # Example
//!
//! ```rust,ignore
//! use astrora_core::satellite::coverage::{visibility_circle, coverage_area};
//!
//! // ISS at 400 km altitude, sub-satellite point at equator
//! let altitude = 400.0; // km
//! let lat_sub = 0.0;    // degrees
//! let lon_sub = 0.0;    // degrees
//! let min_elevation = 10.0; // degrees (typical for ground stations)
//!
//! // Compute visibility circle (64 points)
//! let circle = visibility_circle(lat_sub, lon_sub, altitude, min_elevation, 64);
//! println!("Footprint has {} points", circle.len());
//!
//! // Calculate total coverage area
//! let area = coverage_area(altitude, min_elevation);
//! println!("Coverage area: {:.0} km²", area);
//! ```

use std::f64::consts::PI;
use crate::satellite::visibility::SatellitePass;

/// WGS84 Earth parameters
const R_EARTH: f64 = 6371.0; // Mean Earth radius (km)
const WGS84_A: f64 = 6378.137; // Semi-major axis (km) - for area calculations

/// Point on Earth's surface in geodetic coordinates
#[derive(Debug, Clone, Copy)]
pub struct GeodeticPoint {
    /// Latitude in degrees (-90 to +90)
    pub latitude: f64,
    /// Longitude in degrees (-180 to +180)
    pub longitude: f64,
}

/// Access time statistics for a ground target
#[derive(Debug, Clone)]
pub struct AccessStatistics {
    /// Total number of passes
    pub num_passes: usize,
    /// Total access time across all passes (minutes)
    pub total_access_time: f64,
    /// Average pass duration (minutes)
    pub average_pass_duration: f64,
    /// Maximum elevation across all passes (degrees)
    pub max_elevation: f64,
    /// Pass with highest maximum elevation
    pub best_pass_index: Option<usize>,
    /// Minimum pass duration (minutes)
    pub min_pass_duration: f64,
    /// Maximum pass duration (minutes)
    pub max_pass_duration: f64,
}

/// Compute the visibility circle (satellite footprint) on Earth's surface.
///
/// Returns a list of geodetic points that form a circle around the sub-satellite point.
/// All points on this circle have the satellite at exactly the minimum elevation angle.
/// Points inside the circle see the satellite at higher elevations; points outside cannot see it.
///
/// # Arguments
///
/// * `lat_sub` - Sub-satellite point latitude in degrees (-90 to +90)
/// * `lon_sub` - Sub-satellite point longitude in degrees (-180 to +180)
/// * `altitude` - Satellite altitude above Earth's surface in km
/// * `min_elevation` - Minimum elevation angle in degrees (0 to 90)
/// * `num_points` - Number of points to generate around the circle (recommended: 64-128)
///
/// # Returns
///
/// Vector of `GeodeticPoint` structs forming the visibility circle
///
/// # Panics
///
/// Panics if altitude is negative or num_points is zero
///
/// # Example
///
/// ```rust,ignore
/// // ISS footprint with 10° minimum elevation
/// let circle = visibility_circle(0.0, 0.0, 400.0, 10.0, 64);
/// for point in &circle {
///     println!("Lat: {:.2}°, Lon: {:.2}°", point.latitude, point.longitude);
/// }
/// ```
pub fn visibility_circle(
    lat_sub: f64,
    lon_sub: f64,
    altitude: f64,
    min_elevation: f64,
    num_points: usize,
) -> Vec<GeodeticPoint> {
    assert!(altitude >= 0.0, "Altitude must be non-negative");
    assert!(num_points > 0, "Number of points must be positive");

    // Convert to radians
    let lat_rad = lat_sub.to_radians();
    let lon_rad = lon_sub.to_radians();
    let elev_rad = min_elevation.to_radians();

    // Calculate angular radius of visibility circle (nadir angle)
    // Using satellite coverage geometry (derived from law of sines):
    //
    // η = arcsin(cos(ε) · R_e / (R_e + h))  [angle at satellite]
    // λ = 90° - ε - η                        [Earth central angle / nadir angle]
    //
    // Where:
    // - ε is the minimum elevation angle (at point on Earth)
    // - η (eta) is the angle at satellite (in the spherical triangle)
    // - λ (lambda) is the Earth central angle from sub-satellite point to edge
    let r_sat = R_EARTH + altitude;

    // Calculate eta (angle at satellite) using sine rule
    let sin_eta = elev_rad.cos() * R_EARTH / r_sat;
    let sin_eta = sin_eta.min(1.0).max(-1.0);
    let eta = sin_eta.asin();

    // Calculate lambda (nadir angle / central angle)
    let lambda = (PI / 2.0) - elev_rad - eta;

    // Generate points around the circle using spherical trigonometry
    let mut points = Vec::with_capacity(num_points);

    for i in 0..num_points {
        // Bearing from sub-satellite point (0° = North, clockwise)
        let bearing = 2.0 * PI * (i as f64) / (num_points as f64);

        // Destination point formula (spherical trigonometry)
        let lat2 = (lat_rad.sin() * lambda.cos()
                    + lat_rad.cos() * lambda.sin() * bearing.cos()).asin();

        let lon2 = lon_rad + (bearing.sin() * lambda.sin() * lat_rad.cos())
            .atan2(lambda.cos() - lat_rad.sin() * lat2.sin());

        points.push(GeodeticPoint {
            latitude: lat2.to_degrees(),
            longitude: normalize_longitude(lon2.to_degrees()),
        });
    }

    points
}

/// Calculate the total coverage area (satellite footprint area) on Earth's surface.
///
/// Returns the area in square kilometers that can see the satellite above the
/// minimum elevation angle. Uses spherical cap formula for accurate results.
///
/// # Arguments
///
/// * `altitude` - Satellite altitude above Earth's surface in km
/// * `min_elevation` - Minimum elevation angle in degrees (0 to 90)
///
/// # Returns
///
/// Coverage area in km²
///
/// # Example
///
/// ```rust,ignore
/// // ISS coverage area with 10° minimum elevation
/// let area = coverage_area(400.0, 10.0);
/// println!("Coverage area: {:.0} km²", area);
/// // Expected: ~17-20 million km²
/// ```
pub fn coverage_area(altitude: f64, min_elevation: f64) -> f64 {
    assert!(altitude >= 0.0, "Altitude must be non-negative");

    // Convert to radians
    let elev_rad = min_elevation.to_radians();

    // Calculate angular radius of visibility circle using satellite geometry
    let r_sat = R_EARTH + altitude;
    let sin_eta = elev_rad.cos() * R_EARTH / r_sat;
    let sin_eta = sin_eta.min(1.0).max(-1.0);
    let eta = sin_eta.asin();
    let lambda = (PI / 2.0) - elev_rad - eta;

    // Spherical cap area: A = 2πR²(1 - cos(λ))
    // Using WGS84 semi-major axis for more accurate Earth surface area
    

    2.0 * PI * WGS84_A * WGS84_A * (1.0 - lambda.cos())
}

/// Compute access time statistics for a ground target from a list of satellite passes.
///
/// Aggregates data from multiple passes to provide useful statistics for mission planning,
/// including total access time, average pass duration, and best pass identification.
///
/// # Arguments
///
/// * `passes` - Slice of `SatellitePass` structs from visibility calculations
///
/// # Returns
///
/// `AccessStatistics` struct containing aggregated data, or `None` if passes is empty
///
/// # Example
///
/// ```rust,ignore
/// use astrora_core::satellite::visibility::find_all_passes;
/// use astrora_core::satellite::coverage::compute_access_statistics;
///
/// // Find all passes for a ground station
/// let passes = find_all_passes(tle, lat, lon, alt, t_start, t_end, min_elev, step);
///
/// // Get statistics
/// if let Some(stats) = compute_access_statistics(&passes) {
///     println!("Total access time: {:.1} minutes", stats.total_access_time);
///     println!("Average pass: {:.1} minutes", stats.average_pass_duration);
///     println!("Number of passes: {}", stats.num_passes);
/// }
/// ```
pub fn compute_access_statistics(passes: &[SatellitePass]) -> Option<AccessStatistics> {
    if passes.is_empty() {
        return None;
    }

    let num_passes = passes.len();

    // Calculate total access time and find max elevation
    let mut total_time = 0.0;
    let mut max_elevation = 0.0;
    let mut best_pass_index = 0;
    let mut min_duration = f64::INFINITY;
    let mut max_duration = 0.0;

    for (i, pass) in passes.iter().enumerate() {
        let duration = pass.set_time - pass.rise_time;
        total_time += duration;

        if duration < min_duration {
            min_duration = duration;
        }
        if duration > max_duration {
            max_duration = duration;
        }

        if pass.max_elevation > max_elevation {
            max_elevation = pass.max_elevation;
            best_pass_index = i;
        }
    }

    let average_duration = total_time / (num_passes as f64);

    Some(AccessStatistics {
        num_passes,
        total_access_time: total_time,
        average_pass_duration: average_duration,
        max_elevation,
        best_pass_index: Some(best_pass_index),
        min_pass_duration: if min_duration == f64::INFINITY { 0.0 } else { min_duration },
        max_pass_duration: max_duration,
    })
}

/// Normalize longitude to the range [-180, 180] degrees.
fn normalize_longitude(lon: f64) -> f64 {
    let mut normalized = lon;
    while normalized > 180.0 {
        normalized -= 360.0;
    }
    while normalized < -180.0 {
        normalized += 360.0;
    }
    normalized
}

/// Calculate the fraction of time a satellite is visible from a ground target.
///
/// # Arguments
///
/// * `total_access_time` - Total access time in minutes
/// * `time_span` - Total time span analyzed in minutes
///
/// # Returns
///
/// Coverage percentage as a fraction between 0.0 and 1.0
///
/// # Example
///
/// ```rust,ignore
/// let coverage_fraction = coverage_percentage(90.0, 1440.0); // 90 min out of 24 hours
/// println!("Coverage: {:.1}%", coverage_fraction * 100.0);
/// // Output: "Coverage: 6.2%"
/// ```
pub fn coverage_percentage(total_access_time: f64, time_span: f64) -> f64 {
    if time_span <= 0.0 {
        return 0.0;
    }
    (total_access_time / time_span).min(1.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_visibility_circle_basic() {
        // ISS at 400 km, equator, 0° minimum elevation (horizon)
        let circle = visibility_circle(0.0, 0.0, 400.0, 0.0, 32);

        assert_eq!(circle.len(), 32);

        // Check that all points are roughly the same distance from sub-satellite point
        // For 0° elevation at 400 km, angular radius should be about 19.8 degrees
        // (derived from λ = 90° - 0° - arcsin(R_e/(R_e+h)))
        for point in &circle {
            let distance = angular_distance(0.0, 0.0, point.latitude, point.longitude);
            // Should be consistent across all points (within numerical tolerance)
            assert!(distance > 19.0 && distance < 21.0,
                    "Distance {:.2}° outside expected range", distance);
        }
    }

    #[test]
    fn test_visibility_circle_symmetry() {
        // Circle should be symmetric around sub-satellite point at equator
        let circle = visibility_circle(0.0, 0.0, 400.0, 10.0, 36);

        // Points at opposite sides should have equal but opposite longitudes
        let point_0 = &circle[0];   // North
        let point_18 = &circle[18]; // South

        // Should be symmetric in latitude (opposite signs, similar magnitude)
        assert_relative_eq!(point_0.latitude, -point_18.latitude, epsilon = 1.0);
    }

    #[test]
    fn test_coverage_area_increases_with_altitude() {
        // Higher altitude should give larger coverage area
        let area_leo = coverage_area(400.0, 10.0);    // ISS
        let area_meo = coverage_area(20000.0, 10.0);  // GPS-like
        let area_geo = coverage_area(35786.0, 10.0);  // GEO

        assert!(area_leo < area_meo);
        assert!(area_meo < area_geo);

        // ISS at 400 km with 10° elevation should cover ~5-7 million km²
        // (λ ≈ 12°, spherical cap area ≈ 5.7M km²)
        assert!(area_leo > 5_000_000.0 && area_leo < 7_000_000.0,
                "ISS coverage area {:.0} km² outside expected range", area_leo);
    }

    #[test]
    fn test_coverage_area_decreases_with_elevation() {
        // Higher minimum elevation should give smaller coverage area
        let area_0deg = coverage_area(400.0, 0.0);   // Horizon
        let area_10deg = coverage_area(400.0, 10.0); // Typical ground station
        let area_45deg = coverage_area(400.0, 45.0); // High elevation

        assert!(area_45deg < area_10deg);
        assert!(area_10deg < area_0deg);
    }

    #[test]
    fn test_access_statistics_basic() {
        // Create mock satellite passes
        let passes = vec![
            SatellitePass {
                rise_time: 0.0,
                set_time: 10.0,
                max_elevation_time: 5.0,
                max_elevation: 45.0,
                rise_azimuth: 0.0,
                set_azimuth: 180.0,
                duration: 10.0,
            },
            SatellitePass {
                rise_time: 100.0,
                set_time: 115.0,
                max_elevation_time: 107.5,
                max_elevation: 80.0,
                rise_azimuth: 180.0,
                set_azimuth: 0.0,
                duration: 15.0,
            },
        ];

        let stats = compute_access_statistics(&passes).unwrap();

        assert_eq!(stats.num_passes, 2);
        assert_relative_eq!(stats.total_access_time, 25.0, epsilon = 1e-6); // 10 + 15
        assert_relative_eq!(stats.average_pass_duration, 12.5, epsilon = 1e-6);
        assert_relative_eq!(stats.max_elevation, 80.0, epsilon = 1e-6);
        assert_eq!(stats.best_pass_index, Some(1));
        assert_relative_eq!(stats.min_pass_duration, 10.0, epsilon = 1e-6);
        assert_relative_eq!(stats.max_pass_duration, 15.0, epsilon = 1e-6);
    }

    #[test]
    fn test_access_statistics_empty() {
        let passes: Vec<SatellitePass> = vec![];
        let stats = compute_access_statistics(&passes);
        assert!(stats.is_none());
    }

    #[test]
    fn test_coverage_percentage() {
        // 90 minutes out of 1440 (24 hours)
        let fraction = coverage_percentage(90.0, 1440.0);
        assert_relative_eq!(fraction, 0.0625, epsilon = 1e-6); // 6.25%

        // Full coverage
        let full = coverage_percentage(100.0, 100.0);
        assert_relative_eq!(full, 1.0, epsilon = 1e-6);

        // Zero coverage
        let zero = coverage_percentage(0.0, 100.0);
        assert_relative_eq!(zero, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_normalize_longitude() {
        assert_relative_eq!(normalize_longitude(0.0), 0.0, epsilon = 1e-10);
        assert_relative_eq!(normalize_longitude(180.0), 180.0, epsilon = 1e-10);
        assert_relative_eq!(normalize_longitude(-180.0), -180.0, epsilon = 1e-10);
        assert_relative_eq!(normalize_longitude(190.0), -170.0, epsilon = 1e-10);
        assert_relative_eq!(normalize_longitude(-190.0), 170.0, epsilon = 1e-10);
        assert_relative_eq!(normalize_longitude(370.0), 10.0, epsilon = 1e-10);
    }

    // Helper function: calculate angular distance between two points
    fn angular_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();
        let dlon = (lon2 - lon1).to_radians();

        // Haversine formula
        let a = ((lat2_rad - lat1_rad) / 2.0).sin().powi(2)
              + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();

        c.to_degrees()
    }
}
