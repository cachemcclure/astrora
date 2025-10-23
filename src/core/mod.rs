//! Core mathematical structures and utilities for orbital mechanics

// Module declarations
pub mod constants;
pub mod error;
pub mod linalg;
pub mod numerical;
pub mod integrators_static; // High-performance stack-allocated integrators
pub mod fast_math; // Optimized math functions for Lambert solver
pub mod numpy_integration;
pub mod state;
pub mod elements;
pub mod time;
pub mod anomaly;

// Re-export commonly used types for convenience
pub use linalg::{Vector3, Vector6, Matrix3, Matrix6, Rotation3};
pub use error::{PoliastroError, PoliastroResult};
pub use numerical::{newton_raphson, newton_raphson_ratio, rk4_step, dopri5_step, dopri5_integrate};
pub use time::{Epoch, Duration};
pub use integrators_static::{StateVector6, rk4_step_static, propagate_rk4, propagate_rk4_final_only};

// Placeholder modules for future implementation
