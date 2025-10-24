"""
Test suite for Hohmann transfer calculations

This module tests the Rust-backed Hohmann transfer implementation,
including delta-v calculations, transfer times, phase angles, and
optimal transfer window calculations.
"""

import math

import numpy as np
import pytest
from astrora import _core

# Import functions
hohmann_transfer = _core.hohmann_transfer
hohmann_phase_angle = _core.hohmann_phase_angle
hohmann_synodic_period = _core.hohmann_synodic_period
hohmann_time_to_window = _core.hohmann_time_to_window

# Import constants
GM_EARTH = _core.constants.GM_EARTH
R_MEAN_EARTH = _core.constants.R_MEAN_EARTH
GM_SUN = _core.constants.GM_SUN
AU = _core.constants.AU


class TestHohmannTransferBasic:
    """Basic Hohmann transfer calculations"""

    def test_leo_to_geo_transfer(self):
        """Test LEO to GEO Hohmann transfer"""
        r_leo = R_MEAN_EARTH + 400e3  # 400 km altitude
        r_geo = R_MEAN_EARTH + 35_786e3  # 35,786 km altitude

        result = hohmann_transfer(r_leo, r_geo, GM_EARTH)

        # Check all fields are present
        assert "r_initial" in result
        assert "r_final" in result
        assert "mu" in result
        assert "delta_v1" in result
        assert "delta_v2" in result
        assert "delta_v_total" in result
        assert "transfer_time" in result
        assert "transfer_sma" in result
        assert "transfer_eccentricity" in result

        # Verify delta-v values (from standard references)
        assert result["delta_v1"] == pytest.approx(2427.0, abs=50.0)
        assert result["delta_v2"] == pytest.approx(1469.0, abs=50.0)
        assert result["delta_v_total"] == pytest.approx(3896.0, abs=100.0)

        # Transfer time should be about 5.25 hours
        transfer_time_hours = result["transfer_time"] / 3600.0
        assert transfer_time_hours == pytest.approx(5.25, abs=0.1)

        # Verify total delta-v is sum of components
        assert result["delta_v_total"] == pytest.approx(
            result["delta_v1"] + result["delta_v2"], abs=1e-10
        )

    def test_geo_to_leo_transfer(self):
        """Test descending transfer (GEO to LEO)"""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = R_MEAN_EARTH + 35_786e3

        result_ascending = hohmann_transfer(r_leo, r_geo, GM_EARTH)
        result_descending = hohmann_transfer(r_geo, r_leo, GM_EARTH)

        # Total delta-v should be same for both directions
        assert result_ascending["delta_v_total"] == pytest.approx(
            result_descending["delta_v_total"], abs=1e-10
        )

        # Transfer times should be identical
        assert result_ascending["transfer_time"] == pytest.approx(
            result_descending["transfer_time"], abs=1e-10
        )

    def test_small_altitude_change(self):
        """Test small altitude change (100 km)"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 500e3

        result = hohmann_transfer(r1, r2, GM_EARTH)

        # Delta-v should be relatively small
        assert result["delta_v_total"] < 100.0  # Less than 100 m/s

        # Transfer time should be less than 1 hour
        assert result["transfer_time"] < 3600.0

    def test_earth_mars_transfer(self):
        """Test interplanetary transfer (Earth to Mars)"""
        r_earth = 1.0 * AU
        r_mars = 1.524 * AU

        result = hohmann_transfer(r_earth, r_mars, GM_SUN)

        # Transfer time should be about 259 days
        transfer_days = result["transfer_time"] / 86400.0
        assert transfer_days == pytest.approx(259.0, abs=10.0)

        # Total delta-v should be about 5.7 km/s
        delta_v_km_s = result["delta_v_total"] / 1000.0
        assert delta_v_km_s == pytest.approx(5.7, rel=0.5)


class TestHohmannTransferProperties:
    """Test physical properties of Hohmann transfers"""

    def test_transfer_orbit_semi_major_axis(self):
        """Verify transfer orbit semi-major axis is average of radii"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 10_000e3

        result = hohmann_transfer(r1, r2, GM_EARTH)

        expected_sma = (r1 + r2) / 2.0
        assert result["transfer_sma"] == pytest.approx(expected_sma, rel=1e-10)

    def test_transfer_orbit_eccentricity(self):
        """Verify transfer orbit eccentricity is in valid range"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 10_000e3

        result = hohmann_transfer(r1, r2, GM_EARTH)

        # Eccentricity should be between 0 and 1
        assert result["transfer_eccentricity"] >= 0.0
        assert result["transfer_eccentricity"] < 1.0

    def test_energy_conservation(self):
        """Verify energy conservation in transfer orbit"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 10_000e3

        result = hohmann_transfer(r1, r2, GM_EARTH)

        # Specific energy at periapsis
        e_peri = 0.5 * result["v_transfer_periapsis"] ** 2 - GM_EARTH / r1

        # Specific energy at apoapsis
        e_apo = 0.5 * result["v_transfer_apoapsis"] ** 2 - GM_EARTH / r2

        # Should be equal (energy conservation)
        assert e_peri == pytest.approx(e_apo, rel=1e-3)

        # Should also equal -μ/(2a)
        e_expected = -GM_EARTH / (2.0 * result["transfer_sma"])
        assert e_peri == pytest.approx(e_expected, rel=1e-3)

    def test_circular_orbit_velocities(self):
        """Verify circular orbit velocity calculations"""
        r_leo = R_MEAN_EARTH + 400e3

        result = hohmann_transfer(r_leo, r_leo * 2, GM_EARTH)

        # Circular orbit velocity: v = √(μ/r)
        expected_v_initial = math.sqrt(GM_EARTH / r_leo)
        assert result["v_initial"] == pytest.approx(expected_v_initial, rel=1e-10)


class TestHohmannErrorHandling:
    """Test error handling and input validation"""

    def test_negative_initial_radius(self):
        """Test error with negative initial radius"""
        with pytest.raises(Exception):  # Should raise PoliastroError
            hohmann_transfer(-1.0, 1e7, GM_EARTH)

    def test_negative_final_radius(self):
        """Test error with negative final radius"""
        with pytest.raises(Exception):
            hohmann_transfer(1e7, -1.0, GM_EARTH)

    def test_zero_radius(self):
        """Test error with zero radius"""
        with pytest.raises(Exception):
            hohmann_transfer(0.0, 1e7, GM_EARTH)

    def test_equal_radii(self):
        """Test error with equal radii (no transfer needed)"""
        with pytest.raises(Exception):
            hohmann_transfer(1e7, 1e7, GM_EARTH)

    def test_negative_mu(self):
        """Test error with negative gravitational parameter"""
        with pytest.raises(Exception):
            hohmann_transfer(7e6, 1e7, -1.0)


class TestPhaseAngle:
    """Test optimal phase angle calculations"""

    def test_phase_angle_leo_to_geo(self):
        """Test phase angle calculation for LEO to GEO"""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = R_MEAN_EARTH + 35_786e3

        phase = hohmann_phase_angle(r_leo, r_geo, GM_EARTH)

        # Phase angle should be in valid range [0, 2π]
        assert phase >= 0.0
        assert phase < 2.0 * math.pi

        # For LEO to GEO, should be around 1.75 radians (100.4°)
        assert phase == pytest.approx(1.75, abs=0.01)

    def test_phase_angle_consistency(self):
        """Test phase angle is consistent with transfer time"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 800e3

        phase = hohmann_phase_angle(r1, r2, GM_EARTH)
        transfer_result = hohmann_transfer(r1, r2, GM_EARTH)

        # Calculate target's angular travel during transfer
        n_final = math.sqrt(GM_EARTH / r2**3)  # Mean motion
        theta_target = n_final * transfer_result["transfer_time"]

        # Phase angle should equal π - θ_target (mod 2π)
        expected_phase = (math.pi - theta_target) % (2 * math.pi)
        assert phase == pytest.approx(expected_phase, abs=1e-6)


class TestSynodicPeriod:
    """Test synodic period calculations"""

    def test_synodic_period_basic(self):
        """Test basic synodic period calculation"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 800e3

        synodic = hohmann_synodic_period(r1, r2, GM_EARTH)

        # Synodic period should be positive
        assert synodic > 0.0

        # Should be larger than individual periods
        t1 = 2 * math.pi * math.sqrt(r1**3 / GM_EARTH)
        t2 = 2 * math.pi * math.sqrt(r2**3 / GM_EARTH)
        assert synodic > t1
        assert synodic > t2

    def test_synodic_period_formula(self):
        """Verify synodic period formula: 1/T_syn = |1/T₁ - 1/T₂|"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 1000e3

        synodic = hohmann_synodic_period(r1, r2, GM_EARTH)

        # Calculate orbital periods
        t1 = 2 * math.pi * math.sqrt(r1**3 / GM_EARTH)
        t2 = 2 * math.pi * math.sqrt(r2**3 / GM_EARTH)

        # Verify formula
        expected_synodic = 1.0 / abs(1.0 / t1 - 1.0 / t2)
        assert synodic == pytest.approx(expected_synodic, rel=1e-10)

    def test_synodic_period_error_handling(self):
        """Test error handling for synodic period"""
        with pytest.raises(Exception):
            hohmann_synodic_period(-1.0, 1e7, GM_EARTH)

        with pytest.raises(Exception):
            hohmann_synodic_period(1e7, -1.0, GM_EARTH)


class TestTransferWindow:
    """Test transfer window timing calculations"""

    def test_time_to_window_aligned(self):
        """Test wait time when already at optimal phase"""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = R_MEAN_EARTH + 35_786e3

        optimal_phase = hohmann_phase_angle(r_leo, r_geo, GM_EARTH)
        wait_time = hohmann_time_to_window(optimal_phase, r_leo, r_geo, GM_EARTH)

        # Should be near zero or near synodic period
        assert wait_time == pytest.approx(0.0, abs=1.0) or wait_time > 1000.0

    def test_time_to_window_opposite(self):
        """Test wait time when at opposite phase (worst case)"""
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = R_MEAN_EARTH + 35_786e3

        # Current phase = 0 (worst case scenario)
        wait_time = hohmann_time_to_window(0.0, r_leo, r_geo, GM_EARTH)

        # Wait time should be positive and less than synodic period
        assert wait_time >= 0.0
        synodic = hohmann_synodic_period(r_leo, r_geo, GM_EARTH)
        assert wait_time <= synodic

    def test_time_to_window_range(self):
        """Test wait time for various current phases"""
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 800e3

        synodic = hohmann_synodic_period(r1, r2, GM_EARTH)

        # Test multiple phase angles
        for current_phase in np.linspace(0, 2 * math.pi, 10):
            wait_time = hohmann_time_to_window(current_phase, r1, r2, GM_EARTH)

            # Wait time should always be in valid range
            assert wait_time >= 0.0
            assert wait_time <= synodic


class TestComprehensiveScenarios:
    """Test comprehensive mission scenarios"""

    def test_geostationary_transfer_orbit(self):
        """Test GTO to GEO transfer (typical satellite deployment)"""
        # GTO: 185 km x 35,786 km (approximate periapsis at LEO altitude)
        r_gto_perigee = R_MEAN_EARTH + 185e3
        r_geo = R_MEAN_EARTH + 35_786e3

        result = hohmann_transfer(r_gto_perigee, r_geo, GM_EARTH)

        # Second burn delta-v should be around 1500 m/s
        assert result["delta_v2"] == pytest.approx(1500.0, abs=100.0)

    def test_multiple_transfers_same_body(self):
        """Test multiple transfers with different altitudes"""
        altitudes = [400e3, 800e3, 1200e3, 35_786e3]

        for i in range(len(altitudes) - 1):
            r1 = R_MEAN_EARTH + altitudes[i]
            r2 = R_MEAN_EARTH + altitudes[i + 1]

            result = hohmann_transfer(r1, r2, GM_EARTH)

            # All transfers should have positive delta-v
            assert result["delta_v_total"] > 0.0

            # Transfer time should increase with altitude difference
            assert result["transfer_time"] > 0.0

    def test_numerical_stability(self):
        """Test numerical stability with various orbit sizes"""
        # Very small transfer
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 401e3
        result_small = hohmann_transfer(r1, r2, GM_EARTH)
        assert result_small["delta_v_total"] > 0.0

        # Very large transfer
        r1 = R_MEAN_EARTH + 400e3
        r2 = R_MEAN_EARTH + 100_000e3
        result_large = hohmann_transfer(r1, r2, GM_EARTH)
        assert result_large["delta_v_total"] > 0.0

        # Different central bodies
        result_sun = hohmann_transfer(1.0 * AU, 1.5 * AU, GM_SUN)
        assert result_sun["delta_v_total"] > 0.0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
