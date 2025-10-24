"""
Gravity Assist (Planetary Flyby) Examples
==========================================

This example demonstrates gravity assist maneuvers, where a spacecraft
uses a planet's gravity to change its velocity and trajectory.

Gravity assists are essential for:
- Outer planet missions (Jupiter, Saturn, Uranus, Neptune)
- Mercury missions (need to lose energy)
- Reducing launch vehicle requirements
- Enabling "Grand Tour" multi-planet trajectories

Physical Background
-------------------
In the planet's reference frame:
- Spacecraft speed unchanged (energy conserved)
- Direction changes by deflection angle δ
- Maximum deflection for closest safe approach

In the Sun's reference frame:
- Spacecraft gains/loses orbital energy
- Can achieve "free" delta-v up to 2·v∞
- Planet loses/gains equal energy (negligible effect on planet)

Famous missions using gravity assists:
- Voyager 1 & 2: Jupiter, Saturn (Uranus, Neptune for V2)
- Cassini: Venus (2x), Earth, Jupiter → Saturn
- New Horizons: Jupiter → Pluto
- Parker Solar Probe: Venus (7 flybys) → Close solar approaches
"""

import numpy as np
from astrora._core import gravity_assist, periapsis_from_b_parameter
from astrora.bodies import Earth, Jupiter, Mars, Saturn, Venus


def print_flyby_summary(result, planet_name, v_infinity, r_periapsis):
    """Print formatted summary of gravity assist results."""
    c3 = result["specific_energy"] * 2 / 1e6  # km²/s²

    print(f"\n{'='*70}")
    print(f"{planet_name} Gravity Assist")
    print(f"{'='*70}")
    print(f"Flyby Parameters:")
    print(f"  Hyperbolic excess velocity (v∞):  {v_infinity:.3f} km/s")
    print(f"  Periapsis radius:                  {r_periapsis:,.0f} km")
    print(f"\nHyperbolic Trajectory:")
    print(f"  Eccentricity:                      {result['eccentricity']:.4f}")
    print(f"  Semi-major axis:                   {result['semi_major_axis']/1000:,.0f} km")
    print(f"  Impact parameter (B):              {result['b_parameter']/1000:,.0f} km")
    print(f"  Deflection angle (δ):              {np.rad2deg(result['delta']):.2f}°")
    print(f"  Turn angle (θ∞):                   {np.rad2deg(result['theta_infinity']):.2f}°")
    print(f"\nVelocity Change (Heliocentric Frame):")
    print(f"  Delta-v magnitude:                 {result['delta_v_magnitude']/1000:.3f} km/s")
    print(f"  Maximum possible (2·v∞):           {2*v_infinity:.3f} km/s")
    print(
        f"  Efficiency:                        {result['delta_v_magnitude']/1000/(2*v_infinity)*100:.1f}%"
    )
    print(f"\nC3 Characteristic Energy:")
    print(f"  C3 at flyby approach:              {c3:.3f} km²/s²")
    print(f"  v∞ (planet frame):                 {v_infinity:.3f} km/s")
    print(f"  Specific energy (ε = v∞²/2):       {result['specific_energy']/1e6:.3f} km²/s²")
    print(f"\n  Note: C3 represents the hyperbolic excess energy needed")
    print(f"        to arrive at {planet_name} with v∞ = {v_infinity:.2f} km/s")
    print(f"        This determines the launch requirements from previous body")
    print(f"{'='*70}\n")


def example_jupiter_flyby():
    """
    Example 1: Jupiter Gravity Assist (Voyager-style)

    Jupiter flybys are extremely powerful due to:
    1. Large gravitational parameter (μ = 126,686,534 km³/s²)
    2. Fast orbital velocity (~13 km/s)
    3. Can provide up to ~26 km/s delta-v in solar frame

    This enabled Voyager to reach Saturn, Uranus, and Neptune.
    """
    v_infinity = 5.5  # km/s (typical approach velocity from Earth)
    r_periapsis = 350000  # km (~5 Jupiter radii, safe distance)

    result = gravity_assist(
        v_infinity=v_infinity * 1000,  # Convert to m/s
        r_periapsis=r_periapsis * 1000,
        mu=Jupiter.mu,
    )

    print_flyby_summary(result, "Jupiter", v_infinity, r_periapsis)

    print("Mission Context:")
    print("  - Typical for outer planet missions")
    print("  - Voyager 2 used Jupiter flyby to reach Saturn, Uranus, Neptune")
    print("  - Large deflection angle enables major trajectory changes")
    print("  - Can boost spacecraft to escape solar system")
    print("  - Radiation environment is challenging near Jupiter")


def example_venus_flyby():
    """
    Example 2: Venus Gravity Assist (Cassini, Parker Solar Probe)

    Venus flybys are used to:
    1. Lose orbital energy (for solar missions)
    2. Adjust trajectory for multi-planet tours
    3. Provide delta-v without propellant

    Cassini used Venus twice, then Earth, then Jupiter to reach Saturn.
    """
    v_infinity = 3.2  # km/s
    r_periapsis = 6352 + 300  # km (Venus radius + 300 km altitude)

    result = gravity_assist(
        v_infinity=v_infinity * 1000, r_periapsis=r_periapsis * 1000, mu=Venus.mu
    )

    print_flyby_summary(result, "Venus", v_infinity, r_periapsis)

    print("Mission Context:")
    print("  - Used to reduce orbital energy for inner solar system missions")
    print("  - Parker Solar Probe uses Venus flybys to lower perihelion")
    print("  - Cassini used 2 Venus flybys + 1 Earth + 1 Jupiter to reach Saturn")
    print("  - Dense atmosphere requires higher flyby altitude")


def example_mars_flyby():
    """
    Example 3: Mars Gravity Assist

    Less powerful than Jupiter but useful for:
    1. Asteroid belt missions
    2. Outer planet missions with lower launch energy
    3. Return trajectories from Mars missions
    """
    v_infinity = 2.8  # km/s
    r_periapsis = 3396 + 500  # km (Mars radius + 500 km altitude)

    result = gravity_assist(
        v_infinity=v_infinity * 1000, r_periapsis=r_periapsis * 1000, mu=Mars.mu
    )

    print_flyby_summary(result, "Mars", v_infinity, r_periapsis)

    print("Mission Context:")
    print("  - Useful for asteroid missions beyond main belt")
    print("  - Can assist sample return missions")
    print("  - Smaller effect than Jupiter but still valuable")


def example_b_plane_targeting():
    """
    Example 4: B-Plane Targeting

    The impact parameter B determines the flyby geometry.
    Mission designers target specific B values to achieve:
    1. Desired deflection angle
    2. Post-flyby trajectory
    3. Science observation geometry

    This example shows how to compute required periapsis from desired B.
    """
    print(f"\n{'='*70}")
    print("B-Plane Targeting Example: Jupiter Flyby Design")
    print(f"{'='*70}\n")

    v_infinity = 6.0  # km/s
    target_b_parameter = 500000  # km (target impact parameter)

    result = periapsis_from_b_parameter(
        v_infinity=v_infinity * 1000, b_parameter=target_b_parameter * 1000, mu=Jupiter.mu
    )

    print(f"Mission Requirements:")
    print(f"  Approach velocity (v∞):            {v_infinity:.3f} km/s")
    print(f"  Target B-parameter:                {target_b_parameter:,.0f} km")
    print(f"\nSolution:")
    print(f"  Required periapsis radius:         {result['r_periapsis']/1000:,.0f} km")
    print(f"  Periapsis altitude:                {result['r_periapsis']/1000 - 71492:,.0f} km")
    print(f"  Hyperbolic eccentricity:           {result['eccentricity']:.4f}")
    print(f"  Semi-major axis:                   {result['semi_major_axis']/1000:,.0f} km")
    print(f"\nTrajectory Properties:")
    print(f"  Deflection angle:                  {np.rad2deg(result['delta']):.2f}°")
    print(f"  Turn angle:                        {np.rad2deg(result['theta_infinity']):.2f}°")
    print(f"  Heliocentric delta-v:              {result['delta_v_magnitude']/1000:.3f} km/s")
    print(f"{'='*70}\n")

    print("Navigation Notes:")
    print("  - B-plane is perpendicular to approach asymptote")
    print("  - Trajectory correction maneuvers (TCMs) adjust B")
    print("  - Typical TCM delta-v budget: 10-50 m/s for outer planets")
    print("  - Final TCM usually performed ~1 week before encounter")


def example_flyby_comparison():
    """
    Example 5: Compare gravity assists at different planets
    """
    print(f"\n{'='*70}")
    print("Gravity Assist Comparison: Fixed Approach Velocity")
    print(f"{'='*70}\n")

    v_inf = 5.0  # km/s (fixed for comparison)

    planets = [
        ("Venus", Venus.mu, 6052 + 300),
        ("Earth", Earth.mu, 6378 + 500),
        ("Mars", Mars.mu, 3396 + 500),
        ("Jupiter", Jupiter.mu, 71492 + 10000),
        ("Saturn", Saturn.mu, 60268 + 10000),
    ]

    print(f"Fixed approach velocity: {v_inf} km/s")
    print(f"\n{'Planet':>10} {'Periapsis (km)':>15} {'Deflection (°)':>15} {'ΔV (km/s)':>12}")
    print("-" * 55)

    for name, mu, r_p in planets:
        result = gravity_assist(v_inf * 1000, r_p * 1000, mu)
        deflection = np.rad2deg(result["delta"])
        delta_v = result["delta_v_magnitude"] / 1000  # Convert to km/s

        print(f"{name:>10} {r_p:>15,.0f} {deflection:>15.2f} {delta_v:>12.3f}")

    print()
    print("Observations:")
    print("  - Jupiter provides the largest deflection and delta-v")
    print("  - Gas giants (Jupiter, Saturn) are most effective")
    print("  - Terrestrial planets have smaller but still useful effects")
    print("  - Choice depends on mission trajectory and available opportunities")


if __name__ == "__main__":
    print(
        """
╔══════════════════════════════════════════════════════════════════╗
║     Gravity Assist (Planetary Flyby) Examples                    ║
║     Using Astrora's Gravity Assist Calculator                    ║
╚══════════════════════════════════════════════════════════════════╝

Gravity assists use planetary flybys to change a spacecraft's velocity
without using propellant. They've enabled missions to:
- Reach the outer solar system (Voyager, Cassini, New Horizons)
- Achieve close solar approaches (Parker Solar Probe)
- Visit multiple planets in "Grand Tour" trajectories

The physics: In the planet's frame, speed is conserved but direction
changes. In the Sun's frame, this appears as a velocity change (ΔV).
"""
    )

    # Run all examples
    example_jupiter_flyby()
    print("\n" + "=" * 70 + "\n")

    example_venus_flyby()
    print("\n" + "=" * 70 + "\n")

    example_mars_flyby()
    print("\n" + "=" * 70 + "\n")

    example_b_plane_targeting()
    print("\n" + "=" * 70 + "\n")

    example_flyby_comparison()

    print(
        """
╔══════════════════════════════════════════════════════════════════╗
║  Advanced Topics                                                 ║
╚══════════════════════════════════════════════════════════════════╝

For mission design, consider:

1. **Powered Gravity Assists**: Add delta-v at periapsis for even more
   trajectory control (Oberth effect maximizes efficiency)

2. **Multiple Flybys**: Chain several gravity assists together
   Example: Cassini used V-V-E-J-S (Venus-Venus-Earth-Jupiter-Saturn)

3. **Timing**: Must wait for proper planetary alignment
   Some trajectories only possible every few decades

4. **Resonant Orbits**: Use gravity assists to adjust orbital period
   for repeated encounters (e.g., Parker Solar Probe)

For more mission design tools, see:
  - examples/earth_mars_transfer.py - Interplanetary transfers
  - examples/porkchop_plot.py - Launch window optimization
  - examples/lambert_solver.py - General trajectory design
"""
    )
