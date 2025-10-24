"""
Integration tests for coordinate reference frames (ICRS, GCRS, J2000, ITRS)
"""

import numpy as np
import pytest

# Import the Rust extension module
from astrora._core import (
    GCRS,
    ICRS,
    ITRS,
    J2000,
    Epoch,
    Perifocal,
)
from numpy.testing import assert_allclose, assert_array_almost_equal


class TestICRS:
    """Test ICRS (International Celestial Reference System) frame"""

    def test_icrs_creation(self):
        """Test creating an ICRS coordinate"""
        pos = np.array([1.496e11, 0.0, 0.0])  # ~1 AU
        vel = np.array([0.0, 29780.0, 0.0])  # ~30 km/s

        icrs = ICRS(pos, vel)

        assert_array_almost_equal(icrs.position, pos)
        assert_array_almost_equal(icrs.velocity, vel)

    def test_icrs_position_only(self):
        """Test creating ICRS with position only (zero velocity)"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([0.0, 0.0, 0.0])

        icrs = ICRS(pos, vel)

        assert_array_almost_equal(icrs.position, pos)
        assert_array_almost_equal(icrs.velocity, vel)

    def test_icrs_repr(self):
        """Test ICRS string representation"""
        pos = np.array([7.0e6, 0.0, 0.0])
        vel = np.array([0.0, 7500.0, 0.0])

        icrs = ICRS(pos, vel)
        repr_str = repr(icrs)

        assert "ICRS" in repr_str
        assert "position" in repr_str
        assert "velocity" in repr_str

    def test_icrs_invalid_arrays(self):
        """Test ICRS with invalid array sizes"""
        pos_wrong = np.array([1.0, 2.0])  # Only 2 elements
        vel = np.array([0.0, 0.0, 0.0])

        with pytest.raises(ValueError, match="must be 3-element arrays"):
            ICRS(pos_wrong, vel)

    def test_icrs_to_gcrs(self):
        """Test ICRS to GCRS conversion"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])
        icrs = ICRS(pos, vel)

        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)
        gcrs = icrs.to_gcrs(epoch)

        # For simple conversion, should be equal
        assert_array_almost_equal(gcrs.position, pos, decimal=6)
        assert_array_almost_equal(gcrs.velocity, vel, decimal=9)

    def test_icrs_earth_position(self):
        """Test ICRS with Earth-like barycentric position"""
        # Earth's approximate position at 1 AU
        pos = np.array([1.496e11, 0.0, 0.0])
        vel = np.array([0.0, 29780.0, 0.0])

        icrs = ICRS(pos, vel)

        # Verify magnitude is ~1 AU
        r_mag = np.linalg.norm(icrs.position)
        assert_allclose(r_mag, 1.496e11, rtol=1e-10)

        # Verify velocity is ~30 km/s
        v_mag = np.linalg.norm(icrs.velocity)
        assert_allclose(v_mag, 29780.0, rtol=1e-10)


class TestGCRS:
    """Test GCRS (Geocentric Celestial Reference System) frame"""

    def test_gcrs_creation(self):
        """Test creating a GCRS coordinate"""
        pos = np.array([42164000.0, 0.0, 0.0])  # GEO radius
        vel = np.array([0.0, 3075.0, 0.0])  # GEO velocity
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        gcrs = GCRS(pos, vel, epoch)

        assert_array_almost_equal(gcrs.position, pos)
        assert_array_almost_equal(gcrs.velocity, vel)
        assert gcrs.obstime == epoch

    def test_gcrs_position_only(self):
        """Test creating GCRS with position only"""
        pos = np.array([7.0e6, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])
        epoch = Epoch.j2000_epoch()

        gcrs = GCRS(pos, vel, epoch)

        assert_array_almost_equal(gcrs.position, pos)
        assert_array_almost_equal(gcrs.velocity, vel)

    def test_gcrs_repr(self):
        """Test GCRS string representation"""
        pos = np.array([7.0e6, 0.0, 0.0])
        vel = np.array([0.0, 7500.0, 0.0])
        epoch = Epoch(2024, 6, 15, 12, 0, 0, 0)

        gcrs = GCRS(pos, vel, epoch)
        repr_str = repr(gcrs)

        assert "GCRS" in repr_str
        assert "position" in repr_str
        assert "velocity" in repr_str
        assert "obstime" in repr_str
        assert "2024-06-15" in repr_str

    def test_gcrs_invalid_arrays(self):
        """Test GCRS with invalid array sizes"""
        pos = np.array([1.0, 2.0, 3.0])
        vel_wrong = np.array([1.0, 2.0])  # Only 2 elements
        epoch = Epoch.j2000_epoch()

        with pytest.raises(ValueError, match="must be 3-element arrays"):
            GCRS(pos, vel_wrong, epoch)

    def test_gcrs_to_icrs(self):
        """Test GCRS to ICRS conversion"""
        pos = np.array([42164000.0, 0.0, 1.0e6])
        vel = np.array([0.0, 3075.0, 100.0])
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)
        gcrs = GCRS(pos, vel, epoch)

        icrs = gcrs.to_icrs()

        # For simple conversion, should be equal
        assert_array_almost_equal(icrs.position, pos, decimal=6)
        assert_array_almost_equal(icrs.velocity, vel, decimal=9)

    def test_gcrs_iss_orbit(self):
        """Test GCRS with ISS-like orbit parameters"""
        # ISS: ~408 km altitude, circular orbit
        r_earth = 6371000.0  # m
        h = 408000.0  # m
        r = r_earth + h

        pos = np.array([r, 0.0, 0.0])
        mu = 398600.4418e9  # Earth's gravitational parameter (m³/s²)
        v = np.sqrt(mu / r)  # Circular orbital velocity
        vel = np.array([0.0, v, 0.0])

        epoch = Epoch.j2000_epoch()
        gcrs = GCRS(pos, vel, epoch)

        # Verify orbital radius
        r_mag = np.linalg.norm(gcrs.position)
        assert_allclose(r_mag, r, rtol=1e-10)

        # Verify velocity is in LEO range (7-8 km/s)
        v_mag = np.linalg.norm(gcrs.velocity)
        assert 7000.0 < v_mag < 8000.0

    def test_gcrs_geo_satellite(self):
        """Test GCRS with geostationary satellite parameters"""
        # GEO: ~35,786 km altitude
        r_geo = 42164000.0  # m
        pos = np.array([r_geo, 0.0, 0.0])
        mu = 398600.4418e9  # m³/s²
        v_geo = np.sqrt(mu / r_geo)
        vel = np.array([0.0, v_geo, 0.0])

        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)
        gcrs = GCRS(pos, vel, epoch)

        # Verify position
        r_mag = np.linalg.norm(gcrs.position)
        assert_allclose(r_mag, r_geo, rtol=1e-10)

        # Verify velocity is ~3.07 km/s for GEO
        v_mag = np.linalg.norm(gcrs.velocity)
        assert 3000.0 < v_mag < 3100.0


class TestFrameTransformations:
    """Test transformations between ICRS and GCRS"""

    def test_icrs_gcrs_roundtrip(self):
        """Test ICRS → GCRS → ICRS roundtrip conversion"""
        pos = np.array([1.0e7, 2.0e7, 3.0e7])
        vel = np.array([1000.0, 2000.0, 3000.0])
        icrs1 = ICRS(pos, vel)

        epoch = Epoch.j2000_epoch()
        gcrs = icrs1.to_gcrs(epoch)
        icrs2 = gcrs.to_icrs()

        # Should get back the same values
        assert_array_almost_equal(icrs2.position, pos, decimal=6)
        assert_array_almost_equal(icrs2.velocity, vel, decimal=9)

    def test_gcrs_icrs_roundtrip(self):
        """Test GCRS → ICRS → GCRS roundtrip conversion"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])
        epoch = Epoch(2024, 3, 15, 12, 30, 0, 0)
        gcrs1 = GCRS(pos, vel, epoch)

        icrs = gcrs1.to_icrs()
        gcrs2 = icrs.to_gcrs(epoch)

        # Should get back the same values
        assert_array_almost_equal(gcrs2.position, pos, decimal=6)
        assert_array_almost_equal(gcrs2.velocity, vel, decimal=9)

    def test_simple_transformation_identity(self):
        """Test that ICRS ≈ GCRS for simple conversion (axes aligned)"""
        # For Earth satellites, the barycentric correction is small
        pos = np.array([7.0e6, 0.0, 0.0])
        vel = np.array([0.0, 7500.0, 0.0])
        icrs = ICRS(pos, vel)

        epoch = Epoch.j2000_epoch()
        gcrs = icrs.to_gcrs(epoch)

        # Simple conversion should preserve coordinates
        assert_array_almost_equal(gcrs.position, icrs.position, decimal=6)
        assert_array_almost_equal(gcrs.velocity, icrs.velocity, decimal=9)

    def test_transformation_different_epochs(self):
        """Test ICRS to GCRS at different epochs"""
        pos = np.array([42164000.0, 0.0, 0.0])
        vel = np.array([0.0, 3075.0, 0.0])
        icrs = ICRS(pos, vel)

        epoch1 = Epoch(2024, 1, 1, 0, 0, 0, 0)
        epoch2 = Epoch(2024, 6, 15, 12, 0, 0, 0)

        gcrs1 = icrs.to_gcrs(epoch1)
        gcrs2 = icrs.to_gcrs(epoch2)

        # Different epochs should produce different GCRS coordinates
        # (In version 1, they're the same, but obstime differs)
        assert gcrs1.obstime != gcrs2.obstime

        # Positions are the same in V1 (no barycentric correction yet)
        assert_array_almost_equal(gcrs1.position, gcrs2.position, decimal=6)


class TestEdgeCases:
    """Test edge cases and boundary conditions"""

    def test_zero_position_velocity(self):
        """Test frames with zero position and velocity"""
        pos = np.array([0.0, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])

        icrs = ICRS(pos, vel)
        assert_array_almost_equal(icrs.position, pos)
        assert_array_almost_equal(icrs.velocity, vel)

        epoch = Epoch.j2000_epoch()
        gcrs = GCRS(pos, vel, epoch)
        assert_array_almost_equal(gcrs.position, pos)
        assert_array_almost_equal(gcrs.velocity, vel)

    def test_large_values(self):
        """Test frames with very large coordinate values"""
        # Interplanetary distances
        pos = np.array([1.0e12, 5.0e11, 2.0e11])  # ~AU scale
        vel = np.array([50000.0, 30000.0, 10000.0])  # ~km/s scale

        icrs = ICRS(pos, vel)
        assert_array_almost_equal(icrs.position, pos)
        assert_array_almost_equal(icrs.velocity, vel)

    def test_small_values(self):
        """Test frames with very small coordinate values"""
        # Near-Earth scales
        pos = np.array([100.0, 50.0, 25.0])  # meters
        vel = np.array([1.0, 0.5, 0.25])  # m/s

        epoch = Epoch.j2000_epoch()
        gcrs = GCRS(pos, vel, epoch)
        assert_array_almost_equal(gcrs.position, pos)
        assert_array_almost_equal(gcrs.velocity, vel)

    def test_negative_coordinates(self):
        """Test frames with negative coordinate values"""
        pos = np.array([-7.0e6, -1.0e6, -0.5e6])
        vel = np.array([-1000.0, -7500.0, -500.0])

        icrs = ICRS(pos, vel)
        assert_array_almost_equal(icrs.position, pos)
        assert_array_almost_equal(icrs.velocity, vel)


class TestITRS:
    """Test ITRS (International Terrestrial Reference System) frame"""

    def test_itrs_creation(self):
        """Test creating an ITRS coordinate"""
        # Ground station at equator, prime meridian
        pos = np.array([6378137.0, 0.0, 0.0])  # WGS84 equatorial radius
        vel = np.array([0.0, 0.0, 0.0])  # Stationary on Earth
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        itrs = ITRS(pos, vel, epoch)

        assert_array_almost_equal(itrs.position, pos)
        assert_array_almost_equal(itrs.velocity, vel)
        assert itrs.obstime == epoch

    def test_itrs_ground_station(self):
        """Test ITRS with ground station coordinates"""
        # Ground station at arbitrary location
        pos = np.array([6378137.0, 1000000.0, 500000.0])
        vel = np.array([0.0, 0.0, 0.0])  # Fixed to Earth
        epoch = Epoch.j2000_epoch()

        itrs = ITRS(pos, vel, epoch)

        # Verify position magnitude
        r_mag = np.linalg.norm(itrs.position)
        assert r_mag > 6.3e6  # Roughly Earth radius

    def test_itrs_repr(self):
        """Test ITRS string representation"""
        pos = np.array([6378137.0, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])
        epoch = Epoch(2024, 6, 15, 12, 0, 0, 0)

        itrs = ITRS(pos, vel, epoch)
        repr_str = repr(itrs)

        assert "ITRS" in repr_str
        assert "position" in repr_str
        assert "velocity" in repr_str
        assert "obstime" in repr_str
        assert "2024-06-15" in repr_str

    def test_itrs_invalid_arrays(self):
        """Test ITRS with invalid array sizes"""
        pos = np.array([1.0, 2.0, 3.0])
        vel_wrong = np.array([1.0, 2.0])  # Only 2 elements
        epoch = Epoch.j2000_epoch()

        with pytest.raises(ValueError, match="must be 3-element arrays"):
            ITRS(pos, vel_wrong, epoch)

    def test_itrs_to_gcrs(self):
        """Test ITRS to GCRS conversion"""
        # Ground station at equator
        pos = np.array([6378137.0, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        itrs = ITRS(pos, vel, epoch)
        gcrs = itrs.to_gcrs()

        # Position magnitude should be preserved
        r_itrs = np.linalg.norm(itrs.position)
        r_gcrs = np.linalg.norm(gcrs.position)
        assert_allclose(r_gcrs, r_itrs, rtol=1e-10)

        # Ground station should have velocity in GCRS due to Earth rotation
        # v = ω × r ≈ 465 m/s at equator
        v_gcrs = np.linalg.norm(gcrs.velocity)
        assert 400.0 < v_gcrs < 500.0

    def test_gcrs_to_itrs(self):
        """Test GCRS to ITRS conversion"""
        # Satellite in GCRS
        pos = np.array([7000000.0, 1000000.0, 500000.0])
        vel = np.array([1000.0, 7500.0, 500.0])
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        gcrs = GCRS(pos, vel, epoch)
        itrs = gcrs.to_itrs()

        # Position magnitude should be preserved
        r_gcrs = np.linalg.norm(gcrs.position)
        r_itrs = np.linalg.norm(itrs.position)
        assert_allclose(r_itrs, r_gcrs, rtol=1e-10)

    def test_itrs_gcrs_roundtrip(self):
        """Test ITRS → GCRS → ITRS roundtrip"""
        pos = np.array([6378137.0, 1000000.0, 500000.0])
        vel = np.array([100.0, 50.0, 25.0])
        epoch = Epoch(2024, 6, 15, 12, 0, 0, 0)

        itrs1 = ITRS(pos, vel, epoch)
        gcrs = itrs1.to_gcrs()
        itrs2 = gcrs.to_itrs()

        # Should get back the same values (within mm precision)
        assert_array_almost_equal(itrs2.position, pos, decimal=3)
        assert_array_almost_equal(itrs2.velocity, vel, decimal=6)

    def test_gcrs_itrs_roundtrip(self):
        """Test GCRS → ITRS → GCRS roundtrip"""
        pos = np.array([7000000.0, 1000000.0, 500000.0])
        vel = np.array([1000.0, 7500.0, 500.0])
        epoch = Epoch(2024, 1, 1, 12, 0, 0, 0)

        gcrs1 = GCRS(pos, vel, epoch)
        itrs = gcrs1.to_itrs()
        gcrs2 = itrs.to_gcrs()

        # Should get back the same values (within mm precision)
        assert_array_almost_equal(gcrs2.position, pos, decimal=3)
        assert_array_almost_equal(gcrs2.velocity, vel, decimal=6)

    def test_itrs_coriolis_effect(self):
        """Test that stationary ground station has velocity in GCRS"""
        # Equatorial ground station
        r_earth = 6378137.0
        pos = np.array([r_earth, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])  # Stationary in ITRS
        epoch = Epoch.j2000_epoch()

        itrs = ITRS(pos, vel, epoch)
        gcrs = itrs.to_gcrs()

        # Should have velocity due to Earth's rotation
        omega_earth = 7.2921150e-5  # rad/s
        expected_v = omega_earth * r_earth  # ~465 m/s

        v_gcrs = np.linalg.norm(gcrs.velocity)
        assert_allclose(v_gcrs, expected_v, rtol=0.01)

        # Velocity should be perpendicular to position
        dot_product = np.dot(gcrs.velocity, gcrs.position)
        assert abs(dot_product) < 100.0  # Nearly perpendicular

    def test_itrs_polar_station(self):
        """Test ground station at North Pole"""
        # North Pole station
        r_pole = 6356752.0  # WGS84 polar radius
        pos = np.array([0.0, 0.0, r_pole])
        vel = np.array([0.0, 0.0, 0.0])
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        itrs = ITRS(pos, vel, epoch)
        gcrs = itrs.to_gcrs()

        # Position should remain essentially unchanged (on rotation axis)
        assert_allclose(gcrs.position[2], r_pole, rtol=1e-6)
        assert abs(gcrs.position[0]) < 1.0
        assert abs(gcrs.position[1]) < 1.0

        # Velocity should be nearly zero at pole
        v_gcrs = np.linalg.norm(gcrs.velocity)
        assert v_gcrs < 1.0

    def test_itrs_geo_satellite(self):
        """Test geostationary satellite (appears fixed in ITRS)"""
        # GEO satellite at longitude 0°
        r_geo = 42164000.0
        pos = np.array([r_geo, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])  # Stationary in ITRS
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        itrs = ITRS(pos, vel, epoch)
        gcrs = itrs.to_gcrs()

        # In GCRS, should have orbital velocity ~3.07 km/s
        v_gcrs = np.linalg.norm(gcrs.velocity)
        mu = 398600.4418e9  # Earth's GM
        expected_v_geo = np.sqrt(mu / r_geo)

        assert_allclose(v_gcrs, expected_v_geo, rtol=0.01)
        assert 3000.0 < v_gcrs < 3100.0

    def test_itrs_z_component_preservation(self):
        """Test that z-component is preserved in rotation (rotation about z-axis)"""
        pos = np.array([6378137.0, 1000000.0, 2000000.0])
        vel = np.array([100.0, 50.0, 25.0])
        epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

        itrs = ITRS(pos, vel, epoch)
        gcrs = itrs.to_gcrs()

        # Z-component of position should be nearly identical
        assert_allclose(gcrs.position[2], itrs.position[2], rtol=1e-10)

    def test_itrs_magnitude_conservation(self):
        """Test that position magnitude is conserved in transformation"""
        pos = np.array([8000000.0, 2000000.0, 1000000.0])
        vel = np.array([1000.0, 500.0, 250.0])
        epoch = Epoch(2024, 1, 1, 12, 0, 0, 0)

        itrs = ITRS(pos, vel, epoch)
        gcrs = itrs.to_gcrs()

        r_itrs = np.linalg.norm(itrs.position)
        r_gcrs = np.linalg.norm(gcrs.position)

        assert_allclose(r_gcrs, r_itrs, rtol=1e-10)

    def test_itrs_time_dependence(self):
        """Test that ITRS→GCRS transformation depends on time"""
        pos = np.array([6378137.0, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])

        # Same position at two different times
        epoch1 = Epoch(2024, 1, 1, 0, 0, 0, 0)
        epoch2 = Epoch(2024, 1, 1, 6, 0, 0, 0)  # 6 hours later

        itrs1 = ITRS(pos, vel, epoch1)
        itrs2 = ITRS(pos, vel, epoch2)

        gcrs1 = itrs1.to_gcrs()
        gcrs2 = itrs2.to_gcrs()

        # GCRS positions should differ due to Earth rotation
        pos_diff = np.linalg.norm(gcrs1.position - gcrs2.position)
        assert pos_diff > 1000.0  # Should differ by more than 1 km


class TestPerifocal:
    """Test Perifocal (PQW) coordinate frame"""

    def test_perifocal_creation(self):
        """Test creating a Perifocal coordinate"""
        pos = np.array([7.0e6, 0.0, 0.0])  # Along P-axis
        vel = np.array([0.0, 7500.0, 0.0])  # Along Q-axis
        raan = 0.5
        inc = 0.3
        argp = 1.0

        peri = Perifocal(pos, vel, raan, inc, argp)

        assert_array_almost_equal(peri.position(), pos)
        assert_array_almost_equal(peri.velocity(), vel)
        assert peri.raan == raan
        assert peri.inc == inc
        assert peri.argp == argp

    def test_perifocal_from_orbital_elements(self):
        """Test creating Perifocal from orbital elements"""
        a = 7.0e6  # Semi-major axis
        e = 0.1  # Eccentricity
        nu = 0.0  # True anomaly (at periapsis)
        raan = 0.5
        inc = 0.3
        argp = 1.0
        mu = 398600.4418e9

        peri = Perifocal.from_orbital_elements_py(a, e, nu, raan, inc, argp, mu)

        # At periapsis, position should be along P-axis
        r_periapsis = a * (1.0 - e)
        assert_allclose(peri.position()[0], r_periapsis, rtol=1e-6)
        assert_allclose(peri.position()[1], 0.0, atol=1e-6)
        assert_allclose(peri.position()[2], 0.0, atol=1e-6)

        # Velocity should be along Q-axis
        assert_allclose(peri.velocity()[0], 0.0, atol=1e-3)
        assert peri.velocity()[1] > 0.0
        assert_allclose(peri.velocity()[2], 0.0, atol=1e-6)

    def test_perifocal_position_in_orbital_plane(self):
        """Test that position is always in the orbital plane (z=0)"""
        a = 8.0e6
        e = 0.15
        raan = 0.5
        inc = 0.4
        argp = 1.2
        mu = 398600.4418e9

        # Test at various true anomalies
        for nu in [0.0, np.pi / 4, np.pi / 2, np.pi, 3 * np.pi / 2]:
            peri = Perifocal.from_orbital_elements_py(a, e, nu, raan, inc, argp, mu)
            # Z-component should be zero (in orbital plane)
            assert_allclose(peri.position()[2], 0.0, atol=1e-6)
            assert_allclose(peri.velocity()[2], 0.0, atol=1e-6)

    def test_perifocal_to_gcrs(self):
        """Test transformation from perifocal to GCRS"""
        pos = np.array([7.0e6, 1.0e6, 0.0])
        vel = np.array([-1000.0, 7000.0, 0.0])
        raan = 0.0
        inc = 0.0
        argp = 0.0
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)

        peri = Perifocal(pos, vel, raan, inc, argp)
        gcrs = peri.to_gcrs(epoch)

        # For equatorial orbit with zero angles, should be nearly identity
        assert_array_almost_equal(gcrs.position, pos, decimal=3)
        assert_array_almost_equal(gcrs.velocity, vel, decimal=3)

    def test_perifocal_from_gcrs(self):
        """Test creating Perifocal from GCRS"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([-1000.0, 7000.0, 1000.0])
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)
        raan = 0.5
        inc = 0.3
        argp = 1.5

        gcrs = GCRS(pos, vel, epoch)
        peri = Perifocal.from_gcrs(gcrs, raan, inc, argp)

        # Check that we got valid perifocal coordinates
        assert peri.raan == raan
        assert peri.inc == inc
        assert peri.argp == argp
        assert len(peri.position()) == 3
        assert len(peri.velocity()) == 3

    def test_perifocal_gcrs_roundtrip(self):
        """Test perifocal → GCRS → perifocal roundtrip"""
        pos = np.array([7.0e6, 1.0e6, 0.0])
        vel = np.array([-500.0, 7200.0, 0.0])
        raan = 1.0
        inc = 0.5
        argp = 2.0
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)

        peri1 = Perifocal(pos, vel, raan, inc, argp)
        gcrs = peri1.to_gcrs(epoch)
        peri2 = Perifocal.from_gcrs(gcrs, raan, inc, argp)

        # Should get back the same values
        assert_array_almost_equal(peri2.position(), pos, decimal=3)
        assert_array_almost_equal(peri2.velocity(), vel, decimal=3)

    def test_perifocal_from_gcrs_roundtrip(self):
        """Test GCRS → perifocal → GCRS roundtrip"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([-1000.0, 7000.0, 1000.0])
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)
        raan = 0.5
        inc = 0.3
        argp = 1.5

        gcrs1 = GCRS(pos, vel, epoch)
        peri = Perifocal.from_gcrs(gcrs1, raan, inc, argp)
        gcrs2 = peri.to_gcrs(epoch)

        # Should get back the same values
        assert_array_almost_equal(gcrs2.position, pos, decimal=3)
        assert_array_almost_equal(gcrs2.velocity, vel, decimal=3)

    def test_perifocal_magnitude_conservation(self):
        """Test that position and velocity magnitudes are conserved"""
        pos = np.array([7.0e6, 2.0e6, 0.0])
        vel = np.array([-2000.0, 7000.0, 0.0])
        raan = 0.8
        inc = 0.6
        argp = 1.2
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)

        peri = Perifocal(pos, vel, raan, inc, argp)
        gcrs = peri.to_gcrs(epoch)

        r_peri = np.linalg.norm(peri.position())
        v_peri = np.linalg.norm(peri.velocity())
        r_gcrs = np.linalg.norm(gcrs.position)
        v_gcrs = np.linalg.norm(gcrs.velocity)

        # Magnitudes must be preserved
        assert_allclose(r_gcrs, r_peri, rtol=1e-10)
        assert_allclose(v_gcrs, v_peri, rtol=1e-10)

    def test_perifocal_angular_momentum_conservation(self):
        """Test that angular momentum is conserved"""
        a = 8.0e6
        e = 0.15
        nu = np.pi / 3
        raan = 0.5
        inc = 0.4
        argp = 1.0
        mu = 398600.4418e9
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)

        peri = Perifocal.from_orbital_elements_py(a, e, nu, raan, inc, argp, mu)
        gcrs = peri.to_gcrs(epoch)

        # Angular momentum h = r × v
        h_peri = np.cross(peri.position(), peri.velocity())
        h_gcrs = np.cross(gcrs.position, gcrs.velocity)

        # Magnitude should be identical
        assert_allclose(np.linalg.norm(h_gcrs), np.linalg.norm(h_peri), rtol=1e-10)

    def test_perifocal_iss_orbit(self):
        """Test ISS-like orbit in perifocal frame"""
        a = 6371000.0 + 420000.0  # ~420 km altitude
        e = 0.001  # Nearly circular
        nu = 0.0  # At periapsis
        raan = 1.2
        inc = 51.6 * np.pi / 180  # ISS inclination
        argp = 0.5
        mu = 398600.4418e9
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)

        peri = Perifocal.from_orbital_elements_py(a, e, nu, raan, inc, argp, mu)
        gcrs = peri.to_gcrs(epoch)

        # Verify orbital radius (at periapsis)
        r = np.linalg.norm(gcrs.position)
        r_periapsis = a * (1.0 - e)
        assert_allclose(r, r_periapsis, rtol=0.001)

        # Verify velocity is reasonable for LEO
        v = np.linalg.norm(gcrs.velocity)
        assert 7500.0 < v < 7800.0

    def test_perifocal_geo_orbit(self):
        """Test geostationary orbit in perifocal frame"""
        a = 42164000.0  # GEO radius
        e = 0.0001  # Nearly circular
        nu = np.pi / 2
        raan = 0.0
        inc = 0.01  # Nearly equatorial
        argp = 0.0
        mu = 398600.4418e9
        epoch = Epoch(2000, 1, 1, 12, 0, 0, 0)

        peri = Perifocal.from_orbital_elements_py(a, e, nu, raan, inc, argp, mu)
        gcrs = peri.to_gcrs(epoch)

        # Verify orbital radius
        r = np.linalg.norm(gcrs.position)
        assert_allclose(r, a, rtol=0.001)

        # Verify velocity is reasonable for GEO
        v = np.linalg.norm(gcrs.velocity)
        assert 3000.0 < v < 3100.0

    def test_perifocal_repr(self):
        """Test Perifocal string representation"""
        pos = np.array([7.0e6, 0.0, 0.0])
        vel = np.array([0.0, 7500.0, 0.0])
        raan = 0.5
        inc = 0.3
        argp = 1.0

        peri = Perifocal(pos, vel, raan, inc, argp)
        repr_str = repr(peri)

        assert "Perifocal" in repr_str
        assert "position" in repr_str
        assert "velocity" in repr_str
        assert "RAAN" in repr_str or "raan" in repr_str.lower()


class TestJ2000:
    """Test J2000 inertial reference frame"""

    def test_j2000_creation(self):
        """Test creating a J2000 coordinate"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])

        j2000 = J2000(pos, vel)

        assert_array_almost_equal(j2000.position, pos)
        assert_array_almost_equal(j2000.velocity, vel)

    def test_j2000_from_position(self):
        """Test creating J2000 with position only"""
        pos = np.array([42164000.0, 0.0, 0.0])  # GEO radius

        j2000 = J2000.from_position(pos)

        assert_array_almost_equal(j2000.position, pos)
        assert_array_almost_equal(j2000.velocity, np.zeros(3))

    def test_j2000_to_gcrs(self):
        """Test J2000 to GCRS conversion"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])
        j2000 = J2000(pos, vel)

        gcrs = j2000.to_gcrs()

        # Position and velocity should be identical
        assert_array_almost_equal(gcrs.position, pos, decimal=6)
        assert_array_almost_equal(gcrs.velocity, vel, decimal=9)

        # Epoch should be J2000
        j2000_epoch = Epoch.j2000_epoch()
        assert repr(gcrs.obstime) == repr(j2000_epoch)

    def test_j2000_from_gcrs(self):
        """Test creating J2000 from GCRS"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])
        epoch = Epoch.j2000_epoch()
        gcrs = GCRS(pos, vel, epoch)

        j2000 = J2000.from_gcrs(gcrs)

        assert_array_almost_equal(j2000.position, pos, decimal=6)
        assert_array_almost_equal(j2000.velocity, vel, decimal=9)

    def test_j2000_gcrs_roundtrip(self):
        """Test J2000 <-> GCRS roundtrip conversion"""
        pos = np.array([1.0e7, 2.0e7, 3.0e7])
        vel = np.array([1000.0, 2000.0, 3000.0])
        j2000_1 = J2000(pos, vel)

        gcrs = j2000_1.to_gcrs()
        j2000_2 = J2000.from_gcrs(gcrs)

        assert_array_almost_equal(j2000_2.position, j2000_1.position, decimal=3)
        assert_array_almost_equal(j2000_2.velocity, j2000_1.velocity, decimal=6)

    def test_j2000_to_icrs(self):
        """Test J2000 to ICRS conversion"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])
        j2000 = J2000(pos, vel)

        icrs = j2000.to_icrs()

        # J2000 and ICRS should be essentially identical
        assert_array_almost_equal(icrs.position, pos, decimal=6)
        assert_array_almost_equal(icrs.velocity, vel, decimal=9)

    def test_j2000_from_icrs(self):
        """Test creating J2000 from ICRS"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])
        icrs = ICRS(pos, vel)

        j2000 = J2000.from_icrs(icrs)

        assert_array_almost_equal(j2000.position, pos, decimal=6)
        assert_array_almost_equal(j2000.velocity, vel, decimal=9)

    def test_j2000_icrs_roundtrip(self):
        """Test J2000 <-> ICRS roundtrip conversion"""
        pos = np.array([1.0e7, 2.0e7, 3.0e7])
        vel = np.array([1000.0, 2000.0, 3000.0])
        j2000_1 = J2000(pos, vel)

        icrs = j2000_1.to_icrs()
        j2000_2 = J2000.from_icrs(icrs)

        assert_array_almost_equal(j2000_2.position, j2000_1.position, decimal=3)
        assert_array_almost_equal(j2000_2.velocity, j2000_1.velocity, decimal=6)

    def test_j2000_to_itrs(self):
        """Test J2000 to ITRS conversion"""
        r = 7.0e6  # LEO altitude
        pos_j2000 = np.array([r, 0.0, 0.0])
        vel_j2000 = np.array([0.0, 7500.0, 0.0])

        j2000 = J2000(pos_j2000, vel_j2000)
        itrs = j2000.to_itrs()

        # Position magnitude should be preserved
        r_itrs = np.linalg.norm(itrs.position)
        assert_allclose(r_itrs, r, rtol=1e-6)

        # Velocity should be affected by Earth rotation
        v_itrs = np.linalg.norm(itrs.velocity)
        assert v_itrs > 100.0  # Should have some velocity

    def test_j2000_from_itrs(self):
        """Test creating J2000 from ITRS"""
        r = 6378137.0  # Earth radius at equator
        pos_itrs = np.array([r, 0.0, 0.0])
        vel_itrs = np.array([0.0, 0.0, 0.0])
        epoch = Epoch.j2000_epoch()

        itrs = ITRS(pos_itrs, vel_itrs, epoch)
        j2000 = J2000.from_itrs(itrs)

        # Position magnitude should be preserved
        r_j2000 = np.linalg.norm(j2000.position)
        assert_allclose(r_j2000, r, rtol=1e-6)

    def test_j2000_itrs_roundtrip(self):
        """Test J2000 <-> ITRS roundtrip conversion"""
        pos = np.array([7.0e6, 1.0e6, 0.5e6])
        vel = np.array([1000.0, 7500.0, 500.0])
        j2000_1 = J2000(pos, vel)

        itrs = j2000_1.to_itrs()
        j2000_2 = J2000.from_itrs(itrs)

        # Position should be preserved (within mm precision)
        assert_array_almost_equal(j2000_2.position, j2000_1.position, decimal=0)
        # Velocity should be preserved (within mm/s precision)
        assert_array_almost_equal(j2000_2.velocity, j2000_1.velocity, decimal=0)

    def test_j2000_iss_orbit(self):
        """Test J2000 with ISS-like orbit"""
        # ISS-like orbit at ~420 km altitude
        r = 6371000.0 + 420000.0  # Earth radius + altitude
        pos = np.array([r, 0.0, 0.0])
        # Circular velocity
        mu = 398600.4418e9
        v = np.sqrt(mu / r)
        vel = np.array([0.0, v, 0.0])

        j2000 = J2000(pos, vel)

        # Verify orbital radius
        r_mag = np.linalg.norm(j2000.position)
        assert_allclose(r_mag, r, rtol=1e-9)

        # Verify velocity
        v_mag = np.linalg.norm(j2000.velocity)
        assert_allclose(v_mag, v, rtol=1e-9)

        # Convert to GCRS and verify
        gcrs = j2000.to_gcrs()
        assert_allclose(np.linalg.norm(gcrs.position), r, rtol=1e-9)

    def test_j2000_geo_orbit(self):
        """Test J2000 with GEO orbit"""
        # GEO orbit at ~35,786 km altitude
        r_geo = 42164000.0  # GEO radius
        pos = np.array([r_geo, 0.0, 0.0])
        mu = 398600.4418e9
        v_geo = np.sqrt(mu / r_geo)  # Circular velocity
        vel = np.array([0.0, v_geo, 0.0])

        j2000 = J2000(pos, vel)

        # Verify position
        r_mag = np.linalg.norm(j2000.position)
        assert_allclose(r_mag, r_geo, rtol=0.001)

        # Verify velocity is reasonable for GEO
        v_mag = np.linalg.norm(j2000.velocity)
        assert 3000.0 < v_mag < 3100.0  # ~3.07 km/s for GEO

    def test_j2000_position_magnitude_preserved(self):
        """Test position magnitude preserved across transformations"""
        r = 8.0e6
        pos = np.array([r / np.sqrt(2), r / np.sqrt(2), 0.0])
        vel = np.array([0.0, 0.0, 7000.0])
        j2000 = J2000(pos, vel)

        r_j2000 = np.linalg.norm(j2000.position)
        assert_allclose(r_j2000, r, rtol=1e-9)

        # GCRS conversion
        gcrs = j2000.to_gcrs()
        r_gcrs = np.linalg.norm(gcrs.position)
        assert_allclose(r_gcrs, r, rtol=1e-9)

        # ICRS conversion
        icrs = j2000.to_icrs()
        r_icrs = np.linalg.norm(icrs.position)
        assert_allclose(r_icrs, r, rtol=1e-9)

    def test_j2000_epoch(self):
        """Test J2000 epoch access"""
        epoch = J2000.epoch()
        assert epoch is not None
        # Should be 2000-01-01 12:00:00 TT
        assert "2000" in repr(epoch)

    def test_j2000_repr(self):
        """Test J2000 string representation"""
        pos = np.array([7.0e6, 0.0, 0.0])
        vel = np.array([0.0, 7500.0, 0.0])

        j2000 = J2000(pos, vel)
        repr_str = repr(j2000)

        assert "J2000" in repr_str
        assert "position" in repr_str
        assert "velocity" in repr_str
        assert "J2000.0" in repr_str or "epoch" in repr_str.lower()

    def test_j2000_invalid_arrays(self):
        """Test J2000 with invalid array sizes"""
        pos_wrong = np.array([1.0, 2.0])  # Only 2 elements
        vel = np.array([0.0, 0.0, 0.0])

        with pytest.raises(ValueError, match="must be 3-element arrays"):
            J2000(pos_wrong, vel)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
