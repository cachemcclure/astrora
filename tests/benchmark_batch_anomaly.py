"""
Benchmark batch anomaly conversions vs sequential processing.

Expected: 10-20x performance improvement for batch processing due to:
- Single Python-Rust boundary crossing
- Better CPU cache utilization
- Reduced function call overhead
"""

import numpy as np
import pytest
from astrora._core import (
    batch_mean_to_eccentric_anomaly,
    batch_mean_to_hyperbolic_anomaly,
    batch_mean_to_true_anomaly,
    batch_mean_to_true_anomaly_hyperbolic,
    batch_mean_to_true_anomaly_parabolic,
    mean_to_eccentric_anomaly,
    mean_to_hyperbolic_anomaly,
    mean_to_true_anomaly,
    mean_to_true_anomaly_hyperbolic,
    mean_to_true_anomaly_parabolic,
)


class TestBatchEllipticalBenchmark:
    """Benchmark batch vs sequential for elliptical orbits"""

    def test_benchmark_batch_mean_to_eccentric_small(self, benchmark):
        """Benchmark batch processing with small array (10 orbits)"""
        mean_anomalies = np.linspace(0, 2 * np.pi, 10)
        eccentricity = np.array([0.5])

        result = benchmark(batch_mean_to_eccentric_anomaly, mean_anomalies, eccentricity)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_mean_to_eccentric_small(self, benchmark):
        """Benchmark sequential processing with small array (10 orbits)"""
        mean_anomalies = np.linspace(0, 2 * np.pi, 10)
        eccentricity = 0.5

        def sequential():
            return np.array([mean_to_eccentric_anomaly(M, eccentricity) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_batch_mean_to_eccentric_medium(self, benchmark):
        """Benchmark batch processing with medium array (100 orbits)"""
        mean_anomalies = np.linspace(0, 20 * np.pi, 100)
        eccentricity = np.array([0.6])

        result = benchmark(batch_mean_to_eccentric_anomaly, mean_anomalies, eccentricity)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_mean_to_eccentric_medium(self, benchmark):
        """Benchmark sequential processing with medium array (100 orbits)"""
        mean_anomalies = np.linspace(0, 20 * np.pi, 100)
        eccentricity = 0.6

        def sequential():
            return np.array([mean_to_eccentric_anomaly(M, eccentricity) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_batch_mean_to_eccentric_large(self, benchmark):
        """Benchmark batch processing with large array (1000 orbits)"""
        mean_anomalies = np.linspace(0, 200 * np.pi, 1000)
        eccentricity = np.array([0.7])

        result = benchmark(batch_mean_to_eccentric_anomaly, mean_anomalies, eccentricity)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_mean_to_eccentric_large(self, benchmark):
        """Benchmark sequential processing with large array (1000 orbits)"""
        mean_anomalies = np.linspace(0, 200 * np.pi, 1000)
        eccentricity = 0.7

        def sequential():
            return np.array([mean_to_eccentric_anomaly(M, eccentricity) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_batch_mean_to_true_medium(self, benchmark):
        """Benchmark batch M → ν conversion (100 orbits)"""
        mean_anomalies = np.linspace(0, 20 * np.pi, 100)
        eccentricity = np.array([0.5])

        result = benchmark(batch_mean_to_true_anomaly, mean_anomalies, eccentricity)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_mean_to_true_medium(self, benchmark):
        """Benchmark sequential M → ν conversion (100 orbits)"""
        mean_anomalies = np.linspace(0, 20 * np.pi, 100)
        eccentricity = 0.5

        def sequential():
            return np.array([mean_to_true_anomaly(M, eccentricity) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_batch_variable_eccentricities(self, benchmark):
        """Benchmark batch with different eccentricity per orbit (100 orbits)"""
        mean_anomalies = np.linspace(0, 20 * np.pi, 100)
        eccentricities = np.linspace(0.1, 0.9, 100)

        result = benchmark(batch_mean_to_eccentric_anomaly, mean_anomalies, eccentricities)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_variable_eccentricities(self, benchmark):
        """Benchmark sequential with different eccentricity per orbit (100 orbits)"""
        mean_anomalies = np.linspace(0, 20 * np.pi, 100)
        eccentricities = np.linspace(0.1, 0.9, 100)

        def sequential():
            return np.array(
                [
                    mean_to_eccentric_anomaly(mean_anomalies[i], eccentricities[i])
                    for i in range(len(mean_anomalies))
                ]
            )

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape


class TestBatchHyperbolicBenchmark:
    """Benchmark batch vs sequential for hyperbolic orbits"""

    def test_benchmark_batch_mean_to_hyperbolic_medium(self, benchmark):
        """Benchmark batch M → H conversion (100 orbits)"""
        mean_anomalies = np.linspace(0.5, 10, 100)
        eccentricity = np.array([1.5])

        result = benchmark(batch_mean_to_hyperbolic_anomaly, mean_anomalies, eccentricity)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_mean_to_hyperbolic_medium(self, benchmark):
        """Benchmark sequential M → H conversion (100 orbits)"""
        mean_anomalies = np.linspace(0.5, 10, 100)
        eccentricity = 1.5

        def sequential():
            return np.array([mean_to_hyperbolic_anomaly(M, eccentricity) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_batch_mean_to_true_hyperbolic(self, benchmark):
        """Benchmark batch M → ν conversion for hyperbolic (100 orbits)"""
        mean_anomalies = np.linspace(0.5, 10, 100)
        eccentricity = np.array([2.0])

        result = benchmark(batch_mean_to_true_anomaly_hyperbolic, mean_anomalies, eccentricity)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_mean_to_true_hyperbolic(self, benchmark):
        """Benchmark sequential M → ν conversion for hyperbolic (100 orbits)"""
        mean_anomalies = np.linspace(0.5, 10, 100)
        eccentricity = 2.0

        def sequential():
            return np.array(
                [mean_to_true_anomaly_hyperbolic(M, eccentricity) for M in mean_anomalies]
            )

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape


class TestBatchParabolicBenchmark:
    """Benchmark batch vs sequential for parabolic orbits"""

    def test_benchmark_batch_mean_to_true_parabolic(self, benchmark):
        """Benchmark batch M → ν conversion for parabolic (100 orbits)"""
        mean_anomalies = np.linspace(-5, 5, 100)

        result = benchmark(batch_mean_to_true_anomaly_parabolic, mean_anomalies)
        assert result.shape == mean_anomalies.shape

    def test_benchmark_sequential_mean_to_true_parabolic(self, benchmark):
        """Benchmark sequential M → ν conversion for parabolic (100 orbits)"""
        mean_anomalies = np.linspace(-5, 5, 100)

        def sequential():
            return np.array([mean_to_true_anomaly_parabolic(M) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == mean_anomalies.shape


class TestBatchLargeScaleBenchmark:
    """Benchmark with large arrays (mission analysis scale)"""

    def test_benchmark_batch_constellation_analysis(self, benchmark):
        """Benchmark batch processing for constellation (5000 satellites)"""
        # Simulate analyzing 5000 satellites
        n = 5000
        mean_anomalies = np.random.uniform(0, 2 * np.pi, n)
        eccentricity = np.array([0.001])  # Near-circular LEO

        result = benchmark(batch_mean_to_eccentric_anomaly, mean_anomalies, eccentricity)
        assert result.shape == (n,)

    def test_benchmark_sequential_constellation_analysis(self, benchmark):
        """Benchmark sequential processing for constellation (5000 satellites)"""
        # Simulate analyzing 5000 satellites
        n = 5000
        mean_anomalies = np.random.uniform(0, 2 * np.pi, n)
        eccentricity = 0.001

        def sequential():
            return np.array([mean_to_eccentric_anomaly(M, eccentricity) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == (n,)

    def test_benchmark_batch_propagation_grid(self, benchmark):
        """Benchmark batch for propagation grid (10000 time steps)"""
        # Simulate propagating single orbit over 10000 time steps
        n = 10000
        mean_anomalies = np.linspace(0, 1000 * np.pi, n)
        eccentricity = np.array([0.5])

        result = benchmark(batch_mean_to_true_anomaly, mean_anomalies, eccentricity)
        assert result.shape == (n,)

    def test_benchmark_sequential_propagation_grid(self, benchmark):
        """Benchmark sequential for propagation grid (10000 time steps)"""
        # Simulate propagating single orbit over 10000 time steps
        n = 10000
        mean_anomalies = np.linspace(0, 1000 * np.pi, n)
        eccentricity = 0.5

        def sequential():
            return np.array([mean_to_true_anomaly(M, eccentricity) for M in mean_anomalies])

        result = benchmark(sequential)
        assert result.shape == (n,)


if __name__ == "__main__":
    pytest.main([__file__, "--benchmark-only", "-v"])
