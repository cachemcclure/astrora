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
    m.add_class::<core::time::Epoch>()?;
    m.add_class::<core::time::Duration>()?;
    m.add_class::<core::elements::OrbitalElements>()?;
    m.add_class::<core::elements::EquinoctialElements>()?;

    // Add orbital element conversion functions
    m.add_function(wrap_pyfunction!(py_rv_to_coe, m)?)?;
    m.add_function(wrap_pyfunction!(py_coe_to_rv, m)?)?;
    m.add_function(wrap_pyfunction!(py_coe_to_equinoctial, m)?)?;
    m.add_function(wrap_pyfunction!(py_equinoctial_to_coe, m)?)?;
    m.add_function(wrap_pyfunction!(py_rv_to_equinoctial, m)?)?;
    m.add_function(wrap_pyfunction!(py_equinoctial_to_rv, m)?)?;

    // Add anomaly conversion functions - Elliptical orbits
    m.add_function(wrap_pyfunction!(py_mean_to_eccentric_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_eccentric_to_mean_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_eccentric_to_true_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_true_to_eccentric_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_mean_to_true_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_true_to_mean_anomaly, m)?)?;

    // Add anomaly conversion functions - Hyperbolic orbits
    m.add_function(wrap_pyfunction!(py_mean_to_hyperbolic_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_hyperbolic_to_mean_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_hyperbolic_to_true_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_true_to_hyperbolic_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_mean_to_true_anomaly_hyperbolic, m)?)?;
    m.add_function(wrap_pyfunction!(py_true_to_mean_anomaly_hyperbolic, m)?)?;

    // Add anomaly conversion functions - Parabolic orbits
    m.add_function(wrap_pyfunction!(py_mean_to_true_anomaly_parabolic, m)?)?;
    m.add_function(wrap_pyfunction!(py_true_to_mean_anomaly_parabolic, m)?)?;

    // Add propagator functions
    m.add_function(wrap_pyfunction!(py_propagate_keplerian, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_keplerian_duration, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_state_keplerian, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_lagrange, m)?)?;

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
    m.add("J4_EARTH", J4_EARTH)?;
    m.add("J5_EARTH", J5_EARTH)?;
    m.add("J6_EARTH", J6_EARTH)?;
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

// ============================================================================
// Orbital Element Conversion Functions
// ============================================================================

/// Convert Cartesian state vectors to classical orbital elements
///
/// # Arguments
/// * `r` - Position vector [x, y, z] in meters (NumPy array)
/// * `v` - Velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `mu` - Standard gravitational parameter in m³/s²
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// OrbitalElements object
#[pyfunction]
#[pyo3(name = "rv_to_coe", signature = (r, v, mu, tol=1e-8))]
fn py_rv_to_coe(
    r: PyReadonlyArray1<f64>,
    v: PyReadonlyArray1<f64>,
    mu: f64,
    tol: f64,
) -> PyResult<core::elements::OrbitalElements> {
    // Convert NumPy arrays to Vector3
    let r_array = r.as_array();
    let v_array = v.as_array();

    if r_array.len() != 3 || v_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r_vec = core::linalg::Vector3::new(r_array[0], r_array[1], r_array[2]);
    let v_vec = core::linalg::Vector3::new(v_array[0], v_array[1], v_array[2]);

    // Call the Rust function
    core::elements::rv_to_coe(&r_vec, &v_vec, mu, tol).map_err(|e| e.into())
}

/// Convert classical orbital elements to Cartesian state vectors
///
/// # Arguments
/// * `elements` - OrbitalElements object
/// * `mu` - Standard gravitational parameter in m³/s²
///
/// # Returns
/// Tuple of (position, velocity) as NumPy arrays
#[pyfunction]
#[pyo3(name = "coe_to_rv")]
fn py_coe_to_rv<'py>(
    py: Python<'py>,
    elements: &core::elements::OrbitalElements,
    mu: f64,
) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
    // Call the Rust function
    let (r_vec, v_vec) = core::elements::coe_to_rv(elements, mu);

    // Convert Vector3 to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    (
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    )
}

/// Convert classical orbital elements to modified equinoctial elements
///
/// # Arguments
/// * `elements` - OrbitalElements object
///
/// # Returns
/// EquinoctialElements object
#[pyfunction]
#[pyo3(name = "coe_to_equinoctial")]
fn py_coe_to_equinoctial(
    elements: &core::elements::OrbitalElements,
) -> core::elements::EquinoctialElements {
    core::elements::coe_to_equinoctial(elements)
}

/// Convert modified equinoctial elements to classical orbital elements
///
/// # Arguments
/// * `elements` - EquinoctialElements object
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// OrbitalElements object
#[pyfunction]
#[pyo3(name = "equinoctial_to_coe", signature = (elements, tol=1e-8))]
fn py_equinoctial_to_coe(
    elements: &core::elements::EquinoctialElements,
    tol: f64,
) -> PyResult<core::elements::OrbitalElements> {
    core::elements::equinoctial_to_coe(elements, tol).map_err(|e| e.into())
}

/// Convert Cartesian state vectors to modified equinoctial elements
///
/// # Arguments
/// * `r` - Position vector [x, y, z] in meters (NumPy array)
/// * `v` - Velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `mu` - Standard gravitational parameter in m³/s²
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// EquinoctialElements object
#[pyfunction]
#[pyo3(name = "rv_to_equinoctial", signature = (r, v, mu, tol=1e-8))]
fn py_rv_to_equinoctial(
    r: PyReadonlyArray1<f64>,
    v: PyReadonlyArray1<f64>,
    mu: f64,
    tol: f64,
) -> PyResult<core::elements::EquinoctialElements> {
    // Convert NumPy arrays to Vector3
    let r_array = r.as_array();
    let v_array = v.as_array();

    if r_array.len() != 3 || v_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r_vec = core::linalg::Vector3::new(r_array[0], r_array[1], r_array[2]);
    let v_vec = core::linalg::Vector3::new(v_array[0], v_array[1], v_array[2]);

    // Call the Rust function
    core::elements::rv_to_equinoctial(&r_vec, &v_vec, mu, tol).map_err(|e| e.into())
}

/// Convert modified equinoctial elements to Cartesian state vectors
///
/// # Arguments
/// * `elements` - EquinoctialElements object
/// * `mu` - Standard gravitational parameter in m³/s²
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// Tuple of (position, velocity) as NumPy arrays
#[pyfunction]
#[pyo3(name = "equinoctial_to_rv", signature = (elements, mu, tol=1e-8))]
fn py_equinoctial_to_rv<'py>(
    py: Python<'py>,
    elements: &core::elements::EquinoctialElements,
    mu: f64,
    tol: f64,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    // Call the Rust function
    let (r_vec, v_vec) = core::elements::equinoctial_to_rv(elements, mu, tol)?;

    // Convert Vector3 to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

// ============================================================================
// Anomaly Conversion Functions - Elliptical Orbits
// ============================================================================

/// Convert mean anomaly to eccentric anomaly for elliptical orbits
///
/// # Arguments
/// * `mean_anomaly` - Mean anomaly M (radians)
/// * `eccentricity` - Orbital eccentricity e (must be < 1)
/// * `tol` - Convergence tolerance (optional, default: 1e-12)
/// * `max_iter` - Maximum iterations (optional, default: 50)
///
/// # Returns
/// Eccentric anomaly E (radians)
#[pyfunction]
#[pyo3(name = "mean_to_eccentric_anomaly", signature = (mean_anomaly, eccentricity, tol=None, max_iter=None))]
fn py_mean_to_eccentric_anomaly(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<f64> {
    core::anomaly::mean_to_eccentric_anomaly(mean_anomaly, eccentricity, tol, max_iter)
        .map_err(|e| e.into())
}

/// Convert eccentric anomaly to mean anomaly for elliptical orbits
#[pyfunction]
#[pyo3(name = "eccentric_to_mean_anomaly")]
fn py_eccentric_to_mean_anomaly(eccentric_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::eccentric_to_mean_anomaly(eccentric_anomaly, eccentricity)
        .map_err(|e| e.into())
}

/// Convert eccentric anomaly to true anomaly for elliptical orbits
#[pyfunction]
#[pyo3(name = "eccentric_to_true_anomaly")]
fn py_eccentric_to_true_anomaly(eccentric_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::eccentric_to_true_anomaly(eccentric_anomaly, eccentricity)
        .map_err(|e| e.into())
}

/// Convert true anomaly to eccentric anomaly for elliptical orbits
#[pyfunction]
#[pyo3(name = "true_to_eccentric_anomaly")]
fn py_true_to_eccentric_anomaly(true_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::true_to_eccentric_anomaly(true_anomaly, eccentricity)
        .map_err(|e| e.into())
}

/// Convert mean anomaly to true anomaly for elliptical orbits
#[pyfunction]
#[pyo3(name = "mean_to_true_anomaly", signature = (mean_anomaly, eccentricity, tol=None, max_iter=None))]
fn py_mean_to_true_anomaly(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<f64> {
    core::anomaly::mean_to_true_anomaly(mean_anomaly, eccentricity, tol, max_iter)
        .map_err(|e| e.into())
}

/// Convert true anomaly to mean anomaly for elliptical orbits
#[pyfunction]
#[pyo3(name = "true_to_mean_anomaly")]
fn py_true_to_mean_anomaly(true_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::true_to_mean_anomaly(true_anomaly, eccentricity)
        .map_err(|e| e.into())
}

// ============================================================================
// Anomaly Conversion Functions - Hyperbolic Orbits
// ============================================================================

/// Convert mean anomaly to hyperbolic anomaly for hyperbolic orbits
#[pyfunction]
#[pyo3(name = "mean_to_hyperbolic_anomaly", signature = (mean_anomaly, eccentricity, tol=None, max_iter=None))]
fn py_mean_to_hyperbolic_anomaly(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<f64> {
    core::anomaly::mean_to_hyperbolic_anomaly(mean_anomaly, eccentricity, tol, max_iter)
        .map_err(|e| e.into())
}

/// Convert hyperbolic anomaly to mean anomaly for hyperbolic orbits
#[pyfunction]
#[pyo3(name = "hyperbolic_to_mean_anomaly")]
fn py_hyperbolic_to_mean_anomaly(hyperbolic_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::hyperbolic_to_mean_anomaly(hyperbolic_anomaly, eccentricity)
        .map_err(|e| e.into())
}

/// Convert hyperbolic anomaly to true anomaly for hyperbolic orbits
#[pyfunction]
#[pyo3(name = "hyperbolic_to_true_anomaly")]
fn py_hyperbolic_to_true_anomaly(hyperbolic_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::hyperbolic_to_true_anomaly(hyperbolic_anomaly, eccentricity)
        .map_err(|e| e.into())
}

/// Convert true anomaly to hyperbolic anomaly for hyperbolic orbits
#[pyfunction]
#[pyo3(name = "true_to_hyperbolic_anomaly")]
fn py_true_to_hyperbolic_anomaly(true_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::true_to_hyperbolic_anomaly(true_anomaly, eccentricity)
        .map_err(|e| e.into())
}

/// Convert mean anomaly to true anomaly for hyperbolic orbits
#[pyfunction]
#[pyo3(name = "mean_to_true_anomaly_hyperbolic", signature = (mean_anomaly, eccentricity, tol=None, max_iter=None))]
fn py_mean_to_true_anomaly_hyperbolic(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<f64> {
    core::anomaly::mean_to_true_anomaly_hyperbolic(mean_anomaly, eccentricity, tol, max_iter)
        .map_err(|e| e.into())
}

/// Convert true anomaly to mean anomaly for hyperbolic orbits
#[pyfunction]
#[pyo3(name = "true_to_mean_anomaly_hyperbolic")]
fn py_true_to_mean_anomaly_hyperbolic(true_anomaly: f64, eccentricity: f64) -> PyResult<f64> {
    core::anomaly::true_to_mean_anomaly_hyperbolic(true_anomaly, eccentricity)
        .map_err(|e| e.into())
}

// ============================================================================
// Anomaly Conversion Functions - Parabolic Orbits
// ============================================================================

/// Convert mean anomaly to true anomaly for parabolic orbits
#[pyfunction]
#[pyo3(name = "mean_to_true_anomaly_parabolic")]
fn py_mean_to_true_anomaly_parabolic(mean_anomaly: f64) -> PyResult<f64> {
    core::anomaly::mean_to_true_anomaly_parabolic(mean_anomaly)
        .map_err(|e| e.into())
}

/// Convert true anomaly to mean anomaly for parabolic orbits
#[pyfunction]
#[pyo3(name = "true_to_mean_anomaly_parabolic")]
fn py_true_to_mean_anomaly_parabolic(true_anomaly: f64) -> PyResult<f64> {
    core::anomaly::true_to_mean_anomaly_parabolic(true_anomaly)
        .map_err(|e| e.into())
}

// ============================================================================
// Orbit Propagation Functions
// ============================================================================

/// Propagate orbital elements forward in time using Keplerian (two-body) propagation
///
/// # Arguments
/// * `elements` - Initial orbital elements (OrbitalElements object)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// New OrbitalElements at time t₀ + Δt
///
/// # Note
/// Only elliptical orbits (e < 1) are currently supported.
#[pyfunction]
#[pyo3(name = "propagate_keplerian")]
fn py_propagate_keplerian(
    elements: &core::elements::OrbitalElements,
    dt: f64,
    mu: f64,
) -> PyResult<core::elements::OrbitalElements> {
    propagators::keplerian::propagate_keplerian(elements, dt, mu)
        .map_err(|e| e.into())
}

/// Propagate orbital elements using Duration object
///
/// # Arguments
/// * `elements` - Initial orbital elements
/// * `duration` - Time duration to propagate (Duration object)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// New OrbitalElements at time t₀ + duration
#[pyfunction]
#[pyo3(name = "propagate_keplerian_duration")]
fn py_propagate_keplerian_duration(
    elements: &core::elements::OrbitalElements,
    duration: &core::time::Duration,
    mu: f64,
) -> PyResult<core::elements::OrbitalElements> {
    propagators::keplerian::propagate_keplerian_duration(elements, duration, mu)
        .map_err(|e| e.into())
}

/// Propagate Cartesian state vectors forward in time
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// Tuple of (position, velocity) NumPy arrays at time t₀ + Δt
#[pyfunction]
#[pyo3(name = "propagate_state_keplerian")]
fn py_propagate_state_keplerian<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    // Convert NumPy arrays to Vector3
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    // Call propagator
    let (r_vec, v_vec) = propagators::keplerian::propagate_state_keplerian(&r0_vec, &v0_vec, dt, mu)?;

    // Convert back to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit using Lagrange coefficients (f and g functions)
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// Tuple of (position, velocity) NumPy arrays at time t₀ + Δt
///
/// # Note
/// This is an alternative propagation method that uses Lagrange coefficients
/// directly without converting to orbital elements. Results are identical to
/// propagate_state_keplerian but the algorithm is different.
#[pyfunction]
#[pyo3(name = "propagate_lagrange")]
fn py_propagate_lagrange<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    // Convert NumPy arrays to Vector3
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    // Call propagator
    let (r_vec, v_vec) = propagators::keplerian::propagate_lagrange(&r0_vec, &v0_vec, dt, mu)?;

    // Convert back to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}
