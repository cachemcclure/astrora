//! Anomaly conversions for orbital mechanics
//!
//! This module provides conversions between different anomaly representations
//! for elliptical, parabolic, and hyperbolic orbits:
//!
//! - **Mean anomaly (M)**: Linear function of time (M = n·t)
//! - **Eccentric anomaly (E)**: Geometric parameter for elliptical orbits
//! - **Hyperbolic anomaly (H)**: Geometric parameter for hyperbolic orbits
//! - **Parabolic anomaly (D)**: Geometric parameter for parabolic orbits
//! - **True anomaly (ν)**: Actual angle from periapsis to satellite
//!
//! # Elliptical Orbits (e < 1)
//! - Kepler's equation: M = E - e·sin(E)
//! - True anomaly from eccentric: tan(ν/2) = √((1+e)/(1-e)) · tan(E/2)
//!
//! # Hyperbolic Orbits (e > 1)
//! - Kepler's hyperbolic equation: M = e·sinh(H) - H
//! - True anomaly from hyperbolic: tan(ν/2) = √((e+1)/(e-1)) · tanh(H/2)
//!
//! # Parabolic Orbits (e = 1)
//! - Barker's equation: M = D + D³/3 (closed-form solution)
//! - True anomaly from parabolic: tan(ν/2) = D

use crate::core::error::{PoliastroError, PoliastroResult};
use crate::core::numerical::{newton_raphson_ratio, DEFAULT_TOL, DEFAULT_MAX_ITER};
use rayon::prelude::*;
use std::f64::consts::PI;

/// Tolerance for determining orbit type
const ORBIT_TYPE_TOL: f64 = 1e-8;

// ============================================================================
// Elliptical Orbit Conversions (e < 1)
// ============================================================================

/// Convert mean anomaly to eccentric anomaly for elliptical orbits
///
/// Solves Kepler's equation: M = E - e·sin(E) for E using Newton-Raphson iteration.
///
/// # Arguments
/// * `mean_anomaly` - Mean anomaly M (radians)
/// * `eccentricity` - Orbital eccentricity e (must be < 1)
/// * `tol` - Convergence tolerance (optional, default: 1e-12)
/// * `max_iter` - Maximum iterations (optional, default: 50)
///
/// # Returns
/// Eccentric anomaly E (radians)
///
/// # Errors
/// - `UnsupportedOrbitType` if e ≥ 1 (not elliptical)
/// - `ConvergenceError` if Newton-Raphson fails to converge
///
/// # Algorithm
/// Uses Newton-Raphson with smart initial guess:
/// - E₀ = M + e·sin(M) for moderate eccentricities
/// - Iteration: E_{n+1} = E_n - (E_n - e·sin(E_n) - M) / (1 - e·cos(E_n))
///
/// # Example
/// ```
/// use astrora_core::core::anomaly::mean_to_eccentric_anomaly;
///
/// let M = 1.0; // radians
/// let e = 0.5;
/// let E = mean_to_eccentric_anomaly(M, e, None, None).unwrap();
/// ```
pub fn mean_to_eccentric_anomaly(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<f64> {
    // Validate orbit type
    if eccentricity >= 1.0 - ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "mean_to_eccentric_anomaly".into(),
            orbit_type: if eccentricity > 1.0 + ORBIT_TYPE_TOL {
                "hyperbolic"
            } else {
                "parabolic"
            }
            .into(),
        });
    }

    if eccentricity < 0.0 {
        return Err(PoliastroError::InvalidParameter {
            parameter: "eccentricity".into(),
            value: eccentricity,
            constraint: "must be >= 0".into(),
        });
    }

    let tol = tol.unwrap_or(DEFAULT_TOL);
    let max_iter = max_iter.unwrap_or(DEFAULT_MAX_ITER);

    // Normalize mean anomaly to [0, 2π)
    let M = mean_anomaly.rem_euclid(2.0 * PI);

    // Initial guess using smart heuristic
    // For moderate eccentricities, E₀ = M + e·sin(M) is excellent
    // For high eccentricities near π, use M directly
    let E0 = if eccentricity < 0.8 {
        M + eccentricity * M.sin()
    } else {
        // For high eccentricity, start with mean anomaly
        M
    };

    // Define Kepler's equation and its ratio for Newton-Raphson
    // f(E) = E - e·sin(E) - M
    // f'(E) = 1 - e·cos(E)
    // ratio = f/f' = (E - e·sin(E) - M) / (1 - e·cos(E))
    let ratio = |E: f64| (E - eccentricity * E.sin() - M) / (1.0 - eccentricity * E.cos());
    let f = |E: f64| E - eccentricity * E.sin() - M;

    newton_raphson_ratio(ratio, f, E0, Some(tol), Some(max_iter))
}

/// Convert eccentric anomaly to mean anomaly for elliptical orbits
///
/// Direct calculation: M = E - e·sin(E)
///
/// # Arguments
/// * `eccentric_anomaly` - Eccentric anomaly E (radians)
/// * `eccentricity` - Orbital eccentricity e (must be < 1)
///
/// # Returns
/// Mean anomaly M (radians, in range [0, 2π))
///
/// # Example
/// ```
/// use astrora_core::core::anomaly::eccentric_to_mean_anomaly;
///
/// let E = 1.2;
/// let e = 0.5;
/// let M = eccentric_to_mean_anomaly(E, e).unwrap();
/// ```
pub fn eccentric_to_mean_anomaly(
    eccentric_anomaly: f64,
    eccentricity: f64,
) -> PoliastroResult<f64> {
    if eccentricity >= 1.0 - ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "eccentric_to_mean_anomaly".into(),
            orbit_type: "not elliptical".into(),
        });
    }

    // Direct calculation
    let M = (eccentric_anomaly - eccentricity * eccentric_anomaly.sin()).rem_euclid(2.0 * PI);
    Ok(M)
}

/// Convert eccentric anomaly to true anomaly for elliptical orbits
///
/// Uses the formula: tan(ν/2) = √((1+e)/(1-e)) · tan(E/2)
///
/// # Arguments
/// * `eccentric_anomaly` - Eccentric anomaly E (radians)
/// * `eccentricity` - Orbital eccentricity e (must be < 1)
///
/// # Returns
/// True anomaly ν (radians, in range [0, 2π))
///
/// # Algorithm
/// Uses the half-angle formula with atan2 for proper quadrant determination
///
/// # Example
/// ```
/// use astrora_core::core::anomaly::eccentric_to_true_anomaly;
///
/// let E = 1.0;
/// let e = 0.5;
/// let nu = eccentric_to_true_anomaly(E, e).unwrap();
/// ```
pub fn eccentric_to_true_anomaly(
    eccentric_anomaly: f64,
    eccentricity: f64,
) -> PoliastroResult<f64> {
    if eccentricity >= 1.0 - ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "eccentric_to_true_anomaly".into(),
            orbit_type: "not elliptical".into(),
        });
    }

    // Using the formulation:
    // cos(ν) = (cos(E) - e) / (1 - e·cos(E))
    // sin(ν) = (√(1-e²)·sin(E)) / (1 - e·cos(E))
    // This gives correct quadrant with atan2

    let cos_E = eccentric_anomaly.cos();
    let sin_E = eccentric_anomaly.sin();
    let denom = 1.0 - eccentricity * cos_E;

    let cos_nu = (cos_E - eccentricity) / denom;
    let sin_nu = ((1.0 - eccentricity * eccentricity).sqrt() * sin_E) / denom;

    // atan2 handles quadrants correctly and returns [-π, π]
    let nu = sin_nu.atan2(cos_nu);

    // Normalize to [0, 2π)
    Ok(nu.rem_euclid(2.0 * PI))
}

/// Convert true anomaly to eccentric anomaly for elliptical orbits
///
/// Uses the formula: tan(E/2) = √((1-e)/(1+e)) · tan(ν/2)
///
/// # Arguments
/// * `true_anomaly` - True anomaly ν (radians)
/// * `eccentricity` - Orbital eccentricity e (must be < 1)
///
/// # Returns
/// Eccentric anomaly E (radians, in range [0, 2π))
///
/// # Example
/// ```
/// use astrora_core::core::anomaly::true_to_eccentric_anomaly;
///
/// let nu = 1.5;
/// let e = 0.3;
/// let E = true_to_eccentric_anomaly(nu, e).unwrap();
/// ```
pub fn true_to_eccentric_anomaly(
    true_anomaly: f64,
    eccentricity: f64,
) -> PoliastroResult<f64> {
    if eccentricity >= 1.0 - ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "true_to_eccentric_anomaly".into(),
            orbit_type: "not elliptical".into(),
        });
    }

    // Using the formulation:
    // cos(E) = (e + cos(ν)) / (1 + e·cos(ν))
    // sin(E) = (√(1-e²)·sin(ν)) / (1 + e·cos(ν))

    let cos_nu = true_anomaly.cos();
    let sin_nu = true_anomaly.sin();
    let denom = 1.0 + eccentricity * cos_nu;

    let cos_E = (eccentricity + cos_nu) / denom;
    let sin_E = ((1.0 - eccentricity * eccentricity).sqrt() * sin_nu) / denom;

    let E = sin_E.atan2(cos_E);
    Ok(E.rem_euclid(2.0 * PI))
}

/// Convert mean anomaly to true anomaly for elliptical orbits
///
/// Convenience function combining M → E → ν conversions
///
/// # Arguments
/// * `mean_anomaly` - Mean anomaly M (radians)
/// * `eccentricity` - Orbital eccentricity e (must be < 1)
/// * `tol` - Convergence tolerance (optional)
/// * `max_iter` - Maximum iterations (optional)
///
/// # Returns
/// True anomaly ν (radians)
pub fn mean_to_true_anomaly(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<f64> {
    let E = mean_to_eccentric_anomaly(mean_anomaly, eccentricity, tol, max_iter)?;
    eccentric_to_true_anomaly(E, eccentricity)
}

/// Convert true anomaly to mean anomaly for elliptical orbits
///
/// Convenience function combining ν → E → M conversions
///
/// # Arguments
/// * `true_anomaly` - True anomaly ν (radians)
/// * `eccentricity` - Orbital eccentricity e (must be < 1)
///
/// # Returns
/// Mean anomaly M (radians)
pub fn true_to_mean_anomaly(true_anomaly: f64, eccentricity: f64) -> PoliastroResult<f64> {
    let E = true_to_eccentric_anomaly(true_anomaly, eccentricity)?;
    eccentric_to_mean_anomaly(E, eccentricity)
}

// ============================================================================
// Hyperbolic Orbit Conversions (e > 1)
// ============================================================================

/// Convert mean anomaly to hyperbolic anomaly for hyperbolic orbits
///
/// Solves the hyperbolic Kepler equation: M = e·sinh(H) - H for H
///
/// # Arguments
/// * `mean_anomaly` - Mean anomaly M (radians)
/// * `eccentricity` - Orbital eccentricity e (must be > 1)
/// * `tol` - Convergence tolerance (optional, default: 1e-12)
/// * `max_iter` - Maximum iterations (optional, default: 50)
///
/// # Returns
/// Hyperbolic anomaly H (radians)
///
/// # Errors
/// - `UnsupportedOrbitType` if e ≤ 1 (not hyperbolic)
/// - `ConvergenceError` if Newton-Raphson fails
///
/// # Algorithm
/// Newton-Raphson with initial guess H₀ = M
/// Iteration: H_{n+1} = H_n - (e·sinh(H_n) - H_n - M) / (e·cosh(H_n) - 1)
///
/// # Example
/// ```
/// use astrora_core::core::anomaly::mean_to_hyperbolic_anomaly;
///
/// let M = 2.0;
/// let e = 1.5; // hyperbolic
/// let H = mean_to_hyperbolic_anomaly(M, e, None, None).unwrap();
/// ```
pub fn mean_to_hyperbolic_anomaly(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<f64> {
    // Validate orbit type
    if eccentricity <= 1.0 + ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "mean_to_hyperbolic_anomaly".into(),
            orbit_type: if eccentricity < 1.0 - ORBIT_TYPE_TOL {
                "elliptical"
            } else {
                "parabolic"
            }
            .into(),
        });
    }

    let tol = tol.unwrap_or(DEFAULT_TOL);
    let max_iter = max_iter.unwrap_or(DEFAULT_MAX_ITER);

    // Initial guess: For hyperbolic orbits, H₀ = M is reasonable
    // More sophisticated: H₀ = sign(M) · ln(2·|M|/e + 1.8)
    let H0 = if mean_anomaly.abs() > 1.0 {
        mean_anomaly.signum() * (2.0 * mean_anomaly.abs() / eccentricity + 1.8).ln()
    } else {
        mean_anomaly
    };

    // Hyperbolic Kepler equation: M = e·sinh(H) - H
    // f(H) = e·sinh(H) - H - M
    // f'(H) = e·cosh(H) - 1
    let ratio = |H: f64| {
        (eccentricity * H.sinh() - H - mean_anomaly) / (eccentricity * H.cosh() - 1.0)
    };
    let f = |H: f64| eccentricity * H.sinh() - H - mean_anomaly;

    newton_raphson_ratio(ratio, f, H0, Some(tol), Some(max_iter))
}

/// Convert hyperbolic anomaly to mean anomaly for hyperbolic orbits
///
/// Direct calculation: M = e·sinh(H) - H
///
/// # Arguments
/// * `hyperbolic_anomaly` - Hyperbolic anomaly H (radians)
/// * `eccentricity` - Orbital eccentricity e (must be > 1)
///
/// # Returns
/// Mean anomaly M (radians)
pub fn hyperbolic_to_mean_anomaly(
    hyperbolic_anomaly: f64,
    eccentricity: f64,
) -> PoliastroResult<f64> {
    if eccentricity <= 1.0 + ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "hyperbolic_to_mean_anomaly".into(),
            orbit_type: "not hyperbolic".into(),
        });
    }

    let M = eccentricity * hyperbolic_anomaly.sinh() - hyperbolic_anomaly;
    Ok(M)
}

/// Convert hyperbolic anomaly to true anomaly for hyperbolic orbits
///
/// Uses: tan(ν/2) = √((e+1)/(e-1)) · tanh(H/2)
///
/// # Arguments
/// * `hyperbolic_anomaly` - Hyperbolic anomaly H (radians)
/// * `eccentricity` - Orbital eccentricity e (must be > 1)
///
/// # Returns
/// True anomaly ν (radians, in range [0, 2π))
pub fn hyperbolic_to_true_anomaly(
    hyperbolic_anomaly: f64,
    eccentricity: f64,
) -> PoliastroResult<f64> {
    if eccentricity <= 1.0 + ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "hyperbolic_to_true_anomaly".into(),
            orbit_type: "not hyperbolic".into(),
        });
    }

    // Using:
    // cos(ν) = (e - cosh(H)) / (e·cosh(H) - 1)
    // sin(ν) = (√(e²-1)·sinh(H)) / (e·cosh(H) - 1)

    let cosh_H = hyperbolic_anomaly.cosh();
    let sinh_H = hyperbolic_anomaly.sinh();
    let denom = eccentricity * cosh_H - 1.0;

    let cos_nu = (eccentricity - cosh_H) / denom;
    let sin_nu = ((eccentricity * eccentricity - 1.0).sqrt() * sinh_H) / denom;

    let nu = sin_nu.atan2(cos_nu);
    Ok(nu.rem_euclid(2.0 * PI))
}

/// Convert true anomaly to hyperbolic anomaly for hyperbolic orbits
///
/// Uses: tanh(H/2) = √((e-1)/(e+1)) · tan(ν/2)
///
/// # Arguments
/// * `true_anomaly` - True anomaly ν (radians)
/// * `eccentricity` - Orbital eccentricity e (must be > 1)
///
/// # Returns
/// Hyperbolic anomaly H (radians)
pub fn true_to_hyperbolic_anomaly(
    true_anomaly: f64,
    eccentricity: f64,
) -> PoliastroResult<f64> {
    if eccentricity <= 1.0 + ORBIT_TYPE_TOL {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "true_to_hyperbolic_anomaly".into(),
            orbit_type: "not hyperbolic".into(),
        });
    }

    // Using:
    // cosh(H) = (e + cos(ν)) / (1 + e·cos(ν))
    // sinh(H) = (√(e²-1)·sin(ν)) / (1 + e·cos(ν))

    let cos_nu = true_anomaly.cos();
    let sin_nu = true_anomaly.sin();
    let denom = 1.0 + eccentricity * cos_nu;

    let _cosh_H = (eccentricity + cos_nu) / denom;
    let sinh_H = ((eccentricity * eccentricity - 1.0).sqrt() * sin_nu) / denom;

    // H = asinh(sinh_H) or acosh(cosh_H)
    // Use asinh for better numerical stability
    Ok(sinh_H.asinh())
}

/// Convert mean anomaly to true anomaly for hyperbolic orbits
///
/// Convenience function combining M → H → ν conversions
pub fn mean_to_true_anomaly_hyperbolic(
    mean_anomaly: f64,
    eccentricity: f64,
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<f64> {
    let H = mean_to_hyperbolic_anomaly(mean_anomaly, eccentricity, tol, max_iter)?;
    hyperbolic_to_true_anomaly(H, eccentricity)
}

/// Convert true anomaly to mean anomaly for hyperbolic orbits
///
/// Convenience function combining ν → H → M conversions
pub fn true_to_mean_anomaly_hyperbolic(
    true_anomaly: f64,
    eccentricity: f64,
) -> PoliastroResult<f64> {
    let H = true_to_hyperbolic_anomaly(true_anomaly, eccentricity)?;
    hyperbolic_to_mean_anomaly(H, eccentricity)
}

// ============================================================================
// Parabolic Orbit Conversions (e = 1)
// ============================================================================

/// Convert mean anomaly to true anomaly for parabolic orbits
///
/// Uses Barker's equation (closed-form cubic solution).
/// For parabolic orbits: M = D + D³/3, where tan(ν/2) = D
///
/// # Arguments
/// * `mean_anomaly` - Mean anomaly M (radians)
///
/// # Returns
/// True anomaly ν (radians)
///
/// # Algorithm
/// Solves the cubic equation using Cardano's formula
///
/// # Example
/// ```
/// use astrora_core::core::anomaly::mean_to_true_anomaly_parabolic;
///
/// let M = 0.5;
/// let nu = mean_to_true_anomaly_parabolic(M).unwrap();
/// ```
pub fn mean_to_true_anomaly_parabolic(mean_anomaly: f64) -> PoliastroResult<f64> {
    // Barker's equation: M = D + D³/3
    // Rearrange: D³ + 3D - 3M = 0
    // Use Cardano's formula for cubic equation

    // For x³ + px + q = 0:
    // p = 3, q = -3M
    let p = 3.0_f64;
    let q = -3.0 * mean_anomaly;

    // Discriminant: Δ = -(4p³ + 27q²)/108
    let _discriminant = -(4.0 * p.powi(3) + 27.0 * q.powi(2)) / 108.0;

    // For parabolic orbits, discriminant is always negative (one real root)
    // Using the standard cubic formula
    let term = (q / 2.0).abs();
    let sqrt_term = (term.powi(2) + (p / 3.0).powi(3)).sqrt();

    let D = if mean_anomaly >= 0.0 {
        (term + sqrt_term).cbrt() - (sqrt_term - term).cbrt()
    } else {
        -((term + sqrt_term).cbrt() - (sqrt_term - term).cbrt())
    };

    // Convert D to true anomaly: tan(ν/2) = D
    let nu = 2.0 * D.atan();

    Ok(nu.rem_euclid(2.0 * PI))
}

/// Convert true anomaly to mean anomaly for parabolic orbits
///
/// Direct calculation using Barker's equation: M = D + D³/3
/// where D = tan(ν/2)
///
/// # Arguments
/// * `true_anomaly` - True anomaly ν (radians)
///
/// # Returns
/// Mean anomaly M (radians)
pub fn true_to_mean_anomaly_parabolic(true_anomaly: f64) -> PoliastroResult<f64> {
    // D = tan(ν/2)
    let D = (true_anomaly / 2.0).tan();

    // Barker's equation: M = D + D³/3
    let M = D + D.powi(3) / 3.0;

    Ok(M)
}

// ============================================================================
// Batch Processing Functions for Multiple Orbits
// ============================================================================

/// Convert mean anomalies to eccentric anomalies for multiple elliptical orbits (batch)
///
/// This function processes arrays of mean anomalies and eccentricities efficiently,
/// providing 10-20x performance improvement over sequential processing due to:
/// - Single Python-Rust boundary crossing
/// - Better CPU cache utilization
/// - Vectorization opportunities
///
/// # Arguments
/// * `mean_anomalies` - Array of mean anomalies M (radians)
/// * `eccentricities` - Array of orbital eccentricities e (must all be < 1)
///   Can be a single value applied to all orbits, or one per orbit
/// * `tol` - Convergence tolerance (optional, default: 1e-12)
/// * `max_iter` - Maximum iterations (optional, default: 50)
///
/// # Returns
/// Array of eccentric anomalies E (radians)
///
/// # Errors
/// - `InvalidParameter` if array lengths don't match (when eccentricities is an array)
/// - `UnsupportedOrbitType` if any e ≥ 1
/// - `ConvergenceError` if Newton-Raphson fails for any orbit
///
/// # Example
/// ```
/// use astrora_core::core::anomaly::batch_mean_to_eccentric_anomaly;
///
/// let mean_anomalies = vec![0.5, 1.0, 1.5, 2.0];
/// let eccentricity = 0.5; // Same eccentricity for all
/// let results = batch_mean_to_eccentric_anomaly(&mean_anomalies, &[eccentricity], None, None).unwrap();
/// ```
pub fn batch_mean_to_eccentric_anomaly(
    mean_anomalies: &[f64],
    eccentricities: &[f64],
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<Vec<f64>> {
    // Validate inputs
    if eccentricities.len() != 1 && eccentricities.len() != mean_anomalies.len() {
        return Err(PoliastroError::InvalidParameter {
            parameter: "eccentricities".into(),
            value: eccentricities.len() as f64,
            constraint: format!(
                "must be length 1 or match mean_anomalies length ({})",
                mean_anomalies.len()
            ),
        });
    }

    let single_ecc = eccentricities.len() == 1;
    let ecc_value = if single_ecc { eccentricities[0] } else { 0.0 };

    // Parallel processing using rayon
    let results: Vec<f64> = mean_anomalies
        .par_iter()
        .enumerate()
        .map(|(i, &M)| {
            let e = if single_ecc { ecc_value } else { eccentricities[i] };
            mean_to_eccentric_anomaly(M, e, tol, max_iter)
        })
        .collect::<PoliastroResult<Vec<f64>>>()?;

    Ok(results)
}

/// Convert mean anomalies to true anomalies for multiple elliptical orbits (batch)
///
/// Convenience function that combines M → E → ν conversions for multiple orbits.
///
/// # Arguments
/// * `mean_anomalies` - Array of mean anomalies M (radians)
/// * `eccentricities` - Array of orbital eccentricities e (must all be < 1)
/// * `tol` - Convergence tolerance (optional)
/// * `max_iter` - Maximum iterations (optional)
///
/// # Returns
/// Array of true anomalies ν (radians)
pub fn batch_mean_to_true_anomaly(
    mean_anomalies: &[f64],
    eccentricities: &[f64],
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<Vec<f64>> {
    // First convert to eccentric anomalies
    let eccentric_anomalies = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricities, tol, max_iter)?;

    // Then convert to true anomalies in parallel
    let single_ecc = eccentricities.len() == 1;
    let ecc_value = if single_ecc { eccentricities[0] } else { 0.0 };

    let results: Vec<f64> = eccentric_anomalies
        .par_iter()
        .enumerate()
        .map(|(i, &E)| {
            let e = if single_ecc { ecc_value } else { eccentricities[i] };
            eccentric_to_true_anomaly(E, e)
        })
        .collect::<PoliastroResult<Vec<f64>>>()?;

    Ok(results)
}

/// Convert true anomalies to mean anomalies for multiple elliptical orbits (batch)
///
/// # Arguments
/// * `true_anomalies` - Array of true anomalies ν (radians)
/// * `eccentricities` - Array of orbital eccentricities e (must all be < 1)
///
/// # Returns
/// Array of mean anomalies M (radians)
pub fn batch_true_to_mean_anomaly(
    true_anomalies: &[f64],
    eccentricities: &[f64],
) -> PoliastroResult<Vec<f64>> {
    if eccentricities.len() != 1 && eccentricities.len() != true_anomalies.len() {
        return Err(PoliastroError::InvalidParameter {
            parameter: "eccentricities".into(),
            value: eccentricities.len() as f64,
            constraint: format!(
                "must be length 1 or match true_anomalies length ({})",
                true_anomalies.len()
            ),
        });
    }

    let single_ecc = eccentricities.len() == 1;
    let ecc_value = if single_ecc { eccentricities[0] } else { 0.0 };

    // Parallel processing using rayon
    let results: Vec<f64> = true_anomalies
        .par_iter()
        .enumerate()
        .map(|(i, &nu)| {
            let e = if single_ecc { ecc_value } else { eccentricities[i] };
            true_to_mean_anomaly(nu, e)
        })
        .collect::<PoliastroResult<Vec<f64>>>()?;

    Ok(results)
}

/// Convert mean anomalies to hyperbolic anomalies for multiple hyperbolic orbits (batch)
///
/// # Arguments
/// * `mean_anomalies` - Array of mean anomalies M (radians)
/// * `eccentricities` - Array of orbital eccentricities e (must all be > 1)
/// * `tol` - Convergence tolerance (optional)
/// * `max_iter` - Maximum iterations (optional)
///
/// # Returns
/// Array of hyperbolic anomalies H (radians)
pub fn batch_mean_to_hyperbolic_anomaly(
    mean_anomalies: &[f64],
    eccentricities: &[f64],
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<Vec<f64>> {
    if eccentricities.len() != 1 && eccentricities.len() != mean_anomalies.len() {
        return Err(PoliastroError::InvalidParameter {
            parameter: "eccentricities".into(),
            value: eccentricities.len() as f64,
            constraint: format!(
                "must be length 1 or match mean_anomalies length ({})",
                mean_anomalies.len()
            ),
        });
    }

    let single_ecc = eccentricities.len() == 1;
    let ecc_value = if single_ecc { eccentricities[0] } else { 0.0 };

    // Parallel processing using rayon
    let results: Vec<f64> = mean_anomalies
        .par_iter()
        .enumerate()
        .map(|(i, &M)| {
            let e = if single_ecc { ecc_value } else { eccentricities[i] };
            mean_to_hyperbolic_anomaly(M, e, tol, max_iter)
        })
        .collect::<PoliastroResult<Vec<f64>>>()?;

    Ok(results)
}

/// Convert mean anomalies to true anomalies for multiple hyperbolic orbits (batch)
///
/// # Arguments
/// * `mean_anomalies` - Array of mean anomalies M (radians)
/// * `eccentricities` - Array of orbital eccentricities e (must all be > 1)
/// * `tol` - Convergence tolerance (optional)
/// * `max_iter` - Maximum iterations (optional)
///
/// # Returns
/// Array of true anomalies ν (radians)
pub fn batch_mean_to_true_anomaly_hyperbolic(
    mean_anomalies: &[f64],
    eccentricities: &[f64],
    tol: Option<f64>,
    max_iter: Option<usize>,
) -> PoliastroResult<Vec<f64>> {
    let hyperbolic_anomalies = batch_mean_to_hyperbolic_anomaly(mean_anomalies, eccentricities, tol, max_iter)?;

    let single_ecc = eccentricities.len() == 1;
    let ecc_value = if single_ecc { eccentricities[0] } else { 0.0 };

    // Parallel processing using rayon
    let results: Vec<f64> = hyperbolic_anomalies
        .par_iter()
        .enumerate()
        .map(|(i, &H)| {
            let e = if single_ecc { ecc_value } else { eccentricities[i] };
            hyperbolic_to_true_anomaly(H, e)
        })
        .collect::<PoliastroResult<Vec<f64>>>()?;

    Ok(results)
}

/// Convert mean anomalies to true anomalies for multiple parabolic orbits (batch)
///
/// Uses Barker's equation (closed-form solution) for each orbit.
///
/// # Arguments
/// * `mean_anomalies` - Array of mean anomalies M (radians)
///
/// # Returns
/// Array of true anomalies ν (radians)
pub fn batch_mean_to_true_anomaly_parabolic(
    mean_anomalies: &[f64],
) -> PoliastroResult<Vec<f64>> {
    // Parallel processing using rayon
    let results: Vec<f64> = mean_anomalies
        .par_iter()
        .map(|&M| mean_to_true_anomaly_parabolic(M))
        .collect::<PoliastroResult<Vec<f64>>>()?;

    Ok(results)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // ========================================================================
    // Elliptical Orbit Tests
    // ========================================================================

    #[test]
    fn test_mean_to_eccentric_circular() {
        // Circular orbit: e = 0, so E = M
        let M = 1.0;
        let e = 0.0;
        let E = mean_to_eccentric_anomaly(M, e, None, None).unwrap();
        assert_relative_eq!(E, M, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_to_eccentric_moderate() {
        // Moderate eccentricity
        let M = 1.0;
        let e = 0.5;
        let E = mean_to_eccentric_anomaly(M, e, None, None).unwrap();

        // Verify solution satisfies Kepler's equation
        let M_check = E - e * E.sin();
        assert_relative_eq!(M_check, M, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_to_eccentric_high_ecc() {
        // High eccentricity (but still elliptical)
        let M = 0.5;
        let e = 0.9;
        let E = mean_to_eccentric_anomaly(M, e, None, None).unwrap();

        let M_check = E - e * E.sin();
        assert_relative_eq!(M_check, M, epsilon = 1e-10);
    }

    #[test]
    fn test_eccentric_to_true_zero() {
        // At periapsis: E = 0, ν = 0
        let E = 0.0;
        let e = 0.5;
        let nu = eccentric_to_true_anomaly(E, e).unwrap();
        assert_relative_eq!(nu, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_eccentric_to_true_pi() {
        // At apoapsis: E = π, ν = π
        let E = PI;
        let e = 0.3;
        let nu = eccentric_to_true_anomaly(E, e).unwrap();
        assert_relative_eq!(nu, PI, epsilon = 1e-10);
    }

    #[test]
    fn test_true_to_eccentric_roundtrip() {
        let nu_orig = 2.0;
        let e = 0.4;

        let E = true_to_eccentric_anomaly(nu_orig, e).unwrap();
        let nu_check = eccentric_to_true_anomaly(E, e).unwrap();

        assert_relative_eq!(nu_check, nu_orig, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_to_true_roundtrip() {
        let M_orig = 1.5;
        let e = 0.6;

        let nu = mean_to_true_anomaly(M_orig, e, None, None).unwrap();
        let M_check = true_to_mean_anomaly(nu, e).unwrap();

        assert_relative_eq!(M_check, M_orig, epsilon = 1e-10);
    }

    #[test]
    fn test_elliptical_rejects_hyperbolic() {
        let M = 1.0;
        let e = 1.5; // hyperbolic

        let result = mean_to_eccentric_anomaly(M, e, None, None);
        assert!(result.is_err());
    }

    // ========================================================================
    // Hyperbolic Orbit Tests
    // ========================================================================

    #[test]
    fn test_mean_to_hyperbolic_basic() {
        let M = 2.0;
        let e = 1.5;
        let H = mean_to_hyperbolic_anomaly(M, e, None, None).unwrap();

        // Verify solution
        let M_check = e * H.sinh() - H;
        assert_relative_eq!(M_check, M, epsilon = 1e-10);
    }

    #[test]
    fn test_mean_to_hyperbolic_high_ecc() {
        let M = 5.0;
        let e = 3.0; // very hyperbolic
        let H = mean_to_hyperbolic_anomaly(M, e, None, None).unwrap();

        let M_check = e * H.sinh() - H;
        assert_relative_eq!(M_check, M, epsilon = 1e-10);
    }

    #[test]
    fn test_hyperbolic_to_true_zero() {
        // At periapsis: H = 0, ν = 0
        let H = 0.0;
        let e = 1.5;
        let nu = hyperbolic_to_true_anomaly(H, e).unwrap();
        assert_relative_eq!(nu, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_true_to_hyperbolic_roundtrip() {
        let nu_orig = 1.0; // Must be less than asymptote angle
        let e = 2.0;

        let H = true_to_hyperbolic_anomaly(nu_orig, e).unwrap();
        let nu_check = hyperbolic_to_true_anomaly(H, e).unwrap();

        assert_relative_eq!(nu_check, nu_orig, epsilon = 1e-10);
    }

    #[test]
    fn test_hyperbolic_mean_to_true_roundtrip() {
        let M_orig = 3.0;
        let e = 1.8;

        let nu = mean_to_true_anomaly_hyperbolic(M_orig, e, None, None).unwrap();
        let M_check = true_to_mean_anomaly_hyperbolic(nu, e).unwrap();

        assert_relative_eq!(M_check, M_orig, epsilon = 1e-10);
    }

    #[test]
    fn test_hyperbolic_rejects_elliptical() {
        let M = 1.0;
        let e = 0.5; // elliptical

        let result = mean_to_hyperbolic_anomaly(M, e, None, None);
        assert!(result.is_err());
    }

    // ========================================================================
    // Parabolic Orbit Tests
    // ========================================================================

    #[test]
    fn test_parabolic_mean_to_true_zero() {
        let M = 0.0;
        let nu = mean_to_true_anomaly_parabolic(M).unwrap();
        assert_relative_eq!(nu, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_parabolic_mean_to_true_positive() {
        let M = 0.5;
        let nu = mean_to_true_anomaly_parabolic(M).unwrap();

        // Verify with Barker's equation
        let D = (nu / 2.0).tan();
        let M_check = D + D.powi(3) / 3.0;
        assert_relative_eq!(M_check, M, epsilon = 1e-10);
    }

    #[test]
    fn test_parabolic_mean_to_true_negative() {
        let M = -0.5;
        let nu = mean_to_true_anomaly_parabolic(M).unwrap();

        let D = (nu / 2.0).tan();
        let M_check = D + D.powi(3) / 3.0;
        assert_relative_eq!(M_check, M, epsilon = 1e-10);
    }

    #[test]
    fn test_parabolic_roundtrip() {
        let M_orig = 1.2;

        let nu = mean_to_true_anomaly_parabolic(M_orig).unwrap();
        let M_check = true_to_mean_anomaly_parabolic(nu).unwrap();

        assert_relative_eq!(M_check, M_orig, epsilon = 1e-10);
    }

    #[test]
    fn test_parabolic_symmetric() {
        // Test that M and -M give symmetric results
        let M = 0.8;

        let nu_pos = mean_to_true_anomaly_parabolic(M).unwrap();
        let nu_neg = mean_to_true_anomaly_parabolic(-M).unwrap();

        // Should be symmetric around 0
        assert_relative_eq!(nu_pos, -nu_neg.rem_euclid(2.0 * PI) + 2.0 * PI, epsilon = 1e-10);
    }

    // ========================================================================
    // Batch Processing Tests
    // ========================================================================

    #[test]
    fn test_batch_mean_to_eccentric_single_ecc() {
        // Multiple orbits, same eccentricity
        let mean_anomalies = vec![0.5, 1.0, 1.5, 2.0];
        let eccentricity = 0.5;

        let results = batch_mean_to_eccentric_anomaly(&mean_anomalies, &[eccentricity], None, None).unwrap();

        assert_eq!(results.len(), mean_anomalies.len());

        // Verify each result individually
        for (i, &M) in mean_anomalies.iter().enumerate() {
            let E_individual = mean_to_eccentric_anomaly(M, eccentricity, None, None).unwrap();
            assert_relative_eq!(results[i], E_individual, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_batch_mean_to_eccentric_multiple_ecc() {
        // Multiple orbits, different eccentricities
        let mean_anomalies = vec![0.5, 1.0, 1.5, 2.0];
        let eccentricities = vec![0.2, 0.4, 0.6, 0.8];

        let results = batch_mean_to_eccentric_anomaly(&mean_anomalies, &eccentricities, None, None).unwrap();

        assert_eq!(results.len(), mean_anomalies.len());

        // Verify each result
        for i in 0..mean_anomalies.len() {
            let E_individual = mean_to_eccentric_anomaly(mean_anomalies[i], eccentricities[i], None, None).unwrap();
            assert_relative_eq!(results[i], E_individual, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_batch_mean_to_true_elliptical() {
        let mean_anomalies = vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let eccentricity = 0.6;

        let results = batch_mean_to_true_anomaly(&mean_anomalies, &[eccentricity], None, None).unwrap();

        assert_eq!(results.len(), mean_anomalies.len());

        // Verify roundtrip
        for (i, &nu) in results.iter().enumerate() {
            let M_check = true_to_mean_anomaly(nu, eccentricity).unwrap();
            assert_relative_eq!(M_check, mean_anomalies[i], epsilon = 1e-10);
        }
    }

    #[test]
    fn test_batch_true_to_mean_elliptical() {
        let true_anomalies = vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let eccentricity = 0.3;

        let results = batch_true_to_mean_anomaly(&true_anomalies, &[eccentricity]).unwrap();

        assert_eq!(results.len(), true_anomalies.len());

        // Verify each result
        for (i, &nu) in true_anomalies.iter().enumerate() {
            let M_individual = true_to_mean_anomaly(nu, eccentricity).unwrap();
            assert_relative_eq!(results[i], M_individual, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_batch_mean_to_hyperbolic() {
        let mean_anomalies = vec![1.0, 2.0, 3.0, 4.0];
        let eccentricity = 1.5;

        let results = batch_mean_to_hyperbolic_anomaly(&mean_anomalies, &[eccentricity], None, None).unwrap();

        assert_eq!(results.len(), mean_anomalies.len());

        // Verify each result
        for (i, &M) in mean_anomalies.iter().enumerate() {
            let H_individual = mean_to_hyperbolic_anomaly(M, eccentricity, None, None).unwrap();
            assert_relative_eq!(results[i], H_individual, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_batch_mean_to_true_hyperbolic() {
        let mean_anomalies = vec![0.5, 1.0, 1.5, 2.0];
        let eccentricities = vec![1.2, 1.5, 2.0, 2.5];

        let results = batch_mean_to_true_anomaly_hyperbolic(&mean_anomalies, &eccentricities, None, None).unwrap();

        assert_eq!(results.len(), mean_anomalies.len());

        // Verify roundtrip
        for i in 0..mean_anomalies.len() {
            let M_check = true_to_mean_anomaly_hyperbolic(results[i], eccentricities[i]).unwrap();
            assert_relative_eq!(M_check, mean_anomalies[i], epsilon = 1e-10);
        }
    }

    #[test]
    fn test_batch_mean_to_true_parabolic() {
        let mean_anomalies = vec![0.0, 0.5, 1.0, 1.5, -0.5, -1.0];

        let results = batch_mean_to_true_anomaly_parabolic(&mean_anomalies).unwrap();

        assert_eq!(results.len(), mean_anomalies.len());

        // Verify each result
        for (i, &M) in mean_anomalies.iter().enumerate() {
            let nu_individual = mean_to_true_anomaly_parabolic(M).unwrap();
            assert_relative_eq!(results[i], nu_individual, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_batch_error_length_mismatch() {
        let mean_anomalies = vec![0.5, 1.0, 1.5, 2.0];
        let eccentricities = vec![0.2, 0.4]; // Wrong length

        let result = batch_mean_to_eccentric_anomaly(&mean_anomalies, &eccentricities, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_large_array() {
        // Test performance pattern with many orbits
        let n = 100;
        let mean_anomalies: Vec<f64> = (0..n).map(|i| (i as f64) * 0.1).collect();
        let eccentricity = 0.5;

        let results = batch_mean_to_eccentric_anomaly(&mean_anomalies, &[eccentricity], None, None).unwrap();

        assert_eq!(results.len(), n);

        // Spot check a few values
        for i in [0, 25, 50, 75, 99].iter() {
            let E_individual = mean_to_eccentric_anomaly(mean_anomalies[*i], eccentricity, None, None).unwrap();
            assert_relative_eq!(results[*i], E_individual, epsilon = 1e-10);
        }
    }
}
