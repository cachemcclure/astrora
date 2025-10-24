//! State Transition Matrix (STM) propagation for orbit determination
//!
//! This module implements state transition matrix propagation for orbital mechanics.
//! The STM maps how small deviations in initial conditions propagate forward in time,
//! which is essential for:
//! - Orbit determination and estimation
//! - Uncertainty propagation
//! - Sensitivity analysis
//! - Differential corrections (shooting methods)
//!
//! # Mathematical Background
//!
//! For a dynamical system dx/dt = f(x,t), the STM Φ(t,t₀) satisfies:
//!
//! dΦ/dt = A(t)·Φ(t,t₀)
//!
//! where A(t) = ∂f/∂x is the Jacobian matrix, and Φ(t₀,t₀) = I
//!
//! The STM relates state perturbations: δx(t) ≈ Φ(t,t₀)·δx(t₀)
//!
//! # References
//! - Vallado, D. A. "Fundamentals of Astrodynamics and Applications", 5th Ed., p. 748
//! - Tapley, Schutz, Born "Statistical Orbit Determination" (2004), Ch. 3
//! - nyx-space STM implementation: <https://nyxspace.com/nyxspace/MathSpec/optimization/stm/>

use crate::core::error::PoliastroResult;
use crate::core::linalg::Vector3;
use nalgebra as na;

/// Compute the Jacobian matrix for two-body orbital dynamics
///
/// For the equations of motion:
/// dr/dt = v
/// dv/dt = a = -μ/r³ · r
///
/// The Jacobian matrix A = ∂f/∂x has the structure:
/// ```text
/// A = [ 0₃ₓ₃   I₃ₓ₃  ]
///     [ ∂a/∂r  0₃ₓ₃  ]
/// ```
///
/// where ∂a/∂r is the 3×3 matrix of partial derivatives of acceleration
/// with respect to position:
///
/// ∂a/∂r = -μ/r³·I₃ₓ₃ + 3μ/r⁵·(r⊗r)
///
/// Here (r⊗r) is the outer product of the position vector.
///
/// # Arguments
/// * `r` - Position vector (m)
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// 6×6 Jacobian matrix A
///
/// # Example
/// ```ignore
/// use astrora::propagators::stm::jacobian_two_body;
/// use astrora::core::linalg::Vector3;
/// use astrora::core::constants::GM_EARTH;
///
/// let r = Vector3::new(7000e3, 0.0, 0.0);
/// let A = jacobian_two_body(&r, GM_EARTH);
/// ```
pub fn jacobian_two_body(r: &Vector3, mu: f64) -> na::Matrix6<f64> {
    let r_mag = r.norm();
    let r_mag3 = r_mag * r_mag * r_mag;
    let r_mag5 = r_mag3 * r_mag * r_mag;

    // Upper left: 0₃ₓ₃
    // Upper right: I₃ₓ₃
    // Lower right: 0₃ₓ₃
    // Lower left: ∂a/∂r

    // Compute ∂a/∂r = -μ/r³·I + 3μ/r⁵·(r⊗r)
    let coeff1 = -mu / r_mag3;
    let coeff2 = 3.0 * mu / r_mag5;

    // Outer product r⊗r
    let rx2 = r.x * r.x;
    let ry2 = r.y * r.y;
    let rz2 = r.z * r.z;
    let rxy = r.x * r.y;
    let rxz = r.x * r.z;
    let ryz = r.y * r.z;

    // Build ∂a/∂r matrix (3×3)
    #[rustfmt::skip]
    let da_dr = na::Matrix3::new(
        coeff1 + coeff2 * rx2,  coeff2 * rxy,           coeff2 * rxz,
        coeff2 * rxy,           coeff1 + coeff2 * ry2,  coeff2 * ryz,
        coeff2 * rxz,           coeff2 * ryz,           coeff1 + coeff2 * rz2,
    );

    // Assemble full 6×6 Jacobian
    // A = [ 0  I ]
    //     [ ∂a/∂r  0 ]
    #[rustfmt::skip]
    let jacobian = na::Matrix6::new(
        0.0, 0.0, 0.0,  1.0, 0.0, 0.0,
        0.0, 0.0, 0.0,  0.0, 1.0, 0.0,
        0.0, 0.0, 0.0,  0.0, 0.0, 1.0,
        da_dr[(0,0)], da_dr[(0,1)], da_dr[(0,2)],  0.0, 0.0, 0.0,
        da_dr[(1,0)], da_dr[(1,1)], da_dr[(1,2)],  0.0, 0.0, 0.0,
        da_dr[(2,0)], da_dr[(2,1)], da_dr[(2,2)],  0.0, 0.0, 0.0,
    );

    jacobian
}

/// Compute the Jacobian matrix for J2-perturbed orbital dynamics
///
/// Extends the two-body Jacobian to include J2 oblateness perturbations.
/// The J2 acceleration is:
///
/// a_J2 = (3/2)·J2·μ·R²/r⁴ · [ (x/r)·(5z²/r² - 1), (y/r)·(5z²/r² - 1), (z/r)·(5z²/r² - 3) ]
///
/// The partial derivatives ∂a_J2/∂r must be added to the two-body Jacobian.
///
/// # Arguments
/// * `r` - Position vector (m)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient (dimensionless)
/// * `R` - Body equatorial radius (m)
///
/// # Returns
/// 6×6 Jacobian matrix A including J2 effects
///
/// # Example
/// ```ignore
/// use astrora::propagators::stm::jacobian_j2;
/// use astrora::core::linalg::Vector3;
/// use astrora::core::constants::{GM_EARTH, J2_EARTH, R_EARTH};
///
/// let r = Vector3::new(7000e3, 0.0, 0.0);
/// let A = jacobian_j2(&r, GM_EARTH, J2_EARTH, R_EARTH);
/// ```
pub fn jacobian_j2(r: &Vector3, mu: f64, j2: f64, R: f64) -> na::Matrix6<f64> {
    // Start with two-body Jacobian
    let mut jacobian = jacobian_two_body(r, mu);

    // Add J2 contribution to lower-left 3×3 block
    let x = r.x;
    let y = r.y;
    let z = r.z;

    let r_mag = r.norm();
    let r2 = r_mag * r_mag;
    let r4 = r2 * r2;
    let r6 = r4 * r2;
    let r8 = r6 * r2;

    let z2 = z * z;

    // Common coefficient k = (3/2) * J2 * μ * R²
    let k = 1.5 * j2 * mu * R * R;

    // The J2 acceleration components are:
    // ax = k/r⁴ · (x/r) · (5z²/r² - 1)
    // ay = k/r⁴ · (y/r) · (5z²/r² - 1)
    // az = k/r⁴ · (z/r) · (5z²/r² - 3)

    // We need ∂a_J2/∂r
    // This involves computing partial derivatives of each acceleration component
    // with respect to x, y, z

    // These are complex expressions. For implementation, we use symbolic differentiation results.
    // The partial derivatives are:

    let z2_r2 = z2 / r2;
    let factor_xy = 5.0 * z2_r2 - 1.0;
    let factor_z = 5.0 * z2_r2 - 3.0;

    // ∂(a_J2)/∂x
    let dax_dx = k * (
        (factor_xy / r4) * (1.0/r_mag - x*x/r_mag/r2)
        + (x / r_mag / r4) * (10.0*z2*(-x)/r4 - x*factor_xy*4.0/r2/r_mag)
    );

    let dax_dy = k * (
        (factor_xy / r4) * (-x*y/r_mag/r2)
        + (x / r_mag / r4) * (10.0*z2*(-y)/r4 - y*factor_xy*4.0/r2/r_mag)
    );

    let dax_dz = k * (
        (factor_xy / r4) * (-x*z/r_mag/r2)
        + (x / r_mag / r4) * (10.0*z2*(-z)/r4 + 10.0*z/r2 - z*factor_xy*4.0/r2/r_mag)
    );

    // ∂(a_J2)/∂y
    let day_dx = k * (
        (factor_xy / r4) * (-y*x/r_mag/r2)
        + (y / r_mag / r4) * (10.0*z2*(-x)/r4 - x*factor_xy*4.0/r2/r_mag)
    );

    let day_dy = k * (
        (factor_xy / r4) * (1.0/r_mag - y*y/r_mag/r2)
        + (y / r_mag / r4) * (10.0*z2*(-y)/r4 - y*factor_xy*4.0/r2/r_mag)
    );

    let day_dz = k * (
        (factor_xy / r4) * (-y*z/r_mag/r2)
        + (y / r_mag / r4) * (10.0*z2*(-z)/r4 + 10.0*z/r2 - z*factor_xy*4.0/r2/r_mag)
    );

    // ∂(a_J2)/∂z
    let daz_dx = k * (
        (factor_z / r4) * (-z*x/r_mag/r2)
        + (z / r_mag / r4) * (10.0*z2*(-x)/r4 - x*factor_z*4.0/r2/r_mag)
    );

    let daz_dy = k * (
        (factor_z / r4) * (-z*y/r_mag/r2)
        + (z / r_mag / r4) * (10.0*z2*(-y)/r4 - y*factor_z*4.0/r2/r_mag)
    );

    let daz_dz = k * (
        (factor_z / r4) * (1.0/r_mag - z*z/r_mag/r2)
        + (z / r_mag / r4) * (10.0*z2*(-z)/r4 + 10.0*z/r2 - z*factor_z*4.0/r2/r_mag)
    );

    // Add J2 contributions to the lower-left block (rows 3-5, cols 0-2)
    jacobian[(3, 0)] += dax_dx;
    jacobian[(3, 1)] += dax_dy;
    jacobian[(3, 2)] += dax_dz;

    jacobian[(4, 0)] += day_dx;
    jacobian[(4, 1)] += day_dy;
    jacobian[(4, 2)] += day_dz;

    jacobian[(5, 0)] += daz_dx;
    jacobian[(5, 1)] += daz_dy;
    jacobian[(5, 2)] += daz_dz;

    jacobian
}

/// Propagate state with state transition matrix using RK4 integration
///
/// Integrates both the orbital state (position and velocity) and the
/// state transition matrix simultaneously. The STM starts as identity
/// and evolves according to dΦ/dt = A(t)·Φ.
///
/// # Augmented State Vector
///
/// The function integrates a 42-element augmented state:
/// - Elements 0-5: [x, y, z, vx, vy, vz] (orbital state)
/// - Elements 6-41: Φ (6×6 STM, flattened row-major)
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `n_steps` - Number of RK4 sub-steps
///
/// # Returns
/// Tuple of (final_position, final_velocity, final_STM)
///
/// # Example
/// ```ignore
/// use astrora::propagators::stm::propagate_stm_rk4;
/// use astrora::core::linalg::Vector3;
/// use astrora::core::constants::GM_EARTH;
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
/// let (r, v, stm) = propagate_stm_rk4(&r0, &v0, 3600.0, GM_EARTH, 100).unwrap();
///
/// // Use STM to compute effect of 1m perturbation in x
/// let dr0 = nalgebra::Vector6::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
/// let dr = stm * dr0;  // Perturbation propagated to final time
/// ```
pub fn propagate_stm_rk4(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    n_steps: usize,
) -> PoliastroResult<(Vector3, Vector3, na::Matrix6<f64>)> {
    use crate::core::numerical::rk4_step;

    let h = dt / n_steps as f64;

    // Define the augmented dynamics: [state, STM]
    // State: [x, y, z, vx, vy, vz] - 6 elements
    // STM: 6×6 matrix = 36 elements
    // Total: 42 elements
    let dynamics = |_t: f64, augmented: &na::DVector<f64>| -> na::DVector<f64> {
        // Extract state (first 6 elements)
        let r = Vector3::new(augmented[0], augmented[1], augmented[2]);
        let v = Vector3::new(augmented[3], augmented[4], augmented[5]);

        // Extract STM (remaining 36 elements, row-major)
        let mut stm = na::Matrix6::<f64>::zeros();
        for i in 0..6 {
            for j in 0..6 {
                stm[(i, j)] = augmented[6 + i * 6 + j];
            }
        }

        // Compute two-body acceleration
        let r_mag = r.norm();
        let a = -mu / (r_mag * r_mag * r_mag) * r;

        // Compute Jacobian
        let A = jacobian_two_body(&r, mu);

        // STM dynamics: dΦ/dt = A·Φ
        let dstm_dt = A * stm;

        // Build result vector: [v, a, dΦ/dt flattened]
        let mut result = na::DVector::zeros(42);
        result[0] = v.x;
        result[1] = v.y;
        result[2] = v.z;
        result[3] = a.x;
        result[4] = a.y;
        result[5] = a.z;

        // Flatten dΦ/dt
        for i in 0..6 {
            for j in 0..6 {
                result[6 + i * 6 + j] = dstm_dt[(i, j)];
            }
        }

        result
    };

    // Initial augmented state: [r0, v0, I₆ₓ₆]
    let mut augmented = na::DVector::zeros(42);
    augmented[0] = r0.x;
    augmented[1] = r0.y;
    augmented[2] = r0.z;
    augmented[3] = v0.x;
    augmented[4] = v0.y;
    augmented[5] = v0.z;

    // Initialize STM as identity
    for i in 0..6 {
        augmented[6 + i * 6 + i] = 1.0;
    }

    // Integrate
    let mut t = 0.0;
    for _ in 0..n_steps {
        augmented = rk4_step(dynamics, t, &augmented, h);
        t += h;
    }

    // Extract final state
    let r_final = Vector3::new(augmented[0], augmented[1], augmented[2]);
    let v_final = Vector3::new(augmented[3], augmented[4], augmented[5]);

    // Extract final STM
    let mut stm_final = na::Matrix6::<f64>::zeros();
    for i in 0..6 {
        for j in 0..6 {
            stm_final[(i, j)] = augmented[6 + i * 6 + j];
        }
    }

    Ok((r_final, v_final, stm_final))
}

/// Propagate state with STM using adaptive DOPRI5 integration
///
/// Higher accuracy propagation using Dormand-Prince 5(4) adaptive integration.
/// Automatically adjusts step size to maintain specified error tolerance for
/// both the orbital state and the STM.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `tol` - Error tolerance (default: 1e-10)
///
/// # Returns
/// Tuple of (final_position, final_velocity, final_STM)
///
/// # Example
/// ```ignore
/// use astrora::propagators::stm::propagate_stm_dopri5;
/// use astrora::core::linalg::Vector3;
/// use astrora::core::constants::GM_EARTH;
///
/// let r0 = Vector3::new(7000e3, 0.0, 0.0);
/// let v0 = Vector3::new(0.0, 7546.0, 0.0);
/// let (r, v, stm) = propagate_stm_dopri5(&r0, &v0, 3600.0, GM_EARTH, Some(1e-10)).unwrap();
/// ```
pub fn propagate_stm_dopri5(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    tol: Option<f64>,
) -> PoliastroResult<(Vector3, Vector3, na::Matrix6<f64>)> {
    use crate::core::numerical::dopri5_integrate;

    let tol = tol.unwrap_or(1e-10);

    // Define augmented dynamics (same as RK4 version)
    let dynamics = |_t: f64, augmented: &na::DVector<f64>| -> na::DVector<f64> {
        let r = Vector3::new(augmented[0], augmented[1], augmented[2]);
        let v = Vector3::new(augmented[3], augmented[4], augmented[5]);

        let mut stm = na::Matrix6::<f64>::zeros();
        for i in 0..6 {
            for j in 0..6 {
                stm[(i, j)] = augmented[6 + i * 6 + j];
            }
        }

        let r_mag = r.norm();
        let a = -mu / (r_mag * r_mag * r_mag) * r;
        let A = jacobian_two_body(&r, mu);
        let dstm_dt = A * stm;

        let mut result = na::DVector::zeros(42);
        result[0] = v.x;
        result[1] = v.y;
        result[2] = v.z;
        result[3] = a.x;
        result[4] = a.y;
        result[5] = a.z;

        for i in 0..6 {
            for j in 0..6 {
                result[6 + i * 6 + j] = dstm_dt[(i, j)];
            }
        }

        result
    };

    // Initial augmented state
    let mut augmented0 = na::DVector::zeros(42);
    augmented0[0] = r0.x;
    augmented0[1] = r0.y;
    augmented0[2] = r0.z;
    augmented0[3] = v0.x;
    augmented0[4] = v0.y;
    augmented0[5] = v0.z;

    for i in 0..6 {
        augmented0[6 + i * 6 + i] = 1.0;
    }

    // Integrate
    let augmented_final = dopri5_integrate(
        dynamics,
        0.0,
        &augmented0,
        dt,
        dt.abs() / 10.0,
        tol,
        None,
    )?;

    // Extract results
    let r_final = Vector3::new(augmented_final[0], augmented_final[1], augmented_final[2]);
    let v_final = Vector3::new(augmented_final[3], augmented_final[4], augmented_final[5]);

    let mut stm_final = na::Matrix6::<f64>::zeros();
    for i in 0..6 {
        for j in 0..6 {
            stm_final[(i, j)] = augmented_final[6 + i * 6 + j];
        }
    }

    Ok((r_final, v_final, stm_final))
}

/// Propagate state with STM including J2 perturbations using RK4
///
/// Extends STM propagation to include Earth oblateness (J2) effects.
/// The Jacobian includes both two-body and J2 contributions.
///
/// # Arguments
/// * `r0` - Initial position vector (m)
/// * `v0` - Initial velocity vector (m/s)
/// * `dt` - Time step (seconds)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `j2` - Oblateness coefficient
/// * `R` - Body equatorial radius (m)
/// * `n_steps` - Number of RK4 sub-steps
///
/// # Returns
/// Tuple of (final_position, final_velocity, final_STM)
pub fn propagate_stm_j2_rk4(
    r0: &Vector3,
    v0: &Vector3,
    dt: f64,
    mu: f64,
    j2: f64,
    R: f64,
    n_steps: usize,
) -> PoliastroResult<(Vector3, Vector3, na::Matrix6<f64>)> {
    use crate::core::numerical::rk4_step;
    use crate::propagators::perturbations::j2_perturbation;

    let h = dt / n_steps as f64;

    let dynamics = |_t: f64, augmented: &na::DVector<f64>| -> na::DVector<f64> {
        let r = Vector3::new(augmented[0], augmented[1], augmented[2]);
        let v = Vector3::new(augmented[3], augmented[4], augmented[5]);

        let mut stm = na::Matrix6::<f64>::zeros();
        for i in 0..6 {
            for j in 0..6 {
                stm[(i, j)] = augmented[6 + i * 6 + j];
            }
        }

        // Total acceleration (two-body + J2)
        let r_mag = r.norm();
        let a_twobody = -mu / (r_mag * r_mag * r_mag) * r;
        let a_j2 = j2_perturbation(&r, mu, j2, R);
        let a_total = a_twobody + a_j2;

        // Jacobian including J2
        let A = jacobian_j2(&r, mu, j2, R);
        let dstm_dt = A * stm;

        let mut result = na::DVector::zeros(42);
        result[0] = v.x;
        result[1] = v.y;
        result[2] = v.z;
        result[3] = a_total.x;
        result[4] = a_total.y;
        result[5] = a_total.z;

        for i in 0..6 {
            for j in 0..6 {
                result[6 + i * 6 + j] = dstm_dt[(i, j)];
            }
        }

        result
    };

    let mut augmented0 = na::DVector::zeros(42);
    augmented0[0] = r0.x;
    augmented0[1] = r0.y;
    augmented0[2] = r0.z;
    augmented0[3] = v0.x;
    augmented0[4] = v0.y;
    augmented0[5] = v0.z;

    for i in 0..6 {
        augmented0[6 + i * 6 + i] = 1.0;
    }

    let mut augmented = augmented0;
    let mut t = 0.0;
    for _ in 0..n_steps {
        augmented = rk4_step(dynamics, t, &augmented, h);
        t += h;
    }

    let r_final = Vector3::new(augmented[0], augmented[1], augmented[2]);
    let v_final = Vector3::new(augmented[3], augmented[4], augmented[5]);

    let mut stm_final = na::Matrix6::<f64>::zeros();
    for i in 0..6 {
        for j in 0..6 {
            stm_final[(i, j)] = augmented[6 + i * 6 + j];
        }
    }

    Ok((r_final, v_final, stm_final))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::GM_EARTH;
    use approx::assert_relative_eq;

    #[test]
    fn test_jacobian_two_body_structure() {
        // Test that Jacobian has correct structure
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let A = jacobian_two_body(&r, GM_EARTH);

        // Upper left should be zeros
        for i in 0..3 {
            for j in 0..3 {
                assert_relative_eq!(A[(i, j)], 0.0, epsilon = 1e-20);
            }
        }

        // Upper right should be identity
        assert_relative_eq!(A[(0, 3)], 1.0);
        assert_relative_eq!(A[(1, 4)], 1.0);
        assert_relative_eq!(A[(2, 5)], 1.0);
        assert_relative_eq!(A[(0, 4)], 0.0, epsilon = 1e-20);

        // Lower right should be zeros
        for i in 3..6 {
            for j in 3..6 {
                assert_relative_eq!(A[(i, j)], 0.0, epsilon = 1e-20);
            }
        }

        // Lower left should have non-zero partial derivatives
        assert!(A[(3, 0)].abs() > 1e-10);
    }

    #[test]
    fn test_jacobian_symmetry() {
        // For circular orbit on equator, check expected symmetry
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let A = jacobian_two_body(&r, GM_EARTH);

        // On x-axis: ∂ay/∂y should equal ∂az/∂z (symmetry)
        assert_relative_eq!(A[(4, 1)], A[(5, 2)], epsilon = 1e-10);
    }

    #[test]
    fn test_stm_identity_at_zero_time() {
        // STM at t=0 should be identity
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        // Very short time step (essentially t=0)
        let (_, _, stm) = propagate_stm_rk4(&r0, &v0, 1e-10, GM_EARTH, 1).unwrap();

        // Should be very close to identity
        for i in 0..6 {
            for j in 0..6 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_relative_eq!(stm[(i, j)], expected, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_stm_propagation_basic() {
        // Propagate an orbit and verify STM is computed
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);

        let (r, v, stm) = propagate_stm_rk4(&r0, &v0, 600.0, GM_EARTH, 100).unwrap();

        // State should still be reasonable
        assert!(r.norm() > 6000e3);
        assert!(v.norm() > 1000.0);

        // STM should no longer be identity
        let identity_diff = (stm[(0, 0)] - 1.0).abs() + (stm[(0, 1)]).abs();
        assert!(identity_diff > 0.01); // Should have evolved

        // STM determinant should be ~1 (Liouville's theorem for Hamiltonian systems)
        let det = stm.determinant();
        assert_relative_eq!(det.abs(), 1.0, epsilon = 0.1);
    }

    #[test]
    fn test_stm_linearity() {
        // Test that STM correctly maps perturbations
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0;

        // Propagate nominal trajectory
        let (r_nom, v_nom, stm) = propagate_stm_rk4(&r0, &v0, dt, GM_EARTH, 100).unwrap();

        // Propagate perturbed trajectory
        let dr0 = 1000.0; // 1 km perturbation in x
        let r0_pert = Vector3::new(r0.x + dr0, r0.y, r0.z);
        let (r_pert, v_pert, _) = propagate_stm_rk4(&r0_pert, &v0, dt, GM_EARTH, 100).unwrap();

        // Actual difference
        let dr_actual = r_pert - r_nom;
        let dv_actual = v_pert - v_nom;

        // STM prediction
        let delta_x0 = na::Vector6::new(dr0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let delta_x = stm * delta_x0;

        // STM should predict the change reasonably well
        let r_error = (dr_actual.x - delta_x[0]).abs();
        let v_error = (dv_actual.x - delta_x[3]).abs();

        // For small perturbations and short times, error should be small
        assert!(r_error < dr0 * 0.01); // < 1% error
        assert!(v_error < 1.0); // < 1 m/s error
    }

    #[test]
    fn test_stm_dopri5_vs_rk4() {
        // Compare DOPRI5 and RK4 STM propagation
        let r0 = Vector3::new(7000e3, 0.0, 0.0);
        let v0 = Vector3::new(0.0, 7546.0, 0.0);
        let dt = 600.0;

        let (r_rk4, v_rk4, stm_rk4) = propagate_stm_rk4(&r0, &v0, dt, GM_EARTH, 100).unwrap();
        let (r_dopri5, v_dopri5, stm_dopri5) =
            propagate_stm_dopri5(&r0, &v0, dt, GM_EARTH, Some(1e-10)).unwrap();

        // Should be very close
        assert!((r_rk4 - r_dopri5).norm() < 100.0); // < 100 m
        assert!((v_rk4 - v_dopri5).norm() < 0.1); // < 0.1 m/s

        // STM should also be close
        let stm_diff = (stm_rk4 - stm_dopri5).norm();
        assert!(stm_diff < 0.01);
    }

    #[test]
    fn test_jacobian_j2_vs_two_body() {
        // J2 Jacobian should reduce to two-body when j2=0
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let A_twobody = jacobian_two_body(&r, GM_EARTH);
        let A_j2_zero = jacobian_j2(&r, GM_EARTH, 0.0, 6378e3);

        // Should be identical
        for i in 0..6 {
            for j in 0..6 {
                assert_relative_eq!(A_twobody[(i, j)], A_j2_zero[(i, j)], epsilon = 1e-15);
            }
        }
    }

    #[test]
    fn test_stm_j2_propagation() {
        // Test STM propagation with J2
        use crate::core::constants::{J2_EARTH, R_EARTH};

        let r0 = Vector3::new(7000e3, 0.0, 1000e3); // Inclined orbit
        let v0 = Vector3::new(0.0, 7546.0, 100.0);

        let result = propagate_stm_j2_rk4(&r0, &v0, 600.0, GM_EARTH, J2_EARTH, R_EARTH, 100);
        assert!(result.is_ok());

        let (r, v, stm) = result.unwrap();

        // Verify reasonable results
        assert!(r.norm() > 6000e3);
        assert!(v.norm() > 1000.0);

        // STM determinant should still be ~1
        let det = stm.determinant();
        assert_relative_eq!(det.abs(), 1.0, epsilon = 0.2);
    }
}
