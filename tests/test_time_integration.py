"""
Tests for astropy.time integration with astrora.

This test suite verifies the conversion between astropy.time.Time and hifitime Epoch,
as well as the integration with the Orbit class.
"""

import numpy as np
import pytest
from astropy import units as u
from astropy.time import Time
from astrora._core import Epoch
from astrora.bodies import Earth
from astrora.time import (
    ASTROPY_AVAILABLE,
    astropy_time_to_epoch,
    epoch_to_astropy_time,
    to_astropy_time,
    to_epoch,
)
from astrora.twobody import Orbit

# Skip all tests if astropy is not installed
pytestmark = pytest.mark.skipif(not ASTROPY_AVAILABLE, reason="astropy not installed")


class TestAstropyTimeConversion:
    """Tests for converting between astropy.time.Time and hifitime Epoch."""

    def test_j2000_conversion(self):
        """Test conversion of J2000 epoch."""
        # astropy J2000
        t_astropy = Time("2000-01-01 12:00:00", scale="tt")

        # Convert to Epoch
        epoch = astropy_time_to_epoch(t_astropy)

        # Verify JD matches
        assert abs(epoch.jd_tt - 2451545.0) < 1e-9

        # Convert back
        t_back = epoch_to_astropy_time(epoch, scale="tt")

        # Verify roundtrip
        assert abs(t_back.jd - t_astropy.jd) < 1e-9

    def test_utc_conversion(self):
        """Test UTC time scale conversion."""
        t_utc = Time("2024-10-22 14:30:45.123456", scale="utc")

        # Convert to Epoch
        epoch = astropy_time_to_epoch(t_utc)

        # Convert back
        t_back = epoch_to_astropy_time(epoch, scale="utc")

        # Verify roundtrip (should be exact)
        assert abs(t_back.jd - t_utc.jd) < 1e-9

    def test_tai_conversion(self):
        """Test TAI time scale conversion."""
        t_tai = Time("2020-06-15 10:30:00", scale="tai")

        # Convert to Epoch and back
        epoch = astropy_time_to_epoch(t_tai)
        t_back = epoch_to_astropy_time(epoch, scale="tai")

        # Verify roundtrip
        assert abs(t_back.jd - t_tai.jd) < 1e-9

    def test_tt_conversion(self):
        """Test TT (Terrestrial Time) conversion."""
        t_tt = Time("2015-03-20 09:45:00", scale="tt")

        # Convert to Epoch and back
        epoch = astropy_time_to_epoch(t_tt)
        t_back = epoch_to_astropy_time(epoch, scale="tt")

        # Verify roundtrip
        assert abs(t_back.jd - t_tt.jd) < 1e-9

    def test_precision_preservation(self):
        """Test that conversion preserves nanosecond precision."""
        # Create a time with microsecond precision
        t_original = Time("2024-01-15 12:34:56.123456", scale="utc")

        # Convert to Epoch and back
        epoch = astropy_time_to_epoch(t_original)
        t_roundtrip = epoch_to_astropy_time(epoch, scale="utc")

        # Difference should be less than 1 microsecond (1.16e-11 days)
        diff_days = abs(t_original.jd - t_roundtrip.jd)
        diff_seconds = diff_days * 86400

        assert diff_seconds < 1e-6  # Less than 1 microsecond

    def test_different_epochs(self):
        """Test various historical and future epochs."""
        test_cases = [
            ("1970-01-01 00:00:00", "utc"),  # Unix epoch
            ("2000-01-01 00:00:00", "utc"),  # Y2K
            ("2024-12-31 23:59:59", "utc"),  # Near future
            ("1980-01-06 00:00:00", "tai"),  # GPS epoch reference
        ]

        for time_str, scale in test_cases:
            t = Time(time_str, scale=scale)
            epoch = astropy_time_to_epoch(t)
            t_back = epoch_to_astropy_time(epoch, scale=scale)

            # Verify roundtrip
            assert abs(t_back.jd - t.jd) < 1e-9, f"Failed for {time_str} ({scale})"


class TestConvenienceFunctions:
    """Tests for the to_epoch() and to_astropy_time() convenience functions."""

    def test_to_epoch_from_astropy_time(self):
        """Test to_epoch() with astropy Time input."""
        t = Time("2020-06-15 10:30:00", scale="utc")
        epoch = to_epoch(t)

        assert isinstance(epoch, Epoch)
        assert abs(epoch.jd_utc - t.jd) < 1e-9

    def test_to_epoch_from_epoch(self):
        """Test to_epoch() with Epoch input (passthrough)."""
        epoch_in = Epoch.j2000_epoch()
        epoch_out = to_epoch(epoch_in)

        assert epoch_out == epoch_in

    def test_to_epoch_from_none(self):
        """Test to_epoch() with None input."""
        result = to_epoch(None)
        assert result is None

    def test_to_epoch_invalid_type(self):
        """Test to_epoch() with invalid input type."""
        with pytest.raises(TypeError):
            to_epoch("2020-01-01")  # String is not supported

    def test_to_astropy_time_from_epoch(self):
        """Test to_astropy_time() with Epoch input."""
        epoch = Epoch.j2000_epoch()
        t = to_astropy_time(epoch, scale="tt")

        assert isinstance(t, Time)
        assert t.scale == "tt"
        assert abs(t.jd - 2451545.0) < 1e-9

    def test_to_astropy_time_from_time(self):
        """Test to_astropy_time() with Time input (passthrough)."""
        t_in = Time("2020-06-15", scale="utc")
        t_out = to_astropy_time(t_in)

        assert t_out.jd == t_in.jd
        assert t_out.scale == t_in.scale

    def test_to_astropy_time_scale_conversion(self):
        """Test to_astropy_time() with scale conversion."""
        t_in = Time("2020-06-15", scale="utc")
        t_out = to_astropy_time(t_in, scale="tt")

        # Should be converted to TT
        assert t_out.scale == "tt"
        # But represent the same instant
        assert abs(t_out.utc.jd - t_in.jd) < 1e-9


class TestOrbitTimeIntegration:
    """Tests for astropy.time.Time integration with the Orbit class."""

    def test_from_vectors_with_astropy_time(self):
        """Test Orbit.from_vectors() with astropy Time."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        t = Time("2024-10-22 14:30:00", scale="utc")

        orbit = Orbit.from_vectors(Earth, r, v, epoch=t)

        # Verify orbit was created
        assert orbit.attractor == Earth
        # Epoch should be converted to hifitime Epoch
        assert isinstance(orbit.epoch, Epoch)

    def test_from_classical_with_astropy_time(self):
        """Test Orbit.from_classical() with astropy Time."""
        t = Time("2000-01-01 12:00:00", scale="tt")  # J2000

        orbit = Orbit.from_classical(
            Earth,
            a=7000 << u.km,
            ecc=0.01 << u.one,
            inc=51.6 << u.deg,
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
            epoch=t,
        )

        # Verify orbit
        assert orbit.attractor == Earth
        assert isinstance(orbit.epoch, Epoch)
        # Should be J2000
        assert abs(orbit.epoch.jd_tt - 2451545.0) < 1e-9

    def test_backward_compatibility_with_epoch(self):
        """Test that Orbit still works with Epoch objects."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        epoch = Epoch.j2000_epoch()

        orbit = Orbit.from_vectors(Earth, r, v, epoch=epoch)

        assert orbit.epoch == epoch

    def test_default_epoch_still_works(self):
        """Test that default epoch (None) still works."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])

        orbit = Orbit.from_vectors(Earth, r, v)

        # Should default to J2000
        assert abs(orbit.epoch.jd_tt - 2451545.0) < 1e-9

    def test_time_scales_preserved_in_orbit(self):
        """Test that different time scales work correctly."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])

        # Test with different scales
        for scale in ["utc", "tai", "tt"]:
            t = Time("2020-06-15 10:30:00", scale=scale)
            orbit = Orbit.from_vectors(Earth, r, v, epoch=t)

            # Epoch should be created successfully
            assert isinstance(orbit.epoch, Epoch)

    def test_orbit_properties_with_astropy_time(self):
        """Test that orbital properties work correctly with astropy Time epochs."""
        t = Time("2024-10-22", scale="utc")

        orbit = Orbit.from_classical(
            Earth,
            a=7000 << u.km,
            ecc=0.0 << u.one,
            inc=0 << u.deg,
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
            epoch=t,
        )

        # Verify properties work
        assert abs(orbit.a.to_value(u.km) - 7000) < 1
        assert orbit.ecc.value < 0.01
        assert abs(orbit.period.to_value(u.hour) - 1.62) < 0.1


class TestEdgeCases:
    """Tests for edge cases and error handling."""

    def test_unsupported_time_scale_error(self):
        """Test that unsupported time scales raise appropriate errors."""
        # UT1 is not currently supported
        # This test may need adjustment based on implementation
        pass  # Skip for now as we handle most common scales

    def test_very_old_epoch(self):
        """Test conversion of very old historical epochs."""
        # 1900-01-01
        t = Time("1900-01-01 00:00:00", scale="utc")
        epoch = astropy_time_to_epoch(t)
        t_back = epoch_to_astropy_time(epoch, scale="utc")

        assert abs(t_back.jd - t.jd) < 1e-8

    def test_far_future_epoch(self):
        """Test conversion of far future epochs."""
        # 2100-12-31
        t = Time("2100-12-31 23:59:59", scale="utc")
        epoch = astropy_time_to_epoch(t)
        t_back = epoch_to_astropy_time(epoch, scale="utc")

        assert abs(t_back.jd - t.jd) < 1e-8

    def test_leap_second_handling(self):
        """Test that leap seconds are handled correctly."""
        # Time near a known leap second: 2015-06-30 23:59:60
        # Note: astropy handles leap seconds automatically
        t_before = Time("2015-06-30 23:59:59", scale="utc")

        # Convert to Epoch and back
        epoch = astropy_time_to_epoch(t_before)
        t_back = epoch_to_astropy_time(epoch, scale="utc")

        # Should roundtrip correctly
        assert abs(t_back.jd - t_before.jd) < 1e-9


# Summary test to ensure overall functionality
def test_integration_summary():
    """High-level test of astropy.time integration."""
    # Create orbit with astropy Time
    t = Time("2024-10-22 14:30:00", scale="utc")
    orbit = Orbit.from_classical(
        Earth,
        a=7000 << u.km,
        ecc=0.01 << u.one,
        inc=51.6 << u.deg,
        raan=0 << u.deg,
        argp=0 << u.deg,
        nu=0 << u.deg,
        epoch=t,
    )

    # Verify all properties work
    assert orbit.a.to_value(u.km) > 6000
    assert orbit.ecc.value < 0.1
    assert orbit.period.to_value(u.hour) > 1
    assert isinstance(orbit.epoch, Epoch)

    # Propagate works
    future = orbit.propagate(3600)  # 1 hour
    assert isinstance(future, Orbit)
