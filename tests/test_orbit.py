"""
Tests for the Orbit class.

This module tests the high-level Orbit class implementation, including:
- Creation from vectors and classical elements
- Property access (r, v, a, ecc, inc, etc.)
- Propagation
- Maneuvers
- Sampling
"""

import numpy as np
import pytest
from astropy import units as u
from astrora._core import Duration, Epoch
from astrora.bodies import Earth, Mars, Sun
from astrora.twobody import Orbit


class TestOrbitCreation:
    """Test Orbit creation methods."""

    def test_from_vectors_basic(self):
        """Test creating orbit from position and velocity vectors."""
        # Circular orbit at 7000 km altitude
        r = np.array([7000e3, 0, 0])  # meters
        v = np.array([0, 7546, 0])  # m/s (approximately circular)

        orbit = Orbit.from_vectors(Earth, r, v)

        assert orbit.attractor.name == "Earth"
        # Properties return Quantities, compare values in SI units
        assert np.allclose(orbit.r.to(u.m).value, r, rtol=1e-10)
        assert np.allclose(orbit.v.to(u.m / u.s).value, v, rtol=1e-10)

    def test_from_vectors_with_epoch(self):
        """Test creating orbit with specified epoch."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        # from_midnight_utc takes (year, month, day) for midnight UTC
        epoch = Epoch.from_midnight_utc(2023, 6, 15)

        orbit = Orbit.from_vectors(Earth, r, v, epoch)

        assert orbit.epoch == epoch

    def test_from_vectors_default_epoch(self):
        """Test that default epoch is J2000."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])

        orbit = Orbit.from_vectors(Earth, r, v)

        # Should default to J2000
        j2000 = Epoch.j2000_epoch()
        assert orbit.epoch == j2000

    def test_from_classical_circular(self):
        """Test creating circular orbit from classical elements."""
        a = 7000e3  # meters
        ecc = 0.0  # circular
        inc = np.deg2rad(28.5)
        raan = 0.0
        argp = 0.0
        nu = 0.0

        orbit = Orbit.from_classical(Earth, a, ecc, inc, raan, argp, nu)

        assert orbit.attractor.name == "Earth"
        # Properties return Quantities, compare values
        assert np.isclose(orbit.a.to(u.m).value, a, rtol=1e-10)
        assert np.isclose(orbit.ecc.value, ecc, atol=1e-10)
        assert np.isclose(orbit.inc.to(u.rad).value, inc, rtol=1e-10)

    def test_from_classical_elliptical(self):
        """Test creating elliptical orbit from classical elements."""
        a = 10000e3
        ecc = 0.3
        inc = np.deg2rad(45)
        raan = np.deg2rad(30)
        argp = np.deg2rad(60)
        nu = np.deg2rad(90)

        orbit = Orbit.from_classical(Earth, a, ecc, inc, raan, argp, nu)

        # Check elements match (with some tolerance for roundtrip conversion)
        # Properties return Quantities, compare values
        assert np.isclose(orbit.a.to(u.m).value, a, rtol=1e-8)
        assert np.isclose(orbit.ecc.value, ecc, rtol=1e-8)
        assert np.isclose(orbit.inc.to(u.rad).value, inc, rtol=1e-8)
        assert np.isclose(orbit.raan.to(u.rad).value, raan, rtol=1e-8)
        assert np.isclose(orbit.argp.to(u.rad).value, argp, rtol=1e-8)
        assert np.isclose(orbit.nu.to(u.rad).value, nu, rtol=1e-8)

    def test_from_classical_different_attractors(self):
        """Test creating orbits around different bodies."""
        a = 1.5e11  # ~1 AU
        ecc = 0.1
        inc = 0.0
        raan = 0.0
        argp = 0.0
        nu = 0.0

        orbit_sun = Orbit.from_classical(Sun, a, ecc, inc, raan, argp, nu)
        orbit_earth = Orbit.from_classical(Earth, a, ecc, inc, raan, argp, nu)

        assert orbit_sun.attractor.name == "Sun"
        assert orbit_earth.attractor.name == "Earth"
        # Same elements, but different velocities due to different mu
        assert not np.allclose(orbit_sun.v.value, orbit_earth.v.value)

    def test_invalid_position_shape(self):
        """Test that invalid position shape raises error."""
        r = np.array([7000e3, 0])  # Only 2 elements
        v = np.array([0, 7546, 0])

        with pytest.raises(ValueError, match="3-element"):
            Orbit.from_vectors(Earth, r, v)

    def test_invalid_velocity_shape(self):
        """Test that invalid velocity shape raises error."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546])  # Only 2 elements

        with pytest.raises(ValueError, match="3-element"):
            Orbit.from_vectors(Earth, r, v)


class TestOrbitProperties:
    """Test Orbit property access."""

    @pytest.fixture
    def circular_orbit(self):
        """Circular Earth orbit at 7000 km."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        return Orbit.from_vectors(Earth, r, v)

    @pytest.fixture
    def elliptical_orbit(self):
        """Elliptical orbit (GTO-like)."""
        return Orbit.from_classical(
            Earth,
            a=24000e3,
            ecc=0.7,
            inc=np.deg2rad(7),
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

    def test_position_vector(self, circular_orbit):
        """Test position vector property."""
        r = circular_orbit.r
        # r is a Quantity, check it
        assert isinstance(r, u.Quantity)
        assert r.value.shape == (3,)
        assert np.isclose(np.linalg.norm(r.to(u.m).value), 7000e3, rtol=1e-10)

    def test_velocity_vector(self, circular_orbit):
        """Test velocity vector property."""
        v = circular_orbit.v
        # v is a Quantity
        assert isinstance(v, u.Quantity)
        assert v.value.shape == (3,)
        assert np.isclose(np.linalg.norm(v.to(u.m / u.s).value), 7546, rtol=1e-3)

    def test_semi_major_axis(self, circular_orbit):
        """Test semi-major axis property."""
        a = circular_orbit.a
        # a is a Quantity
        assert isinstance(a, u.Quantity)
        assert np.isclose(a.to(u.m).value, 7000e3, rtol=1e-3)

    def test_eccentricity_circular(self, circular_orbit):
        """Test eccentricity for circular orbit."""
        ecc = circular_orbit.ecc
        # ecc is a dimensionless Quantity
        assert isinstance(ecc, u.Quantity)
        # Numerical precision - circular orbit may have tiny ecc
        assert np.isclose(ecc.value, 0.0, atol=1e-4)

    def test_eccentricity_elliptical(self, elliptical_orbit):
        """Test eccentricity for elliptical orbit."""
        ecc = elliptical_orbit.ecc
        assert np.isclose(ecc, 0.7, rtol=1e-6)

    def test_inclination(self):
        """Test inclination property."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000e3,
            ecc=0.0,
            inc=np.deg2rad(51.6),  # ISS-like
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )
        assert np.isclose(orbit.inc.to(u.rad).value, np.deg2rad(51.6), rtol=1e-6)

    def test_raan(self):
        """Test RAAN property."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000e3,
            ecc=0.0,
            inc=np.deg2rad(28.5),
            raan=np.deg2rad(45),
            argp=0.0,
            nu=0.0,
        )
        assert np.isclose(orbit.raan.to(u.rad).value, np.deg2rad(45), rtol=1e-6)

    def test_argp(self):
        """Test argument of periapsis property."""
        orbit = Orbit.from_classical(
            Earth,
            a=10000e3,
            ecc=0.3,
            inc=0.0,
            raan=0.0,
            argp=np.deg2rad(90),
            nu=0.0,
        )
        assert np.isclose(orbit.argp.to(u.rad).value, np.deg2rad(90), rtol=1e-6)

    def test_true_anomaly(self):
        """Test true anomaly property."""
        orbit = Orbit.from_classical(
            Earth,
            a=10000e3,
            ecc=0.3,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=np.deg2rad(120),
        )
        assert np.isclose(orbit.nu.to(u.rad).value, np.deg2rad(120), rtol=1e-6)

    def test_period(self, circular_orbit):
        """Test orbital period property."""
        period = circular_orbit.period
        # Circular orbit at 7000 km: ~98 minutes
        expected_period = 2 * np.pi * np.sqrt(7000e3**3 / Earth.mu)
        # Relax tolerance due to round-trip element conversions
        assert np.isclose(period.to(u.s).value, expected_period, rtol=1e-4)
        assert period.to(u.s).value > 0
        assert period.to(u.s).value < 10000  # Less than ~3 hours

    def test_mean_motion(self, circular_orbit):
        """Test mean motion property."""
        n = circular_orbit.n
        # n is in rad/s, compute expected value from period
        period_s = circular_orbit.period.to(u.s).value
        expected_n = (2 * np.pi / period_s) * u.rad / u.s
        assert np.isclose(n.to(u.rad / u.s).value, expected_n.to(u.rad / u.s).value, rtol=1e-10)

    def test_specific_energy_elliptical(self, elliptical_orbit):
        """Test specific energy is negative for elliptical orbit."""
        energy = elliptical_orbit.energy
        assert energy < 0  # Bound orbit

    def test_specific_energy_circular(self, circular_orbit):
        """Test specific energy for circular orbit."""
        energy = circular_orbit.energy
        # Energy = -mu / (2*a), extract values for computation
        a_m = circular_orbit.a.to(u.m).value
        expected_energy = (-Earth.mu / (2 * a_m)) * u.m**2 / u.s**2
        assert np.isclose(energy.to(u.J / u.kg).value, expected_energy.to(u.J / u.kg).value, rtol=1e-6)

    def test_semi_latus_rectum(self, elliptical_orbit):
        """Test semi-latus rectum property."""
        p = elliptical_orbit.p
        # p = a * (1 - e²), all are Quantities
        a_val = elliptical_orbit.a.to(u.m).value
        ecc_val = elliptical_orbit.ecc.value
        expected_p_val = a_val * (1 - ecc_val**2)
        assert np.isclose(p.to(u.m).value, expected_p_val, rtol=1e-10)

    def test_periapsis_distance(self, elliptical_orbit):
        """Test periapsis distance."""
        r_p = elliptical_orbit.r_p
        # r_p = a * (1 - e), extract values
        a_val = elliptical_orbit.a.to(u.m).value
        ecc_val = elliptical_orbit.ecc.value
        expected_r_p_val = a_val * (1 - ecc_val)
        assert np.isclose(r_p.to(u.m).value, expected_r_p_val, rtol=1e-10)

    def test_apoapsis_distance(self, elliptical_orbit):
        """Test apoapsis distance."""
        r_a = elliptical_orbit.r_a
        # r_a = a * (1 + e), extract values
        a_val = elliptical_orbit.a.to(u.m).value
        ecc_val = elliptical_orbit.ecc.value
        expected_r_a_val = a_val * (1 + ecc_val)
        assert np.isclose(r_a.to(u.m).value, expected_r_a_val, rtol=1e-10)

    def test_property_caching(self, circular_orbit):
        """Test that orbital elements are cached."""
        # First access
        a1 = circular_orbit.a
        ecc1 = circular_orbit.ecc

        # Second access (should use cache)
        a2 = circular_orbit.a
        ecc2 = circular_orbit.ecc

        assert a1 == a2
        assert ecc1 == ecc2


class TestOrbitPropagation:
    """Test orbit propagation."""

    @pytest.fixture
    def iss_orbit(self):
        """ISS-like orbit."""
        return Orbit.from_classical(
            Earth,
            a=6800e3,  # ~420 km altitude
            ecc=0.0005,  # Nearly circular
            inc=np.deg2rad(51.6),
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

    def test_propagate_zero_time(self, iss_orbit):
        """Test that propagating zero time returns same orbit."""
        dt = 0.0
        future = iss_orbit.propagate(dt)

        assert np.allclose(future.r.value, iss_orbit.r.value, rtol=1e-10)
        assert np.allclose(future.v.value, iss_orbit.v.value, rtol=1e-10)

    def test_propagate_one_period(self, iss_orbit):
        """Test propagating one orbital period."""
        period = iss_orbit.period
        future = iss_orbit.propagate(period)

        # Should return to approximately same position (Keplerian assumption)
        assert np.allclose(future.r.value, iss_orbit.r.value, rtol=1e-6)
        assert np.allclose(future.v.value, iss_orbit.v.value, rtol=1e-6)

    def test_propagate_with_duration(self, iss_orbit):
        """Test propagating with Duration object."""
        dt = Duration.from_hrs(1)
        future = iss_orbit.propagate(dt)

        # Should have different position
        assert not np.allclose(future.r.value, iss_orbit.r.value, rtol=1e-3)
        # But same semi-major axis (Keplerian)
        assert np.isclose(future.a.to(u.m).value, iss_orbit.a.to(u.m).value, rtol=1e-6)

    def test_propagate_backward(self, iss_orbit):
        """Test backward propagation."""
        dt = -3600  # 1 hour backward
        past = iss_orbit.propagate(dt)

        # Propagating forward from past should return to original
        future = past.propagate(3600)
        # Relax tolerance for round-trip numerical propagation
        assert np.allclose(future.r.value, iss_orbit.r.value, rtol=1e-5, atol=1e-3)

    def test_propagate_updates_epoch(self, iss_orbit):
        """Test that propagation updates epoch."""
        dt = Duration.from_hrs(2)
        future = iss_orbit.propagate(dt)

        # Epoch should be updated
        expected_epoch = iss_orbit.epoch + dt
        assert future.epoch == expected_epoch

    def test_propagate_preserves_attractor(self, iss_orbit):
        """Test that propagation preserves attractor."""
        dt = 3600
        future = iss_orbit.propagate(dt)

        assert future.attractor.name == iss_orbit.attractor.name
        assert future.attractor.mu == iss_orbit.attractor.mu

    def test_propagate_quarter_orbit(self, iss_orbit):
        """Test propagating quarter orbit."""
        dt = iss_orbit.period / 4
        future = iss_orbit.propagate(dt)

        # True anomaly should increase by ~90 degrees
        # nu is a Quantity, extract values in radians for comparison
        delta_nu_rad = (future.nu - iss_orbit.nu).to(u.rad).value
        assert np.isclose(delta_nu_rad, np.pi / 2, rtol=0.1)  # Approximate


class TestOrbitSampling:
    """Test orbit sampling at multiple times."""

    @pytest.fixture
    def circular_orbit(self):
        """Circular orbit for sampling tests."""
        return Orbit.from_classical(
            Earth,
            a=7000e3,
            ecc=0.0,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

    def test_sample_basic(self, circular_orbit):
        """Test basic sampling functionality."""
        times = np.linspace(0, circular_orbit.period, 10)
        positions, velocities = circular_orbit.sample(times)

        assert positions.shape == (10, 3)
        assert velocities.shape == (10, 3)

    def test_sample_one_period(self, circular_orbit):
        """Test sampling over one period."""
        times = np.linspace(0, circular_orbit.period, 100)
        positions, velocities = circular_orbit.sample(times)

        # First and last positions should be close (periodic orbit)
        assert np.allclose(positions[0], positions[-1], rtol=1e-6)
        assert np.allclose(velocities[0], velocities[-1], rtol=1e-6)

    def test_sample_preserves_altitude(self, circular_orbit):
        """Test that sampling circular orbit maintains altitude."""
        times = np.linspace(0, circular_orbit.period, 50)
        positions, velocities = circular_orbit.sample(times)

        # All positions should have same magnitude (circular)
        radii = np.linalg.norm(positions, axis=1)
        assert np.allclose(radii, radii[0], rtol=1e-6)

    def test_sample_list_input(self, circular_orbit):
        """Test sampling with list input."""
        times = [0, 1000, 2000, 3000]
        positions, velocities = circular_orbit.sample(times)

        assert positions.shape == (4, 3)
        assert velocities.shape == (4, 3)


class TestOrbitManeuvers:
    """Test orbit maneuver application."""

    @pytest.fixture
    def leo_orbit(self):
        """LEO orbit for maneuver tests."""
        return Orbit.from_classical(
            Earth,
            a=6800e3,
            ecc=0.0,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

    def test_apply_prograde_burn(self, leo_orbit):
        """Test applying prograde delta-v."""
        # 100 m/s prograde burn
        v_hat = leo_orbit.v.value / np.linalg.norm(leo_orbit.v.value)
        delta_v = 100 * v_hat

        new_orbit = leo_orbit.apply_maneuver(delta_v)

        # Velocity should increase
        assert np.linalg.norm(new_orbit.v.value) > np.linalg.norm(leo_orbit.v.value)
        # Position should be same (impulsive)
        assert np.allclose(new_orbit.r.value, leo_orbit.r.value, rtol=1e-10)
        # Semi-major axis should increase (raise apoapsis)
        assert new_orbit.a > leo_orbit.a

    def test_apply_retrograde_burn(self, leo_orbit):
        """Test applying retrograde delta-v."""
        # 100 m/s retrograde burn
        v_hat = leo_orbit.v.value / np.linalg.norm(leo_orbit.v.value)
        delta_v = -100 * v_hat

        new_orbit = leo_orbit.apply_maneuver(delta_v)

        # Velocity should decrease
        assert np.linalg.norm(new_orbit.v.value) < np.linalg.norm(leo_orbit.v.value)
        # Semi-major axis should decrease
        assert new_orbit.a < leo_orbit.a

    def test_apply_normal_burn(self, leo_orbit):
        """Test applying normal (out-of-plane) delta-v."""
        # 50 m/s normal burn
        delta_v = np.array([0, 0, 50])

        new_orbit = leo_orbit.apply_maneuver(delta_v)

        # Should have non-zero inclination now (inc is a Quantity in radians)
        inc_rad = new_orbit.inc.to(u.rad).value
        assert inc_rad > 0
        assert inc_rad < np.pi / 2  # Not too large for 50 m/s

    def test_apply_zero_maneuver(self, leo_orbit):
        """Test applying zero delta-v (should be no change)."""
        delta_v = np.array([0, 0, 0])

        new_orbit = leo_orbit.apply_maneuver(delta_v)

        assert np.allclose(new_orbit.r.value, leo_orbit.r.value, rtol=1e-10)
        assert np.allclose(new_orbit.v.value, leo_orbit.v.value, rtol=1e-10)

    def test_maneuver_invalid_shape(self, leo_orbit):
        """Test that invalid delta-v shape raises error."""
        delta_v = np.array([100, 0])  # Only 2 elements

        with pytest.raises(ValueError, match="3-element array"):
            leo_orbit.apply_maneuver(delta_v)


class TestOrbitRepresentation:
    """Test orbit string representation."""

    def test_repr_basic(self):
        """Test basic __repr__ output."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000e3,
            ecc=0.1,
            inc=np.deg2rad(28.5),
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        repr_str = repr(orbit)

        assert "Orbit" in repr_str
        # Check that repr contains key information
        assert "r =" in repr_str
        assert "v =" in repr_str
        assert "a =" in repr_str
        # Should contain position and velocity
        assert "km" in repr_str
        assert "km/s" in repr_str


class TestOrbitEdgeCases:
    """Test edge cases and special orbits."""

    def test_equatorial_orbit(self):
        """Test orbit at zero inclination."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000e3,
            ecc=0.1,
            inc=0.0,  # Equatorial
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        assert np.isclose(orbit.inc.to(u.rad).value, 0.0, atol=1e-8)

    def test_polar_orbit(self):
        """Test polar orbit (90 degree inclination)."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000e3,
            ecc=0.0,
            inc=np.pi / 2,  # Polar
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        assert np.isclose(orbit.inc.to(u.rad).value, np.pi / 2, rtol=1e-8)

    def test_high_eccentricity(self):
        """Test highly elliptical orbit."""
        orbit = Orbit.from_classical(
            Earth,
            a=20000e3,
            ecc=0.9,  # Very elliptical
            inc=np.deg2rad(63.4),  # Molniya-like
            raan=0.0,
            argp=np.deg2rad(270),
            nu=0.0,
        )

        assert np.isclose(orbit.ecc.value, 0.9, rtol=1e-6)
        # Periapsis should be much closer than apoapsis
        assert orbit.r_p.value < orbit.r_a.value / 10

    def test_very_high_orbit(self):
        """Test GEO-like high orbit."""
        orbit = Orbit.from_classical(
            Earth,
            a=42164e3,  # GEO altitude
            ecc=0.0,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # Period should be ~24 hours (period is a Quantity)
        period_hours = orbit.period.to(u.h).value
        assert np.isclose(period_hours, 24.0, rtol=0.01)

    def test_different_attractors(self):
        """Test orbits around different bodies."""
        # Mars orbit
        orbit_mars = Orbit.from_classical(
            Mars,
            a=10000e3,
            ecc=0.1,
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # Sun orbit (Earth-like)
        orbit_sun = Orbit.from_classical(
            Sun,
            a=1.5e11,  # ~1 AU
            ecc=0.017,  # Earth-like
            inc=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        assert orbit_mars.attractor.name == "Mars"
        assert orbit_sun.attractor.name == "Sun"
        assert orbit_sun.period > orbit_mars.period  # Much longer period


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
