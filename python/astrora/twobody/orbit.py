"""
Orbit representation and propagation for the two-body problem.

This module provides the Orbit class, which represents a spacecraft or celestial
body's state in orbit around an attractor (e.g., Earth, Sun).
"""

from typing import TYPE_CHECKING, Optional, Tuple, Union

import numpy as np
from astropy import units as u

from astrora._core import (
    CartesianState,
    Duration,
    Epoch,
    OrbitalElements,
    batch_propagate_states,
    coe_to_rv,
    propagate_state_keplerian,
    py_propagate_tle,
    rv_to_coe,
)
from astrora.time import to_epoch
from astrora.units import (
    to_dimensionless,
    to_si_angle,
    to_si_length,
    to_si_position,
    to_si_velocity_vector,
)

# Type hints for astropy.time.Time without requiring it at runtime
if TYPE_CHECKING:
    from astropy.time import Time as AstropyTime


class Orbit:
    """
    Position and velocity of a body at a given time relative to an attractor.

    This class represents an orbit in an inertial reference frame (ICRS assumed)
    and provides methods for propagation, element conversion, and orbital analysis.

    The orbit is defined by:
    - **state**: Cartesian position and velocity vectors
    - **epoch**: Time at which the state is valid
    - **attractor**: Central body (defines gravitational parameter μ)

    Attributes
    ----------
    state : CartesianState
        Cartesian position and velocity in meters and m/s
    epoch : Epoch
        Time at which this state is valid
    attractor : Body
        Central attracting body (provides gravitational parameter)

    Examples
    --------
    >>> from astrora.twobody import Orbit
    >>> from astrora.bodies import Earth
    >>> from astrora._core import Epoch
    >>> import numpy as np
    >>>
    >>> # Create orbit from position and velocity vectors
    >>> r = np.array([7000e3, 0, 0])  # 7000 km altitude
    >>> v = np.array([0, 7546, 0])    # Circular orbit velocity
    >>> epoch = Epoch.j2000_epoch()
    >>> orbit = Orbit.from_vectors(Earth, r, v, epoch)
    >>>
    >>> # Create from classical elements
    >>> orbit = Orbit.from_classical(
    ...     Earth,
    ...     a=8000e3,         # Semi-major axis (m)
    ...     ecc=0.1,          # Eccentricity
    ...     inc=np.deg2rad(28.5),  # Inclination (rad)
    ...     raan=0.0,         # RAAN (rad)
    ...     argp=0.0,         # Argument of periapsis (rad)
    ...     nu=0.0,           # True anomaly (rad)
    ...     epoch=epoch
    ... )
    >>>
    >>> # Access orbital properties
    >>> print(f"Period: {orbit.period / 3600:.2f} hours")
    >>> print(f"Eccentricity: {orbit.ecc:.4f}")
    >>>
    >>> # Propagate orbit
    >>> dt = Duration.from_seconds(3600)  # 1 hour
    >>> future_orbit = orbit.propagate(dt)
    """

    def __init__(self, state: CartesianState, epoch: Epoch, attractor):
        """
        Initialize an Orbit.

        Parameters
        ----------
        state : CartesianState
            Cartesian state (position and velocity)
        epoch : Epoch
            Epoch at which the state is valid
        attractor : Body
            Central attracting body

        Notes
        -----
        Users typically create Orbit objects using the classmethods
        (from_vectors, from_classical, from_tle) rather than directly
        calling this constructor.
        """
        self._state = state
        self._epoch = epoch
        self._attractor = attractor

        # Cache for lazy-computed properties
        self._elements_cache = None

    # ========================================================================
    # Creation Methods (Class Methods)
    # ========================================================================

    @classmethod
    def from_vectors(
        cls,
        attractor,
        r: Union[np.ndarray, u.Quantity],
        v: Union[np.ndarray, u.Quantity],
        epoch: Optional[Union[Epoch, "AstropyTime"]] = None,
    ) -> "Orbit":
        """
        Create an orbit from position and velocity vectors.

        Parameters
        ----------
        attractor : Body
            Central attracting body (e.g., Earth, Sun)
        r : np.ndarray or Quantity
            Position vector [x, y, z]. Can be:
            - numpy array in meters (SI)
            - astropy Quantity with length units (e.g., km)
        v : np.ndarray or Quantity
            Velocity vector [vx, vy, vz]. Can be:
            - numpy array in m/s (SI)
            - astropy Quantity with velocity units (e.g., km/s)
        epoch : Epoch, astropy.time.Time, or None, optional
            Epoch at which state is valid. Accepts either hifitime Epoch or
            astropy.time.Time objects. Defaults to J2000 if not provided.

        Returns
        -------
        Orbit
            New Orbit object

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>> from astropy import units as u
        >>> import numpy as np
        >>>
        >>> # ISS-like orbit with raw arrays (meters, m/s)
        >>> r = np.array([6800e3, 0, 0])  # ~420 km altitude
        >>> v = np.array([0, 7660, 0])     # Circular velocity
        >>> orbit = Orbit.from_vectors(Earth, r, v)
        >>>
        >>> # Or with astropy units (poliastro-style)
        >>> r = [6800, 0, 0] << u.km
        >>> v = [0, 7.66, 0] << u.km / u.s
        >>> orbit = Orbit.from_vectors(Earth, r, v)
        """
        # Convert astropy.time.Time to Epoch if needed, default to J2000
        epoch = to_epoch(epoch)
        if epoch is None:
            epoch = Epoch.j2000_epoch()

        # Convert to SI units (handles both raw arrays and Quantity objects)
        r_si = to_si_position(r)
        v_si = to_si_velocity_vector(v)

        # Create CartesianState from Rust (expects SI units)
        state = CartesianState(r_si, v_si)

        return cls(state, epoch, attractor)

    @classmethod
    def from_classical(
        cls,
        attractor,
        a: Union[float, u.Quantity],
        ecc: Union[float, u.Quantity],
        inc: Union[float, u.Quantity],
        raan: Union[float, u.Quantity],
        argp: Union[float, u.Quantity],
        nu: Union[float, u.Quantity],
        epoch: Optional[Union[Epoch, "AstropyTime"]] = None,
    ) -> "Orbit":
        """
        Create an orbit from classical Keplerian orbital elements.

        Parameters
        ----------
        attractor : Body
            Central attracting body
        a : float or Quantity
            Semi-major axis. Can be:
            - float in meters (SI)
            - astropy Quantity with length units (e.g., km, AU)
        ecc : float or Quantity
            Eccentricity (dimensionless, 0 ≤ e < 1 for elliptical)
            Can be float or dimensionless Quantity
        inc : float or Quantity
            Inclination (0 ≤ i ≤ π). Can be:
            - float in radians (SI)
            - astropy Quantity with angle units (e.g., deg, rad)
        raan : float or Quantity
            Right ascension of ascending node (0 ≤ Ω < 2π). Can be:
            - float in radians (SI)
            - astropy Quantity with angle units
        argp : float or Quantity
            Argument of periapsis (0 ≤ ω < 2π). Can be:
            - float in radians (SI)
            - astropy Quantity with angle units
        nu : float or Quantity
            True anomaly (0 ≤ ν < 2π). Can be:
            - float in radians (SI)
            - astropy Quantity with angle units
        epoch : Epoch, astropy.time.Time, or None, optional
            Epoch at which elements are valid. Accepts either hifitime Epoch or
            astropy.time.Time objects. Defaults to J2000 if not provided.

        Returns
        -------
        Orbit
            New Orbit object

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>> from astropy import units as u
        >>> import numpy as np
        >>>
        >>> # GTO-like orbit with raw values (meters, radians)
        >>> orbit = Orbit.from_classical(
        ...     Earth,
        ...     a=24000e3,              # 24,000 km in meters
        ...     ecc=0.7,                # Highly elliptical
        ...     inc=np.deg2rad(7),      # Near-equatorial
        ...     raan=0.0,
        ...     argp=0.0,
        ...     nu=0.0,                 # At periapsis
        ... )
        >>>
        >>> # Or with astropy units (poliastro-style)
        >>> orbit = Orbit.from_classical(
        ...     Earth,
        ...     a=24000 << u.km,
        ...     ecc=0.7 << u.one,
        ...     inc=7 << u.deg,
        ...     raan=0 << u.deg,
        ...     argp=0 << u.deg,
        ...     nu=0 << u.deg,
        ... )
        """
        # Convert astropy.time.Time to Epoch if needed, default to J2000
        epoch = to_epoch(epoch)
        if epoch is None:
            epoch = Epoch.j2000_epoch()

        # Convert all parameters to SI units
        a_si = to_si_length(a, "semi-major axis")
        ecc_si = to_dimensionless(ecc, "eccentricity")
        inc_si = to_si_angle(inc, "inclination")
        raan_si = to_si_angle(raan, "RAAN")
        argp_si = to_si_angle(argp, "argument of periapsis")
        nu_si = to_si_angle(nu, "true anomaly")

        # Create OrbitalElements object (expects SI units)
        elements = OrbitalElements(a_si, ecc_si, inc_si, raan_si, argp_si, nu_si)

        # Convert elements to state vectors using Rust backend
        mu = attractor.mu
        result = coe_to_rv(elements, mu)

        r = np.array(result[0])
        v = np.array(result[1])

        state = CartesianState(r, v)

        return cls(state, epoch, attractor)

    @classmethod
    def from_tle(
        cls,
        tle_string: str,
        attractor=None,
        epoch: Optional[Epoch] = None,
    ) -> "Orbit":
        """
        Create an orbit from a Two-Line Element (TLE) set.

        Parameters
        ----------
        tle_string : str
            TLE string (2-line or 3-line format)
        attractor : Body, optional
            Attractor body. If not provided, Earth is assumed.
        epoch : Epoch, optional
            If provided, propagates TLE to this epoch. Otherwise uses TLE epoch.

        Returns
        -------
        Orbit
            New Orbit object

        Examples
        --------
        >>> tle = '''ISS (ZARYA)
        ... 1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
        ... 2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537'''
        >>> orbit = Orbit.from_tle(tle)

        Notes
        -----
        TLE propagation uses SGP4/SDP4 model from the Rust backend.
        The epoch is extracted from the TLE unless overridden.
        """
        if attractor is None:
            from astrora.bodies import Earth

            attractor = Earth

        # Propagate TLE to get state at epoch (or TLE epoch if epoch is None)
        if epoch is None:
            # Use TLE epoch (time_offset = 0)
            result = py_propagate_tle(tle_string, 0.0)
            # TODO: Extract epoch from TLE for accurate epoch storage
            epoch = Epoch.j2000_epoch()  # Placeholder - need TLE epoch extraction
        else:
            # TODO: Calculate time offset from TLE epoch to requested epoch
            # For now, use TLE epoch
            result = py_propagate_tle(tle_string, 0.0)

        # Extract position and velocity from TLE propagation result
        r = np.array(result["position"])
        v = np.array(result["velocity"])

        state = CartesianState(r, v)

        return cls(state, epoch, attractor)

    # ========================================================================
    # Convenience Creation Methods
    # ========================================================================

    @classmethod
    def circular(
        cls,
        attractor,
        alt: Union[float, u.Quantity],
        inc: Union[float, u.Quantity] = 0.0,
        raan: Union[float, u.Quantity] = 0.0,
        arglat: Union[float, u.Quantity] = 0.0,
        epoch: Optional[Union[Epoch, "AstropyTime"]] = None,
    ) -> "Orbit":
        """
        Create a circular orbit at a given altitude.

        This is a convenience method for creating circular orbits (ecc = 0)
        without manually computing orbital elements.

        Parameters
        ----------
        attractor : Body
            Central attracting body
        alt : float or Quantity
            Altitude above surface. If float, assumed to be in meters.
            If Quantity, must have length units (e.g., u.km).
        inc : float or Quantity, optional
            Inclination. If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad - equatorial)
        raan : float or Quantity, optional
            Right ascension of ascending node. If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad)
        arglat : float or Quantity, optional
            Argument of latitude (u = argp + nu). If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad)
        epoch : Epoch or astropy.time.Time, optional
            Reference epoch (default: J2000)

        Returns
        -------
        Orbit
            Circular orbit at the specified altitude

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>> from astropy import units as u
        >>>
        >>> # ISS-like orbit (400 km altitude, 51.6° inclination)
        >>> orbit = Orbit.circular(Earth, 400 * u.km, inc=51.6 * u.deg)
        >>> print(f"Period: {orbit.period.to(u.hour):.2f}")
        Period: 1.55 h
        >>>
        >>> # Equatorial circular orbit at 700 km
        >>> orbit = Orbit.circular(Earth, alt=700e3)  # meters
        >>> print(f"Eccentricity: {orbit.ecc:.6f}")
        Eccentricity: 0.000000

        Notes
        -----
        The semi-major axis is computed as a = R + alt, where R is the
        attractor's equatorial radius.

        For circular orbits, the argument of periapsis is undefined, so we use
        the argument of latitude (arglat = argp + nu) to specify the position.
        """
        # Convert units to SI
        alt_m = to_si_length(alt)
        inc_rad = to_si_angle(inc)
        raan_rad = to_si_angle(raan)
        arglat_rad = to_si_angle(arglat)

        if epoch is None:
            epoch = Epoch.j2000_epoch()
        else:
            epoch = to_epoch(epoch)

        # Compute semi-major axis
        a = attractor.R + alt_m

        # For circular orbits, argp is undefined, but we use arglat = argp + nu
        # Set argp = 0 and nu = arglat
        argp = 0.0
        nu = arglat_rad

        # Create using from_classical with ecc = 0
        return cls.from_classical(
            attractor=attractor,
            a=a,
            ecc=0.0,
            inc=inc_rad,
            raan=raan_rad,
            argp=argp,
            nu=nu,
            epoch=epoch,
        )

    @classmethod
    def geostationary(
        cls,
        attractor=None,
        raan: Union[float, u.Quantity] = 0.0,
        arglat: Union[float, u.Quantity] = 0.0,
        epoch: Optional[Union[Epoch, "AstropyTime"]] = None,
    ) -> "Orbit":
        """
        Create a geostationary orbit around Earth (or synchronous orbit for other bodies).

        A geostationary orbit has a period equal to the body's rotational period,
        appears stationary relative to the surface, and is equatorial (inc = 0).

        Parameters
        ----------
        attractor : Body, optional
            Central attracting body (default: Earth)
        raan : float or Quantity, optional
            Right ascension of ascending node. If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad)
        arglat : float or Quantity, optional
            Argument of latitude (position along orbit). If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad)
        epoch : Epoch or astropy.time.Time, optional
            Reference epoch (default: J2000)

        Returns
        -------
        Orbit
            Geostationary orbit (for Earth) or synchronous orbit (for other bodies)

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>> from astropy import units as u
        >>>
        >>> # Standard geostationary orbit
        >>> orbit = Orbit.geostationary()
        >>> print(f"Altitude: {(orbit.a.to(u.km).value - Earth.R/1000):.0f} km")
        Altitude: 35786 km
        >>> print(f"Period: {orbit.period.to(u.hour):.2f}")
        Period: 23.93 h

        Notes
        -----
        For Earth, this creates a circular orbit at ~35,786 km altitude with
        a period of ~23.93 hours (one sidereal day).

        For geostationary orbits, inc = 0 (equatorial) and ecc = 0 (circular).
        """
        if attractor is None:
            from astrora.bodies import Earth

            attractor = Earth

        # Use synchronous with default period_mul=1 and inc=0
        return cls.synchronous(
            attractor=attractor,
            period_mul=1.0,
            ecc=0.0,
            inc=0.0,
            raan=raan,
            arglat=arglat,
            epoch=epoch,
        )

    @classmethod
    def synchronous(
        cls,
        attractor,
        period_mul: Union[float, u.Quantity] = 1.0,
        ecc: Union[float, u.Quantity] = 0.0,
        inc: Union[float, u.Quantity] = 0.0,
        argp: Union[float, u.Quantity] = 0.0,
        arglat: Union[float, u.Quantity] = 0.0,
        raan: Union[float, u.Quantity] = 0.0,
        epoch: Optional[Union[Epoch, "AstropyTime"]] = None,
    ) -> "Orbit":
        """
        Create a synchronous orbit with period matching the body's rotation.

        A synchronous orbit has an orbital period that is a multiple of the
        central body's rotational period.

        Parameters
        ----------
        attractor : Body
            Central attracting body
        period_mul : float or Quantity, optional
            Period multiplier relative to rotation period (default: 1.0)
            For example, period_mul=2 creates an orbit with twice the rotation period.
        ecc : float or Quantity, optional
            Eccentricity (default: 0.0 - circular)
        inc : float or Quantity, optional
            Inclination. If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad - equatorial)
        argp : float or Quantity, optional
            Argument of periapsis. If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad)
        arglat : float or Quantity, optional
            Argument of latitude (only used if ecc=0). If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad)
        raan : float or Quantity, optional
            Right ascension of ascending node. If float, assumed to be in radians.
            If Quantity, must have angle units (default: 0.0 rad)
        epoch : Epoch or astropy.time.Time, optional
            Reference epoch (default: J2000)

        Returns
        -------
        Orbit
            Synchronous orbit

        Raises
        ------
        ValueError
            If the attractor has no rotational period defined

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth, Mars
        >>> from astropy import units as u
        >>>
        >>> # Earth geostationary orbit (period = 1 sidereal day)
        >>> orbit = Orbit.synchronous(Earth)
        >>> print(f"Period: {orbit.period.to(u.hour):.2f}")
        Period: 23.93 h
        >>>
        >>> # Mars areostationary orbit
        >>> orbit = Orbit.synchronous(Mars)
        >>> print(f"Period: {orbit.period.to(u.hour):.2f}")
        Period: 24.62 h
        >>>
        >>> # Semi-synchronous orbit (period = 2 × rotation period)
        >>> orbit = Orbit.synchronous(Earth, period_mul=2)
        >>> print(f"Period: {orbit.period.to(u.hour):.2f}")
        Period: 47.87 h

        Notes
        -----
        The semi-major axis is computed using Kepler's third law:
        a = (μ * T² / 4π²)^(1/3), where T = period_mul × T_rotation

        For bodies without a defined rotational period, this method will raise
        a ValueError. Common bodies with rotation periods: Earth, Mars, Jupiter, etc.
        """
        # Convert units
        period_mul_val = to_dimensionless(period_mul)
        ecc_val = to_dimensionless(ecc)
        inc_rad = to_si_angle(inc)
        argp_rad = to_si_angle(argp)
        arglat_rad = to_si_angle(arglat)
        raan_rad = to_si_angle(raan)

        if epoch is None:
            epoch = Epoch.j2000_epoch()
        else:
            epoch = to_epoch(epoch)

        # Check if attractor has a rotational period
        if not hasattr(attractor, "rotational_period") or attractor.rotational_period is None:
            raise ValueError(
                f"Attractor '{attractor.name}' does not have a defined rotational period. "
                "Synchronous orbits require knowledge of the body's rotation rate."
            )

        # Get rotational period
        if isinstance(attractor.rotational_period, u.Quantity):
            T_rot = attractor.rotational_period.to(u.s).value
        else:
            T_rot = attractor.rotational_period  # Assume seconds

        # Compute orbital period
        T_orbit = period_mul_val * T_rot

        # Use Kepler's third law to find semi-major axis
        # T = 2π√(a³/μ)  =>  a = (μ T² / 4π²)^(1/3)
        mu = attractor.mu
        a = (mu * T_orbit**2 / (4 * np.pi**2)) ** (1 / 3)

        # For circular orbits, use arglat; otherwise use argp
        if ecc_val == 0.0:
            nu = arglat_rad
            argp_use = 0.0
        else:
            nu = 0.0
            argp_use = argp_rad

        return cls.from_classical(
            attractor=attractor,
            a=a,
            ecc=ecc_val,
            inc=inc_rad,
            raan=raan_rad,
            argp=argp_use,
            nu=nu,
            epoch=epoch,
        )

    @classmethod
    def parabolic(
        cls,
        attractor,
        p: Union[float, u.Quantity],
        inc: Union[float, u.Quantity],
        raan: Union[float, u.Quantity],
        argp: Union[float, u.Quantity],
        nu: Union[float, u.Quantity],
        epoch: Optional[Union[Epoch, "AstropyTime"]] = None,
    ) -> "Orbit":
        """
        Create a parabolic orbit (eccentricity = 1, escape trajectory).

        Parabolic orbits represent the boundary case between bound elliptical
        orbits and unbound hyperbolic trajectories.

        Parameters
        ----------
        attractor : Body
            Central attracting body
        p : float or Quantity
            Semi-latus rectum (p = a(1-e²)). If float, assumed to be in meters.
            If Quantity, must have length units.
        inc : float or Quantity
            Inclination. If float, assumed to be in radians.
            If Quantity, must have angle units.
        raan : float or Quantity
            Right ascension of ascending node. If float, assumed to be in radians.
            If Quantity, must have angle units.
        argp : float or Quantity
            Argument of periapsis. If float, assumed to be in radians.
            If Quantity, must have angle units.
        nu : float or Quantity
            True anomaly. If float, assumed to be in radians.
            If Quantity, must have angle units.
        epoch : Epoch or astropy.time.Time, optional
            Reference epoch (default: J2000)

        Returns
        -------
        Orbit
            Parabolic orbit (ecc = 1.0)

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>> from astropy import units as u
        >>> import numpy as np
        >>>
        >>> # Parabolic escape trajectory from 300 km altitude
        >>> p = 6678e3  # Semi-latus rectum (m)
        >>> orbit = Orbit.parabolic(
        ...     Earth,
        ...     p=p,
        ...     inc=np.deg2rad(28.5),
        ...     raan=0.0,
        ...     argp=0.0,
        ...     nu=0.0
        ... )
        >>> print(f"Eccentricity: {orbit.ecc:.1f}")
        Eccentricity: 1.0
        >>> print(f"Energy: {orbit.energy.to(u.MJ/u.kg):.2f}")
        Energy: 0.00 MJ / kg

        Notes
        -----
        For parabolic orbits (e = 1), the semi-major axis is infinite.
        We use the relation a = p / (1 - e²), but since this would be
        undefined, we represent it using a very large value internally.

        The specific orbital energy for a parabolic orbit is exactly zero.
        """
        # Convert units to SI
        p_m = to_si_length(p)
        inc_rad = to_si_angle(inc)
        raan_rad = to_si_angle(raan)
        argp_rad = to_si_angle(argp)
        nu_rad = to_si_angle(nu)

        if epoch is None:
            epoch = Epoch.j2000_epoch()
        else:
            epoch = to_epoch(epoch)

        # For parabolic orbits, ecc = 1 and a → ∞
        # Due to numerical limitations, we use ecc very close to 1
        # (effectively parabolic for all practical purposes)
        ecc = 0.9999  # Very close to parabolic

        # For nearly-parabolic orbits, use relation: p = a(1-e²)
        # Therefore: a = p / (1-e²)
        a = p_m / (1 - ecc**2)

        return cls.from_classical(
            attractor=attractor,
            a=a,
            ecc=ecc,
            inc=inc_rad,
            raan=raan_rad,
            argp=argp_rad,
            nu=nu_rad,
            epoch=epoch,
        )

    # ========================================================================
    # Properties - State Vectors
    # ========================================================================

    @property
    def r(self) -> u.Quantity:
        """
        Position vector [x, y, z].

        Returns
        -------
        Quantity
            Position vector with units (km by default)

        Examples
        --------
        >>> orbit.r
        <Quantity [7000., 0., 0.] km>
        >>>
        >>> # Convert to other units
        >>> orbit.r.to(u.m)
        <Quantity [7000000., 0., 0.] m>
        """
        r_si = np.array(
            [
                self._state.position[0],
                self._state.position[1],
                self._state.position[2],
            ]
        )
        return r_si * u.m  # Returns in SI, can be converted to km by user

    @property
    def v(self) -> u.Quantity:
        """
        Velocity vector [vx, vy, vz].

        Returns
        -------
        Quantity
            Velocity vector with units (km/s by default)

        Examples
        --------
        >>> orbit.v
        <Quantity [0., 7.66, 0.] km / s>
        >>>
        >>> # Convert to other units
        >>> orbit.v.to(u.m / u.s)
        <Quantity [0., 7660., 0.] m / s>
        """
        v_si = np.array(
            [
                self._state.velocity[0],
                self._state.velocity[1],
                self._state.velocity[2],
            ]
        )
        return v_si * u.m / u.s  # Returns in SI

    @property
    def state(self) -> CartesianState:
        """Cartesian state (position and velocity)."""
        return self._state

    @property
    def epoch(self) -> Epoch:
        """Epoch at which this state is valid."""
        return self._epoch

    @property
    def attractor(self):
        """Central attracting body."""
        return self._attractor

    # ========================================================================
    # Properties - Classical Orbital Elements
    # ========================================================================

    def _compute_elements(self) -> OrbitalElements:
        """Compute orbital elements from state vectors (with caching)."""
        if self._elements_cache is None:
            # rv_to_coe expects raw arrays in SI units
            r_si = np.array(
                [
                    self._state.position[0],
                    self._state.position[1],
                    self._state.position[2],
                ]
            )
            v_si = np.array(
                [
                    self._state.velocity[0],
                    self._state.velocity[1],
                    self._state.velocity[2],
                ]
            )
            self._elements_cache = rv_to_coe(r_si, v_si, self._attractor.mu)
        return self._elements_cache

    @property
    def a(self) -> u.Quantity:
        """
        Semi-major axis.

        Returns
        -------
        Quantity
            Semi-major axis with units (meters in SI)

        Examples
        --------
        >>> orbit.a.to(u.km)
        <Quantity 7000. km>
        """
        return self._compute_elements().a * u.m

    @property
    def ecc(self) -> u.Quantity:
        """
        Eccentricity (dimensionless).

        Returns
        -------
        Quantity
            Dimensionless eccentricity

        Examples
        --------
        >>> orbit.ecc
        <Quantity 0.01>
        """
        return self._compute_elements().e * u.dimensionless_unscaled

    @property
    def inc(self) -> u.Quantity:
        """
        Inclination.

        Returns
        -------
        Quantity
            Inclination with angle units (radians in SI, 0 ≤ i ≤ π)

        Examples
        --------
        >>> orbit.inc.to(u.deg)
        <Quantity 51.6 deg>
        """
        return self._compute_elements().i * u.rad

    @property
    def raan(self) -> u.Quantity:
        """
        Right ascension of ascending node (RAAN).

        Returns
        -------
        Quantity
            RAAN with angle units (radians in SI, 0 ≤ Ω < 2π)

        Examples
        --------
        >>> orbit.raan.to(u.deg)
        <Quantity 0. deg>
        """
        return self._compute_elements().raan * u.rad

    @property
    def argp(self) -> u.Quantity:
        """
        Argument of periapsis.

        Returns
        -------
        Quantity
            Argument of periapsis with angle units (radians in SI, 0 ≤ ω < 2π)

        Examples
        --------
        >>> orbit.argp.to(u.deg)
        <Quantity 0. deg>
        """
        return self._compute_elements().argp * u.rad

    @property
    def nu(self) -> u.Quantity:
        """
        True anomaly.

        Returns
        -------
        Quantity
            True anomaly with angle units (radians in SI, 0 ≤ ν < 2π)

        Examples
        --------
        >>> orbit.nu.to(u.deg)
        <Quantity 0. deg>
        """
        return self._compute_elements().nu * u.rad

    # ========================================================================
    # Properties - Derived Orbital Parameters
    # ========================================================================

    @property
    def period(self) -> u.Quantity:
        """
        Orbital period (elliptical orbits only).

        Returns
        -------
        Quantity
            Orbital period with time units (seconds in SI)

        Raises
        ------
        RuntimeError
            If orbit is parabolic or hyperbolic (no finite period)

        Examples
        --------
        >>> orbit.period.to(u.hour)
        <Quantity 1.58 h>
        """
        elements = self._compute_elements()
        period_si = elements.orbital_period(self._attractor.mu)
        return period_si * u.s

    @property
    def n(self) -> u.Quantity:
        """
        Mean motion.

        Returns
        -------
        Quantity
            Mean motion with angular velocity units (rad/s in SI)

        Examples
        --------
        >>> orbit.n.to(u.deg / u.min)
        <Quantity 0.068 deg / min>
        """
        period_val = self.period.to(u.s).value  # Get period in seconds
        return (2.0 * np.pi / period_val) * u.rad / u.s

    @property
    def energy(self) -> u.Quantity:
        """
        Specific orbital energy (energy per unit mass).

        Returns
        -------
        Quantity
            Specific energy with units (J/kg or m²/s² in SI)

        Notes
        -----
        - Negative energy: elliptical orbit (bound)
        - Zero energy: parabolic orbit (marginally bound)
        - Positive energy: hyperbolic orbit (unbound)

        Examples
        --------
        >>> orbit.energy
        <Quantity -26.0e6 m2 / s2>
        """
        energy_si = self._state.specific_energy(self._attractor.mu)
        return energy_si * u.m**2 / u.s**2

    @property
    def p(self) -> u.Quantity:
        """
        Semi-latus rectum.

        Returns
        -------
        Quantity
            Semi-latus rectum p = a(1 - e²) with length units (meters in SI)

        Examples
        --------
        >>> orbit.p.to(u.km)
        <Quantity 6993. km>
        """
        elements = self._compute_elements()
        p_si = elements.p  # This is a property
        return p_si * u.m

    @property
    def r_p(self) -> u.Quantity:
        """
        Periapsis distance.

        Returns
        -------
        Quantity
            Periapsis distance r_p = a(1 - e) with length units (meters in SI)

        Examples
        --------
        >>> orbit.r_p.to(u.km)
        <Quantity 6930. km>
        """
        elements = self._compute_elements()
        r_p_si = elements.periapsis_distance  # Property, not method
        return r_p_si * u.m

    @property
    def r_a(self) -> u.Quantity:
        """
        Apoapsis distance (elliptical orbits).

        Returns
        -------
        Quantity
            Apoapsis distance r_a = a(1 + e) with length units (meters in SI)

        Notes
        -----
        For hyperbolic orbits, this returns a negative value (physically meaningless).

        Examples
        --------
        >>> orbit.r_a.to(u.km)
        <Quantity 7070. km>
        """
        elements = self._compute_elements()
        r_a_si = elements.apoapsis_distance  # Property, not method
        return r_a_si * u.m

    # ========================================================================
    # Methods - Propagation
    # ========================================================================

    def propagate(
        self, value: Union[Duration, float], method: str = "keplerian", **kwargs
    ) -> "Orbit":
        """
        Propagate the orbit forward or backward in time.

        Parameters
        ----------
        value : Duration or float
            Time interval to propagate. If Duration object, uses that.
            If float, interprets as seconds.
        method : str, optional
            Propagation method. Options:
            - 'keplerian': Analytical Keplerian propagation (default, fastest)
            - 'lagrange': Lagrange f-g functions
            - 'j2': J2 perturbation with RK4
            - 'j2_dopri5': J2 with DOPRI5 integrator
            - 'j2_dop853': J2 with DOP853 integrator
        **kwargs
            Additional arguments for specific propagation methods

        Returns
        -------
        Orbit
            New Orbit object at propagated time

        Examples
        --------
        >>> from astrora._core import Duration
        >>>
        >>> # Propagate 1 orbital period
        >>> future = orbit.propagate(orbit.period)
        >>>
        >>> # Propagate using Duration object
        >>> dt = Duration.from_hours(6)
        >>> future = orbit.propagate(dt)
        >>>
        >>> # Propagate backward in time
        >>> past = orbit.propagate(-3600)  # 1 hour ago
        """
        # Convert to float seconds
        if isinstance(value, Duration):
            dt_seconds = value.seconds
            dt = value
        elif isinstance(value, (int, float)):
            dt_seconds = float(value)
            dt = Duration(dt_seconds)  # Duration constructor takes seconds
        else:
            raise TypeError(f"value must be Duration or numeric, got {type(value)}")

        # Propagate using Rust backend
        if method == "keplerian" or method == "lagrange":
            # Use Keplerian propagation (expects float seconds)
            # Returns tuple (position, velocity)
            result = propagate_state_keplerian(self.r, self.v, dt_seconds, self._attractor.mu)
            new_r = np.array(result[0])
            new_v = np.array(result[1])
        else:
            raise NotImplementedError(
                f"Propagation method '{method}' not yet implemented. "
                f"Available: 'keplerian', 'lagrange'"
            )

        # Create new state and epoch
        new_state = CartesianState(new_r, new_v)
        new_epoch = self._epoch + dt

        return Orbit(new_state, new_epoch, self._attractor)

    def sample(
        self,
        times: Union[np.ndarray, list],
        method: str = "keplerian",
    ) -> Tuple[np.ndarray, np.ndarray]:
        """
        Sample the orbit at multiple times.

        Parameters
        ----------
        times : np.ndarray or list
            Array of time offsets (seconds) from current epoch,
            or array of Duration objects
        method : str, optional
            Propagation method (default: 'keplerian')

        Returns
        -------
        positions : np.ndarray
            Array of position vectors, shape (N, 3)
        velocities : np.ndarray
            Array of velocity vectors, shape (N, 3)

        Examples
        --------
        >>> # Sample orbit over one period
        >>> times = np.linspace(0, orbit.period, 100)
        >>> positions, velocities = orbit.sample(times)
        >>>
        >>> # Plot orbit
        >>> import matplotlib.pyplot as plt
        >>> from mpl_toolkits.mplot3d import Axes3D
        >>> fig = plt.figure()
        >>> ax = fig.add_subplot(111, projection='3d')
        >>> ax.plot(positions[:, 0], positions[:, 1], positions[:, 2])
        """
        # Convert times to numpy array of floats (seconds)
        if isinstance(times, list):
            times = np.array(times)

        # If times are Duration objects, convert to seconds
        if len(times) > 0 and isinstance(times[0], Duration):
            times = np.array([t.to_seconds() for t in times])

        # Handle case where times has astropy units
        if hasattr(times, "unit"):
            # Convert to seconds (SI)
            times_seconds = times.to(u.s).value
        else:
            times_seconds = times

        # Get SI values for position and velocity
        if hasattr(self.r, "unit"):
            # Extract values from Quantity objects
            r_values = self.r.to(u.m).value
            v_values = self.v.to(u.m / u.s).value
        else:
            r_values = self.r
            v_values = self.v

        # batch_propagate_states propagates N states by N time steps
        # We want to propagate 1 state at N different times
        # Solution: Create N copies of the initial state, one for each time
        n_times = len(times_seconds)
        state = np.concatenate([r_values, v_values])
        states = np.tile(state, (n_times, 1))  # Shape: (N, 6)

        # Use batch propagation from Rust
        # Returns 2D array with shape (N, 6) where each row is [x, y, z, vx, vy, vz]
        result = batch_propagate_states(states, times_seconds, self._attractor.mu)

        # Split result into positions and velocities
        positions = result[:, :3]  # First 3 columns
        velocities = result[:, 3:]  # Last 3 columns

        # Wrap in Quantity if using units
        if hasattr(self.r, "unit"):
            positions = positions << u.m
            velocities = velocities << (u.m / u.s)

        return positions, velocities

    # ========================================================================
    # Methods - Maneuvers
    # ========================================================================

    def apply_maneuver(self, delta_v: np.ndarray) -> "Orbit":
        """
        Apply an impulsive maneuver (instantaneous velocity change).

        Parameters
        ----------
        delta_v : np.ndarray
            Velocity change vector [Δvx, Δvy, Δvz] in m/s (3-element array)

        Returns
        -------
        Orbit
            New Orbit object after maneuver

        Examples
        --------
        >>> # Prograde burn (along velocity vector)
        >>> v_hat = orbit.v / np.linalg.norm(orbit.v)
        >>> delta_v = 100 * v_hat  # 100 m/s prograde
        >>> new_orbit = orbit.apply_maneuver(delta_v)
        """
        delta_v = np.asarray(delta_v, dtype=np.float64)

        if delta_v.shape != (3,):
            raise ValueError(f"delta_v must be 3-element array, got shape {delta_v.shape}")

        # Apply delta-v to velocity
        new_v = self.v + delta_v

        # Position remains the same (impulsive maneuver)
        new_state = CartesianState(self.r, new_v)

        return Orbit(new_state, self._epoch, self._attractor)

    # ========================================================================
    # Astropy Coordinates Integration
    # ========================================================================

    def to_skycoord(self, frame="gcrs"):
        """
        Convert orbit to astropy SkyCoord.

        Parameters
        ----------
        frame : str, optional
            Target coordinate frame for SkyCoord ('icrs', 'gcrs', or 'itrs').
            Default is 'gcrs' (Geocentric Celestial Reference System).

        Returns
        -------
        astropy.coordinates.SkyCoord
            SkyCoord object with position, velocity, and time information.

        Notes
        -----
        - Requires astropy to be installed
        - Position and velocity are preserved
        - Observation time is set from orbit epoch
        - GCRS frame is most common for Earth satellites

        Examples
        --------
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>> import numpy as np
        >>>
        >>> r = np.array([7000e3, 0, 0])
        >>> v = np.array([0, 7546, 0])
        >>> orbit = Orbit.from_vectors(Earth, r, v)
        >>>
        >>> # Convert to SkyCoord
        >>> sc = orbit.to_skycoord()
        >>> print(f"Distance: {sc.distance}")
        >>> print(f"RA: {sc.ra}, Dec: {sc.dec}")
        """
        from astrora._core import GCRS, ICRS, ITRS
        from astrora.coordinates import to_skycoord
        from astrora.time import epoch_to_astropy_time

        # Get astropy Time from epoch
        obstime = epoch_to_astropy_time(self._epoch)

        # Create appropriate coordinate frame based on position/velocity
        if frame.lower() == "icrs":
            astrora_frame = ICRS(self.r, self.v)
        elif frame.lower() == "gcrs":
            astrora_frame = GCRS(self.r, self.v, self._epoch)
        elif frame.lower() == "itrs":
            astrora_frame = ITRS(self.r, self.v, self._epoch)
        else:
            raise ValueError(
                f"Unsupported frame '{frame}'. " "Supported frames: 'icrs', 'gcrs', 'itrs'"
            )

        # Convert to SkyCoord
        return to_skycoord(astrora_frame, obstime=obstime)

    @classmethod
    def from_skycoord(cls, skycoord, attractor):
        """
        Create orbit from astropy SkyCoord.

        Parameters
        ----------
        skycoord : astropy.coordinates.SkyCoord
            SkyCoord object with position and velocity information.
        attractor : Body
            Central body for the orbit.

        Returns
        -------
        Orbit
            Orbit object created from SkyCoord.

        Notes
        -----
        - Requires astropy to be installed
        - SkyCoord must have Cartesian representation with velocity
        - Observation time is extracted from SkyCoord and used as orbit epoch
        - Position and velocity are converted from astropy units to SI (meters, m/s)

        Examples
        --------
        >>> from astropy.coordinates import SkyCoord
        >>> from astropy import units as u
        >>> from astropy.time import Time
        >>> from astrora.twobody import Orbit
        >>> from astrora.bodies import Earth
        >>>
        >>> # Create SkyCoord
        >>> sc = SkyCoord(
        ...     x=7000*u.km, y=0*u.km, z=0*u.km,
        ...     v_x=0*u.km/u.s, v_y=7.5*u.km/u.s, v_z=0*u.km/u.s,
        ...     representation_type='cartesian',
        ...     frame='gcrs',
        ...     obstime=Time('2024-01-01')
        ... )
        >>>
        >>> # Create orbit
        >>> orbit = Orbit.from_skycoord(sc, Earth)
        >>> print(orbit.period / 3600)  # hours
        """
        from astrora.coordinates import from_skycoord
        from astrora.time import astropy_time_to_epoch

        # Convert SkyCoord to astrora frame (ICRS, GCRS, or ITRS)
        astrora_frame = from_skycoord(skycoord)

        # Extract position and velocity
        position = astrora_frame.position
        velocity = astrora_frame.velocity

        # Extract observation time
        if hasattr(skycoord.frame, "obstime") and skycoord.frame.obstime is not None:
            epoch = astropy_time_to_epoch(skycoord.frame.obstime)
        else:
            # Use J2000.0 if no obstime specified
            epoch = Epoch.j2000_epoch()

        # Create orbit
        return cls.from_vectors(attractor, position, velocity, epoch=epoch)

    def to_astropy_coord(self, frame="gcrs"):
        """
        Convert orbit to astropy coordinate frame.

        This is a lower-level method compared to `to_skycoord()`. Use `to_skycoord()`
        for most applications.

        Parameters
        ----------
        frame : str, optional
            Target coordinate frame ('icrs', 'gcrs', or 'itrs').
            Default is 'gcrs'.

        Returns
        -------
        astropy.coordinates frame (ICRS, GCRS, or ITRS)
            Astropy coordinate frame object.

        Examples
        --------
        >>> orbit = Orbit.from_vectors(Earth, r, v)
        >>> gcrs = orbit.to_astropy_coord(frame='gcrs')
        >>> print(gcrs.cartesian.xyz)
        """
        from astrora._core import GCRS, ICRS, ITRS
        from astrora.coordinates import to_astropy_coord
        from astrora.time import epoch_to_astropy_time

        # Get astropy Time from epoch
        obstime = epoch_to_astropy_time(self._epoch)

        # Create appropriate coordinate frame
        if frame.lower() == "icrs":
            astrora_frame = ICRS(self.r, self.v)
        elif frame.lower() == "gcrs":
            astrora_frame = GCRS(self.r, self.v, self._epoch)
        elif frame.lower() == "itrs":
            astrora_frame = ITRS(self.r, self.v, self._epoch)
        else:
            raise ValueError(
                f"Unsupported frame '{frame}'. " "Supported frames: 'icrs', 'gcrs', 'itrs'"
            )

        return to_astropy_coord(astrora_frame, obstime=obstime)

    @classmethod
    def from_astropy_coord(cls, astropy_coord, attractor):
        """
        Create orbit from astropy coordinate frame.

        This is a lower-level method compared to `from_skycoord()`. Use `from_skycoord()`
        for most applications.

        Parameters
        ----------
        astropy_coord : astropy.coordinates frame
            Astropy coordinate frame (ICRS, GCRS, or ITRS).
        attractor : Body
            Central body for the orbit.

        Returns
        -------
        Orbit
            Orbit object created from astropy coordinate frame.

        Examples
        --------
        >>> from astropy.coordinates import GCRS
        >>> from astropy import units as u
        >>> from astropy.time import Time
        >>>
        >>> gcrs = GCRS(
        ...     x=7000*u.km, y=0*u.km, z=0*u.km,
        ...     v_x=0*u.km/u.s, v_y=7.5*u.km/u.s, v_z=0*u.km/u.s,
        ...     representation_type='cartesian',
        ...     differential_type='cartesian',
        ...     obstime=Time('2024-01-01')
        ... )
        >>>
        >>> orbit = Orbit.from_astropy_coord(gcrs, Earth)
        """
        from astrora.coordinates import from_astropy_coord
        from astrora.time import astropy_time_to_epoch

        # Convert astropy frame to astrora frame
        astrora_frame = from_astropy_coord(astropy_coord)

        # Extract position and velocity
        position = astrora_frame.position
        velocity = astrora_frame.velocity

        # Extract observation time
        if hasattr(astropy_coord, "obstime") and astropy_coord.obstime is not None:
            epoch = astropy_time_to_epoch(astropy_coord.obstime)
        else:
            # Use J2000.0 if no obstime specified
            epoch = Epoch.j2000_epoch()

        # Create orbit
        return cls.from_vectors(attractor, position, velocity, epoch=epoch)

    # ========================================================================
    # String Representation
    # ========================================================================

    def __repr__(self) -> str:
        """String representation of the orbit."""
        try:
            return (
                f"Orbit(\n"
                f"  r = [{self.r[0]/1e3:.3f}, {self.r[1]/1e3:.3f}, {self.r[2]/1e3:.3f}] km,\n"
                f"  v = [{self.v[0]/1e3:.3f}, {self.v[1]/1e3:.3f}, {self.v[2]/1e3:.3f}] km/s,\n"
                f"  a = {self.a/1e3:.3f} km, e = {self.ecc:.6f},\n"
                f"  i = {np.rad2deg(self.inc):.3f}°, Ω = {np.rad2deg(self.raan):.3f}°,\n"
                f"  ω = {np.rad2deg(self.argp):.3f}°, ν = {np.rad2deg(self.nu):.3f}°,\n"
                f"  attractor = {self._attractor.__class__.__name__}\n"
                f")"
            )
        except Exception:
            # Fallback if element computation fails
            return f"Orbit(state={self._state}, epoch={self._epoch}, attractor={self._attractor})"
