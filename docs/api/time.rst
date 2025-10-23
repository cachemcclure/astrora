Time Systems
============

High-precision time handling with support for astronomical time scales.

.. currentmodule:: astrora.time

Overview
--------

Astrora uses the `hifitime <https://docs.rs/hifitime/>`_ Rust library for
ultra-precise time handling with nanosecond precision and proper leap second support.

The time module provides:

* **Epoch**: A specific point in time
* **Duration**: A time interval
* Support for multiple time scales (UTC, TAI, TT, TDB, etc.)
* Conversions between time scales
* Leap second handling

Classes
-------

Epoch
~~~~~

.. autoclass:: astrora.time.Epoch
   :members:
   :undoc-members:
   :show-inheritance:
   :special-members: __init__, __add__, __sub__

Duration
~~~~~~~~

.. autoclass:: astrora.time.Duration
   :members:
   :undoc-members:
   :show-inheritance:
   :special-members: __init__, __add__, __sub__, __mul__

Examples
--------

Creating Epochs
~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.time import Epoch

   # From calendar date
   epoch = Epoch.from_datetime(2024, 1, 1, 12, 0, 0.0)

   # From Modified Julian Date
   epoch = Epoch.from_mjd(58849.0)

   # From Julian Date
   epoch = Epoch.from_jd(2458849.5)

   # Current time
   epoch = Epoch.now()

Working with Durations
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.time import Epoch, Duration

   epoch = Epoch.now()

   # Create durations
   one_hour = Duration.from_seconds(3600.0)
   one_day = Duration.from_days(1.0)

   # Epoch arithmetic
   future = epoch + one_hour
   past = epoch - one_day

   # Duration arithmetic
   time_diff = future - past
   print(f"Difference: {time_diff.to_seconds()} seconds")

Time Scale Conversions
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.time import Epoch

   epoch_utc = Epoch.from_datetime(2024, 1, 1, 12, 0, 0.0)

   # Convert to other time scales
   epoch_tai = epoch_utc.to_tai()
   epoch_tt = epoch_utc.to_tt()
   epoch_tdb = epoch_utc.to_tdb()

   # Get as Modified Julian Date in different scales
   mjd_utc = epoch_utc.to_mjd_utc()
   mjd_tai = epoch_utc.to_mjd_tai()
   mjd_tt = epoch_utc.to_mjd_tt()
