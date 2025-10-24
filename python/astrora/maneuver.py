"""
Orbital maneuvers and trajectory corrections.

This module provides the Maneuver class for representing impulsive maneuvers
(instantaneous velocity changes) and factory methods for common orbital transfers.
"""

from typing import List, Tuple, Union

import numpy as np

from astrora._core import (
    Duration,
    bielliptic_transfer,
    hohmann_transfer,
    lambert_solve,
)


class Maneuver:
    """
    Represents a sequence of impulsive maneuvers (Δv).

    A Maneuver consists of one or more impulses, each defined by:
    - Time offset from initial epoch (seconds or Duration)
    - Velocity change vector (Δv) in m/s

    Attributes
    ----------
    impulses : list of tuple
        List of (time_offset, delta_v) pairs where:
        - time_offset: float (seconds) or Duration object
        - delta_v: np.ndarray (3-element velocity change vector in m/s)

    Examples
    --------
    >>> import numpy as np
    >>> from astrora.maneuver import Maneuver
    >>> from astrora._core import Duration
    >>>
    >>> # Single impulse
    >>> dv = np.array([100, 0, 0])  # 100 m/s prograde
    >>> maneuver = Maneuver.impulse(dv)
    >>>
    >>> # Multiple impulses
    >>> dv1 = np.array([50, 0, 0])
    >>> dv2 = np.array([0, 30, 0])
    >>> maneuver = Maneuver(
    ...     (0, dv1),
    ...     (Duration.from_hours(6), dv2)
    ... )
    >>>
    >>> # Hohmann transfer
    >>> from astrora.twobody import Orbit
    >>> from astrora.bodies import Earth
    >>> orbit = Orbit.from_classical(Earth, a=7000e3, ecc=0.01, inc=0, raan=0, argp=0, nu=0)
    >>> maneuver = Maneuver.hohmann(orbit, 42164e3)  # Transfer to GEO
    """

    def __init__(self, *impulses: Tuple[Union[float, Duration], np.ndarray]):
        """
        Initialize a Maneuver with one or more impulses.

        Parameters
        ----------
        *impulses : tuple
            Variable number of (time_offset, delta_v) tuples where:
            - time_offset: float (seconds) or Duration object
            - delta_v: np.ndarray (3-element array in m/s)

        Examples
        --------
        >>> dv1 = np.array([100, 0, 0])
        >>> dv2 = np.array([50, 0, 0])
        >>> maneuver = Maneuver((0, dv1), (3600, dv2))
        """
        if len(impulses) == 0:
            raise ValueError("Maneuver must have at least one impulse")

        self._impulses = []
        for time_offset, delta_v in impulses:
            # Convert Duration to float seconds if needed
            if isinstance(time_offset, Duration):
                time_offset = time_offset.seconds

            # Ensure delta_v is a numpy array
            delta_v = np.asarray(delta_v, dtype=np.float64)
            if delta_v.shape != (3,):
                raise ValueError(f"delta_v must be 3-element array, got shape {delta_v.shape}")

            self._impulses.append((float(time_offset), delta_v))

    @property
    def impulses(self) -> List[Tuple[float, np.ndarray]]:
        """
        List of (time_offset, delta_v) impulse tuples.

        Returns
        -------
        list of tuple
            Each tuple is (time_offset_seconds, delta_v_array)
        """
        return self._impulses.copy()

    def __getitem__(self, key: int) -> Tuple[float, np.ndarray]:
        """
        Access impulse by index.

        Parameters
        ----------
        key : int
            Index of the impulse

        Returns
        -------
        tuple
            (time_offset, delta_v) tuple

        Examples
        --------
        >>> maneuver = Maneuver.impulse(np.array([100, 0, 0]))
        >>> t, dv = maneuver[0]
        >>> print(f"Time: {t}s, Delta-v: {np.linalg.norm(dv):.1f} m/s")
        """
        return self._impulses[key]

    def __len__(self) -> int:
        """Number of impulses in this maneuver."""
        return len(self._impulses)

    # ========================================================================
    # Analysis Methods
    # ========================================================================

    def get_total_time(self) -> float:
        """
        Get the total time span of the maneuver.

        Returns
        -------
        float
            Total time from first to last impulse (seconds)

        Examples
        --------
        >>> maneuver = Maneuver((0, dv1), (3600, dv2), (7200, dv3))
        >>> print(f"Total duration: {maneuver.get_total_time()/3600:.1f} hours")
        """
        if len(self._impulses) == 0:
            return 0.0
        if len(self._impulses) == 1:
            return 0.0

        times = [t for t, _ in self._impulses]
        return max(times) - min(times)

    def get_total_cost(self) -> float:
        """
        Get the total delta-v cost (sum of magnitudes).

        Returns
        -------
        float
            Total Δv magnitude in m/s

        Notes
        -----
        This is the scalar sum of delta-v magnitudes, which represents
        the total propellant cost (via Tsiolkovsky rocket equation).

        Examples
        --------
        >>> maneuver = Maneuver.hohmann(orbit, 42164e3)
        >>> print(f"Total Δv: {maneuver.get_total_cost():.1f} m/s")
        """
        return sum(np.linalg.norm(dv) for _, dv in self._impulses)

    # ========================================================================
    # Factory Methods - Simple Maneuvers
    # ========================================================================

    @classmethod
    def impulse(cls, delta_v: np.ndarray) -> "Maneuver":
        """
        Create a single impulsive maneuver at time t=0.

        Parameters
        ----------
        delta_v : np.ndarray
            Velocity change vector [Δvx, Δvy, Δvz] in m/s (3-element array)

        Returns
        -------
        Maneuver
            Maneuver with single impulse at t=0

        Examples
        --------
        >>> # 100 m/s prograde burn
        >>> orbit = Orbit.from_classical(Earth, a=7000e3, ecc=0, inc=0, raan=0, argp=0, nu=0)
        >>> v_hat = orbit.v / np.linalg.norm(orbit.v)
        >>> maneuver = Maneuver.impulse(100 * v_hat)
        """
        delta_v = np.asarray(delta_v, dtype=np.float64)
        return cls((0.0, delta_v))

    # ========================================================================
    # Factory Methods - Orbital Transfers
    # ========================================================================

    @classmethod
    def hohmann(cls, orbit_i, r_f: float) -> "Maneuver":
        """
        Create a Hohmann transfer maneuver to a circular orbit.

        A Hohmann transfer is the most fuel-efficient two-impulse transfer
        between coplanar circular orbits.

        Parameters
        ----------
        orbit_i : Orbit
            Initial circular orbit
        r_f : float
            Final orbit radius (meters, measured from center of attractor)

        Returns
        -------
        Maneuver
            Two-impulse maneuver for Hohmann transfer

        Raises
        ------
        ValueError
            If initial orbit is not approximately circular (ecc > 0.001)

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>>
        >>> # LEO to GEO transfer
        >>> leo = Orbit.from_classical(Earth, a=6778e3, ecc=0, inc=0, raan=0, argp=0, nu=0)
        >>> maneuver = Maneuver.hohmann(leo, 42164e3)
        >>> print(f"Total Δv: {maneuver.get_total_cost():.1f} m/s")
        >>> print(f"Transfer time: {maneuver.get_total_time()/3600:.1f} hours")

        Notes
        -----
        The initial orbit should be approximately circular. For eccentric orbits,
        use Lambert solver or other methods.

        References
        ----------
        - Curtis, "Orbital Mechanics for Engineering Students", Ch. 6.2
        - Vallado, "Fundamentals of Astrodynamics", Ch. 6.2
        """
        # Check that initial orbit is approximately circular
        if orbit_i.ecc > 0.001:
            raise ValueError(
                f"Initial orbit must be approximately circular (ecc < 0.001), "
                f"got ecc = {orbit_i.ecc:.6f}. Consider using Lambert solver instead."
            )

        # Get initial radius (semi-major axis for circular orbit)
        r_i = orbit_i.a

        # Call Rust backend for Hohmann calculation
        result = hohmann_transfer(r_i, r_f, orbit_i.attractor.mu)

        # Create unit vectors for velocity direction
        # For circular orbit, velocity is perpendicular to position
        r_vec = orbit_i.r
        v_vec = orbit_i.v
        v_hat = v_vec / np.linalg.norm(v_vec)  # Velocity direction

        # First impulse: apply delta_v1 in velocity direction
        dv1 = result["delta_v1"] * v_hat

        # Second impulse: apply delta_v2 in velocity direction at apoapsis/periapsis
        # (velocity direction remains the same for coplanar transfer)
        dv2 = result["delta_v2"] * v_hat

        # Transfer time is when second burn occurs
        transfer_time = result["transfer_time"]

        return cls((0.0, dv1), (transfer_time, dv2))

    @classmethod
    def bielliptic(cls, orbit_i, r_b: float, r_f: float) -> "Maneuver":
        """
        Create a bi-elliptic transfer maneuver.

        A bi-elliptic transfer is a three-impulse maneuver that can be more
        fuel-efficient than Hohmann for large radius ratios (r_f/r_i > ~15.58).

        Parameters
        ----------
        orbit_i : Orbit
            Initial circular orbit
        r_b : float
            Intermediate apoapsis radius (meters)
        r_f : float
            Final orbit radius (meters)

        Returns
        -------
        Maneuver
            Three-impulse maneuver for bi-elliptic transfer

        Raises
        ------
        ValueError
            If initial orbit is not approximately circular (ecc > 0.001)

        Examples
        --------
        >>> # Transfer from LEO to very high orbit using bi-elliptic
        >>> leo = Orbit.from_classical(Earth, a=6778e3, ecc=0, inc=0, raan=0, argp=0, nu=0)
        >>> r_intermediate = 100000e3  # 100,000 km apoapsis
        >>> r_final = 50000e3          # 50,000 km final orbit
        >>> maneuver = Maneuver.bielliptic(leo, r_intermediate, r_final)
        >>> print(f"Total Δv: {maneuver.get_total_cost():.1f} m/s")

        Notes
        -----
        Bi-elliptic transfers trade longer transfer time for reduced delta-v.
        They are typically only beneficial when r_f/r_i > 15.58.

        References
        ----------
        - Curtis, "Orbital Mechanics for Engineering Students", Ch. 6.3
        """
        # Check that initial orbit is approximately circular
        if orbit_i.ecc > 0.001:
            raise ValueError(
                f"Initial orbit must be approximately circular (ecc < 0.001), "
                f"got ecc = {orbit_i.ecc:.6f}"
            )

        # Get initial radius
        r_i = orbit_i.a

        # Call Rust backend for bi-elliptic calculation
        result = bielliptic_transfer(r_i, r_b, r_f, orbit_i.attractor.mu)

        # Velocity unit vector
        v_hat = orbit_i.v / np.linalg.norm(orbit_i.v)

        # Three impulses
        dv1 = result["delta_v1"] * v_hat
        dv2 = result["delta_v2"] * v_hat
        dv3 = result["delta_v3"] * v_hat

        # Times for each burn
        # First burn at t=0
        # Second burn at first half-period (at apoapsis r_b)
        time_1 = 0.0
        time_2 = (
            result["transfer_time"]
            * (result["transfer1_sma"] ** 1.5)
            / (result["transfer1_sma"] ** 1.5 + result["transfer2_sma"] ** 1.5)
        )
        time_3 = result["transfer_time"]

        return cls((time_1, dv1), (time_2, dv2), (time_3, dv3))

    @classmethod
    def lambert(
        cls,
        orbit_i,
        orbit_f,
        short_way: bool = True,
        num_revs: int = 0,
    ) -> "Maneuver":
        """
        Create a Lambert transfer maneuver between two orbits.

        Lambert's problem solves for the velocity vectors needed to transfer
        between two position vectors in a given time of flight.

        Parameters
        ----------
        orbit_i : Orbit
            Initial orbit (position and epoch define departure point)
        orbit_f : Orbit
            Final orbit (position and epoch define arrival point)
        short_way : bool, optional
            If True, use short-way transfer (Δν < 180°), else long-way.
            Default: True
        num_revs : int, optional
            Number of complete revolutions (0 for direct transfer).
            Default: 0

        Returns
        -------
        Maneuver
            Two-impulse maneuver for Lambert transfer

        Examples
        --------
        >>> # Rendezvous maneuver
        >>> orbit1 = Orbit.from_vectors(Earth, r1, v1, epoch1)
        >>> orbit2 = Orbit.from_vectors(Earth, r2, v2, epoch2)
        >>> maneuver = Maneuver.lambert(orbit1, orbit2)
        >>> print(f"Total Δv: {maneuver.get_total_cost():.1f} m/s")

        Notes
        -----
        The time of flight is determined by the difference between the two
        orbit epochs. For interplanetary transfers or rendezvous planning.

        References
        ----------
        - Curtis, "Orbital Mechanics for Engineering Students", Ch. 5
        - Vallado, "Fundamentals of Astrodynamics", Ch. 7
        - Izzo, D. (2015). "Revisiting Lambert's problem"
        """
        # Check that both orbits have the same attractor
        if orbit_i.attractor != orbit_f.attractor:
            raise ValueError("Both orbits must have the same attractor")

        # Calculate time of flight from epoch difference
        dt = (orbit_f.epoch - orbit_i.epoch).seconds

        if dt <= 0:
            raise ValueError(
                f"Final epoch must be after initial epoch. " f"Time difference: {dt:.1f} seconds"
            )

        # Get position vectors
        r1 = orbit_i.r
        r2 = orbit_f.r

        # Solve Lambert's problem using Rust backend
        result = lambert_solve(r1, r2, dt, orbit_i.attractor.mu, short_way, num_revs)

        # Extract velocity vectors from Lambert solution
        v1_transfer = np.array(result["v1"])  # Required initial velocity
        v2_transfer = np.array(result["v2"])  # Required final velocity

        # Calculate delta-v impulses
        dv1 = v1_transfer - orbit_i.v  # Departure burn
        dv2 = orbit_f.v - v2_transfer  # Arrival burn (to match target velocity)

        return cls((0.0, dv1), (dt, dv2))

    # ========================================================================
    # String Representation
    # ========================================================================

    def __repr__(self) -> str:
        """String representation of the maneuver."""
        lines = [f"Maneuver with {len(self._impulses)} impulse(s):"]
        for i, (t, dv) in enumerate(self._impulses):
            dv_mag = np.linalg.norm(dv)
            lines.append(
                f"  [{i}] t={t:>10.1f}s, Δv={dv_mag:>8.3f} m/s "
                f"[{dv[0]:>8.3f}, {dv[1]:>8.3f}, {dv[2]:>8.3f}]"
            )
        lines.append(f"Total Δv: {self.get_total_cost():.3f} m/s")
        lines.append(f"Total time: {self.get_total_time():.1f} s")
        return "\n".join(lines)
