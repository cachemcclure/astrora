"""Tests for anomaly conversion functions"""

import numpy as np
import pytest
from astrora._core import (
    eccentric_to_mean_anomaly,
    eccentric_to_true_anomaly,
    hyperbolic_to_mean_anomaly,
    hyperbolic_to_true_anomaly,
    # Elliptical orbit conversions
    mean_to_eccentric_anomaly,
    # Hyperbolic orbit conversions
    mean_to_hyperbolic_anomaly,
    mean_to_true_anomaly,
    mean_to_true_anomaly_hyperbolic,
    # Parabolic orbit conversions
    mean_to_true_anomaly_parabolic,
    true_to_eccentric_anomaly,
    true_to_hyperbolic_anomaly,
    true_to_mean_anomaly,
    true_to_mean_anomaly_hyperbolic,
    true_to_mean_anomaly_parabolic,
)

# ============================================================================
# Elliptical Orbit Tests
# ============================================================================


class TestEllipticalOrbits:
    """Tests for elliptical orbit anomaly conversions (e < 1)"""

    def test_mean_to_eccentric_circular(self):
        """For circular orbits (e=0), E should equal M"""
        M = 1.5
        e = 0.0
        E = mean_to_eccentric_anomaly(M, e)
        assert abs(E - M) < 1e-10

    def test_mean_to_eccentric_moderate_ecc(self):
        """Test M → E conversion for moderate eccentricity"""
        M = 1.0
        e = 0.5
        E = mean_to_eccentric_anomaly(M, e)

        # Verify solution satisfies Kepler's equation
        M_check = E - e * np.sin(E)
        assert abs(M_check - M) < 1e-10

    def test_mean_to_eccentric_high_ecc(self):
        """Test M → E conversion for high eccentricity (but still elliptical)"""
        M = 0.5
        e = 0.95
        E = mean_to_eccentric_anomaly(M, e)

        M_check = E - e * np.sin(E)
        assert abs(M_check - M) < 1e-10

    def test_eccentric_to_mean_direct(self):
        """Test E → M direct conversion"""
        E = 1.2
        e = 0.3
        M = eccentric_to_mean_anomaly(E, e)

        # Verify Kepler's equation
        M_expected = E - e * np.sin(E)
        assert abs(M - M_expected) < 1e-10

    def test_eccentric_to_true_at_periapsis(self):
        """At periapsis (E=0), true anomaly should also be 0"""
        E = 0.0
        e = 0.6
        nu = eccentric_to_true_anomaly(E, e)
        assert abs(nu) < 1e-10

    def test_eccentric_to_true_at_apoapsis(self):
        """At apoapsis (E=π), true anomaly should also be π"""
        E = np.pi
        e = 0.4
        nu = eccentric_to_true_anomaly(E, e)
        assert abs(nu - np.pi) < 1e-10

    def test_eccentric_to_true_general(self):
        """Test E → ν conversion with verification"""
        E = 1.5
        e = 0.7
        nu = eccentric_to_true_anomaly(E, e)

        # Verify using orbit equation
        # cos(ν) = (cos(E) - e) / (1 - e·cos(E))
        cos_nu_expected = (np.cos(E) - e) / (1 - e * np.cos(E))
        assert abs(np.cos(nu) - cos_nu_expected) < 1e-10

    def test_true_to_eccentric_roundtrip(self):
        """Test ν → E → ν roundtrip"""
        nu_orig = 2.0
        e = 0.5

        E = true_to_eccentric_anomaly(nu_orig, e)
        nu_check = eccentric_to_true_anomaly(E, e)

        assert abs(nu_check - nu_orig) < 1e-10

    def test_mean_to_true_roundtrip(self):
        """Test M → ν → M roundtrip"""
        M_orig = 1.8
        e = 0.6

        nu = mean_to_true_anomaly(M_orig, e)
        M_check = true_to_mean_anomaly(nu, e)

        assert abs(M_check - M_orig) < 1e-10

    def test_complete_conversion_chain(self):
        """Test complete conversion chain: M → E → ν → E → M"""
        M_orig = 2.5
        e = 0.4

        E1 = mean_to_eccentric_anomaly(M_orig, e)
        nu = eccentric_to_true_anomaly(E1, e)
        E2 = true_to_eccentric_anomaly(nu, e)
        M_final = eccentric_to_mean_anomaly(E2, e)

        assert abs(E2 - E1) < 1e-10
        assert abs(M_final - M_orig) < 1e-10

    def test_mean_to_eccentric_multiple_values(self):
        """Test M → E for multiple mean anomalies"""
        M_values = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0]
        e = 0.5

        for M in M_values:
            E = mean_to_eccentric_anomaly(M, e)
            M_check = E - e * np.sin(E)
            assert abs(M_check - M) < 1e-10

    def test_rejects_hyperbolic_eccentricity(self):
        """Should raise error for hyperbolic eccentricity"""
        M = 1.0
        e = 1.5  # hyperbolic

        with pytest.raises(ValueError):
            mean_to_eccentric_anomaly(M, e)

    def test_rejects_parabolic_eccentricity(self):
        """Should raise error for parabolic eccentricity"""
        M = 1.0
        e = 1.0  # parabolic

        with pytest.raises(ValueError):
            mean_to_eccentric_anomaly(M, e)

    def test_rejects_negative_eccentricity(self):
        """Should raise error for negative eccentricity"""
        M = 1.0
        e = -0.1

        with pytest.raises(ValueError):
            mean_to_eccentric_anomaly(M, e)


# ============================================================================
# Hyperbolic Orbit Tests
# ============================================================================


class TestHyperbolicOrbits:
    """Tests for hyperbolic orbit anomaly conversions (e > 1)"""

    def test_mean_to_hyperbolic_basic(self):
        """Test M → H conversion for hyperbolic orbit"""
        M = 2.0
        e = 1.5
        H = mean_to_hyperbolic_anomaly(M, e)

        # Verify solution satisfies hyperbolic Kepler's equation
        M_check = e * np.sinh(H) - H
        assert abs(M_check - M) < 1e-10

    def test_mean_to_hyperbolic_high_ecc(self):
        """Test M → H for very hyperbolic orbit"""
        M = 5.0
        e = 3.0
        H = mean_to_hyperbolic_anomaly(M, e)

        M_check = e * np.sinh(H) - H
        assert abs(M_check - M) < 1e-10

    def test_hyperbolic_to_mean_direct(self):
        """Test H → M direct conversion"""
        H = 1.5
        e = 2.0
        M = hyperbolic_to_mean_anomaly(H, e)

        M_expected = e * np.sinh(H) - H
        assert abs(M - M_expected) < 1e-10

    def test_hyperbolic_to_true_at_periapsis(self):
        """At periapsis (H=0), true anomaly should be 0"""
        H = 0.0
        e = 1.8
        nu = hyperbolic_to_true_anomaly(H, e)
        assert abs(nu) < 1e-10

    def test_hyperbolic_to_true_general(self):
        """Test H → ν conversion"""
        H = 1.0
        e = 2.5
        nu = hyperbolic_to_true_anomaly(H, e)

        # Verify using formula: cos(ν) = (e - cosh(H)) / (e·cosh(H) - 1)
        cos_nu_expected = (e - np.cosh(H)) / (e * np.cosh(H) - 1)
        assert abs(np.cos(nu) - cos_nu_expected) < 1e-10

    def test_true_to_hyperbolic_roundtrip(self):
        """Test ν → H → ν roundtrip"""
        nu_orig = 0.8
        e = 2.0

        H = true_to_hyperbolic_anomaly(nu_orig, e)
        nu_check = hyperbolic_to_true_anomaly(H, e)

        assert abs(nu_check - nu_orig) < 1e-10

    def test_mean_to_true_hyperbolic_roundtrip(self):
        """Test M → ν → M roundtrip for hyperbolic orbit"""
        M_orig = 3.0
        e = 1.8

        nu = mean_to_true_anomaly_hyperbolic(M_orig, e)
        M_check = true_to_mean_anomaly_hyperbolic(nu, e)

        assert abs(M_check - M_orig) < 1e-10

    def test_complete_hyperbolic_chain(self):
        """Test complete hyperbolic conversion chain"""
        M_orig = 2.5
        e = 2.2

        H1 = mean_to_hyperbolic_anomaly(M_orig, e)
        nu = hyperbolic_to_true_anomaly(H1, e)
        H2 = true_to_hyperbolic_anomaly(nu, e)
        M_final = hyperbolic_to_mean_anomaly(H2, e)

        assert abs(H2 - H1) < 1e-10
        assert abs(M_final - M_orig) < 1e-10

    def test_mean_to_hyperbolic_negative(self):
        """Test hyperbolic conversion with negative mean anomaly"""
        M = -2.0
        e = 1.6
        H = mean_to_hyperbolic_anomaly(M, e)

        M_check = e * np.sinh(H) - H
        assert abs(M_check - M) < 1e-10

    def test_rejects_elliptical_eccentricity(self):
        """Should raise error for elliptical eccentricity"""
        M = 1.0
        e = 0.5  # elliptical

        with pytest.raises(ValueError):
            mean_to_hyperbolic_anomaly(M, e)


# ============================================================================
# Parabolic Orbit Tests
# ============================================================================


class TestParabolicOrbits:
    """Tests for parabolic orbit anomaly conversions (e = 1)"""

    def test_parabolic_mean_to_true_zero(self):
        """At periapsis (M=0), true anomaly should be 0"""
        M = 0.0
        nu = mean_to_true_anomaly_parabolic(M)
        assert abs(nu) < 1e-10

    def test_parabolic_mean_to_true_positive(self):
        """Test M → ν for positive mean anomaly"""
        M = 0.5
        nu = mean_to_true_anomaly_parabolic(M)

        # Verify with Barker's equation
        D = np.tan(nu / 2.0)
        M_check = D + D**3 / 3.0
        assert abs(M_check - M) < 1e-10

    def test_parabolic_mean_to_true_negative(self):
        """Test M → ν for negative mean anomaly"""
        M = -0.8
        nu = mean_to_true_anomaly_parabolic(M)

        D = np.tan(nu / 2.0)
        M_check = D + D**3 / 3.0
        assert abs(M_check - M) < 1e-10

    def test_parabolic_true_to_mean_direct(self):
        """Test ν → M direct conversion"""
        nu = 1.5
        M = true_to_mean_anomaly_parabolic(nu)

        D = np.tan(nu / 2.0)
        M_expected = D + D**3 / 3.0
        assert abs(M - M_expected) < 1e-10

    def test_parabolic_roundtrip(self):
        """Test M → ν → M roundtrip"""
        M_orig = 1.2

        nu = mean_to_true_anomaly_parabolic(M_orig)
        M_check = true_to_mean_anomaly_parabolic(nu)

        assert abs(M_check - M_orig) < 1e-10

    def test_parabolic_multiple_values(self):
        """Test parabolic conversions for multiple mean anomalies"""
        M_values = [-1.0, -0.5, 0.0, 0.5, 1.0, 1.5]

        for M in M_values:
            nu = mean_to_true_anomaly_parabolic(M)
            M_check = true_to_mean_anomaly_parabolic(nu)
            assert abs(M_check - M) < 1e-10

    def test_parabolic_symmetry(self):
        """Test that parabolic conversions are symmetric"""
        M = 0.8

        nu_pos = mean_to_true_anomaly_parabolic(M)
        nu_neg = mean_to_true_anomaly_parabolic(-M)

        # True anomalies should be opposite
        # Note: Need to handle wrapping around 2π
        assert abs(nu_pos + nu_neg) < 1e-10 or abs(nu_pos + nu_neg - 2 * np.pi) < 1e-10


# ============================================================================
# Edge Cases and Special Values
# ============================================================================


class TestEdgeCases:
    """Tests for edge cases and special values"""

    def test_zero_mean_anomaly_all_types(self):
        """Test M=0 for all orbit types"""
        # Elliptical
        E = mean_to_eccentric_anomaly(0.0, 0.5)
        assert abs(E) < 1e-10

        # Hyperbolic
        H = mean_to_hyperbolic_anomaly(0.0, 1.5)
        assert abs(H) < 1e-10

        # Parabolic
        nu = mean_to_true_anomaly_parabolic(0.0)
        assert abs(nu) < 1e-10

    def test_very_small_eccentricity(self):
        """Test near-circular orbit"""
        M = 1.5
        e = 1e-10  # Nearly circular
        E = mean_to_eccentric_anomaly(M, e)

        # For very small e, E ≈ M
        assert abs(E - M) < 1e-8

    def test_tolerance_parameter(self):
        """Test custom tolerance parameter"""
        M = 1.0
        e = 0.5

        # Should converge with relaxed tolerance
        E = mean_to_eccentric_anomaly(M, e, tol=1e-6)
        M_check = E - e * np.sin(E)
        assert abs(M_check - M) < 1e-6

    def test_max_iter_parameter(self):
        """Test custom max_iter parameter"""
        M = 1.0
        e = 0.5

        # Should converge with more iterations allowed
        E = mean_to_eccentric_anomaly(M, e, max_iter=100)
        M_check = E - e * np.sin(E)
        assert abs(M_check - M) < 1e-10

    def test_large_mean_anomaly(self):
        """Test with large mean anomaly (should normalize)"""
        M = 10.0  # More than 2π
        e = 0.5
        E = mean_to_eccentric_anomaly(M, e)

        # Should still satisfy Kepler's equation
        M_normalized = M % (2 * np.pi)
        M_check = (E - e * np.sin(E)) % (2 * np.pi)
        assert abs(M_check - M_normalized) < 1e-10


# ============================================================================
# Accuracy and Numerical Precision Tests
# ============================================================================


class TestNumericalAccuracy:
    """Tests for numerical accuracy and precision"""

    def test_elliptical_high_precision(self):
        """Test high precision for elliptical orbits"""
        M = np.pi / 3.0
        e = 0.8

        E = mean_to_eccentric_anomaly(M, e, tol=1e-14)
        M_check = E - e * np.sin(E)

        # Should achieve very high precision
        assert abs(M_check - M) < 1e-13

    def test_hyperbolic_high_precision(self):
        """Test high precision for hyperbolic orbits"""
        M = 2.0
        e = 2.5

        H = mean_to_hyperbolic_anomaly(M, e, tol=1e-14)
        M_check = e * np.sinh(H) - H

        assert abs(M_check - M) < 1e-13

    def test_consistency_across_quadrants(self):
        """Test consistency across all four quadrants"""
        e = 0.6
        M_values = [
            0.5,  # Q1
            np.pi / 2 + 0.5,  # Q2
            np.pi + 0.5,  # Q3
            3 * np.pi / 2 + 0.5,  # Q4
        ]

        for M in M_values:
            E = mean_to_eccentric_anomaly(M, e)
            M_check = (E - e * np.sin(E)) % (2 * np.pi)
            M_normalized = M % (2 * np.pi)
            assert abs(M_check - M_normalized) < 1e-10


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
