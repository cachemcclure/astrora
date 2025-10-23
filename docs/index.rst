Astrora Documentation
====================

**Astrora** is a modern, high-performance astrodynamics library for Python, powered by Rust.

Astrora provides a comprehensive suite of tools for orbital mechanics, spacecraft trajectory analysis,
and mission design. By implementing performance-critical components in Rust, Astrora achieves
10-100x speedups over pure Python implementations while maintaining a familiar, easy-to-use Python API.

.. note::
   Astrora is the successor to the archived `poliastro <https://github.com/poliastro/poliastro>`_ library,
   rebuilt from the ground up with modern Python-Rust integration patterns.

Key Features
-----------

* **High Performance**: 10-100x faster than pure Python implementations
* **Modern Architecture**: Rust backend with seamless Python integration via PyO3
* **Comprehensive**: Orbital propagation, coordinate transformations, Lambert solvers, and more
* **Well Documented**: Full API documentation with examples and tutorials
* **Battle Tested**: Cross-validated against GMAT, Nyx-space, and NASA/ESA tools
* **Easy to Use**: Familiar Python API compatible with poliastro where practical

Quick Start
----------

Installation
~~~~~~~~~~~

.. code-block:: bash

   pip install astrora

Basic Usage
~~~~~~~~~~

.. code-block:: python

   from astrora import Orbit
   from astrora.bodies import Earth
   import numpy as np

   # Create an orbit from classical orbital elements
   orbit = Orbit.from_classical(
       Earth,
       a=7000.0,      # Semi-major axis (km)
       ecc=0.01,      # Eccentricity
       inc=28.5,      # Inclination (degrees)
       raan=0.0,      # Right ascension of ascending node (degrees)
       argp=0.0,      # Argument of periapsis (degrees)
       nu=0.0         # True anomaly (degrees)
   )

   # Propagate the orbit forward in time
   orbit_future = orbit.propagate(3600.0)  # Propagate 1 hour (3600 seconds)

   # Get state vectors
   r, v = orbit.rv()
   print(f"Position: {r} km")
   print(f"Velocity: {v} km/s")

Performance
----------

Astrora leverages Rust's performance characteristics to provide significant speedups:

* **Lambert Problem**: 50-200x faster (with batch operations and parallelization)
* **Numerical Propagation**: 10-50x faster
* **Coordinate Transformations**: 20-80x faster (batch operations)
* **Overall Workflow**: 5-20x typical improvement

Contents
--------

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   installation
   quickstart
   user_guide/index
   examples/index

.. toctree::
   :maxdepth: 2
   :caption: API Reference

   api/core
   api/twobody
   api/coordinates
   api/time
   api/bodies
   api/maneuver
   api/plotting
   api/util

.. toctree::
   :maxdepth: 2
   :caption: Developer Guide

   developer/architecture
   developer/contributing
   developer/benchmarks
   developer/validation

.. toctree::
   :maxdepth: 1
   :caption: About

   changelog
   license
   references

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
