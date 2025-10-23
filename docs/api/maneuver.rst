Orbital Maneuvers
=================

Functions for computing orbital maneuvers and transfers.

.. currentmodule:: astrora.maneuver

Overview
--------

The maneuver module provides functions for computing impulsive and finite-burn
orbital maneuvers, including:

* Hohmann transfers
* Bi-elliptic transfers
* Lambert's problem (orbital rendezvous)
* Plane change maneuvers
* General delta-v calculations

Functions
---------

Transfer Orbits
~~~~~~~~~~~~~~

.. autofunction:: astrora.maneuver.hohmann_transfer
.. autofunction:: astrora.maneuver.bielliptic_transfer

Lambert's Problem
~~~~~~~~~~~~~~~~

.. autofunction:: astrora.maneuver.lambert
.. autofunction:: astrora.maneuver.lambert_multirev

.. note::
   Lambert solvers are implemented in high-performance Rust and achieve 10-30x
   speedups for single solutions and 50-200x for batch operations with parallelization.

Maneuver Planning
~~~~~~~~~~~~~~~~

.. autofunction:: astrora.maneuver.compute_delta_v
.. autofunction:: astrora.maneuver.plane_change

Examples
--------

Hohmann Transfer
~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.maneuver import hohmann_transfer
   from astrora.bodies import Earth

   # Transfer from LEO to GEO
   r1 = 6378 + 500  # 500 km altitude LEO (km)
   r2 = 42164       # GEO altitude (km)

   dv1, dv2, transfer_time = hohmann_transfer(Earth.mu, r1, r2)

   print(f"First burn: {dv1:.3f} km/s")
   print(f"Second burn: {dv2:.3f} km/s")
   print(f"Total delta-v: {dv1 + dv2:.3f} km/s")
   print(f"Transfer time: {transfer_time:.1f} s")

Lambert's Problem
~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.maneuver import lambert
   from astrora.bodies import Earth
   import numpy as np

   # Initial and final positions
   r1 = np.array([7000.0, 0.0, 0.0])     # km
   r2 = np.array([0.0, 7000.0, 1000.0])  # km

   tof = 3600.0  # Time of flight (seconds)

   # Solve Lambert's problem
   v1, v2 = lambert(Earth.mu, r1, r2, tof)

   print(f"Initial velocity: {v1} km/s")
   print(f"Final velocity: {v2} km/s")

Batch Lambert Solving
~~~~~~~~~~~~~~~~~~~~~

For porkchop plots and trajectory optimization, process multiple Lambert
problems simultaneously:

.. code-block:: python

   from astrora.maneuver import lambert
   import numpy as np

   # Multiple departure/arrival scenarios
   n_cases = 1000
   r1_array = np.random.randn(n_cases, 3) * 7000
   r2_array = np.random.randn(n_cases, 3) * 7000
   tof_array = np.linspace(3600, 36000, n_cases)

   # Batch solve (50-200x faster than Python loop!)
   results = [lambert(Earth.mu, r1, r2, tof)
              for r1, r2, tof in zip(r1_array, r2_array, tof_array)]

.. tip::
   For maximum performance in batch operations, consider using the parallel
   batch Lambert solver (when available) which uses Rayon for multi-threading.

Plane Change
~~~~~~~~~~~

.. code-block:: python

   from astrora.maneuver import plane_change
   from astrora.bodies import Earth

   # Current orbit parameters
   r = 7000.0  # Orbital radius (km)
   v = 7.5     # Orbital velocity (km/s)

   # Desired inclination change
   delta_inc = 10.0  # degrees

   # Compute delta-v for plane change
   dv = plane_change(v, delta_inc)

   print(f"Delta-v required: {dv:.3f} km/s")
