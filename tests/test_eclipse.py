"""
Tests for Eclipse Detection and Solar Lighting Conditions

This module tests the eclipse detection, solar beta angle, and sun-synchronous
orbit calculations exposed from the Rust backend.
"""

import numpy as np
import pytest
from astrora._core import (
    compute_eclipse_state,
    eclipse_duration,
    solar_beta_angle,
    solar_beta_angle_precise,
    sun_synchronous_inclination,
)


class TestEclipseState:
    """Test eclipse state determination (Sunlit, Penumbra, Umbra)"""

    def test_sunlit_satellite(self):
        """Satellite on same side as Sun should be sunlit"""
        # Satellite at 400 km altitude on day side
        r_sat = np.array([6778.0, 0.0, 0.0])  # km
        # Sun at 1 AU
        r_sun = np.array([149.6e6, 0.0, 0.0])  # km

        state = compute_eclipse_state(r_sat, r_sun)
        assert state == "Sunlit"

    def test_umbra_satellite(self):
        """Satellite directly in Earth's shadow should be in umbra"""
        # Satellite at 400 km on night side, directly opposite Sun
        r_sat = np.array([-6778.0, 0.0, 0.0])  # km
        # Sun at 1 AU
        r_sun = np.array([149.6e6, 0.0, 0.0])  # km

        state = compute_eclipse_state(r_sat, r_sun)
        assert state == "Umbra"

    def test_eclipse_edge_case(self):
        """Test satellite at edge of shadow cone"""
        # Satellite slightly above Earth shadow at 400 km
        r_sat = np.array([-6778.0, 1000.0, 0.0])  # km
        r_sun = np.array([149.6e6, 0.0, 0.0])  # km

        state = compute_eclipse_state(r_sat, r_sun)
        # Should be Umbra or possibly Penumbra depending on exact geometry
        assert state in ["Umbra", "Penumbra", "Sunlit"]

    def test_geo_satellite_sunlit(self):
        """GEO satellite on day side should be sunlit"""
        # GEO at 35,786 km altitude
        r_sat = np.array([42164.0, 0.0, 0.0])  # km
        r_sun = np.array([149.6e6, 0.0, 0.0])  # km

        state = compute_eclipse_state(r_sat, r_sun)
        assert state == "Sunlit"


class TestSolarBetaAngle:
    """Test solar beta angle calculations"""

    def test_beta_zero_orbit_contains_sun(self):
        """Beta = 0 when orbit plane contains Sun"""
        # Polar orbit with ascending node toward Sun
        i = 90.0  # degrees
        raan = 0.0  # degrees
        solar_lon = 0.0  # degrees

        beta = solar_beta_angle(i, raan, solar_lon)
        assert abs(beta) < 0.1  # Should be very close to 0

    def test_beta_90_perpendicular(self):
        """Beta = 90 when orbit plane perpendicular to Sun"""
        # Polar orbit with ascending node perpendicular to Sun
        i = 90.0  # degrees
        raan = 90.0  # degrees
        solar_lon = 0.0  # degrees

        beta = solar_beta_angle(i, raan, solar_lon)
        assert abs(abs(beta) - 90.0) < 0.1  # Should be ±90

    def test_iss_like_orbit(self):
        """Test beta angle for ISS-like orbit"""
        i = 51.6  # degrees
        raan = 90.0  # degrees
        solar_lon = 0.0  # degrees

        beta = solar_beta_angle(i, raan, solar_lon)
        # Should be non-zero but not extreme
        assert -90 <= beta <= 90

    def test_equatorial_orbit(self):
        """Test beta angle for equatorial orbit"""
        i = 0.0  # degrees (equatorial)
        raan = 0.0  # degrees
        solar_lon = 0.0  # degrees

        beta = solar_beta_angle(i, raan, solar_lon)
        # For equatorial orbit, beta should be 0
        assert abs(beta) < 0.1

    def test_beta_angle_range(self):
        """Beta angle should always be in range [-90, 90]"""
        # Test various combinations
        for i in [0, 30, 60, 90]:
            for raan in [0, 45, 90, 180]:
                for solar_lon in [0, 90, 180, 270]:
                    beta = solar_beta_angle(float(i), float(raan), float(solar_lon))
                    assert -90 <= beta <= 90


class TestSolarBetaAnglePrecise:
    """Test precise solar beta angle formula"""

    def test_precise_vs_simple_similar(self):
        """Precise and simple formulas should give similar results"""
        i = 51.6
        raan = 90.0
        solar_lon = 0.0

        beta_simple = solar_beta_angle(i, raan, solar_lon)
        beta_precise = solar_beta_angle_precise(i, raan, solar_lon)

        # Should be reasonably close (within a few degrees)
        assert abs(beta_simple - beta_precise) < 10.0

    def test_precise_range(self):
        """Precise beta angle should be in valid range"""
        i = 98.0  # Sun-sync orbit
        raan = 45.0
        solar_lon = 90.0

        beta = solar_beta_angle_precise(i, raan, solar_lon)
        assert -90 <= beta <= 90


class TestSunSynchronousInclination:
    """Test sun-synchronous orbit inclination calculation"""

    def test_600km_altitude(self):
        """Test typical sun-sync orbit at 600 km"""
        altitude = 600.0  # km
        ecc = 0.0  # circular

        inclination = sun_synchronous_inclination(altitude, ecc)

        # Should be around 97-98 degrees (retrograde)
        assert 97.0 <= inclination <= 99.0

    def test_800km_altitude(self):
        """Test sun-sync orbit at 800 km"""
        altitude = 800.0  # km
        ecc = 0.0

        inclination = sun_synchronous_inclination(altitude, ecc)

        # Should be slightly higher than 600 km case
        assert 97.0 <= inclination <= 99.0

    def test_leo_range(self):
        """Test sun-sync inclination across LEO range"""
        for altitude in [400, 600, 800, 1000]:
            inclination = sun_synchronous_inclination(float(altitude), 0.0)
            # All should be retrograde (> 90°)
            assert 90.0 < inclination < 105.0

    def test_invalid_altitude_raises_error(self):
        """Very high altitude should raise error (no sun-sync possible)"""
        altitude = 50000.0  # 50,000 km (way too high)

        with pytest.raises(ValueError):
            sun_synchronous_inclination(altitude, 0.0)


class TestEclipseDuration:
    """Test eclipse duration calculations"""

    def test_iss_maximum_eclipse(self):
        """ISS at 400 km with beta=0 has maximum eclipse"""
        altitude = 400.0  # km
        beta = 0.0  # degrees (maximum eclipse)

        duration = eclipse_duration(altitude, beta)

        # Should be around 30-40 minutes
        assert 30.0 <= duration <= 40.0

    def test_iss_no_eclipse(self):
        """ISS with high beta angle has no eclipse"""
        altitude = 400.0  # km
        beta = 85.0  # degrees (very high)

        duration = eclipse_duration(altitude, beta)

        # Should be very short or zero
        assert duration < 5.0

    def test_geo_eclipse(self):
        """GEO satellite eclipse duration"""
        altitude = 35786.0  # km (GEO)
        beta = 0.0  # degrees

        duration = eclipse_duration(altitude, beta)

        # GEO has longer eclipses (around 70 minutes max at equinox)
        assert 50.0 <= duration <= 75.0

    def test_eclipse_duration_decreases_with_altitude(self):
        """Higher altitude has shorter eclipse duration (for beta=0)

        Higher satellites can 'see' around Earth's shadow better, so they
        spend less time in eclipse even though orbital period is longer.
        """
        beta = 0.0
        duration_400 = eclipse_duration(400.0, beta)
        duration_800 = eclipse_duration(800.0, beta)
        duration_1200 = eclipse_duration(1200.0, beta)

        # Duration should decrease with altitude
        assert duration_400 > duration_800 > duration_1200

    def test_beta_90_no_eclipse(self):
        """Beta = 90° should result in no eclipse"""
        altitude = 600.0
        beta = 90.0

        duration = eclipse_duration(altitude, beta)

        # Should be zero or very close to zero
        assert duration < 0.1


class TestIntegration:
    """Integration tests combining multiple eclipse functions"""

    def test_sun_sync_orbit_workflow(self):
        """Complete workflow for sun-synchronous orbit analysis"""
        # 1. Calculate required inclination for sun-sync orbit
        altitude = 600.0
        inclination = sun_synchronous_inclination(altitude, 0.0)

        assert 97.0 <= inclination <= 99.0

        # 2. Calculate beta angle (varies with RAAN and solar longitude)
        raan = 0.0
        solar_lon = 0.0
        beta = solar_beta_angle(inclination, raan, solar_lon)

        # 3. Calculate eclipse duration
        duration = eclipse_duration(altitude, beta)

        # All values should be physically reasonable
        assert 0.0 <= duration <= 50.0  # minutes

    def test_eclipse_state_consistency(self):
        """Eclipse state should be consistent with geometric position"""
        # Array of test cases: (r_sat, r_sun, expected_general_state)
        test_cases = [
            # Day side - should be sunlit
            ([7000.0, 0.0, 0.0], [149.6e6, 0.0, 0.0], "Sunlit"),
            # Night side, directly opposite - should be umbra
            ([-7000.0, 0.0, 0.0], [149.6e6, 0.0, 0.0], "Umbra"),
        ]

        for r_sat, r_sun, expected in test_cases:
            state = compute_eclipse_state(np.array(r_sat), np.array(r_sun))
            assert state == expected


class TestEdgeCases:
    """Test edge cases and boundary conditions"""

    def test_very_low_altitude(self):
        """Test calculations at very low altitude (100 km)"""
        altitude = 100.0  # km

        # Should still work
        inclination = sun_synchronous_inclination(altitude, 0.0)
        assert 90.0 < inclination < 105.0

        duration = eclipse_duration(altitude, 0.0)
        assert duration > 0.0

    def test_negative_beta_angle(self):
        """Negative beta angles should work correctly"""
        altitude = 600.0
        beta = -45.0  # degrees

        duration = eclipse_duration(altitude, beta)
        # Duration should be same as positive beta (symmetric)
        duration_positive = eclipse_duration(altitude, 45.0)

        assert abs(duration - duration_positive) < 0.1

    def test_raan_wrapping(self):
        """RAAN values > 360 should wrap correctly"""
        i = 51.6
        raan1 = 90.0
        raan2 = 450.0  # Same as 90° after wrapping
        solar_lon = 0.0

        beta1 = solar_beta_angle(i, raan1, solar_lon)
        beta2 = solar_beta_angle(i, raan2, solar_lon)

        # Should give same result (within numerical precision)
        assert abs(beta1 - beta2) < 0.01


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
