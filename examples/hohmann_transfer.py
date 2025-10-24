"""
Hohmann Transfer Examples
==========================

This example demonstrates orbital transfer maneuvers using Hohmann transfers,
the most fuel-efficient two-impulse transfer between circular coplanar orbits.

Examples include:
1. LEO to GEO transfer (Low Earth Orbit to Geostationary)
2. LEO to Lunar transfer orbit
3. Interplanetary transfers (Earth-Mars)

Physical Background
-------------------
A Hohmann transfer uses two impulsive burns:
1. First burn: Raise (or lower) periapsis to target orbit
2. Coast in elliptical transfer orbit
3. Second burn: Circularize at target orbit

The Hohmann transfer minimizes delta-v for transfers between circular orbits
but takes longer than higher-energy trajectories.
"""

import numpy as np
from astrora._core import hohmann_phase_angle, hohmann_synodic_period, hohmann_transfer
from astrora.bodies import Earth

# Useful constants
EARTH_RADIUS = 6378.137  # km
LEO_ALTITUDE = 400  # km (typical ISS altitude)
GEO_ALTITUDE = 35786  # km
LUNAR_DISTANCE = 384400  # km


def print_transfer_summary(result, name, r_initial, r_final):
    """Print a formatted summary of a Hohmann transfer."""
    print(f"\n{'='*70}")
    print(f"{name}")
    print(f"{'='*70}")
    print(f"Initial orbit radius:       {r_initial:,.0f} km ({r_initial/EARTH_RADIUS:.2f} R⊕)")
    print(f"Final orbit radius:         {r_final:,.0f} km ({r_final/EARTH_RADIUS:.2f} R⊕)")
    print(f"\nTransfer Orbit:")
    print(f"  Semi-major axis:          {result['transfer_sma']/1000:,.0f} km")
    print(f"  Eccentricity:             {result['transfer_eccentricity']:.4f}")
    print(f"  Periapsis velocity:       {result['v_transfer_periapsis']/1000:.3f} km/s")
    print(f"  Apoapsis velocity:        {result['v_transfer_apoapsis']/1000:.3f} km/s")
    print(f"\nDelta-v Budget:")
    print(f"  First burn (ΔV₁):         {result['delta_v1']/1000:.3f} km/s")
    print(f"  Second burn (ΔV₂):        {result['delta_v2']/1000:.3f} km/s")
    print(f"  Total delta-v:            {result['delta_v_total']/1000:.3f} km/s")
    print(f"\nTiming:")
    print(
        f"  Transfer time:            {result['transfer_time']/3600:.2f} hours ({result['transfer_time']/86400:.3f} days)"
    )
    print(f"  Initial orbit velocity:   {result['v_initial']/1000:.3f} km/s")
    print(f"  Final orbit velocity:     {result['v_final']/1000:.3f} km/s")
    print(f"{'='*70}\n")


def example_leo_to_geo():
    """
    Example 1: LEO to GEO transfer

    This is a classic maneuver for deploying geostationary satellites.
    Typical launch sequence:
    1. Launch to LEO parking orbit (~400 km)
    2. Coast to optimal burn location
    3. Perform Hohmann transfer to GTO (Geostationary Transfer Orbit)
    4. Coast to GEO altitude (35,786 km)
    5. Circularize at GEO
    """
    r_leo = EARTH_RADIUS + LEO_ALTITUDE
    r_geo = EARTH_RADIUS + GEO_ALTITUDE

    result = hohmann_transfer(
        r_initial=r_leo * 1000,  # Convert to meters
        r_final=r_geo * 1000,
        mu=Earth.mu,  # Earth's gravitational parameter
    )

    print_transfer_summary(result, "LEO to GEO Transfer", r_leo, r_geo)

    print("Mission Notes:")
    print("  - Total ΔV is ~3.9 km/s for the orbital transfer")
    print("  - Add ~9.4 km/s for initial LEO insertion from ground")
    print("  - Transfer time is ~5.25 hours (half of transfer orbit period)")
    print("  - This is the minimum-energy transfer (Hohmann is optimal)")
    print("  - Real missions may use slightly faster transfers for scheduling")


def example_leo_to_lunar():
    """
    Example 2: LEO to Lunar Transfer Orbit

    This shows the first step of a lunar mission. The full mission would
    include:
    1. LEO parking orbit
    2. Trans-Lunar Injection (TLI) burn
    3. Coast to Moon's sphere of influence
    4. Lunar Orbit Insertion (LOI) burn
    """
    r_leo = EARTH_RADIUS + LEO_ALTITUDE
    r_lunar = LUNAR_DISTANCE

    result = hohmann_transfer(r_initial=r_leo * 1000, r_final=r_lunar * 1000, mu=Earth.mu)

    print_transfer_summary(result, "LEO to Lunar Transfer Orbit", r_leo, r_lunar)

    print("Mission Notes:")
    print("  - This is the Trans-Lunar Injection (TLI) maneuver")
    print("  - Transfer time is ~5 days (Apollo missions took 3 days with faster trajectory)")
    print("  - Moon's gravity will assist in capture (not included in this calculation)")
    print("  - Additional ΔV needed for Lunar Orbit Insertion (LOI)")
    print("  - Apollo missions used ~3.1 km/s for TLI from LEO")


def example_earth_mars_comparison():
    """
    Example 3: Compare Hohmann transfer to Mars

    This demonstrates an interplanetary Hohmann transfer.
    Note: This uses circular, coplanar orbits as approximation.
    Real missions must account for orbital eccentricity and inclination.
    """
    # Approximate orbital radii (semi-major axes)
    r_earth = 149.6e6  # km (1 AU)
    r_mars = 227.9e6  # km (1.52 AU)

    # Sun's gravitational parameter
    mu_sun = 1.32712440018e11  # km³/s²

    result = hohmann_transfer(
        r_initial=r_earth * 1000, r_final=r_mars * 1000, mu=mu_sun * 1e9  # Convert to m³/s²
    )

    print_transfer_summary(result, "Earth-Mars Hohmann Transfer", r_earth, r_mars)

    # Calculate phase angle and synodic period
    phase_angle_rad = hohmann_phase_angle(
        r_initial=r_earth * 1000, r_final=r_mars * 1000, mu=mu_sun * 1e9
    )

    synodic_period_sec = hohmann_synodic_period(
        r_initial=r_earth * 1000, r_final=r_mars * 1000, mu=mu_sun * 1e9
    )

    print("Launch Window Analysis:")
    print(f"  Required phase angle:     {np.rad2deg(phase_angle_rad):.2f}°")
    print(
        f"  Synodic period:           {synodic_period_sec/86400:.1f} days ({synodic_period_sec/(86400*365.25):.2f} years)"
    )
    print(f"  Launch windows occur every ~{synodic_period_sec/(86400*30.44):.1f} months")
    print()

    print("Mission Notes:")
    print("  - Transfer time is ~259 days (~8.5 months)")
    print("  - Launch windows occur roughly every 26 months")
    print("  - Must launch when Earth-Mars phase angle is correct")
    print("  - Real missions account for planetary orbital eccentricity")
    print("  - Faster transfers are possible with more delta-v")
    print("  - This is the minimum-energy transfer to Mars")


def example_altitude_changes():
    """
    Example 4: Series of small altitude changes

    Demonstrates how delta-v scales with orbit radius change.
    """
    print(f"\n{'='*70}")
    print("Altitude Change Study: LEO Altitude Adjustments")
    print(f"{'='*70}\n")

    base_altitude = 400  # km
    altitude_changes = [10, 50, 100, 200, 500]

    print(f"Base orbit: {base_altitude} km altitude ({EARTH_RADIUS + base_altitude:.0f} km radius)")
    print(f"\n{'Δh (km)':>10} {'Total ΔV (m/s)':>18} {'Transfer Time':>18}")
    print("-" * 50)

    for delta_h in altitude_changes:
        r_initial = (EARTH_RADIUS + base_altitude) * 1000
        r_final = (EARTH_RADIUS + base_altitude + delta_h) * 1000

        result = hohmann_transfer(r_initial, r_final, Earth.mu)

        print(
            f"{delta_h:>10} {result['delta_v_total']:>18.2f} {result['transfer_time']/60:>15.1f} min"
        )

    print()
    print("Observations:")
    print("  - Small altitude changes require less delta-v")
    print("  - Delta-v scales roughly linearly with altitude change")
    print("  - Transfer time increases slightly with altitude")
    print("  - For station-keeping, these are typical maneuver sizes")


if __name__ == "__main__":
    print(
        """
╔══════════════════════════════════════════════════════════════════╗
║     Hohmann Transfer Examples                                    ║
║     Using Astrora's Hohmann Transfer Calculator                  ║
╚══════════════════════════════════════════════════════════════════╝

Hohmann transfers are the most fuel-efficient way to move between
circular coplanar orbits. They require exactly two impulsive burns
and follow an elliptical transfer trajectory.

These examples demonstrate common orbital transfer scenarios.
"""
    )

    # Run all examples
    example_leo_to_geo()
    print("\n" + "=" * 70 + "\n")

    example_leo_to_lunar()
    print("\n" + "=" * 70 + "\n")

    example_earth_mars_comparison()
    print("\n" + "=" * 70 + "\n")

    example_altitude_changes()

    print(
        """
╔══════════════════════════════════════════════════════════════════╗
║  Key Takeaways                                                   ║
╚══════════════════════════════════════════════════════════════════╝

1. Hohmann transfers minimize delta-v but take longer
2. For large radius ratios (>11.94), bi-elliptic transfers can save fuel
3. Launch windows for interplanetary transfers repeat every synodic period
4. Delta-v requirements scale with the magnitude of the orbit change

For more examples, see:
  - examples/bielliptic_transfer.py - Alternative 3-burn transfers
  - examples/plane_change.py - Inclination changes
  - examples/earth_mars_transfer.py - Real planetary positions
  - examples/porkchop_plot.py - Launch window optimization
"""
    )
