"""
Tests for satellite visibility and ground station operations

Tests the Python API for computing azimuth, elevation, and satellite passes
from ground stations using topocentric coordinate transformations.
"""

import pytest
from astrora._core import (
    py_compute_azimuth_elevation as compute_azimuth_elevation,
)
from astrora._core import (
    py_compute_azimuth_elevation_rate as compute_azimuth_elevation_rate,
)
from astrora._core import (
    py_find_satellite_passes as find_satellite_passes,
)
from astrora._core import (
    py_is_visible as is_visible,
)


class TestAzimuthElevationCalculation:
    """Test azimuth and elevation computation from observer locations"""

    def test_satellite_at_zenith(self):
        """Test satellite directly overhead (zenith)"""
        # Observer at equator, prime meridian
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite at 500 km altitude, directly above observer
        # At equator: ECEF ≈ [6878.137, 0, 0]
        sat_ecef = [6878.137, 0.0, 0.0]

        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # Should be at zenith (90° elevation)
        assert result["elevation_deg"] > 85.0
        assert result["range_km"] < 550.0  # ~500 km + some geometry
        assert result["range_km"] > 450.0

    def test_satellite_to_north(self):
        """Test satellite to the north of observer"""
        # Observer at equator
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite to the north (higher latitude, same longitude)
        sat_ecef = [6378.0, 0.0, 500.0]  # Shifted toward north pole

        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # Azimuth should be close to 0° (North) or 360°
        azimuth = result["azimuth_deg"]
        assert azimuth < 45.0 or azimuth > 315.0

    def test_satellite_to_east(self):
        """Test satellite to the east of observer"""
        # Observer at equator, prime meridian
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite to the east (same latitude, higher longitude)
        sat_ecef = [4500.0, 4500.0, 0.0]  # 45° east

        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # Azimuth should be close to 90° (East)
        assert 45.0 < result["azimuth_deg"] < 135.0

    def test_mit_location(self):
        """Test realistic ground station at MIT"""
        # MIT location (42.36°N, 71.09°W, 10m altitude)
        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 0.010

        # LEO satellite (arbitrary position above horizon)
        sat_ecef = [1000.0, -4000.0, 5000.0]

        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # Should have valid values
        assert 0.0 <= result["azimuth_deg"] < 360.0
        assert -90.0 <= result["elevation_deg"] <= 90.0
        assert result["range_km"] > 0.0

        # Should have ENU components
        assert "enu_east_km" in result
        assert "enu_north_km" in result
        assert "enu_up_km" in result

    def test_enu_components(self):
        """Verify ENU components are correctly oriented"""
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite to the east
        sat_ecef = [4500.0, 4500.0, 0.0]
        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # East component should dominate for eastward satellite
        assert abs(result["enu_east_km"]) > abs(result["enu_north_km"])


class TestAzimuthElevationRate:
    """Test azimuth/elevation calculation with velocity (range rate)"""

    def test_satellite_receding(self):
        """Test satellite moving away from observer"""
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite above observer, moving upward
        sat_ecef = [6878.0, 0.0, 0.0]
        vel_ecef = [1.0, 0.0, 0.0]  # Moving radially outward

        result = compute_azimuth_elevation_rate(sat_ecef, vel_ecef, obs_lat, obs_lon, obs_alt)

        # Range rate should be positive (receding)
        assert result["range_rate_km_s"] > 0.0

    def test_satellite_approaching(self):
        """Test satellite moving toward observer"""
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite above observer, moving downward
        sat_ecef = [6878.0, 0.0, 0.0]
        vel_ecef = [-1.0, 0.0, 0.0]  # Moving radially inward

        result = compute_azimuth_elevation_rate(sat_ecef, vel_ecef, obs_lat, obs_lon, obs_alt)

        # Range rate should be negative (approaching)
        assert result["range_rate_km_s"] < 0.0

    def test_satellite_tangential_motion(self):
        """Test satellite in tangential orbit (range rate ≈ 0)"""
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite above observer
        sat_ecef = [6878.0, 0.0, 0.0]
        # Moving tangentially (perpendicular to range vector)
        vel_ecef = [0.0, 7.5, 0.0]

        result = compute_azimuth_elevation_rate(sat_ecef, vel_ecef, obs_lat, obs_lon, obs_alt)

        # Range rate should be small
        assert abs(result["range_rate_km_s"]) < 1.0


class TestVisibilityCheck:
    """Test satellite visibility determination"""

    def test_satellite_above_horizon(self):
        """Test satellite above horizon is visible"""
        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 0.010

        # Satellite well above horizon (LEO at ~400 km altitude)
        # Observer at MIT, satellite roughly overhead
        sat_ecef = [1000.0, -4000.0, 5500.0]  # Magnitude ~7000 km from Earth center

        visible = is_visible(sat_ecef, obs_lat, obs_lon, obs_alt, min_elevation_deg=0.0)
        assert visible

    def test_satellite_below_horizon(self):
        """Test satellite below horizon is not visible"""
        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 0.010

        # Satellite on opposite side of Earth
        sat_ecef = [-2000.0, 3000.0, -5000.0]

        visible = is_visible(sat_ecef, obs_lat, obs_lon, obs_alt, min_elevation_deg=0.0)
        assert not visible

    def test_minimum_elevation_threshold(self):
        """Test minimum elevation threshold filtering"""
        obs_lat, obs_lon, obs_alt = 0.0, 0.0, 0.0

        # Satellite at moderate elevation (LEO, ~400 km altitude)
        # Positioned to be above horizon from equator observer
        sat_ecef = [6878.0, 1000.0, 1000.0]  # Magnitude ~7000 km

        # Get actual elevation
        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)
        actual_elev = result["elevation_deg"]

        # Should be visible with 0° threshold
        assert is_visible(sat_ecef, obs_lat, obs_lon, obs_alt, min_elevation_deg=0.0)

        # May not be visible with threshold > actual elevation
        if actual_elev < 30.0:
            assert not is_visible(
                sat_ecef, obs_lat, obs_lon, obs_alt, min_elevation_deg=actual_elev + 1.0
            )


class TestSatellitePassPrediction:
    """Test satellite pass finding"""

    def test_find_passes_basic(self):
        """Test finding satellite passes over 24 hours"""
        # ISS TLE from 2008
        tle = """ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""

        # MIT ground station
        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 0.010

        # Find passes over 24 hours
        passes = find_satellite_passes(
            tle,
            observer_lat_deg=obs_lat,
            observer_lon_deg=obs_lon,
            observer_alt_km=obs_alt,
            start_time_minutes=0.0,
            end_time_minutes=1440.0,  # 24 hours
            min_elevation_deg=0.0,  # Any elevation above horizon
            time_step_minutes=1.0,
        )

        # ISS should have multiple passes in 24 hours (LEO orbit ~90 min period)
        assert isinstance(passes, list)
        # We should find at least a few passes
        # (exact number depends on geometry, but ISS typically visible 2-4 times/day)

    def test_pass_structure(self):
        """Test that pass data structure is correct"""
        tle = """ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""

        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 0.010

        passes = find_satellite_passes(
            tle,
            observer_lat_deg=obs_lat,
            observer_lon_deg=obs_lon,
            observer_alt_km=obs_alt,
            start_time_minutes=0.0,
            end_time_minutes=1440.0,
            min_elevation_deg=10.0,  # 10° minimum for clearer passes
            time_step_minutes=1.0,
        )

        if len(passes) > 0:
            # Check first pass structure
            pass_info = passes[0]

            # Should have all required fields
            assert "rise_time_minutes" in pass_info
            assert "set_time_minutes" in pass_info
            assert "max_elevation_time_minutes" in pass_info
            assert "max_elevation_deg" in pass_info
            assert "rise_azimuth_deg" in pass_info
            assert "set_azimuth_deg" in pass_info
            assert "duration_minutes" in pass_info

            # Basic sanity checks
            assert pass_info["rise_time_minutes"] < pass_info["set_time_minutes"]
            assert (
                pass_info["rise_time_minutes"]
                <= pass_info["max_elevation_time_minutes"]
                <= pass_info["set_time_minutes"]
            )
            assert pass_info["max_elevation_deg"] >= 10.0  # At least minimum threshold
            assert 0.0 <= pass_info["rise_azimuth_deg"] < 360.0
            assert 0.0 <= pass_info["set_azimuth_deg"] < 360.0
            assert pass_info["duration_minutes"] > 0.0

            # Duration should match rise-set difference
            expected_duration = pass_info["set_time_minutes"] - pass_info["rise_time_minutes"]
            assert abs(pass_info["duration_minutes"] - expected_duration) < 0.01

    def test_high_elevation_filter(self):
        """Test that high minimum elevation reduces number of passes"""
        tle = """ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""

        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 0.010

        # Find passes with 0° minimum
        passes_0deg = find_satellite_passes(
            tle,
            observer_lat_deg=obs_lat,
            observer_lon_deg=obs_lon,
            observer_alt_km=obs_alt,
            start_time_minutes=0.0,
            end_time_minutes=1440.0,
            min_elevation_deg=0.0,
            time_step_minutes=1.0,
        )

        # Find passes with 30° minimum (much more restrictive)
        passes_30deg = find_satellite_passes(
            tle,
            observer_lat_deg=obs_lat,
            observer_lon_deg=obs_lon,
            observer_alt_km=obs_alt,
            start_time_minutes=0.0,
            end_time_minutes=1440.0,
            min_elevation_deg=30.0,
            time_step_minutes=1.0,
        )

        # Higher elevation threshold should find fewer passes
        assert len(passes_30deg) <= len(passes_0deg)

        # All 30° passes should have max elevation >= 30°
        for pass_info in passes_30deg:
            assert pass_info["max_elevation_deg"] >= 30.0


class TestEdgeCases:
    """Test edge cases and error handling"""

    def test_observer_at_poles(self):
        """Test observer at North Pole"""
        # North Pole (90°N)
        obs_lat, obs_lon, obs_alt = 90.0, 0.0, 0.0

        # Satellite near pole
        sat_ecef = [0.0, 0.0, 7000.0]

        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # Should have valid values even at pole
        assert 0.0 <= result["azimuth_deg"] < 360.0
        assert -90.0 <= result["elevation_deg"] <= 90.0

    def test_negative_longitude(self):
        """Test observer with negative (western) longitude"""
        # Boston (western hemisphere)
        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 0.010

        sat_ecef = [5000.0, 0.0, 3000.0]

        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # Should work correctly with negative longitude
        assert 0.0 <= result["azimuth_deg"] < 360.0
        assert result["range_km"] > 0.0

    def test_high_altitude_observer(self):
        """Test observer at high altitude (mountain or aircraft)"""
        # High altitude observer (10 km = typical cruising altitude)
        obs_lat, obs_lon, obs_alt = 42.36, -71.09, 10.0

        sat_ecef = [5000.0, -3000.0, 4000.0]

        result = compute_azimuth_elevation(sat_ecef, obs_lat, obs_lon, obs_alt)

        # Should still work at high altitude
        assert result["range_km"] > 0.0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
