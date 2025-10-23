//! Linear algebra types and utilities for orbital mechanics
//!
//! This module provides type aliases and utility functions for common
//! vector and matrix operations in astrodynamics calculations.
//!
//! # Examples
//!
//! ```
//! use astrora_core::core::linalg::{Vector3, Matrix3};
//!
//! // Create a position vector (in meters)
//! let position = Vector3::new(7000_000.0, 0.0, 0.0);
//!
//! // Create a rotation matrix
//! let rotation = Matrix3::identity();
//! let rotated_position = rotation * position;
//! ```

use nalgebra as na;

// ============================================================================
// Type Aliases for Common Vector Types
// ============================================================================

/// 2D vector (typically for specialized calculations)
pub type Vector2 = na::Vector2<f64>;

/// 3D vector for position, velocity, or angular quantities
pub type Vector3 = na::Vector3<f64>;

/// 4D vector (for quaternions or homogeneous coordinates)
pub type Vector4 = na::Vector4<f64>;

/// 6D state vector (position + velocity)
pub type Vector6 = na::Vector6<f64>;

// ============================================================================
// Type Aliases for Common Matrix Types
// ============================================================================

/// 3x3 matrix for rotation and coordinate transformations
pub type Matrix3 = na::Matrix3<f64>;

/// 3x6 matrix for partial derivatives
pub type Matrix3x6 = na::Matrix3x6<f64>;

/// 6x6 matrix for state transition matrices
pub type Matrix6 = na::Matrix6<f64>;

// ============================================================================
// Type Aliases for Geometric Types
// ============================================================================

/// 3D rotation representation
pub type Rotation3 = na::Rotation3<f64>;

/// Unit quaternion for 3D rotations (more numerically stable)
pub type UnitQuaternion = na::UnitQuaternion<f64>;

/// 3D isometry (rotation + translation)
pub type Isometry3 = na::Isometry3<f64>;

// ============================================================================
// Utility Functions
// ============================================================================

/// Create a 6D state vector from position and velocity
///
/// # Arguments
/// * `position` - Position vector [x, y, z] in meters
/// * `velocity` - Velocity vector [vx, vy, vz] in m/s
///
/// # Returns
/// State vector [x, y, z, vx, vy, vz]
#[inline]
pub fn state_vector(position: Vector3, velocity: Vector3) -> Vector6 {
    Vector6::new(
        position.x,
        position.y,
        position.z,
        velocity.x,
        velocity.y,
        velocity.z,
    )
}

/// Extract position from a 6D state vector
///
/// # Arguments
/// * `state` - State vector [x, y, z, vx, vy, vz]
///
/// # Returns
/// Position vector [x, y, z]
#[inline]
pub fn position_from_state(state: &Vector6) -> Vector3 {
    Vector3::new(state[0], state[1], state[2])
}

/// Extract velocity from a 6D state vector
///
/// # Arguments
/// * `state` - State vector [x, y, z, vx, vy, vz]
///
/// # Returns
/// Velocity vector [vx, vy, vz]
#[inline]
pub fn velocity_from_state(state: &Vector6) -> Vector3 {
    Vector3::new(state[3], state[4], state[5])
}

/// Create a rotation matrix from Euler angles (Z-Y-X convention)
///
/// # Arguments
/// * `roll` - Rotation around X-axis (radians)
/// * `pitch` - Rotation around Y-axis (radians)
/// * `yaw` - Rotation around Z-axis (radians)
///
/// # Returns
/// 3x3 rotation matrix
#[inline]
pub fn rotation_from_euler(roll: f64, pitch: f64, yaw: f64) -> Rotation3 {
    Rotation3::from_euler_angles(roll, pitch, yaw)
}

/// Create a rotation matrix from an axis and angle
///
/// # Arguments
/// * `axis` - Rotation axis (unit vector)
/// * `angle` - Rotation angle in radians
///
/// # Returns
/// 3x3 rotation matrix
#[inline]
pub fn rotation_from_axis_angle(axis: &Vector3, angle: f64) -> Rotation3 {
    Rotation3::from_axis_angle(&na::Unit::new_normalize(*axis), angle)
}

/// Compute the cross product of two 3D vectors
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector
///
/// # Returns
/// Cross product a × b
#[inline]
pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
    a.cross(b)
}

/// Compute the dot product of two vectors
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector
///
/// # Returns
/// Dot product a · b
#[inline]
pub fn dot(a: &Vector3, b: &Vector3) -> f64 {
    a.dot(b)
}

/// Compute the norm (magnitude) of a vector
///
/// # Arguments
/// * `v` - Input vector
///
/// # Returns
/// Euclidean norm ||v||
#[inline]
pub fn norm(v: &Vector3) -> f64 {
    v.norm()
}

/// Normalize a vector to unit length
///
/// # Arguments
/// * `v` - Input vector
///
/// # Returns
/// Unit vector in the same direction
///
/// # Panics
/// Panics if the vector has zero magnitude
#[inline]
pub fn normalize(v: &Vector3) -> Vector3 {
    v.normalize()
}

/// Safely normalize a vector, returning None if magnitude is too small
///
/// # Arguments
/// * `v` - Input vector
/// * `min_norm` - Minimum acceptable magnitude (default: 1e-10)
///
/// # Returns
/// Some(unit vector) if successful, None if magnitude is too small
#[inline]
pub fn try_normalize(v: &Vector3, min_norm: f64) -> Option<Vector3> {
    let magnitude = v.norm();
    if magnitude > min_norm {
        Some(v / magnitude)
    } else {
        None
    }
}

/// Create a skew-symmetric matrix from a 3D vector
///
/// This is useful for representing cross products as matrix multiplication:
/// a × b = \[a\]× b
///
/// # Arguments
/// * `v` - Input vector \[x, y, z\]
///
/// # Returns
/// 3x3 skew-symmetric matrix:
/// ```text
/// [  0  -z   y ]
/// [  z   0  -x ]
/// [ -y   x   0 ]
/// ```
#[inline]
pub fn skew_symmetric(v: &Vector3) -> Matrix3 {
    Matrix3::new(
        0.0, -v.z, v.y, //
        v.z, 0.0, -v.x, //
        -v.y, v.x, 0.0, //
    )
}

/// Compute the angle between two vectors
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector
///
/// # Returns
/// Angle in radians [0, π]
#[inline]
pub fn angle_between(a: &Vector3, b: &Vector3) -> f64 {
    let dot = a.dot(b);
    let norms = a.norm() * b.norm();
    (dot / norms).clamp(-1.0, 1.0).acos()
}

/// Create a 6x6 identity matrix (useful for state transition matrices)
#[inline]
pub fn identity_6x6() -> Matrix6 {
    Matrix6::identity()
}

/// Create a 6x6 block matrix from four 3x3 blocks
///
/// # Arguments
/// * `a` - Top-left 3x3 block
/// * `b` - Top-right 3x3 block
/// * `c` - Bottom-left 3x3 block
/// * `d` - Bottom-right 3x3 block
///
/// # Returns
/// 6x6 matrix with the specified block structure
#[inline]
pub fn block_matrix_6x6(a: Matrix3, b: Matrix3, c: Matrix3, d: Matrix3) -> Matrix6 {
    let mut result = Matrix6::zeros();
    result.fixed_view_mut::<3, 3>(0, 0).copy_from(&a);
    result.fixed_view_mut::<3, 3>(0, 3).copy_from(&b);
    result.fixed_view_mut::<3, 3>(3, 0).copy_from(&c);
    result.fixed_view_mut::<3, 3>(3, 3).copy_from(&d);
    result
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_state_vector_construction() {
        let pos = Vector3::new(7000e3, 0.0, 0.0);
        let vel = Vector3::new(0.0, 7500.0, 0.0);
        let state = state_vector(pos, vel);

        assert_eq!(state[0], 7000e3);
        assert_eq!(state[1], 0.0);
        assert_eq!(state[2], 0.0);
        assert_eq!(state[3], 0.0);
        assert_eq!(state[4], 7500.0);
        assert_eq!(state[5], 0.0);
    }

    #[test]
    fn test_position_velocity_extraction() {
        let state = Vector6::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let pos = position_from_state(&state);
        let vel = velocity_from_state(&state);

        assert_eq!(pos, Vector3::new(1.0, 2.0, 3.0));
        assert_eq!(vel, Vector3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_cross_product() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let c = cross(&a, &b);

        assert_relative_eq!(c.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(c.y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(c.z, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_normalize() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let normalized = normalize(&v);

        assert_relative_eq!(normalized.norm(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(normalized.x, 0.6, epsilon = 1e-10);
        assert_relative_eq!(normalized.y, 0.8, epsilon = 1e-10);
    }

    #[test]
    fn test_try_normalize() {
        let v = Vector3::new(1e-15, 1e-15, 1e-15);
        let result = try_normalize(&v, 1e-10);
        assert!(result.is_none());

        let v2 = Vector3::new(3.0, 4.0, 0.0);
        let result2 = try_normalize(&v2, 1e-10);
        assert!(result2.is_some());
        assert_relative_eq!(result2.unwrap().norm(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_skew_symmetric() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let skew = skew_symmetric(&v);

        assert_eq!(skew[(0, 0)], 0.0);
        assert_eq!(skew[(0, 1)], -3.0);
        assert_eq!(skew[(0, 2)], 2.0);
        assert_eq!(skew[(1, 0)], 3.0);
        assert_eq!(skew[(1, 1)], 0.0);
        assert_eq!(skew[(1, 2)], -1.0);
        assert_eq!(skew[(2, 0)], -2.0);
        assert_eq!(skew[(2, 1)], 1.0);
        assert_eq!(skew[(2, 2)], 0.0);
    }

    #[test]
    fn test_rotation_from_euler() {
        use std::f64::consts::PI;

        // 90 degree rotation around Z-axis
        let rot = rotation_from_euler(0.0, 0.0, PI / 2.0);
        let v = Vector3::new(1.0, 0.0, 0.0);
        let rotated = rot * v;

        assert_relative_eq!(rotated.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(rotated.y, 1.0, epsilon = 1e-10);
        assert_relative_eq!(rotated.z, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_angle_between() {
        use std::f64::consts::PI;

        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let angle = angle_between(&a, &b);

        assert_relative_eq!(angle, PI / 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_block_matrix_construction() {
        let a = Matrix3::identity();
        let b = Matrix3::zeros();
        let c = Matrix3::zeros();
        let d = Matrix3::identity();

        let block = block_matrix_6x6(a, b, c, d);

        // Check diagonal elements
        for i in 0..6 {
            assert_eq!(block[(i, i)], 1.0);
        }

        // Check off-diagonal blocks are zero
        assert_eq!(block[(0, 3)], 0.0);
        assert_eq!(block[(3, 0)], 0.0);
    }

    // ============================================================================
    // Property-Based Tests
    // ============================================================================
    // These tests verify mathematical properties hold for arbitrary inputs

    use proptest::prelude::*;

    // Strategy for generating non-degenerate 3D vectors
    fn vector3_strategy() -> impl Strategy<Value = Vector3> {
        (
            -1e6..1e6_f64,
            -1e6..1e6_f64,
            -1e6..1e6_f64,
        )
            .prop_map(|(x, y, z)| Vector3::new(x, y, z))
            .prop_filter("non-zero vector", |v| v.norm() > 1e-10)
    }

    proptest! {
        #[test]
        fn test_normalize_yields_unit_vector(v in vector3_strategy()) {
            let normalized = normalize(&v);
            let norm = normalized.norm();
            // Normalized vector should have magnitude 1
            assert!((norm - 1.0).abs() < 1e-10, "norm = {}", norm);
        }

        #[test]
        fn test_cross_product_orthogonality(
            a in vector3_strategy(),
            b in vector3_strategy()
        ) {
            let c = cross(&a, &b);
            // Cross product should be orthogonal to both input vectors
            // Use relative tolerance based on magnitudes involved
            let a_mag = a.norm();
            let b_mag = b.norm();
            let c_mag = c.norm();
            // Tolerance accounts for floating point errors in large magnitude products
            let tolerance = a_mag * b_mag * 1e-9;

            let dot_a = c.dot(&a).abs();
            let dot_b = c.dot(&b).abs();
            assert!(dot_a < tolerance, "c·a = {}, tolerance = {}", dot_a, tolerance);
            assert!(dot_b < tolerance, "c·b = {}, tolerance = {}", dot_b, tolerance);
        }

        #[test]
        fn test_cross_product_anticommutativity(
            a in vector3_strategy(),
            b in vector3_strategy()
        ) {
            // Property: a × b = -(b × a)
            let ab = cross(&a, &b);
            let ba = cross(&b, &a);
            let diff = (ab + ba).norm();
            assert!(diff < 1e-10, "||a×b + b×a|| = {}", diff);
        }

        #[test]
        fn test_skew_symmetric_property(v in vector3_strategy()) {
            let skew = skew_symmetric(&v);
            // Skew-symmetric matrix should satisfy: S = -S^T
            let skew_transpose = skew.transpose();
            let sum = skew + skew_transpose;
            let max_elem = sum.abs().max();
            assert!(max_elem < 1e-10, "max|S + S^T| = {}", max_elem);
        }

        #[test]
        fn test_skew_symmetric_cross_product_equivalence(
            a in vector3_strategy(),
            b in vector3_strategy()
        ) {
            // Property: skew(a) * b = a × b
            let skew_a = skew_symmetric(&a);
            let result1 = skew_a * b;
            let result2 = cross(&a, &b);
            let diff = (result1 - result2).norm();
            assert!(diff < 1e-10, "||skew(a)*b - a×b|| = {}", diff);
        }

        #[test]
        fn test_state_roundtrip(
            rx in -1e8..1e8_f64, ry in -1e8..1e8_f64, rz in -1e8..1e8_f64,
            vx in -1e5..1e5_f64, vy in -1e5..1e5_f64, vz in -1e5..1e5_f64
        ) {
            // Property: state construction and extraction should be inverse operations
            let pos = Vector3::new(rx, ry, rz);
            let vel = Vector3::new(vx, vy, vz);
            let state = state_vector(pos, vel);

            let pos_extracted = position_from_state(&state);
            let vel_extracted = velocity_from_state(&state);

            assert_relative_eq!(pos, pos_extracted, epsilon = 1e-10);
            assert_relative_eq!(vel, vel_extracted, epsilon = 1e-10);
        }

        #[test]
        fn test_triple_scalar_product_property(
            a in vector3_strategy(),
            b in vector3_strategy(),
            c in vector3_strategy()
        ) {
            // Property: a · (b × c) = (a × b) · c (scalar triple product)
            let bc = cross(&b, &c);
            let ab = cross(&a, &b);

            let result1 = a.dot(&bc);
            let result2 = ab.dot(&c);

            // Use relative tolerance for larger values
            let max_result = result1.abs().max(result2.abs());
            let tolerance = if max_result > 1.0 {
                max_result * 1e-10
            } else {
                1e-10
            };

            let diff = (result1 - result2).abs();
            assert!(diff < tolerance, "diff = {}, tolerance = {}", diff, tolerance);
        }

        #[test]
        fn test_double_normalization_idempotent(v in vector3_strategy()) {
            // Property: normalizing a normalized vector should not change it
            let normalized_once = normalize(&v);
            let normalized_twice = normalize(&normalized_once);

            let diff = (normalized_once - normalized_twice).norm();
            assert!(diff < 1e-10, "||norm(norm(v)) - norm(v)|| = {}", diff);
        }
    }
}
