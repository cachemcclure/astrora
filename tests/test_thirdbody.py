"""Test suite for third-body perturbations (Sun and Moon)"""

import numpy as np
import pytest
from astrora._core import (
    constants,
    moon_position_simple,
    propagate_state_keplerian,
    propagate_thirdbody_dopri5,
    propagate_thirdbody_rk4,
    sun_moon_perturbation,
    sun_position_simple,
    third_body_perturbation,
)


class TestSunEphemeris:
    """Tests for simplified Sun ephemeris model"""

    def test_sun_position_at_j2000(self):
        """Sun should be at 1 AU at J2000 epoch"""
        r_sun = sun_position_simple(0.0)

        # At J2000 (t=0), Sun should be at approximately [1 AU, 0, 0]
        assert r_sun.shape == (3,)
        assert abs(r_sun[0] - constants.AU) < 1e6  # Within 1000 km
        assert abs(r_sun[1]) < 1e9  # Small y-component
        assert abs(r_sun[2]) < 1.0  # Zero z-component (equatorial plane)

        # Distance should be 1 AU
        r_mag = np.linalg.norm(r_sun)
        assert abs(r_mag - constants.AU) < 1e6

    def test_sun_position_circular_orbit(self):
        """Sun should complete circular orbit in one year"""
        # One sidereal year in seconds
        year_sec = 365.25636 * 86400.0

        r_sun_0 = sun_position_simple(0.0)
        r_sun_1yr = sun_position_simple(year_sec)

        # After 1 year, should be back at starting position (within a few km)
        diff = r_sun_1yr - r_sun_0
        assert np.linalg.norm(diff) < 1e6  # Within 1000 km

    def test_sun_position_quarter_year(self):
        """After quarter year, Sun should be perpendicular"""
        year_sec = 365.25636 * 86400.0
        quarter_year = year_sec / 4.0

        r_sun_q = sun_position_simple(quarter_year)

        # Should be at [0, 1 AU, 0] approximately
        assert abs(r_sun_q[0]) < 1e9  # Small x-component
        assert abs(r_sun_q[1] - constants.AU) < 1e6  # At 1 AU in y-direction


class TestMoonEphemeris:
    """Tests for simplified Moon ephemeris model"""

    def test_moon_position_at_j2000(self):
        """Moon should be at mean distance at J2000"""
        r_moon = moon_position_simple(0.0)

        # Mean lunar distance: 384,400 km
        a_moon = 384_400_000.0

        assert r_moon.shape == (3,)
        assert abs(r_moon[0] - a_moon) < 1e3  # Within 1 km
        assert abs(r_moon[1]) < 1e3
        assert abs(r_moon[2]) < 1.0

        # Distance should be ~384,400 km
        r_mag = np.linalg.norm(r_moon)
        assert abs(r_mag - a_moon) < 1e3

    def test_moon_position_circular_orbit(self):
        """Moon should complete orbit in sidereal month"""
        # One sidereal month in seconds
        month_sec = 27.321661 * 86400.0

        r_moon_0 = moon_position_simple(0.0)
        r_moon_1m = moon_position_simple(month_sec)

        # After 1 month, should be back at starting position
        diff = r_moon_1m - r_moon_0
        assert np.linalg.norm(diff) < 1e4  # Within 10 km


class TestThirdBodyPerturbation:
    """Tests for third-body perturbation calculations"""

    def test_sun_perturbation_magnitude(self):
        """Sun perturbation should be ~10⁻⁶ m/s² at LEO"""
        r_sat = np.array([7000e3, 0.0, 0.0])  # LEO satellite
        r_sun = sun_position_simple(0.0)

        a_sun = third_body_perturbation(r_sat, r_sun, constants.GM_SUN)

        assert a_sun.shape == (3,)

        # Sun perturbation at LEO should be on order of 10⁻⁶ m/s²
        a_mag = np.linalg.norm(a_sun)
        assert a_mag > 1e-7
        assert a_mag < 3e-6

    def test_moon_perturbation_magnitude(self):
        """Moon perturbation should be ~10⁻⁶ m/s² at LEO"""
        r_sat = np.array([7000e3, 0.0, 0.0])
        r_moon = moon_position_simple(0.0)

        a_moon = third_body_perturbation(r_sat, r_moon, constants.GM_MOON)

        assert a_moon.shape == (3,)

        # Moon perturbation at LEO should be on order of 10⁻⁶ m/s²
        a_mag = np.linalg.norm(a_moon)
        assert a_mag > 1e-7
        assert a_mag < 5e-6

    def test_third_body_perturbation_geo(self):
        """At GEO, third-body effects should be measurable"""
        r_geo = np.array([42164e3, 0.0, 0.0])  # GEO altitude
        r_sun = sun_position_simple(0.0)

        a_sun = third_body_perturbation(r_geo, r_sun, constants.GM_SUN)

        # At GEO, Sun perturbation is significant
        a_mag = np.linalg.norm(a_sun)
        assert a_mag > 1e-7
        assert a_mag < 1e-4

    def test_sun_moon_combined(self):
        """Combined Sun+Moon perturbation equals sum"""
        r_sat = np.array([7000e3, 0.0, 0.0])
        t = 0.0

        # Get combined perturbation
        a_combined = sun_moon_perturbation(r_sat, t)

        # Compute individual perturbations
        r_sun = sun_position_simple(t)
        r_moon = moon_position_simple(t)
        a_sun = third_body_perturbation(r_sat, r_sun, constants.GM_SUN)
        a_moon = third_body_perturbation(r_sat, r_moon, constants.GM_MOON)
        a_manual = a_sun + a_moon

        # Should be identical
        assert np.allclose(a_combined, a_manual, rtol=1e-12)

    def test_third_body_time_dependence(self):
        """Perturbations should vary with time"""
        r_sat = np.array([7000e3, 0.0, 0.0])

        a0 = sun_moon_perturbation(r_sat, 0.0)
        a1 = sun_moon_perturbation(r_sat, 86400.0)  # 1 day later

        # Should be different as Sun and Moon move
        diff = np.linalg.norm(a1 - a0)
        assert diff > 1e-8  # Measurable difference


class TestThirdBodyPropagation:
    """Tests for orbit propagation with third-body perturbations"""

    def test_propagate_thirdbody_rk4_sun_only(self):
        """Propagate with Sun perturbation only"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0  # 10 minutes
        t0 = 0.0

        r1, v1 = propagate_thirdbody_rk4(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=True, include_moon=False, n_steps=10
        )

        # Orbit should still be valid
        assert np.linalg.norm(r1) > 6e6  # Still above Earth
        assert np.linalg.norm(v1) > 1000.0  # Still has velocity

    def test_propagate_thirdbody_rk4_moon_only(self):
        """Propagate with Moon perturbation only"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        t0 = 0.0

        r1, v1 = propagate_thirdbody_rk4(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=False, include_moon=True, n_steps=10
        )

        assert np.linalg.norm(r1) > 6e6
        assert np.linalg.norm(v1) > 1000.0

    def test_propagate_thirdbody_rk4_both(self):
        """Propagate with both Sun and Moon"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        t0 = 0.0

        r1, v1 = propagate_thirdbody_rk4(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=True, include_moon=True, n_steps=10
        )

        assert np.linalg.norm(r1) > 6e6
        assert np.linalg.norm(v1) > 1000.0

    def test_propagate_thirdbody_dopri5(self):
        """Adaptive integrator should work"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        t0 = 0.0

        r1, v1 = propagate_thirdbody_dopri5(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=True, include_moon=True, tol=1e-8
        )

        assert np.linalg.norm(r1) > 6e6
        assert np.linalg.norm(v1) > 1000.0

    def test_thirdbody_vs_twobody_difference(self):
        """Third-body effects should cause measurable differences"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 3600.0  # 1 hour
        t0 = 0.0

        # Two-body propagation
        r_twobody, v_twobody = propagate_state_keplerian(r0, v0, dt, constants.GM_EARTH)

        # Third-body propagation
        r_thirdbody, v_thirdbody = propagate_thirdbody_rk4(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=True, include_moon=True, n_steps=100
        )

        # Positions should differ
        pos_diff = np.linalg.norm(r_twobody - r_thirdbody)
        print(f"Position difference after 1 hour: {pos_diff:.2f} m")

        # At LEO over 1 hour, should see at least cm-level differences
        assert pos_diff > 0.1  # At least 10 cm
        assert pos_diff < 1000.0  # Less than 1 km

    def test_thirdbody_geo_significant(self):
        """At GEO, third-body effects are significant"""
        r0 = np.array([42164e3, 0.0, 0.0])  # GEO altitude
        v0 = np.array([0.0, 3075.0, 0.0])  # GEO velocity
        dt = 86400.0  # 1 day
        t0 = 0.0

        # Two-body propagation
        r_twobody, v_twobody = propagate_state_keplerian(r0, v0, dt, constants.GM_EARTH)

        # Third-body propagation
        r_thirdbody, v_thirdbody = propagate_thirdbody_rk4(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=True, include_moon=True, n_steps=1000
        )

        # At GEO over 1 day, difference should be significant
        pos_diff = np.linalg.norm(r_twobody - r_thirdbody)
        print(f"GEO position difference after 1 day: {pos_diff:.2f} m")

        # Should see meters to kilometers of difference
        assert pos_diff > 100.0  # At least 100 m
        assert pos_diff < 1e6  # Less than 1000 km

    def test_rk4_vs_dopri5_agreement(self):
        """RK4 and DOPRI5 should give similar results"""
        r0 = np.array([7000e3, 0.0, 0.0])
        v0 = np.array([0.0, 7546.0, 0.0])
        dt = 600.0
        t0 = 0.0

        # RK4 with many steps
        r_rk4, v_rk4 = propagate_thirdbody_rk4(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=True, include_moon=True, n_steps=100
        )

        # DOPRI5 with tight tolerance
        r_dopri5, v_dopri5 = propagate_thirdbody_dopri5(
            r0, v0, dt, constants.GM_EARTH, t0, include_sun=True, include_moon=True, tol=1e-10
        )

        # Should agree to within reasonable tolerance
        pos_diff = np.linalg.norm(r_rk4 - r_dopri5)
        vel_diff = np.linalg.norm(v_rk4 - v_dopri5)

        assert pos_diff < 100.0  # Within 100 m
        assert vel_diff < 0.1  # Within 0.1 m/s


class TestEdgeCases:
    """Test edge cases and error handling"""

    def test_third_body_perturbation_wrong_shape(self):
        """Should reject vectors with wrong dimensions"""
        r_sat = np.array([7000e3, 0.0])  # Only 2 components
        r_sun = sun_position_simple(0.0)

        with pytest.raises(ValueError):
            third_body_perturbation(r_sat, r_sun, constants.GM_SUN)

    def test_propagate_thirdbody_wrong_shape(self):
        """Should reject state vectors with wrong dimensions"""
        r0 = np.array([7000e3, 0.0])  # Only 2 components
        v0 = np.array([0.0, 7546.0, 0.0])

        with pytest.raises(ValueError):
            propagate_thirdbody_rk4(
                r0, v0, 600.0, constants.GM_EARTH, 0.0, include_sun=True, include_moon=True
            )


if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
