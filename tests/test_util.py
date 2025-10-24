"""
Tests for utility functions in astrora.util module.

This module tests time_range(), norm(), wrap_angle(), alinspace(),
and find_closest_value() functions.
"""

import numpy as np
import pytest
from astropy import units as u
from astropy.time import Time, TimeDelta
from astrora.util import (
    alinspace,
    find_closest_value,
    norm,
    time_range,
    wrap_angle,
)


class TestTimeRange:
    """Tests for time_range() function."""

    def test_time_range_with_end_and_periods(self):
        """Test time_range with start, end, and periods."""
        start = Time("2024-01-01 00:00:00", scale="utc")
        end = Time("2024-01-02 00:00:00", scale="utc")
        times = time_range(start, end=end, periods=25)

        assert len(times) == 25
        # Use approximate comparison for Time objects
        np.testing.assert_allclose(times[0].jd, start.jd, rtol=1e-10)
        np.testing.assert_allclose(times[-1].jd, end.jd, rtol=1e-10)

    def test_time_range_with_spacing_and_periods(self):
        """Test time_range with spacing and periods."""
        start = Time("2024-01-01 00:00:00", scale="utc")
        times = time_range(start, spacing=1 * u.hour, periods=24)

        assert len(times) == 24
        # Check spacing
        dt = times[1] - times[0]
        np.testing.assert_allclose(dt.to(u.hour).value, 1.0, rtol=1e-6)

    def test_time_range_with_spacing_and_end(self):
        """Test time_range with both spacing and end."""
        start = Time("2024-01-01 00:00:00", scale="utc")
        end = Time("2024-01-01 12:00:00", scale="utc")
        times = time_range(start, spacing=2 * u.hour, end=end)

        # Should have 7 points: 0, 2, 4, 6, 8, 10, 12 hours
        assert len(times) == 7
        assert times[0] == start
        # Last time should be at or before end
        assert times[-1] <= end

    def test_time_range_default_periods(self):
        """Test that default periods is 50."""
        start = Time("2024-01-01", scale="utc")
        end = Time("2024-01-10", scale="utc")
        times = time_range(start, end=end)

        assert len(times) == 50  # Default periods

    def test_time_range_with_string_start(self):
        """Test time_range with string start time."""
        times = time_range("2024-01-01", end="2024-01-02", periods=10)

        assert len(times) == 10
        assert isinstance(times, Time)

    def test_time_range_with_timedelta_spacing(self):
        """Test time_range with TimeDelta spacing."""
        start = Time("2024-01-01", scale="utc")
        spacing = TimeDelta(3600, format="sec")  # 1 hour
        times = time_range(start, spacing=spacing, periods=12)

        assert len(times) == 12
        dt = times[1] - times[0]
        np.testing.assert_allclose(dt.sec, 3600, rtol=1e-6)

    def test_time_range_error_without_end_or_spacing(self):
        """Test that error is raised without end or spacing."""
        start = Time("2024-01-01", scale="utc")

        with pytest.raises(ValueError, match="Must provide either"):
            time_range(start, periods=10)

    def test_time_range_preserves_scale(self):
        """Test that time scale is preserved."""
        start = Time("2024-01-01", scale="tt")
        end = Time("2024-01-02", scale="tt")
        times = time_range(start, end=end, periods=5)

        assert times.scale == "tt"


class TestNorm:
    """Tests for norm() function."""

    def test_norm_simple_vector(self):
        """Test norm of simple 3D vector."""
        v = np.array([3, 4, 0])
        result = norm(v)

        np.testing.assert_allclose(result, 5.0, rtol=1e-6)

    def test_norm_with_units(self):
        """Test norm preserves astropy units."""
        r = [6378, 0, 0] << u.km
        result = norm(r)

        assert isinstance(result, u.Quantity)
        assert result.unit == u.km
        np.testing.assert_allclose(result.value, 6378, rtol=1e-6)

    def test_norm_2d_array(self):
        """Test norm of multiple vectors (2D array)."""
        vectors = np.array([[3, 4, 0], [0, 5, 12]])
        result = norm(vectors)

        expected = np.array([5.0, 13.0])
        np.testing.assert_allclose(result, expected, rtol=1e-6)

    def test_norm_2d_with_units(self):
        """Test norm of multiple vectors with units."""
        vectors = np.array([[3, 4, 0], [0, 5, 12]]) << u.km
        result = norm(vectors)

        assert isinstance(result, u.Quantity)
        assert result.unit == u.km
        expected = np.array([5.0, 13.0])
        np.testing.assert_allclose(result.value, expected, rtol=1e-6)

    def test_norm_zero_vector(self):
        """Test norm of zero vector."""
        v = np.array([0, 0, 0])
        result = norm(v)

        np.testing.assert_allclose(result, 0.0, atol=1e-10)

    def test_norm_unit_vector(self):
        """Test norm of unit vector."""
        v = np.array([1, 0, 0])
        result = norm(v)

        np.testing.assert_allclose(result, 1.0, rtol=1e-6)


class TestWrapAngle:
    """Tests for wrap_angle() function."""

    def test_wrap_angle_positive_overflow(self):
        """Test wrapping positive angle > 180°."""
        result = wrap_angle(370)

        np.testing.assert_allclose(result, 10.0, rtol=1e-6)

    def test_wrap_angle_negative_overflow(self):
        """Test wrapping negative angle < -180°."""
        result = wrap_angle(-190)

        np.testing.assert_allclose(result, 170.0, rtol=1e-6)

    def test_wrap_angle_with_units_degrees(self):
        """Test wrap_angle with degree units."""
        result = wrap_angle(370 * u.deg)

        assert isinstance(result, u.Quantity)
        assert result.unit == u.deg
        np.testing.assert_allclose(result.value, 10.0, rtol=1e-6)

    def test_wrap_angle_with_units_radians(self):
        """Test wrap_angle with radian units."""
        result = wrap_angle(7 * u.rad, limit=np.pi * u.rad)

        assert isinstance(result, u.Quantity)
        assert result.unit == u.rad
        # 7 rad wrapped to [-π, π) should be ~0.717 rad
        expected = 7 - 2 * np.pi
        np.testing.assert_allclose(result.value, expected, rtol=1e-6)

    def test_wrap_angle_custom_limit(self):
        """Test wrap_angle with custom limit."""
        result = wrap_angle(100 * u.deg, limit=90 * u.deg)

        assert isinstance(result, u.Quantity)
        # Should wrap to [-90, 90) range
        # 100° wraps to -80°
        np.testing.assert_allclose(result.to(u.deg).value, -80.0, rtol=1e-6)

    def test_wrap_angle_already_in_range(self):
        """Test that angles already in range are unchanged."""
        result = wrap_angle(45 * u.deg)

        np.testing.assert_allclose(result.to(u.deg).value, 45.0, rtol=1e-6)

    def test_wrap_angle_at_boundary(self):
        """Test wrap_angle at boundary (exactly 180° or -180°)."""
        result = wrap_angle(180)

        # 180° should wrap to -180° (or stay 180 depending on implementation)
        assert -180 <= result < 180

    def test_wrap_angle_multiple_revolutions(self):
        """Test wrapping angle with multiple revolutions."""
        result = wrap_angle(720 + 45)  # 2 full revolutions + 45°

        np.testing.assert_allclose(result, 45.0, rtol=1e-6)


class TestAlinspace:
    """Tests for alinspace() function."""

    def test_alinspace_basic(self):
        """Test basic angular linspace."""
        angles = alinspace(0, np.pi, num=5)

        assert len(angles) == 5
        np.testing.assert_allclose(angles[0], 0.0, atol=1e-10)
        np.testing.assert_allclose(angles[-1], np.pi, rtol=1e-6)

    def test_alinspace_with_units_degrees(self):
        """Test alinspace with degree units."""
        angles = alinspace(0 * u.deg, 90 * u.deg, num=4)

        assert isinstance(angles, u.Quantity)
        assert angles.unit == u.deg
        expected = np.array([0, 30, 60, 90])
        np.testing.assert_allclose(angles.value, expected, rtol=1e-6)

    def test_alinspace_with_units_radians(self):
        """Test alinspace with radian units."""
        angles = alinspace(0 * u.rad, np.pi * u.rad, num=3)

        assert isinstance(angles, u.Quantity)
        assert angles.unit == u.rad
        expected = np.array([0, np.pi / 2, np.pi])
        np.testing.assert_allclose(angles.value, expected, rtol=1e-6)

    def test_alinspace_multiple_revolutions(self):
        """Test alinspace across multiple revolutions."""
        angles = alinspace(0 * u.deg, 720 * u.deg, num=9)

        assert len(angles) == 9
        np.testing.assert_allclose(angles[0].value, 0, atol=1e-10)
        np.testing.assert_allclose(angles[-1].value, 720, rtol=1e-6)

    def test_alinspace_default_num(self):
        """Test that default num is 50."""
        angles = alinspace(0, 2 * np.pi)

        assert len(angles) == 50

    def test_alinspace_error_num_too_small(self):
        """Test that error is raised if num < 2."""
        with pytest.raises(ValueError, match="num must be at least 2"):
            alinspace(0, np.pi, num=1)

    def test_alinspace_decreasing_range(self):
        """Test alinspace with decreasing range (start > stop)."""
        angles = alinspace(180 * u.deg, 0 * u.deg, num=3)

        expected = np.array([180, 90, 0])
        np.testing.assert_allclose(angles.value, expected, rtol=1e-6)


class TestFindClosestValue:
    """Tests for find_closest_value() function."""

    def test_find_closest_value_basic(self):
        """Test finding closest value in array."""
        arr = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
        idx = find_closest_value(3.7, arr)

        assert idx == 3  # Closest to 4.0

    def test_find_closest_value_exact_match(self):
        """Test finding exact match."""
        arr = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
        idx = find_closest_value(3.0, arr)

        assert idx == 2

    def test_find_closest_value_with_units(self):
        """Test finding closest value with units."""
        times = np.array([0, 100, 200, 300]) << u.s
        idx = find_closest_value(250 * u.s, times)

        assert idx == 2  # Closest to 200 s

    def test_find_closest_value_first_element(self):
        """Test finding closest when it's the first element."""
        arr = np.array([10, 20, 30, 40])
        idx = find_closest_value(5, arr)

        assert idx == 0

    def test_find_closest_value_last_element(self):
        """Test finding closest when it's the last element."""
        arr = np.array([10, 20, 30, 40])
        idx = find_closest_value(45, arr)

        assert idx == 3

    def test_find_closest_value_negative_values(self):
        """Test with negative values."""
        arr = np.array([-5, -2, 0, 2, 5])
        idx = find_closest_value(-1.5, arr)

        assert idx == 1  # Closest to -2

    def test_find_closest_value_returns_int(self):
        """Test that return type is int."""
        arr = np.array([1, 2, 3, 4, 5])
        idx = find_closest_value(3.2, arr)

        assert isinstance(idx, int)


class TestUtilIntegration:
    """Integration tests for utility functions."""

    def test_time_range_with_norm(self):
        """Test using time_range with orbit propagation and norm."""
        start = Time("2024-01-01", scale="utc")
        end = Time("2024-01-02", scale="utc")
        times = time_range(start, end=end, periods=10)

        # Simulate position vectors (with units)
        positions = np.random.rand(10, 3) * 7000 << u.km

        # Compute norms
        distances = norm(positions)

        assert len(distances) == 10
        assert isinstance(distances, u.Quantity)
        assert distances.unit == u.km

    def test_alinspace_with_wrap_angle(self):
        """Test generating angles and wrapping them."""
        # Generate angles from 0 to 720 degrees
        angles = alinspace(0 * u.deg, 720 * u.deg, num=9)

        # Wrap each angle to [-180, 180)
        wrapped = np.array([wrap_angle(a).value for a in angles]) << u.deg

        # All wrapped angles should be in range [-180, 180)
        assert np.all(wrapped.value >= -180)
        assert np.all(wrapped.value < 180)

    def test_find_closest_with_time_range(self):
        """Test finding closest time in a time range."""
        start = Time("2024-01-01 00:00:00", scale="utc")
        times = time_range(start, spacing=1 * u.hour, periods=24)

        # Find time closest to 10.5 hours
        target_time = start + TimeDelta(10.5 * 3600, format="sec")

        # Convert to JD for comparison
        times_jd = times.jd
        target_jd = target_time.jd

        idx = find_closest_value(target_jd, times_jd)

        # Should find either 10 or 11 hours (index 10 or 11)
        assert idx in [10, 11]
