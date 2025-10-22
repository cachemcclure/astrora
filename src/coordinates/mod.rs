//! Coordinate system transformations and reference frames
//!
//! This module provides coordinate reference frames and transformations between them.
//!
//! # Available Frames
//! - **ICRS**: International Celestial Reference System (barycentric, inertial)
//! - **GCRS**: Geocentric Celestial Reference System (geocentric, inertial)
//! - **J2000**: J2000 inertial reference frame (geocentric, inertial at J2000.0 epoch)
//! - **ITRS**: International Terrestrial Reference System (Earth-fixed, rotating)
//! - **TEME**: True Equator Mean Equinox (geocentric, inertial, legacy SGP4 frame)
//! - **Perifocal**: Perifocal coordinate frame (PQW - orbital plane coordinates)
//!
//! # Rotation Matrices
//!
//! The `rotations` module provides standardized rotation matrix functions for coordinate
//! transformations. All functions use nalgebra's Matrix3 for optimal performance.
//!
//! # Examples
//! ```rust,ignore
//! use astrora_core::coordinates::frames::{ICRS, GCRS, J2000, ITRS};
//! use astrora_core::coordinates::rotations::{rotation_x, rotation_y, rotation_z};
//! use nalgebra::Vector3;
//! use std::f64::consts::PI;
//!
//! // Create a GCRS position for an Earth satellite
//! let pos = Vector3::new(7000000.0, 0.0, 0.0);
//! let vel = Vector3::new(0.0, 7500.0, 0.0);
//! let epoch = Epoch::j2000();
//! let gcrs = GCRS::new(pos, vel, epoch);
//!
//! // Convert to ICRS (barycentric)
//! let icrs = gcrs.to_icrs().unwrap();
//!
//! // Convert to J2000 (standard epoch reference)
//! let j2000 = J2000::from_gcrs(&gcrs);
//!
//! // Convert to ITRS (Earth-fixed)
//! let itrs = gcrs.to_itrs().unwrap();
//!
//! // Use rotation matrices directly
//! let rz = rotation_z(PI / 4.0);  // 45° rotation about Z-axis
//! let v_rotated = rz * pos;
//! ```

pub mod frames;
pub mod rotations;
pub mod precession_nutation;
pub mod transform;

// Re-export commonly used types
pub use frames::{ICRS, GCRS, J2000, ITRS, TEME, Perifocal};
pub use precession_nutation::{
    PrecessionAngles,
    PrecessionNutationError,
    iau2006_precession,
    fukushima_williams_to_matrix,
    iau2006_precession_matrix,
    simplified_nutation_matrix,
    iau2006_precession_nutation_matrix,
};
pub use transform::{
    CoordinateFrame,
    FrameType,
    transform_position_velocity,
};
