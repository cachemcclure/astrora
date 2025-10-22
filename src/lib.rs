//! Poliastro Core - Rust-backed astrodynamics library
//!
//! This crate provides high-performance orbital mechanics calculations
//! with Python bindings via PyO3.

use pyo3::prelude::*;

// Module declarations
pub mod core;
pub mod propagators;
pub mod coordinates;
pub mod utils;

/// Core astrodynamics module for Python
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Add submodules as they are implemented
    // m.add_class::<Orbit>()?;
    // m.add_function(wrap_pyfunction!(propagate_orbit, m)?)?;

    Ok(())
}
