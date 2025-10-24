"""
Validation tests using reference data from poliastro and published sources.

This module uses the reference data from test_reference_data.py to validate
astrora's implementations against known good values from textbooks and the
original poliastro library.

Tests are organized by functionality:
1. Orbit propagation (Vallado, Curtis examples)
2. State vector conversions (Cartesian ↔ Classical elements)
3. Lambert problem solutions
4. Maneuver calculations (Hohmann, Bielliptic)

All tests use reference data fixtures from test_reference_data.py
"""

import numpy as np
import pytest
from astrora._core import (
    Duration,
    Epoch,
    coe_to_rv,
    constants,
    rv_to_coe,
)
from astrora.bodies import Earth
from astrora.twobody import Orbit

# =============================================================================
# Orbit Propagation Validation
# =============================================================================


@pytest.mark.validation
@pytest.mark.propagation
class TestPropagationAgainstReferences:
    """Validate orbit propagation against published reference solutions."""

    @pytest.mark.skip(reason="Astropy units integration issue - needs investigation")
    def test_vallado_example_2_4(self, vallado_2_4):
        """
        Test propagation against Vallado Example 2.4.

        This is a widely-used validation case for orbit propagators.
        """
        ref = vallado_2_4
        r0 = ref["initial_state"]["r"]
        v0 = ref["initial_state"]["v"]
        tof = ref["time_of_flight"]

        # Create orbit
        epoch = Epoch.j2000_epoch()
        orbit = Orbit.from_vectors(Earth, r0, v0, epoch)

        # Propagate
        dt = Duration(tof)  # Duration constructor takes seconds
        final_orbit = orbit.propagate(dt)

        # Get final state
        r_final = final_orbit.r
        v_final = final_orbit.v

        # Compare with expected
        r_expected = ref["expected_final_state"]["r"]
        v_expected = ref["expected_final_state"]["v"]

        # Position tolerance: textbook precision (~1 m)
        r_error = np.linalg.norm(r_final - r_expected)
        assert r_error < ref["tolerance"]["position"], (
            f"Position error: {r_error:.3f} m exceeds tolerance "
            f"{ref['tolerance']['position']} m"
        )

        # Velocity tolerance
        v_error = np.linalg.norm(v_final - v_expected)
        assert v_error < ref["tolerance"]["velocity"], (
            f"Velocity error: {v_error:.6f} m/s exceeds tolerance "
            f"{ref['tolerance']['velocity']} m/s"
        )

    @pytest.mark.slow
    @pytest.mark.xfail(reason="Hyperbolic orbit propagation not yet implemented")
    def test_curtis_example_3_5_hyperbolic(self, curtis_3_5):
        """
        Test hyperbolic orbit propagation against Curtis Example 3.5.

        Tests that hyperbolic trajectories are properly handled.
        """
        ref = curtis_3_5
        r0 = ref["initial_state"]["r"]
        v0 = ref["initial_state"]["v"]
        tof = ref["time_of_flight"]

        # Create hyperbolic orbit
        epoch = Epoch.j2000_epoch()
        orbit = Orbit.from_vectors(Earth, r0, v0, epoch)

        # Verify it's hyperbolic
        assert orbit.ecc > 1.0, "Orbit should be hyperbolic"

        # Propagate
        dt = Duration(tof)  # Duration constructor takes seconds
        final_orbit = orbit.propagate(dt)

        # Get final state magnitudes
        r_mag = np.linalg.norm(final_orbit.r)
        v_mag = np.linalg.norm(final_orbit.v)

        # Compare with expected magnitudes
        r_expected = ref["expected_final_state"]["r_magnitude"]
        v_expected = ref["expected_final_state"]["v_magnitude"]

        r_error = abs(r_mag - r_expected)
        v_error = abs(v_mag - v_expected)

        assert r_error < ref["tolerance"]["position"], (
            f"Position magnitude error: {r_error/1e3:.2f} km exceeds "
            f"tolerance {ref['tolerance']['position']/1e3:.2f} km"
        )

        assert v_error < ref["tolerance"]["velocity"], (
            f"Velocity magnitude error: {v_error:.2f} m/s exceeds "
            f"tolerance {ref['tolerance']['velocity']:.2f} m/s"
        )


# =============================================================================
# State Vector Conversion Validation
# =============================================================================


@pytest.mark.validation
@pytest.mark.unit
class TestStateConversionAgainstReferences:
    """Validate state vector conversions against published examples."""

    def test_curtis_example_4_3_rv_to_coe(self, curtis_4_3):
        """
        Test r,v to classical elements against Curtis Example 4.3.

        This is a standard validation case for coordinate conversions.
        """
        ref = curtis_4_3
        r = ref["state"]["r"]
        v = ref["state"]["v"]
        gm = constants.GM_EARTH

        # Convert to orbital elements
        elements = rv_to_coe(r, v, gm)

        # Extract expected values
        expected = ref["expected_elements"]
        tol = ref["tolerance"]

        # Validate eccentricity
        assert (
            abs(elements.e - expected["e"]) < tol["e"]
        ), f"Eccentricity: got {elements.e:.6f}, expected {expected['e']:.6f}"

        # Validate angles (convert from radians to degrees)
        i_deg = np.rad2deg(elements.i)
        raan_deg = np.rad2deg(elements.raan)
        argp_deg = np.rad2deg(elements.argp)
        nu_deg = np.rad2deg(elements.nu)

        assert (
            abs(i_deg - expected["i"]) < tol["angles_deg"]
        ), f"Inclination: got {i_deg:.2f}°, expected {expected['i']:.2f}°"

        assert (
            abs(raan_deg - expected["raan"]) < tol["angles_deg"]
        ), f"RAAN: got {raan_deg:.2f}°, expected {expected['raan']:.2f}°"

        assert (
            abs(argp_deg - expected["argp"]) < tol["angles_deg"]
        ), f"Arg of periapsis: got {argp_deg:.2f}°, expected {expected['argp']:.2f}°"

        assert (
            abs(nu_deg - expected["nu"]) < tol["angles_deg"]
        ), f"True anomaly: got {nu_deg:.2f}°, expected {expected['nu']:.2f}°"

        # Validate semi-latus rectum
        p_actual = elements.p
        assert abs(p_actual - expected["p"]) < tol["p"], (
            f"Semi-latus rectum: got {p_actual/1e3:.2f} km, " f"expected {expected['p']/1e3:.2f} km"
        )

    def test_curtis_4_3_roundtrip(self, curtis_4_3):
        """
        Test roundtrip conversion: r,v → elements → r,v.

        Ensures conversion is reversible to high precision.
        """
        ref = curtis_4_3
        r_orig = ref["state"]["r"]
        v_orig = ref["state"]["v"]
        gm = constants.GM_EARTH

        # Convert to elements and back
        elements = rv_to_coe(r_orig, v_orig, gm)
        r_new, v_new = coe_to_rv(elements, gm)

        # Should match to numerical precision
        r_error = np.linalg.norm(r_new - r_orig)
        v_error = np.linalg.norm(v_new - v_orig)

        assert r_error < 1e-6, f"Position roundtrip error: {r_error:.9f} m"
        assert v_error < 1e-9, f"Velocity roundtrip error: {v_error:.12f} m/s"


# =============================================================================
# Lambert Problem Validation
# =============================================================================


@pytest.mark.validation
@pytest.mark.maneuvers
class TestLambertAgainstReferences:
    """Validate Lambert problem solutions against published test cases."""

    @pytest.mark.xfail(reason="Lambert solver convergence needs improvement for some cases")
    def test_vallado75_lambert(self, vallado75_lambert):
        """Test Lambert solver against Vallado test case 75."""
        ref = vallado75_lambert
        gm = constants.GM_EARTH

        # Import lambert solver
        from astrora._core import lambert_solve

        # Solve Lambert problem
        result = lambert_solve(ref["r1"], ref["r2"], ref["time_of_flight"], gm, short_way=True)

        # Extract velocities
        v1_computed = result["v1"]
        v2_computed = result["v2"]

        # Compare with expected
        v1_expected = ref["expected_velocities"]["v1"]
        v2_expected = ref["expected_velocities"]["v2"]

        v1_error = np.linalg.norm(v1_computed - v1_expected)
        v2_error = np.linalg.norm(v2_computed - v2_expected)

        tol = ref["tolerance"]["velocity"]

        assert v1_error < tol, f"v1 error: {v1_error:.3f} m/s exceeds tolerance {tol:.3f} m/s"

        assert v2_error < tol, f"v2 error: {v2_error:.3f} m/s exceeds tolerance {tol:.3f} m/s"

    def test_curtis52_lambert(self, curtis52_lambert):
        """Test Lambert solver against Curtis test case 52."""
        ref = curtis52_lambert
        gm = constants.GM_EARTH

        from astrora._core import lambert_solve

        # Solve Lambert problem
        result = lambert_solve(ref["r1"], ref["r2"], ref["time_of_flight"], gm, short_way=True)

        # Extract velocities
        v1_computed = result["v1"]
        v2_computed = result["v2"]

        # Compare with expected
        v1_expected = ref["expected_velocities"]["v1"]
        v2_expected = ref["expected_velocities"]["v2"]

        v1_error = np.linalg.norm(v1_computed - v1_expected)
        v2_error = np.linalg.norm(v2_computed - v2_expected)

        tol = ref["tolerance"]["velocity"]

        assert v1_error < tol, f"v1 error: {v1_error:.3f} m/s exceeds tolerance {tol:.3f} m/s"

        assert v2_error < tol, f"v2 error: {v2_error:.3f} m/s exceeds tolerance {tol:.3f} m/s"

    @pytest.mark.xfail(reason="High-altitude Lambert cases need solver improvements")
    def test_curtis53_lambert(self, curtis53_lambert):
        """Test Lambert solver against Curtis test case 53."""
        ref = curtis53_lambert
        gm = constants.GM_EARTH

        from astrora._core import lambert_solve

        # Solve Lambert problem (high altitude case)
        result = lambert_solve(ref["r1"], ref["r2"], ref["time_of_flight"], gm, short_way=True)

        # Extract v1
        v1_computed = result["v1"]

        # Compare with expected v1 (v2 not provided in reference)
        v1_expected = ref["expected_velocities"]["v1"]

        v1_error = np.linalg.norm(v1_computed - v1_expected)
        tol = ref["tolerance"]["velocity"]

        assert v1_error < tol, f"v1 error: {v1_error:.3f} m/s exceeds tolerance {tol:.3f} m/s"


# =============================================================================
# Maneuver Validation
# =============================================================================


@pytest.mark.validation
@pytest.mark.maneuvers
class TestManeuversAgainstReferences:
    """Validate maneuver calculations against published values."""

    def test_hohmann_leo_to_geo(self, hohmann_leo_geo):
        """Test Hohmann transfer against standard LEO-GEO reference."""
        ref = hohmann_leo_geo
        from astrora._core import hohmann_transfer

        # Calculate radii
        r_initial = constants.R_EARTH + ref["initial_altitude"]
        r_final = constants.R_EARTH + ref["final_altitude"]

        # Compute Hohmann transfer
        result = hohmann_transfer(r_initial, r_final, constants.GM_EARTH)

        # Validate delta-v total
        dv_total = result["delta_v_total"]
        dv_expected = ref["expected_results"]["delta_v_total"]
        dv_error = abs(dv_total - dv_expected)

        assert dv_error < ref["tolerance"]["delta_v"], (
            f"Delta-v error: {dv_error:.2f} m/s exceeds "
            f"tolerance {ref['tolerance']['delta_v']:.2f} m/s"
        )

        # Validate transfer time
        t_transfer = result["transfer_time"]
        t_expected = ref["expected_results"]["transfer_time"]
        t_error = abs(t_transfer - t_expected)

        assert t_error < ref["tolerance"]["time"], (
            f"Transfer time error: {t_error:.2f} s exceeds "
            f"tolerance {ref['tolerance']['time']:.2f} s"
        )

        # Validate transfer orbit eccentricity is reasonable (Hohmann transfer orbit is elliptical)
        e_final = result["transfer_eccentricity"]
        # For Hohmann transfer, eccentricity should be between 0 and 1 (elliptical)
        assert (
            0.0 < e_final < 1.0
        ), f"Transfer orbit should be elliptical (0 < e < 1), got {e_final:.6f}"

    @pytest.mark.skip(
        reason="Bielliptic reference data needs correction - intermediate altitude specification"
    )
    def test_bielliptic_transfer(self, bielliptic_transfer):
        """Test bielliptic transfer against reference values."""
        ref = bielliptic_transfer
        from astrora._core import bielliptic_transfer as compute_bielliptic

        # Calculate radii
        r_initial = constants.R_EARTH + ref["initial_altitude"]
        r_intermediate = constants.R_EARTH + ref["intermediate_altitude"]
        r_final = constants.R_EARTH + ref["final_altitude"]

        # Compute bielliptic transfer
        result = compute_bielliptic(r_initial, r_intermediate, r_final, constants.GM_EARTH)

        # Validate delta-v total
        dv_total = result["delta_v_total"]
        dv_expected = ref["expected_results"]["delta_v_total"]
        dv_error = abs(dv_total - dv_expected)

        assert dv_error < ref["tolerance"]["delta_v"], (
            f"Delta-v error: {dv_error:.2f} m/s exceeds "
            f"tolerance {ref['tolerance']['delta_v']:.2f} m/s"
        )

        # Validate transfer time
        t_transfer = result["total_time"]
        t_expected = ref["expected_results"]["transfer_time"]
        t_error = abs(t_transfer - t_expected)

        assert t_error < ref["tolerance"]["time"], (
            f"Transfer time error: {t_error:.2f} s exceeds "
            f"tolerance {ref['tolerance']['time']:.2f} s"
        )


# =============================================================================
# Basic Formula Validation
# =============================================================================


@pytest.mark.validation
@pytest.mark.unit
class TestBasicFormulas:
    """Validate basic astrodynamics formulas."""

    def test_circular_velocity(self, all_reference_data):
        """Test circular velocity formula: V = sqrt(GM/a)."""
        ref = all_reference_data["basic"]["circular_velocity"]

        gm = ref["gm"]
        a = ref["semi_major_axis"]

        # Compute circular velocity
        v_circular = np.sqrt(gm / a)

        # Compare with expected
        v_expected = ref["expected_velocity"]
        error = abs(v_circular - v_expected)

        assert error < ref["tolerance"], (
            f"Circular velocity: got {v_circular:.6f} m/s, "
            f"expected {v_expected:.6f} m/s, error {error:.6f} m/s"
        )


# =============================================================================
# Summary Test
# =============================================================================


@pytest.mark.validation
class TestReferenceSummary:
    """Summary test showing all reference data sources."""

    def test_print_reference_summary(self, all_reference_data):
        """Print summary of all available reference data."""
        print("\n" + "=" * 70)
        print("REFERENCE DATA SUMMARY")
        print("=" * 70)

        categories = [
            "propagation",
            "state_conversion",
            "lambert",
            "maneuvers",
            "basic",
            "mission_data",
        ]

        for category in categories:
            if category in all_reference_data:
                cases = all_reference_data[category]
                print(f"\n{category.upper()}:")
                for name, data in cases.items():
                    if isinstance(data, dict) and "description" in data:
                        print(f"  • {name}: {data['description']}")
                        if "source" in data:
                            print(f"    Source: {data['source']}")

        print("\n" + "=" * 70)
        print(f"Total reference test cases loaded successfully")
        print("=" * 70 + "\n")

        # This test always passes - it's just for documentation
        assert True
