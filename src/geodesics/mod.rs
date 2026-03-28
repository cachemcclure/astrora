//! Geodesics module — relativistic light path computation
//!
//! This module provides null geodesic integration in Schwarzschild spacetime,
//! weak-field analytical deflection (Einstein formula), photon sphere geometry,
//! and composite light path building for gravitational lensing visualization.
//!
//! # Physics
//!
//! The Binet equation for null geodesics in Schwarzschild spacetime:
//! ```text
//! d²u/dφ² + u = 3GM/c² · u²    where u = 1/r
//! ```
//!
//! Einstein deflection (weak-field): `α = 4GM/(c²b)` where b = impact parameter
//!
//! # References
//! - Wald, R.M. (1984). *General Relativity*. Ch. 6.
//! - Will, C.M. (2014). "The confrontation between general relativity and experiment."
//!   *Living Rev. Relativity* 17, 4.
//! - Shapiro, I.I. (1964). "Fourth test of general relativity." *Phys. Rev. Lett.* 13, 789.

pub mod deflection;
pub mod lightpath;
pub mod photon_sphere;
pub mod schwarzschild;

pub use deflection::{Deflection, DeflectionResult};
pub use lightpath::{DeflectionEvent, LensingBody, LightPath, LightPathResult};
pub use photon_sphere::{BlackHoleGeometry, PhotonSphere, TrajectoryType};
pub use schwarzschild::{GeodesicState, SchwarzschildGeodesic, SchwarzschildGeodesicResult};
