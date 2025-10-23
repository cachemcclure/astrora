//! Astrora Core - Rust-backed astrodynamics library
//!
//! This crate provides high-performance orbital mechanics calculations
//! with Python bindings via PyO3.
//!
//! Formerly known as poliastro (archived 2023), astrora is a modern
//! reimplementation with significant performance improvements.

use pyo3::prelude::*;
use numpy::{PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2, PyReadwriteArray1};

// Module declarations
pub mod core;
pub mod propagators;
pub mod coordinates;
pub mod maneuvers;
pub mod satellite;
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

    // Add coordinate frame classes
    m.add_class::<coordinates::frames::ICRS>()?;
    m.add_class::<coordinates::frames::GCRS>()?;
    m.add_class::<coordinates::frames::J2000>()?;
    m.add_class::<coordinates::frames::ITRS>()?;
    m.add_class::<coordinates::frames::TEME>()?;
    m.add_class::<coordinates::frames::Perifocal>()?;

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

    // Add batch anomaly conversion functions - Elliptical orbits
    m.add_function(wrap_pyfunction!(py_batch_mean_to_eccentric_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_mean_to_true_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_true_to_mean_anomaly, m)?)?;

    // Add batch anomaly conversion functions - Hyperbolic orbits
    m.add_function(wrap_pyfunction!(py_batch_mean_to_hyperbolic_anomaly, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_mean_to_true_anomaly_hyperbolic, m)?)?;

    // Add batch anomaly conversion functions - Parabolic orbits
    m.add_function(wrap_pyfunction!(py_batch_mean_to_true_anomaly_parabolic, m)?)?;

    // Add propagator functions
    m.add_function(wrap_pyfunction!(py_propagate_keplerian, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_keplerian_duration, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_state_keplerian, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_lagrange, m)?)?;

    // Add batch propagator functions
    m.add_function(wrap_pyfunction!(py_batch_propagate_states, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_propagate_lagrange, m)?)?;

    // Add J2 perturbation functions
    m.add_function(wrap_pyfunction!(py_j2_perturbation, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_j2_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_j2_dopri5, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_j2_dop853, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_stm_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_stm_dopri5, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_stm_j2_rk4, m)?)?;

    // High-performance static J2 perturbation functions (zero-allocation, 3-5x faster)
    m.add_function(wrap_pyfunction!(py_propagate_j2_rk4_static, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_j2_j3_j4_rk4_static, m)?)?;

    // Atmospheric drag functions
    m.add_function(wrap_pyfunction!(py_exponential_density, m)?)?;
    m.add_function(wrap_pyfunction!(py_drag_acceleration, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_drag_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_drag_dopri5, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_j2_drag_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_j2_drag_dopri5, m)?)?;

    // Third-body perturbations
    m.add_function(wrap_pyfunction!(py_sun_position_simple, m)?)?;
    m.add_function(wrap_pyfunction!(py_moon_position_simple, m)?)?;
    m.add_function(wrap_pyfunction!(py_third_body_perturbation, m)?)?;
    m.add_function(wrap_pyfunction!(py_sun_moon_perturbation, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_thirdbody_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_thirdbody_dopri5, m)?)?;

    // Solar radiation pressure
    m.add_function(wrap_pyfunction!(py_shadow_function, m)?)?;
    m.add_function(wrap_pyfunction!(py_srp_acceleration, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_srp_rk4, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_srp_dopri5, m)?)?;

    // Batch coordinate transformations (parallelized with rayon)
    m.add_function(wrap_pyfunction!(py_batch_gcrs_to_itrs, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_itrs_to_gcrs, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_gcrs_to_teme, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_teme_to_gcrs, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_teme_to_itrs, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_itrs_to_teme, m)?)?;

    // Maneuver calculations
    m.add_function(wrap_pyfunction!(py_hohmann_transfer, m)?)?;
    m.add_function(wrap_pyfunction!(py_hohmann_phase_angle, m)?)?;
    m.add_function(wrap_pyfunction!(py_hohmann_synodic_period, m)?)?;
    m.add_function(wrap_pyfunction!(py_hohmann_time_to_window, m)?)?;
    m.add_function(wrap_pyfunction!(py_bielliptic_transfer, m)?)?;
    m.add_function(wrap_pyfunction!(py_compare_bielliptic_hohmann, m)?)?;
    m.add_function(wrap_pyfunction!(py_find_optimal_bielliptic, m)?)?;
    m.add_function(wrap_pyfunction!(py_pure_plane_change, m)?)?;
    m.add_function(wrap_pyfunction!(py_combined_plane_change, m)?)?;
    m.add_function(wrap_pyfunction!(py_optimal_plane_change_location, m)?)?;
    m.add_function(wrap_pyfunction!(py_phasing_orbit, m)?)?;
    m.add_function(wrap_pyfunction!(py_coorbital_rendezvous, m)?)?;
    m.add_function(wrap_pyfunction!(py_coplanar_rendezvous, m)?)?;
    m.add_function(wrap_pyfunction!(py_delta_v_budget, m)?)?;
    m.add_function(wrap_pyfunction!(py_gravity_assist, m)?)?;
    m.add_function(wrap_pyfunction!(py_periapsis_from_b_parameter, m)?)?;

    // Lambert's problem solver functions
    m.add_function(wrap_pyfunction!(py_lambert_solve, m)?)?;
    m.add_function(wrap_pyfunction!(py_lambert_solve_batch, m)?)?;
    m.add_function(wrap_pyfunction!(py_lambert_solve_batch_parallel, m)?)?;

    // Satellite operations (SGP4/SDP4 TLE propagation)
    m.add_function(wrap_pyfunction!(py_propagate_tle, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_tle_batch, m)?)?;
    m.add_function(wrap_pyfunction!(py_propagate_omm, m)?)?;

    // Satellite visibility and ground station operations
    m.add_function(wrap_pyfunction!(py_compute_azimuth_elevation, m)?)?;
    m.add_function(wrap_pyfunction!(py_compute_azimuth_elevation_rate, m)?)?;
    m.add_function(wrap_pyfunction!(py_is_visible, m)?)?;
    m.add_function(wrap_pyfunction!(py_find_satellite_passes, m)?)?;

    // Ground track and sub-satellite point functions
    m.add_function(wrap_pyfunction!(py_ecef_to_geodetic, m)?)?;
    m.add_function(wrap_pyfunction!(py_compute_ground_track, m)?)?;
    m.add_function(wrap_pyfunction!(py_maximum_ground_range, m)?)?;
    m.add_function(wrap_pyfunction!(py_calculate_swath_width, m)?)?;

    // Satellite footprint and coverage analysis functions
    m.add_function(wrap_pyfunction!(py_visibility_circle, m)?)?;
    m.add_function(wrap_pyfunction!(py_coverage_area, m)?)?;
    m.add_function(wrap_pyfunction!(py_coverage_percentage, m)?)?;

    // Eclipse detection and solar lighting conditions
    m.add_function(wrap_pyfunction!(py_compute_eclipse_state, m)?)?;
    m.add_function(wrap_pyfunction!(py_solar_beta_angle, m)?)?;
    m.add_function(wrap_pyfunction!(py_solar_beta_angle_precise, m)?)?;
    m.add_function(wrap_pyfunction!(py_sun_synchronous_inclination, m)?)?;
    m.add_function(wrap_pyfunction!(py_eclipse_duration, m)?)?;

    // Satellite lifetime estimation and decay
    m.add_function(wrap_pyfunction!(py_estimate_satellite_lifetime, m)?)?;
    m.add_function(wrap_pyfunction!(py_estimate_decay_rate, m)?)?;

    // Conjunction analysis and collision detection
    m.add_function(wrap_pyfunction!(py_compute_conjunction, m)?)?;
    m.add_function(wrap_pyfunction!(py_closest_approach_distance, m)?)?;

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

/// Convert kilometers to meters
///
/// # Arguments
/// * `km` - Distance in kilometers
///
/// # Returns
/// Distance in meters
///
/// # Example
/// ```python
/// from astrora import km_to_m
/// meters = km_to_m(1.0)  # Returns 1000.0
/// ```
#[pyfunction]
#[pyo3(name = "km_to_m")]
fn py_km_to_m(km: f64) -> f64 {
    core::constants::km_to_m(km)
}

/// Convert meters to kilometers
///
/// # Arguments
/// * `m` - Distance in meters
///
/// # Returns
/// Distance in kilometers
///
/// # Example
/// ```python
/// from astrora import m_to_km
/// km = m_to_km(1000.0)  # Returns 1.0
/// ```
#[pyfunction]
#[pyo3(name = "m_to_km")]
fn py_m_to_km(m: f64) -> f64 {
    core::constants::m_to_km(m)
}

/// Convert degrees to radians
///
/// # Arguments
/// * `deg` - Angle in degrees
///
/// # Returns
/// Angle in radians
///
/// # Example
/// ```python
/// from astrora import deg_to_rad
/// import math
/// rad = deg_to_rad(180.0)  # Returns π (3.14159...)
/// ```
#[pyfunction]
#[pyo3(name = "deg_to_rad")]
fn py_deg_to_rad(deg: f64) -> f64 {
    core::constants::deg_to_rad(deg)
}

/// Convert radians to degrees
///
/// # Arguments
/// * `rad` - Angle in radians
///
/// # Returns
/// Angle in degrees
///
/// # Example
/// ```python
/// from astrora import rad_to_deg
/// import math
/// deg = rad_to_deg(math.pi)  # Returns 180.0
/// ```
#[pyfunction]
#[pyo3(name = "rad_to_deg")]
fn py_rad_to_deg(rad: f64) -> f64 {
    core::constants::rad_to_deg(rad)
}

/// Convert days to seconds
///
/// # Arguments
/// * `days` - Time duration in days
///
/// # Returns
/// Time duration in seconds
///
/// # Example
/// ```python
/// from astrora import days_to_sec
/// seconds = days_to_sec(1.0)  # Returns 86400.0
/// ```
#[pyfunction]
#[pyo3(name = "days_to_sec")]
fn py_days_to_sec(days: f64) -> f64 {
    core::constants::days_to_sec(days)
}

/// Convert seconds to days
///
/// # Arguments
/// * `sec` - Time duration in seconds
///
/// # Returns
/// Time duration in days
///
/// # Example
/// ```python
/// from astrora import sec_to_days
/// days = sec_to_days(86400.0)  # Returns 1.0
/// ```
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

/// Sum all elements in a NumPy array
///
/// # Arguments
/// * `arr` - 1D NumPy array of floats
///
/// # Returns
/// Sum of all array elements
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import sum_array
/// arr = np.array([1.0, 2.0, 3.0])
/// total = sum_array(arr)  # Returns 6.0
/// ```
#[pyfunction]
#[pyo3(name = "sum_array")]
fn py_sum_array(arr: PyReadonlyArray1<f64>) -> f64 {
    core::numpy_integration::sum_array(arr.as_array())
}

/// Multiply each element of a NumPy array by a scalar value
///
/// # Arguments
/// * `arr` - 1D NumPy array of floats
/// * `scalar` - Scalar value to multiply each element by
///
/// # Returns
/// New NumPy array with each element multiplied by the scalar
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import multiply_scalar
/// arr = np.array([1.0, 2.0, 3.0])
/// result = multiply_scalar(arr, 2.0)  # Returns [2.0, 4.0, 6.0]
/// ```
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

/// Multiply each element of a NumPy array by a scalar value in-place
///
/// Modifies the input array directly without allocating new memory.
///
/// # Arguments
/// * `arr` - 1D NumPy array of floats (modified in-place)
/// * `scalar` - Scalar value to multiply each element by
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import multiply_scalar_inplace
/// arr = np.array([1.0, 2.0, 3.0])
/// multiply_scalar_inplace(arr, 2.0)  # arr is now [2.0, 4.0, 6.0]
/// ```
#[pyfunction]
#[pyo3(name = "multiply_scalar_inplace")]
fn py_multiply_scalar_inplace(mut arr: PyReadwriteArray1<f64>, scalar: f64) {
    core::numpy_integration::multiply_scalar_inplace(arr.as_array_mut(), scalar);
}

/// Compute the dot product of two vectors
///
/// # Arguments
/// * `a` - First 1D NumPy array
/// * `b` - Second 1D NumPy array
///
/// # Returns
/// Dot product of the two vectors
///
/// # Errors
/// Returns an error if the arrays have different lengths
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import dot_product
/// a = np.array([1.0, 2.0, 3.0])
/// b = np.array([4.0, 5.0, 6.0])
/// result = dot_product(a, b)  # Returns 32.0 (1*4 + 2*5 + 3*6)
/// ```
#[pyfunction]
#[pyo3(name = "dot_product")]
fn py_dot_product(a: PyReadonlyArray1<f64>, b: PyReadonlyArray1<f64>) -> PyResult<f64> {
    core::numpy_integration::dot_product(a.as_array(), b.as_array())
        .map_err(|e| e.into())
}

/// Compute the cross product of two 3D vectors
///
/// # Arguments
/// * `a` - First 3D vector as NumPy array [x, y, z]
/// * `b` - Second 3D vector as NumPy array [x, y, z]
///
/// # Returns
/// Cross product vector a × b
///
/// # Errors
/// Returns an error if either array is not exactly 3 elements
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import cross_product
/// a = np.array([1.0, 0.0, 0.0])
/// b = np.array([0.0, 1.0, 0.0])
/// result = cross_product(a, b)  # Returns [0.0, 0.0, 1.0]
/// ```
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

/// Add two NumPy arrays element-wise
///
/// # Arguments
/// * `a` - First 1D NumPy array
/// * `b` - Second 1D NumPy array
///
/// # Returns
/// New array containing element-wise sum a + b
///
/// # Errors
/// Returns an error if the arrays have different lengths
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import add_arrays
/// a = np.array([1.0, 2.0, 3.0])
/// b = np.array([4.0, 5.0, 6.0])
/// result = add_arrays(a, b)  # Returns [5.0, 7.0, 9.0]
/// ```
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

/// Normalize a vector to unit length
///
/// # Arguments
/// * `vec` - Input vector as 1D NumPy array
///
/// # Returns
/// Unit vector in the same direction as the input
///
/// # Errors
/// Returns an error if the vector has zero magnitude
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import normalize_vector
/// vec = np.array([3.0, 4.0, 0.0])
/// unit_vec = normalize_vector(vec)  # Returns [0.6, 0.8, 0.0]
/// ```
#[pyfunction]
#[pyo3(name = "normalize_vector")]
fn py_normalize_vector<'py>(
    py: Python<'py>,
    vec: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let result = core::numpy_integration::normalize_vector(vec.as_array())?;
    Ok(PyArray1::from_owned_array_bound(py, result))
}

/// Compute the magnitude (L2 norm) of a vector
///
/// # Arguments
/// * `vec` - Input vector as 1D NumPy array
///
/// # Returns
/// Euclidean magnitude of the vector
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import vector_magnitude
/// vec = np.array([3.0, 4.0, 0.0])
/// mag = vector_magnitude(vec)  # Returns 5.0
/// ```
#[pyfunction]
#[pyo3(name = "vector_magnitude")]
fn py_vector_magnitude(vec: PyReadonlyArray1<f64>) -> f64 {
    core::numpy_integration::vector_magnitude(vec.as_array())
}

/// Apply a simple polynomial transformation to array elements
///
/// This is a placeholder function for polynomial operations.
///
/// # Arguments
/// * `arr` - Input 1D NumPy array
///
/// # Returns
/// Transformed NumPy array
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import apply_polynomial
/// arr = np.array([1.0, 2.0, 3.0])
/// result = apply_polynomial(arr)
/// ```
#[pyfunction]
#[pyo3(name = "apply_polynomial")]
fn py_apply_polynomial<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray1<f64>,
) -> Bound<'py, PyArray1<f64>> {
    let result = core::numpy_integration::apply_polynomial(arr.as_array());
    PyArray1::from_owned_array_bound(py, result)
}

/// Multiply a matrix by a vector
///
/// # Arguments
/// * `matrix` - 2D NumPy array (m × n matrix)
/// * `vector` - 1D NumPy array (n-element vector)
///
/// # Returns
/// Result vector as 1D NumPy array (m elements)
///
/// # Errors
/// Returns an error if matrix columns don't match vector length
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import matrix_vector_multiply
/// matrix = np.array([[1.0, 2.0], [3.0, 4.0]])
/// vector = np.array([5.0, 6.0])
/// result = matrix_vector_multiply(matrix, vector)  # Returns [17.0, 39.0]
/// ```
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

/// Multiply two matrices
///
/// # Arguments
/// * `a` - First 2D NumPy array (m × n matrix)
/// * `b` - Second 2D NumPy array (n × p matrix)
///
/// # Returns
/// Product matrix as 2D NumPy array (m × p)
///
/// # Errors
/// Returns an error if matrix dimensions are incompatible (columns of a ≠ rows of b)
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import matrix_multiply
/// a = np.array([[1.0, 2.0], [3.0, 4.0]])
/// b = np.array([[5.0, 6.0], [7.0, 8.0]])
/// result = matrix_multiply(a, b)  # Returns [[19.0, 22.0], [43.0, 50.0]]
/// ```
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

/// Transpose a matrix
///
/// # Arguments
/// * `matrix` - 2D NumPy array (m × n matrix)
///
/// # Returns
/// Transposed matrix as 2D NumPy array (n × m)
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import transpose_matrix
/// matrix = np.array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])
/// result = transpose_matrix(matrix)  # Returns [[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]
/// ```
#[pyfunction]
#[pyo3(name = "transpose_matrix")]
fn py_transpose_matrix<'py>(
    py: Python<'py>,
    matrix: PyReadonlyArray2<f64>,
) -> Bound<'py, PyArray2<f64>> {
    let result = core::numpy_integration::transpose_matrix(matrix.as_array());
    PyArray2::from_owned_array_bound(py, result)
}

/// Create an identity matrix
///
/// # Arguments
/// * `size` - Dimension of the square identity matrix
///
/// # Returns
/// Identity matrix as 2D NumPy array (size × size)
///
/// # Errors
/// Returns an error if size is zero
///
/// # Example
/// ```python
/// from astrora import identity_matrix
/// I = identity_matrix(3)  # Returns [[1,0,0], [0,1,0], [0,0,1]]
/// ```
#[pyfunction]
#[pyo3(name = "identity_matrix")]
fn py_identity_matrix(py: Python<'_>, size: usize) -> PyResult<Bound<'_, PyArray2<f64>>> {
    let result = core::numpy_integration::identity_matrix(size)?;
    Ok(PyArray2::from_owned_array_bound(py, result))
}

/// Normalize multiple vectors to unit length in batch
///
/// Processes multiple vectors simultaneously, normalizing each row to unit length.
/// Significantly faster than normalizing vectors individually due to Rust vectorization.
///
/// # Arguments
/// * `vectors` - 2D NumPy array where each row is a vector to normalize
///
/// # Returns
/// 2D NumPy array with each row normalized to unit length
///
/// # Errors
/// Returns an error if any vector has zero magnitude
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import batch_normalize_vectors
/// vectors = np.array([
///     [3.0, 4.0, 0.0],
///     [1.0, 0.0, 0.0],
///     [0.0, 5.0, 12.0]
/// ])
/// result = batch_normalize_vectors(vectors)
/// # Returns:
/// # [[0.6, 0.8, 0.0],
/// #  [1.0, 0.0, 0.0],
/// #  [0.0, 0.385, 0.923]]
/// ```
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
// Batch Anomaly Conversion Functions - Elliptical Orbits
// ============================================================================

/// Convert mean anomalies to eccentric anomalies for multiple elliptical orbits (batch)
///
/// This function processes arrays efficiently, providing 10-20x performance improvement
/// over sequential processing by minimizing Python-Rust boundary crossings.
///
/// # Arguments
/// * `mean_anomalies` - NumPy array of mean anomalies M (radians)
/// * `eccentricities` - NumPy array of eccentricities e (can be single value or array)
/// * `tol` - Convergence tolerance (optional, default: 1e-12)
/// * `max_iter` - Maximum iterations (optional, default: 50)
///
/// # Returns
/// NumPy array of eccentric anomalies E (radians)
#[pyfunction]
#[pyo3(name = "batch_mean_to_eccentric_anomaly", signature = (mean_anomalies, eccentricities, tol=None, max_iter=None))]
fn py_batch_mean_to_eccentric_anomaly<'py>(
    py: Python<'py>,
    mean_anomalies: PyReadonlyArray1<f64>,
    eccentricities: PyReadonlyArray1<f64>,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let m_array = mean_anomalies.as_slice()?;
    let e_array = eccentricities.as_slice()?;

    let results = core::anomaly::batch_mean_to_eccentric_anomaly(m_array, e_array, tol, max_iter)
        .map_err(|e| Into::<PyErr>::into(e))?;

    Ok(PyArray1::from_vec_bound(py, results))
}

/// Convert mean anomalies to true anomalies for multiple elliptical orbits (batch)
#[pyfunction]
#[pyo3(name = "batch_mean_to_true_anomaly", signature = (mean_anomalies, eccentricities, tol=None, max_iter=None))]
fn py_batch_mean_to_true_anomaly<'py>(
    py: Python<'py>,
    mean_anomalies: PyReadonlyArray1<f64>,
    eccentricities: PyReadonlyArray1<f64>,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let m_array = mean_anomalies.as_slice()?;
    let e_array = eccentricities.as_slice()?;

    let results = core::anomaly::batch_mean_to_true_anomaly(m_array, e_array, tol, max_iter)
        .map_err(|e| Into::<PyErr>::into(e))?;

    Ok(PyArray1::from_vec_bound(py, results))
}

/// Convert true anomalies to mean anomalies for multiple elliptical orbits (batch)
#[pyfunction]
#[pyo3(name = "batch_true_to_mean_anomaly")]
fn py_batch_true_to_mean_anomaly<'py>(
    py: Python<'py>,
    true_anomalies: PyReadonlyArray1<f64>,
    eccentricities: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let nu_array = true_anomalies.as_slice()?;
    let e_array = eccentricities.as_slice()?;

    let results = core::anomaly::batch_true_to_mean_anomaly(nu_array, e_array)
        .map_err(|e| Into::<PyErr>::into(e))?;

    Ok(PyArray1::from_vec_bound(py, results))
}

// ============================================================================
// Batch Anomaly Conversion Functions - Hyperbolic Orbits
// ============================================================================

/// Convert mean anomalies to hyperbolic anomalies for multiple hyperbolic orbits (batch)
#[pyfunction]
#[pyo3(name = "batch_mean_to_hyperbolic_anomaly", signature = (mean_anomalies, eccentricities, tol=None, max_iter=None))]
fn py_batch_mean_to_hyperbolic_anomaly<'py>(
    py: Python<'py>,
    mean_anomalies: PyReadonlyArray1<f64>,
    eccentricities: PyReadonlyArray1<f64>,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let m_array = mean_anomalies.as_slice()?;
    let e_array = eccentricities.as_slice()?;

    let results = core::anomaly::batch_mean_to_hyperbolic_anomaly(m_array, e_array, tol, max_iter)
        .map_err(|e| Into::<PyErr>::into(e))?;

    Ok(PyArray1::from_vec_bound(py, results))
}

/// Convert mean anomalies to true anomalies for multiple hyperbolic orbits (batch)
#[pyfunction]
#[pyo3(name = "batch_mean_to_true_anomaly_hyperbolic", signature = (mean_anomalies, eccentricities, tol=None, max_iter=None))]
fn py_batch_mean_to_true_anomaly_hyperbolic<'py>(
    py: Python<'py>,
    mean_anomalies: PyReadonlyArray1<f64>,
    eccentricities: PyReadonlyArray1<f64>,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let m_array = mean_anomalies.as_slice()?;
    let e_array = eccentricities.as_slice()?;

    let results = core::anomaly::batch_mean_to_true_anomaly_hyperbolic(m_array, e_array, tol, max_iter)
        .map_err(|e| Into::<PyErr>::into(e))?;

    Ok(PyArray1::from_vec_bound(py, results))
}

// ============================================================================
// Batch Anomaly Conversion Functions - Parabolic Orbits
// ============================================================================

/// Convert mean anomalies to true anomalies for multiple parabolic orbits (batch)
#[pyfunction]
#[pyo3(name = "batch_mean_to_true_anomaly_parabolic")]
fn py_batch_mean_to_true_anomaly_parabolic<'py>(
    py: Python<'py>,
    mean_anomalies: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let m_array = mean_anomalies.as_slice()?;

    let results = core::anomaly::batch_mean_to_true_anomaly_parabolic(m_array)
        .map_err(|e| Into::<PyErr>::into(e))?;

    Ok(PyArray1::from_vec_bound(py, results))
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

/// Batch propagation of multiple state vectors
///
/// Efficiently propagates multiple orbits in a single call, minimizing Python-Rust
/// boundary crossing overhead. This provides 10-20x speedup over sequential calls.
///
/// # Arguments
/// * `states` - 2D NumPy array where each row is [x, y, z, vx, vy, vz] (meters and m/s)
/// * `time_steps` - Either a single time step (float) or array of time steps (seconds)
///   - If single value: propagate all states by the same time step
///   - If array: must have same length as number of states (one per state)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// 2D NumPy array with same shape as input, containing propagated states
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import batch_propagate_states, constants
///
/// # Two orbits at different altitudes
/// states = np.array([
///     [7000e3, 0.0, 0.0, 0.0, 7546.0, 0.0],  # Orbit 1
///     [8000e3, 0.0, 0.0, 0.0, 7000.0, 0.0]   # Orbit 2
/// ])
///
/// # Propagate both for 1 hour
/// result = batch_propagate_states(states, 3600.0, constants.GM_EARTH)
///
/// # Or with different time steps
/// dt = np.array([1800.0, 3600.0])  # 30 min and 1 hour
/// result = batch_propagate_states(states, dt, constants.GM_EARTH)
/// ```
#[pyfunction]
#[pyo3(name = "batch_propagate_states")]
fn py_batch_propagate_states<'py>(
    py: Python<'py>,
    states: PyReadonlyArray2<f64>,
    time_steps: &Bound<'_, pyo3::types::PyAny>,
    mu: f64,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    // Convert states array
    let states_array = states.as_array();

    // Handle time_steps - can be either a single float or an array
    let dt_vec: Vec<f64> = if let Ok(single_dt) = time_steps.extract::<f64>() {
        // Single time step for all states
        vec![single_dt]
    } else if let Ok(dt_array) = time_steps.extract::<PyReadonlyArray1<f64>>() {
        // Array of time steps
        dt_array.as_array().to_vec()
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "time_steps must be either a float or a 1D NumPy array of floats"
        ));
    };

    // Call batch propagator
    let result = propagators::keplerian::batch_propagate_states(
        states_array,
        &dt_vec,
        mu
    )?;

    // Return as NumPy array
    Ok(PyArray2::from_owned_array_bound(py, result))
}

/// Batch propagation using Lagrange coefficients
///
/// Alternative batch propagation method using f and g functions.
/// Useful for cross-validation and performance comparison.
///
/// # Arguments
/// * `states` - 2D NumPy array where each row is [x, y, z, vx, vy, vz]
/// * `time_steps` - Either a single time step or array of time steps (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// 2D NumPy array with same shape as input, containing propagated states
#[pyfunction]
#[pyo3(name = "batch_propagate_lagrange")]
fn py_batch_propagate_lagrange<'py>(
    py: Python<'py>,
    states: PyReadonlyArray2<f64>,
    time_steps: &Bound<'_, pyo3::types::PyAny>,
    mu: f64,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    // Convert states array
    let states_array = states.as_array();

    // Handle time_steps - can be either a single float or an array
    let dt_vec: Vec<f64> = if let Ok(single_dt) = time_steps.extract::<f64>() {
        vec![single_dt]
    } else if let Ok(dt_array) = time_steps.extract::<PyReadonlyArray1<f64>>() {
        dt_array.as_array().to_vec()
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "time_steps must be either a float or a 1D NumPy array of floats"
        ));
    };

    // Call batch propagator
    let result = propagators::keplerian::batch_propagate_lagrange(
        states_array,
        &dt_vec,
        mu
    )?;

    // Return as NumPy array
    Ok(PyArray2::from_owned_array_bound(py, result))
}

// ============================================================================
// J2 Perturbation Functions
// ============================================================================

/// Calculate J2 oblateness perturbation acceleration
///
/// Computes the acceleration due to Earth's oblateness (J2 term) in Cartesian coordinates.
/// This is the dominant perturbation for Earth-orbiting satellites.
///
/// # Arguments
/// * `r` - Position vector [x, y, z] in meters (NumPy array)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient (dimensionless, ~1.08263e-3 for Earth)
/// * `R` - Body equatorial radius (meters)
///
/// # Returns
/// Acceleration vector [ax, ay, az] in m/s² (NumPy array)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import j2_perturbation, constants
///
/// r = np.array([7000e3, 0.0, 0.0])  # Position on equator
/// acc = j2_perturbation(r, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH)
/// print(f"J2 acceleration: {acc} m/s²")
/// ```
#[pyfunction]
#[pyo3(name = "j2_perturbation")]
fn py_j2_perturbation<'py>(
    py: Python<'py>,
    r: PyReadonlyArray1<f64>,
    mu: f64,
    j2: f64,
    R: f64,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let r_array = r.as_array();

    if r_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vector must have exactly 3 components"
        ));
    }

    let r_vec = core::linalg::Vector3::new(r_array[0], r_array[1], r_array[2]);
    let acc = propagators::perturbations::j2_perturbation(&r_vec, mu, j2, R);

    let acc_array = ndarray::arr1(&[acc.x, acc.y, acc.z]);
    Ok(PyArray1::from_owned_array_bound(py, acc_array))
}

/// Propagate orbit with J2 perturbation using RK4 integrator
///
/// Propagates a state vector forward in time accounting for Earth's oblateness.
/// Uses fixed-step RK4 integration with multiple sub-steps for accuracy.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient (default: J2_EARTH from constants)
/// * `R` - Body equatorial radius in meters (default: R_EARTH from constants)
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
///
/// # Returns
/// Tuple of (position, velocity) NumPy arrays at time t₀ + Δt
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_j2_rk4, constants
///
/// # ISS orbit
/// r0 = np.array([6778e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7672.0, 0.0])
///
/// # Propagate for 1 hour with J2 perturbation
/// r1, v1 = propagate_j2_rk4(
///     r0, v0, 3600.0,
///     constants.GM_EARTH,
///     constants.J2_EARTH,
///     constants.R_EARTH,
///     n_steps=100
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_j2_rk4", signature = (r0, v0, dt, mu, j2, R, n_steps=None))]
fn py_propagate_j2_rk4<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    n_steps: Option<usize>,
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
    let (r_vec, v_vec) = propagators::perturbations::propagate_j2_rk4(
        &r0_vec, &v0_vec, dt, mu, j2, R, n_steps
    )?;

    // Convert back to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with J2 perturbation using adaptive DOPRI5 integrator
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
/// Automatically adjusts step size to maintain specified error tolerance.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius in meters
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (position, velocity) NumPy arrays at time t₀ + Δt
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_j2_dopri5, constants
///
/// r0 = np.array([7000e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7546.0, 0.0])
///
/// # High-precision propagation
/// r1, v1 = propagate_j2_dopri5(
///     r0, v0, 3600.0,
///     constants.GM_EARTH,
///     constants.J2_EARTH,
///     constants.R_EARTH,
///     tol=1e-10
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_j2_dopri5", signature = (r0, v0, dt, mu, j2, R, tol=None))]
fn py_propagate_j2_dopri5<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    tol: Option<f64>,
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
    let (r_vec, v_vec) = propagators::perturbations::propagate_j2_dopri5(
        &r0_vec, &v0_vec, dt, mu, j2, R, tol
    )?;

    // Convert back to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with J2 perturbation using ultra-high precision DOP853 integrator
///
/// Uses Dormand-Prince 8(5,3) adaptive integration for maximum accuracy.
/// Recommended for problems requiring very tight error tolerances (tol < 1e-10)
/// or long-duration high-precision propagation.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (GM) in m³/s²
/// * `j2` - Oblateness coefficient (dimensionless)
/// * `R` - Body equatorial radius in meters
/// * `tol` - Error tolerance (default: 1e-10)
///
/// # Returns
/// Tuple of (final position array, final velocity array)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_j2_dop853, constants
///
/// r0 = np.array([7000e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7546.0, 0.0])
///
/// # Ultra-high precision propagation
/// r1, v1 = propagate_j2_dop853(
///     r0, v0, 3600.0,
///     constants.GM_EARTH,
///     constants.J2_EARTH,
///     constants.R_EARTH,
///     tol=1e-12
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_j2_dop853", signature = (r0, v0, dt, mu, j2, R, tol=None))]
fn py_propagate_j2_dop853<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    tol: Option<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    // Convert NumPy arrays to Rust arrays
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
    let (r_vec, v_vec) = propagators::perturbations::propagate_j2_dop853(
        &r0_vec, &v0_vec, dt, mu, j2, R, tol
    )?;

    // Convert back to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

// =============================================================================
// HIGH-PERFORMANCE STATIC J2 PROPAGATION (ZERO-ALLOCATION)
// =============================================================================

/// Propagate orbit with J2 using high-performance static RK4 integrator
///
/// **⚡ PERFORMANCE**: This function uses stack-allocated vectors for
/// zero heap allocations during integration, providing 3-5x speedup
/// compared to the standard `propagate_j2_rk4` function.
///
/// **USE WHEN**: You need maximum performance for RK4 propagation (e.g.,
/// Monte Carlo simulations, porkchop plots, constellation propagation).
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient (Earth: 1.08263×10⁻³)
/// * `R` - Body equatorial radius in meters (Earth: 6,378,137 m)
/// * `n_steps` - Number of RK4 integration steps
///
/// # Returns
/// Tuple of (position, velocity) NumPy arrays at time t₀ + Δt
///
/// # Performance Notes
/// - Zero heap allocations during integration (all stack)
/// - ~3-5x faster than standard `propagate_j2_rk4`
/// - Best for fixed-step propagation (use DOPRI5/DOP853 for adaptive)
/// - Thread-safe (no shared state)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_j2_rk4_static, constants
///
/// # ISS orbit
/// r0 = np.array([6778e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7672.0, 0.0])
///
/// # Ultra-fast J2 propagation (1 orbit, 100 steps)
/// r1, v1 = propagate_j2_rk4_static(
///     r0, v0, 5400.0,  # ~90 min
///     constants.GM_EARTH,
///     constants.J2_EARTH,
///     constants.R_EARTH,
///     n_steps=100
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_j2_rk4_static")]
fn py_propagate_j2_rk4_static<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    n_steps: usize,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    use core::integrators_static::{propagate_rk4_final_only, StateVector6};
    use propagators::perturbations_static::j2_dynamics;

    // Convert NumPy arrays to initial state
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let state0 = StateVector6::new(
        r0_array[0], r0_array[1], r0_array[2],
        v0_array[0], v0_array[1], v0_array[2],
    );

    // Create J2 dynamics function
    let dynamics = j2_dynamics(mu, j2, R);

    // Propagate using zero-allocation RK4
    let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, dt, n_steps);

    // Extract position and velocity
    let r_array = ndarray::arr1(&[state_final[0], state_final[1], state_final[2]]);
    let v_array = ndarray::arr1(&[state_final[3], state_final[4], state_final[5]]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with J2+J3+J4 using high-performance static RK4 integrator
///
/// **⚡ ULTRA-HIGH PERFORMANCE**: Zero-allocation propagation with full zonal harmonics.
/// Even faster than `propagate_j2_rk4_static` when you need higher fidelity.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - J2 oblateness coefficient
/// * `j3` - J3 pear-shape coefficient
/// * `j4` - J4 higher-order coefficient
/// * `R` - Body equatorial radius in meters
/// * `n_steps` - Number of RK4 integration steps
///
/// # Returns
/// Tuple of (position, velocity) NumPy arrays at time t₀ + Δt
///
/// # Earth Parameters
/// - J2 = 1.08263×10⁻³
/// - J3 = -2.53266×10⁻⁶
/// - J4 = -1.61962×10⁻⁶
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_j2_j3_j4_rk4_static, constants
///
/// r0 = np.array([7000e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7500.0, 1000.0])
///
/// # High-fidelity propagation with all zonal harmonics
/// r1, v1 = propagate_j2_j3_j4_rk4_static(
///     r0, v0, 3600.0,
///     constants.GM_EARTH,
///     constants.J2_EARTH,
///     -2.53266e-6,  # J3
///     -1.61962e-6,  # J4
///     constants.R_EARTH,
///     n_steps=100
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_j2_j3_j4_rk4_static")]
fn py_propagate_j2_j3_j4_rk4_static<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    j3: f64,
    j4: f64,
    R: f64,
    n_steps: usize,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    use core::integrators_static::{propagate_rk4_final_only, StateVector6};
    use propagators::perturbations_static::j2_j3_j4_dynamics;

    // Convert NumPy arrays to initial state
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let state0 = StateVector6::new(
        r0_array[0], r0_array[1], r0_array[2],
        v0_array[0], v0_array[1], v0_array[2],
    );

    // Create J2+J3+J4 dynamics function
    let dynamics = j2_j3_j4_dynamics(mu, j2, j3, j4, R);

    // Propagate using zero-allocation RK4
    let state_final = propagate_rk4_final_only(dynamics, 0.0, &state0, dt, n_steps);

    // Extract position and velocity
    let r_array = ndarray::arr1(&[state_final[0], state_final[1], state_final[2]]);
    let v_array = ndarray::arr1(&[state_final[3], state_final[4], state_final[5]]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

// =============================================================================
// ATMOSPHERIC DRAG PERTURBATION FUNCTIONS
// =============================================================================

/// Compute atmospheric density using exponential atmosphere model
///
/// Calculates atmospheric density at a given altitude using:
/// ρ(h) = ρ₀ × exp(-h/H)
///
/// # Arguments
/// * `altitude` - Altitude above reference surface (m)
/// * `rho0` - Reference density at surface (kg/m³)
/// * `H0` - Atmospheric scale height (m)
///
/// # Returns
/// Atmospheric density (kg/m³)
///
/// # Example (Python)
/// ```python
/// from astrora._core import exponential_density, constants
///
/// # Density at 400 km altitude
/// rho = exponential_density(400e3, constants.RHO0_EARTH, constants.H0_EARTH)
/// ```
#[pyfunction]
#[pyo3(name = "exponential_density")]
fn py_exponential_density(altitude: f64, rho0: f64, H0: f64) -> f64 {
    propagators::perturbations::exponential_density(altitude, rho0, H0)
}

/// Compute drag acceleration using exponential atmosphere model
///
/// Returns the acceleration due to atmospheric drag given the current
/// orbital state and spacecraft ballistic coefficient.
///
/// # Arguments
/// * `r` - Position vector [x, y, z] in meters (NumPy array)
/// * `v` - Velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient B = m/(C_d × A) in kg/m²
///
/// # Returns
/// Acceleration vector [ax, ay, az] in m/s² (NumPy array)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import drag_acceleration, constants
///
/// # ISS-like orbit at 400 km
/// r = np.array([6778e3, 0.0, 0.0])
/// v = np.array([0.0, 7670.0, 0.0])
/// B = 50.0  # Ballistic coefficient (kg/m²)
///
/// a_drag = drag_acceleration(r, v, constants.R_EARTH,
///                             constants.RHO0_EARTH,
///                             constants.H0_EARTH, B)
/// ```
#[pyfunction]
#[pyo3(name = "drag_acceleration")]
fn py_drag_acceleration<'py>(
    py: Python<'py>,
    r: PyReadonlyArray1<f64>,
    v: PyReadonlyArray1<f64>,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let r_array = r.as_array();
    let v_array = v.as_array();

    if r_array.len() != 3 || v_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r_vec = core::linalg::Vector3::new(r_array[0], r_array[1], r_array[2]);
    let v_vec = core::linalg::Vector3::new(v_array[0], v_array[1], v_array[2]);

    let a_vec = propagators::perturbations::drag_acceleration(&r_vec, &v_vec, R, rho0, H0, B);

    let a_array = ndarray::arr1(&[a_vec.x, a_vec.y, a_vec.z]);
    Ok(PyArray1::from_owned_array_bound(py, a_array))
}

/// Propagate orbit with atmospheric drag using RK4 integration
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_drag_rk4, constants
///
/// r0 = np.array([6778e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7670.0, 0.0])
/// B = 50.0  # CubeSat ballistic coefficient
///
/// r1, v1 = propagate_drag_rk4(
///     r0, v0, 3600.0,
///     constants.GM_EARTH,
///     constants.R_EARTH,
///     constants.RHO0_EARTH,
///     constants.H0_EARTH,
///     B,
///     n_steps=100
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_drag_rk4", signature = (r0, v0, dt, mu, R, rho0, H0, B, n_steps=None))]
fn py_propagate_drag_rk4<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    n_steps: Option<usize>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_drag_rk4(
        &r0_vec, &v0_vec, dt, mu, R, rho0, H0, B, n_steps
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with atmospheric drag using adaptive DOPRI5 integration
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
#[pyfunction]
#[pyo3(name = "propagate_drag_dopri5", signature = (r0, v0, dt, mu, R, rho0, H0, B, tol=None))]
fn py_propagate_drag_dopri5<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    tol: Option<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_drag_dopri5(
        &r0_vec, &v0_vec, dt, mu, R, rho0, H0, B, tol
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with J2 + drag perturbations using RK4 integration
///
/// Combines Earth oblateness (J2) and atmospheric drag effects.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
#[pyfunction]
#[pyo3(name = "propagate_j2_drag_rk4", signature = (r0, v0, dt, mu, j2, R, rho0, H0, B, n_steps=None))]
fn py_propagate_j2_drag_rk4<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    n_steps: Option<usize>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_j2_drag_rk4(
        &r0_vec, &v0_vec, dt, mu, j2, R, rho0, H0, B, n_steps
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with J2 + drag perturbations using adaptive DOPRI5 integration
///
/// High-accuracy propagation combining Earth oblateness and atmospheric drag.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `rho0` - Reference atmospheric density (kg/m³)
/// * `H0` - Atmospheric scale height (m)
/// * `B` - Ballistic coefficient (kg/m²)
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
#[pyfunction]
#[pyo3(name = "propagate_j2_drag_dopri5", signature = (r0, v0, dt, mu, j2, R, rho0, H0, B, tol=None))]
fn py_propagate_j2_drag_dopri5<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    rho0: f64,
    H0: f64,
    B: f64,
    tol: Option<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_j2_drag_dopri5(
        &r0_vec, &v0_vec, dt, mu, j2, R, rho0, H0, B, tol
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

// =============================================================================
// THIRD-BODY PERTURBATIONS (SUN AND MOON)
// =============================================================================

/// Simplified ephemeris for Sun position relative to Earth
///
/// Returns the Sun's position vector using a simple circular orbit approximation.
///
/// # Arguments
/// * `t` - Time since J2000 epoch (seconds)
///
/// # Returns
/// Sun position vector in ECI frame [x, y, z] in meters (NumPy array)
///
/// # Example (Python)
/// ```python
/// from astrora._core import sun_position_simple
///
/// # Sun position at J2000 epoch
/// r_sun = sun_position_simple(0.0)
/// print(f"Sun position: {r_sun / 1e9} billion meters")
/// ```
#[pyfunction]
#[pyo3(name = "sun_position_simple")]
fn py_sun_position_simple(py: Python, t: f64) -> Bound<PyArray1<f64>> {
    let r_sun = propagators::perturbations::sun_position_simple(t);
    let r_array = ndarray::arr1(&[r_sun.x, r_sun.y, r_sun.z]);
    PyArray1::from_owned_array_bound(py, r_array)
}

/// Simplified ephemeris for Moon position relative to Earth
///
/// Returns the Moon's position vector using a simple circular orbit approximation.
///
/// # Arguments
/// * `t` - Time since J2000 epoch (seconds)
///
/// # Returns
/// Moon position vector in ECI frame [x, y, z] in meters (NumPy array)
///
/// # Example (Python)
/// ```python
/// from astrora._core import moon_position_simple
///
/// # Moon position at J2000 epoch
/// r_moon = moon_position_simple(0.0)
/// print(f"Moon distance: {np.linalg.norm(r_moon) / 1e3} km")
/// ```
#[pyfunction]
#[pyo3(name = "moon_position_simple")]
fn py_moon_position_simple(py: Python, t: f64) -> Bound<PyArray1<f64>> {
    let r_moon = propagators::perturbations::moon_position_simple(t);
    let r_array = ndarray::arr1(&[r_moon.x, r_moon.y, r_moon.z]);
    PyArray1::from_owned_array_bound(py, r_array)
}

/// Third-body perturbation acceleration (point-mass approximation)
///
/// Computes the gravitational perturbation acceleration on a satellite due to
/// a third body (e.g., Sun or Moon) using the point-mass approximation.
///
/// # Arguments
/// * `r` - Satellite position vector [x, y, z] relative to Earth (m) (NumPy array)
/// * `r_third` - Third body position vector [x, y, z] relative to Earth (m) (NumPy array)
/// * `mu_third` - Gravitational parameter of third body (m³/s²)
///
/// # Returns
/// Perturbation acceleration vector [ax, ay, az] in m/s² (NumPy array)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import third_body_perturbation, sun_position_simple, constants
///
/// # Satellite at LEO
/// r_sat = np.array([7000e3, 0.0, 0.0])
///
/// # Sun position at J2000
/// r_sun = sun_position_simple(0.0)
///
/// # Compute Sun perturbation
/// a_sun = third_body_perturbation(r_sat, r_sun, constants.GM_SUN)
/// print(f"Sun perturbation: {np.linalg.norm(a_sun):.2e} m/s²")
/// ```
#[pyfunction]
#[pyo3(name = "third_body_perturbation")]
fn py_third_body_perturbation<'py>(
    py: Python<'py>,
    r: PyReadonlyArray1<f64>,
    r_third: PyReadonlyArray1<f64>,
    mu_third: f64,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let r_array = r.as_array();
    let r_third_array = r_third.as_array();

    if r_array.len() != 3 || r_third_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vectors must have exactly 3 components"
        ));
    }

    let r_vec = core::linalg::Vector3::new(r_array[0], r_array[1], r_array[2]);
    let r_third_vec = core::linalg::Vector3::new(r_third_array[0], r_third_array[1], r_third_array[2]);

    let a_vec = propagators::perturbations::third_body_perturbation(&r_vec, &r_third_vec, mu_third);

    let a_array = ndarray::arr1(&[a_vec.x, a_vec.y, a_vec.z]);
    Ok(PyArray1::from_owned_array_bound(py, a_array))
}

/// Combined Sun and Moon third-body perturbation acceleration
///
/// Convenience function that computes the total perturbation from both
/// the Sun and Moon using simplified ephemerides.
///
/// # Arguments
/// * `r` - Satellite position vector [x, y, z] relative to Earth (m) (NumPy array)
/// * `t` - Time since J2000 epoch (seconds)
///
/// # Returns
/// Combined perturbation acceleration [ax, ay, az] from Sun and Moon in m/s² (NumPy array)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import sun_moon_perturbation
///
/// # GEO satellite
/// r_geo = np.array([42164e3, 0.0, 0.0])
///
/// # Combined Sun+Moon perturbation at J2000
/// a_thirdbody = sun_moon_perturbation(r_geo, 0.0)
/// print(f"Third-body perturbation: {np.linalg.norm(a_thirdbody):.2e} m/s²")
/// ```
#[pyfunction]
#[pyo3(name = "sun_moon_perturbation")]
fn py_sun_moon_perturbation<'py>(
    py: Python<'py>,
    r: PyReadonlyArray1<f64>,
    t: f64,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let r_array = r.as_array();

    if r_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vector must have exactly 3 components"
        ));
    }

    let r_vec = core::linalg::Vector3::new(r_array[0], r_array[1], r_array[2]);
    let a_vec = propagators::perturbations::sun_moon_perturbation(&r_vec, t);

    let a_array = ndarray::arr1(&[a_vec.x, a_vec.y, a_vec.z]);
    Ok(PyArray1::from_owned_array_bound(py, a_array))
}

/// Propagate orbit with third-body perturbations using RK4 integration
///
/// Propagates a state vector forward in time accounting for third-body
/// gravitational perturbations from the Sun and/or Moon.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter of primary body (m³/s²)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `include_sun` - Include Sun perturbation (default: True)
/// * `include_moon` - Include Moon perturbation (default: True)
/// * `n_steps` - Number of RK4 sub-steps (default: 10)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_thirdbody_rk4, constants
///
/// # GEO satellite
/// r0 = np.array([42164e3, 0.0, 0.0])
/// v0 = np.array([0.0, 3075.0, 0.0])
///
/// # Propagate for 1 day with Sun+Moon perturbations
/// r1, v1 = propagate_thirdbody_rk4(
///     r0, v0, 86400.0,
///     constants.GM_EARTH, 0.0,
///     include_sun=True, include_moon=True,
///     n_steps=1000
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_thirdbody_rk4", signature = (r0, v0, dt, mu, t0, include_sun=true, include_moon=true, n_steps=None))]
fn py_propagate_thirdbody_rk4<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    t0: f64,
    include_sun: bool,
    include_moon: bool,
    n_steps: Option<usize>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_thirdbody_rk4(
        &r0_vec, &v0_vec, dt, mu, t0, include_sun, include_moon, n_steps
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with third-body perturbations using adaptive DOPRI5 integration
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter of primary body (m³/s²)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `include_sun` - Include Sun perturbation (default: True)
/// * `include_moon` - Include Moon perturbation (default: True)
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_thirdbody_dopri5, constants
///
/// r0 = np.array([42164e3, 0.0, 0.0])
/// v0 = np.array([0.0, 3075.0, 0.0])
///
/// r1, v1 = propagate_thirdbody_dopri5(
///     r0, v0, 86400.0,
///     constants.GM_EARTH, 0.0,
///     include_sun=True, include_moon=True,
///     tol=1e-10
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_thirdbody_dopri5", signature = (r0, v0, dt, mu, t0, include_sun=true, include_moon=true, tol=None))]
fn py_propagate_thirdbody_dopri5<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    t0: f64,
    include_sun: bool,
    include_moon: bool,
    tol: Option<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_thirdbody_dopri5(
        &r0_vec, &v0_vec, dt, mu, t0, include_sun, include_moon, tol
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

// =============================================================================
// SOLAR RADIATION PRESSURE (SRP) PYTHON BINDINGS
// =============================================================================

/// Compute shadow function for solar radiation pressure
///
/// Determines the degree of illumination of a spacecraft by the Sun,
/// accounting for Earth's shadow (umbra and penumbra regions).
///
/// # Arguments
/// * `r_sat` - Satellite position vector [x, y, z] in meters (NumPy array)
/// * `r_sun` - Sun position vector [x, y, z] in meters (NumPy array)
/// * `R_earth` - Earth's radius in meters
///
/// # Returns
/// Shadow factor k ∈ [0, 1]:
/// - k = 1.0: Full sunlight (no shadow)
/// - k = 0.0: Full umbra (complete shadow)
/// - 0.0 < k < 1.0: Partial shadow (penumbra)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import shadow_function, sun_position_simple, constants
///
/// r_sat = np.array([7000e3, 0.0, 0.0])
/// r_sun = sun_position_simple(0.0)
/// k = shadow_function(r_sat, r_sun, constants.R_EARTH)
/// print(f"Shadow factor: {k}")
/// ```
#[pyfunction]
#[pyo3(name = "shadow_function")]
fn py_shadow_function(
    r_sat: PyReadonlyArray1<f64>,
    r_sun: PyReadonlyArray1<f64>,
    R_earth: f64,
) -> PyResult<f64> {
    let r_sat_array = r_sat.as_array();
    let r_sun_array = r_sun.as_array();

    if r_sat_array.len() != 3 || r_sun_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vectors must have exactly 3 components"
        ));
    }

    let r_sat_vec = core::linalg::Vector3::new(r_sat_array[0], r_sat_array[1], r_sat_array[2]);
    let r_sun_vec = core::linalg::Vector3::new(r_sun_array[0], r_sun_array[1], r_sun_array[2]);

    let k = propagators::perturbations::shadow_function(&r_sat_vec, &r_sun_vec, R_earth);

    Ok(k)
}

/// Solar radiation pressure acceleration (cannon-ball model)
///
/// Computes the acceleration due to solar radiation pressure using the
/// simple cannon-ball model (spacecraft treated as a sphere).
///
/// # Arguments
/// * `r_sat` - Satellite position vector [x, y, z] in meters (NumPy array)
/// * `r_sun` - Sun position vector [x, y, z] in meters (NumPy array)
/// * `area_mass_ratio` - Cross-sectional area divided by mass (A/m) in m²/kg
/// * `C_r` - Reflectivity coefficient (typically 1.0-2.0)
/// * `R_earth` - Earth's radius for shadow calculations (m)
///
/// # Returns
/// Acceleration vector [ax, ay, az] in m/s² (NumPy array)
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import srp_acceleration, sun_position_simple, constants
///
/// # CubeSat: 0.1m × 0.1m × 0.1m, mass 1 kg
/// # A = 0.01 m² (one face), A/m = 0.01 m²/kg
/// area_mass_ratio = 0.01
/// C_r = 1.3  # Typical value
///
/// r_sat = np.array([42164e3, 0.0, 0.0])  # GEO
/// r_sun = sun_position_simple(0.0)
/// a_srp = srp_acceleration(r_sat, r_sun, area_mass_ratio, C_r, constants.R_EARTH)
/// print(f"SRP acceleration: {a_srp} m/s²")
/// ```
#[pyfunction]
#[pyo3(name = "srp_acceleration")]
fn py_srp_acceleration<'py>(
    py: Python<'py>,
    r_sat: PyReadonlyArray1<f64>,
    r_sun: PyReadonlyArray1<f64>,
    area_mass_ratio: f64,
    C_r: f64,
    R_earth: f64,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let r_sat_array = r_sat.as_array();
    let r_sun_array = r_sun.as_array();

    if r_sat_array.len() != 3 || r_sun_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vectors must have exactly 3 components"
        ));
    }

    let r_sat_vec = core::linalg::Vector3::new(r_sat_array[0], r_sat_array[1], r_sat_array[2]);
    let r_sun_vec = core::linalg::Vector3::new(r_sun_array[0], r_sun_array[1], r_sun_array[2]);

    let a_vec = propagators::perturbations::srp_acceleration(&r_sat_vec, &r_sun_vec, area_mass_ratio, C_r, R_earth);

    let a_array = ndarray::arr1(&[a_vec.x, a_vec.y, a_vec.z]);
    Ok(PyArray1::from_owned_array_bound(py, a_array))
}

/// Propagate orbit with solar radiation pressure using RK4 integration
///
/// Propagates a state vector forward in time accounting for solar radiation
/// pressure using the cannon-ball model. Uses fixed-step RK4 integration.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `area_mass_ratio` - Cross-sectional area divided by mass (m²/kg)
/// * `C_r` - Reflectivity coefficient (1.0-2.0)
/// * `R_earth` - Earth's radius for shadow calculations (m)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `n_steps` - Number of RK4 sub-steps (default: 100)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_srp_rk4, constants
///
/// # CubeSat in GEO
/// r0 = np.array([42164e3, 0.0, 0.0])
/// v0 = np.array([0.0, 3075.0, 0.0])
/// area_mass_ratio = 0.01  # 0.01 m²/kg
/// C_r = 1.3
/// t0 = 0.0  # J2000 epoch
///
/// r1, v1 = propagate_srp_rk4(
///     r0, v0, 3600.0,
///     constants.GM_EARTH,
///     area_mass_ratio, C_r,
///     constants.R_EARTH,
///     t0,
///     n_steps=100
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_srp_rk4", signature = (r0, v0, dt, mu, area_mass_ratio, C_r, R_earth, t0, n_steps=None))]
fn py_propagate_srp_rk4<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    area_mass_ratio: f64,
    C_r: f64,
    R_earth: f64,
    t0: f64,
    n_steps: Option<usize>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_srp_rk4(
        &r0_vec, &v0_vec, dt, mu, area_mass_ratio, C_r, R_earth, t0, n_steps
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with solar radiation pressure using adaptive DOPRI5 integrator
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `area_mass_ratio` - Cross-sectional area divided by mass (m²/kg)
/// * `C_r` - Reflectivity coefficient (1.0-2.0)
/// * `R_earth` - Earth's radius for shadow calculations (m)
/// * `t0` - Initial time since J2000 epoch (seconds)
/// * `tol` - Error tolerance (default: 1e-8)
///
/// # Returns
/// Tuple of (final position, final velocity) as NumPy arrays
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_srp_dopri5, constants
///
/// r0 = np.array([42164e3, 0.0, 0.0])
/// v0 = np.array([0.0, 3075.0, 0.0])
/// area_mass_ratio = 0.01
/// C_r = 1.3
/// t0 = 0.0
///
/// r1, v1 = propagate_srp_dopri5(
///     r0, v0, 86400.0,  # 1 day
///     constants.GM_EARTH,
///     area_mass_ratio, C_r,
///     constants.R_EARTH,
///     t0,
///     tol=1e-10
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_srp_dopri5", signature = (r0, v0, dt, mu, area_mass_ratio, C_r, R_earth, t0, tol=None))]
fn py_propagate_srp_dopri5<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    area_mass_ratio: f64,
    C_r: f64,
    R_earth: f64,
    t0: f64,
    tol: Option<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec) = propagators::perturbations::propagate_srp_dopri5(
        &r0_vec, &v0_vec, dt, mu, area_mass_ratio, C_r, R_earth, t0, tol
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
    ))
}

/// Propagate orbit with state transition matrix using RK4 integration
///
/// Integrates both the orbital state (position and velocity) and the
/// state transition matrix simultaneously. The STM maps how small deviations
/// in initial conditions propagate forward in time.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `n_steps` - Number of RK4 sub-steps (default: 100)
///
/// # Returns
/// Tuple of (position, velocity, STM) where STM is a 6×6 NumPy array
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_stm_rk4, constants
///
/// # ISS orbit
/// r0 = np.array([6778e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7672.0, 0.0])
///
/// # Propagate for 10 minutes with STM
/// r, v, stm = propagate_stm_rk4(
///     r0, v0, 600.0,
///     constants.GM_EARTH,
///     n_steps=100
/// )
///
/// # Use STM to compute effect of 1m perturbation in x
/// dr0 = np.array([1.0, 0.0, 0.0, 0.0, 0.0, 0.0])
/// dr = stm @ dr0  # Perturbation propagated to final time
/// print(f"1m x-perturbation becomes {dr[0]:.3f}m after 10 min")
/// ```
#[pyfunction]
#[pyo3(name = "propagate_stm_rk4", signature = (r0, v0, dt, mu, n_steps=100))]
fn py_propagate_stm_rk4<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    n_steps: usize,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray2<f64>>)> {
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

    // Call STM propagator
    let (r_vec, v_vec, stm) = propagators::stm::propagate_stm_rk4(
        &r0_vec, &v0_vec, dt, mu, n_steps
    )?;

    // Convert state vectors to NumPy arrays
    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    // Convert STM to NumPy 2D array
    let mut stm_array = ndarray::Array2::<f64>::zeros((6, 6));
    for i in 0..6 {
        for j in 0..6 {
            stm_array[[i, j]] = stm[(i, j)];
        }
    }

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
        PyArray2::from_owned_array_bound(py, stm_array),
    ))
}

/// Propagate orbit with state transition matrix using adaptive DOPRI5 integration
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
/// Automatically adjusts step size to maintain specified error tolerance for
/// both the orbital state and the STM.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `tol` - Error tolerance (default: 1e-10)
///
/// # Returns
/// Tuple of (position, velocity, STM) where STM is a 6×6 NumPy array
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_stm_dopri5, constants
///
/// r0 = np.array([7000e3, 0.0, 0.0])
/// v0 = np.array([0.0, 7546.0, 0.0])
///
/// # High accuracy propagation with STM
/// r, v, stm = propagate_stm_dopri5(
///     r0, v0, 600.0,
///     constants.GM_EARTH,
///     tol=1e-12
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_stm_dopri5", signature = (r0, v0, dt, mu, tol=None))]
fn py_propagate_stm_dopri5<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    tol: Option<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray2<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec, stm) = propagators::stm::propagate_stm_dopri5(
        &r0_vec, &v0_vec, dt, mu, tol
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    let mut stm_array = ndarray::Array2::<f64>::zeros((6, 6));
    for i in 0..6 {
        for j in 0..6 {
            stm_array[[i, j]] = stm[(i, j)];
        }
    }

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
        PyArray2::from_owned_array_bound(py, stm_array),
    ))
}

/// Propagate orbit with STM including J2 perturbations using RK4
///
/// Extends STM propagation to include Earth oblateness (J2) effects.
/// The STM includes sensitivities to both two-body and J2 perturbations.
///
/// # Arguments
/// * `r0` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `v0` - Initial velocity vector [vx, vy, vz] in m/s (NumPy array)
/// * `dt` - Time step in seconds
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius in meters
/// * `n_steps` - Number of RK4 sub-steps (default: 100)
///
/// # Returns
/// Tuple of (position, velocity, STM) where STM is a 6×6 NumPy array
///
/// # Example (Python)
/// ```python
/// import numpy as np
/// from astrora._core import propagate_stm_j2_rk4, constants
///
/// r0 = np.array([7000e3, 0.0, 1000e3])  # Inclined orbit
/// v0 = np.array([0.0, 7546.0, 100.0])
///
/// r, v, stm = propagate_stm_j2_rk4(
///     r0, v0, 600.0,
///     constants.GM_EARTH,
///     constants.J2_EARTH,
///     constants.R_EARTH,
///     n_steps=100
/// )
/// ```
#[pyfunction]
#[pyo3(name = "propagate_stm_j2_rk4", signature = (r0, v0, dt, mu, j2, R, n_steps=100))]
fn py_propagate_stm_j2_rk4<'py>(
    py: Python<'py>,
    r0: PyReadonlyArray1<f64>,
    v0: PyReadonlyArray1<f64>,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    n_steps: usize,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>, Bound<'py, PyArray2<f64>>)> {
    let r0_array = r0.as_array();
    let v0_array = v0.as_array();

    if r0_array.len() != 3 || v0_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity vectors must have exactly 3 components"
        ));
    }

    let r0_vec = core::linalg::Vector3::new(r0_array[0], r0_array[1], r0_array[2]);
    let v0_vec = core::linalg::Vector3::new(v0_array[0], v0_array[1], v0_array[2]);

    let (r_vec, v_vec, stm) = propagators::stm::propagate_stm_j2_rk4(
        &r0_vec, &v0_vec, dt, mu, j2, R, n_steps
    )?;

    let r_array = ndarray::arr1(&[r_vec.x, r_vec.y, r_vec.z]);
    let v_array = ndarray::arr1(&[v_vec.x, v_vec.y, v_vec.z]);

    let mut stm_array = ndarray::Array2::<f64>::zeros((6, 6));
    for i in 0..6 {
        for j in 0..6 {
            stm_array[[i, j]] = stm[(i, j)];
        }
    }

    Ok((
        PyArray1::from_owned_array_bound(py, r_array),
        PyArray1::from_owned_array_bound(py, v_array),
        PyArray2::from_owned_array_bound(py, stm_array),
    ))
}

// =============================================================================
// Batch Coordinate Transformation Python Wrappers
// =============================================================================

/// Transform multiple state vectors from GCRS to ITRS coordinates in batch
///
/// Converts Geocentric Celestial Reference System (inertial) coordinates to
/// International Terrestrial Reference System (Earth-fixed) coordinates for
/// multiple epochs simultaneously. Significantly faster than individual transformations
/// due to vectorized operations.
///
/// # Arguments
/// * `positions` - 2D NumPy array of shape (N, 3) containing GCRS position vectors in meters
/// * `velocities` - 2D NumPy array of shape (N, 3) containing GCRS velocity vectors in m/s
/// * `obstimes` - List of N Epoch objects for each state vector
///
/// # Returns
/// Tuple of (ITRS positions, ITRS velocities) as 2D NumPy arrays
///
/// # Errors
/// Returns an error if:
/// - Array shapes are not (N, 3)
/// - Number of epochs doesn't match number of state vectors
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import batch_gcrs_to_itrs, Epoch
/// positions = np.array([[7000e3, 0, 0], [0, 7000e3, 0]])  # GCRS positions
/// velocities = np.array([[0, 7.5e3, 0], [-7.5e3, 0, 0]])  # GCRS velocities
/// epochs = [Epoch.from_mjd(60000.0), Epoch.from_mjd(60000.5)]
/// itrs_pos, itrs_vel = batch_gcrs_to_itrs(positions, velocities, epochs)
/// ```
#[pyfunction]
#[pyo3(name = "batch_gcrs_to_itrs")]
fn py_batch_gcrs_to_itrs<'py>(
    py: Python<'py>,
    positions: PyReadonlyArray2<f64>,
    velocities: PyReadonlyArray2<f64>,
    obstimes: Vec<core::time::Epoch>,
) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
    let pos_array = positions.as_array();
    let vel_array = velocities.as_array();

    if pos_array.shape()[1] != 3 || vel_array.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity arrays must have shape (N, 3)"
        ));
    }

    if pos_array.shape()[0] != obstimes.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Number of positions must match number of epochs"
        ));
    }

    // Convert to Vec<Vector3>
    let pos_vec: Vec<nalgebra::Vector3<f64>> = (0..pos_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(pos_array[[i, 0]], pos_array[[i, 1]], pos_array[[i, 2]]))
        .collect();

    let vel_vec: Vec<nalgebra::Vector3<f64>> = (0..vel_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(vel_array[[i, 0]], vel_array[[i, 1]], vel_array[[i, 2]]))
        .collect();

    // Call batch transformation
    let (itrs_pos, itrs_vel) = coordinates::frames::batch_gcrs_to_itrs(&pos_vec, &vel_vec, &obstimes)?;

    // Convert back to NumPy arrays
    let n = itrs_pos.len();
    let mut pos_out = ndarray::Array2::<f64>::zeros((n, 3));
    let mut vel_out = ndarray::Array2::<f64>::zeros((n, 3));

    for (i, (p, v)) in itrs_pos.iter().zip(itrs_vel.iter()).enumerate() {
        pos_out[[i, 0]] = p.x;
        pos_out[[i, 1]] = p.y;
        pos_out[[i, 2]] = p.z;
        vel_out[[i, 0]] = v.x;
        vel_out[[i, 1]] = v.y;
        vel_out[[i, 2]] = v.z;
    }

    Ok((
        PyArray2::from_owned_array_bound(py, pos_out),
        PyArray2::from_owned_array_bound(py, vel_out),
    ))
}

/// Transform multiple state vectors from ITRS to GCRS coordinates in batch
///
/// Converts International Terrestrial Reference System (Earth-fixed) coordinates to
/// Geocentric Celestial Reference System (inertial) coordinates for multiple epochs
/// simultaneously. This is the inverse of batch_gcrs_to_itrs.
///
/// # Arguments
/// * `positions` - 2D NumPy array of shape (N, 3) containing ITRS position vectors in meters
/// * `velocities` - 2D NumPy array of shape (N, 3) containing ITRS velocity vectors in m/s
/// * `obstimes` - List of N Epoch objects for each state vector
///
/// # Returns
/// Tuple of (GCRS positions, GCRS velocities) as 2D NumPy arrays
///
/// # Errors
/// Returns an error if array shapes or epoch count are invalid
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import batch_itrs_to_gcrs, Epoch
/// # ITRS coordinates (Earth-fixed)
/// positions = np.array([[7000e3, 0, 0], [0, 7000e3, 0]])
/// velocities = np.array([[0, 0.5e3, 0], [-0.5e3, 0, 0]])
/// epochs = [Epoch.from_mjd(60000.0), Epoch.from_mjd(60000.5)]
/// gcrs_pos, gcrs_vel = batch_itrs_to_gcrs(positions, velocities, epochs)
/// ```
#[pyfunction]
#[pyo3(name = "batch_itrs_to_gcrs")]
fn py_batch_itrs_to_gcrs<'py>(
    py: Python<'py>,
    positions: PyReadonlyArray2<f64>,
    velocities: PyReadonlyArray2<f64>,
    obstimes: Vec<core::time::Epoch>,
) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
    let pos_array = positions.as_array();
    let vel_array = velocities.as_array();

    if pos_array.shape()[1] != 3 || vel_array.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity arrays must have shape (N, 3)"
        ));
    }

    if pos_array.shape()[0] != obstimes.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Number of positions must match number of epochs"
        ));
    }

    // Convert to Vec<Vector3>
    let pos_vec: Vec<nalgebra::Vector3<f64>> = (0..pos_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(pos_array[[i, 0]], pos_array[[i, 1]], pos_array[[i, 2]]))
        .collect();

    let vel_vec: Vec<nalgebra::Vector3<f64>> = (0..vel_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(vel_array[[i, 0]], vel_array[[i, 1]], vel_array[[i, 2]]))
        .collect();

    // Call batch transformation
    let (gcrs_pos, gcrs_vel) = coordinates::frames::batch_itrs_to_gcrs(&pos_vec, &vel_vec, &obstimes)?;

    // Convert back to NumPy arrays
    let n = gcrs_pos.len();
    let mut pos_out = ndarray::Array2::<f64>::zeros((n, 3));
    let mut vel_out = ndarray::Array2::<f64>::zeros((n, 3));

    for (i, (p, v)) in gcrs_pos.iter().zip(gcrs_vel.iter()).enumerate() {
        pos_out[[i, 0]] = p.x;
        pos_out[[i, 1]] = p.y;
        pos_out[[i, 2]] = p.z;
        vel_out[[i, 0]] = v.x;
        vel_out[[i, 1]] = v.y;
        vel_out[[i, 2]] = v.z;
    }

    Ok((
        PyArray2::from_owned_array_bound(py, pos_out),
        PyArray2::from_owned_array_bound(py, vel_out),
    ))
}

/// Transform multiple state vectors from GCRS to TEME coordinates in batch
///
/// Converts Geocentric Celestial Reference System coordinates to True Equator
/// Mean Equinox (TEME) coordinates for multiple epochs. TEME is the standard
/// reference frame for SGP4/SDP4 satellite propagation.
///
/// # Arguments
/// * `positions` - 2D NumPy array of shape (N, 3) containing GCRS position vectors in meters
/// * `velocities` - 2D NumPy array of shape (N, 3) containing GCRS velocity vectors in m/s
/// * `obstimes` - List of N Epoch objects for each state vector
///
/// # Returns
/// Tuple of (TEME positions, TEME velocities) as 2D NumPy arrays
///
/// # Errors
/// Returns an error if array shapes or epoch count are invalid
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import batch_gcrs_to_teme, Epoch
/// positions = np.array([[7000e3, 0, 0], [0, 7000e3, 0]])
/// velocities = np.array([[0, 7.5e3, 0], [-7.5e3, 0, 0]])
/// epochs = [Epoch.from_mjd(60000.0), Epoch.from_mjd(60000.5)]
/// teme_pos, teme_vel = batch_gcrs_to_teme(positions, velocities, epochs)
/// ```
#[pyfunction]
#[pyo3(name = "batch_gcrs_to_teme")]
fn py_batch_gcrs_to_teme<'py>(
    py: Python<'py>,
    positions: PyReadonlyArray2<f64>,
    velocities: PyReadonlyArray2<f64>,
    obstimes: Vec<core::time::Epoch>,
) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
    let pos_array = positions.as_array();
    let vel_array = velocities.as_array();

    if pos_array.shape()[1] != 3 || vel_array.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity arrays must have shape (N, 3)"
        ));
    }

    if pos_array.shape()[0] != obstimes.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Number of positions must match number of epochs"
        ));
    }

    // Convert to Vec<Vector3>
    let pos_vec: Vec<nalgebra::Vector3<f64>> = (0..pos_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(pos_array[[i, 0]], pos_array[[i, 1]], pos_array[[i, 2]]))
        .collect();

    let vel_vec: Vec<nalgebra::Vector3<f64>> = (0..vel_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(vel_array[[i, 0]], vel_array[[i, 1]], vel_array[[i, 2]]))
        .collect();

    // Call batch transformation
    let (teme_pos, teme_vel) = coordinates::frames::batch_gcrs_to_teme(&pos_vec, &vel_vec, &obstimes)?;

    // Convert back to NumPy arrays
    let n = teme_pos.len();
    let mut pos_out = ndarray::Array2::<f64>::zeros((n, 3));
    let mut vel_out = ndarray::Array2::<f64>::zeros((n, 3));

    for (i, (p, v)) in teme_pos.iter().zip(teme_vel.iter()).enumerate() {
        pos_out[[i, 0]] = p.x;
        pos_out[[i, 1]] = p.y;
        pos_out[[i, 2]] = p.z;
        vel_out[[i, 0]] = v.x;
        vel_out[[i, 1]] = v.y;
        vel_out[[i, 2]] = v.z;
    }

    Ok((
        PyArray2::from_owned_array_bound(py, pos_out),
        PyArray2::from_owned_array_bound(py, vel_out),
    ))
}

/// Transform multiple state vectors from TEME to GCRS coordinates in batch
///
/// Converts True Equator Mean Equinox (TEME) coordinates to Geocentric Celestial
/// Reference System coordinates for multiple epochs. Useful for converting SGP4/SDP4
/// propagated states to standard inertial coordinates.
///
/// # Arguments
/// * `positions` - 2D NumPy array of shape (N, 3) containing TEME position vectors in meters
/// * `velocities` - 2D NumPy array of shape (N, 3) containing TEME velocity vectors in m/s
/// * `obstimes` - List of N Epoch objects for each state vector
///
/// # Returns
/// Tuple of (GCRS positions, GCRS velocities) as 2D NumPy arrays
///
/// # Errors
/// Returns an error if array shapes or epoch count are invalid
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import batch_teme_to_gcrs, Epoch
/// # TEME coordinates from SGP4 propagation
/// positions = np.array([[7000e3, 0, 0], [0, 7000e3, 0]])
/// velocities = np.array([[0, 7.5e3, 0], [-7.5e3, 0, 0]])
/// epochs = [Epoch.from_mjd(60000.0), Epoch.from_mjd(60000.5)]
/// gcrs_pos, gcrs_vel = batch_teme_to_gcrs(positions, velocities, epochs)
/// ```
#[pyfunction]
#[pyo3(name = "batch_teme_to_gcrs")]
fn py_batch_teme_to_gcrs<'py>(
    py: Python<'py>,
    positions: PyReadonlyArray2<f64>,
    velocities: PyReadonlyArray2<f64>,
    obstimes: Vec<core::time::Epoch>,
) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
    let pos_array = positions.as_array();
    let vel_array = velocities.as_array();

    if pos_array.shape()[1] != 3 || vel_array.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity arrays must have shape (N, 3)"
        ));
    }

    if pos_array.shape()[0] != obstimes.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Number of positions must match number of epochs"
        ));
    }

    // Convert to Vec<Vector3>
    let pos_vec: Vec<nalgebra::Vector3<f64>> = (0..pos_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(pos_array[[i, 0]], pos_array[[i, 1]], pos_array[[i, 2]]))
        .collect();

    let vel_vec: Vec<nalgebra::Vector3<f64>> = (0..vel_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(vel_array[[i, 0]], vel_array[[i, 1]], vel_array[[i, 2]]))
        .collect();

    // Call batch transformation
    let (gcrs_pos, gcrs_vel) = coordinates::frames::batch_teme_to_gcrs(&pos_vec, &vel_vec, &obstimes)?;

    // Convert back to NumPy arrays
    let n = gcrs_pos.len();
    let mut pos_out = ndarray::Array2::<f64>::zeros((n, 3));
    let mut vel_out = ndarray::Array2::<f64>::zeros((n, 3));

    for (i, (p, v)) in gcrs_pos.iter().zip(gcrs_vel.iter()).enumerate() {
        pos_out[[i, 0]] = p.x;
        pos_out[[i, 1]] = p.y;
        pos_out[[i, 2]] = p.z;
        vel_out[[i, 0]] = v.x;
        vel_out[[i, 1]] = v.y;
        vel_out[[i, 2]] = v.z;
    }

    Ok((
        PyArray2::from_owned_array_bound(py, pos_out),
        PyArray2::from_owned_array_bound(py, vel_out),
    ))
}

/// Transform multiple state vectors from TEME to ITRS coordinates in batch
///
/// Converts True Equator Mean Equinox (TEME) coordinates to International
/// Terrestrial Reference System (Earth-fixed) coordinates for multiple epochs.
/// Useful for ground track calculations from SGP4 propagated states.
///
/// # Arguments
/// * `positions` - 2D NumPy array of shape (N, 3) containing TEME position vectors in meters
/// * `velocities` - 2D NumPy array of shape (N, 3) containing TEME velocity vectors in m/s
/// * `obstimes` - List of N Epoch objects for each state vector
///
/// # Returns
/// Tuple of (ITRS positions, ITRS velocities) as 2D NumPy arrays
///
/// # Errors
/// Returns an error if array shapes or epoch count are invalid
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import batch_teme_to_itrs, Epoch
/// # TEME coordinates from SGP4
/// positions = np.array([[7000e3, 0, 0], [0, 7000e3, 0]])
/// velocities = np.array([[0, 7.5e3, 0], [-7.5e3, 0, 0]])
/// epochs = [Epoch.from_mjd(60000.0), Epoch.from_mjd(60000.5)]
/// itrs_pos, itrs_vel = batch_teme_to_itrs(positions, velocities, epochs)
/// ```
#[pyfunction]
#[pyo3(name = "batch_teme_to_itrs")]
fn py_batch_teme_to_itrs<'py>(
    py: Python<'py>,
    positions: PyReadonlyArray2<f64>,
    velocities: PyReadonlyArray2<f64>,
    obstimes: Vec<core::time::Epoch>,
) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
    let pos_array = positions.as_array();
    let vel_array = velocities.as_array();

    if pos_array.shape()[1] != 3 || vel_array.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity arrays must have shape (N, 3)"
        ));
    }

    if pos_array.shape()[0] != obstimes.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Number of positions must match number of epochs"
        ));
    }

    // Convert to Vec<Vector3>
    let pos_vec: Vec<nalgebra::Vector3<f64>> = (0..pos_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(pos_array[[i, 0]], pos_array[[i, 1]], pos_array[[i, 2]]))
        .collect();

    let vel_vec: Vec<nalgebra::Vector3<f64>> = (0..vel_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(vel_array[[i, 0]], vel_array[[i, 1]], vel_array[[i, 2]]))
        .collect();

    // Call batch transformation
    let (itrs_pos, itrs_vel) = coordinates::frames::batch_teme_to_itrs(&pos_vec, &vel_vec, &obstimes)?;

    // Convert back to NumPy arrays
    let n = itrs_pos.len();
    let mut pos_out = ndarray::Array2::<f64>::zeros((n, 3));
    let mut vel_out = ndarray::Array2::<f64>::zeros((n, 3));

    for (i, (p, v)) in itrs_pos.iter().zip(itrs_vel.iter()).enumerate() {
        pos_out[[i, 0]] = p.x;
        pos_out[[i, 1]] = p.y;
        pos_out[[i, 2]] = p.z;
        vel_out[[i, 0]] = v.x;
        vel_out[[i, 1]] = v.y;
        vel_out[[i, 2]] = v.z;
    }

    Ok((
        PyArray2::from_owned_array_bound(py, pos_out),
        PyArray2::from_owned_array_bound(py, vel_out),
    ))
}

/// Transform multiple state vectors from ITRS to TEME coordinates in batch
///
/// Converts International Terrestrial Reference System (Earth-fixed) coordinates
/// to True Equator Mean Equinox (TEME) coordinates for multiple epochs. This is
/// the inverse of batch_teme_to_itrs.
///
/// # Arguments
/// * `positions` - 2D NumPy array of shape (N, 3) containing ITRS position vectors in meters
/// * `velocities` - 2D NumPy array of shape (N, 3) containing ITRS velocity vectors in m/s
/// * `obstimes` - List of N Epoch objects for each state vector
///
/// # Returns
/// Tuple of (TEME positions, TEME velocities) as 2D NumPy arrays
///
/// # Errors
/// Returns an error if array shapes or epoch count are invalid
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora import batch_itrs_to_teme, Epoch
/// # ITRS coordinates (Earth-fixed)
/// positions = np.array([[7000e3, 0, 0], [0, 7000e3, 0]])
/// velocities = np.array([[0, 0.5e3, 0], [-0.5e3, 0, 0]])
/// epochs = [Epoch.from_mjd(60000.0), Epoch.from_mjd(60000.5)]
/// teme_pos, teme_vel = batch_itrs_to_teme(positions, velocities, epochs)
/// ```
#[pyfunction]
#[pyo3(name = "batch_itrs_to_teme")]
fn py_batch_itrs_to_teme<'py>(
    py: Python<'py>,
    positions: PyReadonlyArray2<f64>,
    velocities: PyReadonlyArray2<f64>,
    obstimes: Vec<core::time::Epoch>,
) -> PyResult<(Bound<'py, PyArray2<f64>>, Bound<'py, PyArray2<f64>>)> {
    let pos_array = positions.as_array();
    let vel_array = velocities.as_array();

    if pos_array.shape()[1] != 3 || vel_array.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position and velocity arrays must have shape (N, 3)"
        ));
    }

    if pos_array.shape()[0] != obstimes.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Number of positions must match number of epochs"
        ));
    }

    // Convert to Vec<Vector3>
    let pos_vec: Vec<nalgebra::Vector3<f64>> = (0..pos_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(pos_array[[i, 0]], pos_array[[i, 1]], pos_array[[i, 2]]))
        .collect();

    let vel_vec: Vec<nalgebra::Vector3<f64>> = (0..vel_array.shape()[0])
        .map(|i| nalgebra::Vector3::new(vel_array[[i, 0]], vel_array[[i, 1]], vel_array[[i, 2]]))
        .collect();

    // Call batch transformation
    let (teme_pos, teme_vel) = coordinates::frames::batch_itrs_to_teme(&pos_vec, &vel_vec, &obstimes)?;

    // Convert back to NumPy arrays
    let n = teme_pos.len();
    let mut pos_out = ndarray::Array2::<f64>::zeros((n, 3));
    let mut vel_out = ndarray::Array2::<f64>::zeros((n, 3));

    for (i, (p, v)) in teme_pos.iter().zip(teme_vel.iter()).enumerate() {
        pos_out[[i, 0]] = p.x;
        pos_out[[i, 1]] = p.y;
        pos_out[[i, 2]] = p.z;
        vel_out[[i, 0]] = v.x;
        vel_out[[i, 1]] = v.y;
        vel_out[[i, 2]] = v.z;
    }

    Ok((
        PyArray2::from_owned_array_bound(py, pos_out),
        PyArray2::from_owned_array_bound(py, vel_out),
    ))
}

// ============================================================================
// MANEUVER CALCULATIONS
// ============================================================================

/// Calculate Hohmann transfer parameters between two circular orbits
///
/// # Arguments
/// * `r_initial` - Initial orbit radius (m)
/// * `r_final` - Final orbit radius (m)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Dictionary with keys:
/// - r_initial: Initial orbit radius (m)
/// - r_final: Final orbit radius (m)
/// - mu: Gravitational parameter (m³/s²)
/// - delta_v1: First burn delta-v (m/s)
/// - delta_v2: Second burn delta-v (m/s)
/// - delta_v_total: Total delta-v (m/s)
/// - transfer_time: Transfer time (s)
/// - transfer_sma: Transfer orbit semi-major axis (m)
/// - transfer_eccentricity: Transfer orbit eccentricity
/// - v_initial: Initial circular orbit velocity (m/s)
/// - v_final: Final circular orbit velocity (m/s)
/// - v_transfer_periapsis: Velocity at periapsis of transfer orbit (m/s)
/// - v_transfer_apoapsis: Velocity at apoapsis of transfer orbit (m/s)
///
/// # Example
/// ```python
/// from astrora._core import hohmann_transfer
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
///
/// # LEO to GEO transfer
/// r_leo = R_MEAN_EARTH + 400e3  # 400 km altitude
/// r_geo = R_MEAN_EARTH + 35_786e3  # 35,786 km altitude
///
/// result = hohmann_transfer(r_leo, r_geo, GM_EARTH)
/// print(f"Total ΔV: {result['delta_v_total']/1000:.1f} km/s")
/// print(f"Transfer time: {result['transfer_time']/3600:.1f} hours")
/// ```
#[pyfunction]
#[pyo3(name = "hohmann_transfer")]
fn py_hohmann_transfer(
    py: Python<'_>,
    r_initial: f64,
    r_final: f64,
    mu: f64,
) -> PyResult<PyObject> {
    let result = maneuvers::HohmannTransfer::calculate(r_initial, r_final, mu)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("r_initial", result.r_initial)?;
    dict.set_item("r_final", result.r_final)?;
    dict.set_item("mu", result.mu)?;
    dict.set_item("delta_v1", result.delta_v1)?;
    dict.set_item("delta_v2", result.delta_v2)?;
    dict.set_item("delta_v_total", result.delta_v_total)?;
    dict.set_item("transfer_time", result.transfer_time)?;
    dict.set_item("transfer_sma", result.transfer_sma)?;
    dict.set_item("transfer_eccentricity", result.transfer_eccentricity)?;
    dict.set_item("v_initial", result.v_initial)?;
    dict.set_item("v_final", result.v_final)?;
    dict.set_item("v_transfer_periapsis", result.v_transfer_periapsis)?;
    dict.set_item("v_transfer_apoapsis", result.v_transfer_apoapsis)?;

    Ok(dict.into())
}

/// Calculate optimal phase angle for Hohmann transfer
///
/// For a successful rendezvous, the target must be at a specific angular
/// position when the transfer begins.
///
/// # Arguments
/// * `r_initial` - Initial orbit radius (m)
/// * `r_final` - Final orbit radius (m)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Phase angle in radians (0 to 2π). The target should be this angle ahead
/// of the chaser at the time of the first burn.
///
/// # Example
/// ```python
/// from astrora._core import hohmann_phase_angle
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
/// import math
///
/// r_leo = R_MEAN_EARTH + 400e3
/// r_geo = R_MEAN_EARTH + 35_786e3
///
/// phase_rad = hohmann_phase_angle(r_leo, r_geo, GM_EARTH)
/// phase_deg = phase_rad * 180 / math.pi
/// print(f"Required phase angle: {phase_deg:.1f}°")
/// ```
#[pyfunction]
#[pyo3(name = "hohmann_phase_angle")]
fn py_hohmann_phase_angle(
    r_initial: f64,
    r_final: f64,
    mu: f64,
) -> PyResult<f64> {
    maneuvers::HohmannTransfer::phase_angle(r_initial, r_final, mu)
        .map_err(|e| e.into())
}

/// Calculate synodic period between two orbits
///
/// The synodic period is the time between successive alignments of two
/// objects in different orbits. This determines how often transfer
/// opportunities occur.
///
/// # Arguments
/// * `r_initial` - Initial orbit radius (m)
/// * `r_final` - Final orbit radius (m)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Synodic period in seconds
///
/// # Example
/// ```python
/// from astrora._core import hohmann_synodic_period
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
///
/// r_leo = R_MEAN_EARTH + 400e3
/// r_geo = R_MEAN_EARTH + 35_786e3
///
/// synodic = hohmann_synodic_period(r_leo, r_geo, GM_EARTH)
/// print(f"Transfer opportunities every {synodic/3600:.1f} hours")
/// ```
#[pyfunction]
#[pyo3(name = "hohmann_synodic_period")]
fn py_hohmann_synodic_period(
    r_initial: f64,
    r_final: f64,
    mu: f64,
) -> PyResult<f64> {
    maneuvers::HohmannTransfer::synodic_period(r_initial, r_final, mu)
        .map_err(|e| e.into())
}

/// Calculate time until next optimal transfer window
///
/// Given the current phase angle between two objects, calculate how long
/// until the next optimal transfer opportunity.
///
/// # Arguments
/// * `current_phase` - Current phase angle (radians, 0 to 2π)
/// * `r_initial` - Initial orbit radius (m)
/// * `r_final` - Final orbit radius (m)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Time until next transfer window in seconds
///
/// # Example
/// ```python
/// from astrora._core import hohmann_time_to_window
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
/// import math
///
/// r_leo = R_MEAN_EARTH + 400e3
/// r_geo = R_MEAN_EARTH + 35_786e3
/// current_phase = math.radians(45)  # Current 45° separation
///
/// wait_time = hohmann_time_to_window(current_phase, r_leo, r_geo, GM_EARTH)
/// print(f"Wait {wait_time/3600:.1f} hours for next transfer window")
/// ```
#[pyfunction]
#[pyo3(name = "hohmann_time_to_window")]
fn py_hohmann_time_to_window(
    current_phase: f64,
    r_initial: f64,
    r_final: f64,
    mu: f64,
) -> PyResult<f64> {
    maneuvers::HohmannTransfer::time_to_transfer_window(
        current_phase,
        r_initial,
        r_final,
        mu,
    )
    .map_err(|e| e.into())
}

/// Calculate bi-elliptic transfer parameters between two circular orbits
///
/// A bi-elliptic transfer uses three impulses and can be more fuel-efficient than
/// a Hohmann transfer for large orbital radius ratios (typically > 15.58).
///
/// # Arguments
/// * `r_initial` - Initial orbit radius (m)
/// * `r_final` - Final orbit radius (m)
/// * `r_intermediate` - Intermediate apoapsis radius (m), must be > max(r_initial, r_final)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Dictionary containing:
/// - `r_initial`, `r_final`, `r_intermediate` (m)
/// - `mu` (m³/s²)
/// - `delta_v1`, `delta_v2`, `delta_v3` (m/s) - three burn delta-v values
/// - `delta_v_total` (m/s) - total delta-v required
/// - `transfer_time` (s) - total transfer time
/// - `transfer1_sma`, `transfer2_sma` (m) - transfer orbit semi-major axes
/// - `transfer1_eccentricity`, `transfer2_eccentricity` - transfer orbit eccentricities
/// - `v_initial`, `v_final` (m/s) - circular orbit velocities
/// - `v_transfer1_periapsis`, `v_transfer1_apoapsis` (m/s)
/// - `v_transfer2_apoapsis`, `v_transfer2_periapsis` (m/s)
///
/// # Example
/// ```python
/// from astrora._core import bielliptic_transfer
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
///
/// # LEO to very high orbit transfer
/// r_leo = R_MEAN_EARTH + 400e3
/// r_high = R_MEAN_EARTH + 200_000e3
/// r_intermediate = r_high * 3.0  # 3x final orbit
///
/// result = bielliptic_transfer(r_leo, r_high, r_intermediate, GM_EARTH)
/// print(f"Total ΔV: {result['delta_v_total']/1000:.1f} km/s")
/// print(f"Transfer time: {result['transfer_time']/86400:.1f} days")
/// ```
#[pyfunction]
#[pyo3(name = "bielliptic_transfer")]
fn py_bielliptic_transfer(
    py: Python<'_>,
    r_initial: f64,
    r_final: f64,
    r_intermediate: f64,
    mu: f64,
) -> PyResult<PyObject> {
    let result = maneuvers::BiellipticTransfer::calculate(r_initial, r_final, r_intermediate, mu)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("r_initial", result.r_initial)?;
    dict.set_item("r_final", result.r_final)?;
    dict.set_item("r_intermediate", result.r_intermediate)?;
    dict.set_item("mu", result.mu)?;
    dict.set_item("delta_v1", result.delta_v1)?;
    dict.set_item("delta_v2", result.delta_v2)?;
    dict.set_item("delta_v3", result.delta_v3)?;
    dict.set_item("delta_v_total", result.delta_v_total)?;
    dict.set_item("transfer_time", result.transfer_time)?;
    dict.set_item("transfer1_sma", result.transfer1_sma)?;
    dict.set_item("transfer2_sma", result.transfer2_sma)?;
    dict.set_item("transfer1_eccentricity", result.transfer1_eccentricity)?;
    dict.set_item("transfer2_eccentricity", result.transfer2_eccentricity)?;
    dict.set_item("v_initial", result.v_initial)?;
    dict.set_item("v_final", result.v_final)?;
    dict.set_item("v_transfer1_periapsis", result.v_transfer1_periapsis)?;
    dict.set_item("v_transfer1_apoapsis", result.v_transfer1_apoapsis)?;
    dict.set_item("v_transfer2_apoapsis", result.v_transfer2_apoapsis)?;
    dict.set_item("v_transfer2_periapsis", result.v_transfer2_periapsis)?;

    Ok(dict.into())
}

/// Compare bi-elliptic and Hohmann transfer efficiency
///
/// Calculates both transfer types and returns the delta-v savings and time penalty
/// of using a bi-elliptic transfer instead of a Hohmann transfer.
///
/// # Arguments
/// * `r_initial` - Initial orbit radius (m)
/// * `r_final` - Final orbit radius (m)
/// * `r_intermediate` - Intermediate apoapsis radius for bi-elliptic (m)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Dictionary containing:
/// - `bielliptic` - bi-elliptic transfer result dictionary
/// - `hohmann` - Hohmann transfer result dictionary
/// - `dv_savings` (m/s) - positive if bi-elliptic saves delta-v
/// - `time_penalty` - how many times longer bi-elliptic takes
///
/// # Example
/// ```python
/// from astrora._core import compare_bielliptic_hohmann
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
///
/// r_leo = R_MEAN_EARTH + 400e3
/// r_high = r_leo * 20.0  # Large radius ratio
/// r_intermediate = r_high * 5.0
///
/// result = compare_bielliptic_hohmann(r_leo, r_high, r_intermediate, GM_EARTH)
/// print(f"ΔV savings: {result['dv_savings']} m/s")
/// print(f"Time penalty: {result['time_penalty']:.1f}x")
/// ```
#[pyfunction]
#[pyo3(name = "compare_bielliptic_hohmann")]
fn py_compare_bielliptic_hohmann(
    py: Python<'_>,
    r_initial: f64,
    r_final: f64,
    r_intermediate: f64,
    mu: f64,
) -> PyResult<PyObject> {
    let (bielliptic, hohmann, dv_savings, time_penalty) =
        maneuvers::BiellipticTransfer::compare_with_hohmann(r_initial, r_final, r_intermediate, mu)?;

    // Create bi-elliptic result dictionary
    let bielliptic_dict = pyo3::types::PyDict::new_bound(py);
    bielliptic_dict.set_item("r_initial", bielliptic.r_initial)?;
    bielliptic_dict.set_item("r_final", bielliptic.r_final)?;
    bielliptic_dict.set_item("r_intermediate", bielliptic.r_intermediate)?;
    bielliptic_dict.set_item("delta_v1", bielliptic.delta_v1)?;
    bielliptic_dict.set_item("delta_v2", bielliptic.delta_v2)?;
    bielliptic_dict.set_item("delta_v3", bielliptic.delta_v3)?;
    bielliptic_dict.set_item("delta_v_total", bielliptic.delta_v_total)?;
    bielliptic_dict.set_item("transfer_time", bielliptic.transfer_time)?;

    // Create Hohmann result dictionary
    let hohmann_dict = pyo3::types::PyDict::new_bound(py);
    hohmann_dict.set_item("r_initial", hohmann.r_initial)?;
    hohmann_dict.set_item("r_final", hohmann.r_final)?;
    hohmann_dict.set_item("delta_v1", hohmann.delta_v1)?;
    hohmann_dict.set_item("delta_v2", hohmann.delta_v2)?;
    hohmann_dict.set_item("delta_v_total", hohmann.delta_v_total)?;
    hohmann_dict.set_item("transfer_time", hohmann.transfer_time)?;

    // Create result dictionary
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("bielliptic", bielliptic_dict)?;
    dict.set_item("hohmann", hohmann_dict)?;
    dict.set_item("dv_savings", dv_savings)?;
    dict.set_item("time_penalty", time_penalty)?;

    Ok(dict.into())
}

/// Find optimal intermediate radius for bi-elliptic transfer
///
/// Searches for the intermediate radius that minimizes total delta-v for a
/// bi-elliptic transfer, within a specified search range.
///
/// # Arguments
/// * `r_initial` - Initial orbit radius (m)
/// * `r_final` - Final orbit radius (m)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
/// * `search_limit_factor` - Maximum r_intermediate as multiple of max(r_initial, r_final)
///
/// # Returns
/// Tuple of (optimal_r_intermediate, transfer_result_dict)
///
/// # Example
/// ```python
/// from astrora._core import find_optimal_bielliptic
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
///
/// r_leo = R_MEAN_EARTH + 400e3
/// r_high = r_leo * 25.0
///
/// r_opt, result = find_optimal_bielliptic(r_leo, r_high, GM_EARTH, 100.0)
/// print(f"Optimal intermediate radius: {r_opt/1e6:.0f} Mm")
/// print(f"Minimum ΔV: {result['delta_v_total']/1000:.2f} km/s")
/// ```
#[pyfunction]
#[pyo3(name = "find_optimal_bielliptic")]
fn py_find_optimal_bielliptic(
    py: Python<'_>,
    r_initial: f64,
    r_final: f64,
    mu: f64,
    search_limit_factor: f64,
) -> PyResult<PyObject> {
    let (r_opt, result) = maneuvers::BiellipticTransfer::find_optimal_intermediate(
        r_initial,
        r_final,
        mu,
        search_limit_factor,
    )?;

    // Create result dictionary
    let result_dict = pyo3::types::PyDict::new_bound(py);
    result_dict.set_item("r_initial", result.r_initial)?;
    result_dict.set_item("r_final", result.r_final)?;
    result_dict.set_item("r_intermediate", result.r_intermediate)?;
    result_dict.set_item("delta_v1", result.delta_v1)?;
    result_dict.set_item("delta_v2", result.delta_v2)?;
    result_dict.set_item("delta_v3", result.delta_v3)?;
    result_dict.set_item("delta_v_total", result.delta_v_total)?;
    result_dict.set_item("transfer_time", result.transfer_time)?;

    // Create tuple (r_opt, result_dict)
    let tuple = pyo3::types::PyTuple::new_bound(py, &[r_opt.to_object(py), result_dict.into()]);

    Ok(tuple.into())
}

/// Calculate a pure plane change maneuver
///
/// Computes the delta-v required to change the orbital plane by a given angle
/// without changing the orbit size or shape. This is the most basic plane
/// change maneuver.
///
/// # Arguments
/// * `velocity` - Current orbital velocity (m/s)
/// * `delta_angle` - Dihedral angle to change (radians)
///
/// # Returns
/// Dictionary with maneuver parameters:
/// - velocity: Orbital velocity (m/s)
/// - delta_angle: Plane change angle (radians)
/// - delta_v: Required delta-v (m/s)
///
/// # Example
/// ```python
/// from astrora._core import pure_plane_change
/// import math
///
/// # LEO satellite at 7.8 km/s changing inclination by 5°
/// result = pure_plane_change(7800.0, math.radians(5.0))
/// print(f"ΔV required: {result['delta_v']:.1f} m/s")
/// ```
#[pyfunction]
#[pyo3(name = "pure_plane_change")]
fn py_pure_plane_change(
    py: Python<'_>,
    velocity: f64,
    delta_angle: f64,
) -> PyResult<PyObject> {
    let result = maneuvers::PlaneChange::pure_plane_change(velocity, delta_angle)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("velocity", result.velocity)?;
    dict.set_item("delta_angle", result.delta_angle)?;
    dict.set_item("delta_v", result.delta_v)?;

    Ok(dict.into())
}

/// Calculate a combined plane change with orbit change
///
/// Computes the delta-v for a maneuver that simultaneously changes both
/// the orbital plane and the orbit velocity. This is typically more
/// efficient than performing separate maneuvers.
///
/// Uses the simplified formula valid at apoapsis (or when radial velocity is zero):
/// Δv = √(v1² + v2² - 2v1·v2·cos(δ))
///
/// # Arguments
/// * `v_initial` - Initial orbital velocity (m/s)
/// * `v_final` - Final orbital velocity (m/s)
/// * `delta_angle` - Plane change angle (radians)
///
/// # Returns
/// Dictionary with all maneuver parameters:
/// - v_initial, v_final: Velocities (m/s)
/// - delta_angle: Plane change angle (radians)
/// - delta_v: Total required delta-v (m/s)
/// - delta_v_orbit_only: ΔV for orbit change alone (m/s)
/// - delta_v_plane_only: ΔV for plane change alone (m/s)
/// - delta_v_penalty: Extra cost for plane change (m/s)
///
/// # Example
/// ```python
/// from astrora._core import combined_plane_change
/// import math
///
/// # Combined Hohmann transfer + 5° plane change
/// result = combined_plane_change(7800.0, 10000.0, math.radians(5.0))
/// print(f"Total ΔV: {result['delta_v']:.1f} m/s")
/// print(f"Plane change penalty: {result['delta_v_penalty']:.1f} m/s")
/// ```
#[pyfunction]
#[pyo3(name = "combined_plane_change")]
fn py_combined_plane_change(
    py: Python<'_>,
    v_initial: f64,
    v_final: f64,
    delta_angle: f64,
) -> PyResult<PyObject> {
    let result =
        maneuvers::PlaneChange::combined_plane_change(v_initial, v_final, delta_angle)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("v_initial", result.v_initial)?;
    dict.set_item("v_final", result.v_final)?;
    dict.set_item("delta_angle", result.delta_angle)?;
    dict.set_item("delta_v", result.delta_v)?;
    dict.set_item("delta_v_orbit_only", result.delta_v_orbit_only)?;
    dict.set_item("delta_v_plane_only", result.delta_v_plane_only)?;
    dict.set_item("delta_v_penalty", result.delta_v_penalty)?;

    Ok(dict.into())
}

/// Calculate optimal plane change split for a transfer
///
/// For transfers between circular orbits with different inclinations,
/// determines the optimal way to split the plane change between the
/// two burn points. This typically results in a small plane change at
/// the low-altitude burn and the remainder at the high-altitude burn.
///
/// # Arguments
/// * `v_low` - Velocity at lower altitude (m/s)
/// * `v_high` - Velocity at higher altitude (m/s)
/// * `v_transfer_low` - Transfer orbit velocity at low altitude (m/s)
/// * `v_transfer_high` - Transfer orbit velocity at high altitude (m/s)
/// * `total_angle` - Total plane change required (radians)
///
/// # Returns
/// Dictionary with optimal split and delta-v breakdown:
/// - total_angle: Total plane change (radians)
/// - angle_at_first: Plane change at low altitude (radians)
/// - angle_at_second: Plane change at high altitude (radians)
/// - v_first, v_second: Velocities (m/s)
/// - delta_v_total: Total delta-v (m/s)
/// - delta_v_first, delta_v_second: Individual burns (m/s)
/// - delta_v_saved: Savings vs. all at high altitude (m/s)
/// - delta_v_saved_vs_low: Savings vs. all at low altitude (m/s)
///
/// # Example
/// ```python
/// from astrora._core import optimal_plane_change_location
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
/// import math
///
/// # LEO to GEO transfer with 28.5° plane change
/// r_leo = R_MEAN_EARTH + 400e3
/// r_geo = 42164e3
/// v_leo = (GM_EARTH / r_leo) ** 0.5
/// v_geo = (GM_EARTH / r_geo) ** 0.5
///
/// # Calculate transfer velocities
/// a_transfer = (r_leo + r_geo) / 2
/// v_transfer_leo = ((2 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer)) ** 0.5
/// v_transfer_geo = ((2 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer)) ** 0.5
///
/// result = optimal_plane_change_location(
///     v_leo, v_geo, v_transfer_leo, v_transfer_geo, math.radians(28.5)
/// )
/// print(f"Split: {math.degrees(result['angle_at_first']):.1f}° at LEO")
/// print(f"       {math.degrees(result['angle_at_second']):.1f}° at GEO")
/// print(f"ΔV saved: {result['delta_v_saved']:.1f} m/s")
/// ```
#[pyfunction]
#[pyo3(name = "optimal_plane_change_location")]
fn py_optimal_plane_change_location(
    py: Python<'_>,
    v_low: f64,
    v_high: f64,
    v_transfer_low: f64,
    v_transfer_high: f64,
    total_angle: f64,
) -> PyResult<PyObject> {
    let result = maneuvers::PlaneChange::optimal_plane_change_location(
        v_low,
        v_high,
        v_transfer_low,
        v_transfer_high,
        total_angle,
    )?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("total_angle", result.total_angle)?;
    dict.set_item("angle_at_first", result.angle_at_first)?;
    dict.set_item("angle_at_second", result.angle_at_second)?;
    dict.set_item("v_first", result.v_first)?;
    dict.set_item("v_second", result.v_second)?;
    dict.set_item("delta_v_total", result.delta_v_total)?;
    dict.set_item("delta_v_first", result.delta_v_first)?;
    dict.set_item("delta_v_second", result.delta_v_second)?;
    dict.set_item("delta_v_saved", result.delta_v_saved)?;
    dict.set_item("delta_v_saved_vs_low", result.delta_v_saved_vs_low)?;

    Ok(dict.into())
}

/// Calculate phasing orbit maneuver for orbital rendezvous
///
/// Computes the parameters for a phasing orbit that changes the spacecraft's
/// position by a specified phase angle. The spacecraft temporarily enters
/// a different orbit and returns to the original orbit at a different position.
///
/// # Arguments
/// * `r_original` - Original circular orbit radius (m)
/// * `phase_change` - Desired angular change (radians), positive = catch up, negative = wait
/// * `num_orbits` - Number of phasing orbits to complete (must be ≥ 1)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Dictionary with phasing orbit parameters:
/// - `r_original` (m) - Original orbit radius
/// - `a_phasing` (m) - Phasing orbit semi-major axis
/// - `r_phasing_periapsis` (m) - Phasing orbit periapsis radius
/// - `r_phasing_apoapsis` (m) - Phasing orbit apoapsis radius
/// - `e_phasing` - Phasing orbit eccentricity
/// - `v_original` (m/s) - Original orbit velocity
/// - `v_phasing_periapsis` (m/s) - Velocity at phasing orbit periapsis
/// - `v_phasing_apoapsis` (m/s) - Velocity at phasing orbit apoapsis
/// - `delta_v_enter` (m/s) - Delta-v for first burn (enter phasing orbit)
/// - `delta_v_exit` (m/s) - Delta-v for second burn (exit phasing orbit)
/// - `delta_v_total` (m/s) - Total delta-v required
/// - `period_original` (s) - Original orbit period
/// - `period_phasing` (s) - Phasing orbit period
/// - `num_phasing_orbits` - Number of phasing orbits
/// - `phasing_time` (s) - Total phasing time
/// - `phase_change_per_orbit` (rad) - Angular change per phasing orbit
/// - `total_phase_change` (rad) - Total angular change achieved
///
/// # Example
/// ```python
/// from astrora._core import phasing_orbit
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
/// import math
///
/// # ISS orbit, catch up by 30 degrees in 5 orbits
/// r_iss = R_MEAN_EARTH + 400e3
/// phase_change = math.radians(30)
/// num_orbits = 5
///
/// result = phasing_orbit(r_iss, phase_change, num_orbits, GM_EARTH)
/// print(f"Total ΔV: {result['delta_v_total']:.1f} m/s")
/// print(f"Phasing time: {result['phasing_time']/3600:.1f} hours")
/// ```
#[pyfunction]
#[pyo3(name = "phasing_orbit")]
fn py_phasing_orbit(
    py: Python<'_>,
    r_original: f64,
    phase_change: f64,
    num_orbits: f64,
    mu: f64,
) -> PyResult<PyObject> {
    let result = maneuvers::Rendezvous::phasing_orbit(r_original, phase_change, num_orbits, mu)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("r_original", result.r_original)?;
    dict.set_item("a_phasing", result.a_phasing)?;
    dict.set_item("r_phasing_periapsis", result.r_phasing_periapsis)?;
    dict.set_item("r_phasing_apoapsis", result.r_phasing_apoapsis)?;
    dict.set_item("e_phasing", result.e_phasing)?;
    dict.set_item("mu", result.mu)?;
    dict.set_item("v_original", result.v_original)?;
    dict.set_item("v_phasing_periapsis", result.v_phasing_periapsis)?;
    dict.set_item("v_phasing_apoapsis", result.v_phasing_apoapsis)?;
    dict.set_item("delta_v_enter", result.delta_v_enter)?;
    dict.set_item("delta_v_exit", result.delta_v_exit)?;
    dict.set_item("delta_v_total", result.delta_v_total)?;
    dict.set_item("period_original", result.period_original)?;
    dict.set_item("period_phasing", result.period_phasing)?;
    dict.set_item("num_phasing_orbits", result.num_phasing_orbits)?;
    dict.set_item("phasing_time", result.phasing_time)?;
    dict.set_item("phase_change_per_orbit", result.phase_change_per_orbit)?;
    dict.set_item("total_phase_change", result.total_phase_change)?;

    Ok(dict.into())
}

/// Calculate coorbital rendezvous (same orbit, different positions)
///
/// For two spacecraft in the same circular orbit but at different angular positions.
/// Automatically determines the optimal phasing orbit strategy.
///
/// # Arguments
/// * `r_orbit` - Common orbital radius (m)
/// * `phase_difference` - Angular separation (radians, 0 to 2π), positive = target ahead
/// * `num_phasing_orbits` - Number of phasing orbits (≥ 1), more orbits = less delta-v but more time
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Dictionary with rendezvous parameters:
/// - `r_orbit` (m) - Orbital radius
/// - `initial_phase_difference` (rad) - Initial angular separation
/// - `chaser_ahead` (bool) - Whether chaser is ahead of target
/// - `phasing` (dict) - Nested dictionary with all phasing orbit parameters (same as phasing_orbit function)
///
/// # Example
/// ```python
/// from astrora._core import coorbital_rendezvous
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
/// import math
///
/// # Two spacecraft at 400 km, 45° apart
/// r = R_MEAN_EARTH + 400e3
/// phase_diff = math.radians(45)
///
/// result = coorbital_rendezvous(r, phase_diff, 10, GM_EARTH)
/// print(f"Total ΔV: {result['phasing']['delta_v_total']:.1f} m/s")
/// print(f"Time: {result['phasing']['phasing_time']/3600:.1f} hours")
/// ```
#[pyfunction]
#[pyo3(name = "coorbital_rendezvous")]
fn py_coorbital_rendezvous(
    py: Python<'_>,
    r_orbit: f64,
    phase_difference: f64,
    num_phasing_orbits: f64,
    mu: f64,
) -> PyResult<PyObject> {
    let result = maneuvers::Rendezvous::coorbital_rendezvous(
        r_orbit,
        phase_difference,
        num_phasing_orbits,
        mu,
    )?;

    // Create phasing orbit dictionary
    let phasing_dict = pyo3::types::PyDict::new_bound(py);
    phasing_dict.set_item("r_original", result.phasing.r_original)?;
    phasing_dict.set_item("a_phasing", result.phasing.a_phasing)?;
    phasing_dict.set_item("r_phasing_periapsis", result.phasing.r_phasing_periapsis)?;
    phasing_dict.set_item("r_phasing_apoapsis", result.phasing.r_phasing_apoapsis)?;
    phasing_dict.set_item("e_phasing", result.phasing.e_phasing)?;
    phasing_dict.set_item("mu", result.phasing.mu)?;
    phasing_dict.set_item("v_original", result.phasing.v_original)?;
    phasing_dict.set_item("v_phasing_periapsis", result.phasing.v_phasing_periapsis)?;
    phasing_dict.set_item("v_phasing_apoapsis", result.phasing.v_phasing_apoapsis)?;
    phasing_dict.set_item("delta_v_enter", result.phasing.delta_v_enter)?;
    phasing_dict.set_item("delta_v_exit", result.phasing.delta_v_exit)?;
    phasing_dict.set_item("delta_v_total", result.phasing.delta_v_total)?;
    phasing_dict.set_item("period_original", result.phasing.period_original)?;
    phasing_dict.set_item("period_phasing", result.phasing.period_phasing)?;
    phasing_dict.set_item("num_phasing_orbits", result.phasing.num_phasing_orbits)?;
    phasing_dict.set_item("phasing_time", result.phasing.phasing_time)?;
    phasing_dict.set_item("phase_change_per_orbit", result.phasing.phase_change_per_orbit)?;
    phasing_dict.set_item("total_phase_change", result.phasing.total_phase_change)?;

    // Create main result dictionary
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("r_orbit", result.r_orbit)?;
    dict.set_item("mu", result.mu)?;
    dict.set_item("initial_phase_difference", result.initial_phase_difference)?;
    dict.set_item("chaser_ahead", result.chaser_ahead)?;
    dict.set_item("phasing", phasing_dict)?;

    Ok(dict.into())
}

/// Calculate coplanar rendezvous (different orbits, same plane)
///
/// For spacecraft in different circular orbits in the same plane. Uses a
/// Hohmann-like transfer and calculates the required phase angle and wait time.
///
/// # Arguments
/// * `r_chaser` - Chaser orbit radius (m)
/// * `r_target` - Target orbit radius (m)
/// * `current_phase` - Current angular separation (radians, 0 to 2π), positive = target ahead
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
///
/// # Returns
/// Dictionary with transfer parameters:
/// - `r_chaser` (m) - Chaser orbit radius
/// - `r_target` (m) - Target orbit radius
/// - `a_transfer` (m) - Transfer orbit semi-major axis
/// - `e_transfer` - Transfer orbit eccentricity
/// - `transfer_time` (s) - Time for transfer
/// - `lead_angle` (rad) - How far target travels during transfer
/// - `required_phase_angle` (rad) - Required phase angle at transfer start
/// - `current_phase_angle` (rad) - Current phase angle
/// - `wait_time` (s) - Time to wait until proper alignment
/// - `wait_orbits` - Number of chaser orbits to wait
/// - `delta_v1` (m/s) - First burn delta-v
/// - `delta_v2` (m/s) - Second burn delta-v
/// - `delta_v_total` (m/s) - Total delta-v required
///
/// # Example
/// ```python
/// from astrora._core import coplanar_rendezvous
/// from astrora._core.constants import GM_EARTH, R_MEAN_EARTH
/// import math
///
/// # Transfer from LEO to ISS
/// r_leo = R_MEAN_EARTH + 300e3
/// r_iss = R_MEAN_EARTH + 400e3
/// current_phase = math.radians(45)
///
/// result = coplanar_rendezvous(r_leo, r_iss, current_phase, GM_EARTH)
/// print(f"Wait time: {result['wait_time']/3600:.1f} hours")
/// print(f"Transfer ΔV: {result['delta_v_total']:.1f} m/s")
/// ```
#[pyfunction]
#[pyo3(name = "coplanar_rendezvous")]
fn py_coplanar_rendezvous(
    py: Python<'_>,
    r_chaser: f64,
    r_target: f64,
    current_phase: f64,
    mu: f64,
) -> PyResult<PyObject> {
    let result =
        maneuvers::Rendezvous::coplanar_rendezvous(r_chaser, r_target, current_phase, mu)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("r_chaser", result.r_chaser)?;
    dict.set_item("r_target", result.r_target)?;
    dict.set_item("mu", result.mu)?;
    dict.set_item("a_transfer", result.a_transfer)?;
    dict.set_item("e_transfer", result.e_transfer)?;
    dict.set_item("transfer_time", result.transfer_time)?;
    dict.set_item("lead_angle", result.lead_angle)?;
    dict.set_item("required_phase_angle", result.required_phase_angle)?;
    dict.set_item("current_phase_angle", result.current_phase_angle)?;
    dict.set_item("wait_time", result.wait_time)?;
    dict.set_item("wait_orbits", result.wait_orbits)?;
    dict.set_item("delta_v1", result.delta_v1)?;
    dict.set_item("delta_v2", result.delta_v2)?;
    dict.set_item("delta_v_total", result.delta_v_total)?;

    Ok(dict.into())
}

/// Create a delta-v budget for mission planning
///
/// Creates a budget tracking system to accumulate all delta-v maneuvers required
/// for a space mission. Automatically calculates total delta-v, applies contingency
/// margins, and computes propellant requirements using the Tsiolkovsky rocket equation.
///
/// # Arguments
/// * `mission_name` - Name or identifier for the mission
/// * `maneuvers` - List of (name, delta_v) tuples for each maneuver (m/s)
/// * `contingency_margin` - Optional margin as fraction (e.g., 0.1 for 10%)
/// * `dry_mass` - Optional spacecraft dry mass in kg (for propellant calculation)
/// * `specific_impulse` - Optional engine specific impulse in seconds (for propellant calculation)
///
/// # Returns
/// Dictionary with budget summary:
/// - `mission_name` - Mission identifier
/// - `total_delta_v` (m/s) - Sum of all maneuvers
/// - `contingency_margin` - Applied contingency fraction
/// - `total_with_contingency` (m/s) - Total including margin
/// - `num_maneuvers` - Number of maneuvers in budget
/// - `dry_mass` (kg) - Spacecraft dry mass (if provided)
/// - `specific_impulse` (s) - Engine Isp (if provided)
/// - `propellant_mass` (kg) - Required propellant (if mass and Isp provided)
/// - `propellant_fraction` - Propellant mass fraction (if mass and Isp provided)
/// - `total_mass` (kg) - Initial spacecraft mass (if mass and Isp provided)
///
/// # Example
/// ```python
/// from astrora._core import delta_v_budget
///
/// # Create a LEO to GEO mission budget
/// maneuvers = [
///     ("Hohmann transfer burn 1", 2440.0),
///     ("Hohmann transfer burn 2", 1475.0),
///     ("Station keeping (5 years)", 250.0),
/// ]
///
/// budget = delta_v_budget(
///     mission_name="LEO to GEO",
///     maneuvers=maneuvers,
///     contingency_margin=0.1,  # 10% margin
///     dry_mass=1000.0,  # kg
///     specific_impulse=300.0  # seconds
/// )
///
/// print(f"Total ΔV: {budget['total_delta_v']/1000:.2f} km/s")
/// print(f"With margin: {budget['total_with_contingency']/1000:.2f} km/s")
/// print(f"Propellant: {budget['propellant_mass']:.1f} kg")
/// print(f"Mass fraction: {budget['propellant_fraction']:.1%}")
/// ```
#[pyfunction]
#[pyo3(
    name = "delta_v_budget",
    signature = (mission_name, maneuvers, contingency_margin=0.0, dry_mass=None, specific_impulse=None)
)]
fn py_delta_v_budget(
    py: Python<'_>,
    mission_name: String,
    maneuvers: Vec<(String, f64)>,
    contingency_margin: f64,
    dry_mass: Option<f64>,
    specific_impulse: Option<f64>,
) -> PyResult<PyObject> {
    // Create budget
    let mut budget = if contingency_margin > 0.0 {
        maneuvers::DeltaVBudget::with_contingency(&mission_name, contingency_margin)?
    } else {
        maneuvers::DeltaVBudget::new(&mission_name)
    };

    // Add all maneuvers
    for (name, delta_v) in maneuvers {
        budget.add(&name, delta_v)?;
    }

    // Generate result
    let result = budget.result(dry_mass, specific_impulse)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("mission_name", result.mission_name)?;
    dict.set_item("total_delta_v", result.total_delta_v)?;
    dict.set_item("contingency_margin", result.contingency_margin)?;
    dict.set_item("total_with_contingency", result.total_with_contingency)?;
    dict.set_item("num_maneuvers", result.num_maneuvers)?;

    // Optional fields (only set if present)
    if let Some(dm) = result.dry_mass {
        dict.set_item("dry_mass", dm)?;
    }
    if let Some(isp) = result.specific_impulse {
        dict.set_item("specific_impulse", isp)?;
    }
    if let Some(pm) = result.propellant_mass {
        dict.set_item("propellant_mass", pm)?;
    }
    if let Some(pf) = result.propellant_fraction {
        dict.set_item("propellant_fraction", pf)?;
    }
    if let Some(tm) = result.total_mass {
        dict.set_item("total_mass", tm)?;
    }

    Ok(dict.into())
}

// ============================================================================
// GRAVITY ASSIST CALCULATIONS
// ============================================================================

/// Calculate gravity assist (planetary flyby) parameters
///
/// # Arguments
/// * `v_infinity` - Hyperbolic excess velocity magnitude |v∞| (m/s)
/// * `r_periapsis` - Periapsis radius / closest approach distance (m)
/// * `mu` - Gravitational parameter μ = GM of flyby body (m³/s²)
///
/// # Returns
/// Dictionary with keys:
/// - v_infinity: Hyperbolic excess velocity (m/s)
/// - r_periapsis: Closest approach distance (m)
/// - mu: Gravitational parameter (m³/s²)
/// - eccentricity: Hyperbolic trajectory eccentricity (>1)
/// - delta: Turning angle / deflection angle (radians)
/// - delta_v_magnitude: Delta-v magnitude in heliocentric frame (m/s)
/// - semi_major_axis: Semi-major axis (m, negative for hyperbolic)
/// - theta_infinity: Asymptote angle (radians)
/// - b_parameter: Impact parameter / B-vector magnitude (m)
/// - specific_energy: Specific orbital energy (m²/s²)
///
/// # Example
/// ```python
/// from astrora._core import gravity_assist
/// from astrora._core.constants import GM_JUPITER
///
/// # Jupiter flyby with v∞ = 5.64 km/s at 3 Jupiter radii
/// r_jupiter = 71492e3  # m
/// result = gravity_assist(5640.0, 3.0 * r_jupiter, GM_JUPITER)
/// print(f"Deflection angle: {result['delta']*180/3.14159:.1f}°")
/// print(f"Delta-v: {result['delta_v_magnitude']/1000:.2f} km/s")
/// ```
#[pyfunction]
#[pyo3(name = "gravity_assist")]
fn py_gravity_assist(
    py: Python<'_>,
    v_infinity: f64,
    r_periapsis: f64,
    mu: f64,
) -> PyResult<PyObject> {
    let result = maneuvers::GravityAssist::from_periapsis(v_infinity, r_periapsis, mu)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("v_infinity", result.v_infinity)?;
    dict.set_item("r_periapsis", result.r_periapsis)?;
    dict.set_item("mu", result.mu)?;
    dict.set_item("eccentricity", result.eccentricity)?;
    dict.set_item("delta", result.delta)?;
    dict.set_item("delta_v_magnitude", result.delta_v_magnitude)?;
    dict.set_item("semi_major_axis", result.semi_major_axis)?;
    dict.set_item("theta_infinity", result.theta_infinity)?;
    dict.set_item("b_parameter", result.b_parameter)?;
    dict.set_item("specific_energy", result.specific_energy)?;

    Ok(dict.into())
}

/// Calculate periapsis radius from B-plane impact parameter
///
/// # Arguments
/// * `v_infinity` - Hyperbolic excess velocity magnitude |v∞| (m/s)
/// * `b_parameter` - Impact parameter / B-vector magnitude (m)
/// * `mu` - Gravitational parameter μ = GM of flyby body (m³/s²)
///
/// # Returns
/// Periapsis radius (m)
///
/// # Example
/// ```python
/// from astrora._core import periapsis_from_b_parameter
/// from astrora._core.constants import GM_JUPITER
///
/// # Calculate periapsis for a given impact parameter
/// v_inf = 5640.0  # m/s
/// b = 500000e3   # 500,000 km impact parameter
/// r_p = periapsis_from_b_parameter(v_inf, b, GM_JUPITER)
/// print(f"Periapsis: {r_p/1000:.0f} km")
/// ```
#[pyfunction]
#[pyo3(name = "periapsis_from_b_parameter")]
fn py_periapsis_from_b_parameter(
    v_infinity: f64,
    b_parameter: f64,
    mu: f64,
) -> f64 {
    maneuvers::GravityAssist::periapsis_from_b_parameter(v_infinity, b_parameter, mu)
}

/// Solve Lambert's problem for orbital boundary value problem
///
/// Given two position vectors and time of flight, compute the initial and final
/// velocity vectors for the connecting orbit. This is fundamental for:
/// - Interplanetary mission design
/// - Orbital rendezvous planning
/// - Porkchop plots (launch window analysis)
/// - Trajectory optimization
///
/// # Arguments
/// * `r1` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `r2` - Final position vector [x, y, z] in meters (NumPy array)
/// * `tof` - Time of flight in seconds
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
/// * `short_way` - If True, use short-way transfer (<180°); if False, use long-way (>180°)
/// * `revs` - Number of complete revolutions (0 for direct transfer)
///
/// # Returns
/// Dictionary with keys:
/// - r1: Initial position vector (m)
/// - r2: Final position vector (m)
/// - tof: Time of flight (s)
/// - v1: Initial velocity vector (m/s)
/// - v2: Final velocity vector (m/s)
/// - mu: Gravitational parameter (m³/s²)
/// - a: Semi-major axis of transfer orbit (m)
/// - e: Eccentricity of transfer orbit
/// - revs: Number of revolutions
/// - short_way: True for short-way transfer
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora._core import lambert_solve
/// from astrora._core.constants import GM_EARTH
///
/// # Transfer from LEO to GEO-like orbit
/// r1 = np.array([7000e3, 0.0, 0.0])
/// r2 = np.array([0.0, 42000e3, 0.0])
/// tof = 19000.0  # ~5.3 hours
///
/// solution = lambert_solve(r1, r2, tof, GM_EARTH, short_way=True, revs=0)
/// print(f"Initial velocity: {solution['v1']/1000} km/s")
/// print(f"Final velocity: {solution['v2']/1000} km/s")
/// print(f"Transfer orbit a: {solution['a']/1000:.0f} km")
/// print(f"Transfer orbit e: {solution['e']:.4f}")
/// ```
#[pyfunction]
#[pyo3(name = "lambert_solve", signature = (r1, r2, tof, mu, short_way=true, revs=0))]
fn py_lambert_solve<'py>(
    py: Python<'py>,
    r1: PyReadonlyArray1<f64>,
    r2: PyReadonlyArray1<f64>,
    tof: f64,
    mu: f64,
    short_way: bool,
    revs: u32,
) -> PyResult<PyObject> {
    let r1_array = r1.as_array();
    let r2_array = r2.as_array();

    if r1_array.len() != 3 || r2_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vectors must have exactly 3 components",
        ));
    }

    let r1_vec = nalgebra::Vector3::new(r1_array[0], r1_array[1], r1_array[2]);
    let r2_vec = nalgebra::Vector3::new(r2_array[0], r2_array[1], r2_array[2]);

    let transfer_kind = if short_way {
        maneuvers::TransferKind::ShortWay
    } else {
        maneuvers::TransferKind::LongWay
    };

    let solution = maneuvers::Lambert::solve(r1_vec, r2_vec, tof, mu, transfer_kind, revs)?;

    // Create Python dictionary with results
    let dict = pyo3::types::PyDict::new_bound(py);

    // Convert vectors to NumPy arrays
    let r1_out = ndarray::arr1(&[solution.r1[0], solution.r1[1], solution.r1[2]]);
    let r2_out = ndarray::arr1(&[solution.r2[0], solution.r2[1], solution.r2[2]]);
    let v1_out = ndarray::arr1(&[solution.v1[0], solution.v1[1], solution.v1[2]]);
    let v2_out = ndarray::arr1(&[solution.v2[0], solution.v2[1], solution.v2[2]]);

    dict.set_item("r1", PyArray1::from_owned_array_bound(py, r1_out))?;
    dict.set_item("r2", PyArray1::from_owned_array_bound(py, r2_out))?;
    dict.set_item("tof", solution.tof)?;
    dict.set_item("v1", PyArray1::from_owned_array_bound(py, v1_out))?;
    dict.set_item("v2", PyArray1::from_owned_array_bound(py, v2_out))?;
    dict.set_item("mu", solution.mu)?;
    dict.set_item("a", solution.a)?;
    dict.set_item("e", solution.e)?;
    dict.set_item("revs", solution.revs)?;
    dict.set_item("short_way", solution.short_way)?;

    Ok(dict.into())
}

/// Solve Lambert's problem for multiple time-of-flight values (batch operation)
///
/// This is optimized for porkchop plot generation where thousands of Lambert
/// solutions need to be computed. Processing in a batch minimizes Python-Rust
/// boundary overhead.
///
/// # Arguments
/// * `r1` - Initial position vector [x, y, z] in meters (NumPy array)
/// * `r2` - Final position vector [x, y, z] in meters (NumPy array)
/// * `tofs` - Array of time of flight values in seconds (NumPy array)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
/// * `short_way` - If True, use short-way transfer (<180°); if False, use long-way (>180°)
/// * `revs` - Number of complete revolutions (0 for direct transfer)
///
/// # Returns
/// List of dictionaries, one for each time of flight, with same structure as lambert_solve()
///
/// # Performance
/// ~10-20x faster than calling lambert_solve() in a Python loop
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora._core import lambert_solve_batch
/// from astrora._core.constants import GM_EARTH
///
/// r1 = np.array([7000e3, 0.0, 0.0])
/// r2 = np.array([0.0, 42000e3, 0.0])
/// tofs = np.linspace(10000.0, 30000.0, 100)  # 100 different times
///
/// solutions = lambert_solve_batch(r1, r2, tofs, GM_EARTH)
/// # Returns list of 100 solutions
/// ```
#[pyfunction]
#[pyo3(name = "lambert_solve_batch", signature = (r1, r2, tofs, mu, short_way=true, revs=0))]
fn py_lambert_solve_batch<'py>(
    py: Python<'py>,
    r1: PyReadonlyArray1<f64>,
    r2: PyReadonlyArray1<f64>,
    tofs: PyReadonlyArray1<f64>,
    mu: f64,
    short_way: bool,
    revs: u32,
) -> PyResult<Vec<PyObject>> {
    let r1_array = r1.as_array();
    let r2_array = r2.as_array();
    let tofs_array = tofs.as_array();

    if r1_array.len() != 3 || r2_array.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vectors must have exactly 3 components",
        ));
    }

    let r1_vec = nalgebra::Vector3::new(r1_array[0], r1_array[1], r1_array[2]);
    let r2_vec = nalgebra::Vector3::new(r2_array[0], r2_array[1], r2_array[2]);

    let transfer_kind = if short_way {
        maneuvers::TransferKind::ShortWay
    } else {
        maneuvers::TransferKind::LongWay
    };

    let tofs_vec: Vec<f64> = tofs_array.to_vec();
    let solutions = maneuvers::Lambert::solve_batch(r1_vec, r2_vec, &tofs_vec, mu, transfer_kind, revs)?;

    // Convert each solution to a Python dictionary
    solutions
        .into_iter()
        .map(|solution| {
            let dict = pyo3::types::PyDict::new_bound(py);

            let r1_out = ndarray::arr1(&[solution.r1[0], solution.r1[1], solution.r1[2]]);
            let r2_out = ndarray::arr1(&[solution.r2[0], solution.r2[1], solution.r2[2]]);
            let v1_out = ndarray::arr1(&[solution.v1[0], solution.v1[1], solution.v1[2]]);
            let v2_out = ndarray::arr1(&[solution.v2[0], solution.v2[1], solution.v2[2]]);

            dict.set_item("r1", PyArray1::from_owned_array_bound(py, r1_out))?;
            dict.set_item("r2", PyArray1::from_owned_array_bound(py, r2_out))?;
            dict.set_item("tof", solution.tof)?;
            dict.set_item("v1", PyArray1::from_owned_array_bound(py, v1_out))?;
            dict.set_item("v2", PyArray1::from_owned_array_bound(py, v2_out))?;
            dict.set_item("mu", solution.mu)?;
            dict.set_item("a", solution.a)?;
            dict.set_item("e", solution.e)?;
            dict.set_item("revs", solution.revs)?;
            dict.set_item("short_way", solution.short_way)?;

            Ok(dict.into())
        })
        .collect()
}

/// Solve Lambert's problem for multiple position pairs and times (parallelized)
///
/// This processes entire grids of departure/arrival positions and times in
/// a single Rust call with parallel computation. Ideal for porkchop plots.
///
/// # Arguments
/// * `r1s` - Array of initial position vectors, shape (N, 3) (NumPy array)
/// * `r2s` - Array of final position vectors, shape (N, 3) (NumPy array)
/// * `tofs` - Array of time of flight values, length N (NumPy array)
/// * `mu` - Gravitational parameter μ = GM (m³/s²)
/// * `short_way` - If True, use short-way transfer (<180°); if False, use long-way (>180°)
/// * `revs` - Number of complete revolutions (0 for direct transfer)
///
/// # Returns
/// List of N dictionaries with Lambert solutions
///
/// # Performance
/// With parallelization: 50-200x faster than sequential Python implementation
/// for large grids (100x100 or more)
///
/// # Example
/// ```python
/// import numpy as np
/// from astrora._core import lambert_solve_batch_parallel
/// from astrora._core.constants import GM_SUN
///
/// # Generate grid for porkchop plot
/// n = 100
/// r1s = np.random.randn(n, 3) * 1e11  # Earth-like positions
/// r2s = np.random.randn(n, 3) * 2e11  # Mars-like positions
/// tofs = np.linspace(100, 300, n) * 86400  # 100-300 days
///
/// solutions = lambert_solve_batch_parallel(r1s, r2s, tofs, GM_SUN)
/// # Process 100 Lambert problems in parallel
/// ```
#[pyfunction]
#[pyo3(name = "lambert_solve_batch_parallel", signature = (r1s, r2s, tofs, mu, short_way=true, revs=0))]
fn py_lambert_solve_batch_parallel<'py>(
    py: Python<'py>,
    r1s: PyReadonlyArray2<f64>,
    r2s: PyReadonlyArray2<f64>,
    tofs: PyReadonlyArray1<f64>,
    mu: f64,
    short_way: bool,
    revs: u32,
) -> PyResult<Vec<PyObject>> {
    let r1s_array = r1s.as_array();
    let r2s_array = r2s.as_array();
    let tofs_array = tofs.as_array();

    let n = r1s_array.shape()[0];

    if r1s_array.shape()[1] != 3 || r2s_array.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Position vectors must have 3 components",
        ));
    }

    if r2s_array.shape()[0] != n || tofs_array.len() != n {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "All arrays must have the same first dimension",
        ));
    }

    // Convert to vectors
    let r1s_vec: Vec<nalgebra::Vector3<f64>> = (0..n)
        .map(|i| nalgebra::Vector3::new(r1s_array[[i, 0]], r1s_array[[i, 1]], r1s_array[[i, 2]]))
        .collect();

    let r2s_vec: Vec<nalgebra::Vector3<f64>> = (0..n)
        .map(|i| nalgebra::Vector3::new(r2s_array[[i, 0]], r2s_array[[i, 1]], r2s_array[[i, 2]]))
        .collect();

    let tofs_vec: Vec<f64> = tofs_array.to_vec();

    let transfer_kind = if short_way {
        maneuvers::TransferKind::ShortWay
    } else {
        maneuvers::TransferKind::LongWay
    };

    let solutions = maneuvers::Lambert::solve_batch_parallel(&r1s_vec, &r2s_vec, &tofs_vec, mu, transfer_kind, revs)?;

    // Convert each solution to a Python dictionary
    solutions
        .into_iter()
        .map(|solution| {
            let dict = pyo3::types::PyDict::new_bound(py);

            let r1_out = ndarray::arr1(&[solution.r1[0], solution.r1[1], solution.r1[2]]);
            let r2_out = ndarray::arr1(&[solution.r2[0], solution.r2[1], solution.r2[2]]);
            let v1_out = ndarray::arr1(&[solution.v1[0], solution.v1[1], solution.v1[2]]);
            let v2_out = ndarray::arr1(&[solution.v2[0], solution.v2[1], solution.v2[2]]);

            dict.set_item("r1", PyArray1::from_owned_array_bound(py, r1_out))?;
            dict.set_item("r2", PyArray1::from_owned_array_bound(py, r2_out))?;
            dict.set_item("tof", solution.tof)?;
            dict.set_item("v1", PyArray1::from_owned_array_bound(py, v1_out))?;
            dict.set_item("v2", PyArray1::from_owned_array_bound(py, v2_out))?;
            dict.set_item("mu", solution.mu)?;
            dict.set_item("a", solution.a)?;
            dict.set_item("e", solution.e)?;
            dict.set_item("revs", solution.revs)?;
            dict.set_item("short_way", solution.short_way)?;

            Ok(dict.into())
        })
        .collect()
}

// ========================================
// Satellite Operations (SGP4/SDP4 TLE Propagation)
// ========================================

/// Propagate a satellite orbit from TLE string
///
/// # Arguments
///
/// * `tle_string` - TLE data (2-line or 3-line format)
/// * `time_offset_minutes` - Time from epoch in minutes
///
/// # Returns
///
/// Dictionary with satellite state in TEME frame:
/// - `position`: [x, y, z] in km (TEME frame)
/// - `velocity`: [vx, vy, vz] in km/s (TEME frame)
/// - `time_offset_minutes`: Time from epoch
/// - `altitude_km`: Altitude above Earth's surface
/// - `speed_km_s`: Orbital speed
///
/// # Example
///
/// ```python
/// tle = """ISS (ZARYA)
/// 1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
/// 2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""
///
/// state = propagate_tle(tle, 120.0)  # 120 minutes after epoch
/// print(state['position'])  # [x, y, z] in km
/// print(state['altitude_km'])  # Altitude above Earth
/// ```
#[pyfunction]
fn py_propagate_tle<'py>(
    py: Python<'py>,
    tle_string: &str,
    time_offset_minutes: f64,
) -> PyResult<PyObject> {
    use crate::satellite::{parse_tle, propagate_from_elements};

    // Parse TLE
    let elements = parse_tle(tle_string)?;

    // Propagate
    let state = propagate_from_elements(&elements, time_offset_minutes)?;

    // Create Python dictionary
    let dict = pyo3::types::PyDict::new_bound(py);

    // Position and velocity as NumPy arrays
    let position = ndarray::arr1(&state.position);
    let velocity = ndarray::arr1(&state.velocity);

    dict.set_item("position", PyArray1::from_owned_array_bound(py, position))?;
    dict.set_item("velocity", PyArray1::from_owned_array_bound(py, velocity))?;
    dict.set_item("time_offset_minutes", state.time_offset_minutes)?;
    dict.set_item("altitude_km", state.position_magnitude() - 6378.137)?; // WGS84 Earth radius
    dict.set_item("speed_km_s", state.velocity_magnitude())?;

    Ok(dict.into())
}

/// Batch propagate a satellite orbit from TLE to multiple time offsets
///
/// More efficient than calling `propagate_tle` repeatedly.
///
/// # Arguments
///
/// * `tle_string` - TLE data (2-line or 3-line format)
/// * `time_offsets_minutes` - Array of time offsets from epoch (minutes)
///
/// # Returns
///
/// List of dictionaries with satellite states
///
/// # Example
///
/// ```python
/// time_offsets = [0, 60, 120, 180]  # 0, 1, 2, 3 hours
/// states = propagate_tle_batch(tle, time_offsets)
/// for state in states:
///     print(state['altitude_km'])
/// ```
#[pyfunction]
fn py_propagate_tle_batch<'py>(
    py: Python<'py>,
    tle_string: &str,
    time_offsets_minutes: PyReadonlyArray1<f64>,
) -> PyResult<Vec<PyObject>> {
    use crate::satellite::{parse_tle, propagate_batch};

    // Parse TLE
    let elements = parse_tle(tle_string)?;

    // Convert time offsets to Vec
    let time_offsets = time_offsets_minutes.as_array().to_vec();

    // Batch propagate
    let states = propagate_batch(&elements, &time_offsets)?;

    // Convert each state to Python dictionary
    states
        .into_iter()
        .map(|state| {
            let dict = pyo3::types::PyDict::new_bound(py);

            let position = ndarray::arr1(&state.position);
            let velocity = ndarray::arr1(&state.velocity);

            dict.set_item("position", PyArray1::from_owned_array_bound(py, position))?;
            dict.set_item("velocity", PyArray1::from_owned_array_bound(py, velocity))?;
            dict.set_item("time_offset_minutes", state.time_offset_minutes)?;
            dict.set_item("altitude_km", state.position_magnitude() - 6378.137)?;
            dict.set_item("speed_km_s", state.velocity_magnitude())?;

            Ok(dict.into())
        })
        .collect()
}

/// Parse and propagate an OMM (Orbit Mean-Elements Message) JSON string
///
/// # Arguments
///
/// * `omm_json` - OMM data in JSON format
/// * `time_offset_minutes` - Time from epoch in minutes
///
/// # Returns
///
/// Dictionary with satellite state (same format as `propagate_tle`)
///
/// # Example
///
/// ```python
/// omm = '''{
///   "OBJECT_NAME": "ISS (ZARYA)",
///   "EPOCH": "2008-09-20T12:25:40.104",
///   "MEAN_MOTION": 15.72125391,
///   ...
/// }'''
/// state = propagate_omm(omm, 120.0)
/// ```
#[pyfunction]
fn py_propagate_omm<'py>(
    py: Python<'py>,
    omm_json: &str,
    time_offset_minutes: f64,
) -> PyResult<PyObject> {
    use crate::satellite::{parse_omm, propagate_from_elements};

    // Parse OMM
    let elements = parse_omm(omm_json)?;

    // Propagate
    let state = propagate_from_elements(&elements, time_offset_minutes)?;

    // Create Python dictionary (same as TLE)
    let dict = pyo3::types::PyDict::new_bound(py);

    let position = ndarray::arr1(&state.position);
    let velocity = ndarray::arr1(&state.velocity);

    dict.set_item("position", PyArray1::from_owned_array_bound(py, position))?;
    dict.set_item("velocity", PyArray1::from_owned_array_bound(py, velocity))?;
    dict.set_item("time_offset_minutes", state.time_offset_minutes)?;
    dict.set_item("altitude_km", state.position_magnitude() - 6378.137)?;
    dict.set_item("speed_km_s", state.velocity_magnitude())?;

    Ok(dict.into())
}

// ============================================================================
// Satellite Visibility and Ground Station Operations
// ============================================================================

/// Compute azimuth and elevation of a satellite from an observer location
///
/// Transforms satellite position from ECEF (Earth-Centered, Earth-Fixed) coordinates
/// to topocentric coordinates (azimuth, elevation) as seen from a ground observer.
///
/// # Arguments
///
/// * `sat_ecef` - Satellite position in ECEF frame [x, y, z] in km
/// * `observer_lat_deg` - Observer latitude in degrees (-90 to 90)
/// * `observer_lon_deg` - Observer longitude in degrees (-180 to 180)
/// * `observer_alt_km` - Observer altitude above WGS84 ellipsoid in km
///
/// # Returns
///
/// Dictionary with topocentric coordinates:
/// - `azimuth_deg`: Azimuth angle in degrees (0=North, 90=East, 180=South, 270=West)
/// - `elevation_deg`: Elevation angle in degrees (0=horizon, 90=zenith, negative=below)
/// - `range_km`: Distance from observer to satellite in km
/// - `enu_east_km`: East component in local frame (km)
/// - `enu_north_km`: North component in local frame (km)
/// - `enu_up_km`: Up component in local frame (km)
///
/// # Example
///
/// ```python
/// # MIT ground station observing a satellite
/// sat_ecef = [7000.0, 0.0, 0.0]  # ECEF position in km
/// result = compute_azimuth_elevation(
///     sat_ecef,
///     observer_lat_deg=42.36,  # MIT latitude
///     observer_lon_deg=-71.09,  # MIT longitude
///     observer_alt_km=0.010  # 10m altitude
/// )
/// print(f"Azimuth: {result['azimuth_deg']:.1f}°")
/// print(f"Elevation: {result['elevation_deg']:.1f}°")
/// print(f"Range: {result['range_km']:.1f} km")
/// ```
#[pyfunction]
fn py_compute_azimuth_elevation<'py>(
    py: Python<'py>,
    sat_ecef: [f64; 3],
    observer_lat_deg: f64,
    observer_lon_deg: f64,
    observer_alt_km: f64,
) -> PyResult<PyObject> {
    use crate::satellite::visibility::{Observer, compute_azimuth_elevation};

    let observer = Observer::new(
        observer_lat_deg.to_radians(),
        observer_lon_deg.to_radians(),
        observer_alt_km,
    );

    let topo = compute_azimuth_elevation(&sat_ecef, &observer);

    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("azimuth_deg", topo.azimuth.to_degrees())?;
    dict.set_item("elevation_deg", topo.elevation.to_degrees())?;
    dict.set_item("range_km", topo.range)?;
    dict.set_item("enu_east_km", topo.enu[0])?;
    dict.set_item("enu_north_km", topo.enu[1])?;
    dict.set_item("enu_up_km", topo.enu[2])?;

    Ok(dict.into())
}

/// Compute azimuth, elevation, and range rate from satellite position and velocity
///
/// # Arguments
///
/// * `sat_ecef` - Satellite position in ECEF frame [x, y, z] in km
/// * `vel_ecef` - Satellite velocity in ECEF frame [vx, vy, vz] in km/s
/// * `observer_lat_deg` - Observer latitude in degrees
/// * `observer_lon_deg` - Observer longitude in degrees
/// * `observer_alt_km` - Observer altitude in km
///
/// # Returns
///
/// Dictionary with topocentric coordinates including range_rate_km_s
/// (Same as compute_azimuth_elevation plus range_rate_km_s)
///
/// # Example
///
/// ```python
/// sat_ecef = [7000.0, 0.0, 0.0]
/// vel_ecef = [0.0, 7.5, 0.0]
/// result = compute_azimuth_elevation_rate(
///     sat_ecef, vel_ecef, 42.36, -71.09, 0.010
/// )
/// print(f"Range rate: {result['range_rate_km_s']:.3f} km/s")
/// ```
#[pyfunction]
fn py_compute_azimuth_elevation_rate<'py>(
    py: Python<'py>,
    sat_ecef: [f64; 3],
    vel_ecef: [f64; 3],
    observer_lat_deg: f64,
    observer_lon_deg: f64,
    observer_alt_km: f64,
) -> PyResult<PyObject> {
    use crate::satellite::visibility::{Observer, compute_azimuth_elevation_rate};

    let observer = Observer::new(
        observer_lat_deg.to_radians(),
        observer_lon_deg.to_radians(),
        observer_alt_km,
    );

    let topo = compute_azimuth_elevation_rate(&sat_ecef, &vel_ecef, &observer);

    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("azimuth_deg", topo.azimuth.to_degrees())?;
    dict.set_item("elevation_deg", topo.elevation.to_degrees())?;
    dict.set_item("range_km", topo.range)?;
    dict.set_item("range_rate_km_s", topo.range_rate.unwrap_or(0.0))?;
    dict.set_item("enu_east_km", topo.enu[0])?;
    dict.set_item("enu_north_km", topo.enu[1])?;
    dict.set_item("enu_up_km", topo.enu[2])?;

    Ok(dict.into())
}

/// Check if a satellite is visible from an observer location
///
/// # Arguments
///
/// * `sat_ecef` - Satellite position in ECEF frame [x, y, z] in km
/// * `observer_lat_deg` - Observer latitude in degrees
/// * `observer_lon_deg` - Observer longitude in degrees
/// * `observer_alt_km` - Observer altitude in km
/// * `min_elevation_deg` - Minimum elevation for visibility in degrees (typically 0-10)
///
/// # Returns
///
/// `True` if satellite is visible (elevation >= min_elevation), `False` otherwise
///
/// # Example
///
/// ```python
/// sat_ecef = [7000.0, 0.0, 0.0]
/// visible = is_visible(sat_ecef, 42.36, -71.09, 0.010, min_elevation_deg=10.0)
/// if visible:
///     print("Satellite is visible above 10° elevation")
/// ```
#[pyfunction]
fn py_is_visible(
    sat_ecef: [f64; 3],
    observer_lat_deg: f64,
    observer_lon_deg: f64,
    observer_alt_km: f64,
    min_elevation_deg: f64,
) -> PyResult<bool> {
    use crate::satellite::visibility::{Observer, is_visible};

    let observer = Observer::new(
        observer_lat_deg.to_radians(),
        observer_lon_deg.to_radians(),
        observer_alt_km,
    );

    Ok(is_visible(&sat_ecef, &observer, min_elevation_deg.to_radians()))
}

/// Find all satellite passes over a ground station within a time window
///
/// Uses TLE propagation combined with topocentric calculations to predict
/// when a satellite will be visible from a ground station.
///
/// # Arguments
///
/// * `tle_string` - TLE in 2-line or 3-line format
/// * `observer_lat_deg` - Observer latitude in degrees
/// * `observer_lon_deg` - Observer longitude in degrees
/// * `observer_alt_km` - Observer altitude in km
/// * `start_time_minutes` - Start of search window (minutes from TLE epoch)
/// * `end_time_minutes` - End of search window (minutes from TLE epoch)
/// * `min_elevation_deg` - Minimum elevation for visibility (degrees)
/// * `time_step_minutes` - Time step for coarse search (minutes, typically 1-5)
///
/// # Returns
///
/// List of dictionaries, each representing a satellite pass:
/// - `rise_time_minutes`: Time of rise (minutes from epoch)
/// - `set_time_minutes`: Time of set (minutes from epoch)
/// - `max_elevation_time_minutes`: Time of maximum elevation
/// - `max_elevation_deg`: Maximum elevation during pass (degrees)
/// - `rise_azimuth_deg`: Azimuth at rise (degrees)
/// - `set_azimuth_deg`: Azimuth at set (degrees)
/// - `duration_minutes`: Duration of pass (minutes)
///
/// # Example
///
/// ```python
/// tle = '''ISS (ZARYA)
/// 1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
/// 2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537'''
///
/// passes = find_satellite_passes(
///     tle,
///     observer_lat_deg=42.36,  # MIT
///     observer_lon_deg=-71.09,
///     observer_alt_km=0.010,
///     start_time_minutes=0.0,
///     end_time_minutes=1440.0,  # 24 hours
///     min_elevation_deg=10.0,
///     time_step_minutes=1.0
/// )
///
/// for pass_info in passes:
///     print(f"Pass: {pass_info['rise_time_minutes']:.1f} to {pass_info['set_time_minutes']:.1f} min")
///     print(f"  Max elevation: {pass_info['max_elevation_deg']:.1f}°")
/// ```
#[pyfunction]
fn py_find_satellite_passes<'py>(
    py: Python<'py>,
    tle_string: &str,
    observer_lat_deg: f64,
    observer_lon_deg: f64,
    observer_alt_km: f64,
    start_time_minutes: f64,
    end_time_minutes: f64,
    min_elevation_deg: f64,
    time_step_minutes: f64,
) -> PyResult<PyObject> {
    use crate::satellite::{parse_tle, propagate_from_elements, visibility::{Observer, find_all_passes}};

    // Parse TLE once
    let elements = parse_tle(tle_string)?;

    // Create observer
    let observer = Observer::new(
        observer_lat_deg.to_radians(),
        observer_lon_deg.to_radians(),
        observer_alt_km,
    );

    // Create propagation function that converts TEME to ECEF
    let propagate_fn = |t_minutes: f64| -> [f64; 3] {
        // Propagate in TEME frame
        let state = propagate_from_elements(&elements, t_minutes)
            .expect("SGP4 propagation failed");

        // Convert TEME to ITRS (ECEF)
        // Note: For satellite visibility, TEME → ITRS transformation should be used
        // For now, we'll use TEME positions directly as an approximation
        // TODO: Add proper TEME → ITRS transformation in future
        state.position
    };

    // Find all passes
    let passes = find_all_passes(
        &propagate_fn,
        &observer,
        start_time_minutes,
        end_time_minutes,
        min_elevation_deg.to_radians(),
        time_step_minutes,
    );

    // Convert to Python list of dictionaries
    let py_list = pyo3::types::PyList::empty_bound(py);

    for pass in passes {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("rise_time_minutes", pass.rise_time)?;
        dict.set_item("set_time_minutes", pass.set_time)?;
        dict.set_item("max_elevation_time_minutes", pass.max_elevation_time)?;
        dict.set_item("max_elevation_deg", pass.max_elevation.to_degrees())?;
        dict.set_item("rise_azimuth_deg", pass.rise_azimuth.to_degrees())?;
        dict.set_item("set_azimuth_deg", pass.set_azimuth.to_degrees())?;
        dict.set_item("duration_minutes", pass.duration)?;

        py_list.append(dict)?;
    }

    Ok(py_list.into())
}

// ==============================================================================
// Ground Track and Sub-Satellite Point Calculations
// ==============================================================================

/// Convert ECEF Cartesian coordinates to geodetic coordinates (latitude, longitude, altitude)
///
/// # Arguments
/// * `ecef_position` - ECEF position [x, y, z] in km (numpy array)
///
/// # Returns
/// Dictionary with keys:
/// * `latitude_deg` - Geodetic latitude in degrees
/// * `longitude_deg` - Geodetic longitude in degrees
/// * `altitude_km` - Altitude above WGS84 ellipsoid in km
///
/// # Example
/// ```python
/// import astrora_core
/// import numpy as np
///
/// # Satellite position in ECEF
/// ecef = np.array([6700.0, 0.0, 500.0])
///
/// # Convert to geodetic
/// geodetic = astrora_core.ecef_to_geodetic(ecef)
/// print(f"Lat: {geodetic['latitude_deg']:.2f}°")
/// print(f"Lon: {geodetic['longitude_deg']:.2f}°")
/// print(f"Alt: {geodetic['altitude_km']:.2f} km")
/// ```
#[pyfunction(name = "ecef_to_geodetic")]
fn py_ecef_to_geodetic<'py>(
    py: Python<'py>,
    ecef_position: PyReadonlyArray1<f64>,
) -> PyResult<PyObject> {
    use crate::satellite::groundtrack::ecef_to_geodetic;

    // Convert to array
    let ecef = ecef_position.as_array();
    if ecef.len() != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "ECEF position must be a 3-element array [x, y, z]"
        ));
    }

    let ecef_arr = [ecef[0], ecef[1], ecef[2]];

    // Convert to geodetic
    let geodetic = ecef_to_geodetic(&ecef_arr);

    // Return as dictionary
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("latitude_deg", geodetic.latitude.to_degrees())?;
    dict.set_item("longitude_deg", geodetic.longitude.to_degrees())?;
    dict.set_item("altitude_km", geodetic.altitude)?;

    Ok(dict.into())
}

/// Compute ground track from a series of ECEF positions
///
/// # Arguments
/// * `ecef_positions` - Array of ECEF positions, shape (N, 3) in km
/// * `times_minutes` - Array of times in minutes from epoch, length N
///
/// # Returns
/// Dictionary with arrays:
/// * `latitudes_deg` - Array of latitudes in degrees
/// * `longitudes_deg` - Array of longitudes in degrees
/// * `altitudes_km` - Array of altitudes in km
/// * `times_minutes` - Array of times in minutes (copy of input)
///
/// # Example
/// ```python
/// import astrora_core
/// import numpy as np
///
/// # Satellite positions in ECEF at different times
/// positions = np.array([
///     [6700.0, 0.0, 0.0],
///     [6700.0, 100.0, 50.0],
///     [6650.0, 200.0, 100.0]
/// ])
/// times = np.array([0.0, 1.0, 2.0])
///
/// # Compute ground track
/// track = astrora_core.compute_ground_track(positions, times)
/// print(f"Latitudes: {track['latitudes_deg']}")
/// print(f"Longitudes: {track['longitudes_deg']}")
/// ```
#[pyfunction(name = "compute_ground_track")]
fn py_compute_ground_track<'py>(
    py: Python<'py>,
    ecef_positions: PyReadonlyArray2<f64>,
    times_minutes: PyReadonlyArray1<f64>,
) -> PyResult<PyObject> {
    use crate::satellite::groundtrack::compute_ground_track;

    // Get arrays
    let positions = ecef_positions.as_array();
    let times = times_minutes.as_array();

    // Check dimensions
    if positions.shape()[1] != 3 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "ECEF positions must have shape (N, 3)"
        ));
    }

    if positions.shape()[0] != times.len() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            format!("Number of positions ({}) must match number of times ({})",
                    positions.shape()[0], times.len())
        ));
    }

    // Convert to Vec of arrays
    let n = positions.shape()[0];
    let mut pos_vec = Vec::with_capacity(n);
    for i in 0..n {
        pos_vec.push([positions[[i, 0]], positions[[i, 1]], positions[[i, 2]]]);
    }

    let times_vec: Vec<f64> = times.to_vec();

    // Compute ground track
    let track = compute_ground_track(&pos_vec, &times_vec);

    // Convert to Python arrays
    let n_points = track.len();
    let mut lats = Vec::with_capacity(n_points);
    let mut lons = Vec::with_capacity(n_points);
    let mut alts = Vec::with_capacity(n_points);
    let mut times_out = Vec::with_capacity(n_points);

    for point in track {
        lats.push(point.latitude.to_degrees());
        lons.push(point.longitude.to_degrees());
        alts.push(point.altitude);
        times_out.push(point.time);
    }

    // Return as dictionary
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("latitudes_deg", numpy::PyArray1::from_vec_bound(py, lats))?;
    dict.set_item("longitudes_deg", numpy::PyArray1::from_vec_bound(py, lons))?;
    dict.set_item("altitudes_km", numpy::PyArray1::from_vec_bound(py, alts))?;
    dict.set_item("times_minutes", numpy::PyArray1::from_vec_bound(py, times_out))?;

    Ok(dict.into())
}

/// Calculate maximum ground range (distance to horizon) for a satellite
///
/// # Arguments
/// * `altitude_km` - Satellite altitude above Earth's surface in km
///
/// # Returns
/// Maximum ground range to horizon in km
///
/// # Example
/// ```python
/// import astrora_core
///
/// # ISS altitude
/// altitude = 400.0
/// max_range = astrora_core.maximum_ground_range(altitude)
/// print(f"Maximum ground range: {max_range:.1f} km")
/// # Expected: ~2200 km
/// ```
#[pyfunction(name = "maximum_ground_range")]
fn py_maximum_ground_range(altitude_km: f64) -> f64 {
    use crate::satellite::groundtrack::maximum_ground_range;
    maximum_ground_range(altitude_km)
}

/// Calculate satellite swath width on Earth's surface
///
/// # Arguments
/// * `altitude_km` - Satellite altitude above Earth's surface in km
/// * `min_elevation_deg` - Minimum elevation angle in degrees
///
/// # Returns
/// Approximate swath width in km
///
/// # Note
/// This is a simplified approximation. A more accurate formula will be
/// implemented in a future version.
///
/// # Example
/// ```python
/// import astrora_core
///
/// # LEO satellite at 500 km with 10° minimum elevation
/// altitude = 500.0
/// min_elevation = 10.0
/// swath = astrora_core.calculate_swath_width(altitude, min_elevation)
/// print(f"Swath width: {swath:.1f} km")
/// ```
#[pyfunction(name = "calculate_swath_width")]
fn py_calculate_swath_width(altitude_km: f64, min_elevation_deg: f64) -> f64 {
    use crate::satellite::groundtrack::calculate_swath_width;
    calculate_swath_width(altitude_km, min_elevation_deg.to_radians())
}

// ============================================================================
// Satellite Footprint and Coverage Analysis
// ============================================================================

/// Compute the satellite visibility circle (footprint) on Earth's surface.
///
/// Returns a list of latitude/longitude points that form a circle around the
/// sub-satellite point. All points on this circle have the satellite at exactly
/// the minimum elevation angle.
///
/// # Arguments
/// * `lat_sub_deg` - Sub-satellite point latitude in degrees (-90 to +90)
/// * `lon_sub_deg` - Sub-satellite point longitude in degrees (-180 to +180)
/// * `altitude_km` - Satellite altitude above Earth's surface in km
/// * `min_elevation_deg` - Minimum elevation angle in degrees (0 to 90)
/// * `num_points` - Number of points to generate around the circle (default: 64)
///
/// # Returns
/// Dictionary with:
/// - `latitudes`: Array of latitudes in degrees
/// - `longitudes`: Array of longitudes in degrees
/// - `num_points`: Number of points in the circle
/// - `angular_radius_deg`: Angular radius of the circle in degrees
///
/// # Example
/// ```python
/// import astrora_core
///
/// # ISS footprint with 10° minimum elevation
/// circle = astrora_core.visibility_circle(
///     lat_sub_deg=0.0,
///     lon_sub_deg=0.0,
///     altitude_km=400.0,
///     min_elevation_deg=10.0,
///     num_points=64
/// )
/// print(f"Footprint has {circle['num_points']} points")
/// print(f"Angular radius: {circle['angular_radius_deg']:.2f}°")
/// ```
#[pyfunction(name = "visibility_circle")]
fn py_visibility_circle<'py>(
    py: Python<'py>,
    lat_sub_deg: f64,
    lon_sub_deg: f64,
    altitude_km: f64,
    min_elevation_deg: f64,
    num_points: Option<usize>,
) -> PyResult<Bound<'py, pyo3::types::PyDict>> {
    use crate::satellite::coverage::visibility_circle;
    use numpy::PyArray1;

    let num_pts = num_points.unwrap_or(64);

    // Compute visibility circle
    let circle = visibility_circle(lat_sub_deg, lon_sub_deg, altitude_km, min_elevation_deg, num_pts);

    // Extract latitudes and longitudes
    let latitudes: Vec<f64> = circle.iter().map(|p| p.latitude).collect();
    let longitudes: Vec<f64> = circle.iter().map(|p| p.longitude).collect();

    // Calculate angular radius for reference
    let r_sat = 6371.0 + altitude_km;
    let elev_rad = min_elevation_deg.to_radians();
    let sin_eta = elev_rad.cos() * 6371.0 / r_sat;
    let eta = sin_eta.asin();
    let lambda = (std::f64::consts::PI / 2.0) - elev_rad - eta;
    let angular_radius_deg = lambda.to_degrees();

    // Create result dictionary
    let result = pyo3::types::PyDict::new_bound(py);
    result.set_item("latitudes", PyArray1::from_vec_bound(py, latitudes))?;
    result.set_item("longitudes", PyArray1::from_vec_bound(py, longitudes))?;
    result.set_item("num_points", num_pts)?;
    result.set_item("angular_radius_deg", angular_radius_deg)?;

    Ok(result)
}

/// Calculate the total coverage area (satellite footprint area) on Earth's surface.
///
/// Returns the area in square kilometers that can see the satellite above the
/// minimum elevation angle. Uses spherical cap formula for accurate results.
///
/// # Arguments
/// * `altitude_km` - Satellite altitude above Earth's surface in km
/// * `min_elevation_deg` - Minimum elevation angle in degrees (0 to 90)
///
/// # Returns
/// Coverage area in km²
///
/// # Example
/// ```python
/// import astrora_core
///
/// # ISS coverage area with 10° minimum elevation
/// area = astrora_core.coverage_area(400.0, 10.0)
/// print(f"Coverage area: {area/1e6:.2f} million km²")
/// # Expected: ~5.7 million km²
/// ```
#[pyfunction(name = "coverage_area")]
fn py_coverage_area(altitude_km: f64, min_elevation_deg: f64) -> f64 {
    use crate::satellite::coverage::coverage_area;
    coverage_area(altitude_km, min_elevation_deg)
}

/// Calculate the fraction of time a satellite is visible from a ground target.
///
/// # Arguments
/// * `total_access_time_minutes` - Total access time in minutes
/// * `time_span_minutes` - Total time span analyzed in minutes
///
/// # Returns
/// Coverage percentage as a fraction between 0.0 and 1.0
///
/// # Example
/// ```python
/// import astrora_core
///
/// # 90 minutes out of 24 hours
/// coverage = astrora_core.coverage_percentage(90.0, 1440.0)
/// print(f"Coverage: {coverage * 100:.1f}%")
/// # Output: "Coverage: 6.2%"
/// ```
#[pyfunction(name = "coverage_percentage")]
fn py_coverage_percentage(total_access_time_minutes: f64, time_span_minutes: f64) -> f64 {
    use crate::satellite::coverage::coverage_percentage;
    coverage_percentage(total_access_time_minutes, time_span_minutes)
}

// ========================================
// Eclipse Detection and Solar Lighting Conditions
// ========================================

/// Determine eclipse state for a satellite
///
/// Uses the conical shadow model to determine whether a satellite is in
/// sunlight, penumbra (partial shadow), or umbra (full shadow).
///
/// # Arguments
/// * `r_sat_km` - Satellite position vector in ECI frame [x, y, z] (km)
/// * `r_sun_km` - Sun position vector from Earth in ECI frame [x, y, z] (km)
///
/// # Returns
/// String: "Sunlit", "Penumbra", or "Umbra"
///
/// # Example
/// ```python
/// import astrora_core
/// import numpy as np
///
/// # Satellite at 400 km altitude
/// r_sat = np.array([6778.0, 0.0, 0.0])
/// # Sun at 1 AU
/// r_sun = np.array([149.6e6, 0.0, 0.0])
///
/// state = astrora_core.compute_eclipse_state(r_sat, r_sun)
/// print(f"Eclipse state: {state}")
/// # Output: "Sunlit"
/// ```
#[pyfunction(name = "compute_eclipse_state")]
fn py_compute_eclipse_state(r_sat_km: [f64; 3], r_sun_km: [f64; 3]) -> &'static str {
    use crate::satellite::eclipse::{compute_eclipse_state, EclipseState};
    use nalgebra::Vector3;

    // Convert km to m
    let r_sat = Vector3::new(r_sat_km[0] * 1000.0, r_sat_km[1] * 1000.0, r_sat_km[2] * 1000.0);
    let r_sun = Vector3::new(r_sun_km[0] * 1000.0, r_sun_km[1] * 1000.0, r_sun_km[2] * 1000.0);

    let state = compute_eclipse_state(&r_sat, &r_sun);

    match state {
        EclipseState::Sunlit => "Sunlit",
        EclipseState::Penumbra => "Penumbra",
        EclipseState::Umbra => "Umbra",
    }
}

/// Calculate solar beta angle (simplified formula)
///
/// The beta angle is the angle between the orbital plane and the Sun vector.
/// It determines eclipse duration and thermal conditions.
///
/// # Arguments
/// * `inclination_deg` - Orbital inclination (degrees)
/// * `raan_deg` - Right Ascension of Ascending Node (degrees)
/// * `solar_longitude_deg` - Ecliptic longitude of the Sun (degrees)
///
/// # Returns
/// Beta angle in degrees, range [-90, +90]
///
/// # Example
/// ```python
/// import astrora_core
///
/// # ISS-like orbit
/// beta = astrora_core.solar_beta_angle(51.6, 90.0, 0.0)
/// print(f"Beta angle: {beta:.2f}°")
/// ```
#[pyfunction(name = "solar_beta_angle")]
fn py_solar_beta_angle(
    inclination_deg: f64,
    raan_deg: f64,
    solar_longitude_deg: f64,
) -> f64 {
    use crate::satellite::eclipse::solar_beta_angle;

    let i = inclination_deg.to_radians();
    let raan = raan_deg.to_radians();
    let solar_lon = solar_longitude_deg.to_radians();

    solar_beta_angle(i, raan, solar_lon).to_degrees()
}

/// Calculate solar beta angle (precise Vallado formula)
///
/// This version accounts for Earth's axial tilt (obliquity of ecliptic)
/// and is more accurate than the simplified formula.
///
/// # Arguments
/// * `inclination_deg` - Orbital inclination (degrees)
/// * `raan_deg` - Right Ascension of Ascending Node (degrees)
/// * `solar_longitude_deg` - True ecliptic longitude of Sun (degrees)
///
/// # Returns
/// Beta angle in degrees, range [-90, +90]
///
/// # Example
/// ```python
/// import astrora_core
///
/// # ISS-like orbit
/// beta = astrora_core.solar_beta_angle_precise(51.6, 90.0, 0.0)
/// print(f"Beta angle (precise): {beta:.2f}°")
/// ```
#[pyfunction(name = "solar_beta_angle_precise")]
fn py_solar_beta_angle_precise(
    inclination_deg: f64,
    raan_deg: f64,
    solar_longitude_deg: f64,
) -> f64 {
    use crate::satellite::eclipse::solar_beta_angle_precise;

    let i = inclination_deg.to_radians();
    let raan = raan_deg.to_radians();
    let solar_lon = solar_longitude_deg.to_radians();

    solar_beta_angle_precise(i, raan, solar_lon).to_degrees()
}

/// Calculate required inclination for a sun-synchronous orbit
///
/// A sun-synchronous orbit maintains a constant angle between the orbital
/// plane and the Sun direction.
///
/// # Arguments
/// * `altitude_km` - Orbit altitude above Earth's surface (km)
/// * `eccentricity` - Orbital eccentricity (default: 0.0 for circular)
///
/// # Returns
/// Required inclination in degrees for sun-synchronous orbit
///
/// # Raises
/// ValueError if sun-synchronous orbit is not possible at this altitude
///
/// # Example
/// ```python
/// import astrora_core
///
/// # Typical sun-sync orbit at 600 km
/// inclination = astrora_core.sun_synchronous_inclination(600.0, 0.0)
/// print(f"Sun-sync inclination: {inclination:.2f}°")
/// # Output: ~97.8° (retrograde)
/// ```
#[pyfunction(name = "sun_synchronous_inclination", signature = (altitude_km, eccentricity=0.0))]
fn py_sun_synchronous_inclination(altitude_km: f64, eccentricity: f64) -> PyResult<f64> {
    use crate::satellite::eclipse::sun_synchronous_inclination;
    use crate::core::constants::{GM_EARTH, R_EARTH, J2_EARTH};

    let semi_major_axis = (R_EARTH + altitude_km * 1000.0);

    match sun_synchronous_inclination(semi_major_axis, eccentricity, J2_EARTH, R_EARTH, GM_EARTH) {
        Ok(inclination) => Ok(inclination.to_degrees()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Sun-synchronous orbit not possible: {}", e)
        )),
    }
}

/// Calculate eclipse duration for a circular orbit
///
/// Estimates the maximum eclipse duration per orbit based on beta angle
/// and orbit altitude.
///
/// # Arguments
/// * `altitude_km` - Orbit altitude above Earth's surface (km)
/// * `beta_angle_deg` - Solar beta angle (degrees)
///
/// # Returns
/// Eclipse duration in minutes (0 if no eclipse)
///
/// # Example
/// ```python
/// import astrora_core
///
/// # ISS at 400 km with beta = 0° (maximum eclipse)
/// duration = astrora_core.eclipse_duration(400.0, 0.0)
/// print(f"Eclipse duration: {duration:.1f} minutes")
/// # Output: ~37 minutes
/// ```
#[pyfunction(name = "eclipse_duration")]
fn py_eclipse_duration(altitude_km: f64, beta_angle_deg: f64) -> PyResult<f64> {
    use crate::satellite::eclipse::eclipse_duration;
    use crate::core::constants::{GM_EARTH, R_EARTH};

    let semi_major_axis = R_EARTH + altitude_km * 1000.0;
    let beta_angle = beta_angle_deg.to_radians();

    match eclipse_duration(semi_major_axis, beta_angle, GM_EARTH) {
        Ok(duration_sec) => Ok(duration_sec / 60.0), // Convert to minutes
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Eclipse duration calculation failed: {}", e)
        )),
    }
}

// ==============================================================================
// Satellite Lifetime Estimation
// ==============================================================================

/// Estimate orbital lifetime until atmospheric reentry
///
/// Predicts when a satellite will decay to a specified terminal altitude due to
/// atmospheric drag. Uses exponential atmospheric model and simplified drag propagation.
///
/// # Arguments
/// * `r_km` - Initial position vector in kilometers [x, y, z]
/// * `v_km_s` - Initial velocity vector in km/s [vx, vy, vz]
/// * `ballistic_coeff` - Ballistic coefficient B = Cd·A/m (m²/kg)
/// * `terminal_altitude_km` - Altitude at which orbit is considered decayed (km)
/// * `max_time_days` - Maximum propagation time (days)
///
/// # Returns
/// Estimated lifetime in days until reaching terminal altitude
///
/// # Errors
/// Returns an error if:
/// - Orbit is already below terminal altitude
/// - Lifetime exceeds maximum propagation time
/// - Numerical propagation fails
///
/// # Example
/// ```python
/// from astrora import estimate_satellite_lifetime
/// # CubeSat in LEO
/// r = [7000.0, 0.0, 0.0]  # km
/// v = [0.0, 7.5, 0.0]  # km/s
/// B = 0.01  # Ballistic coefficient (m²/kg)
/// lifetime = estimate_satellite_lifetime(r, v, B, 100.0, 365.0)
/// print(f"Orbital lifetime: {lifetime:.1f} days")
/// ```
#[pyfunction]
#[pyo3(name = "estimate_satellite_lifetime")]
fn py_estimate_satellite_lifetime(
    r_km: [f64; 3],
    v_km_s: [f64; 3],
    ballistic_coeff: f64,
    terminal_altitude_km: f64,
    max_time_days: f64,
) -> PyResult<f64> {
    use crate::satellite::lifetime::estimate_lifetime;
    use crate::core::linalg::Vector3;

    // Convert from km to meters
    let r = Vector3::new(r_km[0] * 1000.0, r_km[1] * 1000.0, r_km[2] * 1000.0);
    let v = Vector3::new(v_km_s[0] * 1000.0, v_km_s[1] * 1000.0, v_km_s[2] * 1000.0);
    let terminal_altitude = terminal_altitude_km * 1000.0;
    let max_time = max_time_days * 86400.0; // days to seconds

    match estimate_lifetime(&r, &v, ballistic_coeff, terminal_altitude, max_time, 60.0) {
        Ok(lifetime_sec) => Ok(lifetime_sec / 86400.0), // Convert to days
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Lifetime estimation failed: {}", e)
        )),
    }
}

/// Estimate orbital decay rate at a given altitude
///
/// Calculates the rate of altitude loss per day due to atmospheric drag
/// using an exponential atmospheric density model.
///
/// # Arguments
/// * `altitude_km` - Current orbital altitude above Earth's surface (km)
/// * `ballistic_coeff` - Ballistic coefficient B = Cd·A/m (m²/kg)
///
/// # Returns
/// Altitude decay rate in kilometers per day
///
/// # Example
/// ```python
/// from astrora import estimate_decay_rate
/// # ISS-like orbit at 400 km
/// B = 0.005  # Low ballistic coefficient (large, low mass/area)
/// decay_rate = estimate_decay_rate(400.0, B)
/// print(f"Decay rate: {decay_rate:.3f} km/day")
/// ```
///
/// # Note
/// This is an approximate model. Actual decay depends on:
/// - Solar activity (affects upper atmosphere density)
/// - Satellite orientation (affects drag coefficient)
/// - Orbit eccentricity (varies altitude)
#[pyfunction]
#[pyo3(name = "estimate_decay_rate")]
fn py_estimate_decay_rate(altitude_km: f64, ballistic_coeff: f64) -> f64 {
    use crate::satellite::lifetime::estimate_decay_rate;

    let altitude = altitude_km * 1000.0; // km to meters
    let decay_rate_m_per_day = estimate_decay_rate(altitude, ballistic_coeff);

    decay_rate_m_per_day / 1000.0 // Convert to km/day
}

// ==============================================================================
// Conjunction Analysis
// ==============================================================================

/// Analyze conjunction (close approach) between two satellites
///
/// Determines Time of Closest Approach (TCA), miss distance, and collision risk
/// for two objects in orbit. Uses Keplerian propagation to predict future positions.
///
/// # Arguments
/// * `r1_km` - Position vector of object 1 in kilometers [x, y, z]
/// * `v1_km_s` - Velocity vector of object 1 in km/s [vx, vy, vz]
/// * `r2_km` - Position vector of object 2 in kilometers [x, y, z]
/// * `v2_km_s` - Velocity vector of object 2 in km/s [vx, vy, vz]
/// * `search_window_hours` - Time window to search for close approach (hours)
/// * `collision_threshold_km` - Distance threshold for collision risk (km)
///
/// # Returns
/// Tuple of (TCA in minutes, miss distance in km, collision risk flag)
/// - TCA: Time to closest approach in minutes from now
/// - Miss distance: Minimum separation distance in kilometers
/// - Collision risk: True if miss distance < threshold
///
/// # Errors
/// Returns an error if conjunction analysis fails (e.g., invalid orbits)
///
/// # Example
/// ```python
/// from astrora import compute_conjunction
/// # Two satellites in similar orbits
/// r1 = [7000.0, 0.0, 0.0]
/// v1 = [0.0, 7.5, 0.0]
/// r2 = [7005.0, 0.0, 0.0]  # 5 km offset
/// v2 = [0.0, 7.48, 0.0]  # Slightly different velocity
/// tca, miss_dist, risk = compute_conjunction(r1, v1, r2, v2, 24.0, 1.0)
/// print(f"TCA: {tca:.1f} min, Miss: {miss_dist:.3f} km, Risk: {risk}")
/// ```
#[pyfunction]
#[pyo3(name = "compute_conjunction")]
fn py_compute_conjunction(
    r1_km: [f64; 3],
    v1_km_s: [f64; 3],
    r2_km: [f64; 3],
    v2_km_s: [f64; 3],
    search_window_hours: f64,
    collision_threshold_km: f64,
) -> PyResult<(f64, f64, bool)> {
    use crate::satellite::conjunction::compute_conjunction;
    use crate::core::linalg::Vector3;
    use crate::core::constants::GM_EARTH;

    // Convert from km to meters
    let r1 = Vector3::new(r1_km[0] * 1000.0, r1_km[1] * 1000.0, r1_km[2] * 1000.0);
    let v1 = Vector3::new(v1_km_s[0] * 1000.0, v1_km_s[1] * 1000.0, v1_km_s[2] * 1000.0);
    let r2 = Vector3::new(r2_km[0] * 1000.0, r2_km[1] * 1000.0, r2_km[2] * 1000.0);
    let v2 = Vector3::new(v2_km_s[0] * 1000.0, v2_km_s[1] * 1000.0, v2_km_s[2] * 1000.0);

    let search_window = search_window_hours * 3600.0; // hours to seconds
    let collision_threshold = collision_threshold_km * 1000.0; // km to meters

    match compute_conjunction(&r1, &v1, &r2, &v2, GM_EARTH, search_window, collision_threshold) {
        Ok(result) => Ok((
            result.tca / 60.0, // TCA in minutes
            result.miss_distance / 1000.0, // miss distance in km
            result.collision_risk,
        )),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Conjunction analysis failed: {}", e)
        )),
    }
}

/// Calculate minimum distance during close approach between two satellites
///
/// Computes the closest separation distance between two objects in orbit
/// over a specified time window. Simpler than compute_conjunction - only
/// returns the minimum distance without timing or risk assessment.
///
/// # Arguments
/// * `r1_km` - Position vector of object 1 in kilometers [x, y, z]
/// * `v1_km_s` - Velocity vector of object 1 in km/s [vx, vy, vz]
/// * `r2_km` - Position vector of object 2 in kilometers [x, y, z]
/// * `v2_km_s` - Velocity vector of object 2 in km/s [vx, vy, vz]
/// * `search_window_hours` - Time window to search (hours)
///
/// # Returns
/// Minimum separation distance in kilometers during the search window
///
/// # Errors
/// Returns an error if distance calculation fails (e.g., invalid orbits)
///
/// # Example
/// ```python
/// from astrora import closest_approach_distance
/// # Two satellites in similar orbits
/// r1 = [7000.0, 0.0, 0.0]
/// v1 = [0.0, 7.5, 0.0]
/// r2 = [7010.0, 0.0, 0.0]
/// v2 = [0.0, 7.49, 0.0]
/// min_distance = closest_approach_distance(r1, v1, r2, v2, 24.0)
/// print(f"Closest approach: {min_distance:.3f} km")
/// ```
#[pyfunction]
#[pyo3(name = "closest_approach_distance")]
fn py_closest_approach_distance(
    r1_km: [f64; 3],
    v1_km_s: [f64; 3],
    r2_km: [f64; 3],
    v2_km_s: [f64; 3],
    search_window_hours: f64,
) -> PyResult<f64> {
    use crate::satellite::conjunction::closest_approach_distance;
    use crate::core::linalg::Vector3;
    use crate::core::constants::GM_EARTH;

    // Convert from km to meters
    let r1 = Vector3::new(r1_km[0] * 1000.0, r1_km[1] * 1000.0, r1_km[2] * 1000.0);
    let v1 = Vector3::new(v1_km_s[0] * 1000.0, v1_km_s[1] * 1000.0, v1_km_s[2] * 1000.0);
    let r2 = Vector3::new(r2_km[0] * 1000.0, r2_km[1] * 1000.0, r2_km[2] * 1000.0);
    let v2 = Vector3::new(v2_km_s[0] * 1000.0, v2_km_s[1] * 1000.0, v2_km_s[2] * 1000.0);

    let search_window = search_window_hours * 3600.0; // hours to seconds

    match closest_approach_distance(&r1, &v1, &r2, &v2, GM_EARTH, search_window) {
        Ok(distance) => Ok(distance / 1000.0), // Convert to km
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Closest approach calculation failed: {}", e)
        )),
    }
}
