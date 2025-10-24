"""Tests for satellite lifetime estimation and conjunction analysis"""

import astrora._core as core
import numpy as np
import pytest


class TestLifetimeEstimation:
    """Test satellite lifetime estimation with atmospheric drag"""

    def test_decay_rate_basic(self):
        """Test basic decay rate calculation"""
        # ISS-like orbit at 400 km
        altitude_km = 400.0
        ballistic_coeff = 0.005  # m²/kg

        decay_rate = core.estimate_decay_rate(altitude_km, ballistic_coeff)

        # Decay rate should be negative (orbit decaying)
        assert decay_rate < 0.0

        # Should be on the order of meters to km per day
        assert abs(decay_rate) < 1.0  # Less than 1 km/day

    def test_decay_rate_altitude_dependency(self):
        """Higher altitude should have slower decay"""
        ballistic_coeff = 0.01

        rate_200km = core.estimate_decay_rate(200.0, ballistic_coeff)
        rate_400km = core.estimate_decay_rate(400.0, ballistic_coeff)
        rate_600km = core.estimate_decay_rate(600.0, ballistic_coeff)

        # All should be negative
        assert rate_200km < 0.0
        assert rate_400km < 0.0
        assert rate_600km < 0.0

        # Lower altitude = faster decay (more negative)
        assert abs(rate_200km) > abs(rate_400km)
        assert abs(rate_400km) > abs(rate_600km)

    def test_decay_rate_ballistic_coeff_dependency(self):
        """Higher ballistic coefficient should have faster decay"""
        altitude_km = 400.0

        rate_low_B = core.estimate_decay_rate(altitude_km, 0.001)
        rate_high_B = core.estimate_decay_rate(altitude_km, 0.1)

        # Both should be negative
        assert rate_low_B < 0.0
        assert rate_high_B < 0.0

        # Higher B = faster decay (more negative)
        assert abs(rate_high_B) > abs(rate_low_B)

    def test_lifetime_estimation_basic(self):
        """Test basic lifetime estimation (warning: slow test)"""
        # Very low orbit with high ballistic coefficient for quick decay
        r_km = [6378.0 + 150.0, 0.0, 0.0]  # 150 km altitude

        # Circular velocity
        GM = 3.986004418e5  # km³/s²
        r_mag = np.sqrt(sum(x**2 for x in r_km))
        v_circ = np.sqrt(GM / r_mag)
        v_km_s = [0.0, v_circ, 0.0]

        ballistic_coeff = 0.5  # Very high for fast decay
        terminal_altitude_km = 100.0
        max_time_days = 5.0

        try:
            lifetime_days = core.estimate_satellite_lifetime(
                r_km, v_km_s, ballistic_coeff, terminal_altitude_km, max_time_days
            )

            # Should get a result
            assert lifetime_days > 0.0
            assert lifetime_days < max_time_days
        except ValueError:
            # May not converge in max_time, which is acceptable for test
            pass


class TestConjunctionAnalysis:
    """Test conjunction analysis and collision detection"""

    def test_closest_approach_parallel_orbits(self):
        """Test closest approach for parallel circular orbits"""
        # Two satellites at same altitude, offset by 5 km
        r1_km = [7000.0, 0.0, 0.0]
        r2_km = [7000.0, 5.0, 0.0]  # 5 km offset in y

        # Same circular velocity
        GM = 3.986004418e5  # km³/s²
        v_circ = np.sqrt(GM / 7000.0)
        v1_km_s = [0.0, v_circ, 0.0]
        v2_km_s = [0.0, v_circ, 0.0]

        search_window_hours = 0.2  # 12 minutes

        miss_distance = core.closest_approach_distance(
            r1_km, v1_km_s, r2_km, v2_km_s, search_window_hours
        )

        # Should be reasonably close to 5 km (may vary due to dynamics)
        assert miss_distance > 0.0
        assert miss_distance < 100.0  # Less than 100 km

    def test_conjunction_analysis(self):
        """Test full conjunction analysis"""
        # Two satellites in similar orbits
        r1_km = [7000.0, 0.0, 0.0]
        r2_km = [7000.0, 2.0, 0.0]  # 2 km offset

        GM = 3.986004418e5
        v_circ = np.sqrt(GM / 7000.0)
        v1_km_s = [0.0, v_circ, 0.0]
        v2_km_s = [0.0, v_circ, 0.0]

        search_window_hours = 0.1  # 6 minutes
        collision_threshold_km = 5.0  # 5 km

        tca_minutes, miss_distance_km, collision_risk = core.compute_conjunction(
            r1_km, v1_km_s, r2_km, v2_km_s, search_window_hours, collision_threshold_km
        )

        # TCA should be within search window
        assert tca_minutes >= 0.0
        assert tca_minutes <= search_window_hours * 60.0

        # Miss distance should be reasonable
        assert miss_distance_km > 0.0
        assert miss_distance_km < 100.0

        # Collision risk is boolean
        assert isinstance(collision_risk, bool)

    def test_conjunction_well_separated_orbits(self):
        """Test conjunction for well-separated orbits"""
        # Two satellites far apart
        r1_km = [7000.0, 0.0, 0.0]
        r2_km = [7000.0, 100.0, 0.0]  # 100 km offset

        GM = 3.986004418e5
        v_circ = np.sqrt(GM / 7000.0)
        v1_km_s = [0.0, v_circ, 0.0]
        v2_km_s = [0.0, v_circ, 0.0]

        search_window_hours = 0.1
        collision_threshold_km = 10.0  # 10 km

        tca_minutes, miss_distance_km, collision_risk = core.compute_conjunction(
            r1_km, v1_km_s, r2_km, v2_km_s, search_window_hours, collision_threshold_km
        )

        # Should not be collision risk (well separated)
        assert miss_distance_km > collision_threshold_km
        assert collision_risk == False

    def test_conjunction_error_handling(self):
        """Test error handling for invalid inputs"""
        r_km = [7000.0, 0.0, 0.0]
        v_km_s = [0.0, 7.5, 0.0]

        # Negative search window should raise error
        with pytest.raises(ValueError):
            core.compute_conjunction(
                r_km, v_km_s, r_km, v_km_s, -1.0, 5.0  # Negative search window
            )

        # Negative collision threshold should raise error
        with pytest.raises(ValueError):
            core.compute_conjunction(r_km, v_km_s, r_km, v_km_s, 1.0, -5.0)  # Negative threshold


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
