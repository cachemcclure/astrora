"""
Complete Orbit Plotting Examples
==================================

This example demonstrates all plotting capabilities in astrora, showing how to:
1. Create and visualize different orbit types (circular, elliptical, polar, GTO)
2. Use both 2D (matplotlib) and 3D (Plotly) plotters
3. Customize colors, labels, and visual styles
4. Save plots to files
5. Plot ground tracks for satellite orbits

This is a comprehensive demonstration of the plotting module functionality.
"""

import matplotlib.pyplot as plt
import numpy as np
from astropy import units as u
from astrora.bodies import Earth
from astrora.plotting import StaticOrbitPlotter, plot_ground_track
from astrora.twobody import Orbit

print("=" * 70)
print(" Astrora Complete Orbit Plotting Examples")
print("=" * 70)

# ============================================================================
# Example 1: Multiple Orbits in 2D
# ============================================================================

print("\n[1] Creating multiple orbits on same plot...")

# Create ISS-like orbit (circular, LEO)
iss_orbit = Orbit.from_classical(
    Earth,
    a=6800 << u.km,
    ecc=0.0001 << u.one,
    inc=51.6 << u.deg,
    raan=0 << u.deg,
    argp=0 << u.deg,
    nu=0 << u.deg,
)

# Create MEO orbit (GPS-like, medium eccentricity)
meo_orbit = Orbit.from_classical(
    Earth,
    a=26560 << u.km,  # GPS orbital radius
    ecc=0.01 << u.one,  # Slightly elliptical
    inc=55 << u.deg,  # GPS inclination
    raan=0 << u.deg,
    argp=0 << u.deg,
    nu=0 << u.deg,
)

# Create GEO orbit (circular, equatorial)
geo_orbit = Orbit.from_classical(
    Earth,
    a=42164 << u.km,
    ecc=0.0001 << u.one,
    inc=0 << u.deg,
    raan=0 << u.deg,
    argp=0 << u.deg,
    nu=0 << u.deg,
)

# Plot all three orbits
plotter = StaticOrbitPlotter()
plotter.plot(iss_orbit, label="ISS (LEO)", color="blue")
plotter.plot(meo_orbit, label="GPS (MEO)", color="red")
plotter.plot(geo_orbit, label="GEO", color="green")

plt.title("Comparison of Different Orbit Types", fontsize=14, fontweight="bold")
plotter.savefig("multiple_orbits_2d.png", dpi=150)
print("   ✓ Saved: multiple_orbits_2d.png")
plt.close()

# ============================================================================
# Example 2: Elliptical Orbit with Trail Effect
# ============================================================================

print("\n[2] Creating elliptical orbit with fading trail...")

# Create GTO (Geostationary Transfer Orbit)
# Using more moderate eccentricity for numerical stability
gto_orbit = Orbit.from_classical(
    Earth,
    a=24500 << u.km,
    ecc=0.45 << u.one,  # Elliptical but numerically stable
    inc=7 << u.deg,
    raan=0 << u.deg,
    argp=178 << u.deg,
    nu=0 << u.deg,
)

plotter = StaticOrbitPlotter()
plotter.plot(gto_orbit, label="GTO (Geostationary Transfer)", color="orange", trail=True)

plt.title("Geostationary Transfer Orbit with Trail Effect", fontsize=14)
plotter.savefig("gto_orbit_trail.png", dpi=150)
print("   ✓ Saved: gto_orbit_trail.png")
plt.close()

# ============================================================================
# Example 3: Polar Orbit (Sun-Synchronous)
# ============================================================================

print("\n[3] Creating near-polar orbit...")

# Polar orbit (typical for Earth observation satellites)
polar_orbit = Orbit.from_classical(
    Earth,
    a=7078 << u.km,  # ~700 km altitude
    ecc=0.001 << u.one,
    inc=89.9 << u.deg,  # Near-polar inclination
    raan=0 << u.deg,
    argp=0 << u.deg,
    nu=0 << u.deg,
)

plotter = StaticOrbitPlotter()
plotter.plot(polar_orbit, label="Polar Orbit (700 km)", color="purple")

plt.title("Near-Polar Orbit", fontsize=14)
plotter.savefig("sun_synchronous_orbit.png", dpi=150)
print("   ✓ Saved: sun_synchronous_orbit.png")
plt.close()

# ============================================================================
# Example 4: Dark Mode Plotting
# ============================================================================

print("\n[4] Creating plot with dark theme...")

plotter = StaticOrbitPlotter(dark=True)
plotter.plot(iss_orbit, label="ISS", color="#00D9FF")  # Cyan
plotter.plot(meo_orbit, label="GPS", color="#FF6B9D")  # Pink
plotter.plot(geo_orbit, label="GEO", color="#C3F73A")  # Lime green

plt.title("Dark Mode Orbit Visualization", fontsize=14, fontweight="bold", color="white")
plotter.savefig("orbits_dark_mode.png", dpi=150, facecolor="#1a1a1a")
print("   ✓ Saved: orbits_dark_mode.png")
plt.close()

# ============================================================================
# Example 5: Orbit Created from State Vectors
# ============================================================================

print("\n[5] Creating orbit from state vectors (raw arrays)...")

# ISS state vectors (approximate)
r = np.array([6800e3, 0, 0])  # meters
v = np.array([0, 7.546e3, 0])  # m/s
orbit_from_vectors = Orbit.from_vectors(Earth, r, v)

print(f"   Orbit from vectors:")
print(f"   - Semi-major axis: {orbit_from_vectors.a.to(u.km):.2f}")
print(f"   - Eccentricity: {orbit_from_vectors.ecc:.6f}")
print(f"   - Period: {orbit_from_vectors.period.to(u.hour):.2f}")

plotter = StaticOrbitPlotter()
plotter.plot(orbit_from_vectors, label="ISS (from state vectors)", color="blue")

plt.title("Orbit Created from State Vectors", fontsize=14)
plotter.savefig("orbit_from_vectors.png", dpi=150)
print("   ✓ Saved: orbit_from_vectors.png")
plt.close()

# ============================================================================
# Example 6: Ground Track Visualization
# ============================================================================

print("\n[6] Creating ground track visualization...")

# ISS ground track for one complete orbit
fig, ax = plt.subplots(figsize=(14, 8))
plot_ground_track(
    iss_orbit,
    duration=iss_orbit.period.value,
    dt=30,  # Sample every 30 seconds
    ax=ax,
    color="blue",
    label="ISS Ground Track",
)

plt.title("ISS Ground Track (One Orbit)", fontsize=14, fontweight="bold")
plt.savefig("iss_ground_track.png", dpi=150, bbox_inches="tight")
print("   ✓ Saved: iss_ground_track.png")
plt.close()

# ============================================================================
# Example 7: Polar Orbit Ground Track
# ============================================================================

print("\n[7] Creating polar orbit ground track...")

fig, ax = plt.subplots(figsize=(14, 8))
plot_ground_track(
    polar_orbit,
    duration=polar_orbit.period.value,
    dt=60,  # Sample every minute
    ax=ax,
    color="purple",
    label="Polar Orbit Track",
)

plt.title("Polar Orbit Ground Track", fontsize=14, fontweight="bold")
plt.savefig("polar_ground_track.png", dpi=150, bbox_inches="tight")
print("   ✓ Saved: polar_ground_track.png")
plt.close()

# ============================================================================
# Example 8: Multiple Ground Tracks
# ============================================================================

print("\n[8] Comparing ground tracks of different orbits...")

fig, ax = plt.subplots(figsize=(14, 8))

# ISS track
plot_ground_track(
    iss_orbit,
    duration=iss_orbit.period.value,
    dt=60,
    ax=ax,
    color="blue",
    label="ISS (51.6° inclination)",
)

# Polar orbit track
plot_ground_track(
    polar_orbit,
    duration=polar_orbit.period.value,
    dt=60,
    ax=ax,
    color="purple",
    label="Polar (90° inclination)",
)

plt.title("Ground Track Comparison: ISS vs Polar Orbit", fontsize=14, fontweight="bold")
plt.savefig("ground_track_comparison.png", dpi=150, bbox_inches="tight")
print("   ✓ Saved: ground_track_comparison.png")
plt.close()

# ============================================================================
# Example 9: 3D Interactive Plotting (if Plotly installed)
# ============================================================================

print("\n[9] Creating 3D interactive plot (requires plotly)...")

try:
    from astrora.plotting import OrbitPlotter3D

    plotter3d = OrbitPlotter3D()
    plotter3d.plot(iss_orbit, label="ISS", color="blue")
    plotter3d.plot(meo_orbit, label="GPS", color="red")
    plotter3d.plot(geo_orbit, label="GEO", color="green")

    # Save as interactive HTML
    plotter3d.savefig("orbits_3d_interactive.html")
    print("   ✓ Saved: orbits_3d_interactive.html (open in browser)")

except ImportError:
    print("   ⚠ Plotly not installed. Skipping 3D plot.")
    print("   Install with: uv pip install plotly")

# ============================================================================
# Summary
# ============================================================================

print("\n" + "=" * 70)
print("Summary - Files Created:")
print("=" * 70)
print("✓ multiple_orbits_2d.png       - Comparison of ISS, GPS (MEO), and GEO")
print("✓ gto_orbit_trail.png           - GTO with fading trail effect")
print("✓ sun_synchronous_orbit.png     - Polar sun-synchronous orbit")
print("✓ orbits_dark_mode.png          - Dark theme visualization")
print("✓ orbit_from_vectors.png        - Orbit from state vectors")
print("✓ iss_ground_track.png          - ISS ground track (one orbit)")
print("✓ polar_ground_track.png        - Polar orbit ground track")
print("✓ ground_track_comparison.png   - ISS vs Polar comparison")
print("✓ orbits_3d_interactive.html    - 3D interactive plot (if plotly installed)")
print("\nAll plotting examples completed successfully!")
print("=" * 70)
