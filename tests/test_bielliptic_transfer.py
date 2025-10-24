"""
Tests for bi-elliptic transfer calculations

Tests the Python bindings for bi-elliptic orbital transfers, including:
- Basic calculations and delta-v computation
- Comparison with Hohmann transfers
- Efficiency threshold validation
- Transfer time calculations
- Optimal intermediate radius search
"""

import math

import pytest
from astrora._core import (
    bielliptic_transfer,
    compare_bielliptic_hohmann,
    constants,
    find_optimal_bielliptic,
)

GM_EARTH = constants.GM_EARTH
R_MEAN_EARTH = constants.R_MEAN_EARTH


# Test fixtures
@pytest.fixture
def leo_radius():
    """LEO orbit radius (400 km altitude)"""
    return R_MEAN_EARTH + 400e3


@pytest.fixture
def geo_radius():
    """GEO orbit radius (35,786 km altitude)"""
    return R_MEAN_EARTH + 35_786e3


@pytest.fixture
def high_radius():
    """Very high orbit (20x LEO)"""
    return (R_MEAN_EARTH + 400e3) * 20.0


class TestBiellipticTransferBasic:
    """Basic bi-elliptic transfer calculation tests"""

    def test_basic_calculation(self, leo_radius, geo_radius):
        """Test basic bi-elliptic transfer calculation"""
        r_intermediate = geo_radius * 3.0
        result = bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Verify structure
        assert "delta_v1" in result
        assert "delta_v2" in result
        assert "delta_v3" in result
        assert "delta_v_total" in result
        assert "transfer_time" in result
        assert "transfer1_sma" in result
        assert "transfer2_sma" in result

        # Verify all delta-vs are positive
        assert result["delta_v1"] > 0
        assert result["delta_v2"] > 0
        assert result["delta_v3"] > 0
        assert result["delta_v_total"] > 0

        # Verify delta_v_total is sum of components
        assert (
            abs(
                result["delta_v_total"]
                - (result["delta_v1"] + result["delta_v2"] + result["delta_v3"])
            )
            < 1e-6
        )

    def test_transfer_time_positive(self, leo_radius, geo_radius):
        """Test that transfer time is positive and reasonable"""
        r_intermediate = geo_radius * 5.0
        result = bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Transfer time should be positive
        assert result["transfer_time"] > 0

        # Should be longer than a typical Hohmann transfer (5.3 hours)
        hohmann_time = 5.3 * 3600  # seconds
        assert result["transfer_time"] > hohmann_time

    def test_transfer_orbit_properties(self, leo_radius, geo_radius):
        """Test that transfer orbit properties are correctly calculated"""
        r_intermediate = geo_radius * 4.0
        result = bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Semi-major axes should be correct
        expected_sma1 = (leo_radius + r_intermediate) / 2.0
        expected_sma2 = (geo_radius + r_intermediate) / 2.0

        assert abs(result["transfer1_sma"] - expected_sma1) < 1.0
        assert abs(result["transfer2_sma"] - expected_sma2) < 1.0

        # Eccentricities should be between 0 and 1
        assert 0 < result["transfer1_eccentricity"] < 1
        assert 0 < result["transfer2_eccentricity"] < 1

    def test_circular_orbit_velocities(self, leo_radius, geo_radius):
        """Test that circular orbit velocities are correctly calculated"""
        r_intermediate = geo_radius * 3.0
        result = bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Check initial and final circular orbit velocities
        expected_v_initial = math.sqrt(GM_EARTH / leo_radius)
        expected_v_final = math.sqrt(GM_EARTH / geo_radius)

        assert abs(result["v_initial"] - expected_v_initial) < 1.0
        assert abs(result["v_final"] - expected_v_final) < 1.0


class TestBiellipticVsHohmann:
    """Tests comparing bi-elliptic and Hohmann transfers"""

    def test_comparison_structure(self, leo_radius, high_radius):
        """Test that comparison returns correct structure"""
        r_intermediate = high_radius * 5.0
        result = compare_bielliptic_hohmann(leo_radius, high_radius, r_intermediate, GM_EARTH)

        assert "bielliptic" in result
        assert "hohmann" in result
        assert "dv_savings" in result
        assert "time_penalty" in result

        # Both transfers should have delta_v_total
        assert "delta_v_total" in result["bielliptic"]
        assert "delta_v_total" in result["hohmann"]

    def test_large_ratio_efficiency(self, leo_radius):
        """Test that bi-elliptic is more efficient for large radius ratios"""
        # Use 25x radius ratio (well above 15.58 threshold)
        r_final = leo_radius * 25.0
        r_intermediate = r_final * 5.0

        result = compare_bielliptic_hohmann(leo_radius, r_final, r_intermediate, GM_EARTH)

        # Bi-elliptic should save delta-v
        assert result["dv_savings"] > 0

        # But should take longer
        assert result["time_penalty"] > 1.0

    def test_small_ratio_inefficiency(self, leo_radius, geo_radius):
        """Test that bi-elliptic is less efficient for small radius ratios"""
        # GEO/LEO ratio is about 6.6 (below 15.58 threshold)
        r_intermediate = geo_radius * 2.0

        result = compare_bielliptic_hohmann(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Hohmann should be more efficient (negative savings)
        assert result["dv_savings"] < 0

    def test_time_penalty_grows_with_intermediate(self, leo_radius, high_radius):
        """Test that time penalty increases with larger intermediate radius"""
        r_int_small = high_radius * 2.0
        r_int_large = high_radius * 10.0

        result_small = compare_bielliptic_hohmann(leo_radius, high_radius, r_int_small, GM_EARTH)
        result_large = compare_bielliptic_hohmann(leo_radius, high_radius, r_int_large, GM_EARTH)

        # Larger intermediate radius should mean longer transfer time
        assert result_large["time_penalty"] > result_small["time_penalty"]


class TestOptimalIntermediate:
    """Tests for finding optimal intermediate radius"""

    def test_finds_optimal(self, leo_radius, high_radius):
        """Test that optimal intermediate radius is found"""
        r_opt, result = find_optimal_bielliptic(leo_radius, high_radius, GM_EARTH, 50.0)

        # Optimal radius should be larger than both orbits
        assert r_opt > max(leo_radius, high_radius)

        # Result should be valid bi-elliptic transfer
        assert "delta_v_total" in result
        assert result["delta_v_total"] > 0

        # Intermediate radius in result should match r_opt
        assert abs(result["r_intermediate"] - r_opt) < 1.0

    def test_optimal_saves_deltaV(self, leo_radius):
        """Test that optimal bi-elliptic saves delta-v for large ratios"""
        # Large radius ratio where bi-elliptic should be better
        r_final = leo_radius * 30.0

        r_opt, bielliptic_result = find_optimal_bielliptic(leo_radius, r_final, GM_EARTH, 100.0)

        # Compare with Hohmann
        comparison = compare_bielliptic_hohmann(leo_radius, r_final, r_opt, GM_EARTH)

        # Should save delta-v
        assert comparison["dv_savings"] > 0


class TestParameterValidation:
    """Tests for parameter validation and error handling"""

    def test_negative_radii_rejected(self):
        """Test that negative radii are rejected"""
        with pytest.raises(Exception):
            bielliptic_transfer(-1e6, 2e6, 3e6, GM_EARTH)

        with pytest.raises(Exception):
            bielliptic_transfer(1e6, -2e6, 3e6, GM_EARTH)

        with pytest.raises(Exception):
            bielliptic_transfer(1e6, 2e6, -3e6, GM_EARTH)

    def test_equal_radii_rejected(self, leo_radius):
        """Test that equal initial and final radii are rejected"""
        r_intermediate = leo_radius * 2.0
        with pytest.raises(Exception):
            bielliptic_transfer(leo_radius, leo_radius, r_intermediate, GM_EARTH)

    def test_intermediate_too_small_rejected(self, leo_radius, geo_radius):
        """Test that intermediate radius must be larger than both orbits"""
        # r_intermediate smaller than geo_radius should fail
        r_intermediate = (leo_radius + geo_radius) / 2.0
        with pytest.raises(Exception):
            bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

    def test_negative_mu_rejected(self, leo_radius, geo_radius):
        """Test that negative gravitational parameter is rejected"""
        r_intermediate = geo_radius * 3.0
        with pytest.raises(Exception):
            bielliptic_transfer(leo_radius, geo_radius, r_intermediate, -GM_EARTH)


class TestPhysicalRealism:
    """Tests verifying physical correctness"""

    def test_energy_relationships(self, leo_radius, geo_radius):
        """Test that energy changes are consistent"""
        r_intermediate = geo_radius * 3.0
        result = bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Specific orbital energies
        epsilon_initial = -GM_EARTH / (2.0 * leo_radius)
        epsilon_final = -GM_EARTH / (2.0 * geo_radius)

        # Energy should increase (become less negative) for outward transfer
        assert epsilon_final > epsilon_initial

        # Total delta-v should reflect this energy change
        # (rough check - actual relationship involves kinetic and potential energy)
        assert result["delta_v_total"] > 0

    def test_vis_viva_equation(self, leo_radius, geo_radius):
        """Test that velocities satisfy vis-viva equation"""
        r_intermediate = geo_radius * 3.0
        result = bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Check initial circular orbit
        v_initial_squared = result["v_initial"] ** 2
        expected_v_initial_squared = GM_EARTH / leo_radius
        assert abs(v_initial_squared - expected_v_initial_squared) < 1.0

        # Check final circular orbit
        v_final_squared = result["v_final"] ** 2
        expected_v_final_squared = GM_EARTH / geo_radius
        assert abs(v_final_squared - expected_v_final_squared) < 1.0


class TestNumericalValues:
    """Tests with known numerical values"""

    def test_leo_to_geo_approximate_values(self, leo_radius, geo_radius):
        """Test LEO to GEO transfer with reasonable intermediate radius"""
        r_intermediate = geo_radius * 3.0
        result = bielliptic_transfer(leo_radius, geo_radius, r_intermediate, GM_EARTH)

        # Should have reasonable delta-v (order of km/s)
        assert 1000.0 < result["delta_v_total"] < 10000.0

        # Should take longer than a day but less than a week
        one_day = 86400.0
        one_week = 7 * one_day
        assert one_day < result["transfer_time"] < one_week

    def test_critical_radius_ratio(self):
        """Test behavior near the critical radius ratio of 15.58"""
        r_initial = R_MEAN_EARTH + 400e3  # LEO
        r_final = r_initial * 16.0  # Just above critical ratio
        r_intermediate = r_final * 5.0

        result = compare_bielliptic_hohmann(r_initial, r_final, r_intermediate, GM_EARTH)

        # At this ratio, bi-elliptic might be slightly better
        # (depends on intermediate radius choice)
        # Just verify the comparison runs successfully
        assert "dv_savings" in result
        assert "time_penalty" in result


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
