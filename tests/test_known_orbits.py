"""
Validation tests against known reference values from authoritative sources.

This test file validates state vector conversions (Cartesian ↔ Keplerian)
against published examples from standard astrodynamics textbooks and
mission data.

References:
1. Curtis, H. D. "Orbital Mechanics for Engineering Students", 3rd Ed.
2. Vallado, D. A. "Fundamentals of Astrodynamics and Applications", 4th Ed.
3. NASA mission data and orbital parameters
4. Published academic examples from orbital-mechanics.space

These tests ensure that our implementation matches established reference
implementations and textbook solutions.
"""

import numpy as np
import pytest
from astrora._core import OrbitalElements, coe_to_rv, constants, rv_to_coe


class TestCurtisExamples:
    """Test cases from Curtis's "Orbital Mechanics for Engineering Students"."""

    def test_curtis_example_4_3(self):
        """
        Example 4.3 from Curtis 3rd Edition, page 200.

        This is a well-known textbook example that converts position and
        velocity vectors to classical orbital elements.

        Reference: Curtis, H. D. "Orbital Mechanics for Engineering Students",
        3rd Edition, Example 4.3, p. 200.
        """
        # Input state vectors from Example 4.3
        r = np.array([-6045.0e3, -3490.0e3, 2500.0e3])  # meters
        v = np.array([-3.457e3, 6.618e3, 2.533e3])  # m/s

        # Convert to orbital elements
        elements = rv_to_coe(r, v, constants.GM_EARTH)

        # Expected values from textbook (converted to SI units)
        expected_e = 0.1712  # eccentricity
        expected_i = 153.25  # degrees
        expected_raan = 255.28  # degrees
        expected_argp = 20.07  # degrees
        expected_nu = 28.45  # degrees

        # Validate results (allowing reasonable tolerances for numerical precision)
        assert (
            abs(elements.e - expected_e) < 0.001
        ), f"Eccentricity mismatch: got {elements.e}, expected {expected_e}"

        assert (
            abs(np.rad2deg(elements.i) - expected_i) < 0.1
        ), f"Inclination mismatch: got {np.rad2deg(elements.i)}°, expected {expected_i}°"

        assert (
            abs(np.rad2deg(elements.raan) - expected_raan) < 0.1
        ), f"RAAN mismatch: got {np.rad2deg(elements.raan)}°, expected {expected_raan}°"

        assert (
            abs(np.rad2deg(elements.argp) - expected_argp) < 0.1
        ), f"Argument of periapsis mismatch: got {np.rad2deg(elements.argp)}°, expected {expected_argp}°"

        assert (
            abs(np.rad2deg(elements.nu) - expected_nu) < 0.1
        ), f"True anomaly mismatch: got {np.rad2deg(elements.nu)}°, expected {expected_nu}°"

        # Also verify the semi-latus rectum from the example
        expected_p = 8530.47e3  # meters (from textbook)
        actual_p = elements.p  # Property in Python interface
        assert (
            abs(actual_p - expected_p) < 100.0
        ), f"Semi-latus rectum mismatch: got {actual_p/1e3} km, expected {expected_p/1e3} km"


class TestMolniyaOrbit:
    """Test cases for Molniya orbits (highly elliptical Russian communications satellites)."""

    def test_molniya_orbit_example(self):
        """
        Molniya orbit example from academic literature.

        This example validates a highly elliptical orbit typical of Molniya
        communications satellites used by Russia for high-latitude coverage.

        Reference: Holooly.com orbital mechanics problem,
        Space Exploration Stack Exchange discussions.
        """
        # State vector in ECI coordinates (meters and m/s)
        r = np.array([9031.5e3, -5316.9e3, -1647.2e3])
        v = np.array([-2.8640e3, 5.1112e3, -5.0805e3])

        # Convert to orbital elements
        elements = rv_to_coe(r, v, constants.GM_EARTH)

        # Expected Molniya orbit characteristics
        # Semi-major axis: ~26,564 km (12-hour period)
        expected_a_min = 26000e3  # meters
        expected_a_max = 27000e3  # meters

        # Eccentricity: ~0.74 (highly elliptical)
        expected_e_min = 0.70
        expected_e_max = 0.78

        # Inclination: ~63.4° (critical inclination to minimize apsidal rotation)
        expected_i_min = 60.0  # degrees
        expected_i_max = 66.0  # degrees

        # Validate semi-major axis
        assert (
            expected_a_min < elements.a < expected_a_max
        ), f"Semi-major axis out of Molniya range: got {elements.a/1e3} km"

        # Validate eccentricity
        assert (
            expected_e_min < elements.e < expected_e_max
        ), f"Eccentricity out of Molniya range: got {elements.e}"

        # Validate inclination
        assert (
            expected_i_min < np.rad2deg(elements.i) < expected_i_max
        ), f"Inclination out of Molniya range: got {np.rad2deg(elements.i)}°"

        # Verify orbital period is approximately 12 hours (Molniya characteristic)
        period = elements.orbital_period(constants.GM_EARTH)
        expected_period = 12.0 * 3600.0  # 12 hours in seconds
        assert (
            abs(period - expected_period) < 600.0
        ), f"Orbital period mismatch: got {period/3600} hours, expected ~12 hours"

    def test_molniya_apogee_perigee(self):
        """
        Test Molniya orbit apogee and perigee altitudes.

        Molniya orbits have very high apogees (~40,000 km altitude) and
        low perigees (~500 km altitude) to provide long dwell time over
        high latitudes.
        """
        # Typical Molniya orbital elements
        a = 26564e3  # meters (semi-major axis)
        e = 0.74  # eccentricity
        i = np.deg2rad(63.4)  # critical inclination

        elements = OrbitalElements(
            a=a,
            e=e,
            i=i,
            raan=0.0,
            argp=np.deg2rad(-90.0),  # Perigee over southern hemisphere
            nu=0.0,
        )

        # Calculate apogee and perigee altitudes
        r_a = elements.apoapsis_distance
        r_p = elements.periapsis_distance

        # Convert to altitude (subtract Earth radius)
        altitude_a = (r_a - constants.R_EARTH) / 1e3  # km
        altitude_p = (r_p - constants.R_EARTH) / 1e3  # km

        # Typical Molniya altitudes
        assert 35000 < altitude_a < 45000, f"Apogee altitude out of range: {altitude_a} km"

        assert 400 < altitude_p < 1500, f"Perigee altitude out of range: {altitude_p} km"


class TestGeostationary:
    """Test cases for geostationary orbit parameters."""

    def test_geo_orbit_parameters(self):
        """
        Test geostationary orbit characteristics.

        GEO satellites orbit at a specific altitude (35,786 km) with a period
        matching Earth's sidereal day, making them appear stationary relative
        to ground observers.

        Reference: Standard orbital mechanics texts, NASA factsheets.
        """
        # GEO semi-major axis
        a_geo = 42164e3  # meters (from Earth's center)

        elements = OrbitalElements(
            a=a_geo, e=0.0, i=0.0, raan=0.0, argp=0.0, nu=0.0  # Circular  # Equatorial
        )

        # Orbital period should be one sidereal day
        # Sidereal day = 23h 56m 4.0916s = 86164.0916 seconds
        period = elements.orbital_period(constants.GM_EARTH)
        sidereal_day = 86164.0916  # seconds

        assert (
            abs(period - sidereal_day) < 1.0
        ), f"GEO period mismatch: got {period} s, expected {sidereal_day} s"

        # Convert to state vectors
        r, v = coe_to_rv(elements, constants.GM_EARTH)

        # Verify position magnitude equals semi-major axis
        assert (
            abs(np.linalg.norm(r) - a_geo) < 1.0
        ), "Position magnitude doesn't match semi-major axis"

        # GEO altitude should be 35,786 km above Earth's surface
        altitude = (np.linalg.norm(r) - constants.R_EARTH) / 1e3
        expected_altitude = 35786.0  # km

        assert (
            abs(altitude - expected_altitude) < 10.0
        ), f"GEO altitude mismatch: got {altitude} km, expected {expected_altitude} km"

        # Orbital velocity at GEO
        v_mag = np.linalg.norm(v)
        expected_v = 3.0747e3  # m/s (from v = √(μ/r))

        assert (
            abs(v_mag - expected_v) < 1.0
        ), f"GEO velocity mismatch: got {v_mag} m/s, expected {expected_v} m/s"


class TestLEOOrbits:
    """Test cases for Low Earth Orbit satellites."""

    def test_iss_current_orbit(self):
        """
        Test ISS orbital parameters based on typical values.

        The ISS maintains a nearly circular orbit at approximately 408 km
        altitude with 51.6° inclination (optimized for launches from both
        Kennedy Space Center and Baikonur Cosmodrome).

        Reference: NASA ISS factsheet, real-time tracking data.
        """
        # Typical ISS orbital elements
        altitude = 408e3  # meters above surface
        a = constants.R_EARTH + altitude
        e = 0.0002  # Nearly circular (typical range 0.0001-0.0020)
        i = np.deg2rad(51.64)  # ISS inclination

        elements = OrbitalElements(a=a, e=e, i=i, raan=0.0, argp=0.0, nu=0.0)

        # Orbital period should be approximately 92-93 minutes
        period = elements.orbital_period(constants.GM_EARTH)
        period_minutes = period / 60.0

        assert 90.0 < period_minutes < 95.0, f"ISS period out of range: {period_minutes} minutes"

        # Verify orbital velocity (ISS travels at ~7.66 km/s)
        r, v = coe_to_rv(elements, constants.GM_EARTH)
        v_mag = np.linalg.norm(v) / 1e3  # km/s

        assert 7.5 < v_mag < 7.8, f"ISS velocity out of range: {v_mag} km/s"

    def test_polar_sun_synchronous_orbit(self):
        """
        Test polar sun-synchronous orbit parameters.

        Sun-synchronous orbits are commonly used for Earth observation
        satellites. They maintain a constant angle with respect to the Sun,
        providing consistent lighting conditions for imaging.

        Typical altitude: 600-800 km
        Inclination: ~98° (retrograde polar orbit)

        Reference: Standard satellite mission designs.
        """
        # Typical sun-synchronous orbit parameters
        altitude = 700e3  # meters
        a = constants.R_EARTH + altitude
        e = 0.001  # Nearly circular
        i = np.deg2rad(98.0)  # Retrograde polar orbit

        elements = OrbitalElements(a=a, e=e, i=i, raan=0.0, argp=0.0, nu=0.0)

        # Orbital period should be approximately 98-100 minutes
        period = elements.orbital_period(constants.GM_EARTH)
        period_minutes = period / 60.0

        assert (
            97.0 < period_minutes < 101.0
        ), f"Sun-synchronous orbit period out of range: {period_minutes} minutes"

        # Verify inclination is in sun-synchronous range
        assert 97.0 < np.rad2deg(elements.i) < 99.0, "Inclination not in sun-synchronous range"


class TestRoundtripConsistency:
    """
    Test roundtrip conversion consistency with known orbits.

    These tests verify that converting from orbital elements to state vectors
    and back (or vice versa) preserves the orbital parameters within acceptable
    numerical precision.
    """

    def test_roundtrip_curtis_example(self):
        """
        Roundtrip test using Curtis Example 4.3.

        This test verifies that we can convert the state vector to orbital
        elements and back to the original state vector with minimal error.
        """
        # Original state vector from Curtis Example 4.3
        r_orig = np.array([-6045.0e3, -3490.0e3, 2500.0e3])
        v_orig = np.array([-3.457e3, 6.618e3, 2.533e3])

        # Convert to orbital elements
        elements = rv_to_coe(r_orig, v_orig, constants.GM_EARTH)

        # Convert back to state vectors
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Verify roundtrip accuracy
        r_error = np.linalg.norm(r_new - r_orig)
        v_error = np.linalg.norm(v_new - v_orig)

        assert r_error < 100.0, f"Position error in roundtrip: {r_error} m"

        assert v_error < 0.1, f"Velocity error in roundtrip: {v_error} m/s"

        # Verify orbital energy is conserved
        energy_orig = np.linalg.norm(v_orig) ** 2 / 2.0 - constants.GM_EARTH / np.linalg.norm(
            r_orig
        )
        energy_new = np.linalg.norm(v_new) ** 2 / 2.0 - constants.GM_EARTH / np.linalg.norm(r_new)

        assert (
            abs(energy_new - energy_orig) / abs(energy_orig) < 1e-10
        ), "Energy not conserved in roundtrip"

    def test_roundtrip_molniya_orbit(self):
        """Roundtrip test for Molniya orbit."""
        r_orig = np.array([9031.5e3, -5316.9e3, -1647.2e3])
        v_orig = np.array([-2.8640e3, 5.1112e3, -5.0805e3])

        # Convert to orbital elements
        elements = rv_to_coe(r_orig, v_orig, constants.GM_EARTH)

        # Convert back to state vectors
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # For highly elliptical orbits, allow slightly larger tolerances
        r_error = np.linalg.norm(r_new - r_orig)
        v_error = np.linalg.norm(v_new - v_orig)

        assert r_error < 200.0, f"Position error in Molniya roundtrip: {r_error} m"

        assert v_error < 1.0, f"Velocity error in Molniya roundtrip: {v_error} m/s"


class TestConservationLaws:
    """
    Test that fundamental conservation laws hold for all conversions.

    These tests verify that physical invariants (energy, angular momentum)
    are preserved during state vector ↔ orbital elements conversions.
    """

    def test_energy_conservation_curtis_example(self):
        """Verify specific orbital energy is conserved for Curtis example."""
        r = np.array([-6045.0e3, -3490.0e3, 2500.0e3])
        v = np.array([-3.457e3, 6.618e3, 2.533e3])

        # Calculate original specific energy
        r_mag = np.linalg.norm(r)
        v_mag = np.linalg.norm(v)
        energy_orig = v_mag**2 / 2.0 - constants.GM_EARTH / r_mag

        # Convert through orbital elements
        elements = rv_to_coe(r, v, constants.GM_EARTH)
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Calculate new specific energy
        r_mag_new = np.linalg.norm(r_new)
        v_mag_new = np.linalg.norm(v_new)
        energy_new = v_mag_new**2 / 2.0 - constants.GM_EARTH / r_mag_new

        # Energy should be conserved to machine precision
        relative_error = abs(energy_new - energy_orig) / abs(energy_orig)
        assert relative_error < 1e-10, f"Energy not conserved: relative error = {relative_error}"

    def test_angular_momentum_conservation_curtis_example(self):
        """Verify specific angular momentum is conserved for Curtis example."""
        r = np.array([-6045.0e3, -3490.0e3, 2500.0e3])
        v = np.array([-3.457e3, 6.618e3, 2.533e3])

        # Calculate original angular momentum vector
        h_orig = np.cross(r, v)
        h_mag_orig = np.linalg.norm(h_orig)

        # Convert through orbital elements
        elements = rv_to_coe(r, v, constants.GM_EARTH)
        r_new, v_new = coe_to_rv(elements, constants.GM_EARTH)

        # Calculate new angular momentum
        h_new = np.cross(r_new, v_new)
        h_mag_new = np.linalg.norm(h_new)

        # Angular momentum magnitude should be conserved
        relative_error = abs(h_mag_new - h_mag_orig) / h_mag_orig
        assert (
            relative_error < 1e-10
        ), f"Angular momentum not conserved: relative error = {relative_error}"

        # Angular momentum direction should also be preserved
        h_orig_unit = h_orig / h_mag_orig
        h_new_unit = h_new / h_mag_new
        direction_error = np.linalg.norm(h_orig_unit - h_new_unit)

        assert (
            direction_error < 1e-10
        ), f"Angular momentum direction not conserved: error = {direction_error}"


class TestEdgeCasesWithKnownOrbits:
    """Test edge cases using real-world orbital examples."""

    def test_highly_eccentric_orbit(self):
        """
        Test conversion for highly eccentric orbit (e > 0.7).

        Comets and some transfer orbits have high eccentricities.
        This tests numerical stability for such cases.
        """
        # Create a highly eccentric orbit similar to a comet
        a = 20000e3  # meters
        e = 0.85  # High eccentricity
        i = np.deg2rad(45.0)

        elements = OrbitalElements(a=a, e=e, i=i, raan=0.0, argp=0.0, nu=0.0)

        # Convert to state vectors at periapsis
        r, v = coe_to_rv(elements, constants.GM_EARTH)

        # Convert back to orbital elements
        elements_new = rv_to_coe(r, v, constants.GM_EARTH)

        # Verify orbital parameters are preserved
        assert abs(elements_new.a - a) < 100.0, "Semi-major axis not preserved"
        assert abs(elements_new.e - e) < 0.001, "Eccentricity not preserved"
        assert abs(elements_new.i - i) < 1e-6, "Inclination not preserved"

    def test_near_polar_orbit(self):
        """
        Test conversion for near-polar orbit (i ≈ 90°).

        Polar orbits are common for Earth observation and should be
        handled correctly without singularities.
        """
        # Near-polar orbit at 800 km altitude
        altitude = 800e3
        a = constants.R_EARTH + altitude
        e = 0.001
        i = np.deg2rad(89.9)  # Nearly polar

        elements = OrbitalElements(a=a, e=e, i=i, raan=0.0, argp=0.0, nu=0.0)

        # Convert to state vectors
        r, v = coe_to_rv(elements, constants.GM_EARTH)

        # Convert back
        elements_new = rv_to_coe(r, v, constants.GM_EARTH)

        # Verify inclination is preserved (critical for polar orbits)
        assert (
            abs(elements_new.i - i) < 1e-6
        ), f"Polar orbit inclination not preserved: {np.rad2deg(elements_new.i)}°"


if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
