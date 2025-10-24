"""
Tests for batch anomaly conversion functions.

These functions provide 10-20x performance improvement over sequential processing
by minimizing Python-Rust boundary crossings.
"""

import numpy as np
import pytest
from astrora._core import (
    batch_mean_to_eccentric_anomaly,
    batch_mean_to_hyperbolic_anomaly,
    batch_mean_to_true_anomaly,
    batch_mean_to_true_anomaly_hyperbolic,
    batch_mean_to_true_anomaly_parabolic,
    batch_true_to_mean_anomaly,
    # Individual functions for comparison
    mean_to_eccentric_anomaly,
    mean_to_hyperbolic_anomaly,
    mean_to_true_anomaly,
    mean_to_true_anomaly_hyperbolic,
    mean_to_true_anomaly_parabolic,
    true_to_mean_anomaly,
)


class TestBatchEllipticalAnomalies:
    """Test batch conversion functions for elliptical orbits"""

    def test_batch_mean_to_eccentric_single_eccentricity(self):
        """Test batch conversion with single eccentricity for all orbits"""
        mean_anomalies = np.array([0.5, 1.0, 1.5, 2.0, 2.5, 3.0])
        eccentricity = np.array([0.5])

        results = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricity)

        assert results.shape == mean_anomalies.shape
        assert isinstance(results, np.ndarray)

        # Verify each result matches individual calculation
        for i, M in enumerate(mean_anomalies):
            E_individual = mean_to_eccentric_anomaly(M, eccentricity[0])
            np.testing.assert_allclose(results[i], E_individual, rtol=1e-10)

    def test_batch_mean_to_eccentric_multiple_eccentricities(self):
        """Test batch conversion with different eccentricity for each orbit"""
        mean_anomalies = np.array([0.5, 1.0, 1.5, 2.0])
        eccentricities = np.array([0.2, 0.4, 0.6, 0.8])

        results = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricities)

        assert results.shape == mean_anomalies.shape

        # Verify each result
        for i in range(len(mean_anomalies)):
            E_individual = mean_to_eccentric_anomaly(mean_anomalies[i], eccentricities[i])
            np.testing.assert_allclose(results[i], E_individual, rtol=1e-10)

    def test_batch_mean_to_true_elliptical(self):
        """Test batch M → ν conversion for elliptical orbits"""
        mean_anomalies = np.array([0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0])
        eccentricity = np.array([0.6])

        results = batch_mean_to_true_anomaly(mean_anomalies, eccentricity)

        assert results.shape == mean_anomalies.shape

        # Verify roundtrip conversion
        mean_check = batch_true_to_mean_anomaly(results, eccentricity)
        np.testing.assert_allclose(mean_check, mean_anomalies, rtol=1e-10)

    def test_batch_true_to_mean_elliptical(self):
        """Test batch ν → M conversion for elliptical orbits"""
        true_anomalies = np.array([0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0])
        eccentricity = np.array([0.3])

        results = batch_true_to_mean_anomaly(true_anomalies, eccentricity)

        assert results.shape == true_anomalies.shape

        # Verify each result matches individual calculation
        for i, nu in enumerate(true_anomalies):
            M_individual = true_to_mean_anomaly(nu, eccentricity[0])
            np.testing.assert_allclose(results[i], M_individual, rtol=1e-10)

    def test_batch_circular_orbit(self):
        """Test batch conversion for circular orbits (e ≈ 0)"""
        mean_anomalies = np.linspace(0, 2 * np.pi, 10, endpoint=False)  # Avoid 2π wrapping
        eccentricity = np.array([0.0])

        results = batch_mean_to_true_anomaly(mean_anomalies, eccentricity)

        # For circular orbits, M ≈ E ≈ ν
        np.testing.assert_allclose(results, mean_anomalies, rtol=1e-8)

    def test_batch_high_eccentricity(self):
        """Test batch conversion with high eccentricity"""
        mean_anomalies = np.array([0.1, 0.5, 1.0, 1.5])
        eccentricity = np.array([0.9])  # High eccentricity

        results = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricity)

        # Verify Kepler's equation is satisfied
        for i, M in enumerate(mean_anomalies):
            E = results[i]
            M_check = E - eccentricity[0] * np.sin(E)
            np.testing.assert_allclose(M_check % (2 * np.pi), M % (2 * np.pi), rtol=1e-10)

    def test_batch_large_array(self):
        """Test batch processing with large array (performance pattern)"""
        n = 1000
        mean_anomalies = np.linspace(0, 10 * np.pi, n)
        eccentricity = np.array([0.5])

        results = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricity)

        assert results.shape == (n,)
        assert np.all(np.isfinite(results))

        # Spot check a few values
        for i in [0, 250, 500, 750, 999]:
            E_individual = mean_to_eccentric_anomaly(mean_anomalies[i], eccentricity[0])
            np.testing.assert_allclose(results[i], E_individual, rtol=1e-10)


class TestBatchHyperbolicAnomalies:
    """Test batch conversion functions for hyperbolic orbits"""

    def test_batch_mean_to_hyperbolic(self):
        """Test batch M → H conversion for hyperbolic orbits"""
        mean_anomalies = np.array([1.0, 2.0, 3.0, 4.0])
        eccentricity = np.array([1.5])

        results = batch_mean_to_hyperbolic_anomaly(mean_anomalies, eccentricity)

        assert results.shape == mean_anomalies.shape

        # Verify each result
        for i, M in enumerate(mean_anomalies):
            H_individual = mean_to_hyperbolic_anomaly(M, eccentricity[0])
            np.testing.assert_allclose(results[i], H_individual, rtol=1e-10)

    def test_batch_mean_to_true_hyperbolic(self):
        """Test batch M → ν conversion for hyperbolic orbits"""
        mean_anomalies = np.array([0.5, 1.0, 1.5, 2.0])
        eccentricities = np.array([1.2, 1.5, 2.0, 2.5])

        results = batch_mean_to_true_anomaly_hyperbolic(mean_anomalies, eccentricities)

        assert results.shape == mean_anomalies.shape

        # Verify each result matches individual calculation
        for i in range(len(mean_anomalies)):
            nu_individual = mean_to_true_anomaly_hyperbolic(mean_anomalies[i], eccentricities[i])
            np.testing.assert_allclose(results[i], nu_individual, rtol=1e-10)

    def test_batch_hyperbolic_single_eccentricity(self):
        """Test batch hyperbolic conversion with single eccentricity"""
        mean_anomalies = np.array([0.5, 1.0, 2.0, 3.0, 5.0])
        eccentricity = np.array([2.0])

        results = batch_mean_to_true_anomaly_hyperbolic(mean_anomalies, eccentricity)

        assert results.shape == mean_anomalies.shape
        assert np.all(np.isfinite(results))

    def test_batch_hyperbolic_various_eccentricities(self):
        """Test batch with various hyperbolic eccentricities"""
        mean_anomalies = np.array([1.0, 2.0, 3.0])
        eccentricities = np.array([1.2, 2.0, 5.0])

        results = batch_mean_to_hyperbolic_anomaly(mean_anomalies, eccentricities)

        # Verify hyperbolic Kepler equation: M = e·sinh(H) - H
        for i in range(len(mean_anomalies)):
            H = results[i]
            e = eccentricities[i]
            M_check = e * np.sinh(H) - H
            np.testing.assert_allclose(M_check, mean_anomalies[i], rtol=1e-10)


class TestBatchParabolicAnomalies:
    """Test batch conversion functions for parabolic orbits"""

    def test_batch_mean_to_true_parabolic(self):
        """Test batch M → ν conversion for parabolic orbits"""
        mean_anomalies = np.array([0.0, 0.5, 1.0, 1.5, -0.5, -1.0])

        results = batch_mean_to_true_anomaly_parabolic(mean_anomalies)

        assert results.shape == mean_anomalies.shape

        # Verify each result
        for i, M in enumerate(mean_anomalies):
            nu_individual = mean_to_true_anomaly_parabolic(M)
            np.testing.assert_allclose(results[i], nu_individual, rtol=1e-10)

    def test_batch_parabolic_zero(self):
        """Test batch parabolic conversion at periapsis"""
        mean_anomalies = np.array([0.0, 0.0, 0.0])

        results = batch_mean_to_true_anomaly_parabolic(mean_anomalies)

        np.testing.assert_allclose(results, 0.0, atol=1e-10)

    def test_batch_parabolic_symmetric(self):
        """Test batch parabolic conversion symmetry"""
        mean_anomalies = np.array([0.5, 1.0, 1.5, 2.0])
        mean_anomalies_neg = -mean_anomalies

        results_pos = batch_mean_to_true_anomaly_parabolic(mean_anomalies)
        results_neg = batch_mean_to_true_anomaly_parabolic(mean_anomalies_neg)

        # Results should be symmetric
        expected_neg = (-results_pos) % (2 * np.pi)
        np.testing.assert_allclose(results_neg, expected_neg, rtol=1e-10, atol=1e-10)


class TestBatchErrorHandling:
    """Test error handling for batch functions"""

    def test_batch_length_mismatch(self):
        """Test error when array lengths don't match"""
        mean_anomalies = np.array([0.5, 1.0, 1.5, 2.0])
        eccentricities = np.array([0.2, 0.4])  # Wrong length

        with pytest.raises(ValueError):
            batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricities)

    def test_batch_invalid_orbit_type_elliptical(self):
        """Test error when using hyperbolic e with elliptical function"""
        mean_anomalies = np.array([0.5, 1.0])
        eccentricity = np.array([1.5])  # Hyperbolic, not elliptical

        with pytest.raises(ValueError):
            batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricity)

    def test_batch_invalid_orbit_type_hyperbolic(self):
        """Test error when using elliptical e with hyperbolic function"""
        mean_anomalies = np.array([0.5, 1.0])
        eccentricity = np.array([0.5])  # Elliptical, not hyperbolic

        with pytest.raises(ValueError):
            batch_mean_to_hyperbolic_anomaly(mean_anomalies, eccentricity)

    def test_batch_empty_array(self):
        """Test batch processing with empty arrays"""
        mean_anomalies = np.array([])
        eccentricity = np.array([0.5])

        results = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricity)

        assert results.shape == (0,)


class TestBatchPerformancePattern:
    """Test performance characteristics of batch functions"""

    def test_batch_vs_sequential_correctness(self):
        """Verify batch and sequential produce identical results"""
        n = 50
        mean_anomalies = np.random.uniform(0, 2 * np.pi, n)
        eccentricity = np.array([0.5])

        # Batch processing
        results_batch = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricity)

        # Sequential processing
        results_sequential = np.array(
            [mean_to_eccentric_anomaly(M, eccentricity[0]) for M in mean_anomalies]
        )

        np.testing.assert_allclose(results_batch, results_sequential, rtol=1e-10)

    def test_batch_various_sizes(self):
        """Test batch processing with various array sizes"""
        sizes = [1, 10, 100, 500]
        eccentricity = np.array([0.6])

        for size in sizes:
            mean_anomalies = np.linspace(0, 4 * np.pi, size)
            results = batch_mean_to_true_anomaly(mean_anomalies, eccentricity)

            assert results.shape == (size,)
            assert np.all(np.isfinite(results))

    def test_batch_broadcasting_single_eccentricity(self):
        """Test that single eccentricity is broadcasted correctly"""
        mean_anomalies = np.array([0.5, 1.0, 1.5, 2.0, 2.5])
        eccentricity = np.array([0.7])  # Single value

        results = batch_mean_to_eccentric_anomaly(mean_anomalies, eccentricity)

        # All should use same eccentricity
        for i, M in enumerate(mean_anomalies):
            expected = mean_to_eccentric_anomaly(M, eccentricity[0])
            np.testing.assert_allclose(results[i], expected, rtol=1e-10)

    def test_batch_multiple_eccentricities_per_orbit(self):
        """Test batch with different eccentricity per orbit"""
        n = 20
        mean_anomalies = np.linspace(0, 2 * np.pi, n)
        eccentricities = np.linspace(0.1, 0.9, n)

        results = batch_mean_to_true_anomaly(mean_anomalies, eccentricities)

        assert results.shape == (n,)

        # Verify a few random samples
        for i in [0, 5, 10, 15, 19]:
            expected = mean_to_true_anomaly(mean_anomalies[i], eccentricities[i])
            np.testing.assert_allclose(results[i], expected, rtol=1e-10)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
