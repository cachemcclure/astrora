"""
Integration tests for batch propagation functions

Tests the Python bindings for batch_propagate_states and batch_propagate_lagrange
to ensure they work correctly with NumPy arrays and provide the expected performance benefits.
"""

import numpy as np
import pytest
from astrora._core import (
    batch_propagate_lagrange,
    batch_propagate_states,
    constants,
    propagate_state_keplerian,
)


class TestBatchPropagation:
    """Test batch propagation of multiple state vectors"""

    def test_batch_propagate_single_time_step(self):
        """Test batch propagation with a single time step for all states"""
        # Two circular orbits at different altitudes
        r1 = 7000e3
        r2 = 8000e3
        v1 = np.sqrt(constants.GM_EARTH / r1)
        v2 = np.sqrt(constants.GM_EARTH / r2)

        states = np.array([[r1, 0.0, 0.0, 0.0, v1, 0.0], [r2, 0.0, 0.0, 0.0, v2, 0.0]])

        dt = 3600.0  # 1 hour
        result = batch_propagate_states(states, dt, constants.GM_EARTH)

        # Check shape
        assert result.shape == (2, 6)

        # Both orbits should maintain their radius (circular orbits)
        r1_new = np.linalg.norm(result[0, :3])
        r2_new = np.linalg.norm(result[1, :3])

        np.testing.assert_allclose(r1_new, r1, rtol=1e-6)
        np.testing.assert_allclose(r2_new, r2, rtol=1e-6)

    def test_batch_propagate_multiple_time_steps(self):
        """Test batch propagation with different time steps for each state"""
        r = 7000e3
        v = np.sqrt(constants.GM_EARTH / r)

        states = np.array([[r, 0.0, 0.0, 0.0, v, 0.0], [r, 0.0, 0.0, 0.0, v, 0.0]])

        # Different time steps
        dt = np.array([1800.0, 3600.0])  # 30 min and 1 hour
        result = batch_propagate_states(states, dt, constants.GM_EARTH)

        # Both should still be circular
        r1_new = np.linalg.norm(result[0, :3])
        r2_new = np.linalg.norm(result[1, :3])

        np.testing.assert_allclose(r1_new, r, rtol=1e-6)
        np.testing.assert_allclose(r2_new, r, rtol=1e-6)

    def test_batch_propagate_vs_sequential(self):
        """Test that batch propagation gives same results as sequential calls"""
        # Create 5 different orbits
        np.random.seed(42)
        n_orbits = 5
        states = np.zeros((n_orbits, 6))

        for i in range(n_orbits):
            r = (7000 + i * 1000) * 1e3  # 7000, 8000, 9000, 10000, 11000 km
            v = np.sqrt(constants.GM_EARTH / r)
            states[i] = [r, 0.0, 0.0, 0.0, v, 0.0]

        dt = 3600.0

        # Batch propagation
        result_batch = batch_propagate_states(states, dt, constants.GM_EARTH)

        # Sequential propagation
        result_seq = np.zeros_like(states)
        for i in range(n_orbits):
            r0 = states[i, :3]
            v0 = states[i, 3:]
            r_new, v_new = propagate_state_keplerian(r0, v0, dt, constants.GM_EARTH)
            result_seq[i, :3] = r_new
            result_seq[i, 3:] = v_new

        # Results should match
        np.testing.assert_allclose(result_batch, result_seq, rtol=1e-10)

    def test_batch_propagate_energy_conservation(self):
        """Test that energy is conserved for all orbits in batch"""
        # Elliptical orbits
        r0 = 7000e3
        v0 = 8000.0

        states = np.array(
            [
                [r0, 0.0, 0.0, 0.0, v0, 0.0],
                [r0, 0.0, 0.0, 0.0, v0, 0.0],
                [r0, 0.0, 0.0, 0.0, v0, 0.0],
            ]
        )

        dt = 3600.0
        result = batch_propagate_states(states, dt, constants.GM_EARTH)

        # Check energy conservation for all orbits
        for i in range(3):
            r_mag = np.linalg.norm(result[i, :3])
            v_mag = np.linalg.norm(result[i, 3:])

            energy_initial = 0.5 * v0**2 - constants.GM_EARTH / r0
            energy_final = 0.5 * v_mag**2 - constants.GM_EARTH / r_mag

            np.testing.assert_allclose(energy_final, energy_initial, rtol=1e-8)

    def test_batch_propagate_angular_momentum_conservation(self):
        """Test that angular momentum is conserved for all orbits"""
        r0 = 7000e3
        v0 = 8000.0

        states = np.array([[r0, 0.0, 0.0, 0.0, v0, 0.0], [r0, 0.0, 0.0, 0.0, v0, 0.0]])

        dt = 3600.0
        result = batch_propagate_states(states, dt, constants.GM_EARTH)

        # Check angular momentum conservation
        for i in range(2):
            h_initial = np.cross(states[i, :3], states[i, 3:])
            h_final = np.cross(result[i, :3], result[i, 3:])

            np.testing.assert_allclose(h_final, h_initial, rtol=1e-8)

    def test_batch_propagate_invalid_dimensions(self):
        """Test that invalid state dimensions raise error"""
        # Wrong number of columns
        states = np.array([[7000e3, 0.0, 0.0, 0.0, 7546.0]])  # Only 5 columns

        dt = 3600.0
        with pytest.raises(ValueError):
            batch_propagate_states(states, dt, constants.GM_EARTH)

    def test_batch_propagate_wrong_time_steps(self):
        """Test that mismatched time steps raise error"""
        states = np.array(
            [[7000e3, 0.0, 0.0, 0.0, 7546.0, 0.0], [8000e3, 0.0, 0.0, 0.0, 7000.0, 0.0]]
        )

        # Wrong number of time steps (2 states but 3 time steps)
        dt = np.array([1000.0, 2000.0, 3000.0])
        with pytest.raises(ValueError):
            batch_propagate_states(states, dt, constants.GM_EARTH)

    def test_batch_propagate_lagrange_vs_keplerian(self):
        """Test that Lagrange method gives same results as Keplerian method"""
        r = 7000e3
        v = np.sqrt(constants.GM_EARTH / r)

        states = np.array([[r, 0.0, 0.0, 0.0, v, 0.0], [r * 1.2, 0.0, 0.0, 0.0, v * 0.9, 0.0]])

        dt = 3600.0

        result1 = batch_propagate_states(states, dt, constants.GM_EARTH)
        result2 = batch_propagate_lagrange(states, dt, constants.GM_EARTH)

        # Results should be very close
        np.testing.assert_allclose(result1, result2, rtol=1e-6)

    def test_batch_propagate_backward_in_time(self):
        """Test propagating backward with negative time step"""
        r = 7000e3
        v = np.sqrt(constants.GM_EARTH / r)

        states = np.array([[r, 0.0, 0.0, 0.0, v, 0.0]])

        dt = 3600.0

        # Propagate forward then backward
        result_fwd = batch_propagate_states(states, dt, constants.GM_EARTH)
        result_back = batch_propagate_states(result_fwd, -dt, constants.GM_EARTH)

        # Should return to original state (within 1 meter for position, 0.001 m/s for velocity)
        np.testing.assert_allclose(result_back[:, :3], states[:, :3], atol=1.0)
        np.testing.assert_allclose(result_back[:, 3:], states[:, 3:], atol=1e-3)

    def test_batch_propagate_real_world_orbits(self):
        """Test with realistic orbital scenarios"""
        # ISS-like orbit
        iss_r = 6771e3  # ~400 km altitude
        iss_v = np.sqrt(constants.GM_EARTH / iss_r)

        # GEO orbit
        geo_r = 42164e3
        geo_v = np.sqrt(constants.GM_EARTH / geo_r)

        # Molniya orbit (simplified as elliptical)
        mol_a = 26600e3
        mol_e = 0.74
        mol_r_peri = mol_a * (1 - mol_e)
        mol_v_peri = np.sqrt(constants.GM_EARTH * (2 / mol_r_peri - 1 / mol_a))

        states = np.array(
            [
                [iss_r, 0.0, 0.0, 0.0, iss_v, 0.0],
                [geo_r, 0.0, 0.0, 0.0, geo_v, 0.0],
                [mol_r_peri, 0.0, 0.0, 0.0, mol_v_peri, 0.0],
            ]
        )

        # Propagate for 1 hour
        dt = 3600.0
        result = batch_propagate_states(states, dt, constants.GM_EARTH)

        # Check that all orbits conserve energy
        for i in range(3):
            r0_mag = np.linalg.norm(states[i, :3])
            v0_mag = np.linalg.norm(states[i, 3:])
            r_mag = np.linalg.norm(result[i, :3])
            v_mag = np.linalg.norm(result[i, 3:])

            energy0 = 0.5 * v0_mag**2 - constants.GM_EARTH / r0_mag
            energy = 0.5 * v_mag**2 - constants.GM_EARTH / r_mag

            np.testing.assert_allclose(energy, energy0, rtol=1e-8)

    def test_batch_propagate_large_batch(self):
        """Test performance with larger batch of orbits"""
        # Create 100 different circular orbits
        n_orbits = 100
        states = np.zeros((n_orbits, 6))

        for i in range(n_orbits):
            r = (7000 + i * 10) * 1e3  # 7000 to 7990 km in 10 km increments
            v = np.sqrt(constants.GM_EARTH / r)
            states[i] = [r, 0.0, 0.0, 0.0, v, 0.0]

        dt = 3600.0
        result = batch_propagate_states(states, dt, constants.GM_EARTH)

        # Check shape
        assert result.shape == (n_orbits, 6)

        # Spot check a few orbits for conservation
        for i in [0, n_orbits // 2, n_orbits - 1]:
            r_mag = np.linalg.norm(result[i, :3])
            r0_mag = np.linalg.norm(states[i, :3])
            np.testing.assert_allclose(r_mag, r0_mag, rtol=1e-6)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
