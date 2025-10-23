Utilities
=========

Utility functions for unit conversions, validation, and helper operations.

.. currentmodule:: astrora.util

Overview
--------

The util module provides commonly used utility functions for:

* Unit conversions (angles, distances, time)
* Input validation
* Numerical utilities
* Helper functions for common operations

Functions
---------

Unit Conversions
~~~~~~~~~~~~~~~

Angle Conversions
^^^^^^^^^^^^^^^^

.. autofunction:: astrora.util.deg_to_rad
.. autofunction:: astrora.util.rad_to_deg

Distance Conversions
^^^^^^^^^^^^^^^^^^^

.. autofunction:: astrora.util.km_to_m
.. autofunction:: astrora.util.m_to_km
.. autofunction:: astrora.util.km_to_au
.. autofunction:: astrora.util.au_to_km

Time Conversions
^^^^^^^^^^^^^^^

.. autofunction:: astrora.util.days_to_sec
.. autofunction:: astrora.util.sec_to_days
.. autofunction:: astrora.util.years_to_days

Array Operations
~~~~~~~~~~~~~~~

.. autofunction:: astrora.util.normalize_vector
.. autofunction:: astrora.util.vector_magnitude
.. autofunction:: astrora.util.cross_product
.. autofunction:: astrora.util.dot_product

Validation
~~~~~~~~~

.. autofunction:: astrora.util.check_orbit_validity
.. autofunction:: astrora.util.validate_state_vector

Examples
--------

Unit Conversions
~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.util import deg_to_rad, rad_to_deg
   from astrora.util import km_to_m, au_to_km

   # Angle conversions
   angle_deg = 45.0
   angle_rad = deg_to_rad(angle_deg)
   print(f"{angle_deg}° = {angle_rad} radians")

   # Distance conversions
   dist_km = 7000.0
   dist_m = km_to_m(dist_km)
   print(f"{dist_km} km = {dist_m} m")

   # Astronomical unit conversions
   dist_au = 1.0  # 1 AU
   dist_km = au_to_km(dist_au)
   print(f"{dist_au} AU = {dist_km} km")

Vector Operations
~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.util import normalize_vector, vector_magnitude
   from astrora.util import cross_product, dot_product
   import numpy as np

   # Create vectors
   v1 = np.array([1.0, 2.0, 3.0])
   v2 = np.array([4.0, 5.0, 6.0])

   # Magnitude and normalization
   mag = vector_magnitude(v1)
   v1_norm = normalize_vector(v1)
   print(f"Magnitude: {mag}")
   print(f"Normalized: {v1_norm}")

   # Dot and cross products
   dot = dot_product(v1, v2)
   cross = cross_product(v1, v2)
   print(f"Dot product: {dot}")
   print(f"Cross product: {cross}")

.. note::
   Vector operations use high-performance Rust implementations and are
   significantly faster than pure Python/NumPy for small arrays.
   For large arrays, NumPy's BLAS-backed operations may be more efficient.

Validation
~~~~~~~~~

.. code-block:: python

   from astrora.util import check_orbit_validity, validate_state_vector
   import numpy as np

   # Check if orbital elements are valid
   try:
       check_orbit_validity(
           a=7000.0,
           ecc=0.01,
           inc=28.5,
           raan=0.0,
           argp=0.0,
           nu=0.0
       )
       print("Orbit is valid!")
   except ValueError as e:
       print(f"Invalid orbit: {e}")

   # Validate state vector
   r = np.array([7000.0, 0.0, 0.0])
   v = np.array([0.0, 7.5, 0.0])

   try:
       validate_state_vector(r, v)
       print("State vector is valid!")
   except ValueError as e:
       print(f"Invalid state vector: {e}")

Performance Tips
---------------

* Vector operations are optimized for small vectors (3D)
* For batch operations on many vectors, use NumPy's vectorized operations
* Unit conversion functions have minimal overhead (~10-40 ns)
* Cross products are 27.5x faster in Rust than pure Python
