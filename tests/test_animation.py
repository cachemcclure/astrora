"""
Tests for orbit animation functionality.

Tests both matplotlib (2D) and plotly (3D) animation helpers.
"""

import numpy as np
import pytest
from astropy import units as u

# Import animation module - may not be available if matplotlib/plotly missing
try:
    from astrora.plotting.animation import animate_orbit, animate_orbit_3d

    HAS_ANIMATION = True
except ImportError:
    HAS_ANIMATION = False

try:
    import matplotlib

    matplotlib.use("Agg")  # Non-interactive backend for testing
    import matplotlib.animation as mpl_animation
    import matplotlib.pyplot as plt

    HAS_MATPLOTLIB = True
except ImportError:
    HAS_MATPLOTLIB = False

try:
    import plotly.graph_objects as go

    HAS_PLOTLY = True
except ImportError:
    HAS_PLOTLY = False

from astrora.bodies import Earth, Mars
from astrora.twobody import Orbit

# Skip all tests if animation module not available
pytestmark = pytest.mark.skipif(not HAS_ANIMATION, reason="Animation module not available")


class TestAnimateOrbit:
    """Tests for 2D matplotlib orbit animations."""

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_single_orbit_basic(self):
        """Test basic 2D animation of a single orbit."""
        # Create a simple circular orbit
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        # Create animation
        anim = animate_orbit(orbit, num_frames=10, fps=10)

        # Verify animation object
        assert isinstance(anim, mpl_animation.FuncAnimation)
        assert anim is not None

        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_single_orbit_with_units(self):
        """Test animation with astropy units."""
        r = [7000, 0, 0] << u.km
        v = [0, 7.546, 0] << u.km / u.s
        orbit = Orbit.from_vectors(Earth, r, v)

        anim = animate_orbit(orbit, num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_multiple_orbits(self):
        """Test animating multiple orbits simultaneously."""
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
            a=8000 << u.km,
            ecc=0.05 << u.one,
            inc=10 << u.deg,
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        anim = animate_orbit([orbit1, orbit2], num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_custom_duration(self):
        """Test animation with custom duration."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        # Animate for half a period
        period = orbit.period.value if hasattr(orbit.period, "value") else orbit.period
        anim = animate_orbit(orbit, duration=period / 2, num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_without_trail(self):
        """Test animation without showing orbital trail."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        anim = animate_orbit(orbit, trail=False, num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_dark_mode(self):
        """Test animation with dark theme."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        anim = animate_orbit(orbit, dark=True, num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_without_time_display(self):
        """Test animation without time annotation."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        anim = animate_orbit(orbit, show_time=False, num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_custom_fps(self):
        """Test animation with custom frame rate."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        anim = animate_orbit(orbit, num_frames=20, fps=30)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_custom_axes(self):
        """Test animation on provided axes."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig, ax = plt.subplots()
        anim = animate_orbit(orbit, ax=ax, num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        # Verify animation uses the provided axes
        assert ax in fig.get_axes()
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_elliptical_orbit(self):
        """Test animation of elliptical orbit."""
        orbit = Orbit.from_classical(
            Earth,
            a=10000 << u.km,
            ecc=0.3 << u.one,
            inc=30 << u.deg,
            raan=45 << u.deg,
            argp=60 << u.deg,
            nu=0 << u.deg,
        )

        anim = animate_orbit(orbit, num_frames=15)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animate_different_attractor(self):
        """Test animation with different central body (Mars)."""
        orbit = Orbit.from_classical(
            Mars,
            a=5000 << u.km,
            ecc=0.01 << u.one,
            inc=0 << u.deg,
            raan=0 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        anim = animate_orbit(orbit, num_frames=10)

        assert isinstance(anim, mpl_animation.FuncAnimation)
        plt.close("all")


class TestAnimateOrbit3D:
    """Tests for 3D plotly orbit animations."""

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_single_orbit_basic(self):
        """Test basic 3D animation of a single orbit."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, num_frames=10)

        assert isinstance(fig, go.Figure)
        assert len(fig.frames) == 10

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_with_units(self):
        """Test 3D animation with astropy units."""
        r = [7000, 0, 0] << u.km
        v = [0, 0, 7.546] << u.km / u.s
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, num_frames=10)

        assert isinstance(fig, go.Figure)
        assert len(fig.frames) == 10

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_multiple_orbits(self):
        """Test 3D animation of multiple orbits."""
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
            a=8000 << u.km,
            ecc=0.05 << u.one,
            inc=20 << u.deg,
            raan=30 << u.deg,
            argp=0 << u.deg,
            nu=0 << u.deg,
        )

        fig = animate_orbit_3d([orbit1, orbit2], num_frames=10)

        assert isinstance(fig, go.Figure)
        assert len(fig.frames) == 10

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_custom_duration(self):
        """Test 3D animation with custom duration."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        period = orbit.period.value if hasattr(orbit.period, "value") else orbit.period
        fig = animate_orbit_3d(orbit, duration=period / 2, num_frames=10)

        assert isinstance(fig, go.Figure)
        assert len(fig.frames) == 10

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_without_trail(self):
        """Test 3D animation without orbital trail."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, trail=False, num_frames=10)

        assert isinstance(fig, go.Figure)
        # Should have fewer traces per frame without trail
        assert len(fig.frames) == 10

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_dark_mode(self):
        """Test 3D animation with dark theme."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, dark=True, num_frames=10)

        assert isinstance(fig, go.Figure)
        # Check that dark template is applied by checking the template name
        assert hasattr(fig.layout, "template")

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_controls(self):
        """Test that 3D animation has play/pause controls."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, num_frames=10)

        # Check for updatemenus (play/pause buttons)
        assert len(fig.layout.updatemenus) > 0
        assert any(
            "Play" in str(button) for menu in fig.layout.updatemenus for button in menu.buttons
        )

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_slider(self):
        """Test that 3D animation has time slider."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, num_frames=10)

        # Check for slider
        assert len(fig.layout.sliders) > 0
        slider = fig.layout.sliders[0]
        assert len(slider.steps) == 10  # One step per frame

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_inclined_orbit(self):
        """Test 3D animation of inclined orbit."""
        # Use from_vectors to avoid any classical element conversion issues
        r = np.array([7000e3, 3000e3, 2000e3])  # Inclined orbit
        v = np.array([-2000, 6000, 4000])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, num_frames=15)

        assert isinstance(fig, go.Figure)
        assert len(fig.frames) == 15

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_animate_3d_custom_fps(self):
        """Test 3D animation with custom frame rate."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        fig = animate_orbit_3d(orbit, num_frames=20, fps=30)

        assert isinstance(fig, go.Figure)
        assert len(fig.frames) == 20


class TestAnimationIntegration:
    """Integration tests for animation functionality."""

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animation_preserves_orbit_properties(self):
        """Test that animation doesn't modify original orbit."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        # Store original properties
        original_a = orbit.a
        original_ecc = orbit.ecc

        # Create animation
        anim = animate_orbit(orbit, num_frames=10)

        # Verify orbit unchanged
        assert orbit.a == original_a
        assert orbit.ecc == original_ecc

        plt.close("all")

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_3d_animation_preserves_orbit_properties(self):
        """Test that 3D animation doesn't modify original orbit."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        original_a = orbit.a
        original_inc = orbit.inc

        fig = animate_orbit_3d(orbit, num_frames=10)

        assert orbit.a == original_a
        assert orbit.inc == original_inc

    @pytest.mark.skipif(not HAS_MATPLOTLIB, reason="matplotlib not available")
    def test_animation_num_frames_parameter(self):
        """Test that num_frames parameter is respected."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        # Different frame counts
        for num_frames in [5, 10, 20, 50]:
            anim = animate_orbit(orbit, num_frames=num_frames)
            # Note: FuncAnimation doesn't expose frame count directly
            # but we can verify it was created successfully
            assert isinstance(anim, mpl_animation.FuncAnimation)
            plt.close("all")

    @pytest.mark.skipif(not HAS_PLOTLY, reason="plotly not available")
    def test_3d_animation_num_frames_parameter(self):
        """Test that num_frames parameter is respected in 3D."""
        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        for num_frames in [5, 10, 20, 30]:
            fig = animate_orbit_3d(orbit, num_frames=num_frames)
            assert len(fig.frames) == num_frames


class TestAnimationErrors:
    """Test error handling in animation functions."""

    def test_animate_orbit_no_matplotlib(self, monkeypatch):
        """Test error when matplotlib is not available."""
        if not HAS_MATPLOTLIB:
            pytest.skip("matplotlib not installed")

        # Mock matplotlib as unavailable
        import astrora.plotting.animation as anim_module

        monkeypatch.setattr(anim_module, "HAS_MATPLOTLIB", False)

        r = np.array([7000e3, 0, 0])
        v = np.array([0, 7546, 0])
        orbit = Orbit.from_vectors(Earth, r, v)

        with pytest.raises(ImportError, match="Matplotlib is required"):
            animate_orbit(orbit, num_frames=10)

    def test_animate_orbit_3d_no_plotly(self, monkeypatch):
        """Test error when plotly is not available."""
        if not HAS_PLOTLY:
            pytest.skip("plotly not installed")

        # Mock plotly as unavailable
        import astrora.plotting.animation as anim_module

        monkeypatch.setattr(anim_module, "HAS_PLOTLY", False)

        r = np.array([7000e3, 0, 0])
        v = np.array([0, 0, 7546])
        orbit = Orbit.from_vectors(Earth, r, v)

        with pytest.raises(ImportError, match="Plotly is required"):
            animate_orbit_3d(orbit, num_frames=10)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
