"""
Plotting Module Demonstration
==============================

This example demonstrates the new plotting capabilities in astrora,
showing how to create publication-quality orbit visualizations.
"""

import numpy as np
from astropy import units as u
from astrora.bodies import Earth
from astrora.plotting import StaticOrbitPlotter
from astrora.twobody import Orbit

print("Astrora Plotting Module Demo")
print("=" * 50)

# Create a simple circular orbit (ISS-like)
print("\n1. Creating ISS-like circular orbit...")
r = np.array([6800e3, 0, 0])  # meters
v = np.array([0, 7.66e3, 0])  # m/s
orbit1 = Orbit.from_vectors(Earth, r, v)

print(f"   Semi-major axis: {orbit1.a.to(u.km):.2f}")
print(f"   Eccentricity: {orbit1.ecc:.6f}")
print(f"   Inclination: {orbit1.inc.to(u.deg):.2f}")

# Create an elliptical orbit (GTO-like)
print("\n2. Creating GTO-like elliptical orbit...")
orbit2 = Orbit.from_classical(
    Earth,
    a=24500 << u.km,
    ecc=0.73 << u.one,
    inc=7 << u.deg,
    raan=0 << u.deg,
    argp=178 << u.deg,
    nu=0 << u.deg,
)

print(f"   Semi-major axis: {orbit2.a.to(u.km):.2f}")
print(f"   Eccentricity: {orbit2.ecc:.6f}")
print(f"   Periapsis: {orbit2.r_p.to(u.km):.2f}")
print(f"   Apoapsis: {orbit2.r_a.to(u.km):.2f}")

# Create a polar orbit
print("\n3. Creating polar orbit...")
orbit3 = Orbit.from_classical(
    Earth,
    a=7000 << u.km,
    ecc=0.001 << u.one,
    inc=98 << u.deg,  # Sun-synchronous inclination
    raan=0 << u.deg,
    argp=90 << u.deg,
    nu=0 << u.deg,
)

print(f"   Semi-major axis: {orbit3.a.to(u.km):.2f}")
print(f"   Inclination: {orbit3.inc.to(u.deg):.2f}")

# Demonstrate plotting with StaticOrbitPlotter
print("\n4. Creating plot with StaticOrbitPlotter...")
plotter = StaticOrbitPlotter()

# Note: Full orbit plotting requires working sample() method
# For now, we demonstrate the plotter creation and attractor display

plotter.set_attractor(Earth)
print("   ✓ Plotter created successfully")
print("   ✓ Earth set as attractor")

# Create a simple trajectory manually to demonstrate plotting
print("\n5. Creating manual trajectory for demonstration...")
theta = np.linspace(0, 2 * np.pi, 100)
r_orbit = 7000e3  # meters
positions = np.column_stack(
    [r_orbit * np.cos(theta), r_orbit * np.sin(theta), np.zeros_like(theta)]
)

traj, pos = plotter.plot_trajectory(positions, label="Circular test orbit", color="blue")
print("   ✓ Trajectory plotted successfully")

# Add another trajectory at different altitude
r_orbit2 = 10000e3
positions2 = np.column_stack(
    [r_orbit2 * np.cos(theta) * 0.7, r_orbit2 * np.sin(theta), np.zeros_like(theta)]  # Elliptical
)

traj2, pos2 = plotter.plot_trajectory(positions2, label="Elliptical test orbit", color="red")
print("   ✓ Second trajectory plotted successfully")

# Save the plot
print("\n6. Saving plot to file...")
plotter.savefig("orbit_demo.png", dpi=150)
print("   ✓ Plot saved as 'orbit_demo.png'")

print("\n" + "=" * 50)
print("Demo completed successfully!")
print("\nThe plotting module provides:")
print("  • StaticOrbitPlotter - 2D matplotlib plotting")
print("  • OrbitPlotter3D - 3D interactive Plotly plotting")
print("  • plot_ground_track - Ground track visualization")
print("  • plot_porkchop - Launch window analysis")
print("\nAll poliastro-compatible API maintained!")
