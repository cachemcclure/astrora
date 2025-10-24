"""
Integration tests for the time module using hifitime.

Tests cover:
- Epoch creation and manipulation
- Duration operations
- Time scale conversions
- Compatibility with expected astrodynamics workflows
"""

import pytest
from astrora._core import Duration, Epoch


class TestEpochCreation:
    """Test various methods of creating epochs."""

    def test_epoch_from_gregorian_utc(self):
        """Test creating epoch from Gregorian calendar in UTC."""
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)
        assert epoch is not None

    def test_epoch_with_default_params(self):
        """Test epoch creation with default time parameters."""
        epoch = Epoch(2024, 10, 22)  # Defaults to midnight
        assert epoch is not None

    def test_j2000_epoch(self):
        """Test the J2000.0 reference epoch."""
        j2000 = Epoch.j2000_epoch()

        # J2000 is defined as 2000-01-01 12:00:00 TT
        # MJD at J2000 = 51544.5
        assert abs(j2000.mjd_tt - 51544.5) < 1e-6

        # JD at J2000 = 2451545.0
        assert abs(j2000.jd_tt - 2451545.0) < 1e-6

        # Seconds since J2000 should be 0
        assert abs(j2000.tt_seconds) < 1e-6

    def test_midnight_and_noon_constructors(self):
        """Test convenience constructors for midnight and noon."""
        midnight = Epoch.from_midnight_utc(2000, 1, 1)
        noon = Epoch.from_noon_utc(2000, 1, 1)

        # Difference should be 12 hours
        diff = noon - midnight
        assert abs(diff.hours - 12.0) < 1e-6


class TestTimeScaleConversions:
    """Test conversions between different time scales."""

    def test_utc_to_tai_conversion(self):
        """Test UTC to TAI conversion."""
        utc_epoch = Epoch(2020, 3, 15, 10, 30, 45, 0)
        tai_epoch = utc_epoch.as_tai()

        # They represent the same instant
        diff = tai_epoch - utc_epoch
        assert abs(diff.seconds) < 1e-3

    def test_utc_to_tt_conversion(self):
        """Test UTC to TT (Terrestrial Time) conversion."""
        utc_epoch = Epoch(2020, 3, 15, 10, 30, 45, 0)
        tt_epoch = utc_epoch.as_tt()

        # They represent the same instant
        diff = tt_epoch - utc_epoch
        assert abs(diff.seconds) < 1e-3

    def test_utc_to_tdb_conversion(self):
        """Test UTC to TDB (Barycentric Dynamical Time) conversion."""
        utc_epoch = Epoch(2020, 3, 15, 10, 30, 45, 0)
        tdb_epoch = utc_epoch.as_tdb()

        # They represent the same instant
        diff = tdb_epoch - utc_epoch
        assert abs(diff.seconds) < 1e-3

    def test_mjd_in_different_scales(self):
        """Test Modified Julian Date in different time scales."""
        epoch = Epoch(2000, 1, 1, 0, 0, 0, 0)

        mjd_utc = epoch.mjd_utc
        mjd_tai = epoch.mjd_tai
        mjd_tt = epoch.mjd_tt

        # MJD values should differ due to time scale offsets
        # TAI is ahead of UTC by leap seconds
        assert abs(mjd_tai - mjd_utc) > 0.0003  # At least 30 seconds
        assert abs(mjd_tai - mjd_utc) < 0.0005  # Less than 45 seconds

        # TT is ahead of TAI by 32.184 seconds
        assert abs(mjd_tt - mjd_tai) > 0.00037  # ~32 seconds
        assert abs(mjd_tt - mjd_tai) < 0.00038  # ~33 seconds


class TestMJDJDConversions:
    """Test Modified Julian Date and Julian Date conversions."""

    def test_mjd_jd_relationship(self):
        """Test the relationship JD = MJD + 2400000.5."""
        epoch = Epoch(2000, 1, 1, 0, 0, 0, 0)

        mjd = epoch.mjd_utc
        jd = epoch.jd_utc

        # JD = MJD + 2400000.5
        assert abs(jd - (mjd + 2400000.5)) < 1e-9

    def test_mjd_value_at_j2000(self):
        """Test MJD value at J2000 epoch."""
        # J2000 is 2000-01-01 12:00:00 UTC (noon)
        epoch = Epoch(2000, 1, 1, 0, 0, 0, 0)  # Midnight

        # MJD at 2000-01-01 00:00:00 UTC should be 51544.0
        assert abs(epoch.mjd_utc - 51544.0) < 0.001


class TestDurationOperations:
    """Test duration creation and arithmetic."""

    def test_duration_from_seconds(self):
        """Test creating duration from seconds."""
        dur = Duration(3600.0)  # 1 hour in seconds
        assert abs(dur.seconds - 3600.0) < 1e-9
        assert abs(dur.minutes - 60.0) < 1e-9
        assert abs(dur.hours - 1.0) < 1e-9

    def test_duration_from_minutes(self):
        """Test creating duration from minutes."""
        dur = Duration.from_min(90.0)  # 1.5 hours
        assert abs(dur.minutes - 90.0) < 1e-9
        assert abs(dur.hours - 1.5) < 1e-9
        assert abs(dur.seconds - 5400.0) < 1e-9

    def test_duration_from_hours(self):
        """Test creating duration from hours."""
        dur = Duration.from_hrs(2.5)
        assert abs(dur.hours - 2.5) < 1e-9
        assert abs(dur.minutes - 150.0) < 1e-9

    def test_duration_from_days(self):
        """Test creating duration from days."""
        dur = Duration.from_day(1.5)
        assert abs(dur.days - 1.5) < 1e-9
        assert abs(dur.hours - 36.0) < 1e-9

    def test_duration_addition(self):
        """Test adding durations."""
        d1 = Duration.from_hrs(1.0)
        d2 = Duration.from_min(30.0)
        sum_dur = d1 + d2

        assert abs(sum_dur.minutes - 90.0) < 1e-9

    def test_duration_subtraction(self):
        """Test subtracting durations."""
        d1 = Duration.from_hrs(2.0)
        d2 = Duration.from_min(30.0)
        diff_dur = d1 - d2

        assert abs(diff_dur.minutes - 90.0) < 1e-9

    def test_duration_multiplication(self):
        """Test multiplying duration by scalar."""
        dur = Duration.from_hrs(1.0)
        scaled = dur * 2.5

        assert abs(scaled.hours - 2.5) < 1e-9

    def test_duration_division(self):
        """Test dividing duration by scalar."""
        dur = Duration.from_hrs(3.0)
        divided = dur / 2.0

        assert abs(divided.hours - 1.5) < 1e-9

    def test_duration_negation(self):
        """Test negating a duration."""
        dur = Duration.from_hrs(1.0)
        neg_dur = -dur

        assert abs(neg_dur.hours + 1.0) < 1e-9

    def test_duration_abs(self):
        """Test absolute value of duration."""
        dur = Duration.from_hrs(-2.5)
        abs_dur = abs(dur)

        assert abs(abs_dur.hours - 2.5) < 1e-9


class TestEpochDurationArithmetic:
    """Test arithmetic between epochs and durations."""

    def test_epoch_plus_duration(self):
        """Test adding duration to epoch."""
        epoch = Epoch.j2000_epoch()
        duration = Duration.from_day(1.0)

        future = epoch + duration
        diff = future - epoch

        assert abs(diff.days - 1.0) < 1e-9

    def test_epoch_minus_duration(self):
        """Test subtracting duration from epoch."""
        epoch = Epoch.j2000_epoch()
        duration = Duration.from_day(1.0)

        past = epoch - duration
        diff = epoch - past

        assert abs(diff.days - 1.0) < 1e-9

    def test_epoch_difference(self):
        """Test computing duration between two epochs."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = Epoch(2000, 1, 2, 12, 0, 0, 0)  # One day later

        diff = epoch2 - epoch1

        assert abs(diff.days - 1.0) < 0.001  # Small tolerance for time scale conversions


class TestEpochComparison:
    """Test epoch comparison operations."""

    def test_epoch_equality(self):
        """Test epoch equality."""
        epoch1 = Epoch(2000, 1, 1, 12, 0, 0, 0)
        epoch2 = Epoch(2000, 1, 1, 12, 0, 0, 0)

        assert epoch1 == epoch2

    def test_epoch_less_than(self):
        """Test epoch less than comparison."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 + Duration.from_day(1.0)

        assert epoch1 < epoch2

    def test_epoch_greater_than(self):
        """Test epoch greater than comparison."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 - Duration.from_day(1.0)

        assert epoch1 > epoch2

    def test_epoch_less_equal(self):
        """Test epoch less than or equal comparison."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 + Duration.from_day(1.0)

        assert epoch1 <= epoch2
        assert epoch1 <= epoch1

    def test_epoch_greater_equal(self):
        """Test epoch greater than or equal comparison."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 - Duration.from_day(1.0)

        assert epoch1 >= epoch2
        assert epoch1 >= epoch1


class TestDurationComparison:
    """Test duration comparison operations."""

    def test_duration_equality(self):
        """Test duration equality."""
        d1 = Duration.from_hrs(1.0)
        d2 = Duration.from_min(60.0)

        assert d1 == d2

    def test_duration_less_than(self):
        """Test duration less than comparison."""
        d1 = Duration.from_hrs(1.0)
        d2 = Duration.from_hrs(2.0)

        assert d1 < d2

    def test_duration_greater_than(self):
        """Test duration greater than comparison."""
        d1 = Duration.from_hrs(2.0)
        d2 = Duration.from_hrs(1.0)

        assert d1 > d2


class TestStringRepresentation:
    """Test string representation of epochs and durations."""

    def test_epoch_repr(self):
        """Test epoch repr."""
        epoch = Epoch.j2000_epoch()
        repr_str = repr(epoch)

        assert "Epoch" in repr_str

    def test_epoch_str(self):
        """Test epoch str."""
        epoch = Epoch.j2000_epoch()
        str_repr = str(epoch)

        # Should be ISO format
        assert "2000" in str_repr

    def test_duration_repr(self):
        """Test duration repr."""
        dur = Duration.from_hrs(1.5)
        repr_str = repr(dur)

        assert "Duration" in repr_str

    def test_duration_str(self):
        """Test duration str."""
        dur = Duration.from_day(2.5)
        str_repr = str(dur)

        assert "days" in str_repr or "s" in str_repr


class TestAstrodynamicsWorkflow:
    """Test realistic astrodynamics workflows."""

    def test_orbital_period_calculation(self):
        """Test using epochs to calculate orbital period."""
        # Start at J2000
        epoch_start = Epoch.j2000_epoch()

        # ISS orbital period is about 92 minutes
        orbital_period = Duration.from_min(92.0)

        # Calculate epoch after one orbit
        epoch_end = epoch_start + orbital_period

        # Verify the time difference
        diff = epoch_end - epoch_start
        assert abs(diff.minutes - 92.0) < 1e-6

    def test_time_of_flight_calculation(self):
        """Test time-of-flight calculations."""
        # Launch epoch
        launch = Epoch(2024, 10, 22, 10, 0, 0, 0)

        # Mars transfer takes about 7 months (210 days)
        tof = Duration.from_day(210.0)

        # Arrival epoch
        arrival = launch + tof

        # Verify the calculation
        computed_tof = arrival - launch
        assert abs(computed_tof.days - 210.0) < 1e-6

    def test_propagation_timesteps(self):
        """Test creating propagation timesteps."""
        # Start epoch
        t0 = Epoch.j2000_epoch()

        # Time step of 60 seconds
        dt = Duration(60.0)

        # Generate 10 time steps
        epochs = [t0 + dt * i for i in range(10)]

        # Verify spacing
        for i in range(1, len(epochs)):
            diff = epochs[i] - epochs[i - 1]
            assert abs(diff.seconds - 60.0) < 1e-6

    def test_mission_duration(self):
        """Test calculating total mission duration."""
        # Mission start
        launch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        # Mission end (1 year later)
        end = Epoch(2025, 1, 1, 0, 0, 0, 0)

        # Calculate duration
        mission_duration = end - launch

        # Should be approximately 365 days (accounting for leap year)
        assert 364.9 < mission_duration.days < 366.1


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
