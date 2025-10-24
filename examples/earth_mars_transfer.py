"""
Earth-Mars Transfer Mission Example
====================================

This example demonstrates how to calculate an interplanetary transfer
from Earth to Mars using Lambert's problem solver.

The example shows:
1. Getting planetary positions at departure and arrival times
2. Solving Lambert's problem for the transfer trajectory
3. Computing required delta-v at departure and arrival
4. Displaying orbital elements of the transfer trajectory

Physical Background
-------------------
Interplanetary transfers use Lambert's problem to find the trajectory
connecting two position vectors in a given time of flight. The Hohmann
transfer to Mars typically requires:
- Delta-v at Earth departure: ~3.6 km/s (from LEO)
- Delta-v at Mars arrival: ~2.0 km/s (to Mars orbit)
- Transfer time: ~250-260 days
"""

import numpy as np
from astrora._core import lambert_solve

# Solar gravitational parameter (km³/s²)
MU_SUN = 1.32712440018e11  # km³/s²


def get_planet_state_simple(body_name, angle_deg):
    """
    Get simplified heliocentric position and velocity using circular orbit approximation.

    This uses circular, coplanar orbits as an approximation for demonstration purposes.
    For precise mission planning, use actual ephemeris data.

    Parameters
    ----------
    body_name : str
        Name of the planet ('earth', 'mars')
    angle_deg : float
        True anomaly angle in degrees

    Returns
    -------
    position : np.ndarray
        Position vector [x, y, z] in km
    velocity : np.ndarray
        Velocity vector [vx, vy, vz] in km/s
    """
    # Orbital parameters (circular orbit approximation)
    if body_name.lower() == "earth":
        a = 149.6e6  # km (1 AU)
        mu = MU_SUN
    elif body_name.lower() == "mars":
        a = 227.9e6  # km (1.52 AU)
        mu = MU_SUN
    else:
        raise ValueError(f"Unknown planet: {body_name}")

    # Circular orbit velocity
    v_circ = np.sqrt(mu / a)

    # Convert angle to radians
    theta = np.deg2rad(angle_deg)

    # Position in orbital plane
    position = np.array([a * np.cos(theta), a * np.sin(theta), 0.0])

    # Velocity perpendicular to radius
    velocity = np.array([-v_circ * np.sin(theta), v_circ * np.cos(theta), 0.0])

    return position, velocity


def compute_earth_mars_transfer_simple(tof_days, earth_angle=0, short_way=True):
    """
    Compute an Earth-Mars transfer trajectory using circular orbit approximation.

    Parameters
    ----------
    tof_days : float
        Time of flight in days
    earth_angle : float
        Earth's true anomaly at departure (degrees)
    short_way : bool
        Whether to use short-way transfer (< 180°)

    Returns
    -------
    dict
        Transfer trajectory information including delta-v requirements
    """
    tof_seconds = tof_days * 86400  # Convert to seconds

    print(f"\n{'='*60}")
    print(f"Earth-Mars Transfer Mission (Simplified)")
    print(f"{'='*60}")
    print(f"Time of flight: {tof_days:.1f} days ({tof_days/30.44:.1f} months)")
    print(f"Earth departure angle: {earth_angle:.1f}°")
    print(f"{'='*60}\n")

    # Get Earth state at departure
    r_earth, v_earth = get_planet_state_simple("earth", earth_angle)
    print(f"Earth at departure:")
    print(f"  Position: [{r_earth[0]:,.0f}, {r_earth[1]:,.0f}, {r_earth[2]:,.0f}] km")
    print(f"  Velocity: [{v_earth[0]:.3f}, {v_earth[1]:.3f}, {v_earth[2]:.3f}] km/s")
    print(f"  |r| = {np.linalg.norm(r_earth):,.0f} km ({np.linalg.norm(r_earth)/1.496e8:.3f} AU)")
    print(f"  |v| = {np.linalg.norm(v_earth):.3f} km/s\n")

    # Calculate Mars position at arrival
    # Mars orbital period: ~687 days
    # Earth orbital period: 365.25 days
    # Mars moves slower, so we need to figure out where it will be
    earth_period = 365.25  # days
    mars_period = 686.98  # days

    # Earth's angular velocity (deg/day)
    earth_angular_vel = 360.0 / earth_period
    # Mars's angular velocity (deg/day)
    mars_angular_vel = 360.0 / mars_period

    # Mars angle at arrival (starting from Earth's position)
    # For Hohmann transfer, Mars should be ~44° ahead
    mars_angle = earth_angle + (mars_angular_vel * tof_days)

    # Get Mars state at arrival
    r_mars, v_mars = get_planet_state_simple("mars", mars_angle)
    print(f"Mars at arrival:")
    print(f"  Position: [{r_mars[0]:,.0f}, {r_mars[1]:,.0f}, {r_mars[2]:,.0f}] km")
    print(f"  Velocity: [{v_mars[0]:.3f}, {v_mars[1]:.3f}, {v_mars[2]:.3f}] km/s")
    print(f"  |r| = {np.linalg.norm(r_mars):,.0f} km ({np.linalg.norm(r_mars)/1.496e8:.3f} AU)")
    print(f"  |v| = {np.linalg.norm(v_mars):.3f} km/s\n")

    # Solve Lambert's problem
    print(f"Solving Lambert's problem...")
    result = lambert_solve(
        r1=r_earth, r2=r_mars, tof=tof_seconds, mu=MU_SUN, short_way=short_way, revs=0
    )

    # Extract velocities
    v1 = result["v1"]  # Departure velocity
    v2 = result["v2"]  # Arrival velocity

    print(f"Transfer orbit solution:")
    print(f"  v1: [{v1[0]:.3f}, {v1[1]:.3f}, {v1[2]:.3f}] km/s")
    print(f"  v2: [{v2[0]:.3f}, {v2[1]:.3f}, {v2[2]:.3f}] km/s")
    print(f"  |v1| = {np.linalg.norm(v1):.3f} km/s")
    print(f"  |v2| = {np.linalg.norm(v2):.3f} km/s\n")

    # Compute delta-v requirements
    dv_departure = np.linalg.norm(v1 - v_earth)
    dv_arrival = np.linalg.norm(v2 - v_mars)
    dv_total = dv_departure + dv_arrival

    # Compute C3 characteristic energy
    v_infinity = v1 - v_earth
    c3 = np.linalg.norm(v_infinity) ** 2

    print(f"Delta-v Requirements:")
    print(f"{'='*60}")
    print(f"  At Earth departure: {dv_departure:.3f} km/s")
    print(f"  At Mars arrival:    {dv_arrival:.3f} km/s")
    print(f"  Total delta-v:      {dv_total:.3f} km/s")
    print(f"{'='*60}\n")

    print(f"C3 Characteristic Energy:")
    print(f"{'='*60}")
    print(f"  C3 = {c3:.2f} km²/s²")
    print(f"  v∞ = {np.sqrt(c3):.3f} km/s (hyperbolic excess velocity)")
    print(f"\n  Launch vehicle capability required:")
    if c3 < 15:
        print(f"    ✓ Atlas V 551 capable (C3 max ~15 km²/s²)")
        print(f"    ✓ Falcon 9 capable (C3 max ~30 km²/s²)")
    elif c3 < 25:
        print(f"    ✗ Atlas V 551 insufficient")
        print(f"    ✓ Delta IV Heavy capable (C3 max ~25 km²/s²)")
        print(f"    ✓ Falcon 9 capable (C3 max ~30 km²/s²)")
    elif c3 < 30:
        print(f"    ✗ Delta IV Heavy insufficient")
        print(f"    ✓ Falcon 9 (expendable) capable (C3 max ~30 km²/s²)")
    elif c3 < 40:
        print(f"    ✗ Falcon 9 insufficient")
        print(f"    ✓ Falcon Heavy capable (C3 max ~40 km²/s²)")
    else:
        print(f"    ⚠ Requires Falcon Heavy or SLS (C3 max ~45 km²/s²)")
    print(f"{'='*60}\n")

    # Orbital elements of transfer orbit
    print(f"Transfer Orbit Elements:")
    print(f"  Semi-major axis:    {result['a']:,.0f} km ({result['a']/1.496e8:.3f} AU)")
    print(f"  Eccentricity:       {result['e']:.4f}")

    # Check if elliptic, parabolic, or hyperbolic
    e = result["e"]
    if e < 1:
        print(f"  Orbit type:         Elliptic (e < 1)")
        # Calculate period
        a = result["a"]
        period_seconds = 2 * np.pi * np.sqrt(a**3 / MU_SUN)
        period_days = period_seconds / 86400
        print(f"  Orbital period:     {period_days:.1f} days ({period_days/365.25:.2f} years)")
    elif e > 1:
        print(f"  Orbit type:         Hyperbolic (e > 1)")
    else:
        print(f"  Orbit type:         Parabolic (e = 1)")

    print(f"\n{'='*60}\n")

    return {
        "r_earth": r_earth,
        "v_earth": v_earth,
        "r_mars": r_mars,
        "v_mars": v_mars,
        "v1": v1,
        "v2": v2,
        "dv_departure": dv_departure,
        "dv_arrival": dv_arrival,
        "dv_total": dv_total,
        "c3": c3,
        "v_infinity": np.sqrt(c3),
        "tof_days": tof_days,
        "transfer_orbit": result,
    }


def example_hohmann_transfer():
    """
    Example: Hohmann transfer to Mars.

    This uses circular orbit approximations to demonstrate a typical
    minimum-energy transfer. For precise mission planning with real
    planetary positions, use actual ephemeris data and porkchop plots.
    """
    print("\n" + "=" * 60)
    print("Example 1: Hohmann Transfer to Mars")
    print("=" * 60)

    # Hohmann transfer time for Earth-Mars is ~259 days
    result = compute_earth_mars_transfer_simple(tof_days=259, earth_angle=0, short_way=True)

    print("\nMission Summary:")
    print(f"  This represents a Hohmann transfer to Mars.")
    print(f"  Total mission delta-v: {result['dv_total']:.2f} km/s")
    print(f"  C3 energy: {result['c3']:.2f} km²/s²")
    print(f"  Transfer time: {result['tof_days']:.0f} days")
    print(f"  Transfer orbit eccentricity: {result['transfer_orbit']['e']:.3f}")


def example_fast_transfer():
    """
    Example: Faster transfer to Mars (higher energy, more delta-v).
    """
    print("\n" + "=" * 60)
    print("Example 2: Fast Transfer to Mars (200 days)")
    print("=" * 60)

    result = compute_earth_mars_transfer_simple(tof_days=200, earth_angle=0, short_way=True)

    print("\nMission Summary:")
    print(f"  Faster transfer requires more delta-v than Hohmann transfer.")
    print(f"  Total mission delta-v: {result['dv_total']:.2f} km/s")
    print(f"  C3 energy: {result['c3']:.2f} km²/s² (vs Hohmann: lower)")
    print(f"  Transfer time: {result['tof_days']:.0f} days")
    print(f"  Transfer orbit eccentricity: {result['transfer_orbit']['e']:.3f}")
    print(f"  Trade-off: -23% time compared to Hohmann (259 days)")
    print(f"  Trade-off: Higher C3 may require larger launch vehicle")


if __name__ == "__main__":
    print(
        """
╔══════════════════════════════════════════════════════════════╗
║     Earth-Mars Transfer Mission Planning Example            ║
║     Using Astrora's Lambert Solver                           ║
╚══════════════════════════════════════════════════════════════╝

This example demonstrates interplanetary trajectory design using
Lambert's problem to solve for transfer orbits between planets.
"""
    )

    # Run examples
    example_hohmann_transfer()
    print("\n" + "=" * 60 + "\n")
    example_fast_transfer()

    print(
        """
╔══════════════════════════════════════════════════════════════╗
║  Next Steps                                                  ║
╚══════════════════════════════════════════════════════════════╝

1. Optimize launch windows using porkchop plots (see porkchop_plot.py)
2. Account for parking orbit escape and capture burns
3. Consider gravity assists for outer planet missions
4. Add trajectory correction maneuvers (TCM) for mid-course corrections

For more examples, see:
  - examples/porkchop_plot.py - Launch window optimization
  - examples/gravity_assist.py - Multi-planet flybys
  - examples/hohmann_transfer.py - Simple orbital transfers
"""
    )
