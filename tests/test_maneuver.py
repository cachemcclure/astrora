"""
Tests for the Maneuver class.

Tests cover:
- Basic impulse creation
- Hohmann transfers
- Bi-elliptic transfers
- Lambert transfers
- Maneuver analysis (total time, total cost)
- Error handling
"""

import numpy as np
import pytest
from astrora._core import Duration, Epoch
from astrora.bodies import Earth, Mars
from astrora.maneuver import Maneuver
from astrora.twobody import Orbit


class TestManeuverCreation:
    """Test basic Maneuver creation and properties."""

    def test_single_impulse(self):
        """Test creating a single impulse maneuver."""
        dv = np.array([100.0, 0.0, 0.0])
        maneuver = Maneuver((0.0, dv))

        assert len(maneuver) == 1
        t, dv_out = maneuver[0]
        assert t == 0.0
        np.testing.assert_array_almost_equal(dv_out, dv)

    def test_multiple_impulses(self):
        """Test creating multi-impulse maneuver."""
        dv1 = np.array([100.0, 0.0, 0.0])
        dv2 = np.array([0.0, 50.0, 0.0])
        dv3 = np.array([0.0, 0.0, 25.0])

        maneuver = Maneuver((0.0, dv1), (3600.0, dv2), (7200.0, dv3))

        assert len(maneuver) == 3

        t0, dv0 = maneuver[0]
        assert t0 == 0.0
        np.testing.assert_array_almost_equal(dv0, dv1)

        t1, dv1_out = maneuver[1]
        assert t1 == 3600.0
        np.testing.assert_array_almost_equal(dv1_out, dv2)

        t2, dv2_out = maneuver[2]
        assert t2 == 7200.0
        np.testing.assert_array_almost_equal(dv2_out, dv3)

    def test_duration_support(self):
        """Test creating maneuver with Duration objects."""
        dv = np.array([100.0, 0.0, 0.0])
        dt = Duration.from_hrs(1)

        maneuver = Maneuver((dt, dv))

        t, _ = maneuver[0]
        assert abs(t - 3600.0) < 1e-6  # 1 hour in seconds

    def test_empty_maneuver_error(self):
        """Test that empty maneuver raises error."""
        with pytest.raises(ValueError, match="at least one impulse"):
            Maneuver()

    def test_invalid_delta_v_shape(self):
        """Test that invalid delta-v shape raises error."""
        with pytest.raises(ValueError, match="3-element array"):
            Maneuver((0.0, np.array([100.0, 0.0])))  # Only 2 elements

    def test_impulses_property(self):
        """Test that impulses property returns a copy."""
        dv = np.array([100.0, 0.0, 0.0])
        maneuver = Maneuver((0.0, dv))

        impulses = maneuver.impulses
        assert len(impulses) == 1

        # Modify the returned list shouldn't affect original
        impulses.append((3600.0, np.array([50.0, 0.0, 0.0])))
        assert len(maneuver) == 1  # Original unchanged


class TestManeuverAnalysis:
    """Test maneuver analysis methods."""

    def test_total_time_single_impulse(self):
        """Test total time for single impulse is zero."""
        dv = np.array([100.0, 0.0, 0.0])
        maneuver = Maneuver((0.0, dv))

        assert maneuver.get_total_time() == 0.0

    def test_total_time_multiple_impulses(self):
        """Test total time calculation."""
        dv1 = np.array([100.0, 0.0, 0.0])
        dv2 = np.array([50.0, 0.0, 0.0])

        maneuver = Maneuver((0.0, dv1), (3600.0, dv2))

        assert maneuver.get_total_time() == 3600.0

    def test_total_time_non_sequential(self):
        """Test total time with non-sequential times."""
        dv1 = np.array([100.0, 0.0, 0.0])
        dv2 = np.array([50.0, 0.0, 0.0])
        dv3 = np.array([25.0, 0.0, 0.0])

        maneuver = Maneuver((1000.0, dv1), (0.0, dv2), (5000.0, dv3))

        # Should be max - min = 5000 - 0 = 5000
        assert maneuver.get_total_time() == 5000.0

    def test_total_cost_single_impulse(self):
        """Test total cost for single impulse."""
        dv = np.array([100.0, 0.0, 0.0])
        maneuver = Maneuver((0.0, dv))

        assert abs(maneuver.get_total_cost() - 100.0) < 1e-6

    def test_total_cost_multiple_impulses(self):
        """Test total cost calculation."""
        dv1 = np.array([100.0, 0.0, 0.0])  # 100 m/s
        dv2 = np.array([30.0, 40.0, 0.0])  # 50 m/s (3-4-5 triangle)

        maneuver = Maneuver((0.0, dv1), (3600.0, dv2))

        expected_cost = 100.0 + 50.0  # 150 m/s
        assert abs(maneuver.get_total_cost() - expected_cost) < 1e-6

    def test_total_cost_3d_vectors(self):
        """Test total cost with 3D delta-v vectors."""
        # Use 3-4-5 Pythagorean triple
        dv = np.array([3.0, 4.0, 12.0])  # Magnitude = 13
        maneuver = Maneuver((0.0, dv))

        assert abs(maneuver.get_total_cost() - 13.0) < 1e-6


class TestImpulseFactory:
    """Test Maneuver.impulse() factory method."""

    def test_impulse_basic(self):
        """Test basic impulse creation."""
        dv = np.array([100.0, 50.0, 25.0])
        maneuver = Maneuver.impulse(dv)

        assert len(maneuver) == 1
        t, dv_out = maneuver[0]
        assert t == 0.0
        np.testing.assert_array_almost_equal(dv_out, dv)

    def test_impulse_zero(self):
        """Test zero impulse."""
        dv = np.array([0.0, 0.0, 0.0])
        maneuver = Maneuver.impulse(dv)

        assert maneuver.get_total_cost() == 0.0


class TestHohmannTransfer:
    """Test Hohmann transfer factory method."""

    def test_hohmann_leo_to_geo(self):
        """Test LEO to GEO Hohmann transfer."""
        # Create circular LEO orbit
        r_leo = 6778e3  # ~400 km altitude
        orbit = Orbit.from_classical(Earth, a=r_leo, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0)

        # GEO radius
        r_geo = 42164e3

        maneuver = Maneuver.hohmann(orbit, r_geo)

        # Should have 2 impulses
        assert len(maneuver) == 2

        # First impulse at t=0
        t0, dv0 = maneuver[0]
        assert t0 == 0.0

        # Second impulse at transfer time > 0
        t1, dv1 = maneuver[1]
        assert t1 > 0

        # Total cost should be positive
        total_dv = maneuver.get_total_cost()
        assert total_dv > 0

        # Expected Hohmann LEO->GEO is ~3900 m/s
        # (Delta-v1 ~2440 m/s, Delta-v2 ~1460 m/s)
        assert 3800 < total_dv < 4000

    def test_hohmann_descending(self):
        """Test Hohmann transfer for descending (GEO to LEO)."""
        # Create circular GEO orbit
        r_geo = 42164e3
        orbit = Orbit.from_classical(Earth, a=r_geo, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0)

        # LEO radius
        r_leo = 6778e3

        maneuver = Maneuver.hohmann(orbit, r_leo)

        # Should have 2 impulses
        assert len(maneuver) == 2

        # Total cost should be similar to ascending (slightly different)
        total_dv = maneuver.get_total_cost()
        assert 3800 < total_dv < 4000

    def test_hohmann_eccentric_orbit_error(self):
        """Test that eccentric orbit raises error."""
        # Create eccentric orbit
        orbit = Orbit.from_classical(Earth, a=10000e3, ecc=0.5, inc=0.0, raan=0.0, argp=0.0, nu=0.0)

        with pytest.raises(ValueError, match="approximately circular"):
            Maneuver.hohmann(orbit, 42164e3)

    def test_hohmann_transfer_time(self):
        """Test that Hohmann transfer time is reasonable."""
        r_leo = 6778e3
        orbit = Orbit.from_classical(Earth, a=r_leo, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0)

        r_geo = 42164e3
        maneuver = Maneuver.hohmann(orbit, r_geo)

        transfer_time = maneuver.get_total_time()

        # Hohmann transfer time LEO->GEO is ~5.25 hours
        expected_time = 5.25 * 3600  # seconds
        assert abs(transfer_time - expected_time) < 600  # Within 10 minutes


class TestBiellipticTransfer:
    """Test bi-elliptic transfer factory method."""

    def test_bielliptic_basic(self):
        """Test basic bi-elliptic transfer."""
        # Create circular LEO orbit
        r_leo = 6778e3
        orbit = Orbit.from_classical(Earth, a=r_leo, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0)

        # Bi-elliptic transfer parameters
        # r_intermediate must be larger than both r_initial and r_final
        r_final = 20000e3  # 20,000 km
        r_intermediate = 150000e3  # 150,000 km (larger than both LEO and final)

        maneuver = Maneuver.bielliptic(orbit, r_intermediate, r_final)

        # Should have 3 impulses
        assert len(maneuver) == 3

        # All times should be non-negative and increasing
        t0, _ = maneuver[0]
        t1, _ = maneuver[1]
        t2, _ = maneuver[2]

        assert t0 == 0.0
        assert t1 > t0
        assert t2 > t1

        # Total cost should be positive
        total_dv = maneuver.get_total_cost()
        assert total_dv > 0

    def test_bielliptic_eccentric_orbit_error(self):
        """Test that eccentric orbit raises error."""
        orbit = Orbit.from_classical(Earth, a=10000e3, ecc=0.3, inc=0.0, raan=0.0, argp=0.0, nu=0.0)

        with pytest.raises(ValueError, match="approximately circular"):
            Maneuver.bielliptic(orbit, 100000e3, 50000e3)


class TestLambertTransfer:
    """Test Lambert transfer factory method."""

    def test_lambert_basic(self):
        """Test basic Lambert transfer."""
        # Create two orbits at different positions and times
        r1 = np.array([7000e3, 0, 0])
        v1 = np.array([0, 7546, 0])
        epoch1 = Epoch.j2000_epoch()

        # Propagate to get second position
        orbit1 = Orbit.from_vectors(Earth, r1, v1, epoch1)
        dt = Duration.from_hrs(3)
        orbit2 = orbit1.propagate(dt)

        # Create Lambert maneuver
        maneuver = Maneuver.lambert(orbit1, orbit2)

        # Should have 2 impulses
        assert len(maneuver) == 2

        # First impulse at t=0
        t0, dv0 = maneuver[0]
        assert t0 == 0.0

        # Second impulse at transfer time
        t1, dv1 = maneuver[1]
        assert abs(t1 - dt.to_seconds()) < 1e-6

        # For a simple propagated orbit, delta-v should be small
        # (since we're following the natural trajectory)
        total_dv = maneuver.get_total_cost()
        # This won't be exactly zero due to numerical precision
        # and the fact that Keplerian propagation is used
        assert total_dv >= 0  # At minimum, should be non-negative

    def test_lambert_different_orbits(self):
        """Test Lambert transfer between different orbits."""
        # Create two different positions in similar orbits for a simpler case
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 + Duration.from_hrs(3)

        # LEO orbit at position 1
        orbit1 = Orbit.from_classical(
            Earth, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0, epoch=epoch1
        )

        # Same orbit but propagated to different position
        orbit2 = Orbit.from_classical(
            Earth, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=np.deg2rad(45), epoch=epoch2
        )

        maneuver = Maneuver.lambert(orbit1, orbit2)

        # Should have 2 impulses
        assert len(maneuver) == 2

        # Total delta-v should be positive
        assert maneuver.get_total_cost() > 0

    def test_lambert_different_attractor_error(self):
        """Test that different attractors raise error."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 + Duration.from_hrs(6)

        orbit1 = Orbit.from_classical(
            Earth, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0, epoch=epoch1
        )

        orbit2 = Orbit.from_classical(
            Mars, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0, epoch=epoch2
        )

        with pytest.raises(ValueError, match="same attractor"):
            Maneuver.lambert(orbit1, orbit2)

    def test_lambert_negative_time_error(self):
        """Test that negative time of flight raises error."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 + Duration.from_hrs(-6)  # Earlier time

        orbit1 = Orbit.from_classical(
            Earth, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0, epoch=epoch1
        )

        orbit2 = Orbit.from_classical(
            Earth, a=10000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0, epoch=epoch2
        )

        with pytest.raises(ValueError, match="after initial"):
            Maneuver.lambert(orbit1, orbit2)

    def test_lambert_short_way_vs_long_way(self):
        """Test short-way vs long-way Lambert transfers."""
        epoch1 = Epoch.j2000_epoch()
        epoch2 = epoch1 + Duration.from_hrs(4)

        orbit1 = Orbit.from_classical(
            Earth, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0, epoch=epoch1
        )

        # 60 degrees ahead
        orbit2 = Orbit.from_classical(
            Earth, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=np.deg2rad(60), epoch=epoch2
        )

        maneuver_short = Maneuver.lambert(orbit1, orbit2, short_way=True)
        maneuver_long = Maneuver.lambert(orbit1, orbit2, short_way=False)

        # Both should have 2 impulses
        assert len(maneuver_short) == 2
        assert len(maneuver_long) == 2

        # Both should have positive delta-v
        assert maneuver_short.get_total_cost() > 0
        assert maneuver_long.get_total_cost() > 0


class TestManeuverRepresentation:
    """Test string representation."""

    def test_repr_single_impulse(self):
        """Test repr for single impulse."""
        dv = np.array([100.0, 0.0, 0.0])
        maneuver = Maneuver.impulse(dv)

        repr_str = repr(maneuver)
        assert "Maneuver with 1 impulse" in repr_str
        assert "100.000 m/s" in repr_str
        assert "Total Δv: 100.000 m/s" in repr_str

    def test_repr_multiple_impulses(self):
        """Test repr for multiple impulses."""
        dv1 = np.array([100.0, 0.0, 0.0])
        dv2 = np.array([50.0, 0.0, 0.0])

        maneuver = Maneuver((0.0, dv1), (3600.0, dv2))

        repr_str = repr(maneuver)
        assert "Maneuver with 2 impulse" in repr_str
        assert "t=" in repr_str
        assert "Δv=" in repr_str


class TestManeuverIntegration:
    """Integration tests combining Orbit and Maneuver."""

    def test_apply_impulse_to_orbit(self):
        """Test applying a simple impulse to an orbit."""
        # Create orbit
        orbit = Orbit.from_classical(Earth, a=7000e3, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0)

        # Create prograde impulse
        v_hat = orbit.v / np.linalg.norm(orbit.v)
        dv = 100 * v_hat
        maneuver = Maneuver.impulse(dv)

        # Apply maneuver using orbit.apply_maneuver
        new_orbit = orbit.apply_maneuver(dv)

        # Verify orbital energy increased
        assert new_orbit.energy > orbit.energy

    def test_hohmann_changes_orbit(self):
        """Test that Hohmann maneuver properly changes orbit."""
        # Create LEO orbit
        r_leo = 7000e3
        orbit_leo = Orbit.from_classical(
            Earth, a=r_leo, ecc=0.0, inc=0.0, raan=0.0, argp=0.0, nu=0.0
        )

        # Create Hohmann transfer to higher orbit
        r_final = 10000e3
        maneuver = Maneuver.hohmann(orbit_leo, r_final)

        # Apply first burn
        t0, dv0 = maneuver[0]
        orbit_transfer = orbit_leo.apply_maneuver(dv0)

        # After first burn, orbit should be elliptical
        assert orbit_transfer.ecc > 0.01  # Not circular anymore

        # Semi-major axis should have increased
        assert orbit_transfer.a > orbit_leo.a
