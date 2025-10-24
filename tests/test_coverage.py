"""
Test satellite footprint and coverage analysis functionality.

Tests the visibility circle, coverage area calculations, and access time statistics.
"""

import astrora._core as core
import numpy as np
import pytest


class TestVisibilityCircle:
    """Tests for visibility circle (satellite footprint) calculations."""

    def test_basic_visibility_circle(self):
        """Test basic visibility circle generation for ISS."""
        # ISS at 400 km, equator, 0° minimum elevation
        result = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=400.0,
            min_elevation_deg=0.0,
            num_points=32,
        )

        assert "latitudes" in result
        assert "longitudes" in result
        assert "num_points" in result
        assert "angular_radius_deg" in result

        # Check we got the right number of points
        assert result["num_points"] == 32
        assert len(result["latitudes"]) == 32
        assert len(result["longitudes"]) == 32

        # For ISS at 0° elevation, angular radius should be ~19.8°
        assert 19.0 < result["angular_radius_deg"] < 21.0

    def test_visibility_circle_with_elevation(self):
        """Test visibility circle shrinks with higher minimum elevation."""
        # Test with different elevation angles
        result_0deg = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=400.0,
            min_elevation_deg=0.0,
            num_points=32,
        )

        result_10deg = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=400.0,
            min_elevation_deg=10.0,
            num_points=32,
        )

        result_45deg = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=400.0,
            min_elevation_deg=45.0,
            num_points=32,
        )

        # Higher elevation = smaller circle
        assert result_0deg["angular_radius_deg"] > result_10deg["angular_radius_deg"]
        assert result_10deg["angular_radius_deg"] > result_45deg["angular_radius_deg"]

    def test_visibility_circle_default_points(self):
        """Test visibility circle with default number of points."""
        result = core.visibility_circle(
            lat_sub_deg=0.0, lon_sub_deg=0.0, altitude_km=400.0, min_elevation_deg=10.0
        )

        # Default should be 64 points
        assert result["num_points"] == 64
        assert len(result["latitudes"]) == 64

    def test_visibility_circle_different_locations(self):
        """Test visibility circle at different sub-satellite points."""
        # Equator
        result_eq = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=400.0,
            min_elevation_deg=10.0,
            num_points=32,
        )

        # Mid-latitude
        result_mid = core.visibility_circle(
            lat_sub_deg=45.0,
            lon_sub_deg=10.0,
            altitude_km=400.0,
            min_elevation_deg=10.0,
            num_points=32,
        )

        # Polar
        result_polar = core.visibility_circle(
            lat_sub_deg=85.0,
            lon_sub_deg=0.0,
            altitude_km=400.0,
            min_elevation_deg=10.0,
            num_points=32,
        )

        # All should have same number of points and similar angular radius
        assert result_eq["num_points"] == 32
        assert result_mid["num_points"] == 32
        assert result_polar["num_points"] == 32

        # Angular radius should be similar (independent of location)
        assert abs(result_eq["angular_radius_deg"] - result_mid["angular_radius_deg"]) < 0.1
        assert abs(result_eq["angular_radius_deg"] - result_polar["angular_radius_deg"]) < 0.1

    def test_visibility_circle_longitude_range(self):
        """Test that longitudes are in valid range (-180 to +180)."""
        result = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=175.0,  # Near date line
            altitude_km=400.0,
            min_elevation_deg=10.0,
            num_points=64,
        )

        # All longitudes should be in valid range
        assert np.all(result["longitudes"] >= -180.0)
        assert np.all(result["longitudes"] <= 180.0)

    def test_visibility_circle_latitude_range(self):
        """Test that latitudes are in valid range (-90 to +90)."""
        result = core.visibility_circle(
            lat_sub_deg=50.0,
            lon_sub_deg=0.0,
            altitude_km=400.0,
            min_elevation_deg=10.0,
            num_points=64,
        )

        # All latitudes should be in valid range
        assert np.all(result["latitudes"] >= -90.0)
        assert np.all(result["latitudes"] <= 90.0)


class TestCoverageArea:
    """Tests for satellite coverage area calculations."""

    def test_basic_coverage_area(self):
        """Test basic coverage area calculation for ISS."""
        # ISS at 400 km with 10° minimum elevation
        area = core.coverage_area(400.0, 10.0)

        # Should be ~5-7 million km²
        assert 5_000_000 < area < 7_000_000

    def test_coverage_area_increases_with_altitude(self):
        """Test that coverage area increases with altitude."""
        area_leo = core.coverage_area(400.0, 10.0)  # ISS
        area_meo = core.coverage_area(20000.0, 10.0)  # GPS
        area_geo = core.coverage_area(35786.0, 10.0)  # GEO

        # Higher altitude = larger coverage
        assert area_leo < area_meo < area_geo

    def test_coverage_area_decreases_with_elevation(self):
        """Test that coverage area decreases with higher minimum elevation."""
        area_0deg = core.coverage_area(400.0, 0.0)  # Horizon
        area_10deg = core.coverage_area(400.0, 10.0)  # Typical
        area_45deg = core.coverage_area(400.0, 45.0)  # High

        # Higher elevation = smaller coverage
        assert area_0deg > area_10deg > area_45deg

    def test_coverage_area_zero_elevation(self):
        """Test coverage area at horizon (0° elevation)."""
        # ISS at horizon should have maximum coverage
        area_horizon = core.coverage_area(400.0, 0.0)

        # Should be larger than 10° elevation
        area_10deg = core.coverage_area(400.0, 10.0)
        assert area_horizon > area_10deg

        # Should be in reasonable range for ISS
        assert 15_000_000 < area_horizon < 25_000_000

    def test_coverage_area_high_altitude(self):
        """Test coverage area for high-altitude satellites."""
        # GEO satellite can see ~40% of Earth
        area_geo = core.coverage_area(35786.0, 5.0)

        # Earth surface area is ~510 million km²
        # GEO should cover a significant fraction
        earth_area = 510_000_000
        coverage_fraction = area_geo / earth_area

        assert 0.3 < coverage_fraction < 0.45


class TestCoveragePercentage:
    """Tests for coverage percentage calculations."""

    def test_basic_coverage_percentage(self):
        """Test basic coverage percentage calculation."""
        # 90 minutes out of 1440 (24 hours)
        percentage = core.coverage_percentage(90.0, 1440.0)
        expected = 90.0 / 1440.0  # 6.25%

        assert abs(percentage - expected) < 1e-10

    def test_coverage_percentage_full(self):
        """Test 100% coverage."""
        percentage = core.coverage_percentage(100.0, 100.0)
        assert percentage == 1.0

    def test_coverage_percentage_zero(self):
        """Test 0% coverage."""
        percentage = core.coverage_percentage(0.0, 100.0)
        assert percentage == 0.0

    def test_coverage_percentage_partial(self):
        """Test partial coverage scenarios."""
        # 6 hours out of 24
        percentage_6h = core.coverage_percentage(360.0, 1440.0)
        assert abs(percentage_6h - 0.25) < 1e-10  # 25%

        # 12 hours out of 24
        percentage_12h = core.coverage_percentage(720.0, 1440.0)
        assert abs(percentage_12h - 0.5) < 1e-10  # 50%


class TestIntegration:
    """Integration tests combining multiple coverage functions."""

    def test_coverage_area_matches_circle(self):
        """Test that coverage area matches the area from visibility circle."""
        altitude = 400.0
        min_elevation = 10.0

        # Get coverage area
        area = core.coverage_area(altitude, min_elevation)

        # Get angular radius from visibility circle
        circle = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=altitude,
            min_elevation_deg=min_elevation,
            num_points=64,
        )

        angular_radius_deg = circle["angular_radius_deg"]
        angular_radius_rad = np.radians(angular_radius_deg)

        # Calculate expected area from spherical cap formula
        # A = 2πR²(1 - cos(λ))
        R = 6378.137  # WGS84 semi-major axis
        expected_area = 2 * np.pi * R**2 * (1 - np.cos(angular_radius_rad))

        # Should match within 1%
        assert abs(area - expected_area) / expected_area < 0.01

    def test_iss_realistic_scenario(self):
        """Test realistic ISS coverage scenario."""
        # ISS parameters
        altitude = 408.0  # Current ISS altitude (km)
        min_elevation = 10.0  # Typical ground station requirement

        # Get visibility circle
        circle = core.visibility_circle(
            lat_sub_deg=28.5,  # Kennedy Space Center latitude
            lon_sub_deg=-80.6,  # Kennedy Space Center longitude
            altitude_km=altitude,
            min_elevation_deg=min_elevation,
            num_points=64,
        )

        # Get coverage area
        area = core.coverage_area(altitude, min_elevation)

        # Verify reasonable values
        assert 10.0 < circle["angular_radius_deg"] < 15.0
        assert 5_000_000 < area < 8_000_000

        # Verify circle properties
        assert len(circle["latitudes"]) == 64
        assert len(circle["longitudes"]) == 64

    def test_geo_satellite_coverage(self):
        """Test coverage for geostationary satellite."""
        altitude = 35786.0  # GEO altitude
        min_elevation = 5.0  # Typical for GEO

        # Get coverage
        area = core.coverage_area(altitude, min_elevation)
        circle = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=altitude,
            min_elevation_deg=min_elevation,
            num_points=128,
        )

        # GEO satellites have very large coverage
        assert area > 150_000_000  # > 150 million km²
        assert circle["angular_radius_deg"] > 70.0  # Large angular radius

    def test_polar_orbit_coverage(self):
        """Test coverage for polar orbit satellite."""
        # Typical polar LEO
        altitude = 600.0
        min_elevation = 5.0

        # Test at pole
        circle_pole = core.visibility_circle(
            lat_sub_deg=90.0,
            lon_sub_deg=0.0,
            altitude_km=altitude,
            min_elevation_deg=min_elevation,
            num_points=64,
        )

        # Test at equator
        circle_eq = core.visibility_circle(
            lat_sub_deg=0.0,
            lon_sub_deg=0.0,
            altitude_km=altitude,
            min_elevation_deg=min_elevation,
            num_points=64,
        )

        # Angular radius should be the same regardless of location
        assert abs(circle_pole["angular_radius_deg"] - circle_eq["angular_radius_deg"]) < 0.1


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
