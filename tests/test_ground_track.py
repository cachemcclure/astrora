"""Tests for ground track and sub-satellite point calculations"""

import numpy as np
import pytest
from astrora import _core


class TestECEFToGeodetic:
    """Test ECEF to geodetic coordinate conversion"""

    def test_ecef_to_geodetic_equator(self):
        """Test conversion for point on equator"""
        # Point on equator at sea level
        wgs84_a = 6378.137  # km
        ecef = np.array([wgs84_a, 0.0, 0.0])

        result = _core.ecef_to_geodetic(ecef)

        assert abs(result["latitude_deg"]) < 0.001
        assert abs(result["longitude_deg"]) < 0.001
        assert abs(result["altitude_km"]) < 0.01  # 10m tolerance

    def test_ecef_to_geodetic_north_pole(self):
        """Test conversion for point at north pole"""
        # North pole at sea level
        wgs84_b = 6356.752314245  # km
        ecef = np.array([0.0, 0.0, wgs84_b])

        result = _core.ecef_to_geodetic(ecef)

        assert abs(result["latitude_deg"] - 90.0) < 0.001
        assert abs(result["altitude_km"]) < 0.01

    def test_ecef_to_geodetic_south_pole(self):
        """Test conversion for point at south pole"""
        # South pole at sea level
        wgs84_b = 6356.752314245  # km
        ecef = np.array([0.0, 0.0, -wgs84_b])

        result = _core.ecef_to_geodetic(ecef)

        assert abs(result["latitude_deg"] + 90.0) < 0.001
        assert abs(result["altitude_km"]) < 0.01

    def test_ecef_to_geodetic_with_altitude(self):
        """Test conversion for point with altitude"""
        # Point on equator at 500 km altitude
        wgs84_a = 6378.137  # km
        altitude = 500.0
        ecef = np.array([wgs84_a + altitude, 0.0, 0.0])

        result = _core.ecef_to_geodetic(ecef)

        assert abs(result["latitude_deg"]) < 0.001
        assert abs(result["longitude_deg"]) < 0.001
        assert abs(result["altitude_km"] - altitude) < 0.01

    def test_ecef_to_geodetic_western_hemisphere(self):
        """Test conversion for point in western hemisphere"""
        # Point at 90°W longitude
        wgs84_a = 6378.137  # km
        ecef = np.array([0.0, -wgs84_a, 0.0])

        result = _core.ecef_to_geodetic(ecef)

        assert abs(result["latitude_deg"]) < 0.001
        assert abs(result["longitude_deg"] + 90.0) < 0.001
        assert abs(result["altitude_km"]) < 0.01

    def test_ecef_to_geodetic_leo_satellite(self):
        """Test conversion for typical LEO satellite position"""
        # Satellite in approximate LEO orbit
        ecef = np.array([6700.0, 0.0, 500.0])

        result = _core.ecef_to_geodetic(ecef)

        # Verify reasonable results
        assert -90 <= result["latitude_deg"] <= 90
        assert -180 <= result["longitude_deg"] <= 180
        assert 300 < result["altitude_km"] < 600  # LEO range

    def test_ecef_to_geodetic_invalid_input(self):
        """Test error handling for invalid input"""
        # Wrong size array
        with pytest.raises(ValueError):
            _core.ecef_to_geodetic(np.array([1.0, 2.0]))

        # Empty array
        with pytest.raises(ValueError):
            _core.ecef_to_geodetic(np.array([]))


class TestComputeGroundTrack:
    """Test ground track computation from ECEF positions"""

    def test_compute_ground_track_basic(self):
        """Test basic ground track computation"""
        # Simple ground track with 3 points
        positions = np.array(
            [
                [6778.0, 0.0, 0.0],
                [6778.0, 100.0, 50.0],
                [6750.0, 200.0, 100.0],
            ]
        )
        times = np.array([0.0, 1.0, 2.0])

        result = _core.compute_ground_track(positions, times)

        # Check output structure
        assert "latitudes_deg" in result
        assert "longitudes_deg" in result
        assert "altitudes_km" in result
        assert "times_minutes" in result

        # Check lengths
        assert len(result["latitudes_deg"]) == 3
        assert len(result["longitudes_deg"]) == 3
        assert len(result["altitudes_km"]) == 3
        assert len(result["times_minutes"]) == 3

        # Check times are preserved
        np.testing.assert_array_almost_equal(result["times_minutes"], times)

        # Check all values are finite and reasonable
        assert np.all(np.isfinite(result["latitudes_deg"]))
        assert np.all(np.isfinite(result["longitudes_deg"]))
        assert np.all(np.isfinite(result["altitudes_km"]))

        # Check latitude range
        assert np.all(np.abs(result["latitudes_deg"]) <= 90)

        # Check longitude range
        assert np.all(np.abs(result["longitudes_deg"]) <= 180)

    def test_compute_ground_track_equatorial_orbit(self):
        """Test ground track for equatorial orbit"""
        # Satellite in equatorial orbit moving eastward
        wgs84_a = 6378.137
        altitude = 400.0
        r = wgs84_a + altitude

        # 5 points along equator
        angles = np.linspace(0, np.pi / 4, 5)
        positions = np.array([[r * np.cos(ang), r * np.sin(ang), 0.0] for ang in angles])
        times = np.linspace(0, 10, 5)

        result = _core.compute_ground_track(positions, times)

        # All latitudes should be near equator
        assert np.all(np.abs(result["latitudes_deg"]) < 5.0)

        # Longitudes should increase
        lons = result["longitudes_deg"]
        assert np.all(np.diff(lons) > 0)

        # Altitudes should be consistent
        alts = result["altitudes_km"]
        assert np.all(np.abs(alts - altitude) < 50.0)

    def test_compute_ground_track_polar_orbit(self):
        """Test ground track for polar orbit"""
        # Satellite in polar orbit
        wgs84_a = 6378.137
        altitude = 500.0
        r = wgs84_a + altitude

        # 5 points from south to north
        lats_rad = np.linspace(-np.pi / 3, np.pi / 3, 5)
        positions = np.array([[r * np.cos(lat), 0.0, r * np.sin(lat)] for lat in lats_rad])
        times = np.linspace(0, 15, 5)

        result = _core.compute_ground_track(positions, times)

        # Latitudes should vary significantly
        lat_range = np.ptp(result["latitudes_deg"])
        assert lat_range > 50  # Should cover large latitude range

        # Longitudes should be relatively consistent (near prime meridian)
        assert np.all(np.abs(result["longitudes_deg"]) < 30.0)

    def test_compute_ground_track_invalid_input(self):
        """Test error handling for invalid input"""
        # Mismatched array lengths
        positions = np.array([[6700.0, 0.0, 0.0]])
        times = np.array([0.0, 1.0])

        with pytest.raises(ValueError, match="must match"):
            _core.compute_ground_track(positions, times)

        # Wrong position shape
        positions = np.array([[6700.0, 0.0]])  # Only 2 columns
        times = np.array([0.0])

        with pytest.raises(ValueError, match="shape"):
            _core.compute_ground_track(positions, times)


class TestMaximumGroundRange:
    """Test maximum ground range calculation"""

    def test_maximum_ground_range_iss(self):
        """Test ground range for ISS altitude"""
        altitude = 400.0  # km

        max_range = _core.maximum_ground_range(altitude)

        # ISS can see about 2200-2300 km to horizon
        assert 2100 < max_range < 2400

    def test_maximum_ground_range_higher_altitude(self):
        """Test that higher altitude gives longer range"""
        range_400 = _core.maximum_ground_range(400.0)
        range_800 = _core.maximum_ground_range(800.0)

        assert range_800 > range_400

    def test_maximum_ground_range_scaling(self):
        """Test ground range scales reasonably with altitude"""
        # Test several altitudes
        altitudes = [200, 400, 600, 800, 1000]
        ranges = [_core.maximum_ground_range(alt) for alt in altitudes]

        # Should be monotonically increasing
        assert all(r2 > r1 for r1, r2 in zip(ranges[:-1], ranges[1:]))

        # Should be reasonable values (not crazy large or small)
        for r in ranges:
            assert 1000 < r < 5000  # km

    def test_maximum_ground_range_zero_altitude(self):
        """Test ground range at Earth surface"""
        max_range = _core.maximum_ground_range(0.0)

        # At surface, should have zero or very small range
        assert max_range < 10.0


class TestCalculateSwathWidth:
    """Test swath width calculation"""

    def test_calculate_swath_width_leo(self):
        """Test swath width for LEO satellite"""
        altitude = 500.0  # km
        min_elevation = 10.0  # degrees

        swath = _core.calculate_swath_width(altitude, min_elevation)

        # Should be positive and reasonable for LEO
        assert swath > 0
        assert swath < 10000  # Should not be larger than Earth diameter

    def test_calculate_swath_width_elevation_effect(self):
        """Test that lower elevation gives wider swath"""
        altitude = 500.0

        swath_5deg = _core.calculate_swath_width(altitude, 5.0)
        swath_10deg = _core.calculate_swath_width(altitude, 10.0)
        swath_20deg = _core.calculate_swath_width(altitude, 20.0)

        # Lower elevation should give wider swath
        assert swath_5deg > swath_10deg > swath_20deg

    def test_calculate_swath_width_zero_elevation(self):
        """Test swath width at horizon (0° elevation)"""
        altitude = 500.0
        swath = _core.calculate_swath_width(altitude, 0.0)

        # Should be maximum swath (close to 2x ground range)
        max_range = _core.maximum_ground_range(altitude)
        assert swath > 0
        assert swath <= 2 * max_range * 1.1  # Allow 10% margin

    def test_calculate_swath_width_high_elevation(self):
        """Test swath width at high elevation"""
        altitude = 500.0
        swath = _core.calculate_swath_width(altitude, 80.0)

        # Should be narrow at high elevation
        max_swath = _core.calculate_swath_width(altitude, 0.0)
        assert swath < max_swath * 0.3  # Should be much smaller


class TestIntegration:
    """Integration tests combining ground track functions"""

    def test_ecef_to_geodetic_roundtrip_consistency(self):
        """Test that ECEF conversion is consistent"""
        # Start with known geodetic coordinates
        # Convert to ECEF, then back to geodetic

        # We don't have geodetic_to_ecef in Python yet, so we'll just verify
        # that multiple conversions of the same ECEF give same result

        ecef = np.array([6700.0, 1000.0, 500.0])

        result1 = _core.ecef_to_geodetic(ecef)
        result2 = _core.ecef_to_geodetic(ecef)

        assert result1["latitude_deg"] == result2["latitude_deg"]
        assert result1["longitude_deg"] == result2["longitude_deg"]
        assert result1["altitude_km"] == result2["altitude_km"]

    def test_ground_track_conservation_of_altitude(self):
        """Test that ground track preserves altitude approximately"""
        # Circular orbit should have constant altitude
        wgs84_a = 6378.137
        altitude = 450.0
        r = wgs84_a + altitude

        # Full circle in equatorial plane
        angles = np.linspace(0, 2 * np.pi, 20)
        positions = np.array([[r * np.cos(ang), r * np.sin(ang), 0.0] for ang in angles])
        times = np.linspace(0, 90, 20)

        result = _core.compute_ground_track(positions, times)

        # All altitudes should be similar
        alts = result["altitudes_km"]
        alt_std = np.std(alts)
        assert alt_std < 10.0  # Should be very consistent for circular orbit

        # Mean altitude should match input
        assert abs(np.mean(alts) - altitude) < 10.0
