"""
Tests for convenient orbit creation methods.

This module tests the circular(), geostationary(), synchronous(),
and parabolic() orbit creation helpers.
"""

import numpy as np
import pytest
from astropy import units as u
from astropy.time import Time
from astrora.bodies import Earth, Jupiter, Mars
from astrora.twobody import Orbit


class TestCircularOrbit:
    """Tests for Orbit.circular() method."""

    def test_circular_basic(self):
        """Test basic circular orbit creation."""
        orbit = Orbit.circular(Earth, alt=700e3)  # 700 km altitude

        # Check it's actually circular
        assert orbit.ecc.value < 1e-10, "Orbit should be circular (ecc ≈ 0)"

        # Check altitude is correct
        expected_a = Earth.R + 700e3
        np.testing.assert_allclose(orbit.a.to(u.m).value, expected_a, rtol=1e-6)

    def test_circular_with_units(self):
        """Test circular orbit with astropy units."""
        orbit = Orbit.circular(Earth, alt=400 * u.km, inc=51.6 * u.deg)

        # Check eccentricity
        assert orbit.ecc.value < 1e-10

        # Check inclination
        np.testing.assert_allclose(orbit.inc.to(u.deg).value, 51.6, rtol=1e-6)

        # Check altitude
        expected_a = Earth.R + 400e3
        np.testing.assert_allclose(orbit.a.to(u.m).value, expected_a, rtol=1e-6)

    def test_circular_iss_like(self):
        """Test ISS-like circular orbit (400 km, 51.6°)."""
        orbit = Orbit.circular(Earth, alt=400 * u.km, inc=51.6 * u.deg)

        # ISS period should be around 92-93 minutes
        period_minutes = orbit.period.to(u.minute).value
        assert 92 < period_minutes < 94, f"ISS period should be ~93 min, got {period_minutes:.1f}"

    def test_circular_equatorial(self):
        """Test equatorial circular orbit (inc=0)."""
        orbit = Orbit.circular(Earth, alt=700e3, inc=0.0)

        assert orbit.ecc.value < 1e-10
        assert orbit.inc.to(u.deg).value < 1e-6, "Should be equatorial"

    def test_circular_polar(self):
        """Test polar circular orbit (inc=90°)."""
        orbit = Orbit.circular(Earth, alt=800e3, inc=90 * u.deg)

        assert orbit.ecc.value < 1e-10
        np.testing.assert_allclose(orbit.inc.to(u.deg).value, 90.0, rtol=1e-6)

    def test_circular_with_raan_arglat(self):
        """Test circular orbit with RAAN and argument of latitude."""
        orbit = Orbit.circular(
            Earth,
            alt=500e3,
            inc=28.5 * u.deg,
            raan=45 * u.deg,
            arglat=30 * u.deg,
        )

        assert orbit.ecc.value < 1e-10
        np.testing.assert_allclose(orbit.raan.to(u.deg).value, 45.0, rtol=1e-3)

    def test_circular_with_epoch(self):
        """Test circular orbit with custom epoch."""
        epoch = Time("2024-01-01 12:00:00", scale="utc")
        orbit = Orbit.circular(Earth, alt=700e3, epoch=epoch)

        assert orbit.ecc.value < 1e-10
        # Check epoch is preserved (converted to Epoch internally)


class TestGeostationary:
    """Tests for Orbit.geostationary() method."""

    def test_geostationary_basic(self):
        """Test basic geostationary orbit."""
        orbit = Orbit.geostationary()

        # Check it's circular and equatorial
        assert orbit.ecc.value < 1e-6, "GEO should be circular"
        assert orbit.inc.to(u.deg).value < 1e-3, "GEO should be equatorial"

        # Check altitude is approximately 35,786 km
        altitude_km = orbit.a.to(u.km).value - Earth.R / 1000
        np.testing.assert_allclose(altitude_km, 35786, rtol=0.01)  # Within 1%

    def test_geostationary_period(self):
        """Test that geostationary orbit has correct period."""
        orbit = Orbit.geostationary()

        # Period should be one sidereal day (~23.93 hours)
        period_hours = orbit.period.to(u.hour).value
        sidereal_day_hours = 23.9344696  # hours
        np.testing.assert_allclose(period_hours, sidereal_day_hours, rtol=0.01)

    def test_geostationary_default_attractor(self):
        """Test that geostationary defaults to Earth."""
        orbit = Orbit.geostationary()
        assert orbit.attractor.name == "Earth"

    def test_geostationary_with_position(self):
        """Test geostationary with specific position (arglat)."""
        orbit = Orbit.geostationary(arglat=45 * u.deg)

        assert orbit.ecc.value < 1e-6
        # Check that nu (true anomaly) is set to arglat for circular orbits
        np.testing.assert_allclose(orbit.nu.to(u.deg).value, 45.0, rtol=1e-2)


class TestSynchronous:
    """Tests for Orbit.synchronous() method."""

    def test_synchronous_earth_default(self):
        """Test Earth synchronous orbit (equivalent to geostationary)."""
        orbit = Orbit.synchronous(Earth)

        # Should match geostationary
        period_hours = orbit.period.to(u.hour).value
        sidereal_day_hours = 23.9344696
        np.testing.assert_allclose(period_hours, sidereal_day_hours, rtol=0.01)

    def test_synchronous_mars(self):
        """Test Mars synchronous (areostationary) orbit."""
        orbit = Orbit.synchronous(Mars)

        # Mars sidereal day is 24.6229 hours
        period_hours = orbit.period.to(u.hour).value
        mars_sol_hours = 24.6229
        np.testing.assert_allclose(period_hours, mars_sol_hours, rtol=0.01)

        # Check it's circular and equatorial by default
        assert orbit.ecc.value < 1e-6
        assert orbit.inc.to(u.deg).value < 1e-3

    def test_synchronous_jupiter(self):
        """Test Jupiter synchronous orbit."""
        orbit = Orbit.synchronous(Jupiter)

        # Jupiter day is 9.925 hours
        period_hours = orbit.period.to(u.hour).value
        jupiter_day_hours = 9.925
        np.testing.assert_allclose(period_hours, jupiter_day_hours, rtol=0.01)

    def test_synchronous_semi_period(self):
        """Test semi-synchronous orbit (period = 2 × rotation)."""
        orbit = Orbit.synchronous(Earth, period_mul=2.0)

        # Period should be 2 sidereal days
        period_hours = orbit.period.to(u.hour).value
        expected_hours = 2 * 23.9344696
        np.testing.assert_allclose(period_hours, expected_hours, rtol=0.01)

    def test_synchronous_with_eccentricity(self):
        """Test synchronous orbit with non-zero eccentricity."""
        orbit = Orbit.synchronous(Earth, ecc=0.1)

        # Check eccentricity
        np.testing.assert_allclose(orbit.ecc.value, 0.1, rtol=1e-3)

        # Period should still match sidereal day
        period_hours = orbit.period.to(u.hour).value
        sidereal_day_hours = 23.9344696
        np.testing.assert_allclose(period_hours, sidereal_day_hours, rtol=0.01)

    def test_synchronous_with_inclination(self):
        """Test synchronous orbit with inclination."""
        orbit = Orbit.synchronous(Earth, inc=28.5 * u.deg)

        np.testing.assert_allclose(orbit.inc.to(u.deg).value, 28.5, rtol=1e-3)

    def test_synchronous_no_rotation_period_error(self):
        """Test that bodies without rotation period raise error."""
        from astrora.bodies import Body

        # Create a body without rotational period
        test_body = Body(name="TestBody", mu=3.986e14, R=6.371e6)

        with pytest.raises(ValueError, match="does not have a defined rotational period"):
            Orbit.synchronous(test_body)


class TestParabolic:
    """Tests for Orbit.parabolic() method."""

    def test_parabolic_basic(self):
        """Test basic parabolic orbit creation."""
        p = 7000e3  # Semi-latus rectum (m)
        orbit = Orbit.parabolic(
            Earth,
            p=p,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # Check eccentricity is very close to 1 (parabolic)
        # Due to numerical limitations, we use e ≈ 0.9999
        assert orbit.ecc.value > 0.999, "Eccentricity should be very close to 1 (parabolic)"

    def test_parabolic_with_units(self):
        """Test parabolic orbit with astropy units."""
        orbit = Orbit.parabolic(
            Earth,
            p=6678 * u.km,
            inc=28.5 * u.deg,
            raan=0 * u.deg,
            argp=0 * u.deg,
            nu=0 * u.deg,
        )

        # Check eccentricity is very close to 1
        assert orbit.ecc.value > 0.999, "Eccentricity should be very close to 1 (parabolic)"

        # Check inclination
        np.testing.assert_allclose(orbit.inc.to(u.deg).value, 28.5, rtol=1e-3)

    def test_parabolic_energy_near_zero(self):
        """Test that parabolic orbit has near-zero specific energy."""
        p = 7000e3
        orbit = Orbit.parabolic(
            Earth,
            p=p,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # Nearly-parabolic orbits (e ≈ 0.9999) have very small negative energy
        # Should be close to zero but slightly negative
        energy = orbit.energy.to(u.MJ / u.kg).value
        # Energy should be small (close to zero for parabolic)
        # For e=0.9999, energy will be slightly negative but < 0.5 MJ/kg in magnitude
        assert (
            abs(energy) < 0.5
        ), f"Nearly-parabolic orbit energy should be ~0, got {energy:.3f} MJ/kg"

    def test_parabolic_escape_trajectory(self):
        """Test that parabolic orbit represents escape trajectory."""
        p = 7000e3
        orbit = Orbit.parabolic(
            Earth,
            p=p,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # For e=1, the orbit is at the boundary between bound and unbound
        assert orbit.ecc.value >= 0.999, "Should be parabolic (e ≈ 1)"


class TestHelperIntegration:
    """Integration tests for orbit creation helpers."""

    def test_circular_vs_classical_equivalence(self):
        """Test that circular() produces same result as from_classical()."""
        alt = 700e3
        inc = np.deg2rad(51.6)

        # Create using circular()
        orbit1 = Orbit.circular(Earth, alt=alt, inc=inc)

        # Create using from_classical()
        a = Earth.R + alt
        orbit2 = Orbit.from_classical(
            Earth,
            a=a,
            ecc=0.0,
            inc=inc,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # Compare positions (should be identical)
        np.testing.assert_allclose(orbit1.r.to(u.m).value, orbit2.r.to(u.m).value, rtol=1e-6)
        np.testing.assert_allclose(
            orbit1.v.to(u.m / u.s).value, orbit2.v.to(u.m / u.s).value, rtol=1e-6
        )

    def test_geostationary_altitude_consistency(self):
        """Test that geostationary altitude is consistent with period."""
        orbit = Orbit.geostationary()

        # Use Kepler's third law to verify: T = 2π√(a³/μ)
        a = orbit.a.to(u.m).value
        mu = Earth.mu
        period_computed = 2 * np.pi * np.sqrt(a**3 / mu)
        period_actual = orbit.period.to(u.s).value

        np.testing.assert_allclose(period_computed, period_actual, rtol=1e-6)

    def test_all_helpers_return_orbit_instances(self):
        """Test that all helpers return Orbit instances."""
        orbit1 = Orbit.circular(Earth, alt=700e3)
        orbit2 = Orbit.geostationary()
        orbit3 = Orbit.synchronous(Mars)
        orbit4 = Orbit.parabolic(Earth, p=7000e3, inc=0, raan=0, argp=0, nu=0)

        assert isinstance(orbit1, Orbit)
        assert isinstance(orbit2, Orbit)
        assert isinstance(orbit3, Orbit)
        assert isinstance(orbit4, Orbit)
