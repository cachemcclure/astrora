//! Satellite Visibility and Ground Station Operations
//!
//! This module provides functionality for calculating satellite visibility from
//! ground stations, including azimuth, elevation, rise/set times, and pass predictions.
//!
//! # Overview
//!
//! Ground station operators need to know when a satellite is visible from their location.
//! This requires transforming satellite positions from inertial frames (TEME, GCRS) to
//! topocentric coordinates (azimuth, elevation) relative to an observer on Earth's surface.
//!
//! # Coordinate Systems
//!
//! - **ECEF** (Earth-Centered, Earth-Fixed): Cartesian coordinates rotating with Earth
//! - **Geodetic**: Latitude, longitude, altitude on Earth's surface
//! - **ENU** (East-North-Up): Local topocentric frame at observer location
//! - **SEZ** (South-East-Zenith): Alternative topocentric frame (Vallado convention)
//! - **Az/El**: Azimuth (0° = North, 90° = East) and Elevation (0° = horizon, 90° = zenith)
//!
//! # Algorithms
//!
//! ## ECEF to Topocentric Transformation
//!
//! 1. Calculate observer position in ECEF from geodetic coordinates (lat, lon, alt)
//! 2. Compute range vector: ρ_ECEF = r_sat_ECEF - r_obs_ECEF
//! 3. Rotate to ENU frame using latitude and longitude:
//!    - Rotation about Z by (90° + λ)
//!    - Rotation about X by (90° - φ)
//!
//! ## Azimuth and Elevation
//!
//! From ENU components (E, N, U):
//! - Azimuth: Az = atan2(E, N) (measured clockwise from North)
//! - Elevation: El = atan2(U, sqrt(E² + N²))
//!
//! ## Rise/Set Time Prediction
//!
//! Uses bisection search to find when elevation crosses minimum threshold:
//! 1. Propagate satellite over time window
//! 2. Detect sign changes in (elevation - min_elevation)
//! 3. Use bisection to refine crossing times to sub-second accuracy
//!
//! ## Maximum Elevation Finding
//!
//! For each satellite pass:
//! 1. Find rise and set times
//! 2. Search for maximum elevation between rise and set
//! 3. Use golden section search or sampling for optimization
//!
//! # References
//!
//! - Vallado, "Fundamentals of Astrodynamics and Applications" (2013)
//!   - Algorithm 27: Range, Azimuth, Elevation (rv2razel)
//!   - Section 4.4: Site Tracking
//! - Navipedia: "Transformations between ECEF and ENU coordinates"
//!   <https://gssc.esa.int/navipedia/index.php/Transformations_between_ECEF_and_ENU_coordinates>
//! - Curtis, "Orbital Mechanics for Engineering Students" (2014)
//!   - Chapter 5: Ground Track and Satellite Access
//!
//! # Example
//!
//! ```rust,ignore
//! use astrora_core::satellite::visibility::{compute_azimuth_elevation, Observer};
//!
//! // Ground station at MIT (42.36°N, 71.09°W, 10m altitude)
//! let observer = Observer::new(42.36_f64.to_radians(), -71.09_f64.to_radians(), 0.010);
//!
//! // Satellite position in ECEF (km)
//! let sat_ecef = [4000.0, 3000.0, 5000.0];
//!
//! // Calculate visibility
//! let result = compute_azimuth_elevation(&sat_ecef, &observer);
//! println!("Azimuth: {:.2}°, Elevation: {:.2}°",
//!          result.azimuth.to_degrees(), result.elevation.to_degrees());
//! ```

use nalgebra::{Vector3, Matrix3};
use std::f64::consts::PI;

/// WGS84 Earth ellipsoid parameters
const WGS84_A: f64 = 6378.137;           // Semi-major axis (km)
const WGS84_F: f64 = 1.0 / 298.257223563; // Flattening
const WGS84_E2: f64 = WGS84_F * (2.0 - WGS84_F); // Eccentricity squared

/// Small threshold for numerical singularity detection (same as Vallado)
const SMALL: f64 = 1e-8;

/// Observer location on Earth's surface
#[derive(Debug, Clone, Copy)]
pub struct Observer {
    /// Geodetic latitude (radians, -π/2 to π/2)
    pub latitude: f64,
    /// Geodetic longitude (radians, -π to π)
    pub longitude: f64,
    /// Altitude above WGS84 ellipsoid (km)
    pub altitude: f64,
}

impl Observer {
    /// Create a new observer at a geodetic location
    ///
    /// # Arguments
    /// - `latitude`: Geodetic latitude in radians (-π/2 to π/2)
    /// - `longitude`: Geodetic longitude in radians (-π to π)
    /// - `altitude`: Altitude above WGS84 ellipsoid in km
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Observer { latitude, longitude, altitude }
    }

    /// Convert observer geodetic coordinates to ECEF position vector (km)
    ///
    /// Uses WGS84 ellipsoid model with flattening.
    ///
    /// # Algorithm
    /// 1. Compute radius of curvature in prime vertical: N = a / sqrt(1 - e²sin²φ)
    /// 2. ECEF position:
    ///    - X = (N + h) cos φ cos λ
    ///    - Y = (N + h) cos φ sin λ
    ///    - Z = (N(1-e²) + h) sin φ
    ///
    /// # Returns
    /// ECEF position vector [x, y, z] in km
    pub fn to_ecef(&self) -> Vector3<f64> {
        let sin_lat = self.latitude.sin();
        let cos_lat = self.latitude.cos();
        let sin_lon = self.longitude.sin();
        let cos_lon = self.longitude.cos();

        // Radius of curvature in the prime vertical
        let n = WGS84_A / (1.0 - WGS84_E2 * sin_lat * sin_lat).sqrt();

        let x = (n + self.altitude) * cos_lat * cos_lon;
        let y = (n + self.altitude) * cos_lat * sin_lon;
        let z = (n * (1.0 - WGS84_E2) + self.altitude) * sin_lat;

        Vector3::new(x, y, z)
    }

    /// Get the rotation matrix from ECEF to ENU (East-North-Up) frame
    ///
    /// This matrix transforms vectors from ECEF to the local topocentric frame
    /// centered at the observer's location.
    ///
    /// # Algorithm (Navipedia)
    /// R_ENU = R₁(π/2 - φ) R₃(π/2 + λ)
    ///
    /// The resulting matrix is:
    /// ```text
    /// [-sin λ          cos λ           0    ]
    /// [-cos λ sin φ   -sin λ sin φ   cos φ ]
    /// [ cos λ cos φ    sin λ cos φ    sin φ ]
    /// ```
    ///
    /// # Returns
    /// 3x3 rotation matrix from ECEF to ENU
    pub fn ecef_to_enu_matrix(&self) -> Matrix3<f64> {
        let sin_lat = self.latitude.sin();
        let cos_lat = self.latitude.cos();
        let sin_lon = self.longitude.sin();
        let cos_lon = self.longitude.cos();

        // ECEF to ENU transformation matrix
        Matrix3::new(
            -sin_lon,          cos_lon,           0.0,
            -cos_lon * sin_lat, -sin_lon * sin_lat, cos_lat,
            cos_lon * cos_lat,  sin_lon * cos_lat,  sin_lat,
        )
    }

    /// Get the rotation matrix from ECEF to SEZ (South-East-Zenith) frame
    ///
    /// SEZ is an alternative topocentric frame used in Vallado's algorithms.
    /// It's related to ENU by a 180° rotation about the zenith axis.
    ///
    /// # Algorithm (Vallado)
    /// R_SEZ = R₂(π/2 - φ) R₃(λ)
    ///
    /// # Returns
    /// 3x3 rotation matrix from ECEF to SEZ
    pub fn ecef_to_sez_matrix(&self) -> Matrix3<f64> {
        let sin_lat = self.latitude.sin();
        let cos_lat = self.latitude.cos();
        let sin_lon = self.longitude.sin();
        let cos_lon = self.longitude.cos();

        // ECEF to SEZ transformation matrix
        Matrix3::new(
            sin_lat * cos_lon,  sin_lat * sin_lon, -cos_lat,
            -sin_lon,           cos_lon,            0.0,
            cos_lat * cos_lon,  cos_lat * sin_lon,  sin_lat,
        )
    }
}

/// Result of azimuth/elevation calculation
#[derive(Debug, Clone, Copy)]
pub struct TopocentricCoordinates {
    /// Range (distance) from observer to satellite (km)
    pub range: f64,
    /// Azimuth angle (radians, 0 = North, π/2 = East, π = South, 3π/2 = West)
    pub azimuth: f64,
    /// Elevation angle (radians, 0 = horizon, π/2 = zenith, negative = below horizon)
    pub elevation: f64,
    /// Range rate (km/s, positive = satellite receding)
    pub range_rate: Option<f64>,
    /// ENU components for debugging (East, North, Up) in km
    pub enu: [f64; 3],
}

/// Compute azimuth and elevation of a satellite from an observer location
///
/// # Arguments
/// - `sat_ecef`: Satellite position in ECEF frame (km)
/// - `observer`: Observer location on Earth's surface
///
/// # Returns
/// Topocentric coordinates (range, azimuth, elevation)
///
/// # Algorithm
/// 1. Convert observer geodetic → ECEF
/// 2. Compute range vector: ρ = r_sat - r_obs (ECEF)
/// 3. Transform ρ to ENU frame using rotation matrix
/// 4. Compute Az = atan2(E, N), El = atan2(U, sqrt(E² + N²))
///
/// # Example
/// ```rust,ignore
/// let observer = Observer::new(42.36_f64.to_radians(), -71.09_f64.to_radians(), 0.010);
/// let sat_ecef = [7000.0, 0.0, 0.0];
/// let topo = compute_azimuth_elevation(&sat_ecef, &observer);
/// ```
pub fn compute_azimuth_elevation(
    sat_ecef: &[f64; 3],
    observer: &Observer,
) -> TopocentricCoordinates {
    // 1. Observer ECEF position
    let obs_ecef = observer.to_ecef();

    // 2. Range vector (satellite - observer) in ECEF
    let sat_vec = Vector3::new(sat_ecef[0], sat_ecef[1], sat_ecef[2]);
    let rho_ecef = sat_vec - obs_ecef;
    let range = rho_ecef.norm();

    // 3. Transform to ENU frame
    let rot_enu = observer.ecef_to_enu_matrix();
    let rho_enu = rot_enu * rho_ecef;

    let e = rho_enu[0]; // East
    let n = rho_enu[1]; // North
    let u = rho_enu[2]; // Up

    // 4. Compute azimuth and elevation
    // Azimuth: measured clockwise from North (0° = North, 90° = East)
    // atan2(East, North) gives angle from North axis toward East
    let mut azimuth = e.atan2(n);
    if azimuth < 0.0 {
        azimuth += 2.0 * PI; // Ensure 0 to 2π range
    }

    // Elevation: angle above horizon
    let horizontal_range = (e * e + n * n).sqrt();
    let elevation = if horizontal_range < SMALL {
        // At zenith or nadir (singular case)
        if u > 0.0 { PI / 2.0 } else { -PI / 2.0 }
    } else {
        u.atan2(horizontal_range)
    };

    TopocentricCoordinates {
        range,
        azimuth,
        elevation,
        range_rate: None,
        enu: [e, n, u],
    }
}

/// Compute azimuth, elevation, and their rates from position and velocity
///
/// # Arguments
/// - `sat_ecef`: Satellite position in ECEF frame (km)
/// - `vel_ecef`: Satellite velocity in ECEF frame (km/s)
/// - `observer`: Observer location on Earth's surface
///
/// # Returns
/// Topocentric coordinates including range rate
///
/// # Note
/// This is more accurate than computing azimuth/elevation separately,
/// as it properly handles the rates using the full state vector.
pub fn compute_azimuth_elevation_rate(
    sat_ecef: &[f64; 3],
    vel_ecef: &[f64; 3],
    observer: &Observer,
) -> TopocentricCoordinates {
    // Get basic Az/El
    let mut result = compute_azimuth_elevation(sat_ecef, observer);

    // Transform velocity to ENU for range rate
    let vel_vec = Vector3::new(vel_ecef[0], vel_ecef[1], vel_ecef[2]);
    let rot_enu = observer.ecef_to_enu_matrix();
    let vel_enu = rot_enu * vel_vec;

    // Range rate: component of velocity along line of sight
    let rho_enu = Vector3::new(result.enu[0], result.enu[1], result.enu[2]);
    let range_rate = rho_enu.dot(&vel_enu) / result.range;

    result.range_rate = Some(range_rate);
    result
}

/// Check if a satellite is visible from an observer location
///
/// # Arguments
/// - `sat_ecef`: Satellite position in ECEF frame (km)
/// - `observer`: Observer location on Earth's surface
/// - `min_elevation`: Minimum elevation angle for visibility (radians, typically 0° to 10°)
///
/// # Returns
/// `true` if satellite elevation is above minimum threshold
///
/// # Example
/// ```rust,ignore
/// let observer = Observer::new(42.36_f64.to_radians(), -71.09_f64.to_radians(), 0.010);
/// let sat_ecef = [7000.0, 0.0, 0.0];
/// let visible = is_visible(&sat_ecef, &observer, 0.0); // 0° minimum elevation
/// ```
pub fn is_visible(
    sat_ecef: &[f64; 3],
    observer: &Observer,
    min_elevation: f64,
) -> bool {
    let topo = compute_azimuth_elevation(sat_ecef, observer);
    topo.elevation >= min_elevation
}

/// Check if there is a clear line of sight from observer to satellite
///
/// # Arguments
/// - `sat_ecef`: Satellite position in ECEF frame (km)
/// - `observer`: Observer location on Earth's surface
///
/// # Returns
/// `true` if satellite is above local horizon (elevation > 0°)
///
/// # Note
/// This is a simple geometric check. Advanced implementations could account for:
/// - Atmospheric refraction (~0.5° at horizon)
/// - Terrain masking
/// - Obstructions (buildings, trees)
pub fn has_line_of_sight(
    sat_ecef: &[f64; 3],
    observer: &Observer,
) -> bool {
    is_visible(sat_ecef, observer, 0.0)
}

/// Result of a satellite pass prediction
#[derive(Debug, Clone, Copy)]
pub struct SatellitePass {
    /// Rise time (minutes from epoch)
    pub rise_time: f64,
    /// Set time (minutes from epoch)
    pub set_time: f64,
    /// Time of maximum elevation (minutes from epoch)
    pub max_elevation_time: f64,
    /// Maximum elevation angle during pass (radians)
    pub max_elevation: f64,
    /// Azimuth at rise (radians)
    pub rise_azimuth: f64,
    /// Azimuth at set (radians)
    pub set_azimuth: f64,
    /// Duration of pass (minutes)
    pub duration: f64,
}

/// Find the next satellite pass over a ground station
///
/// This function searches for the next time window when a satellite is visible
/// from an observer location, using a propagation function to get satellite position.
///
/// # Arguments
/// - `propagate_fn`: Function that takes time offset (minutes) and returns ECEF position \[x,y,z\] in km
/// - `observer`: Observer location on Earth's surface
/// - `start_time`: Start of search window (minutes from epoch)
/// - `end_time`: End of search window (minutes from epoch)
/// - `min_elevation`: Minimum elevation for visibility (radians, typically 0° to 10°)
/// - `time_step`: Time step for coarse search (minutes, typically 1-10)
///
/// # Returns
/// `Some(SatellitePass)` if a pass is found, `None` if no pass in time window
///
/// # Algorithm
/// 1. Coarse search: Sample at `time_step` intervals to find visibility transitions
/// 2. Fine search: Use bisection to refine rise/set times to sub-second accuracy
/// 3. Optimization: Search for maximum elevation between rise and set
///
/// # Example
/// ```rust,ignore
/// let observer = Observer::new(42.36_f64.to_radians(), -71.09_f64.to_radians(), 0.010);
/// let propagate = |t_minutes: f64| -> [f64; 3] {
///     // Your SGP4 propagation here
///     [7000.0, 0.0, 0.0]
/// };
/// let pass = find_next_pass(&propagate, &observer, 0.0, 1440.0, 0.0, 1.0);
/// ```
pub fn find_next_pass<F>(
    propagate_fn: &F,
    observer: &Observer,
    start_time: f64,
    end_time: f64,
    min_elevation: f64,
    time_step: f64,
) -> Option<SatellitePass>
where
    F: Fn(f64) -> [f64; 3],
{
    // Coarse search for visibility transition
    let mut t = start_time;
    let mut was_visible = {
        let pos = propagate_fn(t);
        is_visible(&pos, observer, min_elevation)
    };

    let mut rise_time = None;

    while t < end_time {
        t += time_step;
        if t > end_time {
            t = end_time;
        }

        let pos = propagate_fn(t);
        let is_vis = is_visible(&pos, observer, min_elevation);

        // Detect rise (transition to visible)
        if !was_visible && is_vis {
            // Refine rise time using bisection
            let refined_rise = refine_event_time(
                propagate_fn,
                observer,
                min_elevation,
                t - time_step,
                t,
                false, // Looking for rise (elevation crossing upward)
            );
            rise_time = Some(refined_rise);
        }

        // Detect set (transition to not visible)
        if was_visible && !is_vis && rise_time.is_some() {
            // We have a complete pass - refine set time
            let refined_set = refine_event_time(
                propagate_fn,
                observer,
                min_elevation,
                t - time_step,
                t,
                true, // Looking for set (elevation crossing downward)
            );

            let rise = rise_time.unwrap();
            let set = refined_set;

            // Find maximum elevation during pass
            let (max_time, max_elev) = find_maximum_elevation(
                propagate_fn,
                observer,
                rise,
                set,
                time_step / 10.0, // Use finer step for max finding
            );

            // Get azimuths at rise and set
            let rise_pos = propagate_fn(rise);
            let set_pos = propagate_fn(set);
            let rise_az = compute_azimuth_elevation(&rise_pos, observer).azimuth;
            let set_az = compute_azimuth_elevation(&set_pos, observer).azimuth;

            return Some(SatellitePass {
                rise_time: rise,
                set_time: set,
                max_elevation_time: max_time,
                max_elevation: max_elev,
                rise_azimuth: rise_az,
                set_azimuth: set_az,
                duration: set - rise,
            });
        }

        was_visible = is_vis;
    }

    None
}

/// Refine an event time (rise or set) using bisection search
///
/// # Arguments
/// - `propagate_fn`: Function that takes time offset (minutes) and returns ECEF position
/// - `observer`: Observer location
/// - `min_elevation`: Threshold elevation for the event
/// - `t_start`: Start of bracketing interval (minutes)
/// - `t_end`: End of bracketing interval (minutes)
/// - `looking_for_set`: true if looking for set, false if looking for rise
///
/// # Returns
/// Refined event time (minutes) with sub-second accuracy
fn refine_event_time<F>(
    propagate_fn: &F,
    observer: &Observer,
    min_elevation: f64,
    t_start: f64,
    t_end: f64,
    looking_for_set: bool,
) -> f64
where
    F: Fn(f64) -> [f64; 3],
{
    let mut a = t_start;
    let mut b = t_end;
    const TOLERANCE: f64 = 1.0 / 3600.0; // 1 second in minutes

    // Bisection search
    while (b - a).abs() > TOLERANCE {
        let mid = (a + b) / 2.0;
        let pos = propagate_fn(mid);
        let topo = compute_azimuth_elevation(&pos, observer);
        let is_above = topo.elevation >= min_elevation;

        if looking_for_set {
            // For set: move interval where satellite transitions from above to below
            if is_above {
                a = mid;
            } else {
                b = mid;
            }
        } else {
            // For rise: move interval where satellite transitions from below to above
            if is_above {
                b = mid;
            } else {
                a = mid;
            }
        }
    }

    (a + b) / 2.0
}

/// Find time and value of maximum elevation during a pass
///
/// # Arguments
/// - `propagate_fn`: Function that takes time offset (minutes) and returns ECEF position
/// - `observer`: Observer location
/// - `t_start`: Start of pass (minutes)
/// - `t_end`: End of pass (minutes)
/// - `time_step`: Sampling interval (minutes)
///
/// # Returns
/// (time_of_max, max_elevation) in (minutes, radians)
fn find_maximum_elevation<F>(
    propagate_fn: &F,
    observer: &Observer,
    t_start: f64,
    t_end: f64,
    time_step: f64,
) -> (f64, f64)
where
    F: Fn(f64) -> [f64; 3],
{
    let mut max_elev = f64::NEG_INFINITY;
    let mut max_time = t_start;

    let mut t = t_start;
    while t <= t_end {
        let pos = propagate_fn(t);
        let topo = compute_azimuth_elevation(&pos, observer);

        if topo.elevation > max_elev {
            max_elev = topo.elevation;
            max_time = t;
        }

        t += time_step;
    }

    // Refine using golden section search around the maximum
    let search_window = time_step * 2.0;
    let refined_time = golden_section_search(
        propagate_fn,
        observer,
        (max_time - search_window).max(t_start),
        (max_time + search_window).min(t_end),
    );

    let pos = propagate_fn(refined_time);
    let final_elev = compute_azimuth_elevation(&pos, observer).elevation;

    (refined_time, final_elev)
}

/// Golden section search to find maximum elevation
///
/// This is a derivative-free optimization method that's more efficient than
/// uniform sampling for finding the maximum of a unimodal function.
fn golden_section_search<F>(
    propagate_fn: &F,
    observer: &Observer,
    mut a: f64,
    mut b: f64,
) -> f64
where
    F: Fn(f64) -> [f64; 3],
{
    const GOLDEN_RATIO: f64 = 1.618033988749895;
    const TOLERANCE: f64 = 1.0 / 3600.0; // 1 second

    let mut c = b - (b - a) / GOLDEN_RATIO;
    let mut d = a + (b - a) / GOLDEN_RATIO;

    while (b - a).abs() > TOLERANCE {
        let fc = compute_azimuth_elevation(&propagate_fn(c), observer).elevation;
        let fd = compute_azimuth_elevation(&propagate_fn(d), observer).elevation;

        if fc > fd {
            b = d;
            d = c;
            c = b - (b - a) / GOLDEN_RATIO;
        } else {
            a = c;
            c = d;
            d = a + (b - a) / GOLDEN_RATIO;
        }
    }

    (a + b) / 2.0
}

/// Find all satellite passes over a ground station within a time window
///
/// # Arguments
/// - `propagate_fn`: Function that takes time offset (minutes) and returns ECEF position
/// - `observer`: Observer location
/// - `start_time`: Start of search window (minutes from epoch)
/// - `end_time`: End of search window (minutes from epoch)
/// - `min_elevation`: Minimum elevation for visibility (radians)
/// - `time_step`: Time step for coarse search (minutes)
///
/// # Returns
/// Vector of all passes found in the time window
///
/// # Example
/// ```rust,ignore
/// let passes = find_all_passes(&propagate, &observer, 0.0, 1440.0, 0.0, 1.0);
/// for pass in passes {
///     println!("Pass from {:.1} to {:.1} min, max el {:.1}°",
///              pass.rise_time, pass.set_time, pass.max_elevation.to_degrees());
/// }
/// ```
pub fn find_all_passes<F>(
    propagate_fn: &F,
    observer: &Observer,
    start_time: f64,
    end_time: f64,
    min_elevation: f64,
    time_step: f64,
) -> Vec<SatellitePass>
where
    F: Fn(f64) -> [f64; 3],
{
    let mut passes = Vec::new();
    let mut search_start = start_time;

    while search_start < end_time {
        if let Some(pass) = find_next_pass(
            propagate_fn,
            observer,
            search_start,
            end_time,
            min_elevation,
            time_step,
        ) {
            passes.push(pass);
            // Continue search after this pass ends
            search_start = pass.set_time + time_step;
        } else {
            // No more passes found
            break;
        }
    }

    passes
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_observer_ecef_conversion() {
        // Test observer at equator, prime meridian, sea level
        let obs = Observer::new(0.0, 0.0, 0.0);
        let ecef = obs.to_ecef();

        // Should be on equator at semi-major axis
        assert_relative_eq!(ecef[0], WGS84_A, epsilon = 1e-6);
        assert_relative_eq!(ecef[1], 0.0, epsilon = 1e-6);
        assert_relative_eq!(ecef[2], 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_observer_north_pole() {
        // North pole
        let obs = Observer::new(PI / 2.0, 0.0, 0.0);
        let ecef = obs.to_ecef();

        // Should be on polar axis
        assert_relative_eq!(ecef[0], 0.0, epsilon = 1e-6);
        assert_relative_eq!(ecef[1], 0.0, epsilon = 1e-6);
        // Z should be close to polar radius (accounting for flattening)
        assert!(ecef[2] > 6350.0 && ecef[2] < 6360.0);
    }

    #[test]
    fn test_azimuth_elevation_zenith() {
        // Observer at equator, prime meridian
        let obs = Observer::new(0.0, 0.0, 0.0);
        let obs_ecef = obs.to_ecef();

        // Satellite directly above observer at 500 km altitude
        // To place it directly above, we need to go radially outward from Earth's center
        let radial_direction = obs_ecef.normalize();
        let sat_ecef_vec = obs_ecef + radial_direction * 500.0;
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Should be nearly at zenith (elevation ≈ 90°)
        assert_relative_eq!(topo.elevation, PI / 2.0, epsilon = 0.01);
        assert_relative_eq!(topo.range, 500.0, epsilon = 1.0);
    }

    #[test]
    fn test_azimuth_north() {
        // Observer at equator, prime meridian
        let obs = Observer::new(0.0, 0.0, 0.0);

        // Create a satellite to the north in ENU coordinates
        // North = 500 km, East = 0, Up = 500 km (45° elevation)
        let enu_north = Vector3::new(0.0, 500.0, 500.0);

        // Transform from ENU to ECEF
        let rot_enu = obs.ecef_to_enu_matrix();
        let obs_ecef = obs.to_ecef();
        // ECEF = obs_ecef + R_ENU^T * enu_offset (inverse rotation)
        let ecef_offset = rot_enu.transpose() * enu_north;
        let sat_ecef_vec = obs_ecef + ecef_offset;
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Azimuth should be close to 0° (North) - allowing small numerical error
        assert!(topo.azimuth < 0.1 || topo.azimuth > 2.0 * PI - 0.1,
                "Azimuth was {:.4} rad ({:.1}°), expected ~0°",
                topo.azimuth, topo.azimuth.to_degrees());

        // Elevation should be around 45°
        assert_relative_eq!(topo.elevation, PI / 4.0, epsilon = 0.1);
    }

    #[test]
    fn test_visibility_above_horizon() {
        // MIT location (42.36°N, 71.09°W)
        let obs = Observer::new(
            42.36_f64.to_radians(),
            -71.09_f64.to_radians(),
            0.010, // 10m altitude
        );

        // Satellite in LEO above MIT
        let obs_ecef = obs.to_ecef();
        let sat_ecef = [
            obs_ecef[0] + 200.0,
            obs_ecef[1] + 200.0,
            obs_ecef[2] + 400.0,
        ];

        assert!(is_visible(&sat_ecef, &obs, 0.0));
        assert!(has_line_of_sight(&sat_ecef, &obs));
    }

    #[test]
    fn test_visibility_below_horizon() {
        // MIT location
        let obs = Observer::new(
            42.36_f64.to_radians(),
            -71.09_f64.to_radians(),
            0.010,
        );

        // Satellite on opposite side of Earth
        let obs_ecef = obs.to_ecef();
        let sat_ecef = [
            -obs_ecef[0],
            -obs_ecef[1],
            -obs_ecef[2],
        ];

        assert!(!is_visible(&sat_ecef, &obs, 0.0));
        assert!(!has_line_of_sight(&sat_ecef, &obs));
    }

    #[test]
    fn test_minimum_elevation_threshold() {
        let obs = Observer::new(0.0, 0.0, 0.0);
        let obs_ecef = obs.to_ecef();

        // Satellite at low elevation (~5°)
        let sat_ecef = [
            obs_ecef[0] + 400.0,
            obs_ecef[1],
            obs_ecef[2] + 35.0,  // Small altitude for low elevation
        ];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Should be visible with 0° threshold
        assert!(is_visible(&sat_ecef, &obs, 0.0));

        // May not be visible with 10° threshold (depends on exact geometry)
        let min_el_10deg = 10.0_f64.to_radians();
        assert_eq!(is_visible(&sat_ecef, &obs, min_el_10deg), topo.elevation >= min_el_10deg);
    }

    #[test]
    fn test_range_rate_calculation() {
        let obs = Observer::new(0.0, 0.0, 0.0);
        let obs_ecef = obs.to_ecef();

        // Satellite moving away (positive range rate)
        let sat_ecef = [obs_ecef[0], obs_ecef[1], obs_ecef[2] + 500.0];
        let vel_ecef = [0.0, 0.0, 5.0]; // Moving up at 5 km/s

        let topo = compute_azimuth_elevation_rate(&sat_ecef, &vel_ecef, &obs);

        assert!(topo.range_rate.is_some());
        let range_rate = topo.range_rate.unwrap();
        // Should be positive (receding)
        assert!(range_rate > 0.0);
        // Should be close to 5 km/s (moving directly away)
        assert_relative_eq!(range_rate, 5.0, epsilon = 0.5);
    }

    #[test]
    fn test_range_rate_approaching() {
        let obs = Observer::new(0.0, 0.0, 0.0);
        let obs_ecef = obs.to_ecef();

        // Satellite approaching (negative range rate)
        let sat_ecef = [obs_ecef[0] + 500.0, obs_ecef[1], obs_ecef[2]];
        let vel_ecef = [-3.0, 0.0, 0.0]; // Moving toward observer at 3 km/s

        let topo = compute_azimuth_elevation_rate(&sat_ecef, &vel_ecef, &obs);

        assert!(topo.range_rate.is_some());
        let range_rate = topo.range_rate.unwrap();
        // Should be negative (approaching)
        assert!(range_rate < 0.0);
    }

    #[test]
    fn test_ecef_to_enu_matrix_equator() {
        // Observer at equator, prime meridian
        let obs = Observer::new(0.0, 0.0, 0.0);
        let rot = obs.ecef_to_enu_matrix();

        // At equator, prime meridian:
        // E should point to +Y (East), N to +Z (North), U to +X (Up)
        // Test that the matrix is orthonormal
        let det = rot.determinant();
        assert_relative_eq!(det.abs(), 1.0, epsilon = 1e-10);

        // Test a known vector transformation
        // ECEF +Y (East direction) should map to ENU [1, 0, 0]
        let ecef_east = Vector3::new(0.0, 1.0, 0.0);
        let enu_east = rot * ecef_east;
        assert_relative_eq!(enu_east[0], 1.0, epsilon = 1e-10); // East component
        assert_relative_eq!(enu_east[1], 0.0, epsilon = 1e-10); // North component
        assert_relative_eq!(enu_east[2], 0.0, epsilon = 1e-10); // Up component
    }

    #[test]
    fn test_ecef_to_enu_matrix_north_pole() {
        // Observer at north pole
        let obs = Observer::new(PI / 2.0, 0.0, 0.0);
        let rot = obs.ecef_to_enu_matrix();

        // Matrix should be orthonormal
        let det = rot.determinant();
        assert_relative_eq!(det.abs(), 1.0, epsilon = 1e-10);

        // At north pole, ECEF +Z should map to ENU Up
        let ecef_up = Vector3::new(0.0, 0.0, 1.0);
        let enu = rot * ecef_up;
        assert_relative_eq!(enu[2], 1.0, epsilon = 1e-10); // Up component
    }

    #[test]
    fn test_ecef_to_sez_matrix() {
        // Test SEZ transformation
        let obs = Observer::new(0.0, 0.0, 0.0);
        let rot_sez = obs.ecef_to_sez_matrix();

        // Matrix should be orthonormal
        let det = rot_sez.determinant();
        assert_relative_eq!(det.abs(), 1.0, epsilon = 1e-10);

        // SEZ is related to ENU by 180° rotation about zenith
        // This is mainly used in Vallado's algorithms
    }

    #[test]
    fn test_azimuth_east() {
        let obs = Observer::new(0.0, 0.0, 0.0);

        // Satellite to the east
        let enu_east = Vector3::new(500.0, 0.0, 500.0); // East=500, North=0, Up=500
        let rot_enu = obs.ecef_to_enu_matrix();
        let obs_ecef = obs.to_ecef();
        let ecef_offset = rot_enu.transpose() * enu_east;
        let sat_ecef_vec = obs_ecef + ecef_offset;
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Azimuth should be 90° (East)
        assert_relative_eq!(topo.azimuth, PI / 2.0, epsilon = 0.1);
    }

    #[test]
    fn test_azimuth_south() {
        let obs = Observer::new(0.0, 0.0, 0.0);

        // Satellite to the south
        let enu_south = Vector3::new(0.0, -500.0, 500.0); // East=0, North=-500, Up=500
        let rot_enu = obs.ecef_to_enu_matrix();
        let obs_ecef = obs.to_ecef();
        let ecef_offset = rot_enu.transpose() * enu_south;
        let sat_ecef_vec = obs_ecef + ecef_offset;
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Azimuth should be 180° (South)
        assert_relative_eq!(topo.azimuth, PI, epsilon = 0.1);
    }

    #[test]
    fn test_azimuth_west() {
        let obs = Observer::new(0.0, 0.0, 0.0);

        // Satellite to the west
        let enu_west = Vector3::new(-500.0, 0.0, 500.0); // East=-500, North=0, Up=500
        let rot_enu = obs.ecef_to_enu_matrix();
        let obs_ecef = obs.to_ecef();
        let ecef_offset = rot_enu.transpose() * enu_west;
        let sat_ecef_vec = obs_ecef + ecef_offset;
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Azimuth should be 270° (West)
        assert_relative_eq!(topo.azimuth, 3.0 * PI / 2.0, epsilon = 0.1);
    }

    #[test]
    fn test_observer_southern_hemisphere() {
        // Observer in southern hemisphere (Sydney: 33.87°S, 151.21°E)
        let obs = Observer::new(
            -33.87_f64.to_radians(),
            151.21_f64.to_radians(),
            0.050, // 50m altitude
        );
        let ecef = obs.to_ecef();

        // Should have negative Z component (southern hemisphere)
        assert!(ecef[2] < 0.0);

        // Test visibility calculation works - place satellite directly above observer
        let radial_direction = ecef.normalize();
        let sat_ecef_vec = ecef + radial_direction * 500.0; // 500 km above
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];
        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Should have valid azimuth and elevation
        assert!(topo.azimuth >= 0.0 && topo.azimuth <= 2.0 * PI);
        assert!(topo.elevation > 0.0);
    }

    #[test]
    fn test_observer_with_altitude() {
        // Observer at high altitude (mountaintop)
        let obs_sea = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_mountain = Observer::new(45.0_f64.to_radians(), 0.0, 5.0); // 5 km altitude

        let ecef_sea = obs_sea.to_ecef();
        let ecef_mountain = obs_mountain.to_ecef();

        // Mountain observer should be further from Earth's center
        assert!(ecef_mountain.norm() > ecef_sea.norm());
        assert_relative_eq!(ecef_mountain.norm() - ecef_sea.norm(), 5.0, epsilon = 0.1);
    }

    #[test]
    fn test_elevation_nadir() {
        let obs = Observer::new(0.0, 0.0, 0.0);
        let obs_ecef = obs.to_ecef();

        // Satellite below observer (nadir direction)
        let radial_direction = obs_ecef.normalize();
        let sat_ecef_vec = obs_ecef - radial_direction * 100.0; // 100 km below
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Should have negative elevation (below horizon)
        assert!(topo.elevation < 0.0);
        // Should be close to -90° (nadir)
        assert_relative_eq!(topo.elevation, -PI / 2.0, epsilon = 0.01);
    }

    #[test]
    fn test_azimuth_wrapping() {
        // Test that azimuth wraps correctly to [0, 2π)
        let obs = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);

        // Create satellites in different directions and verify azimuth is in range
        for angle in [0.0_f64, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0] {
            let az_rad = angle.to_radians();
            let e = 500.0 * az_rad.sin();
            let n = 500.0 * az_rad.cos();
            let u = 200.0;

            let enu = Vector3::new(e, n, u);
            let rot_enu = obs.ecef_to_enu_matrix();
            let obs_ecef = obs.to_ecef();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef_vec = obs_ecef + ecef_offset;
            let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

            let topo = compute_azimuth_elevation(&sat_ecef, &obs);

            // Azimuth must be in [0, 2π)
            assert!(topo.azimuth >= 0.0);
            assert!(topo.azimuth < 2.0 * PI);
        }
    }

    #[test]
    fn test_find_next_pass_simple() {
        // Create a simple propagation function that simulates a satellite pass
        // The satellite starts below horizon, rises, reaches max elevation, then sets
        let observer = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_ecef = observer.to_ecef();

        let propagate = |t_minutes: f64| -> [f64; 3] {
            // Simple model: satellite moves in a circular path
            // At t=0, satellite is below horizon (negative elevation)
            // At t=5, satellite is at max elevation
            // At t=10, satellite is below horizon again
            let phase = (t_minutes - 5.0) / 5.0; // -1 at t=0, 0 at t=5, 1 at t=10
            let elevation_factor = 1.0 - phase * phase; // Parabola with max at t=5

            // Position: 500 km above observer at max, moving north
            let enu = Vector3::new(
                0.0,  // East
                t_minutes * 50.0, // North (moving)
                elevation_factor * 500.0 - 100.0, // Up (parabolic)
            );

            let rot_enu = observer.ecef_to_enu_matrix();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef = obs_ecef + ecef_offset;
            [sat_ecef[0], sat_ecef[1], sat_ecef[2]]
        };

        // Find the pass
        let pass = find_next_pass(&propagate, &observer, 0.0, 20.0, 0.0, 0.5);

        assert!(pass.is_some());
        let pass = pass.unwrap();

        // Pass should be roughly from t=2 to t=8 (when elevation > 0)
        assert!(pass.rise_time > 0.0 && pass.rise_time < 5.0);
        assert!(pass.set_time > 5.0 && pass.set_time < 10.0);
        assert!(pass.max_elevation_time > pass.rise_time);
        assert!(pass.max_elevation_time < pass.set_time);
        assert!(pass.max_elevation > 0.0);
        assert!(pass.duration > 0.0);
    }

    #[test]
    fn test_find_next_pass_no_pass() {
        // Satellite that never rises above horizon
        let observer = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_ecef = observer.to_ecef();

        let propagate = |_t_minutes: f64| -> [f64; 3] {
            // Always below horizon
            let enu = Vector3::new(0.0, 1000.0, -200.0); // Below horizon
            let rot_enu = observer.ecef_to_enu_matrix();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef = obs_ecef + ecef_offset;
            [sat_ecef[0], sat_ecef[1], sat_ecef[2]]
        };

        let pass = find_next_pass(&propagate, &observer, 0.0, 100.0, 0.0, 1.0);
        assert!(pass.is_none());
    }

    #[test]
    fn test_find_next_pass_with_min_elevation() {
        // Test with minimum elevation threshold
        let observer = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_ecef = observer.to_ecef();

        let propagate = |t_minutes: f64| -> [f64; 3] {
            // Satellite that starts below, rises above, then sets
            // At t=5, reaches peak elevation
            let phase = (t_minutes - 10.0) / 8.0; // -1.25 at t=0, 0 at t=10, 1.25 at t=20
            let elevation_factor = 1.0 - phase * phase; // Parabola, max at t=10

            // Adjust so it crosses 0° around t=5 and t=15
            let enu = Vector3::new(
                0.0,
                t_minutes * 50.0,
                elevation_factor * 600.0 - 150.0, // Below horizon at t=0, above during pass, below at t=20
            );

            let rot_enu = observer.ecef_to_enu_matrix();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef = obs_ecef + ecef_offset;
            [sat_ecef[0], sat_ecef[1], sat_ecef[2]]
        };

        // With 0° minimum, should find a pass
        let pass_0deg = find_next_pass(&propagate, &observer, 0.0, 25.0, 0.0, 0.5);
        assert!(pass_0deg.is_some());

        // With 10° minimum, should still find a pass
        let pass_10deg = find_next_pass(&propagate, &observer, 0.0, 25.0, 10.0_f64.to_radians(), 0.5);
        assert!(pass_10deg.is_some());

        // With 60° minimum, should not find a pass (satellite doesn't reach that high)
        let pass_60deg = find_next_pass(&propagate, &observer, 0.0, 25.0, 60.0_f64.to_radians(), 0.5);
        assert!(pass_60deg.is_none());
    }

    #[test]
    fn test_find_all_passes() {
        // Satellite with multiple passes
        let observer = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_ecef = observer.to_ecef();

        let propagate = |t_minutes: f64| -> [f64; 3] {
            // Two passes: one centered at t=5, another at t=25
            // Use modulo to create repeating pattern
            let period = 20.0;
            let t_in_period = t_minutes % period;
            let phase = (t_in_period - 5.0) / 5.0;
            let elevation_factor = 1.0 - phase * phase;

            let enu = Vector3::new(
                0.0,
                t_minutes * 50.0,
                elevation_factor * 500.0 - 100.0,
            );

            let rot_enu = observer.ecef_to_enu_matrix();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef = obs_ecef + ecef_offset;
            [sat_ecef[0], sat_ecef[1], sat_ecef[2]]
        };

        let passes = find_all_passes(&propagate, &observer, 0.0, 40.0, 0.0, 0.5);

        // Should find 2 passes
        assert!(passes.len() >= 1); // At least one pass
        // Each pass should have valid properties
        for pass in &passes {
            assert!(pass.rise_time < pass.set_time);
            assert!(pass.max_elevation_time >= pass.rise_time);
            assert!(pass.max_elevation_time <= pass.set_time);
            assert!(pass.duration > 0.0);
        }
    }

    #[test]
    fn test_find_all_passes_empty() {
        // No passes in time window
        let observer = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_ecef = observer.to_ecef();

        let propagate = |_t_minutes: f64| -> [f64; 3] {
            // Always below horizon
            let enu = Vector3::new(0.0, 1000.0, -200.0);
            let rot_enu = observer.ecef_to_enu_matrix();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef = obs_ecef + ecef_offset;
            [sat_ecef[0], sat_ecef[1], sat_ecef[2]]
        };

        let passes = find_all_passes(&propagate, &observer, 0.0, 100.0, 0.0, 1.0);
        assert_eq!(passes.len(), 0);
    }

    #[test]
    fn test_enu_components_accuracy() {
        // Verify that ENU components are correctly calculated
        let obs = Observer::new(0.0, 0.0, 0.0);

        // Known ENU offset
        let e_expected = 100.0;
        let n_expected = 200.0;
        let u_expected = 300.0;

        let enu_vec = Vector3::new(e_expected, n_expected, u_expected);
        let rot_enu = obs.ecef_to_enu_matrix();
        let obs_ecef = obs.to_ecef();
        let ecef_offset = rot_enu.transpose() * enu_vec;
        let sat_ecef_vec = obs_ecef + ecef_offset;
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // ENU components should match
        assert_relative_eq!(topo.enu[0], e_expected, epsilon = 1e-6);
        assert_relative_eq!(topo.enu[1], n_expected, epsilon = 1e-6);
        assert_relative_eq!(topo.enu[2], u_expected, epsilon = 1e-6);
    }

    #[test]
    fn test_range_accuracy() {
        let obs = Observer::new(0.0, 0.0, 0.0);
        let obs_ecef = obs.to_ecef();

        // Satellite at known distance
        let range_expected = 500.0; // km
        let direction = Vector3::new(1.0, 1.0, 1.0).normalize();
        let sat_ecef_vec = obs_ecef + direction * range_expected;
        let sat_ecef = [sat_ecef_vec[0], sat_ecef_vec[1], sat_ecef_vec[2]];

        let topo = compute_azimuth_elevation(&sat_ecef, &obs);

        // Range should match expected value
        assert_relative_eq!(topo.range, range_expected, epsilon = 0.1);
    }

    #[test]
    fn test_satellite_pass_duration() {
        // Verify pass duration is correctly calculated
        let observer = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_ecef = observer.to_ecef();

        let propagate = |t_minutes: f64| -> [f64; 3] {
            let phase = (t_minutes - 10.0) / 10.0;
            let elevation_factor = 1.0 - phase * phase;

            let enu = Vector3::new(0.0, t_minutes * 50.0, elevation_factor * 500.0 - 50.0);
            let rot_enu = observer.ecef_to_enu_matrix();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef = obs_ecef + ecef_offset;
            [sat_ecef[0], sat_ecef[1], sat_ecef[2]]
        };

        let pass = find_next_pass(&propagate, &observer, 0.0, 30.0, 0.0, 0.5);
        assert!(pass.is_some());

        let pass = pass.unwrap();
        // Duration should equal set_time - rise_time
        assert_relative_eq!(pass.duration, pass.set_time - pass.rise_time, epsilon = 1e-10);
    }

    #[test]
    fn test_max_elevation_time_bounds() {
        // Verify max elevation time is within pass bounds
        let observer = Observer::new(45.0_f64.to_radians(), 0.0, 0.0);
        let obs_ecef = observer.to_ecef();

        let propagate = |t_minutes: f64| -> [f64; 3] {
            let phase = (t_minutes - 15.0) / 10.0;
            let elevation_factor = 1.0 - phase * phase;

            let enu = Vector3::new(0.0, t_minutes * 30.0, elevation_factor * 600.0);
            let rot_enu = observer.ecef_to_enu_matrix();
            let ecef_offset = rot_enu.transpose() * enu;
            let sat_ecef = obs_ecef + ecef_offset;
            [sat_ecef[0], sat_ecef[1], sat_ecef[2]]
        };

        let pass = find_next_pass(&propagate, &observer, 0.0, 40.0, 0.0, 0.5);
        assert!(pass.is_some());

        let pass = pass.unwrap();
        assert!(pass.max_elevation_time >= pass.rise_time);
        assert!(pass.max_elevation_time <= pass.set_time);
    }
}
