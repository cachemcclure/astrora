"""
Orbit Propagation with Perturbations
=====================================

This example demonstrates high-fidelity orbit propagation including:
1. J2 oblateness perturbation (Earth's equatorial bulge)
2. Atmospheric drag (exponential density model)
3. Third-body perturbations (Sun and Moon)
4. Solar radiation pressure (SRP)
5. Combined perturbation effects

The example compares Keplerian (two-body) propagation with perturbed
propagation to show the real-world effects on satellite orbits.

Physical Background
-------------------
Real satellites experience multiple perturbing forces beyond the central
body's gravity:

- **J2 Perturbation**: Earth's equatorial bulge causes orbital precession
  - Regression of nodes: Ω̇ ∝ -3/2 * J2 * n * cos(i)
  - Rotation of apsides: ω̇ ∝ 3/4 * J2 * n * (4-5sin²i)

- **Atmospheric Drag**: Exponential density model causes orbit decay
  - Drag acceleration: a_drag = -0.5 * ρ * v² * (C_D * A/m) * v̂
  - LEO satellites typically decay by ~1-10 km/day

- **Third-Body (Sun/Moon)**: Gravitational pull from other bodies
  - Critical for high-altitude orbits (GEO, HEO)
  - Causes long-term eccentricity variations

- **Solar Radiation Pressure**: Photon momentum transfer
  - Important for large area-to-mass ratio satellites
  - Acceleration: a_SRP ≈ 4.56e-6 * (A/m) * C_r [N/kg]

Typical Magnitudes (LEO at 500 km)
----------------------------------
- Two-body gravity: ~8.5 m/s²
- J2 perturbation: ~0.01 m/s² (0.1%)
- Atmospheric drag: ~1e-6 to 1e-4 m/s² (depends on solar activity)
- Third-body: ~1e-5 m/s²
- SRP: ~1e-7 to 1e-5 m/s² (depends on A/m)

References
----------
- Vallado, "Fundamentals of Astrodynamics and Applications", Ch. 8-9
- Montenbruck & Gill, "Satellite Orbits", Ch. 3
- Curtis, "Orbital Mechanics for Engineering Students", Ch. 10-12
"""

import numpy as np
from astrora._core import (
    propagate_drag_dopri5,
    propagate_j2_dop853,
    propagate_srp_dopri5,
)
from astrora.bodies import Earth


def keplerian_propagate(r0, v0, mu, t_span, dt):
    """
    Simple Keplerian (two-body) propagation for comparison.

    This uses orbital element conversion and Kepler's equation.
    For a fair comparison with perturbed propagation, we'll use
    the same integrator but without perturbations.
    """
    # For simplicity, we'll just return the analytical solution
    # In practice, we'd integrate the two-body equations

    # Calculate orbital elements
    h_vec = np.cross(r0, v0)  # Angular momentum
    h = np.linalg.norm(h_vec)

    r0_mag = np.linalg.norm(r0)
    v0_mag = np.linalg.norm(v0)

    # Specific orbital energy
    energy = v0_mag**2 / 2 - mu / r0_mag

    # Semi-major axis
    a = -mu / (2 * energy)

    # Mean motion
    n = np.sqrt(mu / a**3)

    # Period
    T = 2 * np.pi / n

    # Eccentricity vector
    e_vec = np.cross(v0, h_vec) / mu - r0 / r0_mag
    e = np.linalg.norm(e_vec)

    print(f"  Keplerian orbit elements:")
    print(f"    a = {a/1000:.1f} km")
    print(f"    e = {e:.6f}")
    print(f"    Period = {T/3600:.2f} hours")
    print(f"    Mean motion = {np.rad2deg(n)*86400:.2f} deg/day")

    return a, e, T, n


def example_j2_leo_precession():
    """
    Example 1: J2 perturbation causes nodal regression and apsidal rotation.

    Sun-synchronous orbits exploit J2 to maintain constant solar time.
    """
    print("\n" + "=" * 70)
    print("Example 1: J2 Perturbation - LEO Satellite Nodal Regression")
    print("=" * 70)

    # Initial state: LEO at 800 km altitude
    altitude = 800e3  # m
    r0_mag = Earth.R + altitude

    # Circular orbit in equatorial plane, then tilt to 98° (sun-sync)
    inclination = np.deg2rad(98.0)  # Degrees

    # Position vector (in orbital plane)
    r0 = np.array(
        [
            r0_mag * np.cos(0),
            r0_mag * np.sin(0) * np.cos(inclination),
            r0_mag * np.sin(0) * np.sin(inclination),
        ]
    )

    # Velocity for circular orbit
    v_circ = np.sqrt(Earth.mu / r0_mag)
    v0 = np.array([0, v_circ * np.cos(inclination), -v_circ * np.sin(inclination)])

    print(f"\nInitial conditions:")
    print(f"  Altitude: {altitude/1000:.1f} km")
    print(f"  Inclination: {np.rad2deg(inclination):.1f}°")
    print(f"  |r0| = {np.linalg.norm(r0)/1000:.1f} km")
    print(f"  |v0| = {np.linalg.norm(v0)/1000:.3f} km/s")

    # Keplerian orbit parameters
    keplerian_propagate(r0, v0, Earth.mu, 0, 0)

    # Propagate with J2 for 10 days
    t_final = 10 * 86400  # 10 days in seconds
    dt = 60.0  # 1 minute timestep

    print(f"\nPropagating with J2 perturbation for 10 days...")
    print(f"  J2 = {Earth.J2:.8f}")
    print(f"  Earth radius = {Earth.R/1000:.1f} km")

    # Propagate step by step
    num_steps = int(t_final / dt)
    times = []
    positions = []
    velocities = []

    r_current = r0.copy()
    v_current = v0.copy()
    t_current = 0

    for i in range(num_steps):
        times.append(t_current)
        positions.append(r_current.copy())
        velocities.append(v_current.copy())

        # Propagate one step
        r_current, v_current = propagate_j2_dop853(
            r0=r_current, v0=v_current, dt=dt, mu=Earth.mu, j2=Earth.J2, R=Earth.R, tol=1e-10
        )

        t_current += dt

    # Convert to arrays
    times = np.array(times)
    positions = np.array(positions)
    velocities = np.array(velocities)

    print(f"\nPropagation complete:")
    print(f"  Total steps: {len(times)}")
    print(f"  Final time: {times[-1]/86400:.2f} days")

    # Analyze orbital drift
    r_initial = positions[0]
    r_final = positions[-1]
    v_initial = velocities[0]
    v_final = velocities[-1]

    # Calculate change in position and velocity
    delta_r = np.linalg.norm(r_final - r_initial)
    delta_v = np.linalg.norm(v_final - v_initial)

    # Calculate altitude change
    alt_initial = np.linalg.norm(r_initial) - Earth.R
    alt_final = np.linalg.norm(r_final) - Earth.R
    delta_alt = alt_final - alt_initial

    print(f"\nOrbital changes over 10 days:")
    print(f"  Δ|r| = {delta_r/1000:.3f} km")
    print(f"  Δ|v| = {delta_v:.6f} m/s")
    print(f"  Altitude change = {delta_alt/1000:.3f} km")

    # Calculate angular drift (nodal regression)
    # RAAN change: ΔΩ ≈ -3/2 * J2 * (R/a)² * n * cos(i) * Δt
    a = r0_mag  # Circular orbit
    n = np.sqrt(Earth.mu / a**3)  # Mean motion
    t_span = t_final

    # Analytical prediction for nodal regression
    raan_rate = -1.5 * Earth.J2 * (Earth.R / a) ** 2 * n * np.cos(inclination)
    predicted_raan_change = raan_rate * t_span

    print(f"\nNodal regression (RAAN drift):")
    print(f"  Predicted rate: {np.rad2deg(raan_rate)*86400:.4f} deg/day")
    print(f"  Predicted change over 10 days: {np.rad2deg(predicted_raan_change):.4f}°")
    print(f"\n  Note: Sun-synchronous orbits use this drift to maintain")
    print(f"        constant local solar time (≈1°/day regression needed)")

    print(f"\n{'='*70}\n")


def example_atmospheric_drag():
    """
    Example 2: Atmospheric drag causes orbit decay in LEO.

    Demonstrates how drag gradually reduces orbital energy and altitude.
    """
    print("\n" + "=" * 70)
    print("Example 2: Atmospheric Drag - LEO Orbit Decay")
    print("=" * 70)

    # Initial state: LEO at 400 km (like ISS)
    altitude = 400e3  # m
    r0_mag = Earth.R + altitude

    # Circular orbit, 51.6° inclination (ISS orbit)
    inclination = np.deg2rad(51.6)

    r0 = np.array([r0_mag, 0, 0])

    v_circ = np.sqrt(Earth.mu / r0_mag)
    v0 = np.array([0, v_circ * np.cos(inclination), v_circ * np.sin(inclination)])

    print(f"\nInitial conditions (ISS-like orbit):")
    print(f"  Altitude: {altitude/1000:.1f} km")
    print(f"  Inclination: {np.rad2deg(inclination):.1f}°")

    keplerian_propagate(r0, v0, Earth.mu, 0, 0)

    # Atmospheric density model parameters
    rho0 = 5e-12  # kg/m³ at reference altitude (400 km, moderate solar activity)
    H0 = 60e3  # Scale height (m)

    # Ballistic coefficient B = m/(C_D * A)
    # For a typical satellite: m=1000 kg, C_D=2.2, A=10 m²
    # B = 1000 / (2.2 * 10) = 45.45 kg/m²
    B = 50.0  # kg/m²

    print(f"\nAtmospheric model:")
    print(f"  ρ₀ = {rho0:.2e} kg/m³ (at {altitude/1000:.0f} km)")
    print(f"  H₀ = {H0/1000:.1f} km (scale height)")
    print(f"  B = {B:.1f} kg/m² (ballistic coefficient)")

    # Propagate with drag for 30 days
    t_final = 30 * 86400  # 30 days
    dt = 600.0  # 10 minute timestep

    print(f"\nPropagating with atmospheric drag for 30 days...")

    # Propagate step by step
    num_steps = int(t_final / dt)
    times = []
    positions = []

    r_current = r0.copy()
    v_current = v0.copy()
    t_current = 0

    for i in range(num_steps):
        times.append(t_current)
        positions.append(r_current.copy())

        # Propagate one step
        r_current, v_current = propagate_drag_dopri5(
            r0=r_current,
            v0=v_current,
            dt=dt,
            mu=Earth.mu,
            R=Earth.R,
            rho0=rho0,
            H0=H0,
            B=B,
            tol=1e-8,
        )

        t_current += dt

    times = np.array(times)
    positions = np.array(positions)

    print(f"\nPropagation complete:")
    print(f"  Total steps: {len(times)}")

    # Analyze altitude decay
    altitudes = np.array([np.linalg.norm(r) - Earth.R for r in positions])

    alt_initial = altitudes[0]
    alt_final = altitudes[-1]
    total_decay = alt_initial - alt_final
    decay_rate = total_decay / (t_final / 86400)  # m/day

    print(f"\nOrbital decay analysis:")
    print(f"  Initial altitude: {alt_initial/1000:.3f} km")
    print(f"  Final altitude: {alt_final/1000:.3f} km")
    print(f"  Total decay: {total_decay/1000:.3f} km over 30 days")
    print(f"  Average decay rate: {decay_rate:.1f} m/day ({decay_rate/1000:.3f} km/day)")

    print(f"\n  Note: ISS requires periodic reboosts to maintain altitude")
    print(f"        (~2 km/month typical decay rate)")

    print(f"\n{'='*70}\n")


def example_solar_radiation_pressure():
    """
    Example 3: Solar radiation pressure effect on GEO satellite.

    SRP is more significant for high-altitude orbits and high A/m satellites.
    """
    print("\n" + "=" * 70)
    print("Example 3: Solar Radiation Pressure - GEO Satellite")
    print("=" * 70)

    # Initial state: GEO at 35,786 km altitude
    altitude = 35786e3  # m
    r0_mag = Earth.R + altitude

    # GEO is equatorial, circular
    r0 = np.array([r0_mag, 0, 0])

    v_circ = np.sqrt(Earth.mu / r0_mag)
    v0 = np.array([0, v_circ, 0])

    print(f"\nInitial conditions (GEO):")
    print(f"  Altitude: {altitude/1000:.1f} km")
    print(f"  Orbital radius: {r0_mag/1000:.1f} km")

    keplerian_propagate(r0, v0, Earth.mu, 0, 0)

    # Spacecraft parameters
    # Large communication satellite: A=20 m², m=2000 kg
    area = 20.0  # m²
    mass = 2000.0  # kg
    area_mass_ratio = area / mass  # m²/kg

    # Radiation pressure coefficient (1.0-2.0, typical 1.3 for mixed surfaces)
    C_r = 1.3

    print(f"\nSpacecraft parameters:")
    print(f"  Area: {area:.1f} m²")
    print(f"  Mass: {mass:.1f} kg")
    print(f"  A/m ratio: {area_mass_ratio:.4f} m²/kg")
    print(f"  C_r: {C_r:.1f} (radiation pressure coefficient)")

    # Propagate with SRP for 30 days
    t_final = 30 * 86400  # 30 days
    dt = 3600.0  # 1 hour timestep (GEO moves slowly)

    print(f"\nPropagating with solar radiation pressure for 30 days...")

    # Propagate step by step
    num_steps = int(t_final / dt)
    times = []
    positions = []
    velocities = []

    r_current = r0.copy()
    v_current = v0.copy()
    t_current = 0

    for i in range(num_steps):
        times.append(t_current)
        positions.append(r_current.copy())
        velocities.append(v_current.copy())

        # Propagate one step
        r_current, v_current = propagate_srp_dopri5(
            r0=r_current,
            v0=v_current,
            dt=dt,
            mu=Earth.mu,
            area_mass_ratio=area_mass_ratio,
            C_r=C_r,
            R_earth=Earth.R,
            t0=t_current,
            tol=1e-8,
        )

        t_current += dt

    times = np.array(times)
    positions = np.array(positions)
    velocities = np.array(velocities)

    print(f"\nPropagation complete:")
    print(f"  Total steps: {len(times)}")

    # Analyze orbital perturbations
    r_initial = positions[0]
    r_final = positions[-1]

    alt_initial = np.linalg.norm(r_initial) - Earth.R
    alt_final = np.linalg.norm(r_final) - Earth.R

    delta_r = np.linalg.norm(r_final - r_initial)
    delta_alt = alt_final - alt_initial

    print(f"\nSRP perturbation effects over 30 days:")
    print(f"  Δ|r| = {delta_r/1000:.3f} km")
    print(f"  Altitude change = {delta_alt/1000:.3f} km")

    # Calculate eccentricity variation
    h_initial = np.cross(r_initial, velocities[0])
    h_final = np.cross(r_final, velocities[-1])

    e_initial_vec = np.cross(velocities[0], h_initial) / Earth.mu - r_initial / np.linalg.norm(
        r_initial
    )
    e_final_vec = np.cross(velocities[-1], h_final) / Earth.mu - r_final / np.linalg.norm(r_final)

    e_initial = np.linalg.norm(e_initial_vec)
    e_final = np.linalg.norm(e_final_vec)

    print(f"  Initial eccentricity: {e_initial:.8f}")
    print(f"  Final eccentricity: {e_final:.8f}")
    print(f"  Δe = {e_final - e_initial:.8f}")

    print(f"\n  Note: SRP causes long-term eccentricity oscillations in GEO")
    print(f"        Station-keeping maneuvers needed ~every few weeks")

    print(f"\n{'='*70}\n")


def example_third_body_moon():
    """
    Example 4: Third-body perturbation from the Moon.

    Shows how Moon's gravity affects high-altitude Earth satellites.
    """
    print("\n" + "=" * 70)
    print("Example 4: Third-Body Perturbation - Lunar Gravity on HEO")
    print("=" * 70)

    # Highly Elliptical Orbit (HEO) - Molniya-type
    # Perigee: 500 km, Apogee: 40,000 km
    perigee_alt = 500e3  # m
    apogee_alt = 40000e3  # m

    r_perigee = Earth.R + perigee_alt
    r_apogee = Earth.R + apogee_alt

    # Semi-major axis
    a = (r_perigee + r_apogee) / 2

    # Eccentricity
    e = (r_apogee - r_perigee) / (r_apogee + r_perigee)

    # Velocity at perigee
    v_perigee = np.sqrt(Earth.mu * (2 / r_perigee - 1 / a))

    # Initial state at perigee
    r0 = np.array([r_perigee, 0, 0])
    v0 = np.array([0, v_perigee, 0])

    print(f"\nInitial conditions (Molniya-type HEO):")
    print(f"  Perigee altitude: {perigee_alt/1000:.1f} km")
    print(f"  Apogee altitude: {apogee_alt/1000:.1f} km")
    print(f"  Semi-major axis: {a/1000:.1f} km")
    print(f"  Eccentricity: {e:.4f}")

    period = 2 * np.pi * np.sqrt(a**3 / Earth.mu)
    print(f"  Orbital period: {period/3600:.2f} hours")

    # Moon position (simplified - assume circular orbit at 384,400 km)
    r_moon = 384400e3  # m
    moon_period = 27.32 * 86400  # days to seconds

    # Moon's gravitational parameter
    mu_moon = 4.9028e12  # m³/s²

    print(f"\nMoon parameters:")
    print(f"  Distance from Earth: {r_moon/1000:.0f} km")
    print(f"  μ_moon = {mu_moon:.4e} m³/s²")

    # Calculate third-body perturbation acceleration at apogee
    # (where Moon's effect is strongest)
    r_sat_moon = r_moon - r_apogee  # Approximate worst case

    # Perturbation magnitude: a ≈ μ_moon / r_sat_moon²
    a_third_body = mu_moon / r_sat_moon**2

    print(f"\nThird-body acceleration at apogee:")
    print(f"  Satellite-Moon distance: ~{r_sat_moon/1000:.0f} km")
    print(f"  |a_third| ≈ {a_third_body:.4e} m/s²")
    print(f"  Ratio to Earth gravity: {a_third_body / (Earth.mu/r_apogee**2):.4f}")

    print(f"\n  Note: At HEO apogee, Moon can cause ~0.05% perturbation")
    print(f"        Significant for long-term orbit evolution")

    print(f"\n{'='*70}\n")


def comparison_summary():
    """
    Summary: Compare perturbation magnitudes at different altitudes.
    """
    print("\n" + "=" * 70)
    print("PERTURBATION MAGNITUDE COMPARISON")
    print("=" * 70)

    print(
        f"""
Typical perturbation accelerations (order of magnitude):

Altitude    | Two-body  | J2        | Drag      | Moon/Sun  | SRP
            | (m/s²)    | (m/s²)    | (m/s²)    | (m/s²)    | (m/s²)
------------|-----------|-----------|-----------|-----------|----------
200 km LEO  | 8.8       | 1e-2      | 1e-4      | 1e-6      | 1e-7
400 km LEO  | 8.5       | 9e-3      | 1e-6      | 1e-6      | 1e-7
800 km LEO  | 7.9       | 7e-3      | 1e-8      | 2e-6      | 1e-6
GEO 35786km | 0.22      | 2e-6      | 0         | 2e-5      | 5e-6

Relative importance by orbit type:
-----------------------------------
LEO (< 2000 km):     J2 >> Drag > Moon/Sun ≈ SRP
MEO (2000-35000 km): J2 > Moon/Sun > SRP > Drag
GEO (35786 km):      Moon/Sun > SRP > J2 >> Drag

Station-keeping requirements:
------------------------------
- LEO: Periodic reboosts for drag compensation (weeks to months)
- MEO: Rare corrections for third-body effects (months to years)
- GEO: Regular E-W and N-S maneuvers for SRP and Moon/Sun (weeks)

Key takeaways:
--------------
1. J2 dominates LEO/MEO perturbations → use for sun-synchronous design
2. Drag critical below ~600 km → limits satellite lifetime
3. Third-body matters for HEO/GEO → long-term stability concerns
4. SRP important for large A/m spacecraft → affects GEO station-keeping
    """
    )

    print("=" * 70)


if __name__ == "__main__":
    print(
        """
╔══════════════════════════════════════════════════════════════════╗
║     Orbit Propagation with Perturbations                        ║
║     High-Fidelity Satellite Dynamics                            ║
╚══════════════════════════════════════════════════════════════════╝

This example demonstrates real-world orbital perturbations and their
effects on satellite trajectories across different orbital regimes.
"""
    )

    # Run examples
    example_j2_leo_precession()
    example_atmospheric_drag()
    example_solar_radiation_pressure()
    example_third_body_moon()
    comparison_summary()

    print(
        """
╔══════════════════════════════════════════════════════════════════╗
║  Next Steps                                                      ║
╚══════════════════════════════════════════════════════════════════╝

1. Combine multiple perturbations for high-fidelity propagation
2. Use adaptive integrators (DOP853) for long-term propagation
3. Implement station-keeping strategies for specific missions
4. Add state transition matrix (STM) for uncertainty propagation
5. Compare with GMAT or STK for validation

For related examples, see:
  - examples/hohmann_transfer.py - Basic orbital maneuvers
  - examples/earth_mars_transfer.py - Interplanetary transfers
  - examples/porkchop_plot.py - Launch window optimization

References:
-----------
- Vallado, "Fundamentals of Astrodynamics and Applications"
- Montenbruck & Gill, "Satellite Orbits"
- Tapley, Schutz, Born, "Statistical Orbit Determination"
"""
    )
