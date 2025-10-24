"""
Tests for astropy.coordinates integration

This module tests bidirectional conversion between astrora coordinate frames
and astropy.coordinates, including:
- Frame conversions (ICRS, GCRS, ITRS)
- SkyCoord integration
- Orbit class integration
- Precision and roundtrip accuracy
"""

import numpy as np
import pytest
from numpy.testing import assert_allclose

# Test if astropy is available
try:
    from astropy import units as u
    from astropy.coordinates import (
        GCRS as AstropyGCRS,
    )
    from astropy.coordinates import (
        ICRS as AstropyICRS,
    )
    from astropy.coordinates import (
        ITRS as AstropyITRS,
    )
    from astropy.coordinates import (
        SkyCoord,
    )
    from astropy.time import Time

    ASTROPY_AVAILABLE = True
except ImportError:
    ASTROPY_AVAILABLE = False

# Astrora imports
from astrora._core import (
    GCRS as AstroraGCRS,
)
from astrora._core import (
    ICRS as AstroraICRS,
)
from astrora._core import (
    ITRS as AstroraITRS,
)
from astrora._core import (
    Epoch,
)
from astrora.bodies import Earth
from astrora.twobody import Orbit

# Skip all tests if astropy not available
pytestmark = pytest.mark.skipif(not ASTROPY_AVAILABLE, reason="astropy not installed")


# ============================================================================
# Frame Conversion Tests
# ============================================================================


class TestFrameConversions:
    """Test conversions between astrora and astropy coordinate frames."""

    def test_icrs_to_astropy(self):
        """Test conversion from astrora ICRS to astropy ICRS."""
        from astrora.coordinates import to_astropy_coord

        # Create astrora ICRS frame (1 AU from barycenter)
        pos = np.array([1.496e11, 0.0, 0.0])  # meters
        vel = np.array([0.0, 29780.0, 0.0])  # m/s
        astrora_frame = AstroraICRS(pos, vel)

        # Convert to astropy
        astropy_frame = to_astropy_coord(astrora_frame)

        # Check type
        assert isinstance(astropy_frame, AstropyICRS)

        # Check position (convert m → km)
        assert_allclose(astropy_frame.cartesian.x.to(u.km).value, pos[0] / 1000.0, rtol=1e-10)
        assert_allclose(astropy_frame.cartesian.y.to(u.km).value, pos[1] / 1000.0, rtol=1e-10)
        assert_allclose(astropy_frame.cartesian.z.to(u.km).value, pos[2] / 1000.0, rtol=1e-10)

        # Check velocity (convert m/s → km/s)
        velocity_data = astropy_frame.cartesian.differentials["s"]
        assert_allclose(velocity_data.d_x.to(u.km / u.s).value, vel[0] / 1000.0, rtol=1e-10)
        assert_allclose(velocity_data.d_y.to(u.km / u.s).value, vel[1] / 1000.0, rtol=1e-10)
        assert_allclose(velocity_data.d_z.to(u.km / u.s).value, vel[2] / 1000.0, rtol=1e-10)

    def test_gcrs_to_astropy(self):
        """Test conversion from astrora GCRS to astropy GCRS."""
        from astrora.coordinates import to_astropy_coord

        # Create astrora GCRS frame (LEO orbit)
        pos = np.array([7000e3, 0.0, 0.0])  # meters
        vel = np.array([0.0, 7500.0, 0.0])  # m/s

        # Create epoch for GCRS frame
        epoch = Epoch.from_noon_utc(2024, 1, 1)
        astrora_frame = AstroraGCRS(pos, vel, epoch)

        # Observation time for comparison
        obstime = Time("2024-01-01 12:00:00", scale="utc")

        # Convert to astropy
        astropy_frame = to_astropy_coord(astrora_frame, obstime=obstime)

        # Check type
        assert isinstance(astropy_frame, AstropyGCRS)

        # Check obstime
        assert astropy_frame.obstime == obstime

        # Check position
        assert_allclose(astropy_frame.cartesian.x.to(u.km).value, 7000.0, rtol=1e-10)
        assert_allclose(astropy_frame.cartesian.y.to(u.km).value, 0.0, atol=1e-10)
        assert_allclose(astropy_frame.cartesian.z.to(u.km).value, 0.0, atol=1e-10)

        # Check velocity
        velocity_data = astropy_frame.cartesian.differentials["s"]
        assert_allclose(velocity_data.d_x.to(u.km / u.s).value, 0.0, atol=1e-10)
        assert_allclose(velocity_data.d_y.to(u.km / u.s).value, 7.5, rtol=1e-10)
        assert_allclose(velocity_data.d_z.to(u.km / u.s).value, 0.0, atol=1e-10)

    def test_itrs_to_astropy(self):
        """Test conversion from astrora ITRS to astropy ITRS."""
        from astrora.coordinates import to_astropy_coord

        # Create astrora ITRS frame (ground station)
        pos = np.array([6371e3, 0.0, 0.0])  # meters (on equator)
        vel = np.array([0.0, 0.0, 0.0])  # m/s (stationary in ITRS)

        # Create epoch for ITRS frame
        epoch = Epoch.from_noon_utc(2024, 1, 1)
        astrora_frame = AstroraITRS(pos, vel, epoch)

        # Observation time for comparison
        obstime = Time("2024-01-01 12:00:00", scale="utc")

        # Convert to astropy
        astropy_frame = to_astropy_coord(astrora_frame, obstime=obstime)

        # Check type
        assert isinstance(astropy_frame, AstropyITRS)

        # Check obstime
        assert astropy_frame.obstime == obstime

        # Check position
        assert_allclose(astropy_frame.cartesian.x.to(u.km).value, 6371.0, rtol=1e-10)

    def test_astropy_to_astrora_icrs(self):
        """Test conversion from astropy ICRS to astrora ICRS."""
        from astrora.coordinates import from_astropy_coord

        # Create astropy ICRS
        astropy_frame = AstropyICRS(
            x=1.496e8 * u.km,
            y=0 * u.km,
            z=0 * u.km,
            v_x=0 * u.km / u.s,
            v_y=29.78 * u.km / u.s,
            v_z=0 * u.km / u.s,
            representation_type="cartesian",
            differential_type="cartesian",
        )

        # Convert to astrora
        astrora_frame = from_astropy_coord(astropy_frame)

        # Check type
        assert isinstance(astrora_frame, AstroraICRS)

        # Check position (km → m)
        assert_allclose(astrora_frame.position[0], 1.496e11, rtol=1e-10)
        assert_allclose(astrora_frame.position[1], 0.0, atol=1e-6)
        assert_allclose(astrora_frame.position[2], 0.0, atol=1e-6)

        # Check velocity (km/s → m/s)
        assert_allclose(astrora_frame.velocity[0], 0.0, atol=1e-6)
        assert_allclose(astrora_frame.velocity[1], 29780.0, rtol=1e-10)
        assert_allclose(astrora_frame.velocity[2], 0.0, atol=1e-6)

    def test_astropy_to_astrora_gcrs(self):
        """Test conversion from astropy GCRS to astrora GCRS."""
        from astrora.coordinates import from_astropy_coord

        # Create astropy GCRS
        astropy_frame = AstropyGCRS(
            x=7000 * u.km,
            y=0 * u.km,
            z=0 * u.km,
            v_x=0 * u.km / u.s,
            v_y=7.5 * u.km / u.s,
            v_z=0 * u.km / u.s,
            representation_type="cartesian",
            differential_type="cartesian",
            obstime=Time("2024-01-01"),
        )

        # Convert to astrora
        astrora_frame = from_astropy_coord(astropy_frame)

        # Check type
        assert isinstance(astrora_frame, AstroraGCRS)

        # Check position
        assert_allclose(astrora_frame.position[0], 7000e3, rtol=1e-10)
        assert_allclose(astrora_frame.position[1], 0.0, atol=1e-6)
        assert_allclose(astrora_frame.position[2], 0.0, atol=1e-6)

        # Check velocity
        assert_allclose(astrora_frame.velocity[0], 0.0, atol=1e-6)
        assert_allclose(astrora_frame.velocity[1], 7500.0, rtol=1e-10)
        assert_allclose(astrora_frame.velocity[2], 0.0, atol=1e-6)


# ============================================================================
# SkyCoord Integration Tests
# ============================================================================


class TestSkyCoordIntegration:
    """Test SkyCoord integration with astrora frames."""

    def test_to_skycoord_icrs(self):
        """Test conversion from astrora frame to SkyCoord (ICRS)."""
        from astrora.coordinates import to_skycoord

        # Create astrora ICRS frame
        pos = np.array([1.496e11, 0.0, 0.0])
        vel = np.array([0.0, 29780.0, 0.0])
        astrora_frame = AstroraICRS(pos, vel)

        # Convert to SkyCoord
        sc = to_skycoord(astrora_frame)

        # Check type
        assert isinstance(sc, SkyCoord)

        # Check frame type
        assert isinstance(sc.frame, AstropyICRS)

        # Check distance via cartesian representation (should be ~1 AU)
        distance_m = np.linalg.norm(
            [
                sc.cartesian.x.to(u.m).value,
                sc.cartesian.y.to(u.m).value,
                sc.cartesian.z.to(u.m).value,
            ]
        )
        distance_au = distance_m / 1.496e11
        assert_allclose(distance_au, 1.0, rtol=1e-6)

    def test_to_skycoord_gcrs(self):
        """Test conversion from astrora frame to SkyCoord (GCRS)."""
        from astrora.coordinates import to_skycoord

        # Create astrora GCRS frame
        pos = np.array([7000e3, 0.0, 0.0])
        vel = np.array([0.0, 7500.0, 0.0])
        epoch = Epoch.from_noon_utc(2024, 1, 1)
        astrora_frame = AstroraGCRS(pos, vel, epoch)

        # Observation time for comparison
        obstime = Time("2024-01-01 12:00:00", scale="utc")

        # Convert to SkyCoord
        sc = to_skycoord(astrora_frame, obstime=obstime)

        # Check type
        assert isinstance(sc, SkyCoord)

        # Check frame type
        assert isinstance(sc.frame, AstropyGCRS)

        # Check obstime
        assert sc.obstime == obstime

        # Check distance via cartesian representation (should be 7000 km)
        distance_m = np.linalg.norm(
            [
                sc.cartesian.x.to(u.m).value,
                sc.cartesian.y.to(u.m).value,
                sc.cartesian.z.to(u.m).value,
            ]
        )
        distance_km = distance_m / 1000.0
        assert_allclose(distance_km, 7000.0, rtol=1e-6)

    def test_from_skycoord_icrs(self):
        """Test conversion from SkyCoord to astrora frame (ICRS)."""
        from astrora.coordinates import from_skycoord

        # Create SkyCoord in ICRS
        sc = SkyCoord(
            x=1.496e8 * u.km,
            y=0 * u.km,
            z=0 * u.km,
            v_x=0 * u.km / u.s,
            v_y=29.78 * u.km / u.s,
            v_z=0 * u.km / u.s,
            representation_type="cartesian",
            frame="icrs",
        )

        # Convert to astrora
        astrora_frame = from_skycoord(sc)

        # Check type
        assert isinstance(astrora_frame, AstroraICRS)

        # Check position
        assert_allclose(astrora_frame.position[0], 1.496e11, rtol=1e-10)

        # Check velocity
        assert_allclose(astrora_frame.velocity[1], 29780.0, rtol=1e-10)

    def test_from_skycoord_gcrs(self):
        """Test conversion from SkyCoord to astrora frame (GCRS)."""
        from astrora.coordinates import from_skycoord

        # Create SkyCoord in GCRS
        sc = SkyCoord(
            x=7000 * u.km,
            y=0 * u.km,
            z=0 * u.km,
            v_x=0 * u.km / u.s,
            v_y=7.5 * u.km / u.s,
            v_z=0 * u.km / u.s,
            representation_type="cartesian",
            frame="gcrs",
            obstime=Time("2024-01-01"),
        )

        # Convert to astrora
        astrora_frame = from_skycoord(sc)

        # Check type
        assert isinstance(astrora_frame, AstroraGCRS)

        # Check position
        assert_allclose(astrora_frame.position[0], 7000e3, rtol=1e-10)

        # Check velocity
        assert_allclose(astrora_frame.velocity[1], 7500.0, rtol=1e-10)


# ============================================================================
# Orbit Integration Tests
# ============================================================================


class TestOrbitIntegration:
    """Test Orbit class integration with astropy coordinates."""

    def test_orbit_to_skycoord(self):
        """Test Orbit.to_skycoord() method."""
        # Create orbit
        r = np.array([7000e3, 0.0, 0.0])
        v = np.array([0.0, 7500.0, 0.0])
        epoch = Epoch.from_noon_utc(2024, 1, 1)
        orbit = Orbit.from_vectors(Earth, r, v, epoch=epoch)

        # Convert to SkyCoord
        sc = orbit.to_skycoord()

        # Check type
        assert isinstance(sc, SkyCoord)

        # Check frame type (should be GCRS by default)
        assert isinstance(sc.frame, AstropyGCRS)

        # Check distance via cartesian representation
        distance_m = np.linalg.norm(
            [
                sc.cartesian.x.to(u.m).value,
                sc.cartesian.y.to(u.m).value,
                sc.cartesian.z.to(u.m).value,
            ]
        )
        distance_km = distance_m / 1000.0
        assert_allclose(distance_km, 7000.0, rtol=1e-6)

    def test_orbit_to_skycoord_icrs(self):
        """Test Orbit.to_skycoord() with ICRS frame."""
        # Create orbit
        r = np.array([7000e3, 0.0, 0.0])
        v = np.array([0.0, 7500.0, 0.0])
        orbit = Orbit.from_vectors(Earth, r, v)

        # Convert to SkyCoord in ICRS frame
        sc = orbit.to_skycoord(frame="icrs")

        # Check frame type
        assert isinstance(sc.frame, AstropyICRS)

    def test_orbit_to_skycoord_itrs(self):
        """Test Orbit.to_skycoord() with ITRS frame."""
        # Create orbit
        r = np.array([7000e3, 0.0, 0.0])
        v = np.array([0.0, 7500.0, 0.0])
        epoch = Epoch.from_noon_utc(2024, 1, 1)
        orbit = Orbit.from_vectors(Earth, r, v, epoch=epoch)

        # Convert to SkyCoord in ITRS frame
        sc = orbit.to_skycoord(frame="itrs")

        # Check frame type
        assert isinstance(sc.frame, AstropyITRS)

    def test_orbit_from_skycoord(self):
        """Test Orbit.from_skycoord() classmethod."""
        # Create SkyCoord
        sc = SkyCoord(
            x=7000 * u.km,
            y=0 * u.km,
            z=0 * u.km,
            v_x=0 * u.km / u.s,
            v_y=7.5 * u.km / u.s,
            v_z=0 * u.km / u.s,
            representation_type="cartesian",
            frame="gcrs",
            obstime=Time("2024-01-01 12:00:00"),
        )

        # Create orbit from SkyCoord
        orbit = Orbit.from_skycoord(sc, Earth)

        # Check type
        assert isinstance(orbit, Orbit)

        # Check position (orbit.r returns Quantity, get value in meters)
        assert_allclose(orbit.r[0].to(u.m).value, 7000e3, rtol=1e-10)

        # Check velocity (orbit.v returns Quantity, get value in m/s)
        assert_allclose(orbit.v[1].to(u.m / u.s).value, 7500.0, rtol=1e-10)

        # Check attractor
        assert orbit.attractor == Earth

    def test_orbit_to_astropy_coord(self):
        """Test Orbit.to_astropy_coord() method."""
        # Create orbit
        r = np.array([7000e3, 0.0, 0.0])
        v = np.array([0.0, 7500.0, 0.0])
        epoch = Epoch.from_noon_utc(2024, 1, 1)
        orbit = Orbit.from_vectors(Earth, r, v, epoch=epoch)

        # Convert to astropy GCRS
        gcrs = orbit.to_astropy_coord(frame="gcrs")

        # Check type
        assert isinstance(gcrs, AstropyGCRS)

        # Check position
        assert_allclose(gcrs.cartesian.x.to(u.km).value, 7000.0, rtol=1e-10)

    def test_orbit_from_astropy_coord(self):
        """Test Orbit.from_astropy_coord() classmethod."""
        # Create astropy GCRS
        gcrs = AstropyGCRS(
            x=7000 * u.km,
            y=0 * u.km,
            z=0 * u.km,
            v_x=0 * u.km / u.s,
            v_y=7.5 * u.km / u.s,
            v_z=0 * u.km / u.s,
            representation_type="cartesian",
            differential_type="cartesian",
            obstime=Time("2024-01-01 12:00:00"),
        )

        # Create orbit from astropy coord
        orbit = Orbit.from_astropy_coord(gcrs, Earth)

        # Check type
        assert isinstance(orbit, Orbit)

        # Check position (orbit.r returns Quantity, get value in meters)
        assert_allclose(orbit.r[0].to(u.m).value, 7000e3, rtol=1e-10)

        # Check velocity (orbit.v returns Quantity, get value in m/s)
        assert_allclose(orbit.v[1].to(u.m / u.s).value, 7500.0, rtol=1e-10)


# ============================================================================
# Roundtrip Tests
# ============================================================================


class TestRoundtrip:
    """Test roundtrip conversions for precision."""

    def test_roundtrip_icrs(self):
        """Test roundtrip: astrora ICRS → astropy → astrora."""
        from astrora.coordinates import from_astropy_coord, to_astropy_coord

        # Original astrora frame
        pos_orig = np.array([1.496e11, 1e9, 1e8])
        vel_orig = np.array([100.0, 29780.0, 50.0])
        astrora_orig = AstroraICRS(pos_orig, vel_orig)

        # Convert to astropy and back
        astropy_frame = to_astropy_coord(astrora_orig)
        astrora_roundtrip = from_astropy_coord(astropy_frame)

        # Check precision
        assert_allclose(astrora_roundtrip.position, pos_orig, rtol=1e-10)
        assert_allclose(astrora_roundtrip.velocity, vel_orig, rtol=1e-10)

    def test_roundtrip_gcrs(self):
        """Test roundtrip: astrora GCRS → astropy → astrora."""
        from astrora.coordinates import from_astropy_coord, to_astropy_coord

        # Original astrora frame
        pos_orig = np.array([7000e3, 1000e3, 500e3])
        vel_orig = np.array([100.0, 7500.0, 50.0])
        epoch = Epoch.from_noon_utc(2024, 1, 1)
        astrora_orig = AstroraGCRS(pos_orig, vel_orig, epoch)

        # Observation time for comparison
        obstime = Time("2024-01-01 12:00:00", scale="utc")

        # Convert to astropy and back
        astropy_frame = to_astropy_coord(astrora_orig, obstime=obstime)
        astrora_roundtrip = from_astropy_coord(astropy_frame)

        # Check precision
        assert_allclose(astrora_roundtrip.position, pos_orig, rtol=1e-10)
        assert_allclose(astrora_roundtrip.velocity, vel_orig, rtol=1e-10)

    def test_roundtrip_orbit_skycoord(self):
        """Test roundtrip: Orbit → SkyCoord → Orbit."""
        # Create original orbit
        r_orig = np.array([7000e3, 1000e3, 500e3])
        v_orig = np.array([100.0, 7500.0, 50.0])
        epoch_orig = Epoch.from_noon_utc(2024, 1, 1)
        orbit_orig = Orbit.from_vectors(Earth, r_orig, v_orig, epoch=epoch_orig)

        # Convert to SkyCoord and back
        sc = orbit_orig.to_skycoord()
        orbit_roundtrip = Orbit.from_skycoord(sc, Earth)

        # Check position precision (orbit.r returns Quantity, get value in meters)
        assert_allclose(orbit_roundtrip.r.to(u.m).value, r_orig, rtol=1e-10)

        # Check velocity precision (orbit.v returns Quantity, get value in m/s)
        assert_allclose(orbit_roundtrip.v.to(u.m / u.s).value, v_orig, rtol=1e-10)


# ============================================================================
# Edge Cases
# ============================================================================


class TestEdgeCases:
    """Test edge cases and error handling."""

    def test_invalid_frame_type(self):
        """Test error handling for unsupported frame types."""
        from astrora.coordinates import to_astropy_coord

        # Create a mock object (not a valid frame)
        class MockFrame:
            position = np.array([1.0, 2.0, 3.0])
            velocity = np.array([0.0, 0.0, 0.0])

        mock = MockFrame()

        # Should raise TypeError
        with pytest.raises(TypeError):
            to_astropy_coord(mock)

    def test_default_obstime(self):
        """Test that default obstime is J2000.0."""
        from astrora.coordinates import to_astropy_coord

        # Create astrora GCRS frame
        pos = np.array([7000e3, 0.0, 0.0])
        vel = np.array([0.0, 7500.0, 0.0])
        epoch = Epoch.j2000_epoch()
        astrora_frame = AstroraGCRS(pos, vel, epoch)

        # Convert without obstime
        astropy_frame = to_astropy_coord(astrora_frame)

        # Check that default obstime is J2000.0
        assert astropy_frame.obstime.scale == "tt"
        assert_allclose(astropy_frame.obstime.jd, 2451545.0, rtol=1e-10)

    def test_zero_velocity(self):
        """Test conversion with zero velocity."""
        from astrora.coordinates import from_astropy_coord, to_astropy_coord

        # Create frame with zero velocity
        pos = np.array([7000e3, 0.0, 0.0])
        vel = np.array([0.0, 0.0, 0.0])
        epoch = Epoch.j2000_epoch()
        astrora_frame = AstroraGCRS(pos, vel, epoch)

        # Convert to astropy and back
        astropy_frame = to_astropy_coord(astrora_frame)
        astrora_roundtrip = from_astropy_coord(astropy_frame)

        # Check velocity is preserved as zero
        assert_allclose(astrora_roundtrip.velocity, vel, atol=1e-10)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
