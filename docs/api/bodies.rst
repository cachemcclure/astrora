Celestial Bodies
================

Physical and orbital data for celestial bodies in the solar system.

.. currentmodule:: astrora.bodies

Overview
--------

The bodies module provides pre-defined celestial body objects with accurate
physical parameters (mass, radius, gravitational parameter) and optional
atmospheric models.

Body Class
----------

.. autoclass:: astrora.bodies.Body
   :members:
   :undoc-members:
   :show-inheritance:
   :special-members: __init__

Available Bodies
---------------

Planets
~~~~~~~

.. autodata:: astrora.bodies.Sun
.. autodata:: astrora.bodies.Mercury
.. autodata:: astrora.bodies.Venus
.. autodata:: astrora.bodies.Earth
.. autodata:: astrora.bodies.Mars
.. autodata:: astrora.bodies.Jupiter
.. autodata:: astrora.bodies.Saturn
.. autodata:: astrora.bodies.Uranus
.. autodata:: astrora.bodies.Neptune

Earth's Moon
~~~~~~~~~~~

.. autodata:: astrora.bodies.Moon

Examples
--------

Using Bodies
~~~~~~~~~~~

.. code-block:: python

   from astrora.bodies import Earth, Mars
   from astrora.twobody import Orbit

   # Create Earth orbit
   orbit_earth = Orbit.from_classical(
       Earth,
       a=7000.0,  # km
       ecc=0.01,
       inc=28.5,
       raan=0.0,
       argp=0.0,
       nu=0.0
   )

   # Access body properties
   print(f"Earth mass: {Earth.mass} kg")
   print(f"Earth radius: {Earth.radius} km")
   print(f"Earth μ: {Earth.mu} km³/s²")

   # Mars properties
   print(f"Mars mass: {Mars.mass} kg")
   print(f"Mars μ: {Mars.mu} km³/s²")

Creating Custom Bodies
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.bodies import Body

   # Create a custom celestial body
   my_asteroid = Body(
       name="MyAsteroid",
       mass=1e15,           # kg
       radius=10.0,         # km
       mu=6.674e-14,        # km³/s²
       J2=0.0,              # No oblateness
       J3=0.0,
       rotation_period=None
   )

   # Use with orbits
   orbit = Orbit.from_classical(
       my_asteroid,
       a=50.0,  # 50 km orbit
       ecc=0.01,
       inc=10.0,
       raan=0.0,
       argp=0.0,
       nu=0.0
   )
