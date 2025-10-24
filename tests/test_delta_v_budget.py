"""Tests for delta-v budget calculations"""

import pytest
from astrora._core import constants, delta_v_budget

GM_EARTH = constants.GM_EARTH


class TestDeltaVBudgetBasics:
    """Test basic delta-v budget functionality"""

    def test_empty_budget(self):
        """Test budget with no maneuvers"""
        result = delta_v_budget("Empty Mission", [])

        assert result["mission_name"] == "Empty Mission"
        assert result["total_delta_v"] == 0.0
        assert result["num_maneuvers"] == 0
        assert result["contingency_margin"] == 0.0
        assert result["total_with_contingency"] == 0.0

    def test_single_maneuver(self):
        """Test budget with a single maneuver"""
        maneuvers = [("Test burn", 1000.0)]
        result = delta_v_budget("Single Maneuver", maneuvers)

        assert result["mission_name"] == "Single Maneuver"
        assert result["total_delta_v"] == 1000.0
        assert result["num_maneuvers"] == 1
        assert result["total_with_contingency"] == 1000.0

    def test_multiple_maneuvers(self):
        """Test budget with multiple maneuvers"""
        maneuvers = [
            ("Burn 1", 1000.0),
            ("Burn 2", 500.0),
            ("Burn 3", 250.0),
        ]
        result = delta_v_budget("Multiple Maneuvers", maneuvers)

        assert result["total_delta_v"] == 1750.0
        assert result["num_maneuvers"] == 3
        assert result["total_with_contingency"] == 1750.0

    def test_negative_delta_v_raises_error(self):
        """Test that negative delta-v raises an error"""
        maneuvers = [("Bad burn", -100.0)]

        with pytest.raises(ValueError):
            delta_v_budget("Bad Mission", maneuvers)


class TestContingencyMargin:
    """Test contingency margin functionality"""

    def test_10_percent_contingency(self):
        """Test 10% contingency margin"""
        maneuvers = [("Burn", 1000.0)]
        result = delta_v_budget("Test Mission", maneuvers, contingency_margin=0.1)

        assert result["total_delta_v"] == 1000.0
        assert result["contingency_margin"] == 0.1
        assert result["total_with_contingency"] == 1100.0

    def test_20_percent_contingency(self):
        """Test 20% contingency margin"""
        maneuvers = [("Burn 1", 500.0), ("Burn 2", 500.0)]
        result = delta_v_budget("Test Mission", maneuvers, contingency_margin=0.2)

        assert result["total_delta_v"] == 1000.0
        assert result["total_with_contingency"] == 1200.0

    def test_invalid_contingency_handling(self):
        """Test handling of edge-case contingency margins"""
        maneuvers = [("Burn", 1000.0)]

        # Negative margin is treated as 0.0 (acceptable behavior)
        result = delta_v_budget("Test", maneuvers, contingency_margin=-0.1)
        assert result["contingency_margin"] == 0.0
        assert result["total_with_contingency"] == 1000.0

        # Greater than 1.0 raises error
        with pytest.raises(Exception):
            delta_v_budget("Test", maneuvers, contingency_margin=1.5)


class TestPropellantCalculation:
    """Test propellant mass calculations using Tsiolkovsky equation"""

    def test_propellant_mass_basic(self):
        """Test basic propellant calculation"""
        maneuvers = [("Burn", 3000.0)]
        result = delta_v_budget(
            "Test Mission",
            maneuvers,
            dry_mass=1000.0,
            specific_impulse=300.0,
        )

        # Tsiolkovsky: m_prop = m_dry × (exp(Δv/(Isp×g₀)) - 1)
        # With Δv=3000, Isp=300, g₀=9.80665:
        # m_prop = 1000 × (exp(3000/(300×9.80665)) - 1)
        #        = 1000 × (exp(1.0194) - 1) ≈ 1772 kg
        assert "propellant_mass" in result
        assert "propellant_fraction" in result
        assert "total_mass" in result
        assert abs(result["propellant_mass"] - 1772.0) < 10.0
        assert 0.63 < result["propellant_fraction"] < 0.65
        assert abs(result["total_mass"] - 2772.0) < 10.0

    def test_propellant_with_contingency(self):
        """Test propellant calculation includes contingency"""
        maneuvers = [("Burn", 1000.0)]

        # Without contingency
        result1 = delta_v_budget(
            "Test",
            maneuvers,
            contingency_margin=0.0,
            dry_mass=1000.0,
            specific_impulse=300.0,
        )

        # With 10% contingency
        result2 = delta_v_budget(
            "Test",
            maneuvers,
            contingency_margin=0.1,
            dry_mass=1000.0,
            specific_impulse=300.0,
        )

        # Propellant should be higher with contingency
        assert result2["propellant_mass"] > result1["propellant_mass"]
        assert result2["total_mass"] > result1["total_mass"]

    def test_no_propellant_without_mass_isp(self):
        """Test that propellant fields are absent without mass/Isp"""
        maneuvers = [("Burn", 1000.0)]

        # No mass or Isp
        result = delta_v_budget("Test", maneuvers)
        assert "propellant_mass" not in result
        assert "propellant_fraction" not in result
        assert "total_mass" not in result

        # Only mass, no Isp
        result = delta_v_budget("Test", maneuvers, dry_mass=1000.0)
        assert "propellant_mass" not in result

        # Only Isp, no mass
        result = delta_v_budget("Test", maneuvers, specific_impulse=300.0)
        assert "propellant_mass" not in result


class TestRealisticMissions:
    """Test realistic mission scenarios"""

    def test_leo_to_geo_hohmann(self):
        """Test LEO to GEO Hohmann transfer budget"""
        maneuvers = [
            ("Hohmann transfer burn 1", 2440.0),
            ("Hohmann transfer burn 2", 1475.0),
            ("Station keeping (5 years)", 250.0),
        ]

        result = delta_v_budget(
            "LEO to GEO",
            maneuvers,
            contingency_margin=0.1,
            dry_mass=1000.0,
            specific_impulse=300.0,
        )

        # Total: 2440 + 1475 + 250 = 4165 m/s
        assert abs(result["total_delta_v"] - 4165.0) < 1.0

        # With 10% margin: 4165 × 1.1 = 4581.5 m/s
        assert abs(result["total_with_contingency"] - 4581.5) < 1.0

        # Should have reasonable propellant mass
        assert result["propellant_mass"] > 2500.0  # Should need substantial propellant
        assert result["propellant_mass"] < 5000.0
        assert result["total_mass"] > 3500.0

    def test_interplanetary_mission(self):
        """Test interplanetary mission with multiple phases"""
        maneuvers = [
            ("LEO departure", 3200.0),
            ("Mid-course correction", 50.0),
            ("Mars orbit insertion", 2100.0),
            ("Descent orbit", 800.0),
        ]

        result = delta_v_budget(
            "Earth to Mars",
            maneuvers,
            contingency_margin=0.15,  # 15% for deep space
            dry_mass=2000.0,
            specific_impulse=320.0,
        )

        # Total: 3200 + 50 + 2100 + 800 = 6150 m/s
        assert abs(result["total_delta_v"] - 6150.0) < 1.0

        # With 15% margin
        assert abs(result["total_with_contingency"] - 7072.5) < 1.0

        assert result["num_maneuvers"] == 4
        assert "propellant_mass" in result

    def test_cubesat_mission(self):
        """Test small CubeSat mission with low delta-v"""
        maneuvers = [
            ("Orbit raise", 50.0),
            ("Station keeping", 20.0),
            ("Deorbit", 30.0),
        ]

        result = delta_v_budget(
            "CubeSat",
            maneuvers,
            contingency_margin=0.2,  # 20% margin for small sat
            dry_mass=10.0,  # 10 kg CubeSat
            specific_impulse=200.0,  # Cold gas thruster
        )

        assert result["total_delta_v"] == 100.0
        assert result["total_with_contingency"] == 120.0

        # For small delta-v, propellant should be small fraction
        assert result["propellant_mass"] < 1.0  # Less than 1 kg
        assert result["propellant_fraction"] < 0.1  # Less than 10%


class TestHighDeltaVScenarios:
    """Test scenarios requiring large delta-v"""

    def test_escape_velocity(self):
        """Test mission requiring escape velocity delta-v"""
        maneuvers = [
            ("LEO to escape", 3200.0),
        ]

        result = delta_v_budget(
            "Solar escape",
            maneuvers,
            dry_mass=1000.0,
            specific_impulse=450.0,  # High-performance engine
        )

        # High delta-v requires significant propellant fraction
        assert result["propellant_fraction"] > 0.5
        assert result["propellant_mass"] > 1000.0

    def test_very_high_delta_v(self):
        """Test unrealistic high delta-v scenario"""
        maneuvers = [
            ("Crazy maneuver", 10000.0),
        ]

        result = delta_v_budget(
            "Unrealistic",
            maneuvers,
            dry_mass=1000.0,
            specific_impulse=300.0,
        )

        # Should require massive propellant (may not be practical)
        assert result["propellant_mass"] > 10000.0
        assert result["propellant_fraction"] > 0.9


class TestPropellantParameterValidation:
    """Test validation of propellant calculation parameters"""

    def test_zero_dry_mass_raises_error(self):
        """Test that zero dry mass raises an error"""
        maneuvers = [("Burn", 1000.0)]

        with pytest.raises(Exception):
            delta_v_budget("Test", maneuvers, dry_mass=0.0, specific_impulse=300.0)

    def test_negative_dry_mass_raises_error(self):
        """Test that negative dry mass raises an error"""
        maneuvers = [("Burn", 1000.0)]

        with pytest.raises(Exception):
            delta_v_budget("Test", maneuvers, dry_mass=-100.0, specific_impulse=300.0)

    def test_zero_specific_impulse_raises_error(self):
        """Test that zero Isp raises an error"""
        maneuvers = [("Burn", 1000.0)]

        with pytest.raises(Exception):
            delta_v_budget("Test", maneuvers, dry_mass=1000.0, specific_impulse=0.0)

    def test_negative_specific_impulse_raises_error(self):
        """Test that negative Isp raises an error"""
        maneuvers = [("Burn", 1000.0)]

        with pytest.raises(Exception):
            delta_v_budget("Test", maneuvers, dry_mass=1000.0, specific_impulse=-300.0)


class TestDeltaVBudgetIntegration:
    """Test integration with other maneuver calculations"""

    def test_budget_from_hohmann_result(self):
        """Test creating budget from actual Hohmann transfer calculation"""
        from astrora._core import hohmann_transfer

        R_MEAN_EARTH = constants.R_MEAN_EARTH

        # Calculate LEO to GEO transfer
        r_leo = R_MEAN_EARTH + 400e3
        r_geo = R_MEAN_EARTH + 35_786e3
        hohmann = hohmann_transfer(r_leo, r_geo, GM_EARTH)

        # Create budget from Hohmann results
        maneuvers = [
            ("Hohmann burn 1", hohmann["delta_v1"]),
            ("Hohmann burn 2", hohmann["delta_v2"]),
        ]

        result = delta_v_budget("LEO-GEO Hohmann", maneuvers)

        # Total should match Hohmann total
        assert abs(result["total_delta_v"] - hohmann["delta_v_total"]) < 1.0

    def test_budget_from_bielliptic(self):
        """Test creating budget from bi-elliptic transfer"""
        from astrora._core import bielliptic_transfer

        R_MEAN_EARTH = constants.R_MEAN_EARTH

        r_initial = R_MEAN_EARTH + 400e3
        r_final = R_MEAN_EARTH + 35_786e3
        r_intermediate = 2 * r_final

        bielliptic = bielliptic_transfer(r_initial, r_final, r_intermediate, GM_EARTH)

        maneuvers = [
            ("Bi-elliptic burn 1", bielliptic["delta_v1"]),
            ("Bi-elliptic burn 2", bielliptic["delta_v2"]),
            ("Bi-elliptic burn 3", bielliptic["delta_v3"]),
        ]

        result = delta_v_budget("Bi-elliptic", maneuvers)

        # Total should match bi-elliptic total
        assert abs(result["total_delta_v"] - bielliptic["delta_v_total"]) < 1.0
        assert result["num_maneuvers"] == 3

    def test_complex_multi_phase_mission(self):
        """Test complex mission with multiple transfer types"""
        import math

        from astrora._core import (
            hohmann_transfer,
            pure_plane_change,
        )

        R_MEAN_EARTH = constants.R_MEAN_EARTH

        # LEO start
        r_leo = R_MEAN_EARTH + 400e3

        # Hohmann to MEO
        r_meo = R_MEAN_EARTH + 20_000e3
        hohmann1 = hohmann_transfer(r_leo, r_meo, GM_EARTH)

        # Plane change at MEO (28.5° to equatorial)
        v_meo = math.sqrt(GM_EARTH / r_meo)
        plane = pure_plane_change(v_meo, math.radians(28.5))

        # Hohmann to GEO
        r_geo = R_MEAN_EARTH + 35_786e3
        hohmann2 = hohmann_transfer(r_meo, r_geo, GM_EARTH)

        # Build budget
        maneuvers = [
            ("LEO to MEO burn 1", hohmann1["delta_v1"]),
            ("LEO to MEO burn 2", hohmann1["delta_v2"]),
            ("Plane change at MEO", plane["delta_v"]),
            ("MEO to GEO burn 1", hohmann2["delta_v1"]),
            ("MEO to GEO burn 2", hohmann2["delta_v2"]),
            ("Station keeping (10 years)", 500.0),
        ]

        result = delta_v_budget(
            "LEO-MEO-GEO with plane change",
            maneuvers,
            contingency_margin=0.15,
            dry_mass=1500.0,
            specific_impulse=315.0,
        )

        assert result["num_maneuvers"] == 6
        assert result["total_delta_v"] > 5000.0  # Should be substantial
        assert "propellant_mass" in result
        assert result["propellant_mass"] > 3000.0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
