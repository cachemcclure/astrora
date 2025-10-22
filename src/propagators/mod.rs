//! Orbit propagators and numerical integrators
//!
//! This module provides various orbit propagation methods:
//! - Keplerian (two-body) propagator for unperturbed motion
//! - Numerical integrators (RK4, Dormand-Prince) - coming soon
//! - Perturbation models (J2, drag, SRP, third-body) - coming soon

pub mod keplerian;

// Re-export commonly used functions
pub use keplerian::{
    propagate_keplerian,
    propagate_keplerian_duration,
    propagate_state_keplerian,
    propagate_lagrange,
};
