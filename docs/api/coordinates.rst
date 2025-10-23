Coordinate Systems
==================

Functions for coordinate transformations between different reference frames.

.. currentmodule:: astrora.coordinates

Overview
--------

Astrora supports transformations between various astronomical coordinate systems:

* **GCRS** (Geocentric Celestial Reference System) - Inertial system with origin at Earth's center
* **ITRS** (International Terrestrial Reference System) - Earth-fixed rotating system
* **TEME** (True Equator Mean Equinox) - Used for TLE/SGP4 propagation

All transformations are implemented in high-performance Rust with support for
batch operations for maximum efficiency.

Functions
---------

GCRS ↔ ITRS Transformations
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. autofunction:: astrora.coordinates.gcrs_to_itrs
.. autofunction:: astrora.coordinates.itrs_to_gcrs

GCRS ↔ TEME Transformations
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. autofunction:: astrora.coordinates.gcrs_to_teme
.. autofunction:: astrora.coordinates.teme_to_gcrs

TEME ↔ ITRS Transformations
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. autofunction:: astrora.coordinates.teme_to_itrs
.. autofunction:: astrora.coordinates.itrs_to_teme

Batch Transformations
~~~~~~~~~~~~~~~~~~~~

For processing multiple coordinates efficiently:

.. autofunction:: astrora.coordinates.batch_gcrs_to_itrs
.. autofunction:: astrora.coordinates.batch_itrs_to_gcrs
.. autofunction:: astrora.coordinates.batch_gcrs_to_teme
.. autofunction:: astrora.coordinates.batch_teme_to_gcrs
.. autofunction:: astrora.coordinates.batch_teme_to_itrs
.. autofunction:: astrora.coordinates.batch_itrs_to_teme

.. note::
   Batch transformations are 10-20x faster than processing coordinates individually
   in a loop. Always use batch functions when processing multiple coordinates.

Examples
--------

Single Coordinate Transformation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.coordinates import gcrs_to_itrs
   import numpy as np

   # Position in GCRS (km)
   pos_gcrs = np.array([7000.0, 0.0, 0.0])

   # Convert to ITRS at a specific time (MJD)
   mjd = 58849.0  # Modified Julian Date
   pos_itrs = gcrs_to_itrs(pos_gcrs, mjd)

Batch Transformation
~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.coordinates import batch_gcrs_to_itrs
   import numpy as np

   # Multiple positions in GCRS (N x 3 array)
   positions = np.array([
       [7000.0, 0.0, 0.0],
       [0.0, 7000.0, 0.0],
       [0.0, 0.0, 7000.0],
   ])

   # Times for each position (N array)
   times = np.array([58849.0, 58849.1, 58849.2])

   # Transform all at once (much faster!)
   positions_itrs = batch_gcrs_to_itrs(positions, times)

Performance Tips
---------------

1. **Use batch functions** for multiple coordinates (10-20x speedup)
2. **Pre-allocate arrays** when possible
3. **Use contiguous arrays** for best performance (``numpy.ascontiguousarray``)
4. **Avoid Python loops** - process arrays in bulk
