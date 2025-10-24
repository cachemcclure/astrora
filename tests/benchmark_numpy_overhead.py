"""
Comprehensive benchmarks for profiling Python-Rust data transfer overhead.

This benchmark suite measures:
1. Pure NumPy operations (baseline)
2. Rust-backed operations (to measure overhead)
3. Scaling behavior with array size
4. Different operation patterns (read-only, in-place, batch)
5. Boundary crossing costs

Run with:
    pytest tests/benchmark_numpy_overhead.py --benchmark-only

For detailed analysis:
    pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-verbose

To save results:
    pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-save=baseline

To compare against saved results:
    pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-compare=baseline
"""

import numpy as np
import pytest
from astrora._core import numpy_ops

# Array sizes for scaling tests
ARRAY_SIZES = {
    "tiny": 10,
    "small": 100,
    "medium": 1_000,
    "large": 10_000,
    "very_large": 100_000,
    "huge": 1_000_000,
}


class TestReadOnlyOperationsOverhead:
    """
    Benchmark read-only operations to measure zero-copy overhead.
    These should have minimal overhead since no data is copied back to Python.
    """

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large", "very_large"])
    def test_sum_array_rust_vs_numpy(self, benchmark, size_name):
        """Compare Rust sum vs NumPy sum across different array sizes"""
        size = ARRAY_SIZES[size_name]
        arr = np.random.randn(size)

        # Benchmark Rust implementation
        result = benchmark(numpy_ops.sum_array, arr)

        # Verify correctness
        assert result == pytest.approx(np.sum(arr))

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large", "very_large"])
    def test_sum_array_numpy_baseline(self, benchmark, size_name):
        """Baseline: Pure NumPy sum for comparison"""
        size = ARRAY_SIZES[size_name]
        arr = np.random.randn(size)

        result = benchmark(np.sum, arr)

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large"])
    def test_dot_product_rust_vs_numpy(self, benchmark, size_name):
        """Compare Rust dot product vs NumPy across sizes"""
        size = ARRAY_SIZES[size_name]
        a = np.random.randn(size)
        b = np.random.randn(size)

        result = benchmark(numpy_ops.dot_product, a, b)

        assert result == pytest.approx(np.dot(a, b))

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large"])
    def test_dot_product_numpy_baseline(self, benchmark, size_name):
        """Baseline: Pure NumPy dot product"""
        size = ARRAY_SIZES[size_name]
        a = np.random.randn(size)
        b = np.random.randn(size)

        result = benchmark(np.dot, a, b)

    def test_vector_magnitude_rust(self, benchmark):
        """Benchmark Rust vector magnitude calculation"""
        vec = np.random.randn(1000)
        result = benchmark(numpy_ops.vector_magnitude, vec)

        assert result == pytest.approx(np.linalg.norm(vec))

    def test_vector_magnitude_numpy_baseline(self, benchmark):
        """Baseline: NumPy norm calculation"""
        vec = np.random.randn(1000)
        result = benchmark(np.linalg.norm, vec)


class TestArrayReturnOverhead:
    """
    Benchmark operations that return new arrays.
    These involve data transfer from Rust back to Python.
    """

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large", "very_large"])
    def test_multiply_scalar_rust(self, benchmark, size_name):
        """Rust scalar multiplication (returns new array)"""
        size = ARRAY_SIZES[size_name]
        arr = np.random.randn(size)
        scalar = 2.5

        result = benchmark(numpy_ops.multiply_scalar, arr, scalar)

        assert np.allclose(result, arr * scalar)

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large", "very_large"])
    def test_multiply_scalar_numpy_baseline(self, benchmark, size_name):
        """Baseline: NumPy scalar multiplication"""
        size = ARRAY_SIZES[size_name]
        arr = np.random.randn(size)
        scalar = 2.5

        result = benchmark(lambda a, s: a * s, arr, scalar)

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large"])
    def test_normalize_vector_rust(self, benchmark, size_name):
        """Rust vector normalization (returns new array)"""
        size = ARRAY_SIZES[size_name]
        vec = np.random.randn(size)

        result = benchmark(numpy_ops.normalize_vector, vec)

        # Verify unit length
        assert np.linalg.norm(result) == pytest.approx(1.0)

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large"])
    def test_normalize_vector_numpy_baseline(self, benchmark, size_name):
        """Baseline: NumPy vector normalization"""
        size = ARRAY_SIZES[size_name]
        vec = np.random.randn(size)

        def normalize_numpy(v):
            return v / np.linalg.norm(v)

        result = benchmark(normalize_numpy, vec)


class TestInPlaceOperationsOverhead:
    """
    Benchmark in-place operations (zero-copy mutable access).
    These should be very efficient as they modify arrays directly.
    """

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large", "very_large"])
    def test_multiply_scalar_inplace_rust(self, benchmark, size_name):
        """Rust in-place scalar multiplication"""
        size = ARRAY_SIZES[size_name]
        scalar = 2.0

        def setup():
            # Return ALL arguments that the function needs
            return (np.random.randn(size), scalar), {}

        result = benchmark.pedantic(
            numpy_ops.multiply_scalar_inplace, setup=setup, iterations=1, rounds=100
        )

    @pytest.mark.parametrize("size_name", ["tiny", "small", "medium", "large", "very_large"])
    def test_multiply_scalar_inplace_numpy_baseline(self, benchmark, size_name):
        """Baseline: NumPy in-place multiplication"""
        size = ARRAY_SIZES[size_name]
        scalar = 2.0

        def setup():
            # Return ALL arguments that the function needs
            return (np.random.randn(size), scalar), {}

        def numpy_inplace(arr, s):
            arr *= s

        result = benchmark.pedantic(numpy_inplace, setup=setup, iterations=1, rounds=100)


class TestBatchOperationsScaling:
    """
    Benchmark batch operations to demonstrate efficiency gains.
    Processing many items in one Rust call should minimize boundary crossing overhead.
    """

    @pytest.mark.parametrize("batch_size", [10, 100, 1000, 10000])
    def test_batch_normalize_rust(self, benchmark, batch_size):
        """Rust batch normalization of 3D vectors"""
        vectors = np.random.randn(batch_size, 3)

        result = benchmark(numpy_ops.batch_normalize_vectors, vectors)

        # Verify all vectors are normalized
        magnitudes = np.linalg.norm(result, axis=1)
        assert np.allclose(magnitudes, 1.0)

    @pytest.mark.parametrize("batch_size", [10, 100, 1000, 10000])
    def test_batch_normalize_numpy_baseline(self, benchmark, batch_size):
        """Baseline: NumPy batch normalization"""
        vectors = np.random.randn(batch_size, 3)

        def normalize_batch_numpy(vecs):
            norms = np.linalg.norm(vecs, axis=1, keepdims=True)
            return vecs / norms

        result = benchmark(normalize_batch_numpy, vectors)

    @pytest.mark.parametrize("batch_size", [10, 100, 1000])
    def test_sequential_normalize_rust(self, benchmark, batch_size):
        """
        Sequential normalization (one call per vector) to show overhead
        of multiple boundary crossings vs batch operation
        """
        vectors = np.random.randn(batch_size, 3)

        def normalize_sequential(vecs):
            results = []
            for vec in vecs:
                results.append(numpy_ops.normalize_vector(vec))
            return np.array(results)

        result = benchmark(normalize_sequential, vectors)


class TestMatrixOperationsOverhead:
    """
    Benchmark matrix operations which involve larger data transfers.
    """

    @pytest.mark.parametrize("size", [10, 50, 100, 200])
    def test_matrix_multiply_rust(self, benchmark, size):
        """Rust matrix-matrix multiplication"""
        a = np.random.randn(size, size)
        b = np.random.randn(size, size)

        result = benchmark(numpy_ops.matrix_multiply, a, b)

        assert np.allclose(result, a @ b)

    @pytest.mark.parametrize("size", [10, 50, 100, 200])
    def test_matrix_multiply_numpy_baseline(self, benchmark, size):
        """Baseline: NumPy matrix multiplication"""
        a = np.random.randn(size, size)
        b = np.random.randn(size, size)

        result = benchmark(np.matmul, a, b)

    @pytest.mark.parametrize("size", [10, 50, 100, 200])
    def test_matrix_vector_multiply_rust(self, benchmark, size):
        """Rust matrix-vector multiplication"""
        matrix = np.random.randn(size, size)
        vector = np.random.randn(size)

        result = benchmark(numpy_ops.matrix_vector_multiply, matrix, vector)

    @pytest.mark.parametrize("size", [10, 50, 100, 200])
    def test_matrix_vector_multiply_numpy_baseline(self, benchmark, size):
        """Baseline: NumPy matrix-vector multiplication"""
        matrix = np.random.randn(size, size)
        vector = np.random.randn(size)

        result = benchmark(np.matmul, matrix, vector)


class TestBoundaryCrossingOverhead:
    """
    Specific tests to isolate and measure the Python-Rust boundary crossing cost.
    """

    def test_minimal_rust_call(self, benchmark):
        """
        Minimal Rust function call (sum of tiny array).
        This primarily measures boundary crossing overhead.
        """
        arr = np.array([1.0, 2.0, 3.0])
        result = benchmark(numpy_ops.sum_array, arr)

    def test_minimal_numpy_call(self, benchmark):
        """Baseline: Minimal NumPy operation"""
        arr = np.array([1.0, 2.0, 3.0])
        result = benchmark(np.sum, arr)

    def test_repeated_tiny_calls_rust(self, benchmark):
        """
        Multiple calls with tiny arrays to amplify boundary overhead.
        This demonstrates the cost of many boundary crossings.
        """
        arrays = [np.array([1.0, 2.0, 3.0]) for _ in range(100)]

        def process_all(arrs):
            return [numpy_ops.sum_array(a) for a in arrs]

        result = benchmark(process_all, arrays)

    def test_repeated_tiny_calls_numpy(self, benchmark):
        """Baseline: Multiple tiny NumPy calls"""
        arrays = [np.array([1.0, 2.0, 3.0]) for _ in range(100)]

        def process_all(arrs):
            return [np.sum(a) for a in arrs]

        result = benchmark(process_all, arrays)

    def test_single_large_call_rust(self, benchmark):
        """
        Single call with large array (minimizes boundary crossing impact).
        Compare with repeated_tiny_calls to see batching benefit.
        """
        # Same total data as repeated_tiny_calls (100 * 3 = 300 elements)
        arr = np.random.randn(300)
        result = benchmark(numpy_ops.sum_array, arr)

    def test_single_large_call_numpy(self, benchmark):
        """Baseline: Single large NumPy call"""
        arr = np.random.randn(300)
        result = benchmark(np.sum, arr)


class TestCrossProductOperations:
    """
    Cross product is a good test case as it's computational but not too heavy.
    Good for seeing where Rust becomes beneficial.
    """

    def test_cross_product_rust(self, benchmark):
        """Rust cross product (3D vectors)"""
        a = np.array([1.0, 2.0, 3.0])
        b = np.array([4.0, 5.0, 6.0])

        result = benchmark(numpy_ops.cross_product, a, b)

    def test_cross_product_numpy_baseline(self, benchmark):
        """Baseline: NumPy cross product"""
        a = np.array([1.0, 2.0, 3.0])
        b = np.array([4.0, 5.0, 6.0])

        result = benchmark(np.cross, a, b)

    @pytest.mark.parametrize("batch_size", [10, 100, 1000])
    def test_batch_cross_product_simulation(self, benchmark, batch_size):
        """
        Simulate batch cross product operations.
        This represents a real-world use case in orbital mechanics.
        """
        vectors_a = np.random.randn(batch_size, 3)
        vectors_b = np.random.randn(batch_size, 3)

        def batch_cross_rust(va, vb):
            results = []
            for a, b in zip(va, vb):
                results.append(numpy_ops.cross_product(a, b))
            return np.array(results)

        result = benchmark(batch_cross_rust, vectors_a, vectors_b)

    @pytest.mark.parametrize("batch_size", [10, 100, 1000])
    def test_batch_cross_product_numpy_baseline(self, benchmark, batch_size):
        """Baseline: NumPy vectorized cross product"""
        vectors_a = np.random.randn(batch_size, 3)
        vectors_b = np.random.randn(batch_size, 3)

        result = benchmark(np.cross, vectors_a, vectors_b)


if __name__ == "__main__":
    pytest.main([__file__, "--benchmark-only", "-v"])
