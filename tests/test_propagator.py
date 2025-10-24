"""
Tests for Keplerian orbit propagator
"""

import numpy as np
import pytest
from astrora._core import (
    Duration,
    OrbitalElements,
    constants,
    propagate_keplerian,
    propagate_keplerian_duration,
    propagate_lagrange,
    propagate_state_keplerian,
)

# Use Earth's GM for all tests
GM_EARTH = constants.GM_EARTH


class TestKeplerianPropagator:
    """Tests for basic Keplerian propagator using mean anomaly"""

    def test_propagate_circular_orbit_quarter_period(self):
        """Test propagating a circular orbit for 1/4 period"""
        # Circular orbit at 7000 km radius
        a = 7000e3  # m
        e = 0.0  # circular
        i = 0.0  # equatorial
        raan = 0.0
        argp = 0.0
        nu0 = 0.0  # starting at periapsis

        elements = OrbitalElements(a, e, i, raan, argp, nu0)

        # Calculate period
        period = elements.orbital_period(GM_EARTH)

        # Propagate for 1/4 period (should be 90° around)
        dt = period / 4.0
        new_elements = propagate_keplerian(elements, dt, GM_EARTH)

        # After 1/4 period, true anomaly should be π/2 (90°)
        assert new_elements.nu == pytest.approx(np.pi / 2.0, abs=1e-6)

        # All other elements should remain constant
        assert new_elements.a == pytest.approx(a, rel=1e-6)
        assert new_elements.e == pytest.approx(e, abs=1e-8)
        assert new_elements.i == pytest.approx(i, abs=1e-8)
        assert new_elements.raan == pytest.approx(raan, abs=1e-8)
        assert new_elements.argp == pytest.approx(argp, abs=1e-8)

    def test_propagate_full_orbit(self):
        """Test propagating for one full orbital period"""
        # Elliptical orbit
        a = 8000e3
        e = 0.1
        i = 0.0
        raan = 0.0
        argp = 0.0
        nu0 = 0.0

        elements = OrbitalElements(a, e, i, raan, argp, nu0)
        period = elements.orbital_period(GM_EARTH)

        # Propagate for one full period
        new_elements = propagate_keplerian(elements, period, GM_EARTH)

        # Should return to original position
        # Account for 2π wrap-around
        nu_diff = abs(new_elements.nu - nu0)
        assert (nu_diff < 1e-6) or (abs(2.0 * np.pi - nu_diff) < 1e-6)

        # All orbital elements should remain the same
        assert new_elements.a == pytest.approx(a, rel=1e-6)
        assert new_elements.e == pytest.approx(e, abs=1e-6)

    def test_propagate_half_orbit(self):
        """Test propagating for half an orbit"""
        a = 7500e3
        e = 0.05
        i = 0.0
        raan = 0.0
        argp = 0.0
        nu0 = 0.0

        elements = OrbitalElements(a, e, i, raan, argp, nu0)
        period = elements.orbital_period(GM_EARTH)

        # Propagate for half period
        new_elements = propagate_keplerian(elements, period / 2.0, GM_EARTH)

        # Should be near apoapsis (true anomaly ≈ π)
        assert new_elements.nu == pytest.approx(np.pi, abs=1e-5)

    def test_propagate_rejects_hyperbolic(self):
        """Test that propagator rejects hyperbolic orbits"""
        # Hyperbolic orbit (e > 1)
        elements = OrbitalElements(8000e3, 1.5, 0.0, 0.0, 0.0, 0.0)

        with pytest.raises(ValueError, match="hyperbolic"):
            propagate_keplerian(elements, 3600.0, GM_EARTH)

    def test_propagate_rejects_parabolic(self):
        """Test that propagator rejects parabolic orbits"""
        # Parabolic orbit (e = 1)
        elements = OrbitalElements(8000e3, 1.0, 0.0, 0.0, 0.0, 0.0)

        with pytest.raises(ValueError, match="parabolic"):
            propagate_keplerian(elements, 3600.0, GM_EARTH)

    def test_propagate_with_duration(self):
        """Test propagating using Duration object"""
        elements = OrbitalElements(7000e3, 0.01, 0.0, 0.0, 0.0, 0.0)
        duration = Duration(3600.0)  # 1 hour (constructor takes seconds directly)

        new_elements = propagate_keplerian_duration(elements, duration, GM_EARTH)

        # Should produce valid result
        assert new_elements.a == pytest.approx(elements.a, rel=1e-6)
        assert new_elements.e == pytest.approx(elements.e, abs=1e-8)
        assert new_elements.nu != elements.nu  # Should have propagated


class TestStateVectorPropagator:
    """Tests for propagating Cartesian state vectors"""

    def test_propagate_state_circular(self):
        """Test propagating circular orbit state vectors"""
        # Circular orbit at 7000 km
        r0 = np.array([7000e3, 0.0, 0.0])
        v_circ = np.sqrt(GM_EARTH / 7000e3)
        v0 = np.array([0.0, v_circ, 0.0])

        # Propagate for 1 hour
        dt = 3600.0
        r, v = propagate_state_keplerian(r0, v0, dt, GM_EARTH)

        # Check that orbit radius remains constant (circular orbit)
        r0_mag = np.linalg.norm(r0)
        r_mag = np.linalg.norm(r)
        assert r_mag == pytest.approx(r0_mag, rel=1e-6)

        # Check velocity magnitude remains constant
        v0_mag = np.linalg.norm(v0)
        v_mag = np.linalg.norm(v)
        assert v_mag == pytest.approx(v0_mag, rel=1e-6)

    def test_propagate_state_energy_conservation(self):
        """Test that orbital energy is conserved"""
        # Elliptical orbit
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 8000.0, 0.0])

        # Calculate initial energy
        v0_mag_sq = np.dot(v0, v0)
        r0_mag = np.linalg.norm(r0)
        energy0 = 0.5 * v0_mag_sq - GM_EARTH / r0_mag

        # Propagate
        dt = 5000.0
        r, v = propagate_state_keplerian(r0, v0, dt, GM_EARTH)

        # Calculate final energy
        v_mag_sq = np.dot(v, v)
        r_mag = np.linalg.norm(r)
        energy = 0.5 * v_mag_sq - GM_EARTH / r_mag

        # Energy should be conserved
        assert energy == pytest.approx(energy0, rel=1e-6)

    def test_propagate_state_angular_momentum_conservation(self):
        """Test that angular momentum is conserved"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 8000.0, 0.0])

        # Calculate initial angular momentum
        h0 = np.cross(r0, v0)

        # Propagate
        dt = 5000.0
        r, v = propagate_state_keplerian(r0, v0, dt, GM_EARTH)

        # Calculate final angular momentum
        h = np.cross(r, v)

        # Angular momentum should be conserved
        np.testing.assert_allclose(h, h0, rtol=1e-6)

    def test_propagate_backward_in_time(self):
        """Test propagating backward with negative dt"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v_circ = np.sqrt(GM_EARTH / 7000e3)
        v0 = np.array([0.0, v_circ, 0.0])

        dt = 3600.0

        # Propagate forward
        r_fwd, v_fwd = propagate_state_keplerian(r0, v0, dt, GM_EARTH)

        # Propagate backward
        r_back, v_back = propagate_state_keplerian(r_fwd, v_fwd, -dt, GM_EARTH)

        # Should return to original state
        np.testing.assert_allclose(r_back, r0, atol=1e-3)
        np.testing.assert_allclose(v_back, v0, atol=1e-3)

    def test_propagate_inclined_orbit(self):
        """Test propagating an inclined orbit"""
        # 45° inclined circular orbit
        r0 = np.array([7000e3, 0.0, 0.0])
        v_circ = np.sqrt(GM_EARTH / 7000e3)
        angle = np.pi / 4.0  # 45°
        v0 = np.array([0.0, v_circ * np.cos(angle), v_circ * np.sin(angle)])

        dt = 2000.0
        r, v = propagate_state_keplerian(r0, v0, dt, GM_EARTH)

        # Check energy conservation
        energy0 = 0.5 * np.dot(v0, v0) - GM_EARTH / np.linalg.norm(r0)
        energy = 0.5 * np.dot(v, v) - GM_EARTH / np.linalg.norm(r)
        assert energy == pytest.approx(energy0, rel=1e-6)

        # Check angular momentum conservation
        h0 = np.cross(r0, v0)
        h = np.cross(r, v)
        np.testing.assert_allclose(h, h0, rtol=1e-6)


class TestLagrangePropagator:
    """Tests for Lagrange coefficient (f and g) propagator"""

    def test_lagrange_vs_mean_anomaly(self):
        """Test that Lagrange method gives same result as mean anomaly method"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v_circ = np.sqrt(GM_EARTH / 7000e3)
        v0 = np.array([0.0, v_circ, 0.0])
        dt = 3600.0

        # Method 1: Mean anomaly propagation
        r1, v1 = propagate_state_keplerian(r0, v0, dt, GM_EARTH)

        # Method 2: Lagrange coefficients
        r2, v2 = propagate_lagrange(r0, v0, dt, GM_EARTH)

        # Results should be very close
        np.testing.assert_allclose(r1, r2, rtol=1e-6, atol=1.0)
        np.testing.assert_allclose(v1, v2, rtol=1e-6, atol=1e-3)

    def test_lagrange_elliptical_orbit(self):
        """Test Lagrange propagation for elliptical orbit"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 8500.0, 0.0])  # Elliptical
        dt = 4000.0

        r, v = propagate_lagrange(r0, v0, dt, GM_EARTH)

        # Check energy conservation
        energy0 = 0.5 * np.dot(v0, v0) - GM_EARTH / np.linalg.norm(r0)
        energy = 0.5 * np.dot(v, v) - GM_EARTH / np.linalg.norm(r)
        assert energy == pytest.approx(energy0, rel=1e-6)

        # Check angular momentum conservation
        h0 = np.cross(r0, v0)
        h = np.cross(r, v)
        np.testing.assert_allclose(h, h0, rtol=1e-6)

    def test_lagrange_multiple_propagations(self):
        """Test multiple sequential propagations"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v_circ = np.sqrt(GM_EARTH / 7000e3)
        v0 = np.array([0.0, v_circ, 0.0])

        dt_step = 1000.0  # 1000 second steps
        r, v = r0, v0

        # Propagate 5 times
        for _ in range(5):
            r, v = propagate_lagrange(r, v, dt_step, GM_EARTH)

        # Compare with single propagation
        r_single, v_single = propagate_lagrange(r0, v0, 5 * dt_step, GM_EARTH)

        # Should give same result (within numerical precision)
        np.testing.assert_allclose(r, r_single, rtol=1e-5, atol=10.0)
        np.testing.assert_allclose(v, v_single, rtol=1e-5, atol=0.1)


class TestInputValidation:
    """Tests for input validation and error handling"""

    def test_state_vector_wrong_size(self):
        """Test that wrong-sized state vectors are rejected"""
        r0 = np.array([7000e3, 0.0])  # Only 2 components
        v0 = np.array([0.0, 7546.0, 0.0])

        with pytest.raises(ValueError, match="exactly 3 components"):
            propagate_state_keplerian(r0, v0, 3600.0, GM_EARTH)

    def test_negative_semi_major_axis(self):
        """Test that negative semi-major axis is rejected"""
        elements = OrbitalElements(-7000e3, 0.1, 0.0, 0.0, 0.0, 0.0)

        with pytest.raises(ValueError, match="positive"):
            propagate_keplerian(elements, 3600.0, GM_EARTH)


class TestRealWorldScenarios:
    """Tests with real-world orbital scenarios"""

    def test_iss_like_orbit(self):
        """Test propagating an ISS-like orbit"""
        # ISS-like orbit: ~400 km altitude, ~51.6° inclination
        altitude = 400e3
        r_earth = 6371e3
        a = r_earth + altitude

        elements = OrbitalElements(a, 0.0005, np.deg2rad(51.6), 0.0, 0.0, 0.0)
        period = elements.orbital_period(GM_EARTH)

        # Propagate for one orbit
        new_elements = propagate_keplerian(elements, period, GM_EARTH)

        # Should complete orbit and return near starting point
        nu_diff = abs(new_elements.nu - elements.nu)
        assert (nu_diff < 1e-5) or (abs(2.0 * np.pi - nu_diff) < 1e-5)

        # Period should be ~90 minutes
        assert period / 60.0 == pytest.approx(92.7, abs=1.0)

    def test_geostationary_orbit(self):
        """Test propagating a geostationary orbit"""
        # GEO orbit: 35,786 km altitude
        altitude = 35786e3
        r_earth = 6371e3
        a = r_earth + altitude

        elements = OrbitalElements(a, 0.0, 0.0, 0.0, 0.0, 0.0)
        period = elements.orbital_period(GM_EARTH)

        # Period should be ~24 hours
        assert period / 3600.0 == pytest.approx(23.93, abs=0.1)

        # Propagate for one orbit
        new_elements = propagate_keplerian(elements, period, GM_EARTH)

        # Should complete orbit
        assert new_elements.a == pytest.approx(a, rel=1e-6)

    def test_molniya_orbit(self):
        """Test propagating a Molniya-like orbit (highly elliptical)"""
        # Molniya orbit: high eccentricity, 12-hour period
        a = 26554e3  # Semi-major axis for 12-hour period
        e = 0.74  # High eccentricity
        i = np.deg2rad(63.4)  # Critical inclination

        elements = OrbitalElements(a, e, i, 0.0, 0.0, 0.0)
        period = elements.orbital_period(GM_EARTH)

        # Period should be ~12 hours
        assert period / 3600.0 == pytest.approx(12.0, abs=0.5)

        # Propagate for half period (should reach apogee)
        new_elements = propagate_keplerian(elements, period / 2.0, GM_EARTH)

        # At apogee, true anomaly should be π
        assert new_elements.nu == pytest.approx(np.pi, abs=1e-5)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
