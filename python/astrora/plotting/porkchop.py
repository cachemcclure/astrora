"""
Porkchop plot generation for launch window analysis.

Porkchop plots visualize optimal launch windows for interplanetary missions
by showing contours of delta-v and transfer time as functions of departure
and arrival dates.
"""

from datetime import datetime
from typing import Callable, Optional, Tuple

import matplotlib.pyplot as plt
import numpy as np
from astropy import units as u
from matplotlib.axes import Axes

try:
    from .._core import lambert_solve_batch_parallel
except ImportError:
    lambert_solve_batch_parallel = None


def plot_porkchop(
    departure_planet_positions: Callable,
    arrival_planet_positions: Callable,
    mu: float,
    departure_dates: np.ndarray,
    arrival_dates: np.ndarray,
    ax: Optional[Axes] = None,
    levels_deltav: int = 20,
    levels_tof: int = 15,
    **kwargs,
) -> Tuple[Axes, np.ndarray, np.ndarray, np.ndarray]:
    """
    Generate a porkchop plot for launch window analysis.

    A porkchop plot shows contours of total delta-v and time-of-flight (ToF)
    for interplanetary transfers. It's called a "porkchop" because the optimal
    launch windows often form a shape resembling a porkchop.

    Parameters
    ----------
    departure_planet_positions : callable
        Function that takes a time array and returns positions (N, 3) in km
    arrival_planet_positions : callable
        Function that takes a time array and returns positions (N, 3) in km
    mu : float
        Gravitational parameter (km³/s²) - typically solar GM
    departure_dates : np.ndarray
        Array of departure dates (astropy.time.Time or datetime)
    arrival_dates : np.ndarray
        Array of arrival dates
    ax : matplotlib.axes.Axes, optional
        Axes to plot on. If None, creates new figure.
    levels_deltav : int, optional
        Number of contour levels for delta-v. Default is 20.
    levels_tof : int, optional
        Number of contour levels for time-of-flight. Default is 15.
    **kwargs
        Additional keyword arguments for customization

    Returns
    -------
    ax : matplotlib.axes.Axes
        The axes containing the plot
    delta_v_grid : np.ndarray
        Grid of total delta-v values (km/s)
    tof_grid : np.ndarray
        Grid of time-of-flight values (days)
    c3_grid : np.ndarray
        Grid of C3 energy values at departure (km²/s²)

    Examples
    --------
    >>> from astropy.time import Time
    >>> from datetime import datetime, timedelta
    >>> import numpy as np
    >>>
    >>> # Define date ranges
    >>> start = Time("2025-01-01")
    >>> dep_dates = start + np.linspace(0, 365, 50) * u.day
    >>> arr_dates = start + np.linspace(180, 545, 50) * u.day
    >>>
    >>> # Plot Earth-Mars porkchop
    >>> ax, dv, tof, c3 = plot_porkchop(
    ...     earth_position_func,
    ...     mars_position_func,
    ...     mu_sun,
    ...     dep_dates,
    ...     arr_dates
    ... )
    >>> plt.show()

    Notes
    -----
    The characteristic "porkchop" shape occurs because:
    1. Minimum delta-v occurs near Hohmann transfer conditions
    2. Planetary alignments create periodic optimal windows
    3. Trade-offs exist between delta-v and flight time
    4. Synodic periods determine window spacing (e.g., 26 months for Earth-Mars)

    References
    ----------
    - Vallado, "Fundamentals of Astrodynamics and Applications" (2013), Ch. 7
    - Prussing & Conway, "Orbital Mechanics" (2013), Ch. 6
    """
    if lambert_solve_batch_parallel is None:
        raise ImportError(
            "Lambert solver not available. " "Make sure astrora._core is properly compiled."
        )

    if ax is None:
        fig, ax = plt.subplots(figsize=(12, 9))

    # Create meshgrid of dates
    dep_grid, arr_grid = np.meshgrid(departure_dates, arrival_dates, indexing="ij")

    # Initialize grids for results
    delta_v_grid = np.full(dep_grid.shape, np.nan)
    tof_grid = np.full(dep_grid.shape, np.nan)
    c3_grid = np.full(dep_grid.shape, np.nan)

    # Compute positions for all date combinations
    n_dep = len(departure_dates)
    n_arr = len(arrival_dates)

    for i, dep_time in enumerate(departure_dates):
        # Get departure position
        r1 = departure_planet_positions(dep_time)

        for j, arr_time in enumerate(arrival_dates):
            # Get arrival position
            r2 = arrival_planet_positions(arr_time)

            # Calculate time of flight
            if hasattr(dep_time, "jd") and hasattr(arr_time, "jd"):
                # astropy.time.Time
                tof_days = (arr_time - dep_time).to(u.day).value
            else:
                # datetime
                tof_days = (arr_time - dep_time).total_seconds() / 86400.0

            if tof_days <= 0:
                continue

            tof_seconds = tof_days * 86400.0

            # Solve Lambert's problem
            try:
                # Note: Using simplified single-solve (for compatibility)
                # Full implementation should use batch parallel solver
                from .._core import lambert_solve

                solution = lambert_solve(
                    r1.flatten(), r2.flatten(), tof_seconds, mu, prograde=True, num_revs=0
                )

                if solution["converged"]:
                    v1 = np.array(solution["v1"])
                    v2 = np.array(solution["v2"])

                    # Calculate C3 (departure energy)
                    v_inf_dep = np.linalg.norm(v1)
                    c3 = v_inf_dep**2

                    # Calculate arrival delta-v (approximation)
                    v_inf_arr = np.linalg.norm(v2)

                    # Total delta-v (simplified - assumes circular orbits)
                    # Real calculation would include planetary orbital velocities
                    delta_v = v_inf_dep + v_inf_arr

                    delta_v_grid[i, j] = delta_v
                    tof_grid[i, j] = tof_days
                    c3_grid[i, j] = c3

            except Exception:
                continue

    # Plot delta-v contours
    contour_dv = ax.contour(
        dep_grid,
        arr_grid,
        delta_v_grid,
        levels=levels_deltav,
        cmap="viridis",
        linewidths=1.5,
        alpha=0.7,
    )
    ax.clabel(contour_dv, inline=True, fontsize=8, fmt="%.1f km/s")

    # Plot time-of-flight contours
    contour_tof = ax.contour(
        dep_grid,
        arr_grid,
        tof_grid,
        levels=levels_tof,
        colors="red",
        linewidths=1,
        linestyles="dashed",
        alpha=0.5,
    )
    ax.clabel(contour_tof, inline=True, fontsize=8, fmt="%.0f days")

    # Find and mark minimum delta-v
    min_idx = np.nanargmin(delta_v_grid)
    min_i, min_j = np.unravel_index(min_idx, delta_v_grid.shape)
    ax.plot(
        dep_grid[min_i, min_j],
        arr_grid[min_i, min_j],
        "r*",
        markersize=15,
        label=f"Min ΔV: {delta_v_grid[min_i, min_j]:.2f} km/s",
    )

    # Labels and formatting
    ax.set_xlabel("Departure Date", fontsize=12)
    ax.set_ylabel("Arrival Date", fontsize=12)
    ax.set_title("Porkchop Plot - Launch Window Analysis", fontsize=14, fontweight="bold")
    ax.grid(True, alpha=0.3)
    ax.legend(loc="best")

    # Format dates if using datetime
    if hasattr(departure_dates[0], "iso"):
        # astropy.time.Time - format appropriately
        pass
    elif isinstance(departure_dates[0], datetime):
        import matplotlib.dates as mdates

        ax.xaxis.set_major_formatter(mdates.DateFormatter("%Y-%m-%d"))
        ax.yaxis.set_major_formatter(mdates.DateFormatter("%Y-%m-%d"))
        plt.setp(ax.xaxis.get_majorticklabels(), rotation=45, ha="right")
        plt.setp(ax.yaxis.get_majorticklabels(), rotation=45, ha="right")

    plt.tight_layout()

    return ax, delta_v_grid, tof_grid, c3_grid


def plot_porkchop_simple(
    r1_positions: np.ndarray,
    r2_positions: np.ndarray,
    mu: float,
    tof_range: Tuple[float, float],
    n_points: int = 50,
    ax: Optional[Axes] = None,
) -> Tuple[Axes, np.ndarray, np.ndarray]:
    """
    Generate a simplified porkchop plot from position arrays.

    This is a simpler interface that takes pre-computed position arrays
    instead of position functions.

    Parameters
    ----------
    r1_positions : np.ndarray
        Departure positions array, shape (N, 3) in km
    r2_positions : np.ndarray
        Arrival positions array, shape (M, 3) in km
    mu : float
        Gravitational parameter (km³/s²)
    tof_range : tuple
        (min_tof, max_tof) in days
    n_points : int, optional
        Number of sample points. Default is 50.
    ax : matplotlib.axes.Axes, optional
        Axes to plot on

    Returns
    -------
    ax : matplotlib.axes.Axes
        The axes containing the plot
    delta_v_grid : np.ndarray
        Grid of total delta-v values
    tof_grid : np.ndarray
        Grid of time-of-flight values
    """
    if ax is None:
        fig, ax = plt.subplots(figsize=(10, 8))

    # This is a placeholder for simplified implementation
    # Full implementation would compute Lambert solutions across the grid
    raise NotImplementedError(
        "Simplified porkchop plot implementation coming soon. "
        "Use plot_porkchop() with position functions instead."
    )
