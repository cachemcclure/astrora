Two-Body Problem
================

The two-body module provides classes and functions for working with Keplerian orbits
and two-body dynamics.

.. currentmodule:: astrora.twobody

Orbit Class
----------

.. autoclass:: astrora.twobody.Orbit
   :members:
   :undoc-members:
   :show-inheritance:
   :special-members: __init__

   .. rubric:: Creation Methods

   .. autosummary::
      :nosignatures:

      ~Orbit.from_classical
      ~Orbit.from_vectors
      ~Orbit.circular
      ~Orbit.parabolic

   .. rubric:: Propagation Methods

   .. autosummary::
      :nosignatures:

      ~Orbit.propagate
      ~Orbit.propagate_to_anomaly

   .. rubric:: State Methods

   .. autosummary::
      :nosignatures:

      ~Orbit.rv
      ~Orbit.classical
      ~Orbit.sample

   .. rubric:: Orbit Properties

   .. autosummary::

      ~Orbit.period
      ~Orbit.energy
      ~Orbit.angular_momentum
      ~Orbit.ecc_vector
      ~Orbit.r
      ~Orbit.v
      ~Orbit.a
      ~Orbit.ecc
      ~Orbit.inc
      ~Orbit.raan
      ~Orbit.argp
      ~Orbit.nu

   .. rubric:: Reference Frame

   .. autosummary::

      ~Orbit.frame
      ~Orbit.attractor

Examples
--------

Creating Orbits
~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.twobody import Orbit
   from astrora.bodies import Earth

   # From classical elements
   orbit = Orbit.from_classical(
       Earth,
       a=7000.0,      # Semi-major axis (km)
       ecc=0.01,      # Eccentricity
       inc=28.5,      # Inclination (degrees)
       raan=0.0,      # Right ascension of ascending node (degrees)
       argp=0.0,      # Argument of periapsis (degrees)
       nu=0.0         # True anomaly (degrees)
   )

   # From state vectors
   import numpy as np
   r = np.array([7000.0, 0.0, 0.0])  # Position (km)
   v = np.array([0.0, 7.5, 0.0])     # Velocity (km/s)
   orbit = Orbit.from_vectors(Earth, r, v)

Propagating Orbits
~~~~~~~~~~~~~~~~~

.. code-block:: python

   # Propagate forward in time
   orbit_future = orbit.propagate(3600.0)  # Propagate 1 hour

   # Propagate to specific true anomaly
   orbit_at_anomaly = orbit.propagate_to_anomaly(90.0)  # Propagate to 90 degrees

Accessing Orbit Properties
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   # Get state vectors
   r, v = orbit.rv()

   # Get classical elements
   a, ecc, inc, raan, argp, nu = orbit.classical()

   # Access specific properties
   print(f"Period: {orbit.period} s")
   print(f"Energy: {orbit.energy} km²/s²")
   print(f"Angular momentum: {orbit.angular_momentum} km²/s")
