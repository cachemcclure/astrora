"""
Tests for State Transition Matrix (STM) propagation

These tests validate the STM propagation implementation for:
- Two-body dynamics
- J2-perturbed dynamics
- Linearity property (STM correctly maps perturbations)
- Conservation properties (Liouville's theorem)
- Comparison with finite differences
"""

import numpy as np
import pytest
from astrora import _core as core


class TestSTMTwoBody:
    """Tests for STM propagation in two-body dynamics"""

    def test_stm_import(self):
        """Test that STM functions are available"""
        assert hasattr(core, "propagate_stm_rk4")
        assert hasattr(core, "propagate_stm_dopri5")
        assert hasattr(core, "propagate_stm_j2_rk4")

    def test_stm_basic_propagation(self):
        """Test basic STM propagation"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0  # 10 minutes
        mu = core.constants.GM_EARTH

        r, v, stm = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)

        # Verify state is reasonable
        assert np.linalg.norm(r) > 6000e3
        assert np.linalg.norm(r) < 8000e3
        assert np.linalg.norm(v) > 1000.0
        assert np.linalg.norm(v) < 10000.0

        # Verify STM shape
        assert stm.shape == (6, 6)

        # STM should not be identity (it evolved)
        assert not np.allclose(stm, np.eye(6), rtol=0.01)

    def test_stm_det_conservation(self):
        """Test that STM determinant is ~1 (Liouville's theorem)"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        mu = core.constants.GM_EARTH

        r, v, stm = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)

        # For Hamiltonian systems, det(STM) ≈ 1
        det = np.linalg.det(stm)
        assert np.abs(det) > 0.99 and np.abs(det) < 1.01

    def test_stm_linearity(self):
        """Test that STM correctly maps perturbations"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        mu = core.constants.GM_EARTH

        # Propagate nominal trajectory with STM
        r_nom, v_nom, stm = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)

        # Apply 1 km perturbation in x
        dr0 = 1000.0
        r0_pert = r0 + np.array([dr0, 0.0, 0.0])

        # Propagate perturbed trajectory (without STM for simplicity)
        r_pert, v_pert, _ = core.propagate_stm_rk4(r0_pert, v0, dt, mu, n_steps=100)

        # Actual difference
        dr_actual = r_pert - r_nom
        dv_actual = v_pert - v_nom

        # STM prediction
        delta_x0 = np.array([dr0, 0.0, 0.0, 0.0, 0.0, 0.0])
        delta_x = stm @ delta_x0

        # STM should predict position change reasonably well
        pos_error = np.abs(dr_actual[0] - delta_x[0])
        vel_error = np.abs(dv_actual[0] - delta_x[3])

        # For small perturbations and short times, error should be small
        assert pos_error < dr0 * 0.01  # < 1% error in position
        assert vel_error < 1.0  # < 1 m/s error in velocity

    def test_stm_dopri5_vs_rk4(self):
        """Test that DOPRI5 and RK4 give similar results"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        mu = core.constants.GM_EARTH

        r_rk4, v_rk4, stm_rk4 = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)
        r_dopri5, v_dopri5, stm_dopri5 = core.propagate_stm_dopri5(r0, v0, dt, mu, tol=1e-10)

        # States should be very close
        assert np.linalg.norm(r_rk4 - r_dopri5) < 100.0  # < 100 m
        assert np.linalg.norm(v_rk4 - v_dopri5) < 0.1  # < 0.1 m/s

        # STMs should be close
        assert np.linalg.norm(stm_rk4 - stm_dopri5) < 0.01

    def test_stm_zero_time(self):
        """Test that STM at t=0 is identity"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 1e-10  # Very small time
        mu = core.constants.GM_EARTH

        r, v, stm = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=1)

        # STM should be very close to identity
        assert np.allclose(stm, np.eye(6), atol=1e-6)

    def test_stm_backward_propagation(self):
        """Test backward propagation (negative time)"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        mu = core.constants.GM_EARTH

        # Forward propagation
        r_fwd, v_fwd, stm_fwd = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)

        # Backward propagation from forward result
        r_back, v_back, stm_back = core.propagate_stm_rk4(r_fwd, v_fwd, -dt, mu, n_steps=100)

        # Should return to initial state (approximately)
        assert np.linalg.norm(r_back - r0) < 1000.0  # < 1 km error
        assert np.linalg.norm(v_back - v0) < 1.0  # < 1 m/s error

        # STMs should be approximate inverses
        stm_product = stm_back @ stm_fwd
        assert np.allclose(stm_product, np.eye(6), atol=0.01)


class TestSTMJ2:
    """Tests for STM propagation with J2 perturbations"""

    def test_stm_j2_basic(self):
        """Test basic J2 STM propagation"""
        r0 = np.array([7000e3, 0.0, 1000e3])  # Inclined orbit
        v0 = np.array([0.0, 7546.0, 100.0])
        dt = 600.0
        mu = core.constants.GM_EARTH
        j2 = core.constants.J2_EARTH
        R = core.constants.R_EARTH

        r, v, stm = core.propagate_stm_j2_rk4(r0, v0, dt, mu, j2, R, n_steps=100)

        # Verify state is reasonable
        assert np.linalg.norm(r) > 6000e3
        assert np.linalg.norm(v) > 1000.0

        # Verify STM shape
        assert stm.shape == (6, 6)

        # STM determinant should still be ~1
        det = np.linalg.det(stm)
        assert np.abs(det) > 0.9 and np.abs(det) < 1.1

    def test_stm_j2_vs_two_body(self):
        """Test that J2 STM differs from two-body STM"""
        r0 = np.array([7000e3, 0.0, 1000e3])
        v0 = np.array([0.0, 7546.0, 100.0])
        dt = 3600.0  # 1 hour - enough for J2 effects
        mu = core.constants.GM_EARTH
        j2 = core.constants.J2_EARTH
        R = core.constants.R_EARTH

        # Two-body STM
        r_tb, v_tb, stm_tb = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)

        # J2 STM
        r_j2, v_j2, stm_j2 = core.propagate_stm_j2_rk4(r0, v0, dt, mu, j2, R, n_steps=100)

        # States should differ due to J2
        assert np.linalg.norm(r_tb - r_j2) > 10.0  # > 10 m after 1 hour

        # STMs should also differ
        stm_diff = np.linalg.norm(stm_tb - stm_j2)
        assert stm_diff > 0.001  # Noticeable difference


class TestSTMFiniteDifferences:
    """Test STM against finite difference approximations"""

    def test_stm_vs_finite_differences(self):
        """Compare STM with finite differences"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 300.0  # 5 minutes (shorter for better FD accuracy)
        mu = core.constants.GM_EARTH

        # Get STM
        r_nom, v_nom, stm = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)

        # Compute finite differences (partial derivatives numerically)
        eps = 1.0  # 1 m perturbation
        fd_matrix = np.zeros((6, 6))

        for i in range(6):
            # Create perturbation
            delta = np.zeros(6)
            delta[i] = eps

            # Apply perturbation
            r0_pert = r0 + delta[0:3]
            v0_pert = v0 + delta[3:6]

            # Propagate perturbed state
            r_pert, v_pert, _ = core.propagate_stm_rk4(r0_pert, v0_pert, dt, mu, n_steps=100)

            # Compute derivative
            dr = r_pert - r_nom
            dv = v_pert - v_nom
            fd_matrix[:, i] = np.concatenate([dr, dv]) / eps

        # STM and finite differences should agree reasonably well
        # (not perfect due to numerical differentiation errors)
        rel_error = np.linalg.norm(stm - fd_matrix) / np.linalg.norm(stm)
        assert rel_error < 0.05  # < 5% relative error


class TestSTMEdgeCases:
    """Tests for edge cases and error handling"""

    def test_stm_invalid_input_shape(self):
        """Test that invalid input shapes raise errors"""
        r0 = np.array([7000e3, 0.0])  # Wrong shape (2 instead of 3)
        v0 = np.array([0.0, 7546.0, 0.0])

        with pytest.raises(ValueError):
            core.propagate_stm_rk4(r0, v0, 600.0, core.constants.GM_EARTH)

    def test_stm_large_time_step(self):
        """Test STM with large time step (multiple orbits)"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 5400.0  # 90 minutes (> 1 orbit period)
        mu = core.constants.GM_EARTH

        r, v, stm = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=200)

        # Should still work
        assert np.linalg.norm(r) > 6000e3
        assert stm.shape == (6, 6)

        # Determinant may deviate more for long integrations but should be reasonable
        det = np.linalg.det(stm)
        assert np.abs(det) > 0.5 and np.abs(det) < 2.0

    def test_stm_high_eccentricity(self):
        """Test STM for highly eccentric orbit"""
        # Molniya orbit parameters
        r0 = np.array([6678e3, 0.0, 0.0])  # Periapsis
        v0 = np.array([0.0, 10300.0, 0.0])  # High velocity at periapsis
        dt = 600.0
        mu = core.constants.GM_EARTH

        r, v, stm = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=200)

        # Should handle eccentric orbit
        assert np.linalg.norm(r) > 6000e3
        assert stm.shape == (6, 6)

    def test_stm_consistency_across_methods(self):
        """Test that different integration methods give consistent results"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        mu = core.constants.GM_EARTH

        # RK4 with different step counts
        _, _, stm_rk4_50 = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=50)
        _, _, stm_rk4_100 = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=100)
        _, _, stm_rk4_200 = core.propagate_stm_rk4(r0, v0, dt, mu, n_steps=200)

        # Higher resolution should converge
        diff_100_200 = np.linalg.norm(stm_rk4_100 - stm_rk4_200)
        diff_50_100 = np.linalg.norm(stm_rk4_50 - stm_rk4_100)

        # Difference should decrease with more steps
        assert diff_100_200 < diff_50_100


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
