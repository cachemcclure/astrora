"""
Tests for SGP4/SDP4 satellite propagation.

This test module validates the satellite operations module, including:
- TLE parsing (2-line and 3-line formats)
- SGP4 propagation
- OMM JSON parsing
- Batch propagation
"""

import numpy as np
import pytest
from astrora._core import py_propagate_omm, py_propagate_tle, py_propagate_tle_batch


class TestTLEPropagation:
    """Test TLE-based satellite propagation."""

    # ISS TLE from 2008-09-20 (Celestrak validation dataset)
    ISS_TLE_2LINE = """1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""

    ISS_TLE_3LINE = """ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""

    def test_propagate_at_epoch(self):
        """Test propagation at TLE epoch (time offset = 0)."""
        state = py_propagate_tle(self.ISS_TLE_2LINE, 0.0)

        # Check return structure
        assert "position" in state
        assert "velocity" in state
        assert "time_offset_minutes" in state
        assert "altitude_km" in state
        assert "speed_km_s" in state

        # Check position is a 3D vector
        assert len(state["position"]) == 3

        # ISS altitude should be around 350-450 km
        altitude = state["altitude_km"]
        assert 300 < altitude < 500, f"ISS altitude should be ~400 km, got {altitude}"

        # ISS speed should be around 7.6-7.8 km/s for LEO
        speed = state["speed_km_s"]
        assert 7.0 < speed < 8.0, f"ISS speed should be ~7.7 km/s, got {speed}"

    def test_propagate_2line_tle(self):
        """Test propagation with 2-line TLE format."""
        state = py_propagate_tle(self.ISS_TLE_2LINE, 120.0)  # 2 hours after epoch

        assert state["time_offset_minutes"] == 120.0
        assert state["altitude_km"] > 300  # Still in orbit

    def test_propagate_3line_tle(self):
        """Test propagation with 3-line TLE format (includes satellite name)."""
        state = py_propagate_tle(self.ISS_TLE_3LINE, 60.0)  # 1 hour after epoch

        assert state["time_offset_minutes"] == 60.0
        assert state["altitude_km"] > 300  # Still in orbit

    def test_propagate_one_orbit(self):
        """Test propagation for one full orbit."""
        # ISS orbital period ~90 minutes (mean motion = 15.72 revs/day)
        period_minutes = 1440.0 / 15.72125391

        state_epoch = py_propagate_tle(self.ISS_TLE_2LINE, 0.0)
        state_one_orbit = py_propagate_tle(self.ISS_TLE_2LINE, period_minutes)

        # Positions should be similar (not exact due to perturbations)
        pos_diff = np.linalg.norm(
            np.array(state_epoch["position"]) - np.array(state_one_orbit["position"])
        )

        # Should be within 100 km after one orbit (SGP4 models perturbations)
        assert pos_diff < 100.0, f"Position drift after one orbit: {pos_diff} km"

    def test_batch_propagation(self):
        """Test batch propagation to multiple time offsets."""
        time_offsets = np.array([0.0, 30.0, 60.0, 90.0, 120.0])  # Every 30 minutes
        states = py_propagate_tle_batch(self.ISS_TLE_2LINE, time_offsets)

        assert len(states) == 5

        # Check each state
        for i, state in enumerate(states):
            assert state["time_offset_minutes"] == time_offsets[i]
            assert 300 < state["altitude_km"] < 500
            assert 7.0 < state["speed_km_s"] < 8.0

    def test_batch_vs_single_propagation(self):
        """Verify batch propagation matches single propagation."""
        time_offsets = np.array([0.0, 60.0, 120.0])
        states_batch = py_propagate_tle_batch(self.ISS_TLE_2LINE, time_offsets)

        for i, offset in enumerate(time_offsets):
            state_single = py_propagate_tle(self.ISS_TLE_2LINE, offset)

            # Positions should match exactly
            np.testing.assert_allclose(
                states_batch[i]["position"],
                state_single["position"],
                rtol=1e-10,
                err_msg=f"Batch and single propagation differ at offset {offset}",
            )

    def test_invalid_tle(self):
        """Test error handling for invalid TLE data."""
        invalid_tle = "This is not a valid TLE"

        with pytest.raises(RuntimeError):
            py_propagate_tle(invalid_tle, 0.0)

    def test_time_out_of_range(self):
        """Test error handling for excessive time offsets."""
        # >1000 days should fail (TLE accuracy degrades)
        with pytest.raises(RuntimeError):
            py_propagate_tle(self.ISS_TLE_2LINE, 1500.0 * 24.0 * 60.0)


class TestOMMPropagation:
    """Test OMM (Orbit Mean-Elements Message) JSON propagation."""

    ISS_OMM = """{
        "OBJECT_NAME": "ISS (ZARYA)",
        "OBJECT_ID": "1998-067A",
        "EPOCH": "2008-09-20T12:25:40.104",
        "MEAN_MOTION": 15.72125391,
        "ECCENTRICITY": 0.0006703,
        "INCLINATION": 51.6416,
        "RA_OF_ASC_NODE": 247.4627,
        "ARG_OF_PERICENTER": 130.5360,
        "MEAN_ANOMALY": 325.0288,
        "EPHEMERIS_TYPE": 0,
        "CLASSIFICATION_TYPE": "U",
        "NORAD_CAT_ID": 25544,
        "ELEMENT_SET_NO": 292,
        "REV_AT_EPOCH": 56353,
        "BSTAR": -0.000011606,
        "MEAN_MOTION_DOT": -0.00002182,
        "MEAN_MOTION_DDOT": 0.0
    }"""

    def test_propagate_omm_at_epoch(self):
        """Test OMM propagation at epoch."""
        state = py_propagate_omm(self.ISS_OMM, 0.0)

        # Check structure
        assert "position" in state
        assert "velocity" in state
        assert len(state["position"]) == 3

        # Check ISS characteristics
        assert 300 < state["altitude_km"] < 500
        assert 7.0 < state["speed_km_s"] < 8.0

    def test_propagate_omm_offset(self):
        """Test OMM propagation with time offset."""
        state = py_propagate_omm(self.ISS_OMM, 90.0)  # 90 minutes after epoch

        assert state["time_offset_minutes"] == 90.0
        assert state["altitude_km"] > 300

    def test_invalid_omm(self):
        """Test error handling for invalid OMM JSON."""
        invalid_omm = '{"invalid": "data"}'

        with pytest.raises(RuntimeError):
            py_propagate_omm(invalid_omm, 0.0)

    def test_omm_vs_tle_equivalence(self):
        """Verify OMM and TLE produce equivalent results for same epoch."""
        tle = """ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""

        state_tle = py_propagate_tle(tle, 0.0)
        state_omm = py_propagate_omm(self.ISS_OMM, 0.0)

        # Positions should match within 1 meter (small numerical differences expected)
        np.testing.assert_allclose(
            state_tle["position"],
            state_omm["position"],
            atol=1e-3,  # 1 meter tolerance
            err_msg="TLE and OMM propagation should produce equivalent results",
        )


class TestPhysicalValidation:
    """Test physical validity of SGP4 propagation results."""

    ISS_TLE = """ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"""

    def test_position_magnitude_reasonable(self):
        """Test that position magnitude is reasonable for LEO."""
        state = py_propagate_tle(self.ISS_TLE, 0.0)
        pos = np.array(state["position"])
        r_mag = np.linalg.norm(pos)

        # LEO satellites: 6400-8000 km from Earth center
        assert 6400 < r_mag < 8000, f"Position magnitude {r_mag} km is unreasonable for LEO"

    def test_velocity_magnitude_reasonable(self):
        """Test that velocity magnitude is reasonable for LEO."""
        state = py_propagate_tle(self.ISS_TLE, 0.0)
        vel = np.array(state["velocity"])
        v_mag = np.linalg.norm(vel)

        # LEO satellites: 7-8 km/s orbital speed
        assert 7.0 < v_mag < 8.0, f"Velocity magnitude {v_mag} km/s is unreasonable for LEO"

    def test_orbital_energy_conservation(self):
        """Test that specific orbital energy remains approximately constant."""
        mu = 398600.4418  # Earth gravitational parameter (km^3/s^2)

        state0 = py_propagate_tle(self.ISS_TLE, 0.0)
        state60 = py_propagate_tle(self.ISS_TLE, 60.0)

        # Specific energy: E = v^2/2 - mu/r
        def specific_energy(state):
            r = np.linalg.norm(state["position"])
            v = np.linalg.norm(state["velocity"])
            return (v**2) / 2.0 - mu / r

        E0 = specific_energy(state0)
        E60 = specific_energy(state60)

        # Energy should be approximately constant (SGP4 has drag, so small decrease expected)
        energy_change_percent = abs(E60 - E0) / abs(E0) * 100

        # Allow up to 1% energy change over 1 hour (atmospheric drag)
        assert (
            energy_change_percent < 1.0
        ), f"Energy changed by {energy_change_percent}% (E0={E0}, E60={E60})"
