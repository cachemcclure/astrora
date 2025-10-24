"""
Test suite for TEME (True Equator Mean Equinox) coordinate frame

Tests cover:
- TEME frame creation and properties
- TEME ↔ ITRS transformations
- TEME ↔ GCRS transformations
- Roundtrip conversions
- Physical validation (conservation laws, Coriolis effect)
- Edge cases and error handling
"""

import numpy as np
import pytest
from astrora._core import GCRS, ITRS, TEME, Epoch

# Test tolerances
POS_TOL = 1.0  # meters for position
VEL_TOL = 0.001  # m/s for velocity


def test_teme_creation():
    """Test TEME frame creation with position and velocity"""
    pos = np.array([7000000.0, 0.0, 0.0])
    vel = np.array([0.0, 7500.0, 0.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    teme = TEME(pos, vel, epoch)

    np.testing.assert_array_equal(teme.position, pos)
    np.testing.assert_array_equal(teme.velocity, vel)
    assert teme.obstime == epoch


def test_teme_properties():
    """Test TEME frame property access"""
    pos = np.array([7000000.0, 1000000.0, 500000.0])
    vel = np.array([1000.0, 7000.0, 500.0])
    epoch = Epoch(2024, 6, 15, 12, 0, 0, 0)

    teme = TEME(pos, vel, epoch)

    # Test position property
    assert isinstance(teme.position, np.ndarray)
    assert teme.position.shape == (3,)
    np.testing.assert_allclose(teme.position, pos, rtol=1e-12)

    # Test velocity property
    assert isinstance(teme.velocity, np.ndarray)
    assert teme.velocity.shape == (3,)
    np.testing.assert_allclose(teme.velocity, vel, rtol=1e-12)

    # Test obstime property
    assert isinstance(teme.obstime, Epoch)
    assert teme.obstime == epoch


def test_teme_repr():
    """Test TEME string representation"""
    pos = np.array([7000000.0, 0.0, 0.0])
    vel = np.array([0.0, 7500.0, 0.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    teme = TEME(pos, vel, epoch)
    repr_str = repr(teme)

    assert "TEME" in repr_str
    assert "7.000e" in repr_str  # Position x-component (e6 or e+06)
    assert "7.500e" in repr_str  # Velocity y-component (e3 or e+03)
    assert "2024-01-01" in repr_str  # Date


def test_teme_to_itrs():
    """Test TEME → ITRS transformation"""
    pos_teme = np.array([7000000.0, 0.0, 0.0])
    vel_teme = np.array([0.0, 7500.0, 0.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    teme = TEME(pos_teme, vel_teme, epoch)
    itrs = teme.to_itrs()

    # Check types
    assert isinstance(itrs, ITRS)
    assert isinstance(itrs.position, np.ndarray)
    assert isinstance(itrs.velocity, np.ndarray)

    # Position magnitude should be conserved
    r_teme = np.linalg.norm(pos_teme)
    r_itrs = np.linalg.norm(itrs.position)
    assert abs(r_itrs - r_teme) < POS_TOL

    # Z-component should be unchanged (rotation around z-axis)
    assert abs(itrs.position[2] - pos_teme[2]) < 1e-6

    # Epoch should be preserved
    assert itrs.obstime == epoch


def test_itrs_to_teme():
    """Test ITRS → TEME transformation"""
    pos_itrs = np.array([6378137.0, 0.0, 0.0])
    vel_itrs = np.array([0.0, 0.0, 0.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    itrs = ITRS(pos_itrs, vel_itrs, epoch)
    teme = itrs.to_teme()

    # Check types
    assert isinstance(teme, TEME)
    assert isinstance(teme.position, np.ndarray)
    assert isinstance(teme.velocity, np.ndarray)

    # Position magnitude should be conserved
    r_itrs = np.linalg.norm(pos_itrs)
    r_teme = np.linalg.norm(teme.position)
    assert abs(r_teme - r_itrs) < POS_TOL

    # Z-component should be unchanged
    assert abs(teme.position[2] - pos_itrs[2]) < 1e-6

    # Epoch should be preserved
    assert teme.obstime == epoch


def test_teme_itrs_roundtrip():
    """Test TEME → ITRS → TEME roundtrip conversion"""
    pos = np.array([7000000.0, 1000000.0, 500000.0])
    vel = np.array([1000.0, 7000.0, 500.0])
    epoch = Epoch(2024, 6, 15, 12, 0, 0, 0)

    teme1 = TEME(pos, vel, epoch)
    itrs = teme1.to_itrs()
    teme2 = itrs.to_teme()

    # Position should match within tolerance
    np.testing.assert_allclose(teme2.position, pos, atol=1e-3)

    # Velocity should match within tolerance
    np.testing.assert_allclose(teme2.velocity, vel, atol=1e-6)

    # Epoch should match exactly
    assert teme2.obstime == epoch


def test_itrs_teme_roundtrip():
    """Test ITRS → TEME → ITRS roundtrip conversion"""
    pos = np.array([6378137.0, 1000000.0, 500000.0])
    vel = np.array([100.0, 50.0, 25.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    itrs1 = ITRS(pos, vel, epoch)
    teme = itrs1.to_teme()
    itrs2 = teme.to_itrs()

    # Position should match within tolerance
    np.testing.assert_allclose(itrs2.position, pos, atol=1e-3)

    # Velocity should match within tolerance
    np.testing.assert_allclose(itrs2.velocity, vel, atol=1e-6)

    # Epoch should match exactly
    assert itrs2.obstime == epoch


def test_teme_to_gcrs():
    """Test TEME → GCRS transformation via ITRS"""
    pos = np.array([7000000.0, 1000000.0, 500000.0])
    vel = np.array([1000.0, 7000.0, 500.0])
    epoch = Epoch(2024, 1, 1, 12, 0, 0, 0)

    teme = TEME(pos, vel, epoch)
    gcrs = teme.to_gcrs()

    # Check types
    assert isinstance(gcrs, GCRS)
    assert isinstance(gcrs.position, np.ndarray)
    assert isinstance(gcrs.velocity, np.ndarray)

    # Position magnitude should be conserved
    r_teme = np.linalg.norm(pos)
    r_gcrs = np.linalg.norm(gcrs.position)
    assert abs(r_gcrs - r_teme) < POS_TOL

    # Epoch should be preserved
    assert gcrs.obstime == epoch


def test_teme_conservation_of_position_magnitude():
    """Test that position magnitude is conserved in TEME transformations"""
    pos = np.array([8000000.0, 5656854.0, 1000000.0])  # ~45° from x-axis
    vel = np.array([-5000.0, 3000.0, 1000.0])
    epoch = Epoch(2024, 1, 1, 12, 0, 0, 0)

    teme = TEME(pos, vel, epoch)
    itrs = teme.to_itrs()

    # Position magnitude must be conserved
    r_teme = np.linalg.norm(pos)
    r_itrs = np.linalg.norm(itrs.position)
    np.testing.assert_allclose(r_itrs, r_teme, atol=1e-3)


def test_teme_polar_position_unchanged():
    """Test that positions at North Pole are rotation-invariant"""
    r_pole = 6356752.0  # WGS84 polar radius
    pos_teme = np.array([0.0, 0.0, r_pole])
    vel_teme = np.array([0.0, 0.0, 100.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    teme = TEME(pos_teme, vel_teme, epoch)
    itrs = teme.to_itrs()

    # Position at pole should remain essentially unchanged (on rotation axis)
    np.testing.assert_allclose(itrs.position[0], 0.0, atol=1e-3)
    np.testing.assert_allclose(itrs.position[1], 0.0, atol=1e-3)
    np.testing.assert_allclose(itrs.position[2], r_pole, atol=POS_TOL)


def test_teme_leo_satellite():
    """Test TEME with ISS-like LEO orbit parameters"""
    r_earth = 6371000.0
    h = 408000.0  # ISS altitude
    r = r_earth + h

    pos = np.array([r, 0.0, 0.0])
    v_circ = np.sqrt(398600.4418e9 / r)  # Circular velocity
    vel = np.array([0.0, v_circ, 0.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    teme = TEME(pos, vel, epoch)

    # Verify orbital radius
    r_mag = np.linalg.norm(teme.position)
    np.testing.assert_allclose(r_mag, r, atol=POS_TOL)

    # Verify velocity is in LEO range
    v_mag = np.linalg.norm(teme.velocity)
    assert 7000.0 < v_mag < 8000.0  # Typical LEO velocity


def test_teme_geo_satellite():
    """Test TEME with GEO satellite parameters"""
    r_geo = 42164000.0  # GEO radius
    pos = np.array([r_geo, 0.0, 0.0])
    v_geo = np.sqrt(398600.4418e9 / r_geo)  # Circular velocity
    vel = np.array([0.0, v_geo, 0.0])
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    teme = TEME(pos, vel, epoch)

    # Verify position
    r_mag = np.linalg.norm(teme.position)
    np.testing.assert_allclose(r_mag, r_geo, atol=POS_TOL)

    # Verify velocity is ~3.07 km/s for GEO
    v_mag = np.linalg.norm(teme.velocity)
    np.testing.assert_allclose(v_mag, v_geo, atol=POS_TOL)
    assert 3000.0 < v_mag < 3100.0


def test_teme_invalid_inputs():
    """Test TEME with invalid inputs"""
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    # Wrong array size for position
    with pytest.raises(ValueError, match="Position and velocity must be 3-element arrays"):
        TEME(np.array([1.0, 2.0]), np.array([1.0, 2.0, 3.0]), epoch)

    # Wrong array size for velocity
    with pytest.raises(ValueError, match="Position and velocity must be 3-element arrays"):
        TEME(np.array([1.0, 2.0, 3.0]), np.array([1.0, 2.0]), epoch)


def test_teme_different_epochs():
    """Test TEME transformations at different epochs"""
    pos = np.array([7000000.0, 0.0, 0.0])
    vel = np.array([0.0, 7500.0, 0.0])

    # Test at J2000 epoch
    epoch1 = Epoch.j2000_epoch()
    teme1 = TEME(pos, vel, epoch1)
    itrs1 = teme1.to_itrs()
    assert itrs1.obstime == epoch1

    # Test at different epoch
    epoch2 = Epoch(2025, 6, 15, 12, 0, 0, 0)
    teme2 = TEME(pos, vel, epoch2)
    itrs2 = teme2.to_itrs()
    assert itrs2.obstime == epoch2

    # ITRS positions should be different due to different GMST
    assert not np.allclose(itrs1.position, itrs2.position, atol=1e3)


def test_teme_itrs_velocity_transformation():
    """Test that velocity transformation includes Coriolis effect"""
    # Ground station at equator (stationary in ITRS)
    r_earth = 6378137.0
    pos_itrs = np.array([r_earth, 0.0, 0.0])
    vel_itrs = np.array([0.0, 0.0, 0.0])  # Stationary in Earth-fixed frame
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    itrs = ITRS(pos_itrs, vel_itrs, epoch)
    teme = itrs.to_teme()

    # In TEME (inertial), ground station has velocity due to Earth's rotation
    # v = ω × r, where ω = 7.2921150e-5 rad/s
    omega_earth = 7.2921150e-5
    expected_v = omega_earth * r_earth  # ~465 m/s at equator

    v_teme = np.linalg.norm(teme.velocity)
    np.testing.assert_allclose(v_teme, expected_v, atol=1.0)

    # Velocity should be perpendicular to position (tangential)
    dot_product = np.dot(teme.velocity, teme.position)
    np.testing.assert_allclose(dot_product, 0.0, atol=100.0)

    # Velocity should be in the equatorial plane (z-component ≈ 0)
    assert abs(teme.velocity[2]) < 1.0


def test_teme_equatorial_plane_satellites():
    """Test TEME with satellites in equatorial plane"""
    # Equatorial orbit at various altitudes
    altitudes = [400e3, 1000e3, 10000e3, 35786e3]  # LEO to GEO
    epoch = Epoch(2024, 1, 1, 0, 0, 0, 0)

    for h in altitudes:
        r = 6371000.0 + h
        pos = np.array([r, 0.0, 0.0])
        v_circ = np.sqrt(398600.4418e9 / r)
        vel = np.array([0.0, v_circ, 0.0])

        teme = TEME(pos, vel, epoch)
        itrs = teme.to_itrs()

        # Position magnitude conserved
        r_teme = np.linalg.norm(pos)
        r_itrs = np.linalg.norm(itrs.position)
        np.testing.assert_allclose(r_itrs, r_teme, atol=POS_TOL)

        # Z-component unchanged (equatorial plane)
        assert abs(itrs.position[2]) < 1e-6


def test_teme_high_inclination_orbit():
    """Test TEME with high-inclination orbit"""
    # Polar orbit
    r = 7000000.0
    pos = np.array([0.0, r, 0.0])
    vel = np.array([0.0, 0.0, 7500.0])  # Velocity in z-direction
    epoch = Epoch(2024, 1, 1, 12, 0, 0, 0)

    teme = TEME(pos, vel, epoch)
    itrs = teme.to_itrs()
    gcrs = teme.to_gcrs()

    # Position magnitude conserved in both transformations
    r_teme = np.linalg.norm(pos)
    r_itrs = np.linalg.norm(itrs.position)
    r_gcrs = np.linalg.norm(gcrs.position)

    np.testing.assert_allclose(r_itrs, r_teme, atol=POS_TOL)
    np.testing.assert_allclose(r_gcrs, r_teme, atol=POS_TOL)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
