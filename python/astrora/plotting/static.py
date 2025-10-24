"""
Static orbit plotting with matplotlib.

This module provides a poliastro-compatible API for creating high-quality
static orbit visualizations using matplotlib.
"""

from typing import List, Optional, Tuple

import matplotlib.pyplot as plt
import numpy as np
from astropy import units as u
from matplotlib.axes import Axes
from matplotlib.patches import Circle

try:
    from .._core import Epoch
    from ..bodies import Body
    from ..maneuver import Maneuver
    from ..twobody import Orbit
except ImportError:
    # For standalone testing
    Orbit = None
    Body = None
    Maneuver = None
    Epoch = None


class StaticOrbitPlotter:
    """
    Static orbit plotter using matplotlib.

    This class provides a poliastro-compatible API for plotting orbits,
    trajectories, and celestial bodies in 2D using matplotlib. The first
    orbit plotted establishes the reference plane for subsequent plots.

    Parameters
    ----------
    ax : matplotlib.axes.Axes, optional
        Matplotlib axes to plot on. If None, creates a new figure and axes.
    plane : None, optional
        Reference plane for plotting. Currently not implemented (uses orbit plane).
    dark : bool, optional
        If True, uses dark theme. Default is False.

    Attributes
    ----------
    ax : matplotlib.axes.Axes
        The matplotlib axes being used for plotting
    attractor : Body
        The central body being used for the plot

    Examples
    --------
    >>> from astrora.plotting import StaticOrbitPlotter
    >>> from astrora.twobody import Orbit
    >>> from astrora.bodies import Earth
    >>> import numpy as np
    >>>
    >>> # Create an orbit
    >>> r = np.array([7000e3, 0, 0])  # meters
    >>> v = np.array([0, 7546, 0])
    >>> orbit = Orbit.from_vectors(Earth, r, v)
    >>>
    >>> # Plot it
    >>> plotter = StaticOrbitPlotter()
    >>> plotter.plot(orbit, label="ISS")
    >>> plotter.show()
    """

    def __init__(self, ax: Optional[Axes] = None, plane=None, dark: bool = False):
        """Initialize the static orbit plotter."""
        if ax is None:
            fig, ax = plt.subplots(figsize=(8, 8))

        self.ax = ax
        self._attractor: Optional[Body] = None
        self._frame_set = False
        self._dark = dark

        # Configure axes
        self.ax.set_aspect("equal")
        self.ax.grid(True, alpha=0.3)

        if dark:
            self.ax.set_facecolor("#1a1a1a")
            self.ax.spines["bottom"].set_color("white")
            self.ax.spines["top"].set_color("white")
            self.ax.spines["left"].set_color("white")
            self.ax.spines["right"].set_color("white")
            self.ax.tick_params(colors="white")
            self.ax.xaxis.label.set_color("white")
            self.ax.yaxis.label.set_color("white")

    @property
    def attractor(self) -> Optional[Body]:
        """Get the current attractor (central body)."""
        return self._attractor

    def set_attractor(self, attractor: Body) -> None:
        """
        Set the central attractor body.

        Parameters
        ----------
        attractor : Body
            The central body (e.g., Earth, Sun, Mars)
        """
        self._attractor = attractor

        # Draw attractor as a circle
        if hasattr(attractor, "R"):
            radius_m = attractor.R
            # Convert to km for display
            radius_km = radius_m / 1000.0

            circle = Circle(
                (0, 0),
                radius_km,
                color="#3d59ab" if not self._dark else "#4d69bb",
                label=attractor.name,
                zorder=10,
            )
            self.ax.add_patch(circle)

    def plot(
        self,
        orbit: "Orbit",
        *,
        label: Optional[str] = None,
        color: Optional[str] = None,
        trail: bool = False,
        num_points: int = 150,
    ) -> Tuple[List, plt.Line2D]:
        """
        Plot an orbit in its orbital plane.

        This method plots the complete orbit trajectory and marks the current
        position. The first orbit plotted establishes the reference plane.

        Parameters
        ----------
        orbit : Orbit
            The orbit object to plot
        label : str, optional
            Label for the orbit (appears in legend)
        color : str, optional
            Color for the orbit line. If None, uses default color cycle.
        trail : bool, optional
            If True, creates a fading trail effect. Default is False.
        num_points : int, optional
            Number of points to use for orbit trajectory. Default is 150.

        Returns
        -------
        trajectory : list
            List containing the matplotlib Line2D object for the orbit path
        position : matplotlib.lines.Line2D
            The Line2D object for the current position marker

        Examples
        --------
        >>> plotter = StaticOrbitPlotter()
        >>> traj, pos = plotter.plot(orbit, label="ISS", color="red")
        >>> traj[0].set_linewidth(2)  # Make orbit line thicker
        >>> pos.set_marker('s')  # Use square marker for position
        """
        # Set attractor if not already set
        if self._attractor is None:
            self.set_attractor(orbit.attractor)

        # Generate orbit points
        if orbit.ecc < 1.0:
            # Closed orbit - sample full period
            times = np.linspace(0, orbit.period, num_points)
        else:
            # Open orbit (parabolic/hyperbolic) - sample a range
            # Use ±3 times the periapsis passage time
            if hasattr(orbit, "period") and orbit.period > 0:
                t_range = 3 * orbit.period
            else:
                # Estimate based on periapsis
                t_range = 2 * np.pi * np.sqrt(orbit.p**3 / orbit.attractor.mu)
            times = np.linspace(-t_range, t_range, num_points)

        # Sample positions
        positions, _ = orbit.sample(times)

        # Convert to km for display
        # Check if positions have units
        if hasattr(positions, "unit"):
            x = positions[:, 0].to(u.km).value
            y = positions[:, 1].to(u.km).value
        else:
            # Raw values in meters
            x = positions[:, 0] / 1000.0
            y = positions[:, 1] / 1000.0

        # Plot trajectory
        line_kwargs = {"label": label}
        if color is not None:
            line_kwargs["color"] = color

        if trail:
            # Create fading trail effect
            segments = len(x) // 10
            alpha_values = np.linspace(0.2, 1.0, segments)
            lines = []
            for i in range(segments):
                start = i * (len(x) // segments)
                end = (i + 1) * (len(x) // segments)
                (line,) = self.ax.plot(
                    x[start:end], y[start:end], alpha=alpha_values[i], **line_kwargs
                )
                lines.append(line)
                line_kwargs.pop("label", None)  # Only label first segment
        else:
            (line,) = self.ax.plot(x, y, **line_kwargs)
            lines = [line]

        # Plot current position
        if hasattr(orbit.r, "unit"):
            pos_x = orbit.r[0].to(u.km).value
            pos_y = orbit.r[1].to(u.km).value
        else:
            pos_x = orbit.r[0] / 1000.0
            pos_y = orbit.r[1] / 1000.0

        pos_color = color if color is not None else lines[0].get_color()
        (position,) = self.ax.plot(
            pos_x,
            pos_y,
            marker="o",
            markersize=8,
            color=pos_color,
            markeredgecolor="white" if not self._dark else "black",
            markeredgewidth=1.5,
            zorder=20,
        )

        # Update axes labels if not set
        if not self.ax.get_xlabel():
            self.ax.set_xlabel("x (km)")
        if not self.ax.get_ylabel():
            self.ax.set_ylabel("y (km)")

        return lines, position

    def plot_body_orbit(
        self,
        body: Body,
        epoch: Optional["Epoch"] = None,
        *,
        label: Optional[str] = None,
        color: Optional[str] = None,
        trail: bool = False,
    ) -> Tuple[List, plt.Line2D]:
        """
        Plot a celestial body's orbit around the attractor.

        This creates a circular or elliptical orbit for the body based on
        its orbital parameters.

        Parameters
        ----------
        body : Body
            The celestial body whose orbit to plot
        epoch : Epoch, optional
            The epoch for the body's position. If None, uses J2000.
        label : str, optional
            Label for the orbit. If None, uses body name.
        color : str, optional
            Color for the orbit line
        trail : bool, optional
            If True, creates a fading trail effect

        Returns
        -------
        trajectory : list
            List containing the matplotlib Line2D object for the orbit path
        position : matplotlib.lines.Line2D
            The Line2D object for the current position marker

        Notes
        -----
        This is a simplified implementation that assumes circular orbits.
        For accurate planetary positions, use JPL ephemerides with Orbit.from_ephem().
        """
        # This is a placeholder - in a full implementation, you would
        # query ephemerides or use predefined orbital elements
        raise NotImplementedError(
            "plot_body_orbit requires ephemeris data integration. "
            "Use plot() with an Orbit object created from ephemerides instead."
        )

    def plot_trajectory(
        self,
        coordinates: np.ndarray,
        *,
        label: Optional[str] = None,
        color: Optional[str] = None,
        trail: bool = False,
    ) -> Tuple[List, plt.Line2D]:
        """
        Plot a precomputed trajectory.

        Parameters
        ----------
        coordinates : np.ndarray
            Array of position vectors, shape (N, 3) in meters or (N, 3) Quantity
        label : str, optional
            Label for the trajectory
        color : str, optional
            Color for the trajectory line
        trail : bool, optional
            If True, creates a fading trail effect

        Returns
        -------
        trajectory : list
            List containing the matplotlib Line2D object for the path
        position : matplotlib.lines.Line2D
            The Line2D object for the final position marker
        """
        if self._attractor is None:
            raise ValueError("Must set attractor before plotting trajectory")

        # Convert to km
        if hasattr(coordinates, "unit"):
            x = coordinates[:, 0].to(u.km).value
            y = coordinates[:, 1].to(u.km).value
        else:
            x = coordinates[:, 0] / 1000.0
            y = coordinates[:, 1] / 1000.0

        # Plot trajectory
        line_kwargs = {"label": label}
        if color is not None:
            line_kwargs["color"] = color

        if trail:
            segments = len(x) // 10
            alpha_values = np.linspace(0.2, 1.0, segments)
            lines = []
            for i in range(segments):
                start = i * (len(x) // segments)
                end = (i + 1) * (len(x) // segments)
                (line,) = self.ax.plot(
                    x[start:end], y[start:end], alpha=alpha_values[i], **line_kwargs
                )
                lines.append(line)
                line_kwargs.pop("label", None)
        else:
            (line,) = self.ax.plot(x, y, **line_kwargs)
            lines = [line]

        # Mark final position
        pos_color = color if color is not None else lines[0].get_color()
        (position,) = self.ax.plot(
            x[-1],
            y[-1],
            marker="o",
            markersize=8,
            color=pos_color,
            markeredgecolor="white" if not self._dark else "black",
            markeredgewidth=1.5,
            zorder=20,
        )

        return lines, position

    def plot_maneuver(
        self,
        initial_orbit: "Orbit",
        maneuver: "Maneuver",
        *,
        label: Optional[str] = None,
        color: Optional[str] = None,
        trail: bool = False,
    ) -> Tuple[List, plt.Line2D]:
        """
        Plot a maneuver trajectory.

        Parameters
        ----------
        initial_orbit : Orbit
            The initial orbit before the maneuver
        maneuver : Maneuver
            The maneuver to apply
        label : str, optional
            Label for the maneuver trajectory
        color : str, optional
            Color for the trajectory line
        trail : bool, optional
            If True, creates a fading trail effect

        Returns
        -------
        trajectory : list
            List containing the matplotlib Line2D object for the path
        position : matplotlib.lines.Line2D
            The Line2D object for the final position marker
        """
        # Apply maneuver to get final orbit
        # This requires the Maneuver class which should have an apply() method
        if not hasattr(maneuver, "impulses"):
            raise ValueError("Maneuver must have impulses attribute")

        # For now, plot the initial orbit
        # A full implementation would show the transfer trajectory
        return self.plot(initial_orbit, label=label or "Maneuver", color=color, trail=trail)

    def show(self) -> None:
        """
        Display the plot.

        This is a convenience method that calls matplotlib's show() and
        adds a legend if any labeled orbits have been plotted.
        """
        # Add legend if there are labeled items
        handles, labels = self.ax.get_legend_handles_labels()
        if labels:
            self.ax.legend(loc="upper right")

        plt.tight_layout()
        plt.show()

    def savefig(self, filename: str, **kwargs) -> None:
        """
        Save the plot to a file.

        Parameters
        ----------
        filename : str
            Output filename
        **kwargs
            Additional arguments passed to matplotlib's savefig()
        """
        # Add legend before saving
        handles, labels = self.ax.get_legend_handles_labels()
        if labels:
            self.ax.legend(loc="upper right")

        plt.tight_layout()
        self.ax.figure.savefig(filename, **kwargs)
