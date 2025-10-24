"""
Tests for atmospheric drag perturbation functions.

Tests exponential atmosphere density model and drag acceleration computations.
"""

import numpy as np
import pytest
from astrora._core import (
    constants,
    drag_acceleration,
    exponential_density,
    propagate_drag_dopri5,
    propagate_drag_rk4,
    propagate_j2_drag_dopri5,
    propagate_j2_drag_rk4,
    propagate_state_keplerian,
)


class TestExponentialDensity:
    """Test exponential atmosphere density model."""

    def test_density_at_sea_level(self):
        """At zero altitude, density should equal reference density."""
        rho = exponential_density(0.0, constants.RHO0_EARTH, constants.H0_EARTH)
        assert abs(rho - constants.RHO0_EARTH) < 1e-10

    def test_density_decreases_with_altitude(self):
        """Density should decrease exponentially with altitude."""
        rho_100km = exponential_density(100e3, constants.RHO0_EARTH, constants.H0_EARTH)
        rho_200km = exponential_density(200e3, constants.RHO0_EARTH, constants.H0_EARTH)
        rho_400km = exponential_density(400e3, constants.RHO0_EARTH, constants.H0_EARTH)

        # Each should be less than the previous
        assert rho_100km < constants.RHO0_EARTH
        assert rho_200km < rho_100km
        assert rho_400km < rho_200km

        # At 400 km, density should be very small
        assert rho_400km < 1e-10
        assert rho_400km > 0.0  # But not zero

    def test_density_at_scale_height(self):
        """At one scale height, density should be 1/e of reference."""
        rho_H = exponential_density(constants.H0_EARTH, constants.RHO0_EARTH, constants.H0_EARTH)
        expected = constants.RHO0_EARTH / np.e

        assert abs(rho_H - expected) < 1e-10


class TestDragAcceleration:
    """Test drag acceleration computations."""

    def test_drag_opposes_velocity(self):
        """Drag should oppose velocity direction."""
        # ISS-like orbit at 400 km altitude
        r = np.array([6778e3, 0.0, 0.0])
        v = np.array([0.0, 7670.0, 0.0])
        B = 50.0  # CubeSat-like ballistic coefficient

        a_drag = drag_acceleration(
            r, v, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B
        )

        # Drag should oppose velocity: a_drag · v < 0
        dot_product = np.dot(a_drag, v)
        assert dot_product < 0.0, "Drag should oppose velocity"

        # Drag should be antiparallel to velocity
        a_drag_normalized = a_drag / np.linalg.norm(a_drag)
        v_normalized = v / np.linalg.norm(v)
        cos_angle = np.dot(a_drag_normalized, v_normalized)

        # Should be antiparallel: cos(angle) ≈ -1
        assert abs(cos_angle - (-1.0)) < 1e-10

    def test_drag_acceleration_magnitude(self):
        """Check drag acceleration magnitude at ISS altitude."""
        r = np.array([6778e3, 0.0, 0.0])
        v = np.array([0.0, 7670.0, 0.0])
        B = 82.0  # ISS ballistic coefficient

        a_drag = drag_acceleration(
            r, v, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B
        )

        # At 400 km with this simple model, drag is extremely small
        mag = np.linalg.norm(a_drag)
        assert mag > 0.0 and mag < 1e-5

    def test_drag_increases_at_lower_altitude(self):
        """Drag should be stronger at lower altitudes."""
        B = 50.0

        # Compare drag at 300 km vs 400 km
        r_300km = np.array([6678e3, 0.0, 0.0])
        v = np.array([0.0, 7730.0, 0.0])
        a_drag_300 = drag_acceleration(
            r_300km, v, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B
        )

        r_400km = np.array([6778e3, 0.0, 0.0])
        v2 = np.array([0.0, 7670.0, 0.0])
        a_drag_400 = drag_acceleration(
            r_400km, v2, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B
        )

        # Drag at lower altitude should be stronger
        assert np.linalg.norm(a_drag_300) > np.linalg.norm(a_drag_400)

    def test_drag_with_zero_velocity(self):
        """With zero velocity, drag should be zero."""
        r = np.array([6778e3, 0.0, 0.0])
        v = np.zeros(3)
        B = 50.0

        a_drag = drag_acceleration(
            r, v, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B
        )

        assert np.linalg.norm(a_drag) < 1e-20

    def test_ballistic_coefficient_effect(self):
        """Smaller B should result in more drag."""
        r = np.array([6778e3, 0.0, 0.0])
        v = np.array([0.0, 7670.0, 0.0])

        B_small = 20.0  # Small satellite (more drag)
        B_large = 200.0  # Large satellite (less drag)

        a_drag_small = drag_acceleration(
            r, v, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B_small
        )
        a_drag_large = drag_acceleration(
            r, v, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B_large
        )

        # Smaller B should experience more drag
        assert np.linalg.norm(a_drag_small) > np.linalg.norm(a_drag_large)

        # Ratio should be approximately B_large / B_small
        ratio = np.linalg.norm(a_drag_small) / np.linalg.norm(a_drag_large)
        expected_ratio = B_large / B_small
        assert abs(ratio - expected_ratio) < 1e-10

    def test_invalid_vector_sizes(self):
        """Test error handling for wrong vector sizes."""
        r = np.array([6778e3, 0.0])  # Only 2 components
        v = np.array([0.0, 7670.0, 0.0])
        B = 50.0

        with pytest.raises(ValueError):
            drag_acceleration(r, v, constants.R_EARTH, constants.RHO0_EARTH, constants.H0_EARTH, B)


class TestDragPropagation:
    """Test orbit propagation with atmospheric drag."""

    def test_propagate_drag_rk4_basic(self):
        """Basic test of RK4 drag propagation."""
        # ISS-like orbit
        r0 = np.array([6778e3, 0.0, 0.0])
        v0 = np.array([0.0, 7670.0, 0.0])
        B = 50.0

        r1, v1 = propagate_drag_rk4(
            r0,
            v0,
            600.0,
            constants.GM_EARTH,
            constants.R_EARTH,
            constants.RHO0_EARTH,
            constants.H0_EARTH,
            B,
            n_steps=100,
        )

        # Orbit should still be valid
        assert np.linalg.norm(r1) > 6000e3  # Still above Earth
        assert np.linalg.norm(v1) > 1000.0  # Still has significant velocity

    def test_propagate_drag_dopri5_basic(self):
        """Basic test of adaptive DOPRI5 drag propagation."""
        r0 = np.array([6778e3, 0.0, 0.0])
        v0 = np.array([0.0, 7670.0, 0.0])
        B = 50.0

        r1, v1 = propagate_drag_dopri5(
            r0,
            v0,
            600.0,
            constants.GM_EARTH,
            constants.R_EARTH,
            constants.RHO0_EARTH,
            constants.H0_EARTH,
            B,
            tol=1e-8,
        )

        assert np.linalg.norm(r1) > 6000e3
        assert np.linalg.norm(v1) > 1000.0

    def test_drag_causes_orbit_decay(self):
        """Drag should cause orbital energy to decrease."""
        r0 = np.array([6778e3, 0.0, 0.0])
        v0 = np.array([0.0, 7670.0, 0.0])
        B = 50.0
        dt = 3600.0  # 1 hour

        # Two-body propagation (no drag)
        r_twobody, v_twobody = propagate_state_keplerian(r0, v0, dt, constants.GM_EARTH)

        # Drag propagation
        r_drag, v_drag = propagate_drag_rk4(
            r0,
            v0,
            dt,
            constants.GM_EARTH,
            constants.R_EARTH,
            constants.RHO0_EARTH,
            constants.H0_EARTH,
            B,
            n_steps=100,
        )

        # Calculate specific energies
        e_twobody = np.linalg.norm(v_twobody) ** 2 / 2.0 - constants.GM_EARTH / np.linalg.norm(
            r_twobody
        )
        e_drag = np.linalg.norm(v_drag) ** 2 / 2.0 - constants.GM_EARTH / np.linalg.norm(r_drag)

        # With drag, orbital energy should decrease
        assert e_drag < e_twobody, "Drag should decrease orbital energy"

        # Orbital altitude should decrease
        assert np.linalg.norm(r_drag) < np.linalg.norm(
            r_twobody
        ), "Drag should cause orbit to decay"


class TestCombinedJ2Drag:
    """Test combined J2 + drag propagation."""

    def test_propagate_j2_drag_rk4(self):
        """Test combined J2 + drag with RK4."""
        r0 = np.array([6778e3, 0.0, 1000e3])  # Slightly inclined
        v0 = np.array([100.0, 7670.0, 0.0])
        B = 50.0

        r1, v1 = propagate_j2_drag_rk4(
            r0,
            v0,
            600.0,
            constants.GM_EARTH,
            constants.J2_EARTH,
            constants.R_EARTH,
            constants.RHO0_EARTH,
            constants.H0_EARTH,
            B,
            n_steps=100,
        )

        assert np.linalg.norm(r1) > 6000e3
        assert np.linalg.norm(v1) > 1000.0

    def test_propagate_j2_drag_dopri5(self):
        """Test combined J2 + drag with DOPRI5."""
        r0 = np.array([6778e3, 0.0, 0.0])
        v0 = np.array([0.0, 7670.0, 0.0])
        B = 50.0

        r1, v1 = propagate_j2_drag_dopri5(
            r0,
            v0,
            600.0,
            constants.GM_EARTH,
            constants.J2_EARTH,
            constants.R_EARTH,
            constants.RHO0_EARTH,
            constants.H0_EARTH,
            B,
            tol=1e-8,
        )

        assert np.linalg.norm(r1) > 6000e3
        assert np.linalg.norm(v1) > 1000.0

    def test_invalid_vector_sizes_propagation(self):
        """Test error handling in propagation functions."""
        r0 = np.array([6778e3, 0.0])  # Only 2 components
        v0 = np.array([0.0, 7670.0, 0.0])
        B = 50.0

        with pytest.raises(ValueError):
            propagate_drag_rk4(
                r0,
                v0,
                600.0,
                constants.GM_EARTH,
                constants.R_EARTH,
                constants.RHO0_EARTH,
                constants.H0_EARTH,
                B,
            )


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
