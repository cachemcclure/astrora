"""
Celestial bodies and their properties.

This module provides definitions for planets, moons, and other celestial bodies
with their physical and gravitational parameters.
"""

from astrora import _core


class Body:
    """
    Celestial body with physical and gravitational properties.

    Attributes
    ----------
    name : str
        Name of the body (e.g., "Earth", "Sun")
    mu : float
        Standard gravitational parameter GM (m³/s²)
    R : float
        Equatorial radius (m)
    R_polar : float
        Polar radius (m), defaults to R if not provided
    R_mean : float
        Mean radius (m), defaults to R if not provided
    J2 : float, optional
        J2 gravitational coefficient (dimensionless)
    J3 : float, optional
        J3 gravitational coefficient (dimensionless)
    rotational_period : float, optional
        Rotational period in seconds (for synchronous orbit calculations)

    Examples
    --------
    >>> from astrora.bodies import Earth
    >>> print(f"Earth's GM: {Earth.mu:.3e} m³/s²")
    >>> print(f"Earth's radius: {Earth.R / 1e3:.1f} km")
    >>> print(f"Earth's rotational period: {Earth.rotational_period / 3600:.2f} hours")
    """

    def __init__(
        self,
        name: str,
        mu: float,
        R: float,
        R_polar: float = None,
        R_mean: float = None,
        J2: float = None,
        J3: float = None,
        rotational_period: float = None,
    ):
        """
        Initialize a celestial body.

        Parameters
        ----------
        name : str
            Body name
        mu : float
            Standard gravitational parameter GM (m³/s²)
        R : float
            Equatorial radius (m)
        R_polar : float, optional
            Polar radius (m)
        R_mean : float, optional
            Mean radius (m)
        J2 : float, optional
            J2 gravitational coefficient
        J3 : float, optional
            J3 gravitational coefficient
        rotational_period : float, optional
            Rotational period in seconds (sidereal rotation period)
        """
        self.name = name
        self.mu = mu
        self.R = R
        self.R_polar = R_polar if R_polar is not None else R
        self.R_mean = R_mean if R_mean is not None else R
        self.J2 = J2
        self.J3 = J3
        self.rotational_period = rotational_period

    def __repr__(self):
        return f"Body({self.name}, R={self.R/1e3:.1f} km, μ={self.mu:.3e} m³/s²)"


# =============================================================================
# Solar System Bodies
# =============================================================================

Sun = Body(
    name="Sun",
    mu=_core.constants.GM_SUN,
    R=_core.constants.R_SUN,
    R_polar=_core.constants.R_SUN,  # Sun is assumed spherical
    R_mean=_core.constants.R_MEAN_SUN,
    J2=_core.constants.J2_SUN,
    rotational_period=2192832.0,  # 25.38 days at equator (differential rotation)
)

Mercury = Body(
    name="Mercury",
    mu=_core.constants.GM_MERCURY,
    R=_core.constants.R_MERCURY,
    R_polar=_core.constants.R_POLAR_MERCURY,
    R_mean=_core.constants.R_MEAN_MERCURY,
)

Venus = Body(
    name="Venus",
    mu=_core.constants.GM_VENUS,
    R=_core.constants.R_VENUS,
    R_polar=_core.constants.R_POLAR_VENUS,
    R_mean=_core.constants.R_MEAN_VENUS,
    J2=_core.constants.J2_VENUS,
    J3=_core.constants.J3_VENUS,
)

Earth = Body(
    name="Earth",
    mu=_core.constants.GM_EARTH,
    R=_core.constants.R_EARTH,
    R_polar=_core.constants.R_POLAR_EARTH,
    R_mean=_core.constants.R_MEAN_EARTH,
    J2=_core.constants.J2_EARTH,
    J3=_core.constants.J3_EARTH,
    rotational_period=86164.0905,  # Sidereal day: 23h 56m 4.0905s
)

Moon = Body(
    name="Moon",
    mu=_core.constants.GM_MOON,
    R=_core.constants.R_MOON,
    R_polar=_core.constants.R_POLAR_MOON,
    R_mean=_core.constants.R_MEAN_MOON,
)

Mars = Body(
    name="Mars",
    mu=_core.constants.GM_MARS,
    R=_core.constants.R_MARS,
    R_polar=_core.constants.R_POLAR_MARS,
    R_mean=_core.constants.R_MEAN_MARS,
    J2=_core.constants.J2_MARS,
    rotational_period=88642.66,  # Martian sol: 24.6229 hours
)

Jupiter = Body(
    name="Jupiter",
    mu=_core.constants.GM_JUPITER,
    R=_core.constants.R_JUPITER,
    R_polar=_core.constants.R_POLAR_JUPITER,
    R_mean=_core.constants.R_MEAN_JUPITER,
    rotational_period=35730.0,  # 9.925 hours
)

Saturn = Body(
    name="Saturn",
    mu=_core.constants.GM_SATURN,
    R=_core.constants.R_SATURN,
    R_polar=_core.constants.R_POLAR_SATURN,
    R_mean=_core.constants.R_MEAN_SATURN,
    rotational_period=38361.6,  # 10.656 hours (equatorial)
)

Uranus = Body(
    name="Uranus",
    mu=_core.constants.GM_URANUS,
    R=_core.constants.R_URANUS,
    R_polar=_core.constants.R_POLAR_URANUS,
    R_mean=_core.constants.R_MEAN_URANUS,
)

Neptune = Body(
    name="Neptune",
    mu=_core.constants.GM_NEPTUNE,
    R=_core.constants.R_NEPTUNE,
    R_polar=_core.constants.R_POLAR_NEPTUNE,
    R_mean=_core.constants.R_MEAN_NEPTUNE,
)

Pluto = Body(
    name="Pluto",
    mu=_core.constants.GM_PLUTO,
    R=_core.constants.R_PLUTO,
    R_polar=_core.constants.R_PLUTO,  # Assumed spherical
    R_mean=_core.constants.R_MEAN_PLUTO,
)


# Export common bodies
__all__ = [
    "Body",
    "Sun",
    "Mercury",
    "Venus",
    "Earth",
    "Moon",
    "Mars",
    "Jupiter",
    "Saturn",
    "Uranus",
    "Neptune",
    "Pluto",
]
