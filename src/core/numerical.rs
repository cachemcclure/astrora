//! Numerical utilities for root finding and ODE integration
//!
//! This module provides core numerical methods essential for astrodynamics:
//! - Newton-Raphson root finding (for Kepler's equation)
//! - Runge-Kutta 4th order integration (fixed-step)
//! - Dormand-Prince 5(4) adaptive integration
//!
//! These implementations are optimized for orbital mechanics applications
//! and designed to minimize allocations for performance.

use crate::core::error::{PoliastroError, PoliastroResult};
use nalgebra as na;

/// Default tolerance for iterative methods
pub const DEFAULT_TOL: f64 = 1e-12;

/// Default maximum iterations for iterative methods
pub const DEFAULT_MAX_ITER: usize = 50;

/// Newton-Raphson root finder for scalar functions
///
/// Solves f(x) = 0 using the Newton-Raphson iteration:
/// x_{n+1} = x_n - f(x_n) / f'(x_n)
///
/// # Arguments
/// * `f` - Function to find root of
/// * `df` - Derivative of function
/// * `x0` - Initial guess
/// * `tol` - Convergence tolerance (default: 1e-12)
/// * `max_iter` - Maximum iterations (default: 50)
///
/// # Returns
/// The root x such that |f(x)| < tol
///
/// # Errors
/// Returns `ConvergenceError` if:
/// - Maximum iterations exceeded
/// - Derivative becomes too small (near-zero)
/// - Result is NaN or infinite
///
/// # Example
/// ```ignore
/// // Solve x^2 - 2 = 0 (root should be sqrt(2) ≈ 1.414...)
/// let f = |x: f64| x * x - 2.0;
/// let df = |x: f64| 2.0 * x;
/// let root = newton_raphson(f, df, 1.0, None, None)?;
/// assert!((root - 2.0_f64.sqrt()).abs() < 1e-10);
/// ```
pub fn newton_raphson<F, DF>(
    f: F,
    df: DF,
    x0: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<f64>
where
    F: Fn(f64) -> f64,
    DF: Fn(f64) -> f64,
{
    let tol = tol.unwrap_or(DEFAULT_TOL);
    let max_iter = max_iter.unwrap_or(DEFAULT_MAX_ITER);
    let min_derivative = 1e-14; // Prevent division by very small numbers

    let mut x = x0;

    for iter in 0..max_iter {
        let fx = f(x);

        // Check convergence
        if fx.abs() < tol {
            return Ok(x);
        }

        // Check for NaN or infinity
        if !fx.is_finite() {
            return Err(PoliastroError::InvalidNumericalValue {
                context: format!("Newton-Raphson function evaluation at x = {x}"),
                value: fx,
            });
        }

        let dfx = df(x);

        // Check derivative magnitude to avoid division by near-zero
        if dfx.abs() < min_derivative {
            return Err(PoliastroError::DivisionByZero {
                context: format!("Newton-Raphson derivative at iteration {iter}"),
                divisor: dfx,
            });
        }

        // Newton-Raphson step
        let x_new = x - fx / dfx;

        // Check for NaN or infinity in result
        if !x_new.is_finite() {
            return Err(PoliastroError::InvalidNumericalValue {
                context: format!("Newton-Raphson step result at iteration {iter}"),
                value: x_new,
            });
        }

        x = x_new;
    }

    // Maximum iterations exceeded
    Err(PoliastroError::convergence_failure(
        "Newton-Raphson",
        max_iter,
        tol,
    ))
}

/// Newton-Raphson solver optimized for Kepler's equation
///
/// This specialized version combines f(x) / f'(x) evaluation to avoid
/// cancellation errors and improve performance. Useful when the derivative
/// naturally appears in the function evaluation.
///
/// Solves using: x_{n+1} = x_n - ratio(x_n)
///
/// # Arguments
/// * `ratio` - Function that returns f(x) / f'(x)
/// * `f` - Original function (for convergence check)
/// * `x0` - Initial guess
/// * `tol` - Convergence tolerance
/// * `max_iter` - Maximum iterations
///
/// # Example
/// ```ignore
/// // For Kepler's equation: M = E - e*sin(E)
/// // ratio = (E - e*sin(E) - M) / (1 - e*cos(E))
/// let e = 0.5;
/// let M = 1.0;
/// let ratio = |E: f64| (E - e * E.sin() - M) / (1.0 - e * E.cos());
/// let f = |E: f64| E - e * E.sin() - M;
/// let E = newton_raphson_ratio(ratio, f, M, None, None)?;
/// ```
pub fn newton_raphson_ratio<R, F>(
    ratio: R,
    f: F,
    x0: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<f64>
where
    R: Fn(f64) -> f64,
    F: Fn(f64) -> f64,
{
    let tol = tol.unwrap_or(DEFAULT_TOL);
    let max_iter = max_iter.unwrap_or(DEFAULT_MAX_ITER);

    let mut x = x0;

    for iter in 0..max_iter {
        let fx = f(x);

        // Check convergence
        if fx.abs() < tol {
            return Ok(x);
        }

        let ratio_x = ratio(x);

        // Check for numerical issues
        if !ratio_x.is_finite() {
            return Err(PoliastroError::InvalidNumericalValue {
                context: format!("Newton-Raphson ratio at iteration {iter} (x = {x})"),
                value: ratio_x,
            });
        }

        // Update step
        let x_new = x - ratio_x;

        if !x_new.is_finite() {
            return Err(PoliastroError::InvalidNumericalValue {
                context: format!("Newton-Raphson step result at iteration {iter}"),
                value: x_new,
            });
        }

        x = x_new;
    }

    Err(PoliastroError::convergence_failure(
        "Newton-Raphson (ratio)",
        max_iter,
        tol,
    ))
}

/// Runge-Kutta 4th order (RK4) fixed-step integrator
///
/// Integrates dy/dt = f(t, y) from t0 to t0 + h using the classic RK4 method.
/// This is a fixed-step, 4th-order accurate explicit method.
///
/// # Arguments
/// * `f` - Right-hand side function dy/dt = f(t, y)
/// * `t0` - Initial time
/// * `y0` - Initial state vector
/// * `h` - Step size
///
/// # Returns
/// State vector y at time t0 + h
///
/// # Example
/// ```ignore
/// // Integrate dy/dt = -y (exponential decay)
/// let f = |_t: f64, y: &na::DVector<f64>| -y.clone();
/// let y0 = na::DVector::from_vec(vec![1.0]);
/// let y1 = rk4_step(f, 0.0, &y0, 0.1);
/// // Should be close to exp(-0.1) ≈ 0.9048
/// ```
pub fn rk4_step<F>(f: F, t0: f64, y0: &na::DVector<f64>, h: f64) -> na::DVector<f64>
where
    F: Fn(f64, &na::DVector<f64>) -> na::DVector<f64>,
{
    // k1 = f(t0, y0)
    let k1 = f(t0, y0);

    // k2 = f(t0 + h/2, y0 + h*k1/2)
    let k2 = f(t0 + h / 2.0, &(y0 + &k1 * (h / 2.0)));

    // k3 = f(t0 + h/2, y0 + h*k2/2)
    let k3 = f(t0 + h / 2.0, &(y0 + &k2 * (h / 2.0)));

    // k4 = f(t0 + h, y0 + h*k3)
    let k4 = f(t0 + h, &(y0 + &k3 * h));

    // y1 = y0 + h/6 * (k1 + 2*k2 + 2*k3 + k4)
    y0 + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (h / 6.0)
}

/// Dormand-Prince 5(4) coefficients
///
/// These are the Butcher tableau coefficients for the DOPRI5 method,
/// a 5th-order embedded Runge-Kutta method with 4th-order error estimation.
#[allow(dead_code)]
struct DopriCoefficients {
    // Time step fractions (c_i)
    c: [f64; 7],
    // RK matrix coefficients (a_ij)
    a: [[f64; 6]; 7],
    // 5th order solution weights (b_i)
    b: [f64; 7],
    // 4th order solution weights for error estimation (b*_i)
    b_star: [f64; 7],
}

/// Dormand-Prince 8(5,3) coefficients
///
/// These are the Butcher tableau coefficients for the DOP853 method,
/// an 8th-order embedded Runge-Kutta method with 5th-order error estimation.
/// This method has 12 stages and provides higher accuracy than DOPRI5.
#[allow(dead_code)]
struct Dop853Coefficients {
    // Time step fractions (c_i) - 12 stages
    c: [f64; 12],
    // RK matrix coefficients (a_ij) - sparse matrix stored efficiently
    a: [[f64; 11]; 12],
    // 8th order solution weights (b_i)
    b: [f64; 12],
    // 5th order solution weights for error estimation
    b_star: [f64; 12],
}

impl DopriCoefficients {
    fn new() -> Self {
        Self {
            c: [0.0, 1.0 / 5.0, 3.0 / 10.0, 4.0 / 5.0, 8.0 / 9.0, 1.0, 1.0],
            a: [
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [1.0 / 5.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [3.0 / 40.0, 9.0 / 40.0, 0.0, 0.0, 0.0, 0.0],
                [44.0 / 45.0, -56.0 / 15.0, 32.0 / 9.0, 0.0, 0.0, 0.0],
                [
                    19372.0 / 6561.0,
                    -25360.0 / 2187.0,
                    64448.0 / 6561.0,
                    -212.0 / 729.0,
                    0.0,
                    0.0,
                ],
                [
                    9017.0 / 3168.0,
                    -355.0 / 33.0,
                    46732.0 / 5247.0,
                    49.0 / 176.0,
                    -5103.0 / 18656.0,
                    0.0,
                ],
                [
                    35.0 / 384.0,
                    0.0,
                    500.0 / 1113.0,
                    125.0 / 192.0,
                    -2187.0 / 6784.0,
                    11.0 / 84.0,
                ],
            ],
            // 5th order weights
            b: [
                35.0 / 384.0,
                0.0,
                500.0 / 1113.0,
                125.0 / 192.0,
                -2187.0 / 6784.0,
                11.0 / 84.0,
                0.0,
            ],
            // 4th order weights for error estimation
            b_star: [
                5179.0 / 57600.0,
                0.0,
                7571.0 / 16695.0,
                393.0 / 640.0,
                -92097.0 / 339200.0,
                187.0 / 2100.0,
                1.0 / 40.0,
            ],
        }
    }
}

impl Dop853Coefficients {
    fn new() -> Self {
        Self {
            // Time step fractions (c_i) for 12 stages
            c: [
                0.0,
                5.260_015_195_876_773e-2,
                7.890_022_793_815_16e-2,
                1.183_503_419_072_274e-1,
                2.816_496_580_927_726e-1,
                3.333_333_333_333_333e-1,
                2.5e-1,
                3.076_923_076_923_077e-1,
                6.512_820_512_820_513e-1,
                6.0e-1,
                8.571_428_571_428_571e-1,
                1.0,
            ],
            // Butcher tableau A matrix (sparse, stored row by row)
            a: [
                [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [5.260_015_195_876_773e-2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [1.972_505_698_453_79e-2, 5.917_517_095_361_37e-2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [2.958_758_547_680_685e-2, 0.0, 8.876_275_643_042_054e-2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [2.413_651_341_592_667e-1, 0.0, -8.845_494_793_282_861e-1, 9.248_340_032_617_92e-1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [3.703_703_703_703_703_5e-2, 0.0, 0.0, 1.708_286_087_294_738_6e-1, 1.254_676_875_668_224_2e-1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                [3.7109375e-2, 0.0, 0.0, 1.702_522_110_195_440_5e-1, 6.021_653_898_045_596e-2, -1.7578125e-2, 0.0, 0.0, 0.0, 0.0, 0.0],
                [3.709_200_011_850_479e-2, 0.0, 0.0, 1.703_839_257_122_399_8e-1, 1.072_620_304_463_732_8e-1, -1.531_943_774_862_440_2e-2, 8.273_789_163_814_023e-3, 0.0, 0.0, 0.0, 0.0],
                [6.241_109_587_160_757e-1, 0.0, 0.0, -3.360_892_629_446_941_4, -8.682_193_468_417_26e-1, 2.759_209_969_944_671e1, 2.015_406_755_047_789_4e1, -4.348_988_418_106_996e1, 0.0, 0.0, 0.0],
                [4.776_625_364_382_643_4e-1, 0.0, 0.0, -2.488_114_619_971_667_7, -5.902_908_268_368_43e-1, 2.123_005_144_818_119_3e1, 1.527_923_363_288_242_3e1, -3.328_821_096_898_486e1, -2.033_120_170_850_862_7e-2, 0.0, 0.0],
                [-9.371_424_300_859_873e-1, 0.0, 0.0, 5.186_372_428_844_064, 1.091_437_348_996_729_5, -8.149_787_010_746_927, -1.852_006_565_999_696e1, 2.273_948_709_935_050_5e1, 2.493_605_552_679_652_3, -3.046_764_471_898_219_6, 0.0],
                [2.273_310_147_516_538, 0.0, 0.0, -1.053_449_546_673_725e1, -2.000_872_058_224_862_5, -1.795_893_186_311_88e1, 2.794_888_452_941_996e1, -2.858_998_277_135_023_5, -8.872_856_933_530_63, 1.236_056_717_579_430_3e1, 6.433_927_460_157_636e-1],
            ],
            // 8th order solution weights (b_i)
            b: [
                5.429_373_411_656_876_5e-2,
                0.0,
                0.0,
                0.0,
                0.0,
                4.450_312_892_752_409,
                1.891_517_899_314_500_3,
                -5.801_203_960_010_585,
                3.111_643_669_578_199e-1,
                -1.521_609_496_625_161e-1,
                2.013_654_008_040_303_4e-1,
                4.471_061_572_777_259e-2,
            ],
            // 5th order solution weights for error estimation (b*_i)
            // Note: b_star = b - error_coefficients
            b_star: [
                5.429_373_411_656_876_5e-2 - 1.312_004_499_419_488e-2,
                0.0,
                0.0,
                0.0,
                0.0,
                4.450_312_892_752_409 - (-1.225_156_446_376_204_4),
                1.891_517_899_314_500_3 - (-4.957_589_496_572_502e-1),
                -5.801_203_960_010_585 - 1.664_377_182_454_986_4,
                3.111_643_669_578_199e-1 - (-3.503_288_487_499_736_6e-1),
                -1.521_609_496_625_161e-1 - 3.341_791_187_130_175e-1,
                2.013_654_008_040_303_4e-1 - 8.192_320_648_511_571e-2,
                4.471_061_572_777_259e-2 - (-2.235_530_786_388_629_4e-2),
            ],
        }
    }
}

/// Dormand-Prince 5(4) adaptive step integrator (single step)
///
/// Performs one adaptive step of DOPRI5 integration with error control.
/// This is a 5th-order method with embedded 4th-order error estimation.
///
/// # Arguments
/// * `f` - Right-hand side function dy/dt = f(t, y)
/// * `t0` - Initial time
/// * `y0` - Initial state vector
/// * `h` - Initial step size (will be adapted)
/// * `tol` - Error tolerance (controls step size adaptation)
///
/// # Returns
/// Tuple of (new_time, new_state, new_step_size, error_estimate)
///
/// # Example
/// ```ignore
/// let f = |_t: f64, y: &na::DVector<f64>| -y.clone();
/// let y0 = na::DVector::from_vec(vec![1.0]);
/// let (t1, y1, h_new, error) = dopri5_step(f, 0.0, &y0, 0.1, 1e-8);
/// ```
pub fn dopri5_step<F>(
    f: F,
    t0: f64,
    y0: &na::DVector<f64>,
    h: f64,
    tol: f64,
) -> (f64, na::DVector<f64>, f64, f64)
where
    F: Fn(f64, &na::DVector<f64>) -> na::DVector<f64>,
{
    let coeff = DopriCoefficients::new();

    // Compute the 7 stages (k_i)
    let mut k = Vec::with_capacity(7);

    // k1 = f(t0, y0)
    k.push(f(t0, y0));

    // Compute remaining stages
    for i in 1..7 {
        let mut y_temp = y0.clone();
        for j in 0..i {
            y_temp += &k[j] * (h * coeff.a[i][j]);
        }
        k.push(f(t0 + coeff.c[i] * h, &y_temp));
    }

    // Compute 5th order solution
    let mut y_new = y0.clone();
    for i in 0..7 {
        y_new += &k[i] * (h * coeff.b[i]);
    }

    // Compute 4th order solution for error estimation
    let mut y_star = y0.clone();
    for i in 0..7 {
        y_star += &k[i] * (h * coeff.b_star[i]);
    }

    // Error estimate (difference between 5th and 4th order solutions)
    let error_vec = &y_new - &y_star;
    let error_norm = error_vec.norm() / (1.0 + y0.norm()); // Relative error

    // Adaptive step size calculation (PI controller)
    let safety_factor = 0.9;
    let min_factor = 0.2;
    let max_factor = 5.0;

    let factor = if error_norm > 0.0 {
        safety_factor * (tol / error_norm).powf(0.2)
    } else {
        max_factor
    };

    let h_new = h * factor.clamp(min_factor, max_factor);

    (t0 + h, y_new, h_new, error_norm)
}

/// Integrate ODE from t0 to tf using Dormand-Prince 5(4) with adaptive stepping
///
/// # Arguments
/// * `f` - Right-hand side function dy/dt = f(t, y)
/// * `t0` - Initial time
/// * `y0` - Initial state
/// * `tf` - Final time
/// * `h0` - Initial step size
/// * `tol` - Error tolerance
/// * `max_steps` - Maximum number of steps (prevents infinite loops)
///
/// # Returns
/// Final state vector at time tf
///
/// # Errors
/// Returns error if integration fails or exceeds max_steps
pub fn dopri5_integrate<F>(
    f: F,
    t0: f64,
    y0: &na::DVector<f64>,
    tf: f64,
    h0: f64,
    tol: f64,
    max_steps: Option<usize>,
) -> PoliastroResult<na::DVector<f64>>
where
    F: Fn(f64, &na::DVector<f64>) -> na::DVector<f64>,
{
    let max_steps = max_steps.unwrap_or(100000);
    let mut t = t0;
    let mut y = y0.clone();
    let mut h = h0.abs() * (tf - t0).signum();
    let min_step_size = 1e-14;

    for _step in 0..max_steps {
        // Check if we've reached the final time
        if (t - tf).abs() < min_step_size {
            return Ok(y);
        }

        // Don't overshoot the final time
        if (tf - t0).signum() * (t + h - tf) > 0.0 {
            h = tf - t;
        }

        // Check for tiny step size before taking step
        if h.abs() < min_step_size {
            // If we're very close to the end, just return current state
            if (t - tf).abs() < 1e-10 {
                return Ok(y);
            }
            return Err(PoliastroError::NumericalInstability {
                operation: "DOPRI5 integration".to_string(),
                details: format!("Step size became too small at t = {t} (h = {h})"),
            });
        }

        let (t_new, y_new, h_new, error) = dopri5_step(&f, t, &y, h, tol);

        // Accept step if error is acceptable
        if error <= tol {
            t = t_new;
            y = y_new;
            h = h_new;
        } else {
            // Reject step and retry with smaller step size
            h = h_new;
        }
    }

    Err(PoliastroError::convergence_failure(
        "DOPRI5 integration",
        max_steps,
        tol,
    ))
}

/// Dormand-Prince 8(5,3) adaptive step integrator (single step)
///
/// Performs one adaptive step of DOP853 integration with error control.
/// This is an 8th-order method with embedded 5th-order error estimation,
/// providing higher accuracy than DOPRI5 for smooth problems.
///
/// # Arguments
/// * `f` - Right-hand side function dy/dt = f(t, y)
/// * `t0` - Initial time
/// * `y0` - Initial state vector
/// * `h` - Initial step size (will be adapted)
/// * `tol` - Error tolerance (controls step size adaptation)
///
/// # Returns
/// Tuple of (new_time, new_state, new_step_size, error_estimate)
///
/// # Example
/// ```ignore
/// let f = |_t: f64, y: &na::DVector<f64>| -y.clone();
/// let y0 = na::DVector::from_vec(vec![1.0]);
/// let (t1, y1, h_new, error) = dop853_step(f, 0.0, &y0, 0.1, 1e-10);
/// ```
pub fn dop853_step<F>(
    f: F,
    t0: f64,
    y0: &na::DVector<f64>,
    h: f64,
    tol: f64,
) -> (f64, na::DVector<f64>, f64, f64)
where
    F: Fn(f64, &na::DVector<f64>) -> na::DVector<f64>,
{
    let coeff = Dop853Coefficients::new();

    // Compute the 12 stages (k_i)
    let mut k = Vec::with_capacity(12);

    // k1 = f(t0, y0)
    k.push(f(t0, y0));

    // Compute remaining stages (i = 1 to 11)
    for i in 1..12 {
        let mut y_temp = y0.clone();
        for j in 0..i {
            y_temp += &k[j] * (h * coeff.a[i][j]);
        }
        k.push(f(t0 + coeff.c[i] * h, &y_temp));
    }

    // Compute 8th order solution
    let mut y_new = y0.clone();
    for i in 0..12 {
        y_new += &k[i] * (h * coeff.b[i]);
    }

    // Compute 5th order solution for error estimation
    let mut y_star = y0.clone();
    for i in 0..12 {
        y_star += &k[i] * (h * coeff.b_star[i]);
    }

    // Error estimate (difference between 8th and 5th order solutions)
    let error_vec = &y_new - &y_star;
    let error_norm = error_vec.norm() / (1.0 + y0.norm()); // Relative error

    // Adaptive step size calculation (PI controller)
    // For DOP853: 8th order solution with 5th order error estimate
    // Step size control exponent: 1/(error_order + 1) = 1/6
    let safety_factor = 0.9;
    let min_factor = 0.2;
    let max_factor = 10.0;

    let factor = if error_norm > 0.0 {
        safety_factor * (tol / error_norm).powf(1.0 / 6.0) // 1/6 for 5th order error estimate
    } else {
        max_factor
    };

    let h_new = h * factor.clamp(min_factor, max_factor);

    (t0 + h, y_new, h_new, error_norm)
}

/// Integrate ODE from t0 to tf using Dormand-Prince 8(5,3) with adaptive stepping
///
/// This method provides higher accuracy than DOPRI5 and is recommended for
/// problems requiring tight error tolerances (e.g., tol < 1e-10).
///
/// # Arguments
/// * `f` - Right-hand side function dy/dt = f(t, y)
/// * `t0` - Initial time
/// * `y0` - Initial state
/// * `tf` - Final time
/// * `h0` - Initial step size
/// * `tol` - Error tolerance
/// * `max_steps` - Maximum number of steps (prevents infinite loops)
///
/// # Returns
/// Final state vector at time tf
///
/// # Errors
/// Returns error if integration fails or exceeds max_steps
///
/// # Example
/// ```ignore
/// let f = |_t: f64, y: &na::DVector<f64>| -y.clone();
/// let y0 = na::DVector::from_vec(vec![1.0]);
/// let y_final = dop853_integrate(f, 0.0, &y0, 1.0, 0.1, 1e-10, None).unwrap();
/// ```
pub fn dop853_integrate<F>(
    f: F,
    t0: f64,
    y0: &na::DVector<f64>,
    tf: f64,
    h0: f64,
    tol: f64,
    max_steps: Option<usize>,
) -> PoliastroResult<na::DVector<f64>>
where
    F: Fn(f64, &na::DVector<f64>) -> na::DVector<f64>,
{
    let max_steps = max_steps.unwrap_or(100000);
    let mut t = t0;
    let mut y = y0.clone();
    let mut h = h0.abs() * (tf - t0).signum();
    let min_step_size = 1e-14;

    for _step in 0..max_steps {
        // Check if we've reached the final time
        if (t - tf).abs() < min_step_size {
            return Ok(y);
        }

        // Don't overshoot the final time
        if (tf - t0).signum() * (t + h - tf) > 0.0 {
            h = tf - t;
        }

        // Check for tiny step size before taking step
        if h.abs() < min_step_size {
            // If we're very close to the end, just return current state
            if (t - tf).abs() < 1e-10 {
                return Ok(y);
            }
            return Err(PoliastroError::NumericalInstability {
                operation: "DOP853 integration".to_string(),
                details: format!("Step size became too small at t = {t} (h = {h})"),
            });
        }

        let (t_new, y_new, h_new, error) = dop853_step(&f, t, &y, h, tol);

        // Accept step if error is acceptable
        if error <= tol {
            t = t_new;
            y = y_new;
            h = h_new;
        } else {
            // Reject step and retry with smaller step size
            h = h_new;
        }
    }

    Err(PoliastroError::convergence_failure(
        "DOP853 integration",
        max_steps,
        tol,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_newton_raphson_sqrt2() {
        // Solve x^2 - 2 = 0
        let f = |x: f64| x * x - 2.0;
        let df = |x: f64| 2.0 * x;

        let root = newton_raphson(f, df, 1.0, None, None).unwrap();
        assert_relative_eq!(root, 2.0_f64.sqrt(), epsilon = 1e-10);
    }

    #[test]
    fn test_newton_raphson_cubic() {
        // Solve x^3 - x - 2 = 0 (root at x ≈ 1.521)
        let f = |x: f64| x.powi(3) - x - 2.0;
        let df = |x: f64| 3.0 * x.powi(2) - 1.0;

        let root = newton_raphson(f, df, 1.5, None, None).unwrap();
        assert!(f(root).abs() < 1e-10);
        assert_relative_eq!(root, 1.521379706804568, epsilon = 1e-10);
    }

    #[test]
    fn test_newton_raphson_convergence_failure() {
        // Pathological case: f(x) = x^(1/3) has zero derivative at x=0
        let f = |x: f64| x.powf(1.0 / 3.0);
        let df = |x: f64| (1.0 / 3.0) * x.powf(-2.0 / 3.0);

        let result = newton_raphson(f, df, 0.1, Some(1e-6), Some(10));
        // Should fail due to derivative issues
        assert!(result.is_err());
    }

    #[test]
    fn test_rk4_exponential_decay() {
        // dy/dt = -y, y(0) = 1
        // Exact solution: y(t) = exp(-t)
        let f = |_t: f64, y: &na::DVector<f64>| -y;

        let y0 = na::DVector::from_vec(vec![1.0]);
        let h = 0.1;

        // Take 10 steps to reach t = 1.0
        let mut y = y0.clone();
        let mut t = 0.0;
        for _ in 0..10 {
            y = rk4_step(&f, t, &y, h);
            t += h;
        }

        // Compare with exact solution at t = 1.0
        let exact = (-1.0_f64).exp();
        assert_relative_eq!(y[0], exact, epsilon = 1e-6);
    }

    #[test]
    fn test_rk4_harmonic_oscillator() {
        // d²x/dt² = -x (harmonic oscillator)
        // Convert to system: dy1/dt = y2, dy2/dt = -y1
        // Initial: y1(0) = 1, y2(0) = 0
        // Exact: y1(t) = cos(t), y2(t) = -sin(t)

        let f = |_t: f64, y: &na::DVector<f64>| na::DVector::from_vec(vec![y[1], -y[0]]);

        let y0 = na::DVector::from_vec(vec![1.0, 0.0]);
        let h = 0.01; // Small step for accuracy

        // Integrate to t = π/2
        let n_steps = (std::f64::consts::FRAC_PI_2 / h) as usize;
        let mut y = y0.clone();
        let mut t = 0.0;

        for _ in 0..n_steps {
            y = rk4_step(&f, t, &y, h);
            t += h;
        }

        // At t = π/2: y1 ≈ 0, y2 ≈ -1
        assert_relative_eq!(y[0], 0.0, epsilon = 1e-3);
        assert_relative_eq!(y[1], -1.0, epsilon = 1e-3);
    }

    #[test]
    fn test_dopri5_exponential_decay() {
        // dy/dt = -y, y(0) = 1
        let f = |_t: f64, y: &na::DVector<f64>| -y;

        let y0 = na::DVector::from_vec(vec![1.0]);
        let y_final = dopri5_integrate(f, 0.0, &y0, 1.0, 0.1, 1e-8, None).unwrap();

        let exact = (-1.0_f64).exp();
        assert_relative_eq!(y_final[0], exact, epsilon = 1e-7);
    }

    #[test]
    fn test_dopri5_harmonic_oscillator() {
        // Harmonic oscillator system
        let f = |_t: f64, y: &na::DVector<f64>| na::DVector::from_vec(vec![y[1], -y[0]]);

        let y0 = na::DVector::from_vec(vec![1.0, 0.0]);
        let y_final =
            dopri5_integrate(f, 0.0, &y0, std::f64::consts::FRAC_PI_2, 0.1, 1e-8, None).unwrap();

        // At t = π/2: y1 ≈ 0, y2 ≈ -1
        assert_relative_eq!(y_final[0], 0.0, epsilon = 1e-6);
        assert_relative_eq!(y_final[1], -1.0, epsilon = 1e-6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_newton_raphson_ratio_kepler() {
        // Test with actual Kepler's equation: M = E - e*sin(E)
        // For e = 0.5, M = 1.0, solve for E
        // Note: Capital letters are conventional in orbital mechanics
        // (E = eccentric anomaly, M = mean anomaly)
        let e = 0.5;
        let M = 1.0;

        let ratio = |E: f64| (E - e * E.sin() - M) / (1.0 - e * E.cos());
        let f = |E: f64| E - e * E.sin() - M;

        let E = newton_raphson_ratio(ratio, f, M, None, None).unwrap();

        // Verify the solution
        assert!(f(E).abs() < 1e-10);
    }

    #[test]
    fn test_dopri5_step_size_adaptation() {
        // Test that step size adapts correctly
        let f = |_t: f64, y: &na::DVector<f64>| -y;
        let y0 = na::DVector::from_vec(vec![1.0]);

        let (_, _, h_new, error) = dopri5_step(&f, 0.0, &y0, 0.1, 1e-6);

        // Error should be within tolerance
        assert!(error < 1e-6 || h_new < 0.1); // Either error is acceptable or step reduced

        // New step size should be reasonable
        assert!(h_new > 0.0);
        assert!(h_new < 10.0);
    }

    #[test]
    fn test_dop853_exponential_decay() {
        // dy/dt = -y, y(0) = 1
        // Exact solution: y(t) = exp(-t)
        let f = |_t: f64, y: &na::DVector<f64>| -y;

        let y0 = na::DVector::from_vec(vec![1.0]);
        let y_final = dop853_integrate(f, 0.0, &y0, 1.0, 0.1, 1e-10, None).unwrap();

        let exact = (-1.0_f64).exp();
        // DOP853 should achieve much higher accuracy than DOPRI5
        assert_relative_eq!(y_final[0], exact, epsilon = 1e-9);
    }

    #[test]
    fn test_dop853_harmonic_oscillator() {
        // d²x/dt² = -x (harmonic oscillator)
        // Convert to system: dy1/dt = y2, dy2/dt = -y1
        // Initial: y1(0) = 1, y2(0) = 0
        // Exact: y1(t) = cos(t), y2(t) = -sin(t)

        let f = |_t: f64, y: &na::DVector<f64>| na::DVector::from_vec(vec![y[1], -y[0]]);

        let y0 = na::DVector::from_vec(vec![1.0, 0.0]);
        let y_final =
            dop853_integrate(f, 0.0, &y0, std::f64::consts::FRAC_PI_2, 0.1, 1e-10, None).unwrap();

        // At t = π/2: y1 ≈ 0, y2 ≈ -1
        // DOP853 should achieve very high accuracy
        assert_relative_eq!(y_final[0], 0.0, epsilon = 1e-8);
        assert_relative_eq!(y_final[1], -1.0, epsilon = 1e-8);
    }

    #[test]
    fn test_dop853_step_size_adaptation() {
        // Test that step size adapts correctly with DOP853
        let f = |_t: f64, y: &na::DVector<f64>| -y;
        let y0 = na::DVector::from_vec(vec![1.0]);

        let (_, _, h_new, error) = dop853_step(&f, 0.0, &y0, 0.1, 1e-10);

        // Error should be within tolerance or step should be reduced
        assert!(error < 1e-10 || h_new < 0.1);

        // New step size should be reasonable
        assert!(h_new > 0.0);
        assert!(h_new < 10.0);
    }

    #[test]
    fn test_dop853_vs_dopri5_accuracy() {
        // Compare accuracy of DOP853 vs DOPRI5 for the same problem
        // dy/dt = -y, y(0) = 1, integrate to t = 5.0
        let f = |_t: f64, y: &na::DVector<f64>| -y;
        let y0 = na::DVector::from_vec(vec![1.0]);
        let tf = 5.0;
        let tol = 1e-10;

        // Integrate with both methods
        let y_dopri5 = dopri5_integrate(&f, 0.0, &y0, tf, 0.1, tol, None).unwrap();
        let y_dop853 = dop853_integrate(&f, 0.0, &y0, tf, 0.1, tol, None).unwrap();

        let exact = (-tf).exp();

        // DOP853 should be more accurate than DOPRI5
        let error_dopri5 = (y_dopri5[0] - exact).abs();
        let error_dop853 = (y_dop853[0] - exact).abs();

        // DOP853 should have significantly better accuracy
        assert!(error_dop853 < error_dopri5);
        assert_relative_eq!(y_dop853[0], exact, epsilon = 1e-9);
    }

    #[test]
    fn test_dop853_stiff_problem() {
        // Test DOP853 on a mildly stiff problem
        // dy/dt = -10*y, y(0) = 1
        // Exact solution: y(t) = exp(-10*t)
        let f = |_t: f64, y: &na::DVector<f64>| y * -10.0;

        let y0 = na::DVector::from_vec(vec![1.0]);
        let y_final = dop853_integrate(f, 0.0, &y0, 0.5, 0.01, 1e-8, None).unwrap();

        let exact = (-5.0_f64).exp();
        assert_relative_eq!(y_final[0], exact, epsilon = 1e-7);
    }

    #[test]
    fn test_dop853_backward_integration() {
        // Test backward integration (negative time direction)
        // dy/dt = -y, y(1) = exp(-1), integrate back to t = 0
        let f = |_t: f64, y: &na::DVector<f64>| -y;

        let y0 = na::DVector::from_vec(vec![(-1.0_f64).exp()]);
        let y_final = dop853_integrate(f, 1.0, &y0, 0.0, -0.1, 1e-10, None).unwrap();

        // Should get back to y ≈ 1.0
        assert_relative_eq!(y_final[0], 1.0, epsilon = 1e-8);
    }

    #[test]
    fn test_dop853_multidimensional() {
        // Test with a 3D system
        // dy1/dt = -y1, dy2/dt = -2*y2, dy3/dt = -3*y3
        let f = |_t: f64, y: &na::DVector<f64>| {
            na::DVector::from_vec(vec![-y[0], -2.0 * y[1], -3.0 * y[2]])
        };

        let y0 = na::DVector::from_vec(vec![1.0, 1.0, 1.0]);
        let t_final = 0.5;
        let y_final = dop853_integrate(f, 0.0, &y0, t_final, 0.1, 1e-10, None).unwrap();

        // Exact solutions
        let exact1 = (-t_final).exp();
        let exact2 = (-2.0 * t_final).exp();
        let exact3 = (-3.0 * t_final).exp();

        assert_relative_eq!(y_final[0], exact1, epsilon = 1e-9);
        assert_relative_eq!(y_final[1], exact2, epsilon = 1e-9);
        assert_relative_eq!(y_final[2], exact3, epsilon = 1e-9);
    }
}
