Installation
============

Requirements
-----------

* Python 3.8 or later
* pip or uv (recommended)

Quick Install
------------

Using pip:

.. code-block:: bash

   pip install astrora

Using uv (faster):

.. code-block:: bash

   uv pip install astrora

From Source
----------

.. code-block:: bash

   git clone https://github.com/cachemcclure/astrora.git
   cd astrora
   uv pip install -e ".[dev]"

Verify Installation
------------------

.. code-block:: python

   import astrora
   print(astrora.__version__)
