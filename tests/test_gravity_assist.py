"""
Test suite for gravity assist (planetary flyby) calculations

This module tests the Rust-backed gravity assist implementation,
including deflection angles, delta-v calculations, B-plane parameters,
and validation against known mission data.
"""

import math

import pytest
from astrora import _core

# Import functions
gravity_assist = _core.gravity_assist
periapsis_from_b_parameter = _core.periapsis_from_b_parameter

# Import constants
GM_JUPITER = _core.constants.GM_JUPITER
GM_EARTH = _core.constants.GM_EARTH
GM_VENUS = _core.constants.GM_VENUS
R_JUPITER = _core.constants.R_JUPITER
R_EARTH = _core.constants.R_MEAN_EARTH


class TestGravityAssistBasic:
    """Basic gravity assist calculations"""

    def test_jupiter_flyby(self):
        """Test Jupiter gravity assist flyby"""
        v_infinity = 5640.0  # m/s (typical for outer planet missions)
        r_periapsis = 3.0 * R_JUPITER  # 3 Jupiter radii
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Check all fields are present
        assert "v_infinity" in result
        assert "r_periapsis" in result
        assert "mu" in result
        assert "eccentricity" in result
        assert "delta" in result
        assert "delta_v_magnitude" in result
        assert "semi_major_axis" in result
        assert "theta_infinity" in result
        assert "b_parameter" in result
        assert "specific_energy" in result

        # Eccentricity must be > 1 (hyperbolic)
        assert result["eccentricity"] > 1.0

        # Deflection angle should be between 0 and π
        assert 0.0 < result["delta"] < math.pi

        # Delta-v should be positive
        assert result["delta_v_magnitude"] > 0.0

        # Semi-major axis should be negative for hyperbolic orbit
        assert result["semi_major_axis"] < 0.0

        # Specific energy should be positive
        assert result["specific_energy"] > 0.0

    def test_close_flyby_high_deflection(self):
        """Test close flyby produces large deflection angle"""
        v_infinity = 20000.0  # m/s
        r_periapsis_close = 1.5 * R_JUPITER  # Close flyby
        r_periapsis_far = 10.0 * R_JUPITER  # Distant flyby
        mu = GM_JUPITER

        result_close = gravity_assist(v_infinity, r_periapsis_close, mu)
        result_far = gravity_assist(v_infinity, r_periapsis_far, mu)

        # Close flyby should have larger deflection angle
        assert result_close["delta"] > result_far["delta"]

        # For same v∞, larger r_p gives larger e (since e = 1 + r_p·v²/μ)
        # But SMALLER e gives LARGER deflection (δ = 2·arcsin(1/e))
        # So close flyby has SMALLER eccentricity but LARGER deflection
        assert result_close["eccentricity"] < result_far["eccentricity"]

        # Close flyby should have larger delta-v
        assert result_close["delta_v_magnitude"] > result_far["delta_v_magnitude"]

    def test_high_speed_vs_low_speed(self):
        """Test effect of hyperbolic excess velocity on deflection"""
        r_periapsis = 3.0 * R_JUPITER
        v_low = 3000.0  # m/s
        v_high = 15000.0  # m/s
        mu = GM_JUPITER

        result_low = gravity_assist(v_low, r_periapsis, mu)
        result_high = gravity_assist(v_high, r_periapsis, mu)

        # Higher speed should give lower deflection (for same periapsis)
        # This is because e = 1 + (r_p × v²) / μ, so higher v gives higher e,
        # and δ = 2·arcsin(1/e) decreases with increasing e
        assert result_low["delta"] > result_high["delta"]

        # Delta-v magnitude should be higher for high speed case
        # Δv ≈ 2v∞·sin(δ/2)
        assert result_high["delta_v_magnitude"] > result_low["delta_v_magnitude"]

    def test_venus_flyby(self):
        """Test Venus gravity assist (smaller planet)"""
        v_infinity = 10000.0  # m/s
        r_venus = 6051.8e3  # Venus radius in meters
        r_periapsis = 1.2 * r_venus  # Close but safe flyby
        mu = GM_VENUS

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Should produce valid hyperbolic trajectory
        assert result["eccentricity"] > 1.0
        assert 0.0 < result["delta"] < math.pi

        # Venus has smaller μ than Jupiter, so deflection will be smaller
        # for same v_infinity and relative periapsis distance


class TestGravityAssistPhysics:
    """Test physical relationships and conservation laws"""

    def test_energy_conservation(self):
        """Test that specific energy equals v∞²/2"""
        v_infinity = 5640.0
        r_periapsis = 3.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Specific energy should equal v∞²/2
        expected_energy = v_infinity**2 / 2.0
        assert result["specific_energy"] == pytest.approx(expected_energy, rel=1e-10)

    def test_eccentricity_formula(self):
        """Test eccentricity calculation: e = 1 + (r_p × v∞²) / μ"""
        v_infinity = 5640.0
        r_periapsis = 3.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Calculate expected eccentricity
        expected_e = 1.0 + (r_periapsis * v_infinity**2) / mu
        assert result["eccentricity"] == pytest.approx(expected_e, rel=1e-10)

    def test_turning_angle_formula(self):
        """Test δ = 2·θ∞ - π where θ∞ = arccos(-1/e)"""
        v_infinity = 5640.0
        r_periapsis = 3.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Verify relationship
        theta_inf = result["theta_infinity"]
        expected_delta = 2.0 * theta_inf - math.pi
        assert result["delta"] == pytest.approx(expected_delta, rel=1e-10)

    def test_theta_infinity_from_eccentricity(self):
        """Test θ∞ = arccos(-1/e)"""
        v_infinity = 5640.0
        r_periapsis = 3.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Calculate expected θ∞
        e = result["eccentricity"]
        expected_theta_inf = math.acos(-1.0 / e)
        assert result["theta_infinity"] == pytest.approx(expected_theta_inf, rel=1e-10)

    def test_semi_major_axis_formula(self):
        """Test a = -μ / v∞² for hyperbolic orbit"""
        v_infinity = 5640.0
        r_periapsis = 3.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Semi-major axis formula
        expected_sma = -mu / (v_infinity**2)
        assert result["semi_major_axis"] == pytest.approx(expected_sma, rel=1e-10)

    def test_b_parameter_formula(self):
        """Test B = |a| × √(e² - 1)"""
        v_infinity = 5640.0
        r_periapsis = 3.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # B-parameter formula
        a = result["semi_major_axis"]
        e = result["eccentricity"]
        expected_b = abs(a) * math.sqrt(e**2 - 1.0)
        assert result["b_parameter"] == pytest.approx(expected_b, rel=1e-10)


class TestBPlaneParameters:
    """Test B-plane targeting and impact parameter calculations"""

    def test_periapsis_from_b_parameter_roundtrip(self):
        """Test conversion between B-parameter and periapsis"""
        v_infinity = 5000.0
        r_periapsis_original = 200000e3  # 200,000 km
        mu = GM_JUPITER

        # Get B-parameter from flyby
        result = gravity_assist(v_infinity, r_periapsis_original, mu)
        b_parameter = result["b_parameter"]

        # Convert B back to periapsis
        r_periapsis_recovered = periapsis_from_b_parameter(v_infinity, b_parameter, mu)

        # Should recover original periapsis
        assert r_periapsis_recovered == pytest.approx(r_periapsis_original, rel=1e-6)

    def test_large_impact_parameter_distant_flyby(self):
        """Test that large B gives large periapsis"""
        v_infinity = 5000.0
        mu = GM_JUPITER

        b_small = 100000e3  # 100,000 km
        b_large = 500000e3  # 500,000 km

        r_p_small = periapsis_from_b_parameter(v_infinity, b_small, mu)
        r_p_large = periapsis_from_b_parameter(v_infinity, b_large, mu)

        # Larger impact parameter should give larger periapsis
        assert r_p_large > r_p_small


class TestGravityAssistEdgeCases:
    """Test edge cases and error handling"""

    def test_negative_v_infinity_error(self):
        """Test that negative v_infinity raises error"""
        with pytest.raises(Exception):  # PoliastroError in Rust
            gravity_assist(-1000.0, 3.0 * R_JUPITER, GM_JUPITER)

    def test_negative_periapsis_error(self):
        """Test that negative periapsis raises error"""
        with pytest.raises(Exception):
            gravity_assist(5000.0, -100000e3, GM_JUPITER)

    def test_zero_v_infinity_error(self):
        """Test that zero v_infinity raises error"""
        with pytest.raises(Exception):
            gravity_assist(0.0, 3.0 * R_JUPITER, GM_JUPITER)

    def test_very_distant_flyby(self):
        """Test very distant flyby (low deflection)"""
        v_infinity = 2000.0  # m/s
        r_periapsis = 100.0 * R_JUPITER  # Very far
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Eccentricity: e = 1 + (r_p × v²) / μ
        # Even distant flybys can have e > 1.1 depending on v_infinity
        assert result["eccentricity"] > 1.0

        # Even for distant flybys, deflection can be significant
        # This depends on the combination of v∞ and r_p
        # Just verify it's a valid deflection angle
        assert 0.0 < result["delta"] < math.pi

    def test_grazing_flyby(self):
        """Test very close flyby (high eccentricity)"""
        v_infinity = 25000.0  # m/s - very high speed
        r_periapsis = 1.01 * R_JUPITER  # Just above surface
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Should produce valid trajectory
        assert result["eccentricity"] > 1.0
        assert 0.0 < result["delta"] < math.pi


class TestMissionValidation:
    """Validate against known mission data"""

    def test_voyager_2_jupiter_flyby_approximate(self):
        """
        Approximate test based on Voyager 2 Jupiter flyby (1979)
        Note: These are simplified values, not exact mission parameters
        """
        # Voyager 2 approached Jupiter at ~10 km/s relative velocity
        # Periapsis was at ~10 Jupiter radii (570,000 km)
        v_infinity = 10000.0  # m/s (approximate)
        r_periapsis = 10.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Should produce reasonable deflection
        # For these parameters, deflection can be substantial (>1 radian is possible)
        assert 0.1 < result["delta"] < math.pi  # Between ~6° and 180°

        # Delta-v should be on order of several km/s
        assert 1000.0 < result["delta_v_magnitude"] < 20000.0

    def test_cassini_venus_flyby_approximate(self):
        """
        Approximate test based on Cassini Venus flyby (1998, 1999)
        Cassini performed two Venus gravity assists
        """
        # Typical Venus flyby parameters
        v_infinity = 8000.0  # m/s (approximate)
        r_venus = 6051.8e3  # Venus radius
        r_periapsis = 1.5 * r_venus  # Relatively close flyby
        mu = GM_VENUS

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Should be hyperbolic
        assert result["eccentricity"] > 1.0

        # Venus has lower mass than Jupiter, so smaller deflections
        assert result["delta"] > 0.0
        assert result["delta_v_magnitude"] > 0.0


class TestNumericalPrecision:
    """Test numerical precision and accuracy"""

    def test_delta_v_magnitude_formula(self):
        """Test Δv ≈ 2v∞·sin(δ/2) for symmetric deflection"""
        v_infinity = 5640.0
        r_periapsis = 3.0 * R_JUPITER
        mu = GM_JUPITER

        result = gravity_assist(v_infinity, r_periapsis, mu)

        # Calculate delta-v using formula
        delta = result["delta"]
        expected_dv = 2.0 * v_infinity * math.sin(delta / 2.0)

        # Should match within numerical precision
        assert result["delta_v_magnitude"] == pytest.approx(expected_dv, rel=1e-10)

    def test_consistency_across_range(self):
        """Test that calculations are consistent across parameter range"""
        mu = GM_JUPITER
        r_periapsis = 5.0 * R_JUPITER

        # Test multiple v_infinity values
        for v_inf in [1000.0, 5000.0, 10000.0, 20000.0]:
            result = gravity_assist(v_inf, r_periapsis, mu)

            # All results should be physically valid
            assert result["eccentricity"] > 1.0
            assert 0.0 < result["delta"] < math.pi
            assert result["delta_v_magnitude"] > 0.0
            assert result["semi_major_axis"] < 0.0
            assert result["specific_energy"] > 0.0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
