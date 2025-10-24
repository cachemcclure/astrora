"""
Demonstration of orbit animation capabilities.

This example showcases both 2D (matplotlib) and 3D (plotly) animations
for visualizing orbital motion over time.
"""

import matplotlib.pyplot as plt
import numpy as np
from astropy import units as u
from astrora.bodies import Earth
from astrora.plotting import animate_orbit, animate_orbit_3d
from astrora.twobody import Orbit


def demo_2d_animation_single_orbit():
    """Demonstrate 2D animation of a single orbit."""
    print("Creating 2D animation of single ISS-like orbit...")

    # Create an ISS-like orbit
    r = np.array([6800e3, 0, 0])  # meters
    v = np.array([0, 7546, 0])  # m/s
    orbit = Orbit.from_vectors(Earth, r, v)

    # Create animation
    anim = animate_orbit(orbit, num_frames=100, fps=30, trail=True, show_time=True, dark=False)

    print(f"Orbit period: {orbit.period.to(u.hour):.2f}")
    print("Animation created! Close the window to continue...")
    plt.show()


def demo_2d_animation_multiple_orbits():
    """Demonstrate 2D animation of multiple orbits."""
    print("\nCreating 2D animation with multiple orbits...")

    # Create two different orbits
    orbit1 = Orbit.from_classical(
        Earth,
        a=7000 << u.km,
        ecc=0.01 << u.one,
        inc=0 << u.deg,
        raan=0 << u.deg,
        argp=0 << u.deg,
        nu=0 << u.deg,
    )

    orbit2 = Orbit.from_classical(
        Earth,
        a=8500 << u.km,
        ecc=0.15 << u.one,
        inc=0 << u.deg,
        raan=0 << u.deg,
        argp=0 << u.deg,
        nu=0 << u.deg,
    )

    # Animate both orbits for 2 periods of the first orbit
    duration = 2 * orbit1.period.value

    anim = animate_orbit(
        [orbit1, orbit2], duration=duration, num_frames=150, fps=30, trail=True, dark=False
    )

    print("Multiple orbits animation created! Close the window to continue...")
    plt.show()


def demo_2d_animation_dark_mode():
    """Demonstrate 2D animation with dark theme."""
    print("\nCreating 2D animation with dark theme...")

    # Create an elliptical orbit
    orbit = Orbit.from_classical(
        Earth,
        a=10000 << u.km,
        ecc=0.3 << u.one,
        inc=0 << u.deg,
        raan=0 << u.deg,
        argp=0 << u.deg,
        nu=0 << u.deg,
    )

    anim = animate_orbit(orbit, num_frames=100, fps=25, dark=True, trail=True)

    print("Dark mode animation created! Close the window to continue...")
    plt.show()


def demo_3d_animation_single_orbit():
    """Demonstrate interactive 3D animation."""
    print("\nCreating 3D interactive animation...")

    # Create an inclined orbit
    r = np.array([7000e3, 0, 0])
    v = np.array([0, 5000, 5000])  # Inclined velocity
    orbit = Orbit.from_vectors(Earth, r, v)

    # Create 3D animation
    fig = animate_orbit_3d(orbit, num_frames=100, fps=30, trail=True, dark=False, show_time=True)

    print("3D animation created! Interact with it in your browser...")
    print("Use mouse to rotate, zoom, and pan. Use play/pause controls.")
    fig.show()


def demo_3d_animation_multiple_orbits():
    """Demonstrate 3D animation with multiple orbits."""
    print("\nCreating 3D animation with multiple orbits...")

    # Create three orbits with different inclinations
    orbit1 = Orbit.from_vectors(
        Earth, r=np.array([7000e3, 0, 0]), v=np.array([0, 7546, 0])  # Equatorial
    )

    orbit2 = Orbit.from_vectors(
        Earth, r=np.array([7000e3, 0, 0]), v=np.array([0, 5000, 5000])  # Inclined
    )

    orbit3 = Orbit.from_vectors(
        Earth, r=np.array([8000e3, 0, 0]), v=np.array([0, 0, 7000])  # Polar-ish
    )

    # Animate all three
    fig = animate_orbit_3d([orbit1, orbit2, orbit3], num_frames=100, fps=30, trail=True)

    print("Multi-orbit 3D animation created!")
    fig.show()


def demo_save_animation():
    """Demonstrate saving animations to files."""
    print("\nDemonstrating animation file saving...")

    # Create a simple orbit
    orbit = Orbit.from_classical(
        Earth,
        a=7500 << u.km,
        ecc=0.05 << u.one,
        inc=0 << u.deg,
        raan=0 << u.deg,
        argp=0 << u.deg,
        nu=0 << u.deg,
    )

    # Save 2D animation as GIF
    print("Saving 2D animation as GIF (this may take a moment)...")
    try:
        anim = animate_orbit(
            orbit,
            num_frames=50,  # Fewer frames for smaller file
            fps=15,
            save_to="orbit_animation_2d.gif",
        )
        print("✓ Saved to: orbit_animation_2d.gif")
    except Exception as e:
        print(f"✗ Could not save GIF: {e}")
        print("  (pillow package may be needed: pip install pillow)")

    # Save 3D animation as HTML
    print("\nSaving 3D animation as HTML...")
    try:
        fig = animate_orbit_3d(orbit, num_frames=50, fps=20, save_to="orbit_animation_3d.html")
        print("✓ Saved to: orbit_animation_3d.html")
        print("  Open this file in a web browser to view the interactive animation.")
    except Exception as e:
        print(f"✗ Could not save HTML: {e}")

    plt.close("all")


def main():
    """Run all animation demonstrations."""
    print("=" * 70)
    print("Astrora Orbit Animation Demo")
    print("=" * 70)
    print("\nThis demo showcases the new animation capabilities!")
    print("These features go beyond what poliastro/hapsira offered.\n")

    # Comment out demonstrations you don't want to run
    demo_2d_animation_single_orbit()
    demo_2d_animation_multiple_orbits()
    demo_2d_animation_dark_mode()
    demo_3d_animation_single_orbit()
    demo_3d_animation_multiple_orbits()
    demo_save_animation()

    print("\n" + "=" * 70)
    print("Demo complete!")
    print("=" * 70)
    print("\nKey features demonstrated:")
    print("  ✓ 2D matplotlib animations with FuncAnimation")
    print("  ✓ 3D interactive plotly animations")
    print("  ✓ Multiple orbits animated simultaneously")
    print("  ✓ Dark mode support")
    print("  ✓ Customizable frame rates and durations")
    print("  ✓ Trail visualization options")
    print("  ✓ Time display and interactive controls (3D)")
    print("  ✓ Export to GIF, HTML, and other formats")


if __name__ == "__main__":
    main()
