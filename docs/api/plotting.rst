Visualization
=============

Tools for visualizing orbits, trajectories, and mission analysis results.

.. currentmodule:: astrora.plotting

Overview
--------

Astrora provides comprehensive visualization capabilities through multiple modules:

* **static**: Static 2D/3D plots using Matplotlib
* **interactive**: Interactive 3D plots using Plotly
* **animation**: Animated orbit propagation
* **porkchop**: Porkchop plots for interplanetary transfers
* **groundtrack**: Ground track plotting for satellite visibility

All plotting functions integrate seamlessly with the Orbit class and support
multiple orbits, custom styling, and dark mode.

Modules
-------

Static Plotting
~~~~~~~~~~~~~~

.. automodule:: astrora.plotting.static
   :members:
   :undoc-members:
   :show-inheritance:

Interactive Plotting
~~~~~~~~~~~~~~~~~~~

.. automodule:: astrora.plotting.interactive
   :members:
   :undoc-members:
   :show-inheritance:

Animation
~~~~~~~~~

.. automodule:: astrora.plotting.animation
   :members:
   :undoc-members:
   :show-inheritance:

Porkchop Plots
~~~~~~~~~~~~~

.. automodule:: astrora.plotting.porkchop
   :members:
   :undoc-members:
   :show-inheritance:

Ground Tracks
~~~~~~~~~~~~

.. automodule:: astrora.plotting.groundtrack
   :members:
   :undoc-members:
   :show-inheritance:

Examples
--------

Static 2D Plot
~~~~~~~~~~~~~

.. code-block:: python

   from astrora.twobody import Orbit
   from astrora.bodies import Earth
   from astrora.plotting import plot_orbit

   orbit = Orbit.from_classical(
       Earth, a=7000.0, ecc=0.01, inc=28.5,
       raan=0.0, argp=0.0, nu=0.0
   )

   plot_orbit(orbit, title="LEO Orbit")

Interactive 3D Plot
~~~~~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.plotting.interactive import plot_orbit_3d

   # Create interactive 3D plot (opens in browser)
   fig = plot_orbit_3d(orbit, show_trail=True)
   fig.show()

Multiple Orbits
~~~~~~~~~~~~~~

.. code-block:: python

   from astrora.plotting import plot_orbits

   orbit1 = Orbit.from_classical(Earth, a=7000.0, ecc=0.01, inc=28.5,
                                  raan=0.0, argp=0.0, nu=0.0)
   orbit2 = Orbit.from_classical(Earth, a=8000.0, ecc=0.05, inc=45.0,
                                  raan=30.0, argp=0.0, nu=0.0)

   plot_orbits([orbit1, orbit2], labels=["Orbit 1", "Orbit 2"])

Animated Orbit
~~~~~~~~~~~~~

.. code-block:: python

   from astrora.plotting.animation import animate_orbit

   # Create animated GIF
   animate_orbit(
       orbit,
       filename="orbit_animation.gif",
       duration=10.0,  # Animation duration (seconds)
       fps=30,         # Frames per second
       show_trail=True
   )

Porkchop Plot
~~~~~~~~~~~~

.. code-block:: python

   from astrora.plotting.porkchop import porkchop_plot
   from astrora.bodies import Earth, Mars
   from astrora.time import Epoch

   # Launch window analysis for Earth-Mars transfer
   launch_span = (Epoch.from_datetime(2026, 1, 1),
                  Epoch.from_datetime(2026, 12, 31))
   arrival_span = (Epoch.from_datetime(2026, 6, 1),
                   Epoch.from_datetime(2027, 6, 30))

   porkchop_plot(
       Earth, Mars,
       launch_span, arrival_span,
       max_c3=30.0  # Maximum departure energy (km²/s²)
   )

Ground Track
~~~~~~~~~~~

.. code-block:: python

   from astrora.plotting.groundtrack import plot_groundtrack

   # Plot satellite ground track
   plot_groundtrack(
       orbit,
       duration=orbit.period,  # One orbit period
       num_points=100
   )

Dark Mode
~~~~~~~~~

All plotting functions support dark mode:

.. code-block:: python

   from astrora.plotting import plot_orbit

   # Enable dark mode
   plot_orbit(orbit, dark_mode=True)

Customization
~~~~~~~~~~~~

.. code-block:: python

   from astrora.plotting import plot_orbit

   # Custom styling
   plot_orbit(
       orbit,
       color='red',
       linewidth=2.0,
       title="Custom Styled Orbit",
       show_grid=True,
       show_attractor=True,
       attractor_radius_scale=1.5
   )
