"""Tests for plane change maneuver calculations."""

import math

import pytest
from astrora import _core

# Import functions
pure_plane_change = _core.pure_plane_change
combined_plane_change = _core.combined_plane_change
optimal_plane_change_location = _core.optimal_plane_change_location

# Import constants
GM_EARTH = _core.constants.GM_EARTH
R_MEAN_EARTH = _core.constants.R_MEAN_EARTH


class TestPurePlaneChange:
    """Test pure plane change calculations."""

    def test_basic_5_degree_change(self):
        """Test a basic 5° plane change at LEO velocity."""
        v = 7800.0  # m/s (typical LEO)
        angle = math.radians(5.0)

        result = pure_plane_change(v, angle)

        # Check structure
        assert "velocity" in result
        assert "delta_angle" in result
        assert "delta_v" in result

        # Expected: Δv = 2 * 7800 * sin(2.5°) ≈ 680 m/s
        assert abs(result["delta_v"] - 680.0) < 10.0
        assert result["velocity"] == v
        assert result["delta_angle"] == angle

    def test_45_degree_change(self):
        """Test a 45° plane change (expensive!)."""
        v = 7800.0
        angle = math.radians(45.0)

        result = pure_plane_change(v, angle)

        # Expected: Δv = 2 * 7800 * sin(22.5°) ≈ 5969 m/s
        assert abs(result["delta_v"] - 5969.0) < 10.0

    def test_180_degree_orbit_reversal(self):
        """Test a 180° plane change (orbit reversal)."""
        v = 7800.0
        angle = math.pi

        result = pure_plane_change(v, angle)

        # Expected: Δv = 2 * 7800 * sin(90°) = 15600 m/s
        assert abs(result["delta_v"] - 2.0 * v) < 0.1

    def test_zero_plane_change(self):
        """Test zero plane change (no maneuver)."""
        v = 7800.0
        angle = 0.0

        result = pure_plane_change(v, angle)

        assert abs(result["delta_v"]) < 1e-6

    def test_small_angle_scaling(self):
        """Test that small angles scale approximately linearly."""
        v = 7800.0
        angle1 = math.radians(1.0)
        angle2 = math.radians(2.0)

        result1 = pure_plane_change(v, angle1)
        result2 = pure_plane_change(v, angle2)

        # For small angles, sin(x) ≈ x, so ΔV should be approximately proportional
        ratio = result2["delta_v"] / result1["delta_v"]
        assert abs(ratio - 2.0) < 0.01  # Should be very close to 2.0

    def test_velocity_scaling(self):
        """Test that ΔV scales linearly with velocity."""
        angle = math.radians(10.0)
        v1 = 5000.0
        v2 = 10000.0

        result1 = pure_plane_change(v1, angle)
        result2 = pure_plane_change(v2, angle)

        # ΔV should scale linearly with velocity
        ratio = result2["delta_v"] / result1["delta_v"]
        assert abs(ratio - 2.0) < 1e-6

    def test_error_negative_velocity(self):
        """Test that negative velocity raises an error."""
        with pytest.raises(Exception):  # PoliastroError
            pure_plane_change(-100.0, 0.1)

    def test_error_invalid_angle(self):
        """Test that angles outside [0, π] raise an error."""
        with pytest.raises(Exception):  # PoliastroError
            pure_plane_change(7800.0, 4.0)  # > π

    def test_error_negative_angle(self):
        """Test that negative angle raises an error."""
        with pytest.raises(Exception):  # PoliastroError
            pure_plane_change(7800.0, -0.1)


class TestCombinedPlaneChange:
    """Test combined plane change with orbit change."""

    def test_hohmann_plus_plane_change(self):
        """Test combined Hohmann transfer + plane change."""
        # LEO (400 km) to transfer orbit
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))

        angle = math.radians(28.5)

        result = combined_plane_change(v_leo, v_transfer_leo, angle)

        # Check structure
        assert "v_initial" in result
        assert "v_final" in result
        assert "delta_angle" in result
        assert "delta_v" in result
        assert "delta_v_orbit_only" in result
        assert "delta_v_plane_only" in result
        assert "delta_v_penalty" in result

        # With 28.5° plane change, total ΔV should be significantly higher
        # than coplanar Hohmann
        assert result["delta_v"] > result["delta_v_orbit_only"]
        assert result["delta_v_penalty"] > 0.0

        # The penalty should be substantial for 28.5°
        assert result["delta_v_penalty"] > 1000.0  # At least 1 km/s

    def test_coplanar_reduces_to_simple_difference(self):
        """Test that 0° plane change gives simple velocity difference."""
        v1 = 7800.0
        v2 = 10000.0
        angle = 0.0

        result = combined_plane_change(v1, v2, angle)

        # Coplanar should give simple velocity difference
        assert abs(result["delta_v"] - abs(v2 - v1)) < 0.1
        assert abs(result["delta_v_penalty"]) < 1.0

    def test_90_degree_perpendicular_planes(self):
        """Test 90° plane change with perpendicular orbits."""
        v1 = 7800.0
        v2 = 7800.0  # Same speed, different plane
        angle = math.pi / 2.0

        result = combined_plane_change(v1, v2, angle)

        # For perpendicular planes with same speed:
        # Δv = √(v² + v² - 2v²cos(90°)) = √(2v²) = v√2
        expected = v1 * math.sqrt(2.0)
        assert abs(result["delta_v"] - expected) < 0.1

    def test_penalty_calculation(self):
        """Test that penalty is correctly calculated."""
        v1 = 7800.0
        v2 = 8000.0
        angle = math.radians(10.0)

        result = combined_plane_change(v1, v2, angle)

        # Penalty should be: total - orbit_only
        expected_penalty = result["delta_v"] - result["delta_v_orbit_only"]
        assert abs(result["delta_v_penalty"] - expected_penalty) < 1e-6

    def test_symmetry_in_velocity(self):
        """Test that swapping velocities gives same result."""
        v1 = 7800.0
        v2 = 8000.0
        angle = math.radians(15.0)

        result1 = combined_plane_change(v1, v2, angle)
        result2 = combined_plane_change(v2, v1, angle)

        # ΔV should be the same (law of cosines is symmetric)
        assert abs(result1["delta_v"] - result2["delta_v"]) < 1e-6


class TestOptimalPlaneChangeLocation:
    """Test optimal plane change location for transfers."""

    def test_leo_to_geo_transfer(self):
        """Test optimal split for LEO to GEO transfer with plane change."""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        v_geo = math.sqrt(GM_EARTH / r_geo)

        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))
        v_transfer_geo = math.sqrt((2.0 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer))

        angle = math.radians(28.5)

        result = optimal_plane_change_location(v_leo, v_geo, v_transfer_leo, v_transfer_geo, angle)

        # Check structure
        assert "total_angle" in result
        assert "angle_at_first" in result
        assert "angle_at_second" in result
        assert "v_first" in result
        assert "v_second" in result
        assert "delta_v_total" in result
        assert "delta_v_first" in result
        assert "delta_v_second" in result
        assert "delta_v_saved" in result
        assert "delta_v_saved_vs_low" in result

        # The optimal split should favor doing most at GEO (high altitude)
        assert result["angle_at_second"] > result["angle_at_first"]

        # Should have some savings
        assert result["delta_v_saved"] > 0.0
        assert result["delta_v_saved_vs_low"] > result["delta_v_saved"]

        # Total should be sum of parts
        assert (
            abs(result["delta_v_total"] - (result["delta_v_first"] + result["delta_v_second"]))
            < 0.1
        )

        # Angles should sum to total
        assert abs(result["angle_at_first"] + result["angle_at_second"] - angle) < 1e-6

    def test_small_angle_split(self):
        """Test that small angles favor split maneuvers."""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        v_geo = math.sqrt(GM_EARTH / r_geo)

        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))
        v_transfer_geo = math.sqrt((2.0 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer))

        angle = math.radians(5.0)  # Small angle

        result = optimal_plane_change_location(v_leo, v_geo, v_transfer_leo, v_transfer_geo, angle)

        # For small angles, optimal might be a more even split
        # (documented ~2-3° at low altitude for such cases)
        assert result["angle_at_first"] > 0.0
        assert result["delta_v_saved"] >= 0.0

    def test_zero_plane_change(self):
        """Test that zero plane change gives zero split."""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        v_geo = math.sqrt(GM_EARTH / r_geo)

        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))
        v_transfer_geo = math.sqrt((2.0 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer))

        angle = 0.0

        result = optimal_plane_change_location(v_leo, v_geo, v_transfer_leo, v_transfer_geo, angle)

        # With zero plane change, both splits should be zero
        assert abs(result["angle_at_first"]) < 1e-6
        assert abs(result["angle_at_second"]) < 1e-6
        assert abs(result["delta_v_saved"]) < 1e-6

    def test_large_angle_all_at_high(self):
        """Test that large angles strongly favor doing all at high altitude."""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        v_geo = math.sqrt(GM_EARTH / r_geo)

        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))
        v_transfer_geo = math.sqrt((2.0 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer))

        angle = math.radians(60.0)  # Large angle

        result = optimal_plane_change_location(v_leo, v_geo, v_transfer_leo, v_transfer_geo, angle)

        # For large angles, should do almost all at high altitude
        # (velocity is much lower there)
        ratio_at_high = result["angle_at_second"] / angle
        assert ratio_at_high > 0.9  # > 90% at high altitude


class TestValidationAgainstLiterature:
    """Validate plane change calculations against published values."""

    def test_leo_geo_28_5_degrees_case_1(self):
        """
        Test LEO to GEO with 28.5° plane change at LEO (Case 1).

        From orbital-mechanics.space example:
        - Initial orbit: 300 km circular LEO at 28.6° inclination
        - Target: GEO at equator (42,164 km)
        - Case 1: Plane change at LEO
        - Published total ΔV: 6.469 km/s
        """
        r_leo = R_MEAN_EARTH + 300e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        v_geo = math.sqrt(GM_EARTH / r_geo)

        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))
        v_transfer_geo = math.sqrt((2.0 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer))

        angle = math.radians(28.6)

        # First burn: combined plane change + Hohmann entry at LEO
        burn1 = combined_plane_change(v_leo, v_transfer_leo, angle)

        # Second burn: coplanar circularization at GEO
        burn2 = combined_plane_change(v_transfer_geo, v_geo, 0.0)

        total_dv = burn1["delta_v"] + burn2["delta_v"]

        # Published value: 6.469 km/s
        # Allow 100 m/s tolerance due to slightly different assumptions
        assert abs(total_dv - 6469.0) < 100.0

    def test_leo_geo_28_5_degrees_case_2(self):
        """
        Test LEO to GEO with 28.5° plane change at GEO (Case 2).

        From orbital-mechanics.space example:
        - Published total ΔV: 4.258 km/s
        - Much better than Case 1 due to lower velocity at GEO
        """
        r_leo = R_MEAN_EARTH + 300e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        v_geo = math.sqrt(GM_EARTH / r_geo)

        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))
        v_transfer_geo = math.sqrt((2.0 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer))

        angle = math.radians(28.6)

        # First burn: coplanar Hohmann entry at LEO
        burn1 = combined_plane_change(v_leo, v_transfer_leo, 0.0)

        # Second burn: combined plane change + circularization at GEO
        burn2 = combined_plane_change(v_transfer_geo, v_geo, angle)

        total_dv = burn1["delta_v"] + burn2["delta_v"]

        # Published value: 4.258 km/s
        assert abs(total_dv - 4258.0) < 100.0

    def test_optimal_split_leo_geo_28_5(self):
        """
        Test optimal split for LEO to GEO with 28.5° plane change.

        From orbital-mechanics.space:
        - Optimal split: ~2.5° at LEO, remainder at GEO
        - Total ΔV: 4.233 km/s
        - "less than 1% improvement" over all-at-GEO
        """
        r_leo = R_MEAN_EARTH + 300e3
        r_geo = 42164e3

        v_leo = math.sqrt(GM_EARTH / r_leo)
        v_geo = math.sqrt(GM_EARTH / r_geo)

        a_transfer = (r_leo + r_geo) / 2.0
        v_transfer_leo = math.sqrt((2.0 * GM_EARTH / r_leo) - (GM_EARTH / a_transfer))
        v_transfer_geo = math.sqrt((2.0 * GM_EARTH / r_geo) - (GM_EARTH / a_transfer))

        angle = math.radians(28.6)

        result = optimal_plane_change_location(v_leo, v_geo, v_transfer_leo, v_transfer_geo, angle)

        # Published optimal value: 4.233 km/s
        assert abs(result["delta_v_total"] - 4233.0) < 100.0

        # Savings should be small (< 1% of ~4.26 km/s = ~42 m/s)
        assert result["delta_v_saved"] > 0.0
        assert result["delta_v_saved"] < 50.0  # Should be small improvement


# Run pytest if this file is executed directly
if __name__ == "__main__":
    pytest.main([__file__, "-v"])
