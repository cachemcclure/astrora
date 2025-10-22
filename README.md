# Poliastro - Rust-Backed Astrodynamics Library

A modern, high-performance orbital mechanics library combining Python's ease of use with Rust's computational performance.

## Overview

This project aims to modernize and enhance the poliastro astrodynamics library by implementing performance-critical components in Rust while maintaining a Python interface for ease of use.

### Background

The original [poliastro](https://github.com/poliastro/poliastro) library was archived on October 14, 2023. While active forks like [hapsira](https://github.com/pleiszenburg/hapsira) continue development, this project represents a ground-up modernization effort that:
- Leverages the mature Rust astrodynamics ecosystem (2024-2025)
- Implements cutting-edge Python-Rust integration patterns
- Provides 10-100x performance improvements over pure Python
- Maintains compatibility with the poliastro API where practical

## Features (Planned)

- **High-performance orbit propagators** (Kepler, Cowell, numerical integrators)
- **Coordinate transformations** (GCRS, ICRS, ITRS conversions)
- **Lambert solvers** for orbital transfer calculations
- **State vector computations** (classical orbital elements ↔ Cartesian)
- **Atmospheric models** for drag calculations
- **N-body simulations**

## Installation

### Prerequisites

- Python 3.8 or higher
- Rust 1.70 or higher (automatically installed during build)

### From Source

```bash
# Install uv (recommended package manager)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Clone the repository
git clone https://github.com/cachemcclure/rustpoliastro.git
cd rustpoliastro

# Create virtual environment and install
uv venv --python 3.8
source .venv/bin/activate  # On Unix/macOS
# .venv\Scripts\activate   # On Windows

# Install in development mode
uv pip install -e ".[dev]"
```

## Quick Start

```python
import poliastro

# Basic usage examples coming soon
```

## Performance Goals

- Numerical propagation: 10-50x faster
- Lambert problem (batch): 50-100x faster with parallel processing
- Coordinate transformations (batch): 20-80x faster
- Overall workflow: 5-10x typical improvement

## Technical Stack

- **Python**: High-level API and user interface
- **Rust**: Performance-critical computations
- **PyO3**: Seamless Rust-Python bindings
- **maturin**: Build system for Rust-backed Python packages
- **uv**: Ultra-fast package management

## License

MIT License

## Acknowledgments

- Original [poliastro](https://github.com/poliastro/poliastro) project and contributors
- [AeroRust](https://aerorust.org/) community
- [nyx-space](https://github.com/nyx-space/nyx) for validation reference
