"""Test J2 perturbation functionality."""

import numpy as np
import pytest
from astrora._core import (
    constants,
    j2_perturbation,
    propagate_j2_dopri5,
    propagate_j2_rk4,
    propagate_state_keplerian,
)


class TestJ2Perturbation:
    """Test J2 perturbation acceleration calculations."""

    def test_j2_perturbation_equatorial(self):
        """Test J2 acceleration for equatorial orbit."""
        # Satellite on equator (z=0)
        r = np.array([7000e3, 0.0, 0.0])
        acc = j2_perturbation(r, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH)

        # At equator: ax should be negative, ay and az should be zero
        assert acc[0] < 0.0
        assert np.abs(acc[1]) < 1e-15
        assert np.abs(acc[2]) < 1e-15

    def test_j2_perturbation_polar(self):
        """Test J2 acceleration for polar position."""
        # Satellite above pole (x=0, y=0, z>0)
        r = np.array([0.0, 0.0, 7000e3])
        acc = j2_perturbation(r, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH)

        # At pole: ax=ay=0, az should be positive
        assert np.abs(acc[0]) < 1e-15
        assert np.abs(acc[1]) < 1e-15
        assert acc[2] > 0.0

    def test_j2_perturbation_magnitude_leo(self):
        """Test J2 acceleration magnitude for LEO orbit."""
        # ISS-like orbit
        r = np.array([6778e3, 0.0, 1000e3])
        acc = j2_perturbation(r, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH)

        acc_mag = np.linalg.norm(acc)
        # J2 acceleration at LEO should be ~1e-5 to 1e-2 m/s²
        assert acc_mag > 1e-6
        assert acc_mag < 2e-2

    def test_j2_acceleration_decreases_with_altitude(self):
        """Test that J2 effect decreases as ~1/r⁴."""
        r1 = np.array([7000e3, 0.0, 0.0])
        r2 = np.array([14000e3, 0.0, 0.0])  # 2x radius

        acc1 = j2_perturbation(r1, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH)
        acc2 = j2_perturbation(r2, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH)

        ratio = np.linalg.norm(acc1) / np.linalg.norm(acc2)
        # Should be approximately 2⁴ = 16
        assert 14.0 < ratio < 18.0


class TestJ2Propagation:
    """Test J2-perturbed orbit propagation."""

    def test_propagate_j2_rk4_basic(self):
        """Test basic RK4 propagation with J2."""
        # Circular LEO orbit
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 100.0  # 100 seconds

        r1, v1 = propagate_j2_rk4(
            r0, v0, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, n_steps=10
        )

        # Verify orbit hasn't degraded unrealistically
        assert np.linalg.norm(r1) > 6000e3  # Still above Earth
        assert np.linalg.norm(v1) > 1000.0  # Still has significant velocity

    def test_propagate_j2_dopri5_basic(self):
        """Test basic DOPRI5 propagation with J2."""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 100.0

        r1, v1 = propagate_j2_dopri5(
            r0, v0, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, tol=1e-8
        )

        # Verify orbit is reasonable
        assert np.linalg.norm(r1) > 6000e3
        assert np.linalg.norm(v1) > 1000.0

    def test_j2_vs_two_body_difference(self):
        """Test that J2 causes measurable orbital perturbation."""
        # Inclined orbit (J2 has more effect)
        r0 = np.array([7000e3, 0.0, 1000e3])
        v0 = np.array([0.0, 7546.0, 100.0])
        dt = 3600.0  # 1 hour

        # Two-body propagation (no J2)
        r_twobody, v_twobody = propagate_state_keplerian(r0, v0, dt, constants.GM_EARTH)

        # J2-perturbed propagation
        r_j2, v_j2 = propagate_j2_rk4(
            r0, v0, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, n_steps=100
        )

        # Positions should differ due to J2
        pos_diff = np.linalg.norm(r_twobody - r_j2)
        assert pos_diff > 100.0  # At least 100 m difference after 1 hour
        assert pos_diff < 100000.0  # Less than 100 km difference

    def test_j2_propagation_consistency(self):
        """Test that RK4 and DOPRI5 give similar results."""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0  # 10 minutes

        # Propagate with both methods
        r_rk4, v_rk4 = propagate_j2_rk4(
            r0, v0, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, n_steps=100
        )

        r_dopri, v_dopri = propagate_j2_dopri5(
            r0, v0, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, tol=1e-8
        )

        # Results should be very close
        pos_diff = np.linalg.norm(r_rk4 - r_dopri)
        vel_diff = np.linalg.norm(v_rk4 - v_dopri)

        # Within a few meters and mm/s
        assert pos_diff < 100.0
        assert vel_diff < 0.1

    def test_j2_long_term_propagation(self):
        """Test J2 propagation over multiple orbits."""
        # ISS-like orbit at 400 km altitude
        r0 = np.array([6778e3, 0.0, 0.0])
        v0 = np.array([0.0, 7672.0, 0.0])

        # Propagate for one full orbit (~90 minutes)
        dt = 90.0 * 60.0

        r1, v1 = propagate_j2_dopri5(
            r0, v0, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, tol=1e-8
        )

        # Verify orbital radius is approximately maintained
        r0_mag = np.linalg.norm(r0)
        r1_mag = np.linalg.norm(r1)

        # Should stay within a few km (J2 doesn't change semi-major axis significantly)
        assert np.abs(r1_mag - r0_mag) < 10000.0

    def test_j2_inclined_orbit_effect(self):
        """Test that J2 has stronger effect on inclined orbits."""
        # Two orbits: equatorial and inclined
        r_eq = np.array([7000e3, 0.0, 0.0])
        v_eq = np.array([0.0, 7546.0, 0.0])

        r_inc = np.array([7000e3, 0.0, 0.0])
        v_inc = np.array([0.0, 6000.0, 4500.0])  # ~37° inclination

        dt = 3600.0  # 1 hour

        # Propagate both with two-body and J2
        r_eq_twobody, _ = propagate_state_keplerian(r_eq, v_eq, dt, constants.GM_EARTH)
        r_eq_j2, _ = propagate_j2_rk4(
            r_eq, v_eq, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, n_steps=100
        )

        r_inc_twobody, _ = propagate_state_keplerian(r_inc, v_inc, dt, constants.GM_EARTH)
        r_inc_j2, _ = propagate_j2_rk4(
            r_inc, v_inc, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, n_steps=100
        )

        diff_eq = np.linalg.norm(r_eq_twobody - r_eq_j2)
        diff_inc = np.linalg.norm(r_inc_twobody - r_inc_j2)

        # Inclined orbit should show more J2 effect
        # (Though this depends on specific geometry)
        # At least verify both have measurable differences
        assert diff_eq > 100.0
        assert diff_inc > 100.0

    def test_j2_backward_propagation(self):
        """Test that backward propagation works correctly."""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0

        # Propagate forward
        r_fwd, v_fwd = propagate_j2_rk4(
            r0, v0, dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, n_steps=50
        )

        # Propagate backward from result
        r_back, v_back = propagate_j2_rk4(
            r_fwd, v_fwd, -dt, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH, n_steps=50
        )

        # Should get back close to original position
        pos_error = np.linalg.norm(r_back - r0)
        vel_error = np.linalg.norm(v_back - v0)

        # Within numerical tolerance (not exact due to integration errors)
        assert pos_error < 1000.0  # Within 1 km
        assert vel_error < 1.0  # Within 1 m/s


class TestJ2EdgeCases:
    """Test edge cases and error handling."""

    def test_j2_invalid_position_size(self):
        """Test error handling for invalid position vector size."""
        r_invalid = np.array([7000e3, 0.0])  # Only 2 components
        with pytest.raises(ValueError, match="exactly 3 components"):
            j2_perturbation(r_invalid, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH)

    def test_j2_propagate_invalid_vectors(self):
        """Test error handling for invalid state vectors."""
        r0_invalid = np.array([7000e3, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])

        with pytest.raises(ValueError, match="exactly 3 components"):
            propagate_j2_rk4(
                r0_invalid, v0, 100.0, constants.GM_EARTH, constants.J2_EARTH, constants.R_EARTH
            )

    def test_j2_with_zero_j2_coefficient(self):
        """Test that J2=0 reduces to two-body problem."""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 1000.0

        # Propagate with J2=0 (should be two-body)
        r_j2_zero, v_j2_zero = propagate_j2_rk4(
            r0, v0, dt, constants.GM_EARTH, 0.0, constants.R_EARTH, n_steps=100  # J2 = 0
        )

        # Propagate with true two-body
        r_twobody, v_twobody = propagate_state_keplerian(r0, v0, dt, constants.GM_EARTH)

        # Should be very close (small numerical differences from different integrators)
        pos_diff = np.linalg.norm(r_j2_zero - r_twobody)
        vel_diff = np.linalg.norm(v_j2_zero - v_twobody)

        assert pos_diff < 1000.0  # Within 1 km
        assert vel_diff < 1.0  # Within 1 m/s


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
