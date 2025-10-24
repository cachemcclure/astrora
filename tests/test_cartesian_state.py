"""Tests for CartesianState class and orbital property calculations."""

import astrora._core as core
import pytest

# Import constants
GM_EARTH = core.constants.GM_EARTH


class TestCartesianState:
    """Test CartesianState orbital property calculations."""

    def test_creation(self):
        """Test creating a CartesianState object."""
        pos = [7000e3, 0.0, 0.0]
        vel = [0.0, 7546.0, 0.0]
        state = core.CartesianState(pos, vel)

        assert state.position == pos
        assert state.velocity == vel

    def test_repr(self):
        """Test string representation."""
        pos = [7000e3, 0.0, 0.0]
        vel = [0.0, 7546.0, 0.0]
        state = core.CartesianState(pos, vel)

        repr_str = repr(state)
        assert "CartesianState" in repr_str
        assert "position" in repr_str
        assert "velocity" in repr_str

    def test_specific_energy_circular_orbit(self):
        """Test specific energy calculation for circular orbit."""
        r = 7000e3
        v = (GM_EARTH / r) ** 0.5

        state = core.CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])
        energy = state.specific_energy(GM_EARTH)

        expected_energy = -GM_EARTH / (2.0 * r)
        assert abs(energy - expected_energy) < 1e3

    def test_specific_angular_momentum(self):
        """Test angular momentum calculation."""
        pos = [7000e3, 0.0, 0.0]
        vel = [0.0, 7546.0, 0.0]
        state = core.CartesianState(pos, vel)

        h = state.specific_angular_momentum()

        # Should be in +z direction for this configuration
        assert abs(h[0]) < 1e-6
        assert abs(h[1]) < 1e-6
        assert h[2] > 0.0

        # Magnitude should be r * v
        h_mag = (h[0] ** 2 + h[1] ** 2 + h[2] ** 2) ** 0.5
        expected_h = 7000e3 * 7546.0
        assert abs(h_mag - expected_h) / expected_h < 1e-6

    def test_semi_major_axis_circular_orbit(self):
        """Test semi-major axis calculation for circular orbit."""
        r = 7000e3
        v = (GM_EARTH / r) ** 0.5

        state = core.CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])
        a = state.semi_major_axis(GM_EARTH)

        # For circular orbit, a = r
        assert abs(a - r) / r < 1e-6

    def test_period_circular_orbit(self):
        """Test period calculation for circular orbit."""
        import math

        r = 7000e3
        v = (GM_EARTH / r) ** 0.5

        state = core.CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])
        period = state.period(GM_EARTH)

        expected_period = 2.0 * math.pi * (r**3 / GM_EARTH) ** 0.5
        assert abs(period - expected_period) / expected_period < 1e-6

        # Should be approximately 97.14 minutes
        period_minutes = period / 60.0
        assert abs(period_minutes - 97.14) < 0.1

    def test_eccentricity_circular_orbit(self):
        """Test eccentricity calculation for circular orbit."""
        r = 7000e3
        v = (GM_EARTH / r) ** 0.5

        state = core.CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])
        ecc = state.eccentricity(GM_EARTH)

        # Circular orbit should have e ≈ 0
        assert ecc < 1e-6

    def test_eccentricity_elliptical_orbit(self):
        """Test eccentricity calculation for elliptical orbit."""
        # Create elliptical orbit with e = 0.5
        e_target = 0.5
        a = 10000e3

        r_p = a * (1.0 - e_target)  # Periapsis
        v_p = ((GM_EARTH / a) * (1.0 + e_target) / (1.0 - e_target)) ** 0.5

        state = core.CartesianState([r_p, 0.0, 0.0], [0.0, v_p, 0.0])
        ecc = state.eccentricity(GM_EARTH)

        assert abs(ecc - 0.5) < 1e-6

    def test_eccentricity_vector(self):
        """Test eccentricity vector calculation."""
        # Elliptical orbit at periapsis
        e_target = 0.3
        a = 8000e3

        r_p = a * (1.0 - e_target)
        v_p = ((GM_EARTH / a) * (1.0 + e_target) / (1.0 - e_target)) ** 0.5

        state = core.CartesianState([r_p, 0.0, 0.0], [0.0, v_p, 0.0])
        ecc_vec = state.eccentricity_vector(GM_EARTH)

        # Should point in +x direction at periapsis
        assert ecc_vec[0] > 0.0
        assert abs(ecc_vec[1]) < 1e-6
        assert abs(ecc_vec[2]) < 1e-6

        # Magnitude should equal target eccentricity
        ecc_mag = (ecc_vec[0] ** 2 + ecc_vec[1] ** 2 + ecc_vec[2] ** 2) ** 0.5
        assert abs(ecc_mag - e_target) < 1e-6

    def test_orbit_type_classification(self):
        """Test orbit type classification."""
        # Circular orbit
        r = 7000e3
        v = (GM_EARTH / r) ** 0.5
        state_circular = core.CartesianState([r, 0.0, 0.0], [0.0, v, 0.0])
        assert state_circular.orbit_type(GM_EARTH) == "circular"

        # Elliptical orbit
        e = 0.5
        a = 10000e3
        r_p = a * (1.0 - e)
        v_p = ((GM_EARTH / a) * (1.0 + e) / (1.0 - e)) ** 0.5
        state_elliptical = core.CartesianState([r_p, 0.0, 0.0], [0.0, v_p, 0.0])
        assert state_elliptical.orbit_type(GM_EARTH) == "elliptical"

        # Hyperbolic orbit
        v_escape = (2.0 * GM_EARTH / r) ** 0.5 * 1.5
        state_hyperbolic = core.CartesianState([r, 0.0, 0.0], [0.0, v_escape, 0.0])
        assert state_hyperbolic.orbit_type(GM_EARTH) == "hyperbolic"

    def test_hyperbolic_orbit_error(self):
        """Test that hyperbolic orbits raise errors for invalid operations."""
        # Create hyperbolic orbit
        r = 7000e3
        v_escape = (2.0 * GM_EARTH / r) ** 0.5 * 1.5

        state = core.CartesianState([r, 0.0, 0.0], [0.0, v_escape, 0.0])

        # Semi-major axis should raise ValueError
        with pytest.raises(ValueError):
            state.semi_major_axis(GM_EARTH)

        # Period should also raise ValueError
        with pytest.raises(ValueError):
            state.period(GM_EARTH)
