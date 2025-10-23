//! Orbit propagators and numerical integrators
//!
//! This module provides various orbit propagation methods:
//! - Keplerian (two-body) propagator for unperturbed motion
//! - Perturbation models (J2, drag, SRP, third-body)
//! - High-performance static perturbations (zero-allocation)
//! - State transition matrix (STM) propagation for orbit determination
//! - Numerical integrators (RK4, Dormand-Prince available in core::numerical)

pub mod keplerian;
pub mod perturbations;
pub mod perturbations_static; // High-performance zero-allocation perturbations
pub mod stm;

// Re-export commonly used functions
pub use keplerian::{
    propagate_keplerian,
    propagate_keplerian_duration,
    propagate_state_keplerian,
    propagate_lagrange,
};

pub use perturbations::{
    j2_perturbation,
    propagate_j2_rk4,
    propagate_j2_dopri5,
};

pub use stm::{
    jacobian_two_body,
    jacobian_j2,
    propagate_stm_rk4,
    propagate_stm_dopri5,
    propagate_stm_j2_rk4,
};

pub use perturbations_static::{
    j2_perturbation_static,
    j3_perturbation_static,
    j4_perturbation_static,
    j2_j3_j4_perturbation_static,
    j2_dynamics,
    j2_j3_j4_dynamics,
};
