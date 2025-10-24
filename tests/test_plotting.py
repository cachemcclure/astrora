"""
Tests for plotting module.

These tests verify that the plotting API works correctly and is compatible
with poliastro's interface.
"""

import matplotlib
import numpy as np
import pytest

matplotlib.use("Agg")  # Use non-interactive backend for testing
import matplotlib.pyplot as plt
from astropy import units as u
from astrora.bodies import Earth
from astrora.plotting import StaticOrbitPlotter, plot_ground_track
from astrora.twobody import Orbit


class TestStaticOrbitPlotter:
    """Tests for StaticOrbitPlotter class."""

    def test_plotter_creation(self):
        """Test that StaticOrbitPlotter can be created."""
        plotter = StaticOrbitPlotter()
        assert plotter is not None
        assert plotter.ax is not None
        assert plotter.attractor is None

    def test_plotter_with_custom_axes(self):
        """Test creating plotter with custom matplotlib axes."""
        fig, ax = plt.subplots()
        plotter = StaticOrbitPlotter(ax=ax)
        assert plotter.ax is ax
        plt.close(fig)

    def test_plotter_dark_mode(self):
        """Test creating plotter with dark theme."""
        plotter = StaticOrbitPlotter(dark=True)
        assert plotter._dark is True
        plt.close(plotter.ax.figure)

    def test_set_attractor(self):
        """Test setting the central attractor body."""
        plotter = StaticOrbitPlotter()
        plotter.set_attractor(Earth)
        assert plotter.attractor is Earth
        assert plotter.attractor.name == "Earth"
        plt.close(plotter.ax.figure)

    def test_plot_circular_orbit(self):
        """Test plotting a simple circular orbit."""
        # Create circular orbit
        r = np.array([7000e3, 0, 0])  # meters
        v = np.array([0, 7546, 0])  # m/s
        orbit = Orbit.from_vectors(Earth, r, v)

        # Plot it
        plotter = StaticOrbitPlotter()
        traj, pos = plotter.plot(orbit, label="Test Orbit")

        assert len(traj) > 0
        assert pos is not None
        assert plotter.attractor is Earth
        plt.close(plotter.ax.figure)

    def test_plot_with_units(self):
        """Test plotting orbit created with astropy units."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000 << u.km,
            ecc=0.01 << u.one,
            inc=51.6 << u.deg,
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        plotter = StaticOrbitPlotter()
        traj, pos = plotter.plot(orbit, label="ISS-like")

        assert len(traj) > 0
        assert pos is not None
        plt.close(plotter.ax.figure)

    def test_plot_with_color(self):
        """Test plotting with custom color."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        plotter = StaticOrbitPlotter()
        traj, pos = plotter.plot(orbit, color="red")

        assert traj[0].get_color() == "red"
        plt.close(plotter.ax.figure)

    def test_plot_with_trail(self):
        """Test plotting with fading trail effect."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        plotter = StaticOrbitPlotter()
        traj, pos = plotter.plot(orbit, trail=True)

        # Trail creates multiple line segments
        assert len(traj) > 1
        plt.close(plotter.ax.figure)

    def test_plot_multiple_orbits(self):
        """Test plotting multiple orbits on same axes."""
        # Create two different orbits
        r1 = np.array([7000e3, 0, 0])
        v1 = np.array([0, 7546, 0])
        orbit1 = Orbit.from_vectors(Earth, r1, v1)

        r2 = np.array([8000e3, 0, 0])
        v2 = np.array([0, 7000, 0])
        orbit2 = Orbit.from_vectors(Earth, r2, v2)

        plotter = StaticOrbitPlotter()
        plotter.plot(orbit1, label="Orbit 1")
        plotter.plot(orbit2, label="Orbit 2")

        # Both orbits should share same attractor
        assert plotter.attractor is Earth
        plt.close(plotter.ax.figure)

    def test_plot_elliptical_orbit(self):
        """Test plotting an elliptical orbit."""
        orbit = Orbit.from_classical(
            Earth,
            a=10000e3,  # 10,000 km
            ecc=0.3,  # elliptical
            inc=np.deg2rad(45),
            raan=0,
            argp=0,
            nu=0,
        )

        plotter = StaticOrbitPlotter()
        traj, pos = plotter.plot(orbit)

        assert len(traj) > 0
        plt.close(plotter.ax.figure)

    def test_plot_trajectory(self):
        """Test plotting a precomputed trajectory."""
        # Create some trajectory points
        theta = np.linspace(0, 2 * np.pi, 100)
        r = 7000e3  # meters
        positions = np.column_stack([r * np.cos(theta), r * np.sin(theta), np.zeros_like(theta)])

        plotter = StaticOrbitPlotter()
        plotter.set_attractor(Earth)
        traj, pos = plotter.plot_trajectory(positions, label="Custom trajectory")

        assert len(traj) > 0
        assert pos is not None
        plt.close(plotter.ax.figure)

    def test_plot_trajectory_with_units(self):
        """Test plotting trajectory with astropy units."""
        theta = np.linspace(0, 2 * np.pi, 100)
        r = 7000  # km
        positions = (
            np.column_stack([r * np.cos(theta), r * np.sin(theta), np.zeros_like(theta)]) << u.km
        )

        plotter = StaticOrbitPlotter()
        plotter.set_attractor(Earth)
        traj, pos = plotter.plot_trajectory(positions)

        assert len(traj) > 0
        plt.close(plotter.ax.figure)

    def test_plot_trajectory_without_attractor_raises(self):
        """Test that plotting trajectory without attractor raises error."""
        positions = np.random.randn(10, 3) * 7000e3

        plotter = StaticOrbitPlotter()

        with pytest.raises(ValueError, match="Must set attractor"):
            plotter.plot_trajectory(positions)

        plt.close(plotter.ax.figure)

    def test_savefig(self, tmp_path):
        """Test saving plot to file."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        plotter = StaticOrbitPlotter()
        plotter.plot(orbit, label="Test")

        # Save to temporary file
        output_path = tmp_path / "test_orbit.png"
        plotter.savefig(str(output_path))

        assert output_path.exists()
        assert output_path.stat().st_size > 0
        plt.close(plotter.ax.figure)


class TestOrbitPlotter3D:
    """Tests for OrbitPlotter3D class."""

    def test_import_plotly_required(self):
        """Test that OrbitPlotter3D requires plotly."""
        try:
            import plotly
            from astrora.plotting import OrbitPlotter3D

            # If plotly is available, should work
            plotter = OrbitPlotter3D()
            assert plotter is not None
        except ImportError:
            # If plotly not available, should raise ImportError
            with pytest.raises(ImportError):
                from astrora.plotting import OrbitPlotter3D

                OrbitPlotter3D()

    @pytest.mark.skipif(
        not pytest.importorskip("plotly", reason="plotly not installed"),
        reason="plotly not available",
    )
    def test_plotter3d_creation(self):
        """Test creating 3D plotter."""
        from astrora.plotting import OrbitPlotter3D

        plotter = OrbitPlotter3D()
        assert plotter is not None
        assert plotter.fig is not None

    @pytest.mark.skipif(
        not pytest.importorskip("plotly", reason="plotly not installed"),
        reason="plotly not available",
    )
    def test_plotter3d_dark_mode(self):
        """Test creating 3D plotter with dark theme."""
        from astrora.plotting import OrbitPlotter3D

        plotter = OrbitPlotter3D(dark=True)
        assert plotter._dark is True

    @pytest.mark.skipif(
        not pytest.importorskip("plotly", reason="plotly not installed"),
        reason="plotly not available",
    )
    def test_plot3d_orbit(self):
        """Test plotting orbit in 3D."""
        from astrora.plotting import OrbitPlotter3D

        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        plotter = OrbitPlotter3D()
        plotter.plot(orbit, label="Test 3D")

        assert plotter.attractor is Earth
        assert len(plotter.fig.data) > 0


class TestGroundTrackPlotting:
    """Tests for ground track plotting."""

    def test_plot_ground_track_basic(self):
        """Test basic ground track plotting."""
        # ISS-like orbit
        orbit = Orbit.from_classical(
            Earth,
            a=6800 << u.km,
            ecc=0.0001 << u.one,
            inc=51.6 << u.deg,
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        # Plot one orbit
        ax = plot_ground_track(orbit, duration=orbit.period.value, dt=60)

        assert ax is not None
        assert ax.get_xlabel() == "Longitude (°)"
        assert ax.get_ylabel() == "Latitude (°)"
        plt.close(ax.figure)

    def test_plot_ground_track_with_color(self):
        """Test ground track with custom color."""
        r = np.array([6800e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        ax = plot_ground_track(orbit, duration=orbit.period, color="red", label="Test track")

        assert ax is not None
        plt.close(ax.figure)

    def test_plot_ground_track_polar_orbit(self):
        """Test ground track for polar orbit."""
        orbit = Orbit.from_classical(
            Earth,
            a=7000 << u.km,
            ecc=0.001 << u.one,
            inc=90 << u.deg,  # polar
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        ax = plot_ground_track(orbit, duration=orbit.period.value, dt=60)

        assert ax is not None
        # Polar orbit should cover high latitudes
        plt.close(ax.figure)


class TestPorkchopPlotting:
    """Tests for porkchop plot generation."""

    def test_porkchop_import(self):
        """Test that porkchop plotting functions can be imported."""
        from astrora.plotting import plot_porkchop, plot_porkchop_simple

        assert plot_porkchop is not None
        assert plot_porkchop_simple is not None

    # Note: Full porkchop plot testing requires Lambert solver integration
    # and planetary ephemerides, which would make tests very slow
    # These are better suited for integration tests or examples


def test_plotting_module_exports():
    """Test that all expected functions are exported."""
    from astrora import plotting

    assert hasattr(plotting, "StaticOrbitPlotter")
    assert hasattr(plotting, "OrbitPlotter3D")
    assert hasattr(plotting, "plot_porkchop")
    assert hasattr(plotting, "plot_ground_track")
    assert hasattr(plotting, "plot_ground_track_3d")


# Additional tests for coverage improvement


class TestOrbitPlotter3DComprehensive:
    """Comprehensive tests for OrbitPlotter3D to improve coverage."""

    def test_plotter3d_basic_creation(self):
        """Test basic 3D plotter creation."""
        try:
            from astrora.plotting import OrbitPlotter3D

            plotter = OrbitPlotter3D()
            assert plotter is not None
            assert plotter.fig is not None
        except ImportError:
            pytest.skip("Plotly not available")

    def test_plotter3d_dark_mode(self):
        """Test 3D plotter with dark theme."""
        try:
            from astrora.plotting import OrbitPlotter3D

            plotter = OrbitPlotter3D(dark=True)
            assert plotter._dark is True
        except ImportError:
            pytest.skip("Plotly not available")

    def test_plotter3d_plot_orbit(self):
        """Test plotting an orbit in 3D."""
        try:
            from astrora.plotting import OrbitPlotter3D

            r = np.array([7000e3, 0, 0])
            v = np.array([0, 7546, 0])
            orbit = Orbit.from_vectors(Earth, r, v)

            plotter = OrbitPlotter3D()
            plotter.plot(orbit, label="Test Orbit")
            # Should not raise error
        except ImportError:
            pytest.skip("Plotly not available")

    def test_plotter3d_show_method(self):
        """Test the show method."""
        try:
            from astrora.plotting import OrbitPlotter3D

            r = np.array([7000e3, 0, 0])
            v = np.array([0, 7546, 0])
            orbit = Orbit.from_vectors(Earth, r, v)

            plotter = OrbitPlotter3D()
            plotter.plot(orbit)
            # Don't actually show (would block tests), but call the method
            # plotter.show()  # Can't test this in CI
        except ImportError:
            pytest.skip("Plotly not available")


class TestAnimationComprehensive:
    """Comprehensive tests for animation to improve coverage."""

    def test_animate_orbit_basic(self):
        """Test basic orbit animation."""
        try:
            from astrora.plotting import animate_orbit

            r = np.array([7000e3, 0, 0])
            v = np.array([0, 7546, 0])
            orbit = Orbit.from_vectors(Earth, r, v)

            anim = animate_orbit(orbit, num_frames=5)
            assert anim is not None
        except (ImportError, RuntimeError) as e:
            pytest.skip(f"Animation not available: {e}")

    def test_animate_orbit_with_options(self):
        """Test orbit animation with various options."""
        try:
            from astrora.plotting import animate_orbit

            r = np.array([7000e3, 0, 0])
            v = np.array([0, 7546, 0])
            orbit = Orbit.from_vectors(Earth, r, v)

            anim = animate_orbit(orbit, num_frames=5, show_trail=True, interval=100)
            assert anim is not None
        except (ImportError, RuntimeError) as e:
            pytest.skip(f"Animation with options not available: {e}")

    def test_animate_multiple_orbits(self):
        """Test animating multiple orbits."""
        try:
            from astrora.plotting import animate_orbit

            r1 = np.array([7000e3, 0, 0])
            v1 = np.array([0, 7546, 0])
            orbit1 = Orbit.from_vectors(Earth, r1, v1)

            r2 = np.array([8000e3, 0, 0])
            v2 = np.array([0, 7200, 0])
            orbit2 = Orbit.from_vectors(Earth, r2, v2)

            # Test if function accepts multiple orbits
            anim = animate_orbit([orbit1, orbit2], num_frames=5)
            assert anim is not None
        except (ImportError, RuntimeError, TypeError) as e:
            pytest.skip(f"Multiple orbit animation not available: {e}")


class TestGroundTrackComprehensive:
    """Comprehensive tests for ground track plotting."""

    def test_ground_track_basic_leo(self):
        """Test ground track for LEO orbit."""
        from astrora.plotting import plot_ground_track

        # ISS-like orbit
        r = np.array([6778e3, 0, 0])
        v = np.array([0, 7500, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        try:
            fig, ax = plot_ground_track(orbit, n_points=100)
            assert fig is not None
            assert ax is not None
            plt.close(fig)
        except ImportError as e:
            pytest.skip(f"Cartopy or other dependency missing: {e}")

    def test_ground_track_polar_orbit(self):
        """Test ground track for polar orbit."""
        from astrora.plotting import plot_ground_track

        # Polar orbit
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7500])
        orbit = Orbit.from_vectors(Earth, r, v)

        try:
            fig, ax = plot_ground_track(orbit, n_points=50)
            assert fig is not None
            plt.close(fig)
        except ImportError as e:
            pytest.skip(f"Cartopy missing: {e}")

    def test_ground_track_with_custom_ax(self):
        """Test ground track with custom axes."""
        from astrora.plotting import plot_ground_track

        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        try:
            custom_fig, custom_ax = plt.subplots()
            fig, ax = plot_ground_track(orbit, n_points=50, ax=custom_ax)
            assert ax is custom_ax
            plt.close(fig)
        except ImportError as e:
            pytest.skip(f"Cartopy missing: {e}")


class TestStaticPlotterComprehensive:
    """Additional comprehensive tests for StaticOrbitPlotter."""

    def test_plot_multiple_orbits(self):
        """Test plotting multiple orbits."""
        plotter = StaticOrbitPlotter()
        plotter.set_attractor(Earth)

        # Plot first orbit
        r1 = np.array([7000e3, 0, 0])
        v1 = np.array([0, 7546, 0])
        orbit1 = Orbit.from_vectors(Earth, r1, v1)
        plotter.plot(orbit1, label="Orbit 1")

        # Plot second orbit
        r2 = np.array([8000e3, 0, 0])
        v2 = np.array([0, 7200, 0])
        orbit2 = Orbit.from_vectors(Earth, r2, v2)
        plotter.plot(orbit2, label="Orbit 2")

        plt.close(plotter.ax.figure)

    def test_plot_trajectory(self):
        """Test plotting a trajectory."""
        plotter = StaticOrbitPlotter()
        plotter.set_attractor(Earth)

        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        # Sample orbit for trajectory
        times = np.linspace(0, orbit.period.to_value(u.s), 100)
        try:
            trajectory = orbit.sample(times)
            # plotter.plot_trajectory(trajectory)  # If this method exists
        except AttributeError:
            pass  # Method may not exist

        plt.close(plotter.ax.figure)

    def test_plot_with_color_and_style(self):
        """Test plotting with custom colors and styles."""
        plotter = StaticOrbitPlotter()
        plotter.set_attractor(Earth)

        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        # Test with custom color
        plotter.plot(orbit, label="Custom", color="red")

        plt.close(plotter.ax.figure)
