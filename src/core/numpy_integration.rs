//! NumPy integration utilities and examples
//!
//! This module demonstrates various patterns for integrating NumPy arrays
//! with Rust code using PyO3 and rust-numpy. It serves as both a test bed
//! for the integration and a reference for future implementations.

use ndarray::{Array1, Array2, ArrayView1, ArrayView2, ArrayViewMut1};

use crate::core::error::{PoliastroError, PoliastroResult};

/// Sum all elements in a 1D numpy array
///
/// This demonstrates basic array input with zero-copy read-only access.
///
/// # Arguments
/// * `arr` - Input 1D array (read-only, zero-copy)
///
/// # Returns
/// Sum of all elements
pub fn sum_array(arr: ArrayView1<f64>) -> f64 {
    arr.sum()
}

/// Multiply each element of a 1D array by a scalar
///
/// This demonstrates creating a new array and returning it to Python.
///
/// # Arguments
/// * `arr` - Input 1D array (read-only, zero-copy)
/// * `scalar` - Scalar multiplier
///
/// # Returns
/// New array with each element multiplied by scalar
pub fn multiply_scalar(arr: ArrayView1<f64>, scalar: f64) -> Array1<f64> {
    &arr * scalar
}

/// In-place multiplication of array elements by a scalar
///
/// This demonstrates mutable array access (still zero-copy).
///
/// # Arguments
/// * `arr` - Mutable view of 1D array (zero-copy)
/// * `scalar` - Scalar multiplier
pub fn multiply_scalar_inplace(mut arr: ArrayViewMut1<f64>, scalar: f64) {
    arr.mapv_inplace(|x| x * scalar);
}

/// Compute the dot product of two 1D arrays
///
/// # Arguments
/// * `a` - First vector (read-only, zero-copy)
/// * `b` - Second vector (read-only, zero-copy)
///
/// # Returns
/// Dot product of a and b
///
/// # Errors
/// Returns error if arrays have different lengths
pub fn dot_product(a: ArrayView1<f64>, b: ArrayView1<f64>) -> PoliastroResult<f64> {
    if a.len() != b.len() {
        return Err(PoliastroError::invalid_parameter(
            "array_length",
            a.len() as f64,
            format!("must match other array length ({})", b.len())
        ));
    }
    Ok(a.dot(&b))
}

/// Compute the cross product of two 3D vectors
///
/// # Arguments
/// * `a` - First 3D vector (read-only, zero-copy)
/// * `b` - Second 3D vector (read-only, zero-copy)
///
/// # Returns
/// Cross product a × b as a new 3D array
///
/// # Errors
/// Returns error if either array is not exactly 3 elements
pub fn cross_product(a: ArrayView1<f64>, b: ArrayView1<f64>) -> PoliastroResult<Array1<f64>> {
    if a.len() != 3 {
        return Err(PoliastroError::invalid_parameter(
            "vector_length",
            a.len() as f64,
            "must be 3 for cross product"
        ));
    }
    if b.len() != 3 {
        return Err(PoliastroError::invalid_parameter(
            "vector_length",
            b.len() as f64,
            "must be 3 for cross product"
        ));
    }

    let result = Array1::from_vec(vec![
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]);

    Ok(result)
}

/// Matrix-vector multiplication
///
/// Computes M × v where M is a 2D matrix and v is a 1D vector.
///
/// # Arguments
/// * `matrix` - 2D matrix (read-only, zero-copy)
/// * `vector` - 1D vector (read-only, zero-copy)
///
/// # Returns
/// Result vector as a new 1D array
///
/// # Errors
/// Returns error if matrix columns don't match vector length
pub fn matrix_vector_multiply(
    matrix: ArrayView2<f64>,
    vector: ArrayView1<f64>,
) -> PoliastroResult<Array1<f64>> {
    if matrix.ncols() != vector.len() {
        return Err(PoliastroError::invalid_parameter(
            "matrix_columns",
            matrix.ncols() as f64,
            format!("must match vector length ({})", vector.len())
        ));
    }

    Ok(matrix.dot(&vector))
}

/// Matrix-matrix multiplication
///
/// Computes A × B where both are 2D matrices.
///
/// # Arguments
/// * `a` - First matrix (read-only, zero-copy)
/// * `b` - Second matrix (read-only, zero-copy)
///
/// # Returns
/// Result matrix as a new 2D array
///
/// # Errors
/// Returns error if matrix dimensions are incompatible
pub fn matrix_multiply(
    a: ArrayView2<f64>,
    b: ArrayView2<f64>,
) -> PoliastroResult<Array2<f64>> {
    if a.ncols() != b.nrows() {
        return Err(PoliastroError::invalid_parameter(
            "matrix_dimensions",
            a.ncols() as f64,
            format!("matrix A columns must match matrix B rows ({})", b.nrows())
        ));
    }

    Ok(a.dot(&b))
}

/// Element-wise addition of two 1D arrays
///
/// # Arguments
/// * `a` - First array (read-only, zero-copy)
/// * `b` - Second array (read-only, zero-copy)
///
/// # Returns
/// New array with element-wise sum
///
/// # Errors
/// Returns error if arrays have different lengths
pub fn add_arrays(a: ArrayView1<f64>, b: ArrayView1<f64>) -> PoliastroResult<Array1<f64>> {
    if a.len() != b.len() {
        return Err(PoliastroError::invalid_parameter(
            "array_length",
            a.len() as f64,
            format!("must match other array length ({})", b.len())
        ));
    }
    Ok(&a + &b)
}

/// Normalize a vector (make it unit length)
///
/// # Arguments
/// * `vec` - Input vector (read-only, zero-copy)
///
/// # Returns
/// Normalized vector as a new array
///
/// # Errors
/// Returns error if vector has zero magnitude
pub fn normalize_vector(vec: ArrayView1<f64>) -> PoliastroResult<Array1<f64>> {
    let magnitude = vec.dot(&vec).sqrt();

    if magnitude == 0.0 {
        return Err(PoliastroError::DivisionByZero {
            context: "vector normalization".to_string(),
            divisor: magnitude,
        });
    }

    Ok(&vec / magnitude)
}

/// Compute the magnitude (L2 norm) of a vector
///
/// # Arguments
/// * `vec` - Input vector (read-only, zero-copy)
///
/// # Returns
/// Magnitude of the vector
pub fn vector_magnitude(vec: ArrayView1<f64>) -> f64 {
    vec.dot(&vec).sqrt()
}

/// Apply a function element-wise to an array
///
/// This demonstrates how to process arrays with custom operations.
/// Computes f(x) = x^2 + 2x + 1 for each element.
///
/// # Arguments
/// * `arr` - Input array (read-only, zero-copy)
///
/// # Returns
/// New array with function applied to each element
pub fn apply_polynomial(arr: ArrayView1<f64>) -> Array1<f64> {
    arr.mapv(|x| x * x + 2.0 * x + 1.0)
}

/// Transpose a 2D matrix
///
/// # Arguments
/// * `matrix` - Input matrix (read-only, zero-copy)
///
/// # Returns
/// Transposed matrix as a new 2D array
pub fn transpose_matrix(matrix: ArrayView2<f64>) -> Array2<f64> {
    matrix.t().to_owned()
}

/// Create an identity matrix of given size
///
/// # Arguments
/// * `size` - Size of the square identity matrix
///
/// # Returns
/// Identity matrix as a new 2D array
///
/// # Errors
/// Returns error if size is 0
pub fn identity_matrix(size: usize) -> PoliastroResult<Array2<f64>> {
    if size == 0 {
        return Err(PoliastroError::invalid_parameter(
            "matrix_size",
            size as f64,
            "must be positive"
        ));
    }

    Ok(Array2::eye(size))
}

/// Batch operation: normalize multiple 3D vectors
///
/// This demonstrates processing multiple vectors efficiently.
/// Each row of the input matrix is treated as a 3D vector.
///
/// # Arguments
/// * `vectors` - 2D array where each row is a 3D vector (read-only, zero-copy)
///
/// # Returns
/// New 2D array with normalized vectors
///
/// # Errors
/// Returns error if any vector has zero magnitude or if columns != 3
pub fn batch_normalize_vectors(vectors: ArrayView2<f64>) -> PoliastroResult<Array2<f64>> {
    if vectors.ncols() != 3 {
        return Err(PoliastroError::invalid_parameter(
            "vector_dimensions",
            vectors.ncols() as f64,
            "must be 3 for 3D vectors"
        ));
    }

    let mut result = Array2::zeros((vectors.nrows(), 3));

    for (i, vector) in vectors.rows().into_iter().enumerate() {
        let magnitude = vector.dot(&vector).sqrt();
        if magnitude == 0.0 {
            return Err(PoliastroError::DivisionByZero {
                context: format!("vector normalization at row {i}"),
                divisor: magnitude,
            });
        }
        result.row_mut(i).assign(&(&vector / magnitude));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use ndarray::array;

    #[test]
    fn test_sum_array() {
        let arr = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sum_array(arr.view());
        assert_relative_eq!(result, 15.0, epsilon = 1e-10);
    }

    #[test]
    fn test_multiply_scalar() {
        let arr = array![1.0, 2.0, 3.0];
        let result = multiply_scalar(arr.view(), 2.5);
        assert_relative_eq!(result[0], 2.5, epsilon = 1e-10);
        assert_relative_eq!(result[1], 5.0, epsilon = 1e-10);
        assert_relative_eq!(result[2], 7.5, epsilon = 1e-10);
    }

    #[test]
    fn test_multiply_scalar_inplace() {
        let mut arr = array![1.0, 2.0, 3.0];
        multiply_scalar_inplace(arr.view_mut(), 2.0);
        assert_relative_eq!(arr[0], 2.0, epsilon = 1e-10);
        assert_relative_eq!(arr[1], 4.0, epsilon = 1e-10);
        assert_relative_eq!(arr[2], 6.0, epsilon = 1e-10);
    }

    #[test]
    fn test_dot_product() {
        let a = array![1.0, 2.0, 3.0];
        let b = array![4.0, 5.0, 6.0];
        let result = dot_product(a.view(), b.view()).unwrap();
        assert_relative_eq!(result, 32.0, epsilon = 1e-10); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_dot_product_length_mismatch() {
        let a = array![1.0, 2.0];
        let b = array![1.0, 2.0, 3.0];
        let result = dot_product(a.view(), b.view());
        assert!(result.is_err());
    }

    #[test]
    fn test_cross_product() {
        let a = array![1.0, 0.0, 0.0];
        let b = array![0.0, 1.0, 0.0];
        let result = cross_product(a.view(), b.view()).unwrap();
        assert_relative_eq!(result[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result[1], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result[2], 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_cross_product_invalid_length() {
        let a = array![1.0, 2.0];
        let b = array![3.0, 4.0, 5.0];
        let result = cross_product(a.view(), b.view());
        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_vector_multiply() {
        let matrix = array![[1.0, 2.0], [3.0, 4.0]];
        let vector = array![5.0, 6.0];
        let result = matrix_vector_multiply(matrix.view(), vector.view()).unwrap();
        assert_relative_eq!(result[0], 17.0, epsilon = 1e-10); // 1*5 + 2*6 = 17
        assert_relative_eq!(result[1], 39.0, epsilon = 1e-10); // 3*5 + 4*6 = 39
    }

    #[test]
    fn test_matrix_multiply() {
        let a = array![[1.0, 2.0], [3.0, 4.0]];
        let b = array![[5.0, 6.0], [7.0, 8.0]];
        let result = matrix_multiply(a.view(), b.view()).unwrap();
        assert_relative_eq!(result[[0, 0]], 19.0, epsilon = 1e-10); // 1*5 + 2*7 = 19
        assert_relative_eq!(result[[0, 1]], 22.0, epsilon = 1e-10); // 1*6 + 2*8 = 22
        assert_relative_eq!(result[[1, 0]], 43.0, epsilon = 1e-10); // 3*5 + 4*7 = 43
        assert_relative_eq!(result[[1, 1]], 50.0, epsilon = 1e-10); // 3*6 + 4*8 = 50
    }

    #[test]
    fn test_add_arrays() {
        let a = array![1.0, 2.0, 3.0];
        let b = array![4.0, 5.0, 6.0];
        let result = add_arrays(a.view(), b.view()).unwrap();
        assert_relative_eq!(result[0], 5.0, epsilon = 1e-10);
        assert_relative_eq!(result[1], 7.0, epsilon = 1e-10);
        assert_relative_eq!(result[2], 9.0, epsilon = 1e-10);
    }

    #[test]
    fn test_normalize_vector() {
        let vec = array![3.0, 4.0, 0.0];
        let result = normalize_vector(vec.view()).unwrap();
        assert_relative_eq!(result[0], 0.6, epsilon = 1e-10);
        assert_relative_eq!(result[1], 0.8, epsilon = 1e-10);
        assert_relative_eq!(result[2], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_normalize_zero_vector() {
        let vec = array![0.0, 0.0, 0.0];
        let result = normalize_vector(vec.view());
        assert!(result.is_err());
    }

    #[test]
    fn test_vector_magnitude() {
        let vec = array![3.0, 4.0, 0.0];
        let result = vector_magnitude(vec.view());
        assert_relative_eq!(result, 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_apply_polynomial() {
        let arr = array![0.0, 1.0, 2.0];
        let result = apply_polynomial(arr.view());
        assert_relative_eq!(result[0], 1.0, epsilon = 1e-10); // 0^2 + 2*0 + 1 = 1
        assert_relative_eq!(result[1], 4.0, epsilon = 1e-10); // 1^2 + 2*1 + 1 = 4
        assert_relative_eq!(result[2], 9.0, epsilon = 1e-10); // 2^2 + 2*2 + 1 = 9
    }

    #[test]
    fn test_transpose_matrix() {
        let matrix = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let result = transpose_matrix(matrix.view());
        assert_eq!(result.shape(), &[3, 2]);
        assert_relative_eq!(result[[0, 0]], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result[[0, 1]], 4.0, epsilon = 1e-10);
        assert_relative_eq!(result[[1, 0]], 2.0, epsilon = 1e-10);
        assert_relative_eq!(result[[1, 1]], 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_identity_matrix() {
        let result = identity_matrix(3).unwrap();
        assert_eq!(result.shape(), &[3, 3]);
        assert_relative_eq!(result[[0, 0]], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result[[1, 1]], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result[[2, 2]], 1.0, epsilon = 1e-10);
        assert_relative_eq!(result[[0, 1]], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result[[1, 2]], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_batch_normalize_vectors() {
        let vectors = array![[3.0, 4.0, 0.0], [0.0, 0.0, 5.0], [1.0, 1.0, 1.0]];
        let result = batch_normalize_vectors(vectors.view()).unwrap();

        // First vector: [3, 4, 0] / 5 = [0.6, 0.8, 0]
        assert_relative_eq!(result[[0, 0]], 0.6, epsilon = 1e-10);
        assert_relative_eq!(result[[0, 1]], 0.8, epsilon = 1e-10);
        assert_relative_eq!(result[[0, 2]], 0.0, epsilon = 1e-10);

        // Second vector: [0, 0, 5] / 5 = [0, 0, 1]
        assert_relative_eq!(result[[1, 0]], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result[[1, 1]], 0.0, epsilon = 1e-10);
        assert_relative_eq!(result[[1, 2]], 1.0, epsilon = 1e-10);

        // Third vector: [1, 1, 1] / sqrt(3)
        let sqrt3_inv = 1.0 / 3.0_f64.sqrt();
        assert_relative_eq!(result[[2, 0]], sqrt3_inv, epsilon = 1e-10);
        assert_relative_eq!(result[[2, 1]], sqrt3_inv, epsilon = 1e-10);
        assert_relative_eq!(result[[2, 2]], sqrt3_inv, epsilon = 1e-10);
    }
}
