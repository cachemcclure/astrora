# Astrora Examples

This directory contains examples demonstrating various orbital mechanics and astrodynamics calculations using Astrora.

## Example Files

### Orbit Propagation

#### `orbit_propagation_perturbations.py`
High-fidelity orbit propagation with realistic perturbation forces.

**Features:**
- J2 oblateness perturbation (nodal regression, apsidal rotation)
- Atmospheric drag modeling (orbit decay analysis)
- Third-body perturbations (Sun/Moon gravitational effects)
- Solar radiation pressure (SRP for large area-to-mass spacecraft)
- Comparison of perturbation magnitudes across orbital regimes

**Examples include:**
- LEO sun-synchronous orbit with J2
- ISS-like orbit decay from atmospheric drag
- GEO satellite with solar radiation pressure
- HEO (Molniya) with lunar perturbations

**Key concepts:** Perturbation forces, station-keeping, orbital lifetime, adaptive integrators

**Run:**
```bash
python examples/orbit_propagation_perturbations.py
```

### Orbital Transfers

#### `hohmann_transfer.py`
Demonstrates Hohmann transfers, the most fuel-efficient two-impulse transfer between circular coplanar orbits.

**Examples include:**
- LEO to GEO transfer
- LEO to Lunar transfer orbit
- Earth-Mars interplanetary transfer
- Altitude change study

**Key concepts:** Delta-v budgets, transfer times, phase angles, synodic periods

**Run:**
```bash
python examples/hohmann_transfer.py
```

### Interplanetary Missions

#### `earth_mars_transfer.py`
Complete Earth-Mars transfer mission planning using circular orbit approximation for guaranteed convergence.

**Features:**
- Simplified circular orbit model for educational purposes
- Lambert solver integration for transfer trajectory
- Delta-v calculations at departure and arrival
- **C3 characteristic energy calculation** with launch vehicle compatibility check
- Multiple transfer window examples (Hohmann and fast transfers)
- Launch vehicle selection guidance (Atlas V, Falcon 9, Falcon Heavy, etc.)

**Run:**
```bash
python examples/earth_mars_transfer.py
```

#### `porkchop_plot.py`
Generate "porkchop plots" for launch window optimization in interplanetary missions.

**Features:**
- High-performance parallel Lambert solver (10-100x speedup)
- Contour plots of delta-v and time-of-flight
- Optimal launch date identification
- **C3 characteristic energy calculation and constraint filtering**
- Launch vehicle performance limits (Falcon 9, Delta IV Heavy, Atlas V, etc.)
- Smart date suggestion when Lambert solver fails to converge
- Synodic period-based transfer window finder
- Examples for Earth-Mars and Earth-Venus transfers

**Requirements:** matplotlib

**Run:**
```bash
python examples/porkchop_plot.py
```

**Output:** Creates `earth_mars_2025_porkchop.png` visualization

**New Features:**
- `calculate_c3()` - Compute C3 energy for each trajectory
- `filter_by_c3()` - Apply launch vehicle C3 constraints
- `find_good_transfer_dates()` - Intelligent transfer window finder
- `test_date_range_viability()` - Pre-validate date ranges

**Important Note:** This example uses real planetary ephemerides which can cause Lambert solver convergence issues for certain date ranges. The example demonstrates the workflow but may not always produce valid results depending on planetary geometry. For guaranteed results, consider:
- Using date ranges near known synodic periods (~26 months for Earth-Mars)
- Adjusting solver tolerance or iteration limits
- Using simplified circular orbit approximations (see `earth_mars_transfer.py`)
- Consulting specialized mission design tools for production use

#### `gravity_assist.py`
Planetary flyby maneuvers for trajectory modification without propellant.

**Examples:**
- Jupiter flyby (Voyager-style)
- Venus flyby (Cassini, Parker Solar Probe)
- Mars flyby for asteroid missions
- B-plane targeting for precision navigation
- Comparative analysis across all planets

**Key concepts:** Hyperbolic trajectories, deflection angles, heliocentric delta-v, impact parameters, **C3 energy at flyby**

**Enhanced output:**
- C3 characteristic energy for each flyby
- Launch requirements from previous body
- Hyperbolic excess velocity (v∞)
- Mission context and examples

**Run:**
```bash
python examples/gravity_assist.py
```

## Prerequisites

### Required Dependencies
All examples require the core Astrora installation:
```bash
pip install -e .
```

### Optional Dependencies
Some examples have additional requirements:

**For porkchop plots:**
```bash
pip install matplotlib
```

**For advanced visualization (future):**
```bash
pip install plotly
```

## Running the Examples

### Basic Usage
From the project root directory:
```bash
# Run a specific example
python examples/hohmann_transfer.py

# Or with uv (faster)
uv run python examples/hohmann_transfer.py
```

### Development Environment
If using the development environment with `uv`:
```bash
# Ensure you're in a virtual environment
source .venv/bin/activate  # Unix/macOS
# .venv\Scripts\activate   # Windows

# Run examples
python examples/earth_mars_transfer.py
```

## Example Output

Each example provides detailed console output with:
- Input parameters and mission constraints
- Calculated trajectories and orbital elements
- Delta-v budgets and transfer times
- Physical interpretations and mission context
- Comparison with real missions where applicable

Example output format:
```
======================================================================
Earth-Mars Transfer Mission
======================================================================
Departure: 2025-03-15
Arrival:   2025-11-01
Time of flight: 231.0 days (7.6 months)
======================================================================

Delta-v Requirements:
======================================================================
  At Earth departure: 3.612 km/s
  At Mars arrival:    2.034 km/s
  Total delta-v:      5.646 km/s
======================================================================
```

## Learning Path

Recommended order for learning orbital mechanics with these examples:

1. **`hohmann_transfer.py`** - Start here for basic orbital transfers
   - Understand two-impulse maneuvers
   - Learn about delta-v budgets
   - See practical LEO-GEO transfers

2. **`orbit_propagation_perturbations.py`** - Understand real-world forces
   - J2, drag, third-body, and SRP perturbations
   - Orbit decay and station-keeping requirements
   - LEO, MEO, and GEO perturbation magnitudes

3. **`earth_mars_transfer.py`** - Move to interplanetary transfers
   - Simplified planetary orbits for guaranteed results
   - Lambert's problem applications
   - C3 energy and launch vehicle selection
   - Mission planning fundamentals

4. **`porkchop_plot.py`** - Optimize launch windows
   - Batch trajectory calculations
   - Trade-off analysis (delta-v vs time)
   - C3 constraint filtering for realistic missions
   - Visual mission planning

5. **`gravity_assist.py`** - Advanced mission design
   - Planetary flybys
   - Multi-planet trajectories
   - "Free" delta-v from gravity
   - C3 requirements for flyby approach

## Performance Notes

Astrora's Rust backend provides significant performance improvements:

- **Single Lambert solve:** ~2-5 μs (19-31x faster than pure Python)
- **Batch operations:** ~19-20x faster with low overhead
- **Porkchop plots (1,600 points):** ~0.5-2 seconds (with parallelization)
- **Large porkchop plots (10,000 points):** ~5-20 seconds

Compare to pure Python implementations: typically 10-100x slower for equivalent calculations.

## Advanced Features

### C3 Characteristic Energy
All transfer and flyby examples now include C3 calculations:
- **C3 = v∞²** (hyperbolic excess energy)
- Determines launch vehicle requirements
- Used for mission feasibility analysis
- Launch vehicle database included (Falcon 9, Delta IV Heavy, Atlas V, etc.)

### Perturbation Modeling
High-fidelity orbit propagation with:
- **J2 oblateness** - Nodal regression, apsidal rotation
- **Atmospheric drag** - Exponential density model, orbit decay
- **Third-body** - Sun/Moon gravitational perturbations
- **Solar radiation pressure** - Cannon-ball model with eclipse handling
- **Adaptive integrators** - RK4, DoPri5, DOP853 for long-term accuracy

### Performance Optimizations
- **Parallel Lambert solver** - 10-100x faster than pure Python
- **Batch operations** - Process thousands of trajectories efficiently
- **Smart date finding** - Automatic transfer window identification

## Future Examples (Planned)

- `sgp4_satellite_tracking.py` - Real-world satellite tracking with TLEs
- `ground_station_visibility.py` - Satellite pass predictions
- `constellation_design.py` - Walker constellations and coverage analysis
- `rendezvous_planning.py` - Spacecraft rendezvous maneuvers
- `three_body_problem.py` - Lagrange points and restricted three-body dynamics
- `orbit_determination.py` - Batch least squares and Kalman filtering

## References

### Textbooks
- Vallado, "Fundamentals of Astrodynamics and Applications" (2013)
- Curtis, "Orbital Mechanics for Engineering Students" (2014)
- Bate, Mueller, White, "Fundamentals of Astrodynamics" (1971)

### Online Resources
- [Orbital Mechanics & Astrodynamics](https://orbital-mechanics.space/)
- [NASA JPL Solar System Dynamics](https://ssd.jpl.nasa.gov/)
- [Astropy Documentation](https://docs.astropy.org/)

### Historical Missions
- Voyager 1 & 2 (gravity assists)
- Cassini (V-V-E-J-S trajectory)
- New Horizons (Jupiter flyby)
- Parker Solar Probe (Venus resonance)
- Mars missions (various Hohmann transfers)

## Contributing

Have an interesting example to share? Contributions are welcome!

1. Follow the existing example format
2. Include comprehensive docstrings
3. Add mission context and physical interpretation
4. Reference real missions where applicable
5. Test the example before submitting

## License

These examples are part of the Astrora project and are licensed under the MIT License.

## Support

For questions or issues:
- GitHub Issues: https://github.com/cachemcclure/astrora/issues
- Documentation: See main README.md
