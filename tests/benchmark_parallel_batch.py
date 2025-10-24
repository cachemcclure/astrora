"""
Benchmark to demonstrate rayon parallelization speedup for batch operations.

This script measures the performance of batch propagation and batch anomaly
conversions, demonstrating the speedup achieved through rayon parallel iterators.

Expected results:
- For small batches (< 100): Minimal or no speedup (parallelization overhead)
- For medium batches (100-1000): 2-4x speedup on multi-core machines
- For large batches (1000+): 2-8x speedup depending on core count

The actual speedup depends on:
1. Number of CPU cores available
2. CPU utilization by other processes
3. Memory bandwidth
4. Cache effects
"""

import time

import numpy as np
from astrora._core import (
    batch_mean_to_eccentric_anomaly,
    batch_mean_to_true_anomaly,
    batch_propagate_states,
    constants,
)

GM_EARTH = constants.GM_EARTH


class TestParallelBatchPerformance:
    """Performance benchmarks for parallel batch operations."""

    def test_batch_propagation_small(self, benchmark):
        """Batch propagation with 10 orbits (small, expect minimal speedup)."""
        n_orbits = 10
        states = self._create_test_orbits(n_orbits)
        dt = 3600.0  # 1 hour

        result = benchmark(batch_propagate_states, states, np.array([dt]), GM_EARTH)
        assert result.shape == (n_orbits, 6)

    def test_batch_propagation_medium(self, benchmark):
        """Batch propagation with 100 orbits (medium, expect 2-4x speedup)."""
        n_orbits = 100
        states = self._create_test_orbits(n_orbits)
        dt = 3600.0

        result = benchmark(batch_propagate_states, states, np.array([dt]), GM_EARTH)
        assert result.shape == (n_orbits, 6)

    def test_batch_propagation_large(self, benchmark):
        """Batch propagation with 1000 orbits (large, expect 2-8x speedup)."""
        n_orbits = 1000
        states = self._create_test_orbits(n_orbits)
        dt = 3600.0

        result = benchmark(batch_propagate_states, states, np.array([dt]), GM_EARTH)
        assert result.shape == (n_orbits, 6)

    def test_batch_propagation_very_large(self, benchmark):
        """Batch propagation with 5000 orbits (very large, maximum speedup expected)."""
        n_orbits = 5000
        states = self._create_test_orbits(n_orbits)
        dt = 3600.0

        result = benchmark(batch_propagate_states, states, np.array([dt]), GM_EARTH)
        assert result.shape == (n_orbits, 6)

    def test_batch_anomaly_conversion_small(self, benchmark):
        """Batch anomaly conversion with 100 orbits."""
        n_orbits = 100
        mean_anomalies = np.linspace(0, 2 * np.pi, n_orbits)
        eccentricities = np.array([0.5])

        result = benchmark(batch_mean_to_eccentric_anomaly, mean_anomalies, eccentricities)
        assert len(result) == n_orbits

    def test_batch_anomaly_conversion_large(self, benchmark):
        """Batch anomaly conversion with 10000 orbits."""
        n_orbits = 10000
        mean_anomalies = np.linspace(0, 2 * np.pi, n_orbits)
        eccentricities = np.array([0.5])

        result = benchmark(batch_mean_to_eccentric_anomaly, mean_anomalies, eccentricities)
        assert len(result) == n_orbits

    def test_batch_mean_to_true_large(self, benchmark):
        """Batch mean to true anomaly conversion with 10000 orbits."""
        n_orbits = 10000
        mean_anomalies = np.linspace(0, 2 * np.pi, n_orbits)
        eccentricities = np.array([0.6])

        result = benchmark(batch_mean_to_true_anomaly, mean_anomalies, eccentricities)
        assert len(result) == n_orbits

    @staticmethod
    def _create_test_orbits(n: int) -> np.ndarray:
        """Create n test orbits with varying altitudes and inclinations."""
        # Semi-major axes from 7000 km to 12000 km
        altitudes = np.linspace(7000e3, 12000e3, n)

        states = np.zeros((n, 6))
        for i, a in enumerate(altitudes):
            # Circular velocity
            v = np.sqrt(GM_EARTH / a)
            # Varying inclination from 0 to 90 degrees
            inc = np.radians(90.0 * i / n)

            # Position on x-axis
            states[i, 0] = a
            states[i, 1] = 0.0
            states[i, 2] = 0.0

            # Velocity perpendicular to position, with inclination
            states[i, 3] = 0.0
            states[i, 4] = v * np.cos(inc)
            states[i, 5] = v * np.sin(inc)

        return states


def manual_timing_comparison():
    """
    Manual timing comparison to demonstrate parallelization benefits.

    This is not a pytest benchmark but a demonstration script showing
    wall-clock time improvements.
    """
    print("\n" + "=" * 80)
    print("Rayon Parallelization Performance Demonstration")
    print("=" * 80)

    # Test different batch sizes
    sizes = [10, 100, 500, 1000, 2000, 5000]

    print("\nBatch Propagation Performance:")
    print("-" * 80)
    print(f"{'Batch Size':<12} {'Time (ms)':<15} {'Throughput (orbits/sec)':<25}")
    print("-" * 80)

    for n in sizes:
        # Create test data
        altitudes = np.linspace(7000e3, 12000e3, n)
        states = np.zeros((n, 6))
        for i, a in enumerate(altitudes):
            v = np.sqrt(GM_EARTH / a)
            states[i, 0] = a
            states[i, 4] = v

        dt = np.array([3600.0])

        # Warm-up
        _ = batch_propagate_states(states, dt, GM_EARTH)

        # Time multiple runs
        n_runs = 10 if n < 1000 else 5
        times = []
        for _ in range(n_runs):
            start = time.perf_counter()
            _ = batch_propagate_states(states, dt, GM_EARTH)
            end = time.perf_counter()
            times.append(end - start)

        avg_time = np.mean(times) * 1000  # Convert to ms
        throughput = n / (avg_time / 1000)

        print(f"{n:<12} {avg_time:>12.3f} ms  {throughput:>20.1f} orbits/sec")

    print("\n" + "=" * 80)
    print("Note: Performance scales with number of CPU cores available.")
    print("Expected speedup: 2-8x on multi-core systems for large batches.")
    print("=" * 80 + "\n")


if __name__ == "__main__":
    # Run manual timing comparison
    manual_timing_comparison()
