//! High-level coordinate transformation API
//!
//! This module provides a unified, ergonomic interface for transforming coordinates
//! between different reference frames. It automatically handles multi-step transformations
//! and provides type-safe conversions.
//!
//! # Design
//!
//! The transformation system uses GCRS as a central hub, since:
//! - ICRS ↔ GCRS: Simple (nearly identity for Earth satellites)
//! - J2000 ↔ GCRS: Simple (identity at J2000 epoch)
//! - ITRS ↔ GCRS: ERA rotation + Coriolis
//! - TEME ↔ GCRS: Via ITRS with GMST rotation
//!
//! This hub-and-spoke design minimizes the number of direct transformations needed
//! while ensuring all frames can be converted to each other.
//!
//! # Examples
//!
//! ```rust,ignore
//! use astrora_core::coordinates::frames::{GCRS, ITRS};
//! use astrora_core::coordinates::transform::CoordinateFrame;
//! use nalgebra::Vector3;
//! use hifitime::Epoch;
//!
//! // Create a GCRS coordinate
//! let gcrs = GCRS::new(
//!     Vector3::new(7000e3, 0.0, 0.0),
//!     Vector3::new(0.0, 7500.0, 0.0),
//!     Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0)
//! );
//!
//! // Transform to ITRS
//! let itrs = gcrs.transform_to_itrs().unwrap();
//! ```

use crate::coordinates::frames::{GCRS, ICRS, ITRS, J2000, TEME};
use crate::core::error::PoliastroResult;
use crate::core::time::Epoch;
use nalgebra::Vector3;

/// Enum representing all supported coordinate frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// International Celestial Reference System (barycentric inertial)
    ICRS,
    /// Geocentric Celestial Reference System (geocentric inertial)
    GCRS,
    /// J2000 inertial frame (geocentric, epoch J2000.0)
    J2000,
    /// International Terrestrial Reference System (Earth-fixed, rotating)
    ITRS,
    /// True Equator Mean Equinox (geocentric inertial, legacy SGP4)
    TEME,
}

impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameType::ICRS => write!(f, "ICRS"),
            FrameType::GCRS => write!(f, "GCRS"),
            FrameType::J2000 => write!(f, "J2000"),
            FrameType::ITRS => write!(f, "ITRS"),
            FrameType::TEME => write!(f, "TEME"),
        }
    }
}

/// Trait for coordinate frames that can be transformed to other frames
///
/// This trait provides a unified interface for all coordinate transformations.
/// Each frame type implements methods to get its position, velocity, and convert
/// to other frames via the GCRS hub.
pub trait CoordinateFrame: Sized {
    /// Get the frame type
    fn frame_type(&self) -> FrameType;

    /// Get the position vector in this frame
    fn position(&self) -> Vector3<f64>;

    /// Get the velocity vector in this frame
    fn velocity(&self) -> Vector3<f64>;

    /// Get the observation time (epoch) for this coordinate
    /// Returns None for frames without a time attribute (like ICRS in V1)
    fn obstime(&self) -> Option<Epoch>;

    /// Convert this frame to GCRS (central hub for all transformations)
    fn to_gcrs_frame(&self) -> PoliastroResult<GCRS>;

    /// Create this frame from GCRS
    fn from_gcrs_frame(gcrs: &GCRS) -> PoliastroResult<Self>;

    /// Transform to ICRS (barycentric inertial)
    fn transform_to_icrs(&self) -> PoliastroResult<ICRS> {
        let gcrs = self.to_gcrs_frame()?;
        gcrs.to_icrs()
    }

    /// Transform to GCRS (geocentric inertial)
    fn transform_to_gcrs(&self) -> PoliastroResult<GCRS> {
        self.to_gcrs_frame()
    }

    /// Transform to J2000 (standard epoch inertial)
    fn transform_to_j2000(&self) -> PoliastroResult<J2000> {
        let gcrs = self.to_gcrs_frame()?;
        Ok(J2000::from_gcrs(&gcrs))
    }

    /// Transform to ITRS (Earth-fixed rotating)
    fn transform_to_itrs(&self) -> PoliastroResult<ITRS> {
        let gcrs = self.to_gcrs_frame()?;
        gcrs.to_itrs()
    }

    /// Transform to TEME (legacy SGP4 frame)
    fn transform_to_teme(&self) -> PoliastroResult<TEME> {
        let gcrs = self.to_gcrs_frame()?;
        // TEME doesn't have from_gcrs, must go via ITRS
        let itrs = gcrs.to_itrs()?;
        itrs.to_teme()
    }
}

// Implement CoordinateFrame for ICRS
impl CoordinateFrame for ICRS {
    fn frame_type(&self) -> FrameType {
        FrameType::ICRS
    }

    fn position(&self) -> Vector3<f64> {
        self.position
    }

    fn velocity(&self) -> Vector3<f64> {
        self.velocity
    }

    fn obstime(&self) -> Option<Epoch> {
        None // ICRS doesn't have a time attribute in V1
    }

    fn to_gcrs_frame(&self) -> PoliastroResult<GCRS> {
        // Need an epoch for GCRS - use J2000 as default
        self.to_gcrs(&Epoch::j2000()) // J2000.0
    }

    fn from_gcrs_frame(gcrs: &GCRS) -> PoliastroResult<Self> {
        gcrs.to_icrs()
    }
}

// Implement CoordinateFrame for GCRS
impl CoordinateFrame for GCRS {
    fn frame_type(&self) -> FrameType {
        FrameType::GCRS
    }

    fn position(&self) -> Vector3<f64> {
        self.position
    }

    fn velocity(&self) -> Vector3<f64> {
        self.velocity
    }

    fn obstime(&self) -> Option<Epoch> {
        Some(self.obstime)
    }

    fn to_gcrs_frame(&self) -> PoliastroResult<GCRS> {
        Ok(self.clone())
    }

    fn from_gcrs_frame(gcrs: &GCRS) -> PoliastroResult<Self> {
        Ok(gcrs.clone())
    }
}

// Implement CoordinateFrame for J2000
impl CoordinateFrame for J2000 {
    fn frame_type(&self) -> FrameType {
        FrameType::J2000
    }

    fn position(&self) -> Vector3<f64> {
        self.position
    }

    fn velocity(&self) -> Vector3<f64> {
        self.velocity
    }

    fn obstime(&self) -> Option<Epoch> {
        Some(Epoch::j2000())
    }

    fn to_gcrs_frame(&self) -> PoliastroResult<GCRS> {
        Ok(self.to_gcrs())
    }

    fn from_gcrs_frame(gcrs: &GCRS) -> PoliastroResult<Self> {
        Ok(J2000::from_gcrs(gcrs))
    }
}

// Implement CoordinateFrame for ITRS
impl CoordinateFrame for ITRS {
    fn frame_type(&self) -> FrameType {
        FrameType::ITRS
    }

    fn position(&self) -> Vector3<f64> {
        self.position
    }

    fn velocity(&self) -> Vector3<f64> {
        self.velocity
    }

    fn obstime(&self) -> Option<Epoch> {
        Some(self.obstime)
    }

    fn to_gcrs_frame(&self) -> PoliastroResult<GCRS> {
        self.to_gcrs()
    }

    fn from_gcrs_frame(gcrs: &GCRS) -> PoliastroResult<Self> {
        gcrs.to_itrs()
    }
}

// Implement CoordinateFrame for TEME
impl CoordinateFrame for TEME {
    fn frame_type(&self) -> FrameType {
        FrameType::TEME
    }

    fn position(&self) -> Vector3<f64> {
        self.position
    }

    fn velocity(&self) -> Vector3<f64> {
        self.velocity
    }

    fn obstime(&self) -> Option<Epoch> {
        Some(self.obstime)
    }

    fn to_gcrs_frame(&self) -> PoliastroResult<GCRS> {
        self.to_gcrs()
    }

    fn from_gcrs_frame(gcrs: &GCRS) -> PoliastroResult<Self> {
        // TEME doesn't have from_gcrs, must go via ITRS
        let itrs = gcrs.to_itrs()?;
        itrs.to_teme()
    }
}

/// Generic transformation function that can transform any frame to any other frame
///
/// This function provides a convenient way to transform coordinates without knowing
/// the specific types at compile time. It uses the frame type enum to dispatch to
/// the appropriate transformation method.
///
/// # Example
///
/// ```rust,ignore
/// use astrora_core::coordinates::transform::{transform, FrameType};
/// use astrora_core::coordinates::frames::GCRS;
///
/// let gcrs = GCRS::new(...);
/// let itrs_pos = transform(&gcrs, FrameType::ITRS)?;
/// ```
pub fn transform_position_velocity<F: CoordinateFrame>(
    from_frame: &F,
    to_frame_type: FrameType,
) -> PoliastroResult<(Vector3<f64>, Vector3<f64>)> {
    match to_frame_type {
        FrameType::ICRS => {
            let icrs = from_frame.transform_to_icrs()?;
            Ok((*icrs.position(), *icrs.velocity()))
        }
        FrameType::GCRS => {
            let gcrs = from_frame.transform_to_gcrs()?;
            Ok((*gcrs.position(), *gcrs.velocity()))
        }
        FrameType::J2000 => {
            let j2000 = from_frame.transform_to_j2000()?;
            Ok((*j2000.position(), *j2000.velocity()))
        }
        FrameType::ITRS => {
            let itrs = from_frame.transform_to_itrs()?;
            Ok((*itrs.position(), *itrs.velocity()))
        }
        FrameType::TEME => {
            let teme = from_frame.transform_to_teme()?;
            Ok((*teme.position(), *teme.velocity()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_gcrs_to_icrs_transform() {
        let epoch = Epoch::j2000(); // J2000.0
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let icrs = gcrs.transform_to_icrs().unwrap();

        // At J2000, GCRS ≈ ICRS (barycentric correction is small for Earth satellites)
        assert_abs_diff_eq!(icrs.position().norm(), gcrs.position().norm(), epsilon = 1.0);
        assert_abs_diff_eq!(icrs.velocity().norm(), gcrs.velocity().norm(), epsilon = 0.1);
    }

    #[test]
    fn test_gcrs_to_j2000_transform() {
        let epoch = Epoch::j2000(); // J2000.0
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let j2000 = gcrs.transform_to_j2000().unwrap();

        // At J2000 epoch, GCRS and J2000 should be identical
        assert_abs_diff_eq!(j2000.position(), gcrs.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(j2000.velocity(), gcrs.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_gcrs_to_itrs_transform() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let itrs = gcrs.transform_to_itrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(itrs.position().norm(), gcrs.position().norm(), epsilon = 1e-6);

        // Velocity will differ due to Earth rotation and Coriolis effect
        assert!(itrs.velocity().norm() > 0.0);
    }

    #[test]
    fn test_gcrs_to_teme_transform() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let teme = gcrs.transform_to_teme().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(teme.position().norm(), gcrs.position().norm(), epsilon = 1.0);

        // TEME and GCRS are both inertial, so velocities should be similar
        assert_abs_diff_eq!(teme.velocity().norm(), gcrs.velocity().norm(), epsilon = 100.0);
    }

    #[test]
    fn test_roundtrip_gcrs_itrs() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs_orig = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let itrs = gcrs_orig.transform_to_itrs().unwrap();
        let gcrs_back = itrs.transform_to_gcrs().unwrap();

        // Roundtrip should preserve position and velocity
        assert_abs_diff_eq!(gcrs_back.position(), gcrs_orig.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(gcrs_back.velocity(), gcrs_orig.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_transform_position_velocity() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        // Transform to ITRS using generic function
        let (pos, vel) = transform_position_velocity(&gcrs, FrameType::ITRS).unwrap();

        // Compare with direct transformation
        let itrs = gcrs.transform_to_itrs().unwrap();
        assert_abs_diff_eq!(pos, itrs.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(vel, itrs.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_frame_type_display() {
        assert_eq!(FrameType::ICRS.to_string(), "ICRS");
        assert_eq!(FrameType::GCRS.to_string(), "GCRS");
        assert_eq!(FrameType::J2000.to_string(), "J2000");
        assert_eq!(FrameType::ITRS.to_string(), "ITRS");
        assert_eq!(FrameType::TEME.to_string(), "TEME");
    }

    #[test]
    fn test_multi_hop_transformation() {
        // Test TEME → GCRS → J2000 path
        // Use frames that all preserve time information
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        // Test roundtrip: GCRS → TEME → GCRS
        let teme = gcrs.transform_to_teme().unwrap();
        let gcrs_back = teme.transform_to_gcrs().unwrap();

        // Roundtrip should preserve position and velocity (within tolerance for multi-hop)
        assert_abs_diff_eq!(
            *gcrs_back.position(),
            *gcrs.position(),
            epsilon = 1.0 // Meter-level precision
        );
        assert_abs_diff_eq!(
            *gcrs_back.velocity(),
            *gcrs.velocity(),
            epsilon = 0.01 // cm/s precision
        );
    }

    // Test frame_type() methods for all frames
    #[test]
    fn test_icrs_frame_type() {
        let icrs = ICRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );
        assert_eq!(icrs.frame_type(), FrameType::ICRS);
    }

    #[test]
    fn test_gcrs_frame_type() {
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            Epoch::j2000(),
        );
        assert_eq!(gcrs.frame_type(), FrameType::GCRS);
    }

    #[test]
    fn test_j2000_frame_type() {
        let j2000 = J2000::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );
        assert_eq!(j2000.frame_type(), FrameType::J2000);
    }

    #[test]
    fn test_itrs_frame_type() {
        let itrs = ITRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            Epoch::j2000(),
        );
        assert_eq!(itrs.frame_type(), FrameType::ITRS);
    }

    #[test]
    fn test_teme_frame_type() {
        let teme = TEME::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            Epoch::j2000(),
        );
        assert_eq!(teme.frame_type(), FrameType::TEME);
    }

    // Test position() and velocity() accessor methods
    #[test]
    fn test_icrs_accessors() {
        let pos = Vector3::new(7000e3, 1000e3, 2000e3);
        let vel = Vector3::new(0.0, 7500.0, 100.0);
        let icrs = ICRS::new(pos, vel);

        assert_abs_diff_eq!(*icrs.position(), pos, epsilon = 1e-6);
        assert_abs_diff_eq!(*icrs.velocity(), vel, epsilon = 1e-6);
        assert!(icrs.obstime().is_none());
    }

    #[test]
    fn test_gcrs_accessors() {
        let pos = Vector3::new(7000e3, 1000e3, 2000e3);
        let vel = Vector3::new(0.0, 7500.0, 100.0);
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(pos, vel, epoch);

        assert_abs_diff_eq!(*gcrs.position(), pos, epsilon = 1e-6);
        assert_abs_diff_eq!(*gcrs.velocity(), vel, epsilon = 1e-6);
        assert_eq!(gcrs.frame_type(), FrameType::GCRS);
    }

    #[test]
    fn test_j2000_accessors() {
        let pos = Vector3::new(7000e3, 1000e3, 2000e3);
        let vel = Vector3::new(0.0, 7500.0, 100.0);
        let j2000 = J2000::new(pos, vel);

        assert_abs_diff_eq!(*j2000.position(), pos, epsilon = 1e-6);
        assert_abs_diff_eq!(*j2000.velocity(), vel, epsilon = 1e-6);
        assert_eq!(j2000.obstime(), Some(Epoch::j2000()));
    }

    #[test]
    fn test_itrs_accessors() {
        let pos = Vector3::new(7000e3, 1000e3, 2000e3);
        let vel = Vector3::new(0.0, 7500.0, 100.0);
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let itrs = ITRS::new(pos, vel, epoch);

        assert_abs_diff_eq!(*itrs.position(), pos, epsilon = 1e-6);
        assert_abs_diff_eq!(*itrs.velocity(), vel, epsilon = 1e-6);
        assert_eq!(itrs.frame_type(), FrameType::ITRS);
    }

    #[test]
    fn test_teme_accessors() {
        let pos = Vector3::new(7000e3, 1000e3, 2000e3);
        let vel = Vector3::new(0.0, 7500.0, 100.0);
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let teme = TEME::new(pos, vel, epoch);

        assert_abs_diff_eq!(*teme.position(), pos, epsilon = 1e-6);
        assert_abs_diff_eq!(*teme.velocity(), vel, epsilon = 1e-6);
        assert_eq!(teme.frame_type(), FrameType::TEME);
    }

    // Test ICRS transformations
    #[test]
    fn test_icrs_to_gcrs() {
        let icrs = ICRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let gcrs = icrs.transform_to_gcrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(gcrs.position().norm(), icrs.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_icrs_to_j2000() {
        let icrs = ICRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let j2000 = icrs.transform_to_j2000().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(j2000.position().norm(), icrs.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_icrs_to_itrs() {
        let icrs = ICRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let itrs = icrs.transform_to_itrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(itrs.position().norm(), icrs.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_icrs_to_teme() {
        let icrs = ICRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let teme = icrs.transform_to_teme().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(teme.position().norm(), icrs.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_icrs_to_icrs() {
        let icrs = ICRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let icrs2 = icrs.transform_to_icrs().unwrap();

        // Should be nearly identical (roundtrip through GCRS at J2000)
        assert_abs_diff_eq!(icrs2.position(), icrs.position(), epsilon = 1.0);
        assert_abs_diff_eq!(icrs2.velocity(), icrs.velocity(), epsilon = 0.1);
    }

    // Test J2000 transformations
    #[test]
    fn test_j2000_to_icrs() {
        let j2000 = J2000::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let icrs = j2000.transform_to_icrs().unwrap();

        // At J2000 epoch, should be very similar
        assert_abs_diff_eq!(icrs.position().norm(), j2000.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_j2000_to_itrs() {
        let j2000 = J2000::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let itrs = j2000.transform_to_itrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(itrs.position().norm(), j2000.position().norm(), epsilon = 1e-6);
    }

    #[test]
    fn test_j2000_to_teme() {
        let j2000 = J2000::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
        );

        let teme = j2000.transform_to_teme().unwrap();

        // Both are inertial, magnitude should be conserved
        assert_abs_diff_eq!(teme.position().norm(), j2000.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_j2000_roundtrip() {
        let j2000_orig = J2000::new(
            Vector3::new(7000e3, 1000e3, 2000e3),
            Vector3::new(100.0, 7500.0, 200.0),
        );

        let gcrs = j2000_orig.transform_to_gcrs().unwrap();
        let j2000_back = gcrs.transform_to_j2000().unwrap();

        // Roundtrip should be exact for J2000
        assert_abs_diff_eq!(j2000_back.position(), j2000_orig.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(j2000_back.velocity(), j2000_orig.velocity(), epsilon = 1e-6);
    }

    // Test ITRS transformations
    #[test]
    fn test_itrs_to_icrs() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let itrs = ITRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let icrs = itrs.transform_to_icrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(icrs.position().norm(), itrs.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_itrs_to_j2000() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let itrs = ITRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let j2000 = itrs.transform_to_j2000().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(j2000.position().norm(), itrs.position().norm(), epsilon = 1e-6);
    }

    #[test]
    fn test_itrs_to_teme() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let itrs = ITRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let teme = itrs.transform_to_teme().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(teme.position().norm(), itrs.position().norm(), epsilon = 1e-6);
    }

    // Test TEME transformations
    #[test]
    fn test_teme_to_icrs() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let teme = TEME::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let icrs = teme.transform_to_icrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(icrs.position().norm(), teme.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_teme_to_j2000() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let teme = TEME::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let j2000 = teme.transform_to_j2000().unwrap();

        // Both inertial, magnitude should be conserved
        assert_abs_diff_eq!(j2000.position().norm(), teme.position().norm(), epsilon = 1.0);
    }

    #[test]
    fn test_teme_to_itrs() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let teme = TEME::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let itrs = teme.transform_to_itrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(itrs.position().norm(), teme.position().norm(), epsilon = 1e-6);
    }

    #[test]
    fn test_teme_to_teme() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let teme = TEME::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let teme2 = teme.transform_to_teme().unwrap();

        // Roundtrip should preserve values
        assert_abs_diff_eq!(teme2.position(), teme.position(), epsilon = 1.0);
        assert_abs_diff_eq!(teme2.velocity(), teme.velocity(), epsilon = 0.01);
    }

    // Test transform_position_velocity for all frame types
    #[test]
    fn test_transform_position_velocity_to_icrs() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let (pos, vel) = transform_position_velocity(&gcrs, FrameType::ICRS).unwrap();

        // Compare with direct transformation
        let icrs = gcrs.transform_to_icrs().unwrap();
        assert_abs_diff_eq!(pos, icrs.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(vel, icrs.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_transform_position_velocity_to_gcrs() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let itrs = ITRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let (pos, vel) = transform_position_velocity(&itrs, FrameType::GCRS).unwrap();

        // Compare with direct transformation
        let gcrs = itrs.transform_to_gcrs().unwrap();
        assert_abs_diff_eq!(pos, gcrs.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(vel, gcrs.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_transform_position_velocity_to_j2000() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let (pos, vel) = transform_position_velocity(&gcrs, FrameType::J2000).unwrap();

        // Compare with direct transformation
        let j2000 = gcrs.transform_to_j2000().unwrap();
        assert_abs_diff_eq!(pos, j2000.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(vel, j2000.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_transform_position_velocity_to_teme() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let (pos, vel) = transform_position_velocity(&gcrs, FrameType::TEME).unwrap();

        // Compare with direct transformation
        let teme = gcrs.transform_to_teme().unwrap();
        assert_abs_diff_eq!(pos, teme.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(vel, teme.velocity(), epsilon = 1e-6);
    }

    // Test with different epochs
    #[test]
    fn test_gcrs_to_itrs_different_epoch() {
        let epoch = Epoch::from_gregorian_utc(2020, 1, 1, 0, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let itrs = gcrs.transform_to_itrs().unwrap();

        // Position magnitude should be conserved
        assert_abs_diff_eq!(itrs.position().norm(), gcrs.position().norm(), epsilon = 1e-6);
        // Frame type should be correct
        assert_eq!(itrs.frame_type(), FrameType::ITRS);
    }

    // Test from_gcrs_frame and to_gcrs_frame consistency
    #[test]
    fn test_gcrs_from_to_consistency() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs_orig = GCRS::new(
            Vector3::new(7000e3, 1000e3, 2000e3),
            Vector3::new(100.0, 7500.0, 200.0),
            epoch,
        );

        // to_gcrs_frame on GCRS should return clone
        let gcrs_copy = gcrs_orig.to_gcrs_frame().unwrap();
        assert_abs_diff_eq!(gcrs_copy.position(), gcrs_orig.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(gcrs_copy.velocity(), gcrs_orig.velocity(), epsilon = 1e-6);

        // from_gcrs_frame on GCRS should return clone
        let gcrs_from = GCRS::from_gcrs_frame(&gcrs_orig).unwrap();
        assert_abs_diff_eq!(gcrs_from.position(), gcrs_orig.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(gcrs_from.velocity(), gcrs_orig.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_itrs_from_to_consistency() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let itrs = ITRS::from_gcrs_frame(&gcrs).unwrap();
        let gcrs_back = itrs.to_gcrs_frame().unwrap();

        // Roundtrip should preserve values
        assert_abs_diff_eq!(gcrs_back.position(), gcrs.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(gcrs_back.velocity(), gcrs.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_j2000_from_to_consistency() {
        let epoch = Epoch::j2000();
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let j2000 = J2000::from_gcrs_frame(&gcrs).unwrap();
        let gcrs_back = j2000.to_gcrs_frame().unwrap();

        // At J2000, should be nearly identical
        assert_abs_diff_eq!(gcrs_back.position(), gcrs.position(), epsilon = 1e-6);
        assert_abs_diff_eq!(gcrs_back.velocity(), gcrs.velocity(), epsilon = 1e-6);
    }

    #[test]
    fn test_teme_from_to_consistency() {
        let epoch = Epoch::from_gregorian_utc(2025, 10, 22, 12, 0, 0, 0);
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let teme = TEME::from_gcrs_frame(&gcrs).unwrap();
        let gcrs_back = teme.to_gcrs_frame().unwrap();

        // Roundtrip should preserve values (within tolerance for TEME path via ITRS)
        assert_abs_diff_eq!(gcrs_back.position(), gcrs.position(), epsilon = 1.0);
        assert_abs_diff_eq!(gcrs_back.velocity(), gcrs.velocity(), epsilon = 0.01);
    }

    #[test]
    fn test_icrs_from_to_consistency() {
        let epoch = Epoch::j2000();
        let gcrs = GCRS::new(
            Vector3::new(7000e3, 0.0, 0.0),
            Vector3::new(0.0, 7500.0, 0.0),
            epoch,
        );

        let icrs = ICRS::from_gcrs_frame(&gcrs).unwrap();
        let gcrs_back = icrs.to_gcrs_frame().unwrap();

        // At J2000, should be very similar (barycentric correction is small)
        assert_abs_diff_eq!(gcrs_back.position().norm(), gcrs.position().norm(), epsilon = 1.0);
        assert_abs_diff_eq!(gcrs_back.velocity().norm(), gcrs.velocity().norm(), epsilon = 0.1);
    }
}
