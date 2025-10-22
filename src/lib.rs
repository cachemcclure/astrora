//! Poliastro Core - Rust-backed astrodynamics library
//!
//! This crate provides high-performance orbital mechanics calculations
//! with Python bindings via PyO3.

use pyo3::prelude::*;
use numpy::{PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2, PyReadwriteArray1};

// Module declarations
pub mod core;
pub mod propagators;
pub mod coordinates;
pub mod utils;

// Re-export commonly used types for Rust API
pub use core::{PoliastroError, PoliastroResult};

/// Core astrodynamics module for Python
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Add constants submodule
    let constants = PyModule::new_bound(m.py(), "constants")?;
    add_constants(&constants)?;
    m.add_submodule(&constants)?;

    // Add numpy_ops submodule for array operations
    let numpy_ops = PyModule::new_bound(m.py(), "numpy_ops")?;
    add_numpy_ops(&numpy_ops)?;
    m.add_submodule(&numpy_ops)?;

    // Add core classes
    m.add_class::<core::state::CartesianState>()?;

    // Add submodules as they are implemented
    // m.add_function(wrap_pyfunction!(propagate_orbit, m)?)?;

    Ok(())
}

/// Add all physical and astronomical constants to the constants submodule
fn add_constants(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use crate::core::constants::*;

    // Fundamental physical constants
    m.add("G", G)?;
    m.add("C", C)?;
    m.add("AU", AU)?;

    // Solar constants
    m.add("GM_SUN", GM_SUN)?;
    m.add("R_SUN", R_SUN)?;
    m.add("R_MEAN_SUN", R_MEAN_SUN)?;
    m.add("J2_SUN", J2_SUN)?;
    m.add("SOLAR_IRRADIANCE", SOLAR_IRRADIANCE)?;
    m.add("SOLAR_RADIATION_PRESSURE", SOLAR_RADIATION_PRESSURE)?;

    // Mercury
    m.add("GM_MERCURY", GM_MERCURY)?;
    m.add("R_MERCURY", R_MERCURY)?;
    m.add("R_POLAR_MERCURY", R_POLAR_MERCURY)?;
    m.add("R_MEAN_MERCURY", R_MEAN_MERCURY)?;

    // Venus
    m.add("GM_VENUS", GM_VENUS)?;
    m.add("R_VENUS", R_VENUS)?;
    m.add("R_POLAR_VENUS", R_POLAR_VENUS)?;
    m.add("R_MEAN_VENUS", R_MEAN_VENUS)?;
    m.add("J2_VENUS", J2_VENUS)?;
    m.add("J3_VENUS", J3_VENUS)?;

    // Earth
    m.add("GM_EARTH", GM_EARTH)?;
    m.add("R_EARTH", R_EARTH)?;
    m.add("R_POLAR_EARTH", R_POLAR_EARTH)?;
    m.add("R_MEAN_EARTH", R_MEAN_EARTH)?;
    m.add("J2_EARTH", J2_EARTH)?;
    m.add("J3_EARTH", J3_EARTH)?;
    m.add("H0_EARTH", H0_EARTH)?;
    m.add("RHO0_EARTH", RHO0_EARTH)?;

    // Moon
    m.add("GM_MOON", GM_MOON)?;
    m.add("R_MOON", R_MOON)?;
    m.add("R_POLAR_MOON", R_POLAR_MOON)?;
    m.add("R_MEAN_MOON", R_MEAN_MOON)?;

    // Galilean Moons (Jupiter)
    m.add("GM_IO", GM_IO)?;
    m.add("R_IO", R_IO)?;
    m.add("GM_EUROPA", GM_EUROPA)?;
    m.add("R_EUROPA", R_EUROPA)?;
    m.add("GM_GANYMEDE", GM_GANYMEDE)?;
    m.add("R_GANYMEDE", R_GANYMEDE)?;
    m.add("GM_CALLISTO", GM_CALLISTO)?;
    m.add("R_CALLISTO", R_CALLISTO)?;

    // Saturnian Moons
    m.add("GM_TITAN", GM_TITAN)?;
    m.add("R_TITAN", R_TITAN)?;

    // Neptunian Moons
    m.add("GM_TRITON", GM_TRITON)?;
    m.add("R_TRITON", R_TRITON)?;

    // Mars
    m.add("GM_MARS", GM_MARS)?;
    m.add("R_MARS", R_MARS)?;
    m.add("R_POLAR_MARS", R_POLAR_MARS)?;
    m.add("R_MEAN_MARS", R_MEAN_MARS)?;
    m.add("J2_MARS", J2_MARS)?;
    m.add("J3_MARS", J3_MARS)?;

    // Jupiter
    m.add("GM_JUPITER", GM_JUPITER)?;
    m.add("R_JUPITER", R_JUPITER)?;
    m.add("R_POLAR_JUPITER", R_POLAR_JUPITER)?;
    m.add("R_MEAN_JUPITER", R_MEAN_JUPITER)?;

    // Saturn
    m.add("GM_SATURN", GM_SATURN)?;
    m.add("R_SATURN", R_SATURN)?;
    m.add("R_POLAR_SATURN", R_POLAR_SATURN)?;
    m.add("R_MEAN_SATURN", R_MEAN_SATURN)?;

    // Uranus
    m.add("GM_URANUS", GM_URANUS)?;
    m.add("R_URANUS", R_URANUS)?;
    m.add("R_POLAR_URANUS", R_POLAR_URANUS)?;
    m.add("R_MEAN_URANUS", R_MEAN_URANUS)?;

    // Neptune
    m.add("GM_NEPTUNE", GM_NEPTUNE)?;
    m.add("R_NEPTUNE", R_NEPTUNE)?;
    m.add("R_POLAR_NEPTUNE", R_POLAR_NEPTUNE)?;
    m.add("R_MEAN_NEPTUNE", R_MEAN_NEPTUNE)?;

    // Pluto
    m.add("GM_PLUTO", GM_PLUTO)?;
    m.add("R_PLUTO", R_PLUTO)?;
    m.add("R_POLAR_PLUTO", R_POLAR_PLUTO)?;
    m.add("R_MEAN_PLUTO", R_MEAN_PLUTO)?;

    // Planetary orbital parameters (semi-major axes)
    m.add("A_MERCURY", A_MERCURY)?;
    m.add("A_VENUS", A_VENUS)?;
    m.add("A_EARTH", A_EARTH)?;
    m.add("A_MARS", A_MARS)?;
    m.add("A_JUPITER", A_JUPITER)?;
    m.add("A_SATURN", A_SATURN)?;
    m.add("A_URANUS", A_URANUS)?;
    m.add("A_NEPTUNE", A_NEPTUNE)?;

    // Sphere of influence radii
    m.add("R_SOI_MERCURY", R_SOI_MERCURY)?;
    m.add("R_SOI_VENUS", R_SOI_VENUS)?;
    m.add("R_SOI_EARTH", R_SOI_EARTH)?;
    m.add("R_SOI_MARS", R_SOI_MARS)?;
    m.add("R_SOI_JUPITER", R_SOI_JUPITER)?;
    m.add("R_SOI_SATURN", R_SOI_SATURN)?;
    m.add("R_SOI_URANUS", R_SOI_URANUS)?;
    m.add("R_SOI_NEPTUNE", R_SOI_NEPTUNE)?;

    // Conversion factors
    m.add("KM_TO_M", KM_TO_M)?;
    m.add("M_TO_KM", M_TO_KM)?;
    m.add("DEG_TO_RAD", DEG_TO_RAD)?;
    m.add("RAD_TO_DEG", RAD_TO_DEG)?;
    m.add("DAY_TO_SEC", DAY_TO_SEC)?;
    m.add("SEC_TO_DAY", SEC_TO_DAY)?;

    // Time reference epochs
    m.add("J2000_TT", J2000_TT)?;
    m.add("J2000_MJD", J2000_MJD)?;

    // Helper functions
    m.add_function(wrap_pyfunction!(py_km_to_m, m)?)?;
    m.add_function(wrap_pyfunction!(py_m_to_km, m)?)?;
    m.add_function(wrap_pyfunction!(py_deg_to_rad, m)?)?;
    m.add_function(wrap_pyfunction!(py_rad_to_deg, m)?)?;
    m.add_function(wrap_pyfunction!(py_days_to_sec, m)?)?;
    m.add_function(wrap_pyfunction!(py_sec_to_days, m)?)?;

    Ok(())
}

// Python-facing conversion functions
#[pyfunction]
#[pyo3(name = "km_to_m")]
fn py_km_to_m(km: f64) -> f64 {
    core::constants::km_to_m(km)
}

#[pyfunction]
#[pyo3(name = "m_to_km")]
fn py_m_to_km(m: f64) -> f64 {
    core::constants::m_to_km(m)
}

#[pyfunction]
#[pyo3(name = "deg_to_rad")]
fn py_deg_to_rad(deg: f64) -> f64 {
    core::constants::deg_to_rad(deg)
}

#[pyfunction]
#[pyo3(name = "rad_to_deg")]
fn py_rad_to_deg(rad: f64) -> f64 {
    core::constants::rad_to_deg(rad)
}

#[pyfunction]
#[pyo3(name = "days_to_sec")]
fn py_days_to_sec(days: f64) -> f64 {
    core::constants::days_to_sec(days)
}

#[pyfunction]
#[pyo3(name = "sec_to_days")]
fn py_sec_to_days(sec: f64) -> f64 {
    core::constants::sec_to_days(sec)
}

/// Add NumPy array operation functions to the numpy_ops submodule
fn add_numpy_ops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 1D array operations
    m.add_function(wrap_pyfunction!(py_sum_array, m)?)?;
    m.add_function(wrap_pyfunction!(py_multiply_scalar, m)?)?;
    m.add_function(wrap_pyfunction!(py_multiply_scalar_inplace, m)?)?;
    m.add_function(wrap_pyfunction!(py_dot_product, m)?)?;
    m.add_function(wrap_pyfunction!(py_cross_product, m)?)?;
    m.add_function(wrap_pyfunction!(py_add_arrays, m)?)?;
    m.add_function(wrap_pyfunction!(py_normalize_vector, m)?)?;
    m.add_function(wrap_pyfunction!(py_vector_magnitude, m)?)?;
    m.add_function(wrap_pyfunction!(py_apply_polynomial, m)?)?;

    // 2D array operations
    m.add_function(wrap_pyfunction!(py_matrix_vector_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(py_matrix_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(py_transpose_matrix, m)?)?;
    m.add_function(wrap_pyfunction!(py_identity_matrix, m)?)?;

    // Batch operations
    m.add_function(wrap_pyfunction!(py_batch_normalize_vectors, m)?)?;

    Ok(())
}

// Python-facing NumPy operation functions

#[pyfunction]
#[pyo3(name = "sum_array")]
fn py_sum_array(arr: PyReadonlyArray1<f64>) -> f64 {
    core::numpy_integration::sum_array(arr.as_array())
}

#[pyfunction]
#[pyo3(name = "multiply_scalar")]
fn py_multiply_scalar<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray1<f64>,
    scalar: f64,
) -> Bound<'py, PyArray1<f64>> {
    let result = core::numpy_integration::multiply_scalar(arr.as_array(), scalar);
    PyArray1::from_owned_array_bound(py, result)
}

#[pyfunction]
#[pyo3(name = "multiply_scalar_inplace")]
fn py_multiply_scalar_inplace(mut arr: PyReadwriteArray1<f64>, scalar: f64) {
    core::numpy_integration::multiply_scalar_inplace(arr.as_array_mut(), scalar);
}

#[pyfunction]
#[pyo3(name = "dot_product")]
fn py_dot_product(a: PyReadonlyArray1<f64>, b: PyReadonlyArray1<f64>) -> PyResult<f64> {
    core::numpy_integration::dot_product(a.as_array(), b.as_array())
        .map_err(|e| e.into())
}

#[pyfunction]
#[pyo3(name = "cross_product")]
fn py_cross_product<'py>(
    py: Python<'py>,
    a: PyReadonlyArray1<f64>,
    b: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let result = core::numpy_integration::cross_product(a.as_array(), b.as_array())?;
    Ok(PyArray1::from_owned_array_bound(py, result))
}

#[pyfunction]
#[pyo3(name = "add_arrays")]
fn py_add_arrays<'py>(
    py: Python<'py>,
    a: PyReadonlyArray1<f64>,
    b: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let result = core::numpy_integration::add_arrays(a.as_array(), b.as_array())?;
    Ok(PyArray1::from_owned_array_bound(py, result))
}

#[pyfunction]
#[pyo3(name = "normalize_vector")]
fn py_normalize_vector<'py>(
    py: Python<'py>,
    vec: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let result = core::numpy_integration::normalize_vector(vec.as_array())?;
    Ok(PyArray1::from_owned_array_bound(py, result))
}

#[pyfunction]
#[pyo3(name = "vector_magnitude")]
fn py_vector_magnitude(vec: PyReadonlyArray1<f64>) -> f64 {
    core::numpy_integration::vector_magnitude(vec.as_array())
}

#[pyfunction]
#[pyo3(name = "apply_polynomial")]
fn py_apply_polynomial<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray1<f64>,
) -> Bound<'py, PyArray1<f64>> {
    let result = core::numpy_integration::apply_polynomial(arr.as_array());
    PyArray1::from_owned_array_bound(py, result)
}

#[pyfunction]
#[pyo3(name = "matrix_vector_multiply")]
fn py_matrix_vector_multiply<'py>(
    py: Python<'py>,
    matrix: PyReadonlyArray2<f64>,
    vector: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let result = core::numpy_integration::matrix_vector_multiply(
        matrix.as_array(),
        vector.as_array(),
    )?;
    Ok(PyArray1::from_owned_array_bound(py, result))
}

#[pyfunction]
#[pyo3(name = "matrix_multiply")]
fn py_matrix_multiply<'py>(
    py: Python<'py>,
    a: PyReadonlyArray2<f64>,
    b: PyReadonlyArray2<f64>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let result = core::numpy_integration::matrix_multiply(a.as_array(), b.as_array())?;
    Ok(PyArray2::from_owned_array_bound(py, result))
}

#[pyfunction]
#[pyo3(name = "transpose_matrix")]
fn py_transpose_matrix<'py>(
    py: Python<'py>,
    matrix: PyReadonlyArray2<f64>,
) -> Bound<'py, PyArray2<f64>> {
    let result = core::numpy_integration::transpose_matrix(matrix.as_array());
    PyArray2::from_owned_array_bound(py, result)
}

#[pyfunction]
#[pyo3(name = "identity_matrix")]
fn py_identity_matrix(py: Python<'_>, size: usize) -> PyResult<Bound<'_, PyArray2<f64>>> {
    let result = core::numpy_integration::identity_matrix(size)?;
    Ok(PyArray2::from_owned_array_bound(py, result))
}

#[pyfunction]
#[pyo3(name = "batch_normalize_vectors")]
fn py_batch_normalize_vectors<'py>(
    py: Python<'py>,
    vectors: PyReadonlyArray2<f64>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    let result = core::numpy_integration::batch_normalize_vectors(vectors.as_array())?;
    Ok(PyArray2::from_owned_array_bound(py, result))
}
