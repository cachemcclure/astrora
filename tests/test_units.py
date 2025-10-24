"""
Tests for astropy.units integration in Orbit class.

This test suite verifies that:
1. Orbit class accepts both raw values and Quantity objects
2. Properties return Quantity objects with appropriate units
3. Unit conversions work correctly
4. Backward compatibility with raw values is maintained
"""

import numpy as np
import pytest
from astropy import units as u
from astrora.bodies import Earth
from astrora.twobody import Orbit


class TestOrbitCreationWithUnits:
    """Test orbit creation methods with astropy units."""

    def test_from_vectors_with_units(self):
        """Test from_vectors with Quantity objects."""
        # Create orbit using units (poliastro-style)
        r = [7000, 0, 0] << u.km
        v = [0, 7.546, 0] << u.km / u.s

        orbit = Orbit.from_vectors(Earth, r, v)

        # Verify orbit was created successfully
        assert orbit is not None
        assert orbit.attractor == Earth

        # Check that position and velocity are close to expected (in SI units internally)
        r_result = orbit.r.to(u.km).value
        np.testing.assert_allclose(r_result, [7000, 0, 0], rtol=1e-10)

    def test_from_vectors_with_raw_arrays(self):
        """Test from_vectors with raw arrays (backward compatibility)."""
        # Create orbit using raw SI units
        r = np.array([7000e3, 0, 0])  # meters
        v = np.array([0, 7546, 0])  # m/s

        orbit = Orbit.from_vectors(Earth, r, v)

        # Verify orbit was created successfully
        assert orbit is not None

        # Check values
        r_result = orbit.r.to(u.m).value
        np.testing.assert_allclose(r_result, [7000e3, 0, 0], rtol=1e-10)

    def test_from_vectors_unit_conversion(self):
        """Test that different input units convert correctly."""
        # Create with AU and km/s
        r = [1, 0, 0] << u.AU
        v = [0, 30, 0] << u.km / u.s

        orbit = Orbit.from_vectors(Earth, r, v)

        # Convert back to AU for verification
        r_au = orbit.r.to(u.AU).value
        np.testing.assert_allclose(r_au[0], 1.0, rtol=1e-10)

    def test_from_classical_with_units(self):
        """Test from_classical with Quantity objects."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000 << u.km,
            ecc=0.01 << u.one,
            inc=51.6 << u.deg,
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        assert orbit is not None

        # Check semi-major axis
        a_km = orbit.a.to(u.km).value
        np.testing.assert_allclose(a_km, 7000, rtol=1e-6)

        # Check inclination
        inc_deg = orbit.inc.to(u.deg).value
        np.testing.assert_allclose(inc_deg, 51.6, rtol=1e-6)

    def test_from_classical_with_raw_values(self):
        """Test from_classical with raw values (backward compatibility)."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000e3,  # meters
            ecc=0.01,  # dimensionless
            inc=np.deg2rad(51.6),  # radians
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        assert orbit is not None

        # Check that values match
        a_m = orbit.a.to(u.m).value
        np.testing.assert_allclose(a_m, 7000e3, rtol=1e-6)

    def test_mixed_units_in_from_classical(self):
        """Test from_classical with mixed unit types."""
        orbit = Orbit.from_classical(
            Earth,
            a=1.0 << u.AU,  # Astronomical units
            ecc=0.0167 << u.one,  # Dimensionless
            inc=7.25 << u.deg,  # Degrees
            raan=348.7 << u.deg,
            argp=114.2 << u.deg,
            nu=0 << u.rad,  # Radians
        )

        assert orbit is not None

        # Verify AU is preserved
        a_au = orbit.a.to(u.AU).value
        np.testing.assert_allclose(a_au, 1.0, rtol=1e-6)


class TestOrbitPropertiesWithUnits:
    """Test that orbit properties return Quantity objects."""

    @pytest.fixture
    def circular_orbit(self):
        """Create a circular LEO orbit for testing."""
        r = [7000, 0, 0] << u.km
        v = [0, 7.546, 0] << u.km / u.s
        return Orbit.from_vectors(Earth, r, v)

    def test_position_returns_quantity(self, circular_orbit):
        """Test that r property returns Quantity."""
        r = circular_orbit.r

        # Check it's a Quantity
        assert isinstance(r, u.Quantity)

        # Check it has length units
        assert r.unit.physical_type == "length"

        # Can convert to different units
        r_km = r.to(u.km)
        assert r_km.value[0] == pytest.approx(7000, rel=1e-6)

    def test_velocity_returns_quantity(self, circular_orbit):
        """Test that v property returns Quantity."""
        v = circular_orbit.v

        # Check it's a Quantity
        assert isinstance(v, u.Quantity)

        # Check it has velocity units
        assert v.unit.physical_type == "speed"

        # Can convert to different units
        v_kms = v.to(u.km / u.s)
        assert v_kms.value[1] == pytest.approx(7.546, rel=1e-3)

    def test_semi_major_axis_returns_quantity(self, circular_orbit):
        """Test that a property returns Quantity."""
        a = circular_orbit.a

        assert isinstance(a, u.Quantity)
        assert a.unit.physical_type == "length"

        # Check value
        a_km = a.to(u.km).value
        assert a_km == pytest.approx(7000, rel=1e-4)

    def test_eccentricity_returns_quantity(self, circular_orbit):
        """Test that ecc property returns dimensionless Quantity."""
        ecc = circular_orbit.ecc

        assert isinstance(ecc, u.Quantity)
        assert ecc.unit.physical_type == "dimensionless"

        # Should be nearly zero for circular orbit
        assert ecc.value == pytest.approx(0.0, abs=1e-3)

    def test_inclination_returns_quantity(self, circular_orbit):
        """Test that inc property returns Quantity with angle units."""
        inc = circular_orbit.inc

        assert isinstance(inc, u.Quantity)
        assert inc.unit.physical_type == "angle"

        # Can convert to degrees
        inc_deg = inc.to(u.deg)
        assert isinstance(inc_deg, u.Quantity)

    def test_angles_return_quantities(self, circular_orbit):
        """Test that all angle properties return Quantities."""
        angles = [circular_orbit.raan, circular_orbit.argp, circular_orbit.nu]

        for angle in angles:
            assert isinstance(angle, u.Quantity)
            assert angle.unit.physical_type == "angle"

            # Can convert to degrees
            angle_deg = angle.to(u.deg)
            assert isinstance(angle_deg, u.Quantity)

    def test_period_returns_quantity(self, circular_orbit):
        """Test that period property returns Quantity with time units."""
        period = circular_orbit.period

        assert isinstance(period, u.Quantity)
        assert period.unit.physical_type == "time"

        # Can convert to hours
        period_hr = period.to(u.hour)
        # Period for 7000 km orbit is ~1.62 hours
        assert period_hr.value == pytest.approx(1.62, rel=0.01)

    def test_mean_motion_returns_quantity(self, circular_orbit):
        """Test that n property returns Quantity."""
        n = circular_orbit.n

        assert isinstance(n, u.Quantity)

        # Should have angular velocity units
        # Can convert to deg/min
        n_deg_min = n.to(u.deg / u.min)
        assert isinstance(n_deg_min, u.Quantity)

    def test_energy_returns_quantity(self, circular_orbit):
        """Test that energy property returns Quantity."""
        energy = circular_orbit.energy

        assert isinstance(energy, u.Quantity)

        # Should be negative for bound orbit
        assert energy.value < 0

    def test_periapsis_apoapsis_return_quantities(self, circular_orbit):
        """Test that r_p and r_a return Quantities."""
        r_p = circular_orbit.r_p
        r_a = circular_orbit.r_a

        assert isinstance(r_p, u.Quantity)
        assert isinstance(r_a, u.Quantity)

        assert r_p.unit.physical_type == "length"
        assert r_a.unit.physical_type == "length"

        # For circular orbit, r_p ≈ r_a ≈ a
        r_p_km = r_p.to(u.km).value
        r_a_km = r_a.to(u.km).value
        assert r_p_km == pytest.approx(r_a_km, rel=0.01)


class TestUnitConversions:
    """Test various unit conversions."""

    def test_position_multiple_conversions(self):
        """Test converting position to various units."""
        r = [1, 0, 0] << u.AU
        v = [0, 30, 0] << u.km / u.s

        orbit = Orbit.from_vectors(Earth, r, v)

        # Convert to different units
        r_m = orbit.r.to(u.m)
        r_km = orbit.r.to(u.km)
        r_au = orbit.r.to(u.AU)

        # Check consistency
        assert r_au.value[0] == pytest.approx(1.0, rel=1e-6)
        assert r_km.value[0] == pytest.approx(1.496e8, rel=1e-3)

    def test_angle_degree_radian_conversion(self):
        """Test angle conversions between degrees and radians."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000 << u.km,
            ecc=0.0 << u.one,
            inc=90 << u.deg,  # Polar orbit
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        inc_deg = orbit.inc.to(u.deg).value
        inc_rad = orbit.inc.to(u.rad).value

        assert inc_deg == pytest.approx(90, rel=1e-6)
        assert inc_rad == pytest.approx(np.pi / 2, rel=1e-6)

    def test_period_time_unit_conversions(self):
        """Test period conversions to various time units."""
        r = [7000, 0, 0] << u.km
        v = [0, 7.546, 0] << u.km / u.s

        orbit = Orbit.from_vectors(Earth, r, v)

        period_s = orbit.period.to(u.s)
        period_min = orbit.period.to(u.min)
        period_hr = orbit.period.to(u.hour)

        # Check consistency
        assert period_min.value == pytest.approx(period_s.value / 60, rel=1e-6)
        assert period_hr.value == pytest.approx(period_s.value / 3600, rel=1e-6)


class TestUnitValidation:
    """Test that invalid units are rejected."""

    def test_incompatible_position_units_rejected(self):
        """Test that incompatible position units raise an error."""
        r = [1, 0, 0] << u.kg  # Wrong unit type!
        v = [0, 7.546, 0] << u.km / u.s

        with pytest.raises(ValueError, match="Cannot convert"):
            Orbit.from_vectors(Earth, r, v)

    def test_incompatible_velocity_units_rejected(self):
        """Test that incompatible velocity units raise an error."""
        r = [7000, 0, 0] << u.km
        v = [0, 7.546, 0] << u.km  # Wrong! Should be km/s

        with pytest.raises(ValueError, match="Cannot convert"):
            Orbit.from_vectors(Earth, r, v)

    def test_incompatible_angle_units_rejected(self):
        """Test that non-angle units for angles are rejected."""
        with pytest.raises(ValueError, match="Cannot convert"):
            Orbit.from_classical(
                Earth,
                a=7000 << u.km,
                ecc=0.0 << u.one,
                inc=90 << u.km,  # Wrong! Should be angle
                raan=0 << u.deg,
                argp=0 << u.deg,
                nu=0 << u.deg,
            )

    def test_dimensional_eccentricity_rejected(self):
        """Test that eccentricity with dimensions is rejected."""
        with pytest.raises(ValueError, match="dimensionless"):
            Orbit.from_classical(
                Earth,
                a=7000 << u.km,
                ecc=0.0 << u.km,  # Wrong! Should be dimensionless
                inc=0 << u.deg,
                raan=0 << u.deg,
                argp=0 << u.deg,
                nu=0 << u.deg,
            )


class TestBackwardCompatibility:
    """Test that existing code without units still works."""

    def test_raw_arrays_still_work(self):
        """Test that raw NumPy arrays work as before."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])

        orbit = Orbit.from_vectors(Earth, r, v)

        assert orbit is not None
        assert isinstance(orbit.r, u.Quantity)  # But properties return Quantities

    def test_raw_floats_still_work(self):
        """Test that raw floats work in from_classical."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000e3,
            ecc=0.01,
            inc=0.9,
            raan=0.0,
            argp=0.0,
            nu=0.0,
        )

        assert orbit is not None

    def test_mixed_raw_and_quantity(self):
        """Test that mixing raw values and Quantities works."""
        # This should work: some with units, some without
        orbit = Orbit.from_classical(
            Earth,
            a=7000 << u.km,  # With units
            ecc=0.01,  # Raw
            inc=51.6 << u.deg,  # With units
            raan=0.0,  # Raw (radians)
            argp=0.0,
            nu=0.0,
        )

        assert orbit is not None
        a_km = orbit.a.to(u.km).value
        assert a_km == pytest.approx(7000, rel=1e-6)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
