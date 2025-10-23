//! SGP4/SDP4 Propagator Wrapper
//!
//! This module wraps the `sgp4` crate and provides a clean interface
//! for satellite orbit propagation using TLE (Two-Line Element) data.
//!
//! # Algorithm
//!
//! The SGP4/SDP4 algorithm automatically selects the appropriate model:
//! - **SGP4**: Near-Earth orbits (period < 225 minutes)
//!   - Simplified atmospheric drag model
//!   - Simplified gravity perturbations
//! - **SDP4**: Deep-space orbits (period ≥ 225 minutes)
//!   - Lunar and solar gravitational perturbations
//!   - Resonance effects for 12-hour and 24-hour orbits
//!
//! # Accuracy
//!
//! Based on Celestrak validation:
//! - Position error: < 0.2 meters after 3.5 years
//! - Velocity error: < 10⁻⁹ km/s after 3.5 years
//! - Performance: ~7% faster than C++ reference
//!
//! # Limitations
//!
//! - **Not suitable for high-precision applications** (use numerical integrators instead)
//! - **Accuracy degrades over time** (refit TLEs every few days)
//! - **No maneuver modeling** (assumes natural orbital evolution only)
//! - **Simplified perturbation models** (good for operational planning, not for science)
//!
//! # References
//!
//! - Hoots & Roehrich (1980, revised 2006): Spacetrack Report #3
//! - Vallado et al. (2006): AIAA 2006-6753 "Revisiting Spacetrack Report #3"

use sgp4::{Constants, Elements, MinutesSinceEpoch};
use thiserror::Error;
use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;

/// Errors that can occur during SGP4 propagation
#[derive(Error, Debug)]
pub enum Sgp4Error {
    #[error("Invalid orbital elements: {0}")]
    InvalidElements(String),

    #[error("Propagation failed: {0}")]
    PropagationFailed(String),

    #[error("TLE parsing failed: {0}")]
    TleParsingFailed(String),

    #[error("OMM parsing failed: {0}")]
    OmmParsingFailed(String),

    #[error("Time offset out of range: {0} minutes")]
    TimeOutOfRange(f64),
}

// Implement conversion from Sgp4Error to PyErr for Python bindings
impl From<Sgp4Error> for PyErr {
    fn from(err: Sgp4Error) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

/// Satellite state in TEME (True Equator, Mean Equinox) reference frame
///
/// # Coordinate System
///
/// TEME is the native output frame of SGP4. It is an inertial frame with:
/// - Origin: Earth's center of mass
/// - X-Y plane: Earth's true equator (includes nutation)
/// - X-axis: Mean vernal equinox (no nutation)
/// - Z-axis: True celestial pole
///
/// To convert to other frames, use the coordinate transformation module:
/// - TEME → ITRS: For ground station visibility
/// - TEME → GCRS: For interplanetary mission planning
///
/// # Units
///
/// - Position: kilometers \[km\]
/// - Velocity: kilometers per second \[km/s\]
#[derive(Debug, Clone)]
pub struct SatelliteState {
    /// Position vector [x, y, z] in TEME frame (km)
    pub position: [f64; 3],

    /// Velocity vector [vx, vy, vz] in TEME frame (km/s)
    pub velocity: [f64; 3],

    /// Time offset from TLE epoch (minutes)
    pub time_offset_minutes: f64,
}

impl SatelliteState {
    /// Get position magnitude (orbital radius) in km
    pub fn position_magnitude(&self) -> f64 {
        (self.position[0].powi(2) + self.position[1].powi(2) + self.position[2].powi(2)).sqrt()
    }

    /// Get velocity magnitude (orbital speed) in km/s
    pub fn velocity_magnitude(&self) -> f64 {
        (self.velocity[0].powi(2) + self.velocity[1].powi(2) + self.velocity[2].powi(2)).sqrt()
    }
}

/// Propagate satellite orbit from TLE elements
///
/// # Arguments
///
/// * `elements` - Parsed TLE orbital elements
/// * `time_offset_minutes` - Time offset from TLE epoch in minutes
///
/// # Returns
///
/// Satellite state in TEME frame at the requested time
///
/// # Errors
///
/// - `InvalidElements`: If orbital elements are physically impossible
/// - `PropagationFailed`: If SGP4 algorithm fails (e.g., orbit decayed)
/// - `TimeOutOfRange`: If time offset is too large (>1000 days is questionable)
///
/// # Example
///
/// ```rust,ignore
/// use sgp4::Elements;
/// use astrora_core::satellite::propagate_from_elements;
///
/// let elements = Elements { /* ... */ };
/// let state = propagate_from_elements(&elements, 120.0)?;
/// println!("Position: {:?} km", state.position);
/// ```
pub fn propagate_from_elements(
    elements: &Elements,
    time_offset_minutes: f64,
) -> Result<SatelliteState, Sgp4Error> {
    // Sanity check on time offset (>1000 days is questionable for TLE accuracy)
    const MAX_MINUTES: f64 = 1000.0 * 24.0 * 60.0; // 1000 days
    if time_offset_minutes.abs() > MAX_MINUTES {
        return Err(Sgp4Error::TimeOutOfRange(time_offset_minutes));
    }

    // Initialize propagator constants from elements
    let constants = Constants::from_elements(elements)
        .map_err(|e| Sgp4Error::InvalidElements(e.to_string()))?;

    // Propagate to requested time
    let prediction = constants
        .propagate(MinutesSinceEpoch(time_offset_minutes))
        .map_err(|e| Sgp4Error::PropagationFailed(e.to_string()))?;

    Ok(SatelliteState {
        position: prediction.position,
        velocity: prediction.velocity,
        time_offset_minutes,
    })
}

/// Batch propagation for multiple time offsets
///
/// More efficient than calling `propagate_from_elements` repeatedly,
/// as it only initializes the propagator constants once.
///
/// # Arguments
///
/// * `elements` - Parsed TLE orbital elements
/// * `time_offsets_minutes` - Array of time offsets from epoch (minutes)
///
/// # Returns
///
/// Vector of satellite states in TEME frame
///
/// # Example
///
/// ```rust,ignore
/// let time_offsets = vec![0.0, 60.0, 120.0, 180.0]; // 0, 1, 2, 3 hours
/// let states = propagate_batch(&elements, &time_offsets)?;
/// ```
pub fn propagate_batch(
    elements: &Elements,
    time_offsets_minutes: &[f64],
) -> Result<Vec<SatelliteState>, Sgp4Error> {
    // Initialize propagator once
    let constants = Constants::from_elements(elements)
        .map_err(|e| Sgp4Error::InvalidElements(e.to_string()))?;

    // Propagate to all requested times
    time_offsets_minutes
        .iter()
        .map(|&offset| {
            // Sanity check
            const MAX_MINUTES: f64 = 1000.0 * 24.0 * 60.0;
            if offset.abs() > MAX_MINUTES {
                return Err(Sgp4Error::TimeOutOfRange(offset));
            }

            let prediction = constants
                .propagate(MinutesSinceEpoch(offset))
                .map_err(|e| Sgp4Error::PropagationFailed(e.to_string()))?;

            Ok(SatelliteState {
                position: prediction.position,
                velocity: prediction.velocity,
                time_offset_minutes: offset,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ISS TLE from 2008-09-20 (Celestrak validation dataset)
    fn get_test_iss_elements() -> Elements {
        // Use TLE parsing instead of manual construction
        let tle = "ISS (ZARYA)\n\
                   1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927\n\
                   2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";

        sgp4::parse_3les(tle).unwrap().into_iter().next().unwrap()
    }

    #[test]
    fn test_propagate_at_epoch() {
        let elements = get_test_iss_elements();
        let state = propagate_from_elements(&elements, 0.0).unwrap();

        // At epoch, should be at reasonable LEO altitude
        let altitude_km = state.position_magnitude() - 6378.137; // Earth radius
        assert!(altitude_km > 300.0 && altitude_km < 500.0,
                "ISS altitude should be ~400 km, got {}", altitude_km);

        // Velocity should be ~7.7 km/s for LEO
        let speed = state.velocity_magnitude();
        assert!(speed > 7.0 && speed < 8.0,
                "ISS speed should be ~7.7 km/s, got {}", speed);
    }

    #[test]
    fn test_propagate_one_orbit() {
        let elements = get_test_iss_elements();

        // ISS orbital period ~90 minutes
        let period_minutes = 1440.0 / elements.mean_motion; // mean_motion is revs/day

        let state_epoch = propagate_from_elements(&elements, 0.0).unwrap();
        let state_one_orbit = propagate_from_elements(&elements, period_minutes).unwrap();

        // Position should be similar (not exact due to perturbations)
        let pos_diff = (
            (state_epoch.position[0] - state_one_orbit.position[0]).powi(2) +
            (state_epoch.position[1] - state_one_orbit.position[1]).powi(2) +
            (state_epoch.position[2] - state_one_orbit.position[2]).powi(2)
        ).sqrt();

        // Should be within 100 km after one orbit (perturbations cause drift)
        assert!(pos_diff < 100.0, "Position drift after one orbit: {} km", pos_diff);
    }

    #[test]
    fn test_batch_propagation() {
        let elements = get_test_iss_elements();
        let time_offsets = vec![0.0, 30.0, 60.0, 90.0, 120.0];

        let states = propagate_batch(&elements, &time_offsets).unwrap();
        assert_eq!(states.len(), 5);

        // Check that time offsets are correctly stored
        for (i, state) in states.iter().enumerate() {
            assert_eq!(state.time_offset_minutes, time_offsets[i]);
        }
    }

    #[test]
    fn test_time_out_of_range() {
        let elements = get_test_iss_elements();

        // Try to propagate >1000 days (should fail)
        let result = propagate_from_elements(&elements, 1500.0 * 24.0 * 60.0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Sgp4Error::TimeOutOfRange(_)));
    }
}
