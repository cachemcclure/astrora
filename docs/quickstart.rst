Quick Start
===========

Basic Example
------------

.. code-block:: python

   from astrora.twobody import Orbit
   from astrora.bodies import Earth

   # Create an orbit
   orbit = Orbit.from_classical(
       Earth,
       a=7000.0,
       ecc=0.01,
       inc=28.5,
       raan=0.0,
       argp=0.0,
       nu=0.0
   )

   # Propagate
   orbit_future = orbit.propagate(3600.0)

   # Get state
   r, v = orbit.rv()
   print(f"Position: {r} km")
   print(f"Velocity: {v} km/s")
