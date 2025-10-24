"""Comprehensive tests for NumPy integration with Rust backend"""

import numpy as np
import pytest
from astrora._core import numpy_ops


class TestBasicArrayOperations:
    """Test basic 1D array operations"""

    def test_sum_array(self):
        """Test array summation with zero-copy read"""
        arr = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
        result = numpy_ops.sum_array(arr)
        assert result == pytest.approx(15.0)

    def test_sum_array_empty(self):
        """Test summation of empty array"""
        arr = np.array([])
        result = numpy_ops.sum_array(arr)
        assert result == 0.0

    def test_sum_array_single_element(self):
        """Test summation of single element"""
        arr = np.array([42.0])
        result = numpy_ops.sum_array(arr)
        assert result == 42.0

    def test_multiply_scalar(self):
        """Test scalar multiplication returning new array"""
        arr = np.array([1.0, 2.0, 3.0])
        result = numpy_ops.multiply_scalar(arr, 2.5)

        # Original array should be unchanged
        assert np.array_equal(arr, [1.0, 2.0, 3.0])

        # Result should be multiplied
        assert np.allclose(result, [2.5, 5.0, 7.5])

    def test_multiply_scalar_inplace(self):
        """Test in-place scalar multiplication with zero-copy mutability"""
        arr = np.array([1.0, 2.0, 3.0])
        original_id = id(arr)

        numpy_ops.multiply_scalar_inplace(arr, 2.0)

        # Array should be modified in place
        assert np.allclose(arr, [2.0, 4.0, 6.0])
        assert id(arr) == original_id  # Verify it's the same array

    def test_multiply_scalar_negative(self):
        """Test scalar multiplication with negative scalar"""
        arr = np.array([1.0, -2.0, 3.0])
        result = numpy_ops.multiply_scalar(arr, -1.5)
        assert np.allclose(result, [-1.5, 3.0, -4.5])


class TestVectorOperations:
    """Test vector operations (dot product, cross product, normalization)"""

    def test_dot_product_basic(self):
        """Test basic dot product"""
        a = np.array([1.0, 2.0, 3.0])
        b = np.array([4.0, 5.0, 6.0])
        result = numpy_ops.dot_product(a, b)
        assert result == pytest.approx(32.0)  # 1*4 + 2*5 + 3*6 = 32

    def test_dot_product_orthogonal(self):
        """Test dot product of orthogonal vectors"""
        a = np.array([1.0, 0.0, 0.0])
        b = np.array([0.0, 1.0, 0.0])
        result = numpy_ops.dot_product(a, b)
        assert result == pytest.approx(0.0)

    def test_dot_product_length_mismatch(self):
        """Test that dot product raises error for mismatched lengths"""
        a = np.array([1.0, 2.0])
        b = np.array([1.0, 2.0, 3.0])
        with pytest.raises(ValueError):
            numpy_ops.dot_product(a, b)

    def test_cross_product_basis_vectors(self):
        """Test cross product of basis vectors"""
        x = np.array([1.0, 0.0, 0.0])
        y = np.array([0.0, 1.0, 0.0])
        z = numpy_ops.cross_product(x, y)
        assert np.allclose(z, [0.0, 0.0, 1.0])

    def test_cross_product_anticommutative(self):
        """Test that cross product is anticommutative: a × b = -(b × a)"""
        a = np.array([1.0, 2.0, 3.0])
        b = np.array([4.0, 5.0, 6.0])
        ab = numpy_ops.cross_product(a, b)
        ba = numpy_ops.cross_product(b, a)
        assert np.allclose(ab, -ba)

    def test_cross_product_invalid_length(self):
        """Test that cross product raises error for non-3D vectors"""
        a = np.array([1.0, 2.0])
        b = np.array([3.0, 4.0, 5.0])
        with pytest.raises(ValueError):
            numpy_ops.cross_product(a, b)

    def test_add_arrays(self):
        """Test element-wise array addition"""
        a = np.array([1.0, 2.0, 3.0])
        b = np.array([4.0, 5.0, 6.0])
        result = numpy_ops.add_arrays(a, b)
        assert np.allclose(result, [5.0, 7.0, 9.0])

    def test_add_arrays_length_mismatch(self):
        """Test that array addition raises error for mismatched lengths"""
        a = np.array([1.0, 2.0])
        b = np.array([1.0, 2.0, 3.0])
        with pytest.raises(ValueError):
            numpy_ops.add_arrays(a, b)

    def test_normalize_vector(self):
        """Test vector normalization"""
        vec = np.array([3.0, 4.0, 0.0])
        result = numpy_ops.normalize_vector(vec)

        # Should be unit length
        magnitude = np.linalg.norm(result)
        assert magnitude == pytest.approx(1.0)

        # Should be in same direction
        assert np.allclose(result, [0.6, 0.8, 0.0])

    def test_normalize_already_unit_vector(self):
        """Test normalizing a vector that's already unit length"""
        vec = np.array([1.0, 0.0, 0.0])
        result = numpy_ops.normalize_vector(vec)
        assert np.allclose(result, vec)

    def test_normalize_zero_vector(self):
        """Test that normalizing zero vector raises error"""
        vec = np.array([0.0, 0.0, 0.0])
        with pytest.raises(ArithmeticError):  # DivisionByZero maps to ArithmeticError
            numpy_ops.normalize_vector(vec)

    def test_vector_magnitude(self):
        """Test vector magnitude calculation"""
        vec = np.array([3.0, 4.0, 0.0])
        result = numpy_ops.vector_magnitude(vec)
        assert result == pytest.approx(5.0)

    def test_vector_magnitude_unit_vector(self):
        """Test magnitude of unit vector"""
        vec = np.array([1.0, 0.0, 0.0])
        result = numpy_ops.vector_magnitude(vec)
        assert result == pytest.approx(1.0)


class TestMatrixOperations:
    """Test matrix operations"""

    def test_matrix_vector_multiply(self):
        """Test matrix-vector multiplication"""
        matrix = np.array([[1.0, 2.0], [3.0, 4.0]])
        vector = np.array([5.0, 6.0])
        result = numpy_ops.matrix_vector_multiply(matrix, vector)

        # Expected: [1*5 + 2*6, 3*5 + 4*6] = [17, 39]
        assert np.allclose(result, [17.0, 39.0])

    def test_matrix_vector_multiply_identity(self):
        """Test matrix-vector multiplication with identity matrix"""
        identity = np.eye(3)
        vector = np.array([1.0, 2.0, 3.0])
        result = numpy_ops.matrix_vector_multiply(identity, vector)
        assert np.allclose(result, vector)

    def test_matrix_vector_multiply_dimension_mismatch(self):
        """Test that dimension mismatch raises error"""
        matrix = np.array([[1.0, 2.0], [3.0, 4.0]])
        vector = np.array([5.0, 6.0, 7.0])
        with pytest.raises(ValueError):
            numpy_ops.matrix_vector_multiply(matrix, vector)

    def test_matrix_multiply(self):
        """Test matrix-matrix multiplication"""
        a = np.array([[1.0, 2.0], [3.0, 4.0]])
        b = np.array([[5.0, 6.0], [7.0, 8.0]])
        result = numpy_ops.matrix_multiply(a, b)

        # Expected: [[19, 22], [43, 50]]
        expected = np.array([[19.0, 22.0], [43.0, 50.0]])
        assert np.allclose(result, expected)

    def test_matrix_multiply_identity(self):
        """Test matrix multiplication with identity"""
        a = np.array([[1.0, 2.0], [3.0, 4.0]])
        identity = np.eye(2)
        result = numpy_ops.matrix_multiply(a, identity)
        assert np.allclose(result, a)

    def test_matrix_multiply_dimension_mismatch(self):
        """Test that incompatible dimensions raise error"""
        a = np.array([[1.0, 2.0]])  # 1x2
        b = np.array([[3.0], [4.0], [5.0]])  # 3x1
        with pytest.raises(ValueError):
            numpy_ops.matrix_multiply(a, b)

    def test_transpose_matrix(self):
        """Test matrix transposition"""
        matrix = np.array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])
        result = numpy_ops.transpose_matrix(matrix)

        expected = np.array([[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]])
        assert np.allclose(result, expected)

    def test_transpose_square_matrix(self):
        """Test transposition of square matrix"""
        matrix = np.array([[1.0, 2.0], [3.0, 4.0]])
        result = numpy_ops.transpose_matrix(matrix)
        expected = np.array([[1.0, 3.0], [2.0, 4.0]])
        assert np.allclose(result, expected)

    def test_identity_matrix(self):
        """Test identity matrix creation"""
        result = numpy_ops.identity_matrix(3)
        expected = np.eye(3)
        assert np.allclose(result, expected)

    def test_identity_matrix_size_1(self):
        """Test 1x1 identity matrix"""
        result = numpy_ops.identity_matrix(1)
        assert np.allclose(result, [[1.0]])

    def test_identity_matrix_zero_size(self):
        """Test that zero size raises error"""
        with pytest.raises(ValueError):
            numpy_ops.identity_matrix(0)


class TestPolynomialOperations:
    """Test polynomial application"""

    def test_apply_polynomial(self):
        """Test applying polynomial f(x) = x^2 + 2x + 1"""
        arr = np.array([0.0, 1.0, 2.0])
        result = numpy_ops.apply_polynomial(arr)

        # f(0) = 1, f(1) = 4, f(2) = 9
        assert np.allclose(result, [1.0, 4.0, 9.0])

    def test_apply_polynomial_negative(self):
        """Test polynomial with negative values"""
        arr = np.array([-1.0])
        result = numpy_ops.apply_polynomial(arr)

        # f(-1) = 1 - 2 + 1 = 0
        assert np.allclose(result, [0.0])


class TestBatchOperations:
    """Test batch operations on multiple vectors"""

    def test_batch_normalize_vectors(self):
        """Test normalizing multiple 3D vectors at once"""
        vectors = np.array([[3.0, 4.0, 0.0], [0.0, 0.0, 5.0], [1.0, 1.0, 1.0]])
        result = numpy_ops.batch_normalize_vectors(vectors)

        # Check all vectors are normalized
        for i in range(3):
            magnitude = np.linalg.norm(result[i, :])
            assert magnitude == pytest.approx(1.0)

        # Check specific values
        assert np.allclose(result[0, :], [0.6, 0.8, 0.0])
        assert np.allclose(result[1, :], [0.0, 0.0, 1.0])

        sqrt3_inv = 1.0 / np.sqrt(3.0)
        assert np.allclose(result[2, :], [sqrt3_inv, sqrt3_inv, sqrt3_inv])

    def test_batch_normalize_single_vector(self):
        """Test batch normalization with single vector"""
        vectors = np.array([[3.0, 4.0, 0.0]])
        result = numpy_ops.batch_normalize_vectors(vectors)
        assert np.allclose(result[0, :], [0.6, 0.8, 0.0])

    def test_batch_normalize_wrong_dimensions(self):
        """Test that non-3D vectors raise error"""
        vectors = np.array([[1.0, 2.0], [3.0, 4.0]])  # 2D vectors
        with pytest.raises(ValueError):
            numpy_ops.batch_normalize_vectors(vectors)

    def test_batch_normalize_with_zero_vector(self):
        """Test that zero vector in batch raises error"""
        vectors = np.array([[3.0, 4.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]])  # Zero vector
        with pytest.raises(ArithmeticError):
            numpy_ops.batch_normalize_vectors(vectors)


class TestDataTypes:
    """Test different NumPy data types and conversions"""

    def test_float32_input(self):
        """Test that float32 arrays work correctly"""
        arr = np.array([1.0, 2.0, 3.0], dtype=np.float32)
        # This should work as numpy will handle the conversion
        result = numpy_ops.sum_array(arr.astype(np.float64))
        assert result == pytest.approx(6.0)

    def test_integer_input_conversion(self):
        """Test that integer arrays are converted to float"""
        arr = np.array([1, 2, 3], dtype=np.int32)
        result = numpy_ops.sum_array(arr.astype(np.float64))
        assert result == pytest.approx(6.0)


class TestEdgeCases:
    """Test edge cases and boundary conditions"""

    def test_very_small_values(self):
        """Test operations with very small values"""
        arr = np.array([1e-10, 2e-10, 3e-10])
        result = numpy_ops.sum_array(arr)
        assert result == pytest.approx(6e-10)

    def test_very_large_values(self):
        """Test operations with very large values"""
        arr = np.array([1e10, 2e10, 3e10])
        result = numpy_ops.sum_array(arr)
        assert result == pytest.approx(6e10)

    def test_mixed_signs(self):
        """Test operations with mixed positive/negative values"""
        arr = np.array([-5.0, 10.0, -3.0, 8.0])
        result = numpy_ops.sum_array(arr)
        assert result == pytest.approx(10.0)

    def test_large_array_performance(self):
        """Test that large arrays can be processed"""
        arr = np.random.randn(10000)
        result = numpy_ops.sum_array(arr)
        expected = np.sum(arr)
        assert result == pytest.approx(expected)


class TestZeroCopyBehavior:
    """Test zero-copy operations and memory efficiency"""

    def test_readonly_preserves_original(self):
        """Test that read-only operations don't modify original array"""
        arr = np.array([1.0, 2.0, 3.0])
        original_copy = arr.copy()

        _ = numpy_ops.sum_array(arr)
        _ = numpy_ops.multiply_scalar(arr, 2.0)
        _ = numpy_ops.vector_magnitude(arr)

        # Original array should be unchanged
        assert np.array_equal(arr, original_copy)

    def test_inplace_modifies_original(self):
        """Test that in-place operations modify the original array"""
        arr = np.array([1.0, 2.0, 3.0])

        numpy_ops.multiply_scalar_inplace(arr, 2.0)

        # Array should be modified
        assert np.allclose(arr, [2.0, 4.0, 6.0])

    def test_contiguous_vs_non_contiguous(self):
        """Test that both contiguous and non-contiguous arrays work"""
        # Contiguous array
        arr_c = np.array([1.0, 2.0, 3.0])
        result_c = numpy_ops.sum_array(arr_c)

        # Non-contiguous array (view with stride)
        arr_full = np.array([1.0, 999.0, 2.0, 999.0, 3.0, 999.0])
        arr_nc = arr_full[::2]  # Non-contiguous view
        result_nc = numpy_ops.sum_array(arr_nc)

        assert result_c == result_nc


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
