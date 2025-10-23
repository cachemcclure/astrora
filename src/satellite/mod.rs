//! Satellite Operations Module
//!
//! This module provides functionality for real-world satellite operations,
//! including SGP4/SDP4 propagation of Two-Line Element (TLE) sets and
//! Orbit Mean-Elements Message (OMM) data.
//!
//! # Overview
//!
//! The SGP4 (Simplified General Perturbations 4) model is the standard
//! algorithm for propagating satellite orbits from TLE data. It includes:
//! - **SGP4**: For near-Earth satellites (period < 225 minutes)
//! - **SDP4**: For deep-space satellites (period ≥ 225 minutes) with lunar/solar perturbations
//!
//! # Key Features
//!
//! - **TLE Parsing**: Support for 2-line and 3-line element formats
//! - **OMM Support**: Modern JSON format for orbital elements
//! - **High Accuracy**: Sub-meter position errors (<0.2m after 3.5 years)
//! - **Performance**: ~7% faster than C++ reference implementation
//! - **Automatic Mode Selection**: Automatically uses SGP4 or SDP4 based on orbit period
//!
//! # Coordinate System
//!
//! SGP4 outputs are in the **TEME** (True Equator, Mean Equinox) reference frame.
//! Use the coordinate transformation module to convert to other frames:
//! - TEME → ITRS (Earth-fixed)
//! - TEME → GCRS (inertial)
//!
//! # References
//!
//! - **Implementation**: Uses `neuromorphicsystems/sgp4` v2.3.0 crate
//! - **Validation**: Celestrak C++ reference implementation
//! - **Standards**:
//!   - Spacetrack Report #3 (1980, revised 2006)
//!   - AIAA 2006-6753 (Vallado et al.)
//! - **TLE Format**: <https://celestrak.org/NORAD/documentation/tle-fmt.php>
//! - **OMM Format**: <https://public.ccsds.org/Pubs/502x0b2c1.pdf>
//!
//! # Example
//!
//! ```rust,ignore
//! use astrora_core::satellite::propagate_tle;
//!
//! // ISS TLE (example)
//! let tle = "ISS (ZARYA)
//! 1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
//! 2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
//!
//! // Propagate to 120 minutes after epoch
//! let state = propagate_tle(tle, 120.0)?;
//! // state.position: [x, y, z] in km (TEME frame)
//! // state.velocity: [vx, vy, vz] in km/s (TEME frame)
//! ```

pub mod sgp4_wrapper;
pub mod tle;
pub mod omm;
pub mod visibility;
pub mod groundtrack;
pub mod coverage;
pub mod eclipse;
pub mod lifetime;
pub mod conjunction;

pub use sgp4_wrapper::{propagate_from_elements, propagate_batch, SatelliteState, Sgp4Error};
pub use tle::parse_tle;
pub use omm::parse_omm;
pub use visibility::{
    Observer, TopocentricCoordinates, SatellitePass,
    compute_azimuth_elevation, compute_azimuth_elevation_rate,
    is_visible, has_line_of_sight,
    find_next_pass, find_all_passes,
};
pub use groundtrack::{
    GeodeticCoordinates, GroundTrackPoint,
    ecef_to_geodetic, sub_satellite_point, compute_ground_track,
    calculate_swath_width, maximum_ground_range,
};
pub use coverage::{
    GeodeticPoint, AccessStatistics,
    visibility_circle, coverage_area,
    compute_access_statistics, coverage_percentage,
};
pub use eclipse::{
    EclipseState,
    compute_eclipse_state, solar_beta_angle, solar_beta_angle_precise,
    sun_synchronous_inclination, eclipse_duration,
};
pub use lifetime::{
    estimate_lifetime, estimate_decay_rate,
    DEFAULT_TERMINAL_ALTITUDE, TYPICAL_DRAG_COEFFICIENT,
};
pub use conjunction::{
    ConjunctionResult,
    compute_conjunction, check_collision, closest_approach_distance,
};
