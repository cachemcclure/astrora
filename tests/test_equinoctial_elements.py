"""Test suite for Modified Equinoctial Orbital Elements

This module tests the implementation of modified equinoctial elements,
which provide a singularity-free representation for circular and
near-equatorial orbits.
"""

import numpy as np
import pytest
from astrora._core import (
    EquinoctialElements,
    OrbitalElements,
    coe_to_equinoctial,
    constants,
    equinoctial_to_coe,
    equinoctial_to_rv,
    rv_to_equinoctial,
)


class TestEquinoctialElementsBasic:
    """Test basic EquinoctialElements functionality"""

    def test_create_equinoctial_elements(self):
        """Test creating equinoctial elements"""
        eq = EquinoctialElements(
            p=7000e3,  # semi-latus rectum
            f=0.0,  # ecc x-component
            g=0.0,  # ecc y-component
            h=0.0,  # inc x-component
            k=0.0,  # inc y-component
            L=0.5,  # true longitude
        )

        assert eq.p == 7000e3
        assert eq.f == 0.0
        assert eq.g == 0.0
        assert eq.h == 0.0
        assert eq.k == 0.0
        assert eq.L == 0.5

    def test_equinoctial_properties(self):
        """Test derived properties of equinoctial elements"""
        # Create elements with known eccentricity: e = 0.1 = √(0.06² + 0.08²)
        eq = EquinoctialElements(p=7000e3, f=0.06, g=0.08, h=0.0, k=0.0, L=0.0)

        # Test eccentricity calculation
        assert np.isclose(eq.eccentricity_value, 0.1, rtol=1e-10)

        # Test semi-major axis calculation
        a = eq.semi_major_axis_value
        expected_a = 7000e3 / (1.0 - 0.1**2)
        assert np.isclose(a, expected_a, rtol=1e-10)

    def test_equinoctial_period(self):
        """Test orbital period calculation from equinoctial elements"""
        # Circular orbit at 7000 km
        eq = EquinoctialElements(p=7000e3, f=0.0, g=0.0, h=0.0, k=0.0, L=0.0)

        period = eq.orbital_period(constants.GM_EARTH)

        # Expected period ≈ 97 minutes
        expected_period = 97.0 * 60.0
        assert np.isclose(period, expected_period, atol=100.0)


class TestConversionKeplerianEquinoctial:
    """Test conversions between Keplerian and Equinoctial elements"""

    def test_coe_to_equinoctial_circular_equatorial(self):
        """Test conversion of circular equatorial orbit"""
        coe = OrbitalElements(a=7000e3, e=0.0, i=0.0, raan=0.0, argp=0.0, nu=0.0)
        eq = coe_to_equinoctial(coe)

        # For circular equatorial: f=g=h=k=0
        assert np.isclose(eq.f, 0.0, atol=1e-10)
        assert np.isclose(eq.g, 0.0, atol=1e-10)
        assert np.isclose(eq.h, 0.0, atol=1e-10)
        assert np.isclose(eq.k, 0.0, atol=1e-10)
        assert np.isclose(eq.p, 7000e3, rtol=1e-10)

    def test_coe_to_equinoctial_elliptical(self):
        """Test conversion of elliptical orbit"""
        coe = OrbitalElements(a=8000e3, e=0.1, i=0.0, raan=0.0, argp=0.0, nu=0.0)
        eq = coe_to_equinoctial(coe)

        # Check semi-latus rectum
        expected_p = 8000e3 * (1.0 - 0.1**2)
        assert np.isclose(eq.p, expected_p, rtol=1e-10)

        # Check eccentricity recovery
        e_recovered = np.sqrt(eq.f**2 + eq.g**2)
        assert np.isclose(e_recovered, 0.1, rtol=1e-10)

    def test_equinoctial_to_coe_circular(self):
        """Test conversion back to Keplerian elements"""
        eq = EquinoctialElements(p=7000e3, f=0.0, g=0.0, h=0.0, k=0.0, L=0.5)
        coe = eq.to_classical()

        assert np.isclose(coe.a, 7000e3, rtol=1e-10)
        assert coe.e < 1e-8
        assert coe.i < 1e-8

    def test_roundtrip_conversion(self):
        """Test COE → Equinoctial → COE roundtrip"""
        coe_orig = OrbitalElements(
            a=8000e3,
            e=0.1,
            i=np.pi / 4.0,  # 45°
            raan=0.5,
            argp=0.3,
            nu=0.7,
        )

        # Convert to equinoctial and back
        eq = coe_to_equinoctial(coe_orig)
        coe_new = equinoctial_to_coe(eq)

        assert np.isclose(coe_new.a, coe_orig.a, rtol=1e-10)
        assert np.isclose(coe_new.e, coe_orig.e, rtol=1e-10)
        assert np.isclose(coe_new.i, coe_orig.i, rtol=1e-10)
        assert np.isclose(coe_new.raan, coe_orig.raan, rtol=1e-8)
        assert np.isclose(coe_new.argp, coe_orig.argp, rtol=1e-8)
        assert np.isclose(coe_new.nu, coe_orig.nu, rtol=1e-8)


class TestConversionCartesianEquinoctial:
    """Test conversions between Cartesian and Equinoctial elements"""

    def test_rv_to_equinoctial_circular(self):
        """Test conversion from Cartesian to equinoctial"""
        r = np.array([7000e3, 0.0, 0.0])
        v_mag = np.sqrt(constants.GM_EARTH / 7000e3)
        v = np.array([0.0, v_mag, 0.0])

        eq = rv_to_equinoctial(r, v, constants.GM_EARTH)

        # Check eccentricity (should be very small)
        assert eq.eccentricity_value < 1e-8

        # Check semi-major axis
        a = eq.semi_major_axis_value
        assert np.isclose(a, 7000e3, rtol=1e-10)

    def test_equinoctial_to_rv_circular(self):
        """Test conversion from equinoctial to Cartesian"""
        eq = EquinoctialElements(p=7000e3, f=0.0, g=0.0, h=0.0, k=0.0, L=0.0)
        r, v = equinoctial_to_rv(eq, constants.GM_EARTH)

        # Check radius
        r_mag = np.linalg.norm(r)
        assert np.isclose(r_mag, 7000e3, rtol=1e-10)

        # Check velocity magnitude (circular)
        v_mag = np.linalg.norm(v)
        v_expected = np.sqrt(constants.GM_EARTH / 7000e3)
        assert np.isclose(v_mag, v_expected, rtol=1e-10)

    def test_roundtrip_rv_equinoctial(self):
        """Test rv → equinoctial → rv roundtrip"""
        r_orig = np.array([7000e3, 0.0, 0.0])
        v_orig = np.array([0.0, 8000.0, 0.0])

        # Convert to equinoctial and back
        eq = rv_to_equinoctial(r_orig, v_orig, constants.GM_EARTH)
        r_new, v_new = equinoctial_to_rv(eq, constants.GM_EARTH)

        assert np.allclose(r_new, r_orig, atol=10.0)
        assert np.allclose(v_new, v_orig, atol=1.0)


class TestSingularityFreeProperty:
    """Test that equinoctial elements are singularity-free"""

    def test_near_circular_orbit(self):
        """Test with extremely small eccentricity"""
        # Near-circular orbit (e = 1e-10)
        coe = OrbitalElements(
            a=7000e3,
            e=1e-10,
            i=np.pi / 4.0,
            raan=0.5,
            argp=0.0,
            nu=0.3,
        )

        # Convert to equinoctial (should not have issues)
        eq = coe_to_equinoctial(coe)

        # Convert back (should recover original values)
        coe_recovered = equinoctial_to_coe(eq)
        assert np.isclose(coe_recovered.e, 1e-10, rtol=1e-2, atol=1e-12)
        assert np.isclose(coe_recovered.a, 7000e3, rtol=1e-10)

    def test_near_equatorial_orbit(self):
        """Test with extremely small inclination"""
        # Near-equatorial orbit (i = 1e-10)
        coe = OrbitalElements(a=7000e3, e=0.1, i=1e-10, raan=0.0, argp=0.3, nu=0.5)

        # Convert to equinoctial (should not have issues)
        eq = coe_to_equinoctial(coe)

        # Convert back (should recover original values)
        coe_recovered = equinoctial_to_coe(eq)
        assert np.isclose(coe_recovered.e, 0.1, rtol=1e-10)
        assert np.isclose(coe_recovered.i, 1e-10, rtol=1e-2, atol=1e-12)

    def test_90_degree_inclination(self):
        """Test polar orbit (i = 90°) - should have no singularity"""
        coe = OrbitalElements(a=7000e3, e=0.1, i=np.pi / 2.0, raan=0.5, argp=0.3, nu=0.7)

        # Convert to equinoctial and back
        eq = coe_to_equinoctial(coe)
        coe_recovered = equinoctial_to_coe(eq)

        assert np.isclose(coe_recovered.i, np.pi / 2.0, rtol=1e-10)
        assert np.isclose(coe_recovered.e, 0.1, rtol=1e-10)


class TestEquinoctialClassMethods:
    """Test EquinoctialElements class methods"""

    def test_from_classical(self):
        """Test creating equinoctial from classical elements"""
        coe = OrbitalElements(a=8000e3, e=0.1, i=np.pi / 6.0, raan=0.5, argp=0.3, nu=0.7)

        eq = EquinoctialElements.from_classical(coe)

        # Verify it's an EquinoctialElements instance
        assert isinstance(eq, EquinoctialElements)

        # Verify conversion is correct
        assert np.isclose(eq.eccentricity_value, 0.1, rtol=1e-10)

    def test_string_representation(self):
        """Test __repr__ and __str__ methods"""
        eq = EquinoctialElements(p=7000e3, f=0.06, g=0.08, h=0.01, k=0.02, L=0.5)

        # Test __repr__
        repr_str = repr(eq)
        assert "EquinoctialElements" in repr_str
        assert "p=" in repr_str

        # Test __str__
        str_str = str(eq)
        assert "p =" in str_str
        assert "f =" in str_str
        assert "derived:" in str_str  # Should show derived e and i


if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v"])
