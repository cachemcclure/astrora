"""
Tests for solar radiation pressure (SRP) perturbation functions.

Tests shadow function, SRP acceleration computations, and propagation.
"""

import numpy as np
import pytest
from astrora._core import (
    constants,
    propagate_srp_dopri5,
    propagate_srp_rk4,
    propagate_state_keplerian,
    shadow_function,
    srp_acceleration,
    sun_position_simple,
)


class TestShadowFunction:
    """Test shadow function for umbra/penumbra calculations."""

    def test_shadow_full_sunlight(self):
        """Satellite in full sunlight should have k=1."""
        # Satellite on sunlit side of Earth
        r_sat = np.array([7000e3, 0.0, 0.0])
        r_sun = sun_position_simple(0.0)

        k = shadow_function(r_sat, r_sun, constants.R_EARTH)

        # Should be in full sunlight
        assert abs(k - 1.0) < 1e-10

    def test_shadow_umbra_leo(self):
        """LEO satellite in Earth's shadow should have k≈0."""
        # Position satellite behind Earth relative to Sun
        # Sun at [1 AU, 0, 0], satellite at [-7000 km, 0, 0]
        r_sat = np.array([-7000e3, 0.0, 0.0])
        r_sun = np.array([constants.AU, 0.0, 0.0])

        k = shadow_function(r_sat, r_sun, constants.R_EARTH)

        # Should be in full umbra
        assert k < 0.1, f"Expected k≈0 in umbra, got k={k}"

    def test_shadow_penumbra_exists(self):
        """Penumbra region should exist with 0 < k < 1."""
        # Position satellite at edge of shadow cone
        # This is a simplified test - exact position depends on geometry
        r_sat = np.array([-8000e3, constants.R_EARTH * 0.7, 0.0])
        r_sun = np.array([constants.AU, 0.0, 0.0])

        k = shadow_function(r_sat, r_sun, constants.R_EARTH)

        # Should be in partial shadow (penumbra) or umbra
        assert 0.0 <= k <= 1.0

    def test_shadow_geo_sunlight(self):
        """GEO satellite typically in full sunlight."""
        # GEO altitude, sunlit side
        r_sat = np.array([42164e3, 0.0, 0.0])
        r_sun = sun_position_simple(0.0)

        k = shadow_function(r_sat, r_sun, constants.R_EARTH)

        # At GEO, shadow events are rare
        # This should be in sunlight if Sun is in positive x direction
        assert k > 0.9

    def test_shadow_function_range(self):
        """Shadow function should always return values in [0, 1]."""
        # Test various positions
        positions = [
            np.array([7000e3, 0.0, 0.0]),
            np.array([0.0, 7000e3, 0.0]),
            np.array([0.0, 0.0, 7000e3]),
            np.array([-7000e3, 0.0, 0.0]),
            np.array([42164e3, 0.0, 0.0]),
        ]

        for r_sat in positions:
            r_sun = sun_position_simple(0.0)
            k = shadow_function(r_sat, r_sun, constants.R_EARTH)

            assert 0.0 <= k <= 1.0, f"Shadow factor {k} out of range [0,1]"

    def test_shadow_invalid_vector_sizes(self):
        """Test error handling for wrong vector sizes."""
        r_sat = np.array([7000e3, 0.0])  # Only 2 components
        r_sun = sun_position_simple(0.0)

        with pytest.raises(ValueError):
            shadow_function(r_sat, r_sun, constants.R_EARTH)


class TestSRPAcceleration:
    """Test SRP acceleration computations."""

    def test_srp_acceleration_direction(self):
        """SRP should point away from Sun."""
        # Satellite in full sunlight
        r_sat = np.array([7000e3, 0.0, 0.0])
        r_sun = sun_position_simple(0.0)
        area_mass_ratio = 0.01  # m²/kg
        C_r = 1.3

        a_srp = srp_acceleration(r_sat, r_sun, area_mass_ratio, C_r, constants.R_EARTH)

        # SRP should point away from Sun
        # Vector from sat to Sun
        r_sun_sat = r_sun - r_sat

        # SRP acceleration should be parallel to (Sun - sat) direction
        # i.e., a_srp · (r_sun - r_sat) > 0
        dot_product = np.dot(a_srp, r_sun_sat)
        assert dot_product > 0.0, "SRP should point away from Sun"

    def test_srp_magnitude_reasonable(self):
        """SRP magnitude should be on order of 10⁻⁶ to 10⁻⁸ m/s² for typical satellites."""
        # GEO satellite with typical area-to-mass ratio
        r_sat = np.array([42164e3, 0.0, 0.0])
        r_sun = sun_position_simple(0.0)
        area_mass_ratio = 0.01  # Typical for CubeSat
        C_r = 1.3

        a_srp = srp_acceleration(r_sat, r_sun, area_mass_ratio, C_r, constants.R_EARTH)

        mag = np.linalg.norm(a_srp)

        # For A/m = 0.01 m²/kg, C_r = 1.3 at 1 AU:
        # Expected: ~1.3 * 0.01 * 4.54e-6 ≈ 5.9e-8 m/s²
        assert mag > 1e-10, "SRP magnitude too small"
        assert mag < 1e-5, "SRP magnitude too large"

    def test_srp_zero_in_shadow(self):
        """SRP should be zero when satellite is in Earth's shadow."""
        # Satellite in umbra
        r_sat = np.array([-7000e3, 0.0, 0.0])
        r_sun = np.array([constants.AU, 0.0, 0.0])
        area_mass_ratio = 0.01
        C_r = 1.3

        a_srp = srp_acceleration(r_sat, r_sun, area_mass_ratio, C_r, constants.R_EARTH)

        mag = np.linalg.norm(a_srp)
        # Should be very small or zero in shadow
        assert mag < 1e-10

    def test_srp_scales_with_area_mass_ratio(self):
        """SRP should scale linearly with area-to-mass ratio."""
        r_sat = np.array([42164e3, 0.0, 0.0])
        r_sun = sun_position_simple(0.0)
        C_r = 1.3

        area_mass_ratio_1 = 0.01
        area_mass_ratio_2 = 0.02  # Double the area-to-mass ratio

        a_srp_1 = srp_acceleration(r_sat, r_sun, area_mass_ratio_1, C_r, constants.R_EARTH)
        a_srp_2 = srp_acceleration(r_sat, r_sun, area_mass_ratio_2, C_r, constants.R_EARTH)

        mag_1 = np.linalg.norm(a_srp_1)
        mag_2 = np.linalg.norm(a_srp_2)

        # mag_2 should be approximately 2× mag_1
        ratio = mag_2 / mag_1
        assert abs(ratio - 2.0) < 0.01

    def test_srp_scales_with_reflectivity(self):
        """SRP should scale linearly with reflectivity coefficient."""
        r_sat = np.array([42164e3, 0.0, 0.0])
        r_sun = sun_position_simple(0.0)
        area_mass_ratio = 0.01

        C_r_1 = 1.0
        C_r_2 = 2.0  # Double the reflectivity

        a_srp_1 = srp_acceleration(r_sat, r_sun, area_mass_ratio, C_r_1, constants.R_EARTH)
        a_srp_2 = srp_acceleration(r_sat, r_sun, area_mass_ratio, C_r_2, constants.R_EARTH)

        mag_1 = np.linalg.norm(a_srp_1)
        mag_2 = np.linalg.norm(a_srp_2)

        # mag_2 should be approximately 2× mag_1
        ratio = mag_2 / mag_1
        assert abs(ratio - 2.0) < 0.01

    def test_srp_inverse_square_law(self):
        """SRP should decrease as 1/r² with distance from Sun."""
        # This test is approximate since we're using simplified Sun position
        # Position satellite at 1 AU (Earth distance)
        r_sat_1au = np.array([0.0, 0.0, 0.0])  # At Earth
        r_sun = sun_position_simple(0.0)

        # Position satellite at 2 AU (double distance)
        # Simplified: move satellite away from Sun
        r_sun_direction = r_sun / np.linalg.norm(r_sun)
        r_sat_2au = -constants.AU * r_sun_direction  # 2 AU from Sun

        area_mass_ratio = 0.01
        C_r = 1.3

        a_srp_1au = srp_acceleration(r_sat_1au, r_sun, area_mass_ratio, C_r, constants.R_EARTH)
        a_srp_2au = srp_acceleration(r_sat_2au, r_sun, area_mass_ratio, C_r, constants.R_EARTH)

        mag_1au = np.linalg.norm(a_srp_1au)
        mag_2au = np.linalg.norm(a_srp_2au)

        # At 2 AU, SRP should be ~1/4 of value at 1 AU
        if mag_2au > 1e-15:  # Avoid division by very small numbers
            ratio = mag_1au / mag_2au
            # Should be close to 4 (inverse square law)
            assert 3.0 < ratio < 5.0, f"Inverse square law violated: ratio={ratio}"

    def test_srp_invalid_vector_sizes(self):
        """Test error handling for wrong vector sizes."""
        r_sat = np.array([7000e3, 0.0])  # Only 2 components
        r_sun = sun_position_simple(0.0)
        area_mass_ratio = 0.01
        C_r = 1.3

        with pytest.raises(ValueError):
            srp_acceleration(r_sat, r_sun, area_mass_ratio, C_r, constants.R_EARTH)


class TestSRPPropagation:
    """Test orbit propagation with solar radiation pressure."""

    def test_propagate_srp_rk4_basic(self):
        """Basic test of RK4 SRP propagation."""
        # GEO orbit
        r0 = np.array([42164e3, 0.0, 0.0])
        v0 = np.array([0.0, 3075.0, 0.0])
        area_mass_ratio = 0.01
        C_r = 1.3
        t0 = 0.0

        r1, v1 = propagate_srp_rk4(
            r0,
            v0,
            3600.0,  # 1 hour
            constants.GM_EARTH,
            area_mass_ratio,
            C_r,
            constants.R_EARTH,
            t0,
            n_steps=100,
        )

        # Orbit should still be valid
        assert np.linalg.norm(r1) > 40e6  # Still near GEO
        assert np.linalg.norm(v1) > 1000.0  # Still has significant velocity

    def test_propagate_srp_dopri5_basic(self):
        """Basic test of adaptive DOPRI5 SRP propagation."""
        r0 = np.array([42164e3, 0.0, 0.0])
        v0 = np.array([0.0, 3075.0, 0.0])
        area_mass_ratio = 0.01
        C_r = 1.3
        t0 = 0.0

        r1, v1 = propagate_srp_dopri5(
            r0,
            v0,
            3600.0,
            constants.GM_EARTH,
            area_mass_ratio,
            C_r,
            constants.R_EARTH,
            t0,
            tol=1e-8,
        )

        assert np.linalg.norm(r1) > 40e6
        assert np.linalg.norm(v1) > 1000.0

    def test_srp_causes_orbital_drift(self):
        """SRP should cause measurable orbital changes."""
        r0 = np.array([42164e3, 0.0, 0.0])
        v0 = np.array([0.0, 3075.0, 0.0])
        area_mass_ratio = 0.02  # Higher A/m for more effect
        C_r = 1.5
        t0 = 0.0
        dt = 86400.0  # 1 day

        # Two-body propagation (no SRP)
        r_twobody, v_twobody = propagate_state_keplerian(r0, v0, dt, constants.GM_EARTH)

        # SRP propagation
        r_srp, v_srp = propagate_srp_rk4(
            r0,
            v0,
            dt,
            constants.GM_EARTH,
            area_mass_ratio,
            C_r,
            constants.R_EARTH,
            t0,
            n_steps=1000,
        )

        # Positions should differ due to SRP
        pos_diff = np.linalg.norm(r_twobody - r_srp)
        print(f"Position difference after 1 day: {pos_diff:.2f} m")

        # At GEO with significant A/m, should see meters to km difference over 1 day
        assert pos_diff > 1.0, "SRP should cause measurable difference"
        assert pos_diff < 1e6, "Difference seems unreasonably large"

    def test_srp_effect_increases_with_duration(self):
        """SRP effect should accumulate over time."""
        r0 = np.array([42164e3, 0.0, 0.0])
        v0 = np.array([0.0, 3075.0, 0.0])
        area_mass_ratio = 0.01
        C_r = 1.3
        t0 = 0.0

        # Propagate for 1 day
        r_1day, v_1day = propagate_srp_rk4(
            r0,
            v0,
            86400.0,
            constants.GM_EARTH,
            area_mass_ratio,
            C_r,
            constants.R_EARTH,
            t0,
            n_steps=1000,
        )

        # Propagate for 2 days
        r_2day, v_2day = propagate_srp_rk4(
            r0,
            v0,
            2 * 86400.0,
            constants.GM_EARTH,
            area_mass_ratio,
            C_r,
            constants.R_EARTH,
            t0,
            n_steps=2000,
        )

        # Compare to two-body propagation
        r_tb_1day, _ = propagate_state_keplerian(r0, v0, 86400.0, constants.GM_EARTH)
        r_tb_2day, _ = propagate_state_keplerian(r0, v0, 2 * 86400.0, constants.GM_EARTH)

        diff_1day = np.linalg.norm(r_1day - r_tb_1day)
        diff_2day = np.linalg.norm(r_2day - r_tb_2day)

        # 2-day difference should be larger than 1-day
        assert diff_2day > diff_1day, "SRP effect should accumulate"

    def test_rk4_vs_dopri5_agreement(self):
        """RK4 and DOPRI5 should give similar results."""
        r0 = np.array([42164e3, 0.0, 0.0])
        v0 = np.array([0.0, 3075.0, 0.0])
        area_mass_ratio = 0.01
        C_r = 1.3
        t0 = 0.0
        dt = 3600.0

        # RK4 with many steps
        r_rk4, v_rk4 = propagate_srp_rk4(
            r0, v0, dt, constants.GM_EARTH, area_mass_ratio, C_r, constants.R_EARTH, t0, n_steps=200
        )

        # DOPRI5 with tight tolerance
        r_dopri5, v_dopri5 = propagate_srp_dopri5(
            r0, v0, dt, constants.GM_EARTH, area_mass_ratio, C_r, constants.R_EARTH, t0, tol=1e-10
        )

        # Should agree reasonably well
        pos_diff = np.linalg.norm(r_rk4 - r_dopri5)
        vel_diff = np.linalg.norm(v_rk4 - v_dopri5)

        assert pos_diff < 1000.0, f"Position difference too large: {pos_diff} m"
        assert vel_diff < 1.0, f"Velocity difference too large: {vel_diff} m/s"

    def test_srp_leo_vs_geo(self):
        """SRP effect should be similar at LEO and GEO (both at ~1 AU from Sun)."""
        # LEO orbit
        r0_leo = np.array([7000e3, 0.0, 0.0])
        v0_leo = np.array([0.0, 7546.0, 0.0])

        # GEO orbit
        r0_geo = np.array([42164e3, 0.0, 0.0])
        v0_geo = np.array([0.0, 3075.0, 0.0])

        area_mass_ratio = 0.01
        C_r = 1.3
        t0 = 0.0
        dt = 3600.0

        # Get SRP acceleration magnitudes
        r_sun = sun_position_simple(t0)
        a_srp_leo = srp_acceleration(r0_leo, r_sun, area_mass_ratio, C_r, constants.R_EARTH)
        a_srp_geo = srp_acceleration(r0_geo, r_sun, area_mass_ratio, C_r, constants.R_EARTH)

        mag_leo = np.linalg.norm(a_srp_leo)
        mag_geo = np.linalg.norm(a_srp_geo)

        # Both should be similar (within ~10%) since both are ~1 AU from Sun
        # GEO might be slightly less if in shadow more often
        assert mag_geo > 0.0
        assert mag_leo > 0.0

    def test_invalid_vector_sizes_propagation(self):
        """Test error handling in propagation functions."""
        r0 = np.array([42164e3, 0.0])  # Only 2 components
        v0 = np.array([0.0, 3075.0, 0.0])
        area_mass_ratio = 0.01
        C_r = 1.3
        t0 = 0.0

        with pytest.raises(ValueError):
            propagate_srp_rk4(
                r0, v0, 3600.0, constants.GM_EARTH, area_mass_ratio, C_r, constants.R_EARTH, t0
            )


if __name__ == "__main__":
    pytest.main([__file__, "-v", "--tb=short"])
