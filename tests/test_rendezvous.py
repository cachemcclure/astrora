"""Tests for rendezvous and phasing orbit calculations"""

import math

import pytest
from astrora._core import (
    constants,
    coorbital_rendezvous,
    coplanar_rendezvous,
    phasing_orbit,
)

GM_EARTH = constants.GM_EARTH
R_MEAN_EARTH = constants.R_MEAN_EARTH


class TestPhasingOrbit:
    """Test phasing orbit calculations"""

    def test_phasing_orbit_catch_up(self):
        """Test catching up to a target (positive phase change)"""
        r = R_MEAN_EARTH + 400e3  # 400 km altitude
        phase_change = math.radians(30)  # 30 degrees
        num_orbits = 5.0

        result = phasing_orbit(r, phase_change, num_orbits, GM_EARTH)

        # Check all required fields are present
        assert "delta_v_total" in result
        assert "phasing_time" in result
        assert "a_phasing" in result
        assert "period_phasing" in result

        # Verify basic properties
        assert result["delta_v_total"] > 0
        assert result["phasing_time"] > 0
        assert pytest.approx(result["total_phase_change"]) == phase_change
        assert pytest.approx(result["phase_change_per_orbit"]) == phase_change / num_orbits

        # For catching up, phasing orbit should be smaller (faster)
        assert result["a_phasing"] < r
        assert result["period_phasing"] < result["period_original"]

    def test_phasing_orbit_wait(self):
        """Test waiting for a target (negative phase change)"""
        r = R_MEAN_EARTH + 400e3
        phase_change = -math.radians(30)  # -30 degrees (wait)
        num_orbits = 5.0

        result = phasing_orbit(r, phase_change, num_orbits, GM_EARTH)

        # For waiting, phasing orbit should be larger (slower)
        assert result["a_phasing"] > r
        assert result["period_phasing"] > result["period_original"]
        assert pytest.approx(result["total_phase_change"]) == phase_change

    def test_phasing_orbit_small_change(self):
        """Test small phase change over many orbits"""
        r = R_MEAN_EARTH + 400e3
        phase_change = math.radians(10)  # 10 degrees
        num_orbits = 10.0

        result = phasing_orbit(r, phase_change, num_orbits, GM_EARTH)

        assert result["delta_v_total"] > 0
        # Small phase change per orbit should require smaller delta-v per maneuver
        phase_per_orbit = result["phase_change_per_orbit"]
        assert abs(phase_per_orbit) < math.radians(5)  # Less than 5° per orbit

    def test_phasing_orbit_symmetric_maneuver(self):
        """Test that enter and exit burns are equal for phasing"""
        r = R_MEAN_EARTH + 400e3
        phase_change = math.radians(20)
        num_orbits = 5.0

        result = phasing_orbit(r, phase_change, num_orbits, GM_EARTH)

        # Phasing maneuvers should be symmetric
        assert pytest.approx(result["delta_v_enter"]) == result["delta_v_exit"]
        assert pytest.approx(result["delta_v_total"]) == 2 * result["delta_v_enter"]

    def test_phasing_orbit_invalid_inputs(self):
        """Test error handling for invalid inputs"""
        r = R_MEAN_EARTH + 400e3

        # Zero phase change
        with pytest.raises(Exception):  # Should raise InvalidParameter
            phasing_orbit(r, 0.0, 2.0, GM_EARTH)

        # Less than 1 orbit
        with pytest.raises(Exception):
            phasing_orbit(r, math.radians(30), 0.5, GM_EARTH)

        # Negative radius
        with pytest.raises(Exception):
            phasing_orbit(-r, math.radians(30), 2.0, GM_EARTH)


class TestCoorbitalRendezvous:
    """Test coorbital rendezvous (same orbit, different positions)"""

    def test_coorbital_rendezvous_basic(self):
        """Test basic coorbital rendezvous"""
        r = R_MEAN_EARTH + 400e3
        phase_diff = math.radians(45)  # 45 degrees apart
        num_orbits = 10.0

        result = coorbital_rendezvous(r, phase_diff, num_orbits, GM_EARTH)

        # Check structure
        assert "r_orbit" in result
        assert "initial_phase_difference" in result
        assert "phasing" in result
        assert isinstance(result["phasing"], dict)

        # Verify values
        assert pytest.approx(result["r_orbit"]) == r
        assert pytest.approx(result["initial_phase_difference"]) == phase_diff

        # Check phasing orbit details
        phasing = result["phasing"]
        assert phasing["delta_v_total"] > 0
        assert pytest.approx(phasing["total_phase_change"]) == phase_diff

    def test_coorbital_rendezvous_90_degrees(self):
        """Test catching up when 90° behind"""
        r = R_MEAN_EARTH + 400e3
        phase_diff = math.radians(90)
        num_orbits = 15.0

        result = coorbital_rendezvous(r, phase_diff, num_orbits, GM_EARTH)

        assert pytest.approx(result["initial_phase_difference"]) == math.radians(90)
        assert result["phasing"]["delta_v_total"] > 0

    def test_coorbital_rendezvous_phase_normalization(self):
        """Test that phase difference is normalized to [0, 2π)"""
        r = R_MEAN_EARTH + 400e3
        phase_diff = math.radians(400)  # > 360 degrees
        num_orbits = 10.0

        result = coorbital_rendezvous(r, phase_diff, num_orbits, GM_EARTH)

        # Should normalize to [0, 2π)
        assert result["initial_phase_difference"] >= 0
        assert result["initial_phase_difference"] < 2 * math.pi


class TestCoplanarRendezvous:
    """Test coplanar rendezvous (different orbits, same plane)"""

    def test_coplanar_rendezvous_ascending(self):
        """Test rendezvous from lower to higher orbit"""
        r_leo = R_MEAN_EARTH + 300e3  # 300 km
        r_iss = R_MEAN_EARTH + 400e3  # 400 km
        current_phase = math.radians(45)

        result = coplanar_rendezvous(r_leo, r_iss, current_phase, GM_EARTH)

        # Check all required fields
        assert "r_chaser" in result
        assert "r_target" in result
        assert "transfer_time" in result
        assert "wait_time" in result
        assert "delta_v_total" in result
        assert "required_phase_angle" in result
        assert "lead_angle" in result

        # Verify values
        assert pytest.approx(result["r_chaser"]) == r_leo
        assert pytest.approx(result["r_target"]) == r_iss
        assert result["transfer_time"] > 0
        assert result["delta_v_total"] > 0
        assert result["wait_time"] >= 0

        # Transfer orbit should be between the two orbits
        assert result["a_transfer"] > r_leo
        assert result["a_transfer"] < r_iss

    def test_coplanar_rendezvous_descending(self):
        """Test rendezvous from higher to lower orbit"""
        r_high = R_MEAN_EARTH + 500e3
        r_low = R_MEAN_EARTH + 350e3
        current_phase = math.radians(60)

        result = coplanar_rendezvous(r_high, r_low, current_phase, GM_EARTH)

        # Transfer should still work
        assert result["delta_v_total"] > 0
        assert result["a_transfer"] > r_low
        assert result["a_transfer"] < r_high

    def test_coplanar_rendezvous_lead_angle(self):
        """Test that lead angle is calculated correctly"""
        r_chaser = R_MEAN_EARTH + 300e3
        r_target = R_MEAN_EARTH + 400e3
        current_phase = 0.0  # Target directly ahead

        result = coplanar_rendezvous(r_chaser, r_target, current_phase, GM_EARTH)

        # Lead angle should account for target motion during transfer
        assert result["lead_angle"] > 0
        assert result["lead_angle"] < 2 * math.pi

        # Required phase angle should account for lead
        assert "required_phase_angle" in result

    def test_coplanar_rendezvous_wait_orbits(self):
        """Test wait time and orbit calculation"""
        r_chaser = R_MEAN_EARTH + 300e3
        r_target = R_MEAN_EARTH + 400e3
        current_phase = math.radians(180)  # Target on opposite side

        result = coplanar_rendezvous(r_chaser, r_target, current_phase, GM_EARTH)

        # Should calculate how many chaser orbits to wait
        assert result["wait_orbits"] >= 0
        assert result["wait_time"] >= 0

    def test_coplanar_rendezvous_invalid_same_orbit(self):
        """Test error when orbits are the same"""
        r = R_MEAN_EARTH + 400e3

        with pytest.raises(Exception):
            coplanar_rendezvous(r, r, math.radians(45), GM_EARTH)


class TestRendezvousPhysics:
    """Test physical correctness of rendezvous calculations"""

    def test_energy_conservation_phasing(self):
        """Test that phasing orbit conserves energy correctly"""
        r = R_MEAN_EARTH + 400e3
        phase_change = math.radians(20)
        num_orbits = 5.0

        result = phasing_orbit(r, phase_change, num_orbits, GM_EARTH)

        # Calculate specific orbital energy
        # ε = -μ/(2a)
        epsilon_original = -GM_EARTH / (2 * r)
        epsilon_phasing = -GM_EARTH / (2 * result["a_phasing"])

        # Energies should be different (that's the point!)
        assert epsilon_original != epsilon_phasing

        # Velocities should satisfy vis-viva equation: v² = μ(2/r - 1/a)
        v_original_calc = math.sqrt(GM_EARTH / r)
        assert pytest.approx(result["v_original"], rel=1e-6) == v_original_calc

    def test_delta_v_positive(self):
        """All delta-v values should be positive"""
        r = R_MEAN_EARTH + 400e3
        phase_change = math.radians(30)
        num_orbits = 5.0

        result = phasing_orbit(r, phase_change, num_orbits, GM_EARTH)

        assert result["delta_v_enter"] > 0
        assert result["delta_v_exit"] > 0
        assert result["delta_v_total"] > 0
        assert result["delta_v_total"] >= result["delta_v_enter"]
        assert result["delta_v_total"] >= result["delta_v_exit"]

    def test_eccentricity_bounds(self):
        """Test that eccentricity is in valid range"""
        r = R_MEAN_EARTH + 400e3
        phase_change = math.radians(25)
        num_orbits = 8.0

        result = phasing_orbit(r, phase_change, num_orbits, GM_EARTH)

        # For an ellipse, 0 < e < 1
        assert result["e_phasing"] > 0
        assert result["e_phasing"] < 1


class TestRendezvousRealism:
    """Test realistic rendezvous scenarios"""

    def test_iss_resupply_scenario(self):
        """Simulate a typical ISS resupply rendezvous"""
        # Cargo vehicle starts at slightly lower orbit
        r_cargo = R_MEAN_EARTH + 380e3  # 380 km
        r_iss = R_MEAN_EARTH + 408e3  # 408 km (typical ISS altitude)

        # ISS is currently 30° ahead
        current_phase = math.radians(30)

        result = coplanar_rendezvous(r_cargo, r_iss, current_phase, GM_EARTH)

        # Should produce reasonable values
        # Transfer should take less than a few hours
        transfer_hours = result["transfer_time"] / 3600
        assert transfer_hours < 10  # Typical ISS transfers are < 6 hours

        # Delta-v should be reasonable (tens to hundreds of m/s)
        assert result["delta_v_total"] < 200  # m/s (typical for ISS rendezvous)
        assert result["delta_v_total"] > 10  # m/s (must be non-trivial)

    def test_geostationary_phasing(self):
        """Test phasing maneuver for GEO satellite repositioning"""
        r_geo = R_MEAN_EARTH + 35_786e3  # GEO altitude

        # Need to move 10° in longitude over 7 days (common scenario)
        phase_change = math.radians(10)

        # GEO period is ~24 hours, so 7 orbits ≈ 7 days
        num_orbits = 7.0

        result = phasing_orbit(r_geo, phase_change, num_orbits, GM_EARTH)

        # Should require very small delta-v (GEO satellites have limited fuel)
        # Typical GEO phasing: < 50 m/s
        assert result["delta_v_total"] < 100  # m/s

        # Phasing time should be ~7 days
        phasing_days = result["phasing_time"] / 86400
        assert pytest.approx(phasing_days, rel=0.1) == 7.0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
