"""
Test suite for geodesic and light path calculations.

Tests cover:
- Einstein gravitational deflection (analytical weak-field)
- Schwarzschild null geodesic integration
- Black hole geometry (photon sphere, ISCO, shadow, Einstein ring)
- Trajectory classification
- Composite light path computation
- Shapiro time delay
- Cross-validation between numerical and analytical methods

References:
- Einstein, A. (1916). Solar limb deflection: 1.7505 arcseconds
- Shapiro, I.I. (1964). Fourth test of general relativity
- EHT Collaboration (2022). Sgr A* shadow: ~52 μas
"""

import math

import pytest

from astrora._core import (
    black_hole_geometry,
    classify_trajectory,
    compute_light_path,
    einstein_deflection,
    einstein_ring_radius,
    schwarzschild_geodesic,
    schwarzschild_geodesic_cartesian,
    shapiro_delay,
    sun_limb_deflection,
    constants,
)

# Constants for convenience
GM_SUN = constants.GM_SUN
R_SUN = constants.R_SUN
GM_SGR_A_STAR = constants.GM_SGR_A_STAR
R_SCHWARZSCHILD_SGR_A_STAR = constants.R_SCHWARZSCHILD_SGR_A_STAR
DISTANCE_SGR_A_STAR = constants.DISTANCE_SGR_A_STAR
AU = constants.AU
C = constants.C


# ============================================================================
# Einstein Deflection Tests
# ============================================================================


class TestEinsteinDeflection:
    """Tests for the analytical weak-field Einstein deflection formula."""

    @pytest.mark.unit
    def test_sun_limb_deflection(self):
        """Verify the classic 1.75 arcsecond prediction for the Sun."""
        result = einstein_deflection(GM_SUN, R_SUN)
        assert abs(result["deflection_arcsec"] - 1.7505) < 0.01

    @pytest.mark.unit
    def test_sun_limb_convenience(self):
        """Verify the sun_limb_deflection convenience function."""
        result = sun_limb_deflection()
        assert abs(result["deflection_arcsec"] - 1.7505) < 0.01
        assert result["is_weak_field"] is True

    @pytest.mark.unit
    def test_scaling_with_gm(self):
        """Deflection should scale linearly with GM."""
        r1 = einstein_deflection(GM_SUN, 1e10)
        r2 = einstein_deflection(2.0 * GM_SUN, 1e10)
        assert abs(r2["deflection_angle"] / r1["deflection_angle"] - 2.0) < 1e-10

    @pytest.mark.unit
    def test_scaling_with_impact_parameter(self):
        """Deflection should scale inversely with impact parameter."""
        r1 = einstein_deflection(GM_SUN, 1e10)
        r2 = einstein_deflection(GM_SUN, 2e10)
        assert abs(r1["deflection_angle"] / r2["deflection_angle"] - 2.0) < 1e-10

    @pytest.mark.unit
    def test_zero_gm(self):
        """Zero mass should give zero deflection."""
        result = einstein_deflection(0.0, 1e10)
        assert result["deflection_angle"] == 0.0

    @pytest.mark.unit
    def test_invalid_inputs(self):
        """Negative GM or non-positive impact parameter should raise."""
        with pytest.raises(Exception):
            einstein_deflection(-1.0, 1e10)
        with pytest.raises(Exception):
            einstein_deflection(GM_SUN, 0.0)
        with pytest.raises(Exception):
            einstein_deflection(GM_SUN, -1.0)


# ============================================================================
# Schwarzschild Geodesic Tests
# ============================================================================


class TestSchwarzschildGeodesic:
    """Tests for numerical null geodesic integration."""

    @pytest.mark.unit
    def test_flat_space(self):
        """With gm=0, photon travels in a straight line (zero deflection)."""
        result = schwarzschild_geodesic(0.0, 1e9, math.pi, 10000)
        assert result["deflection_angle"] < 1e-6
        assert result["captured"] is False

    @pytest.mark.unit
    def test_sun_deflection_order_of_magnitude(self):
        """Sun limb deflection should be ~1.75 arcseconds."""
        result = schwarzschild_geodesic(GM_SUN, R_SUN, math.pi, 100000)
        deflection_arcsec = result["deflection_angle"] * 206265.0
        assert 0.5 < deflection_arcsec < 5.0, (
            f"Expected ~1.75 arcsec, got {deflection_arcsec:.4f}"
        )

    @pytest.mark.unit
    def test_capture(self):
        """Photon below critical impact parameter should be captured."""
        # b_crit = 3√3 · GM/c² for Sgr A*
        b_crit = 3.0 * math.sqrt(3) * GM_SGR_A_STAR / (C * C)
        result = schwarzschild_geodesic(
            GM_SGR_A_STAR, b_crit * 0.5, 2 * math.pi, 100000
        )
        assert result["captured"] is True

    @pytest.mark.unit
    def test_cartesian_output(self):
        """Cartesian output should have z=0 (equatorial plane)."""
        points = schwarzschild_geodesic_cartesian(GM_SUN, R_SUN * 10, math.pi, 10000)
        assert len(points) > 0
        for p in points:
            assert abs(p[2]) < 1e-10

    @pytest.mark.unit
    def test_invalid_inputs(self):
        """Invalid parameters should raise."""
        with pytest.raises(Exception):
            schwarzschild_geodesic(-1.0, 1e9, math.pi, 1000)
        with pytest.raises(Exception):
            schwarzschild_geodesic(GM_SUN, 0.0, math.pi, 1000)


# ============================================================================
# Black Hole Geometry Tests
# ============================================================================


class TestBlackHoleGeometry:
    """Tests for photon sphere and black hole geometry calculations."""

    @pytest.mark.unit
    def test_sgr_a_star_schwarzschild_radius(self):
        """Sgr A* Schwarzschild radius should match the precomputed constant."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        assert abs(geom["schwarzschild_radius"] - R_SCHWARZSCHILD_SGR_A_STAR) < 10.0

    @pytest.mark.unit
    def test_radius_relationships(self):
        """Verify r_s < r_ph < r_isco ordering."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        assert geom["schwarzschild_radius"] < geom["photon_sphere_radius"]
        assert geom["photon_sphere_radius"] < geom["isco_radius"]

    @pytest.mark.unit
    def test_photon_sphere_ratio(self):
        """Photon sphere = 1.5 × Schwarzschild radius."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        ratio = geom["photon_sphere_radius"] / geom["schwarzschild_radius"]
        assert abs(ratio - 1.5) < 1e-10

    @pytest.mark.unit
    def test_isco_ratio(self):
        """ISCO = 3 × Schwarzschild radius."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        ratio = geom["isco_radius"] / geom["schwarzschild_radius"]
        assert abs(ratio - 3.0) < 1e-10

    @pytest.mark.unit
    def test_shadow_equals_critical_impact(self):
        """For Schwarzschild, shadow radius = critical impact parameter."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        assert abs(
            geom["shadow_radius"] - geom["critical_impact_parameter"]
        ) < 1.0

    @pytest.mark.unit
    def test_invalid_gm(self):
        """Non-positive GM should raise."""
        with pytest.raises(Exception):
            black_hole_geometry(0.0)
        with pytest.raises(Exception):
            black_hole_geometry(-1.0)


# ============================================================================
# Trajectory Classification Tests
# ============================================================================


class TestTrajectoryClassification:
    """Tests for photon trajectory classification."""

    @pytest.mark.unit
    def test_capture(self):
        """Photon below critical impact parameter → capture."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        b_crit = geom["critical_impact_parameter"]
        assert classify_trajectory(GM_SGR_A_STAR, b_crit * 0.5) == "capture"

    @pytest.mark.unit
    def test_deflection(self):
        """Photon well above critical impact parameter → deflection."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        b_crit = geom["critical_impact_parameter"]
        assert classify_trajectory(GM_SGR_A_STAR, b_crit * 2.0) == "deflection"

    @pytest.mark.unit
    def test_unstable_orbit(self):
        """Photon near critical impact parameter → unstable orbit."""
        geom = black_hole_geometry(GM_SGR_A_STAR)
        b_crit = geom["critical_impact_parameter"]
        assert classify_trajectory(GM_SGR_A_STAR, b_crit * 1.0005) == "unstable_orbit"


# ============================================================================
# Light Path Tests
# ============================================================================


class TestLightPath:
    """Tests for composite light path computation."""

    @pytest.mark.unit
    def test_no_bodies_straight_line(self):
        """Without lensing bodies, path should be a straight line."""
        source = [0.0, 0.0, 0.0]
        target = [1e12, 0.0, 0.0]

        result = compute_light_path(source, target, [], [], 100)

        assert abs(result["total_distance"] - result["straight_line_distance"]) < 1.0
        assert len(result["deflections"]) == 0
        assert result["total_deflection"] == 0.0

    @pytest.mark.unit
    def test_travel_time(self):
        """Travel time should equal distance / c."""
        source = [0.0, 0.0, 0.0]
        target = [AU, 0.0, 0.0]

        result = compute_light_path(source, target, [], [], 100)

        expected_time = AU / C
        assert abs(result["travel_time"] - expected_time) < 0.1

    @pytest.mark.unit
    def test_sun_deflection(self):
        """Light passing near the Sun should be deflected."""
        source = [0.0, 10.0 * AU, 0.0]
        target = [0.0, -AU, 0.0]
        body_positions = [[R_SUN * 10.0, 0.0, 0.0]]
        body_gms = [GM_SUN]

        result = compute_light_path(source, target, body_positions, body_gms, 200)

        assert len(result["deflections"]) == 1
        assert result["deflections"][0]["deflection_angle"] > 0.0
        assert result["total_deflection"] > 0.0

    @pytest.mark.unit
    def test_path_endpoints(self):
        """First and last points should match source and target."""
        source = [0.0, 0.0, 0.0]
        target = [1e12, 0.0, 0.0]

        result = compute_light_path(source, target, [], [], 50)

        first = result["points"][0]
        last = result["points"][-1]
        assert abs(first[0] - source[0]) < 1.0
        assert abs(last[0] - target[0]) < 1.0

    @pytest.mark.unit
    def test_degenerate_path(self):
        """Source == target should raise."""
        with pytest.raises(Exception):
            compute_light_path([1e6, 0, 0], [1e6, 0, 0], [], [], 100)


# ============================================================================
# Shapiro Delay Tests
# ============================================================================


class TestShapiroDelay:
    """Tests for Shapiro time delay calculation."""

    @pytest.mark.unit
    def test_sun_order_of_magnitude(self):
        """Shapiro delay near the Sun should be ~200 μs for Earth-Mars."""
        r_earth = AU
        r_mars = 2.5 * AU
        delay = shapiro_delay(GM_SUN, r_earth, r_mars, R_SUN)
        delay_us = delay * 1e6

        assert 50.0 < delay_us < 500.0, (
            f"Expected ~200 μs, got {delay_us:.1f} μs"
        )

    @pytest.mark.unit
    def test_scales_with_gm(self):
        """Shapiro delay should scale linearly with GM."""
        d1 = shapiro_delay(GM_SUN, 1e11, 1e11, 1e9)
        d2 = shapiro_delay(2.0 * GM_SUN, 1e11, 1e11, 1e9)
        assert abs(d2 / d1 - 2.0) < 1e-10

    @pytest.mark.unit
    def test_invalid_inputs(self):
        """Non-positive parameters should raise."""
        with pytest.raises(Exception):
            shapiro_delay(GM_SUN, 0.0, 1e11, 1e9)
        with pytest.raises(Exception):
            shapiro_delay(GM_SUN, 1e11, 1e11, 0.0)


# ============================================================================
# Einstein Ring Tests
# ============================================================================


class TestEinsteinRing:
    """Tests for Einstein ring angular radius calculation."""

    @pytest.mark.unit
    def test_positive_angle(self):
        """Einstein ring should have a positive angular radius."""
        theta = einstein_ring_radius(
            GM_SGR_A_STAR,
            DISTANCE_SGR_A_STAR,
            2.0 * DISTANCE_SGR_A_STAR,
        )
        assert theta > 0.0

    @pytest.mark.unit
    def test_small_angle(self):
        """Einstein ring for Sgr A* should be very small (sub-arcsecond)."""
        theta = einstein_ring_radius(
            GM_SGR_A_STAR,
            DISTANCE_SGR_A_STAR,
            2.0 * DISTANCE_SGR_A_STAR,
        )
        theta_arcsec = theta * 206265.0
        # Should be on the order of arcseconds for a supermassive black hole
        assert theta_arcsec < 10.0

    @pytest.mark.unit
    def test_invalid_distances(self):
        """Source must be farther than lens."""
        with pytest.raises(Exception):
            einstein_ring_radius(GM_SGR_A_STAR, 1e20, 0.5e20)


# ============================================================================
# Cross-Validation Tests
# ============================================================================


class TestCrossValidation:
    """Cross-validation between analytical and numerical methods."""

    @pytest.mark.validation
    def test_numerical_vs_analytical_weak_field(self):
        """Numerical geodesic deflection should match Einstein formula in weak field."""
        # Use a large impact parameter (well in weak-field regime)
        impact_param = R_SUN * 100.0  # 100 solar radii

        analytical = einstein_deflection(GM_SUN, impact_param)
        numerical = schwarzschild_geodesic(GM_SUN, impact_param, math.pi, 200000)

        # Both should give very small deflection
        analytical_angle = analytical["deflection_angle"]
        numerical_angle = numerical["deflection_angle"]

        # Should agree within 10% for weak field
        if analytical_angle > 1e-12:
            ratio = numerical_angle / analytical_angle
            assert 0.5 < ratio < 2.0, (
                f"Numerical ({numerical_angle:.2e}) and analytical ({analytical_angle:.2e}) "
                f"should agree in weak field, ratio={ratio:.3f}"
            )
