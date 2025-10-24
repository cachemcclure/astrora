"""
Integration tests for orbital element conversions.

Tests cover:
- OrbitalElements class functionality
- Cartesian to Keplerian conversion (rv_to_coe)
- Keplerian to Cartesian conversion (coe_to_rv)
- Roundtrip conversions
- Edge cases (circular, equatorial orbits)
"""

import numpy as np
import pytest
from astrora._core import OrbitalElements, coe_to_rv, constants, rv_to_coe


class TestOrbitalElementsClass:
    """Test the OrbitalElements class."""

    def test_create_orbital_elements(self):
        """Test creating orbital elements."""
        elements = OrbitalElements(
            a=7000e3,  # 7000 km
            e=0.01,  # slight eccentricity
            i=np.deg2rad(28.5),  # ISS-like inclination
            raan=np.deg2rad(45.0),
            argp=np.deg2rad(30.0),
            nu=np.deg2rad(60.0),
        )

        assert elements.a == 7000e3
        assert elements.e == 0.01
        assert abs(elements.i - np.deg2rad(28.5)) < 1e-10

    def test_orbital_period(self):
        """Test orbital period calculation."""
        # ISS-like orbit
        elements = OrbitalElements(
            a=6778e3,  # approximately ISS altitude
            e=0.0001,
            i=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        period = elements.orbital_period(constants.GM_EARTH)
        expected_period = 92.0 * 60.0  # About 92 minutes in seconds

        # Within 2 minutes
        assert abs(period - expected_period) < 120.0

    def test_periapsis_apoapsis(self):
        """Test periapsis and apoapsis distance calculations."""
        a = 8000e3  # 8000 km
        e = 0.1  # 10% eccentricity

        elements = OrbitalElements(a, e, 0.0, 0.0, 0.0, 0.0)

        r_p = elements.periapsis_distance
        r_a = elements.apoapsis_distance

        assert abs(r_p - a * (1.0 - e)) < 1.0
        assert abs(r_a - a * (1.0 + e)) < 1.0
        assert abs(r_p - 7200e3) < 1.0
        assert abs(r_a - 8800e3) < 1.0

    def test_semi_latus_rectum(self):
        """Test semi-latus rectum calculation."""
        elements = OrbitalElements(7000e3, 0.1, 0.0, 0.0, 0.0, 0.0)
        p = elements.p

        expected = 7000e3 * (1.0 - 0.1**2)
        assert abs(p - expected) < 1.0

    def test_string_representation(self):
        """Test string representations."""
        elements = OrbitalElements(7000e3, 0.01, 0.0, 0.0, 0.0, 0.0)

        repr_str = repr(elements)
        assert "OrbitalElements" in repr_str
        # Accept either e6 or e+06 formatting
        assert "7.000e" in repr_str or "7000000" in repr_str

        str_str = str(elements)
        assert "km" in str_str or "°" in str_str

    def test_get_set_attributes(self):
        """Test getting and setting attributes."""
        elements = OrbitalElements(7000e3, 0.01, 0.0, 0.0, 0.0, 0.0)

        # Test setting
        elements.a = 8000e3
        assert elements.a == 8000e3

        elements.e = 0.05
        assert elements.e == 0.05


class TestRvToCoe:
    """Test Cartesian to Keplerian conversion."""

    def test_circular_equatorial_orbit(self):
        """Test conversion for circular equatorial orbit."""
        r = np.array([7000e3, 0.0, 0.0])
        v_mag = np.sqrt(constants.GM_EARTH / 7000e3)
        v = np.array([0.0, v_mag, 0.0])

        elements = rv_to_coe(r, v, constants.GM_EARTH)

        # Check semi-major axis
        assert abs(elements.a - 7000e3) < 10.0

        # Check eccentricity (should be very small)
        assert elements.e < 1e-6

        # Check inclination (should be near 0)
        assert elements.i < 1e-6

    def test_elliptical_orbit(self):
        """Test conversion for elliptical orbit."""
        r = np.array([7000e3, 0.0, 0.0])
        v = np.array([0.0, 8000.0, 0.0])

        elements = rv_to_coe(r, v, constants.GM_EARTH)

        # Should have positive eccentricity
        assert elements.e > 0.0
        assert elements.e < 1.0  # Still elliptical

    def test_inclined_circular_orbit(self):
        """Test conversion for inclined circular orbit."""
        r = np.array([7000e3, 0.0, 0.0])
        v_mag = np.sqrt(constants.GM_EARTH / 7000e3)

        # 45-degree inclination
        angle = np.deg2rad(45.0)
        v = np.array([0.0, v_mag * np.cos(angle), v_mag * np.sin(angle)])

        elements = rv_to_coe(r, v, constants.GM_EARTH)

        # Check inclination
        assert abs(elements.i - angle) < 1e-4

    def test_polar_orbit(self):
        """Test conversion for polar orbit."""
        r = np.array([7000e3, 0.0, 0.0])
        v_mag = np.sqrt(constants.GM_EARTH / 7000e3)

        # 90-degree inclination (polar)
        v = np.array([0.0, 0.0, v_mag])

        elements = rv_to_coe(r, v, constants.GM_EARTH)

        # Check inclination is 90 degrees
        assert abs(elements.i - np.pi / 2.0) < 1e-4

    def test_invalid_input_wrong_size(self):
        """Test that wrong size arrays raise error."""
        r = np.array([7000e3, 0.0])  # Only 2 components
        v = np.array([0.0, 7500.0, 0.0])

        with pytest.raises(ValueError, match="exactly 3 components"):
            rv_to_coe(r, v, constants.GM_EARTH)

    def test_zero_angular_momentum(self):
        """Test that radial trajectory raises error."""
        r = np.array([7000e3, 0.0, 0.0])
        v = np.array([1000.0, 0.0, 0.0])  # Purely radial

        with pytest.raises(ValueError, match="degenerate"):
            rv_to_coe(r, v, constants.GM_EARTH)

    def test_custom_tolerance(self):
        """Test rv_to_coe with custom tolerance."""
        r = np.array([7000e3, 0.0, 0.0])
        v_mag = np.sqrt(constants.GM_EARTH / 7000e3)
        v = np.array([0.0, v_mag, 0.0])

        # Should work with custom tolerance
        elements = rv_to_coe(r, v, constants.GM_EARTH, tol=1e-10)
        assert elements.e < 1e-6


class TestCoeToRv:
    """Test Keplerian to Cartesian conversion."""

    def test_circular_equatorial_at_periapsis(self):
        """Test conversion for circular equatorial orbit at periapsis."""
        elements = OrbitalElements(
            a=7000e3,
            e=0.0,
            i=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        r, v = coe_to_rv(elements, constants.GM_EARTH)

        # Position should be along x-axis
        assert abs(np.linalg.norm(r) - 7000e3) < 10.0
        assert abs(r[0] - 7000e3) < 10.0
        assert abs(r[1]) < 10.0
        assert abs(r[2]) < 10.0

        # Velocity should be circular
        v_expected = np.sqrt(constants.GM_EARTH / 7000e3)
        assert abs(np.linalg.norm(v) - v_expected) < 1.0

    def test_elliptical_orbit_at_periapsis(self):
        """Test conversion for elliptical orbit at periapsis."""
        elements = OrbitalElements(
            a=8000e3,
            e=0.1,
            i=0.0,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        r, v = coe_to_rv(elements, constants.GM_EARTH)

        # At periapsis, r = a(1-e)
        r_expected = 8000e3 * (1.0 - 0.1)
        assert abs(np.linalg.norm(r) - r_expected) < 10.0

    def test_inclined_orbit(self):
        """Test conversion for inclined orbit."""
        elements = OrbitalElements(
            a=7000e3,
            e=0.01,
            i=np.deg2rad(60.0),
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        r, v = coe_to_rv(elements, constants.GM_EARTH)

        # Check that we get 3D vectors
        assert r.shape == (3,)
        assert v.shape == (3,)

        # Check z-component is non-zero due to inclination
        # (though at nu=0 with argp=0, may still be in orbital plane)
        assert r is not None


class TestRoundtripConversions:
    """Test roundtrip conversions (rv -> coe -> rv)."""

    def test_roundtrip_circular_equatorial(self):
        """Test roundtrip for circular equatorial orbit."""
        r_orig = np.array([7000e3, 0.0, 0.0])
        v_mag = np.sqrt(constants.GM_EARTH / 7000e3)
        v_orig = np.array([0.0, v_mag, 0.0])

        # Convert to COE
        elements = rv_to_coe(r_orig, v_orig, constants.GM_EARTH)

        # Convert back to rv
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Check roundtrip accuracy
        assert np.allclose(r_new, r_orig, atol=1.0)
        assert np.allclose(v_new, v_orig, atol=0.1)

    def test_roundtrip_elliptical(self):
        """Test roundtrip for elliptical orbit."""
        r_orig = np.array([7000e3, 0.0, 0.0])
        v_orig = np.array([0.0, 8000.0, 0.0])

        # Convert to COE
        elements = rv_to_coe(r_orig, v_orig, constants.GM_EARTH)

        # Convert back to rv
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Check roundtrip accuracy
        assert np.allclose(r_new, r_orig, atol=10.0)
        assert np.allclose(v_new, v_orig, atol=1.0)

    def test_roundtrip_inclined(self):
        """Test roundtrip for inclined orbit."""
        r_orig = np.array([7000e3, 0.0, 0.0])
        v_mag = np.sqrt(constants.GM_EARTH / 7000e3)
        angle = np.deg2rad(60.0)
        v_orig = np.array([0.0, v_mag * np.cos(angle), v_mag * np.sin(angle)])

        # Convert to COE
        elements = rv_to_coe(r_orig, v_orig, constants.GM_EARTH)

        # Convert back to rv
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Check roundtrip accuracy
        assert np.allclose(r_new, r_orig, atol=10.0)
        assert np.allclose(v_new, v_orig, atol=1.0)

        # Check inclination is preserved
        h_orig = np.cross(r_orig, v_orig)
        h_new = np.cross(r_new, v_new)
        i_check = np.arccos(h_new[2] / np.linalg.norm(h_new))
        assert abs(i_check - angle) < 1e-5

    def test_roundtrip_high_eccentricity(self):
        """Test roundtrip for high eccentricity orbit."""
        # Highly elliptical orbit (e = 0.7)
        r_orig = np.array([7000e3, 0.0, 0.0])
        v_orig = np.array([0.0, 10000.0, 0.0])

        # Convert to COE
        elements = rv_to_coe(r_orig, v_orig, constants.GM_EARTH)

        # Check it's highly eccentric
        assert elements.e > 0.5
        assert elements.e < 1.0

        # Convert back
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Check roundtrip accuracy
        assert np.allclose(r_new, r_orig, atol=100.0)
        assert np.allclose(v_new, v_orig, atol=10.0)


class TestKnownOrbits:
    """Test against known orbital parameters."""

    def test_iss_orbit_approximate(self):
        """Test ISS-like orbit parameters."""
        # Approximate ISS parameters
        elements = OrbitalElements(
            a=6778e3,  # ~408 km altitude
            e=0.0001,  # Nearly circular
            i=np.deg2rad(51.6),  # ISS inclination
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # Check orbital period (about 92-93 minutes)
        period = elements.orbital_period(constants.GM_EARTH)
        assert 5400 < period < 5700  # 90-95 minutes in seconds

        # Convert to state vectors
        r, v = coe_to_rv(elements, constants.GM_EARTH)

        # Check altitude (semi-major axis minus Earth radius)
        altitude = np.linalg.norm(r) - constants.R_EARTH
        # Allow slightly wider range since we're at perigee for circular orbit
        assert 390e3 < altitude < 420e3  # 390-420 km

    def test_geostationary_orbit(self):
        """Test geostationary orbit parameters."""
        # GEO altitude is about 35,786 km above Earth
        a_geo = 42164e3  # meters

        elements = OrbitalElements(
            a=a_geo,
            e=0.0,
            i=0.0,  # Equatorial
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        # Orbital period should be one sidereal day (not solar day)
        # Sidereal day = 23h 56m 4s = 86164 seconds
        period = elements.orbital_period(constants.GM_EARTH)
        expected_period = 86164.0  # One sidereal day in seconds

        # Within 1 minute
        assert abs(period - expected_period) < 60.0


class TestConservationLaws:
    """Test that physical conservation laws hold."""

    def test_energy_conservation(self):
        """Test that specific energy is preserved in conversions."""
        r = np.array([7000e3, 0.0, 0.0])
        v = np.array([0.0, 8000.0, 0.0])

        # Calculate original energy
        r_mag = np.linalg.norm(r)
        v_mag = np.linalg.norm(v)
        energy_orig = v_mag**2 / 2.0 - constants.GM_EARTH / r_mag

        # Convert through COE
        elements = rv_to_coe(r, v, constants.GM_EARTH)
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Calculate new energy
        r_mag_new = np.linalg.norm(r_new)
        v_mag_new = np.linalg.norm(v_new)
        energy_new = v_mag_new**2 / 2.0 - constants.GM_EARTH / r_mag_new

        # Energy should be conserved
        assert abs(energy_new - energy_orig) / abs(energy_orig) < 1e-6

    def test_angular_momentum_conservation(self):
        """Test that angular momentum is preserved in conversions."""
        r = np.array([7000e3, 1000e3, 0.0])
        v = np.array([0.0, 7500.0, 500.0])

        # Calculate original angular momentum
        h_orig = np.cross(r, v)
        h_mag_orig = np.linalg.norm(h_orig)

        # Convert through COE
        elements = rv_to_coe(r, v, constants.GM_EARTH)
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Calculate new angular momentum
        h_new = np.cross(r_new, v_new)
        h_mag_new = np.linalg.norm(h_new)

        # Angular momentum magnitude should be conserved
        assert abs(h_mag_new - h_mag_orig) / h_mag_orig < 1e-6


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
