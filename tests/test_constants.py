"""Tests for the constants module."""

import astrora._core as core
import pytest


class TestPhysicalConstants:
    """Test fundamental physical constants."""

    def test_gravitational_constant(self):
        """Test the gravitational constant G."""
        # CODATA 2018 value
        assert core.constants.G == pytest.approx(6.67430e-11, rel=1e-10)

    def test_speed_of_light(self):
        """Test the speed of light c."""
        # Exact value by definition
        assert core.constants.C == 299_792_458.0

    def test_astronomical_unit(self):
        """Test the astronomical unit."""
        # IAU 2012 exact value
        assert core.constants.AU == pytest.approx(149_597_870_700.0, rel=1e-10)


class TestSolarConstants:
    """Test solar constants."""

    def test_gm_sun(self):
        """Test Sun's gravitational parameter."""
        # JPL DE440 value: 1.32712440041279419 × 10²⁰ m³/s²
        assert core.constants.GM_SUN == pytest.approx(1.3271244004127942e20, rel=1e-10)

    def test_r_sun(self):
        """Test Sun's radius."""
        assert core.constants.R_SUN == pytest.approx(6.957e8, rel=1e-5)

    def test_j2_sun(self):
        """Test Sun's J2 coefficient."""
        assert core.constants.J2_SUN == pytest.approx(2.0e-7, rel=1e-5)

    def test_solar_irradiance(self):
        """Test total solar irradiance (solar constant) at 1 AU."""
        # NASA SORCE mission value (Kopp & Lean, 2011)
        assert core.constants.SOLAR_IRRADIANCE == pytest.approx(1361.0, rel=1e-10)

    def test_solar_radiation_pressure(self):
        """Test solar radiation pressure at 1 AU."""
        # P = E/c where E is solar irradiance and c is speed of light
        # Expected: 1361 / 299792458 ≈ 4.5398e-6 N/m²
        assert core.constants.SOLAR_RADIATION_PRESSURE == pytest.approx(
            4.53980733564684e-6, rel=1e-12
        )


class TestPlanetaryConstants:
    """Test planetary constants."""

    def test_gm_earth(self):
        """Test Earth's gravitational parameter."""
        # JPL DE440: 398,600.44 km³/s² = 3.986004418e14 m³/s²
        assert core.constants.GM_EARTH == pytest.approx(3.9860044e14, rel=1e-7)

    def test_r_earth(self):
        """Test Earth's equatorial radius (WGS84)."""
        assert core.constants.R_EARTH == 6_378_137.0

    def test_r_polar_earth(self):
        """Test Earth's polar radius (WGS84)."""
        assert core.constants.R_POLAR_EARTH == pytest.approx(6_356_752.314_245, rel=1e-10)

    def test_j2_earth(self):
        """Test Earth's J2 coefficient."""
        # WGS84 value
        assert core.constants.J2_EARTH == pytest.approx(1.082626683e-3, rel=1e-9)

    def test_j3_earth(self):
        """Test Earth's J3 coefficient."""
        assert core.constants.J3_EARTH == pytest.approx(-2.532435346e-6, rel=1e-8)

    def test_j4_earth(self):
        """Test Earth's J4 coefficient."""
        # Value from satellite observations
        assert core.constants.J4_EARTH == pytest.approx(-1.649e-6, rel=1e-3)

    def test_j5_earth(self):
        """Test Earth's J5 coefficient."""
        # Value from satellite observations
        # Note: This is zero for WGS84 reference ellipsoid but non-zero for actual Earth
        assert core.constants.J5_EARTH == pytest.approx(-0.21e-6, rel=1e-2)

    def test_j6_earth(self):
        """Test Earth's J6 coefficient."""
        # Value from satellite observations
        assert core.constants.J6_EARTH == pytest.approx(0.646e-6, rel=1e-3)

    def test_j_coefficients_relative_magnitudes(self):
        """Test that Earth's J coefficients follow expected magnitude hierarchy."""
        # J2 should be the dominant term (about 1000x larger than others)
        assert abs(core.constants.J2_EARTH) > abs(core.constants.J3_EARTH) * 400
        assert abs(core.constants.J2_EARTH) > abs(core.constants.J4_EARTH) * 600
        assert abs(core.constants.J2_EARTH) > abs(core.constants.J5_EARTH) * 5000
        assert abs(core.constants.J2_EARTH) > abs(core.constants.J6_EARTH) * 1500

    def test_j_coefficients_signs(self):
        """Test the signs of Earth's J coefficients."""
        # J2 is positive (oblate Earth - equatorial bulge)
        assert core.constants.J2_EARTH > 0
        # J3 is negative (pear-shaped - Northern hemisphere slightly larger)
        assert core.constants.J3_EARTH < 0
        # J4 is negative
        assert core.constants.J4_EARTH < 0
        # J5 is negative
        assert core.constants.J5_EARTH < 0
        # J6 is positive
        assert core.constants.J6_EARTH > 0

    def test_gm_mars(self):
        """Test Mars's gravitational parameter."""
        # JPL DE440: 42,828.38 km³/s²
        assert core.constants.GM_MARS == pytest.approx(4.282838e13, rel=1e-6)

    def test_gm_jupiter(self):
        """Test Jupiter's gravitational parameter."""
        # JPL DE440: 126,712,764.10 km³/s²
        assert core.constants.GM_JUPITER == pytest.approx(1.267127641e17, rel=1e-8)

    def test_gm_saturn(self):
        """Test Saturn's gravitational parameter."""
        assert core.constants.GM_SATURN == pytest.approx(3.794058484e16, rel=1e-8)

    def test_gm_uranus(self):
        """Test Uranus's gravitational parameter."""
        assert core.constants.GM_URANUS == pytest.approx(5.7945564e15, rel=1e-7)

    def test_gm_neptune(self):
        """Test Neptune's gravitational parameter."""
        assert core.constants.GM_NEPTUNE == pytest.approx(6.8365271e15, rel=1e-7)

    def test_gm_pluto(self):
        """Test Pluto's gravitational parameter."""
        assert core.constants.GM_PLUTO == pytest.approx(9.755e11, rel=1e-4)


class TestMoonConstants:
    """Test lunar constants."""

    def test_gm_moon(self):
        """Test Moon's gravitational parameter."""
        # JPL DE440: 4,902.80 km³/s²
        assert core.constants.GM_MOON == pytest.approx(4.90280e12, rel=1e-6)

    def test_r_moon(self):
        """Test Moon's equatorial radius."""
        assert core.constants.R_MOON == 1_738_100.0


class TestGalileanMoonConstants:
    """Test Galilean moon (Jupiter's major satellites) constants."""

    def test_gm_io(self):
        """Test Io's gravitational parameter."""
        # JPL DE440: 5,959.91547 km³/s²
        assert core.constants.GM_IO == pytest.approx(5.959915466180539e12, rel=1e-10)

    def test_r_io(self):
        """Test Io's mean radius."""
        # JUP365 ephemeris: 1821.49 km
        assert core.constants.R_IO == 1_821_490.0

    def test_gm_europa(self):
        """Test Europa's gravitational parameter."""
        # JPL DE440: 3,202.71210 km³/s²
        assert core.constants.GM_EUROPA == pytest.approx(3.202712099607295e12, rel=1e-10)

    def test_r_europa(self):
        """Test Europa's mean radius."""
        # JUP365 ephemeris: 1560.80 km
        assert core.constants.R_EUROPA == 1_560_800.0

    def test_gm_ganymede(self):
        """Test Ganymede's gravitational parameter."""
        # JPL DE440: 9,887.83275 km³/s²
        # Ganymede is the largest moon in the Solar System
        assert core.constants.GM_GANYMEDE == pytest.approx(9.887832752719638e12, rel=1e-10)

    def test_r_ganymede(self):
        """Test Ganymede's mean radius."""
        # JUP365 ephemeris: 2631.20 km
        # Larger than Mercury!
        assert core.constants.R_GANYMEDE == 2_631_200.0

    def test_gm_callisto(self):
        """Test Callisto's gravitational parameter."""
        # JPL DE440: 7,179.28340 km³/s²
        assert core.constants.GM_CALLISTO == pytest.approx(7.179283402579837e12, rel=1e-10)

    def test_r_callisto(self):
        """Test Callisto's mean radius."""
        # JUP365 ephemeris: 2410.30 km
        assert core.constants.R_CALLISTO == 2_410_300.0

    def test_galilean_moons_gm_hierarchy(self):
        """Test that Galilean moons GM values follow expected hierarchy."""
        # Ganymede (largest) > Callisto > Io > Europa (smallest)
        assert core.constants.GM_GANYMEDE > core.constants.GM_CALLISTO
        assert core.constants.GM_CALLISTO > core.constants.GM_IO
        assert core.constants.GM_IO > core.constants.GM_EUROPA


class TestSaturnianMoonConstants:
    """Test Saturnian moon constants."""

    def test_gm_titan(self):
        """Test Titan's gravitational parameter."""
        # JPL DE440: 8,978.13710 km³/s²
        # Titan is the only moon with a substantial atmosphere
        assert core.constants.GM_TITAN == pytest.approx(8.978137095521046e12, rel=1e-10)

    def test_r_titan(self):
        """Test Titan's mean radius."""
        # SAT441 ephemeris: 2574.76 km
        # Second-largest moon in the Solar System
        assert core.constants.R_TITAN == 2_574_760.0


class TestNeptunianMoonConstants:
    """Test Neptunian moon constants."""

    def test_gm_triton(self):
        """Test Triton's gravitational parameter."""
        # JPL DE440: 1,428.49546 km³/s²
        # Triton orbits Neptune retrograde
        assert core.constants.GM_TRITON == pytest.approx(1.428495462910464e12, rel=1e-10)

    def test_r_triton(self):
        """Test Triton's mean radius."""
        # NEP097 ephemeris: 1352.60 km
        assert core.constants.R_TRITON == 1_352_600.0


class TestConversionFactors:
    """Test unit conversion factors."""

    def test_km_to_m(self):
        """Test kilometer to meter conversion factor."""
        assert core.constants.KM_TO_M == 1_000.0

    def test_m_to_km(self):
        """Test meter to kilometer conversion factor."""
        assert core.constants.M_TO_KM == pytest.approx(1.0 / 1_000.0, rel=1e-10)

    def test_deg_to_rad(self):
        """Test degree to radian conversion factor."""
        import math

        assert core.constants.DEG_TO_RAD == pytest.approx(math.pi / 180.0, rel=1e-15)

    def test_rad_to_deg(self):
        """Test radian to degree conversion factor."""
        import math

        assert core.constants.RAD_TO_DEG == pytest.approx(180.0 / math.pi, rel=1e-15)

    def test_day_to_sec(self):
        """Test day to second conversion factor."""
        assert core.constants.DAY_TO_SEC == 86_400.0

    def test_sec_to_day(self):
        """Test second to day conversion factor."""
        assert core.constants.SEC_TO_DAY == pytest.approx(1.0 / 86_400.0, rel=1e-10)


class TestEpochReferences:
    """Test time reference epochs."""

    def test_j2000_tt(self):
        """Test J2000 epoch in Julian Date."""
        assert core.constants.J2000_TT == 2_451_545.0

    def test_j2000_mjd(self):
        """Test J2000 epoch in Modified Julian Date."""
        assert core.constants.J2000_MJD == 51_544.5


class TestConversionFunctions:
    """Test conversion helper functions."""

    def test_km_to_m_function(self):
        """Test kilometer to meter conversion function."""
        assert core.constants.km_to_m(1.0) == 1_000.0
        assert core.constants.km_to_m(5.5) == 5_500.0

    def test_m_to_km_function(self):
        """Test meter to kilometer conversion function."""
        assert core.constants.m_to_km(1_000.0) == 1.0
        assert core.constants.m_to_km(5_500.0) == 5.5

    def test_deg_to_rad_function(self):
        """Test degree to radian conversion function."""
        import math

        assert core.constants.deg_to_rad(180.0) == pytest.approx(math.pi, rel=1e-15)
        assert core.constants.deg_to_rad(90.0) == pytest.approx(math.pi / 2.0, rel=1e-15)
        assert core.constants.deg_to_rad(0.0) == 0.0

    def test_rad_to_deg_function(self):
        """Test radian to degree conversion function."""
        import math

        assert core.constants.rad_to_deg(math.pi) == pytest.approx(180.0, rel=1e-15)
        assert core.constants.rad_to_deg(math.pi / 2.0) == pytest.approx(90.0, rel=1e-15)
        assert core.constants.rad_to_deg(0.0) == 0.0

    def test_days_to_sec_function(self):
        """Test days to seconds conversion function."""
        assert core.constants.days_to_sec(1.0) == 86_400.0
        assert core.constants.days_to_sec(0.5) == 43_200.0

    def test_sec_to_days_function(self):
        """Test seconds to days conversion function."""
        assert core.constants.sec_to_days(86_400.0) == 1.0
        assert core.constants.sec_to_days(43_200.0) == 0.5


class TestAtmosphericConstants:
    """Test Earth atmospheric model constants."""

    def test_h0_earth(self):
        """Test Earth's atmospheric scale height."""
        assert core.constants.H0_EARTH == 8_500.0

    def test_rho0_earth(self):
        """Test Earth's atmospheric density at sea level."""
        assert core.constants.RHO0_EARTH == 1.225


class TestSemiMajorAxes:
    """Test planetary semi-major axis constants."""

    def test_a_mercury(self):
        """Test Mercury's semi-major axis."""
        # JPL value: 0.38709927 AU
        expected = 0.38709927 * core.constants.AU
        assert core.constants.A_MERCURY == pytest.approx(expected, rel=1e-8)

    def test_a_venus(self):
        """Test Venus's semi-major axis."""
        # JPL value: 0.72333566 AU
        expected = 0.72333566 * core.constants.AU
        assert core.constants.A_VENUS == pytest.approx(expected, rel=1e-8)

    def test_a_earth(self):
        """Test Earth's semi-major axis."""
        # JPL value: 1.00000261 AU (Earth-Moon barycenter)
        expected = 1.00000261 * core.constants.AU
        assert core.constants.A_EARTH == pytest.approx(expected, rel=1e-8)

    def test_a_mars(self):
        """Test Mars's semi-major axis."""
        # JPL value: 1.52371034 AU
        expected = 1.52371034 * core.constants.AU
        assert core.constants.A_MARS == pytest.approx(expected, rel=1e-8)

    def test_a_jupiter(self):
        """Test Jupiter's semi-major axis."""
        # JPL value: 5.20288700 AU
        expected = 5.20288700 * core.constants.AU
        assert core.constants.A_JUPITER == pytest.approx(expected, rel=1e-8)

    def test_a_saturn(self):
        """Test Saturn's semi-major axis."""
        # JPL value: 9.53667594 AU
        expected = 9.53667594 * core.constants.AU
        assert core.constants.A_SATURN == pytest.approx(expected, rel=1e-8)

    def test_a_uranus(self):
        """Test Uranus's semi-major axis."""
        # JPL value: 19.18916464 AU
        expected = 19.18916464 * core.constants.AU
        assert core.constants.A_URANUS == pytest.approx(expected, rel=1e-8)

    def test_a_neptune(self):
        """Test Neptune's semi-major axis."""
        # JPL value: 30.06992276 AU
        expected = 30.06992276 * core.constants.AU
        assert core.constants.A_NEPTUNE == pytest.approx(expected, rel=1e-8)

    def test_semi_major_axis_ordering(self):
        """Test that semi-major axes increase with distance from Sun."""
        assert core.constants.A_MERCURY < core.constants.A_VENUS
        assert core.constants.A_VENUS < core.constants.A_EARTH
        assert core.constants.A_EARTH < core.constants.A_MARS
        assert core.constants.A_MARS < core.constants.A_JUPITER
        assert core.constants.A_JUPITER < core.constants.A_SATURN
        assert core.constants.A_SATURN < core.constants.A_URANUS
        assert core.constants.A_URANUS < core.constants.A_NEPTUNE


class TestSphereOfInfluence:
    """Test planetary sphere of influence radii."""

    def test_soi_formula_earth(self):
        """Test that Earth's SOI matches the formula r_SOI = a × (GM_planet / GM_sun)^(2/5)."""
        a = core.constants.A_EARTH
        gm_ratio = core.constants.GM_EARTH / core.constants.GM_SUN
        expected_soi = a * (gm_ratio ** (2.0 / 5.0))
        assert core.constants.R_SOI_EARTH == pytest.approx(expected_soi, rel=1e-10)

    def test_soi_formula_jupiter(self):
        """Test that Jupiter's SOI matches the formula."""
        a = core.constants.A_JUPITER
        gm_ratio = core.constants.GM_JUPITER / core.constants.GM_SUN
        expected_soi = a * (gm_ratio ** (2.0 / 5.0))
        assert core.constants.R_SOI_JUPITER == pytest.approx(expected_soi, rel=1e-10)

    def test_soi_formula_mars(self):
        """Test that Mars's SOI matches the formula."""
        a = core.constants.A_MARS
        gm_ratio = core.constants.GM_MARS / core.constants.GM_SUN
        expected_soi = a * (gm_ratio ** (2.0 / 5.0))
        assert core.constants.R_SOI_MARS == pytest.approx(expected_soi, rel=1e-10)

    def test_moon_within_earth_soi(self):
        """Test that the Moon's orbital radius is well within Earth's sphere of influence."""
        # Moon's mean orbital radius: ~384,400 km
        moon_orbit_radius = 384_400_000.0  # meters
        # Earth's SOI should be ~924,000 km, much larger than Moon's orbit
        assert core.constants.R_SOI_EARTH > moon_orbit_radius
        # Verify it's significantly larger (at least 2x)
        assert core.constants.R_SOI_EARTH > 2.0 * moon_orbit_radius

    def test_saturn_largest_soi(self):
        """Test that Saturn has the largest sphere of influence."""
        # Saturn has the largest SOI due to its great distance from the Sun
        # (even though Jupiter has more mass, distance wins in the r_SOI formula)
        assert core.constants.R_SOI_SATURN > core.constants.R_SOI_JUPITER
        assert core.constants.R_SOI_SATURN > core.constants.R_SOI_URANUS
        assert core.constants.R_SOI_SATURN > core.constants.R_SOI_EARTH
        assert core.constants.R_SOI_SATURN > core.constants.R_SOI_MARS
        assert core.constants.R_SOI_SATURN > core.constants.R_SOI_VENUS
        assert core.constants.R_SOI_SATURN > core.constants.R_SOI_MERCURY

    def test_soi_mercury_magnitude(self):
        """Test Mercury's SOI is approximately 112,000 km."""
        # Expected: ~112,000 km based on formula
        expected_km = 112_000_000.0  # meters (112,000 km)
        assert core.constants.R_SOI_MERCURY == pytest.approx(expected_km, rel=0.01)

    def test_soi_venus_magnitude(self):
        """Test Venus's SOI is approximately 616,000 km."""
        # Expected: ~616,000 km based on formula
        expected_km = 616_000_000.0  # meters (616,000 km)
        assert core.constants.R_SOI_VENUS == pytest.approx(expected_km, rel=0.01)

    def test_soi_earth_magnitude(self):
        """Test Earth's SOI is approximately 924,000 km."""
        # Expected: ~924,000 km based on formula
        expected_km = 924_000_000.0  # meters (924,000 km)
        assert core.constants.R_SOI_EARTH == pytest.approx(expected_km, rel=0.01)

    def test_soi_mars_magnitude(self):
        """Test Mars's SOI is approximately 578,000 km."""
        # Expected: ~578,000 km based on formula
        expected_km = 578_000_000.0  # meters (578,000 km)
        assert core.constants.R_SOI_MARS == pytest.approx(expected_km, rel=0.01)

    def test_all_soi_values_positive(self):
        """Test that all SOI radii are positive."""
        assert core.constants.R_SOI_MERCURY > 0
        assert core.constants.R_SOI_VENUS > 0
        assert core.constants.R_SOI_EARTH > 0
        assert core.constants.R_SOI_MARS > 0
        assert core.constants.R_SOI_JUPITER > 0
        assert core.constants.R_SOI_SATURN > 0
        assert core.constants.R_SOI_URANUS > 0
        assert core.constants.R_SOI_NEPTUNE > 0

    def test_soi_all_formulas(self):
        """Test that all planet SOI values match the formula r_SOI = a × (GM_planet / GM_sun)^(2/5)."""
        planets = [
            (
                "MERCURY",
                core.constants.A_MERCURY,
                core.constants.GM_MERCURY,
                core.constants.R_SOI_MERCURY,
            ),
            ("VENUS", core.constants.A_VENUS, core.constants.GM_VENUS, core.constants.R_SOI_VENUS),
            ("EARTH", core.constants.A_EARTH, core.constants.GM_EARTH, core.constants.R_SOI_EARTH),
            ("MARS", core.constants.A_MARS, core.constants.GM_MARS, core.constants.R_SOI_MARS),
            (
                "JUPITER",
                core.constants.A_JUPITER,
                core.constants.GM_JUPITER,
                core.constants.R_SOI_JUPITER,
            ),
            (
                "SATURN",
                core.constants.A_SATURN,
                core.constants.GM_SATURN,
                core.constants.R_SOI_SATURN,
            ),
            (
                "URANUS",
                core.constants.A_URANUS,
                core.constants.GM_URANUS,
                core.constants.R_SOI_URANUS,
            ),
            (
                "NEPTUNE",
                core.constants.A_NEPTUNE,
                core.constants.GM_NEPTUNE,
                core.constants.R_SOI_NEPTUNE,
            ),
        ]

        for name, a, gm, r_soi in planets:
            gm_ratio = gm / core.constants.GM_SUN
            expected_soi = a * (gm_ratio ** (2.0 / 5.0))
            assert r_soi == pytest.approx(expected_soi, rel=1e-10), f"{name} SOI mismatch"


class TestRelationships:
    """Test relationships between constants."""

    def test_earth_flattening(self):
        """Test Earth's flattening factor calculation."""
        # Flattening f = (R_eq - R_polar) / R_eq
        # WGS84 flattening: 1/298.257223563
        r_eq = core.constants.R_EARTH
        r_polar = core.constants.R_POLAR_EARTH
        flattening = (r_eq - r_polar) / r_eq
        expected_flattening = 1.0 / 298.257223563
        assert flattening == pytest.approx(expected_flattening, rel=1e-9)

    def test_reciprocal_conversions(self):
        """Test that conversion factors are reciprocals."""
        assert core.constants.KM_TO_M * core.constants.M_TO_KM == pytest.approx(1.0, rel=1e-15)
        assert core.constants.DAY_TO_SEC * core.constants.SEC_TO_DAY == pytest.approx(
            1.0, rel=1e-15
        )
        assert core.constants.DEG_TO_RAD * core.constants.RAD_TO_DEG == pytest.approx(
            1.0, rel=1e-15
        )

    def test_gm_hierarchy(self):
        """Test that gravitational parameters follow expected hierarchy."""
        # Sun >> Jupiter >> Saturn >> Uranus ~ Neptune >> Earth >> Mars >> Moon
        assert core.constants.GM_SUN > core.constants.GM_JUPITER
        assert core.constants.GM_JUPITER > core.constants.GM_SATURN
        assert core.constants.GM_SATURN > core.constants.GM_EARTH
        assert core.constants.GM_EARTH > core.constants.GM_MARS
        assert core.constants.GM_MARS > core.constants.GM_MOON

    def test_solar_radiation_pressure_derivation(self):
        """Test that solar radiation pressure equals irradiance divided by speed of light."""
        # P = E/c (radiation pressure = energy flux / speed of light)
        expected_pressure = core.constants.SOLAR_IRRADIANCE / core.constants.C
        assert core.constants.SOLAR_RADIATION_PRESSURE == pytest.approx(
            expected_pressure, rel=1e-15
        )
