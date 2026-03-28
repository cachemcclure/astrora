//! Core mathematical structures and utilities for orbital mechanics

// Module declarations
pub mod anomaly;
pub mod constants;
pub mod elements;
pub mod error;
pub mod fast_math; // Optimized math functions for Lambert solver
pub mod integrators_static; // High-performance stack-allocated integrators
pub mod linalg;
pub mod numerical;
pub mod numpy_integration;
pub mod state;
pub mod time;

// Re-export commonly used types for convenience
pub use error::{PoliastroError, PoliastroResult};
pub use integrators_static::{
    propagate_rk4, propagate_rk4_final_only, rk4_step_static, StateVector6,
};
pub use linalg::{Matrix3, Matrix6, Rotation3, Vector3, Vector6};
pub use numerical::{
    dopri5_integrate, dopri5_step, newton_raphson, newton_raphson_ratio, rk4_step,
};
pub use time::{Duration, Epoch};

// Placeholder modules for future implementation
