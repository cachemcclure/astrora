Core Module
===========

The core module provides fundamental astrodynamics data structures and constants.

.. currentmodule:: astrora

Module Contents
--------------

.. automodule:: astrora
   :members:
   :undoc-members:
   :show-inheritance:
   :exclude-members: _core

Rust Core Functions
------------------

The ``_core`` module contains high-performance Rust implementations of mathematical
and astrodynamics operations. These are automatically used by the Python API for
optimal performance.

.. note::
   The ``_core`` module is a compiled Rust extension and should not be imported directly.
   Use the high-level Python API instead.

Constants
---------

Physical and astronomical constants used throughout the library.

.. autodata:: astrora.G
   :annotation: Gravitational constant

.. autodata:: astrora.C
   :annotation: Speed of light

State Vectors
------------

Functions for working with Cartesian state vectors (position and velocity).

.. autofunction:: astrora.rv_to_coe
.. autofunction:: astrora.coe_to_rv
