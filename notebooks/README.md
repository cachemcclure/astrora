# Astrora Jupyter Notebook Examples

This directory contains comprehensive Jupyter notebook tutorials for learning orbital mechanics and astrodynamics with Astrora.

## Overview

These notebooks provide hands-on, interactive examples covering everything from basic orbit creation to advanced mission analysis. Each notebook builds on previous concepts and demonstrates Astrora's high-performance Rust-backed computations.

## Notebooks

### 01_quickstart.ipynb
**Introduction to Astrora**

Perfect starting point for new users!

**Topics:**
- Creating orbits from position/velocity vectors
- Using classical orbital elements
- Working with astropy units (poliastro-compatible)
- Basic orbit propagation
- Simple Hohmann transfers
- Working with different celestial bodies
- Performance comparison

**Prerequisites:** Basic Python knowledge

**Duration:** ~30 minutes

---

### 02_orbital_mechanics.ipynb
**Deep Dive into Orbital Mechanics**

Comprehensive exploration of orbital mechanics fundamentals.

**Topics:**
- Classical orbital elements (COE) in detail
- Effect of each orbital element
- Orbital energy and angular momentum
- Conservation laws
- True, eccentric, and mean anomaly
- Orbit classification by eccentricity
- State vector conversions (Cartesian ↔ COE)

**Prerequisites:** Completion of 01_quickstart.ipynb

**Duration:** ~45 minutes

---

### 03_maneuvers_transfers.ipynb
**Orbital Maneuvers and Transfers**

Essential for satellite operations and mission planning.

**Topics:**
- Hohmann transfers (most fuel-efficient)
- Bielliptic transfers (for large radius changes)
- Plane change maneuvers
- Lambert's problem
- Interplanetary transfers (Earth-Mars)
- Delta-v budgets
- Transfer orbit visualization

**Prerequisites:** Understanding of orbital mechanics

**Duration:** ~45 minutes

---

### 04_visualization_plotting.ipynb
**Advanced Visualization Techniques**

Create publication-quality plots and animations.

**Topics:**
- 2D orbit plotting with matplotlib
- 3D interactive visualization with plotly
- Ground track plotting
- Orbit animations (2D and 3D)
- Multiple orbit comparisons
- Custom styling and themes
- Transfer orbit visualization

**Prerequisites:** Basic plotting knowledge

**Duration:** ~40 minutes

---

### 05_advanced_techniques.ipynb
**Advanced Astrodynamics Techniques**

Sophisticated mission analysis leveraging Rust performance.

**Topics:**
- Monte Carlo uncertainty analysis
- High-performance batch processing
- Advanced perturbation modeling (J2, drag, SRP)
- Porkchop plots for launch windows
- Constellation deployment analysis
- Performance optimization strategies
- Real-world mission scenarios

**Prerequisites:** Strong understanding from previous notebooks

**Duration:** ~60 minutes

---

## Getting Started

### Installation

First, ensure Astrora is installed:

```bash
# Using uv (recommended - 10-100x faster than pip)
uv pip install astrora

# Or using pip
pip install astrora

# Install Jupyter
uv pip install jupyter
# or: pip install jupyter
```

### Running the Notebooks

#### Option 1: Jupyter Notebook (Classic)

```bash
# Navigate to the notebooks directory
cd notebooks/

# Start Jupyter Notebook
jupyter notebook

# Your browser will open - click on a notebook to start!
```

#### Option 2: JupyterLab (Modern Interface)

```bash
# Install JupyterLab
uv pip install jupyterlab

# Start JupyterLab
jupyter lab
```

#### Option 3: VS Code

If you use VS Code:
1. Install the "Jupyter" extension
2. Open any `.ipynb` file
3. Click "Select Kernel" and choose your Python environment
4. Run cells interactively!

### Recommended Learning Path

**For Beginners:**
1. Start with `01_quickstart.ipynb`
2. Progress through notebooks sequentially (01 → 02 → 03 → 04 → 05)
3. Experiment with modifying code examples
4. Complete exercises in each notebook

**For Experienced Users:**
- Jump directly to notebooks matching your interest
- `03_maneuvers_transfers.ipynb` for mission planning
- `04_visualization_plotting.ipynb` for creating plots
- `05_advanced_techniques.ipynb` for performance-critical applications

**For Educators:**
- Use notebooks as teaching materials
- Modify examples for your curriculum
- Students can run code interactively
- Great for homework assignments

## Key Features

### 🚀 High Performance
All notebooks leverage Astrora's Rust backend:
- **10-100x faster** than pure Python implementations
- Real-time propagation and visualization
- Batch processing for large datasets
- Monte Carlo simulations with thousands of samples

### 📊 Rich Visualizations
- Interactive 3D plots with Plotly
- Publication-quality matplotlib figures
- Orbit animations (2D and 3D)
- Ground track plotting
- Custom themes and styling

### 🎓 Educational Focus
- Clear explanations of concepts
- Real-world mission examples
- Step-by-step code walkthroughs
- Physical interpretations
- References to textbooks and papers

### 🔧 Production-Ready
- Code examples ready for adaptation
- Best practices demonstrated
- Performance optimization tips
- Error handling examples

## Dependencies

All notebooks require:
- **astrora** - The main library
- **numpy** - Array operations
- **astropy** - Units and time handling
- **matplotlib** - 2D plotting
- **plotly** - 3D interactive visualization

Optional but recommended:
- **jupyterlab** - Modern Jupyter interface
- **ipywidgets** - Interactive widgets

Install all dependencies:
```bash
uv pip install astrora numpy astropy matplotlib plotly jupyterlab ipywidgets
```

## Tips for Success

### Running Code Cells
- Press `Shift + Enter` to run a cell and move to the next
- Press `Ctrl + Enter` to run a cell and stay on it
- Press `Alt + Enter` to run a cell and insert a new one below

### Troubleshooting

**Kernel dies or crashes:**
- Restart kernel: `Kernel → Restart`
- May indicate memory issue with large datasets
- Try reducing `n_samples` in Monte Carlo examples

**Import errors:**
```python
# Check if astrora is installed
import sys
print(sys.executable)  # Should match your virtual environment

# Reinstall if needed
!pip install --force-reinstall astrora
```

**Plots not showing:**
```python
# Ensure matplotlib inline backend is active
%matplotlib inline

# For interactive plots, try:
%matplotlib widget
```

**Slow performance:**
- Ensure you're using the Rust-compiled version of astrora
- Check that `maturin develop --release` was used during installation
- Reduce `n_samples` or `n_steps` for faster execution

### Best Practices

1. **Always run cells in order** - Later cells depend on earlier ones
2. **Restart kernel if confused** - Fresh start eliminates state issues
3. **Modify and experiment** - Best way to learn is by doing!
4. **Save your work** - Notebooks auto-save, but use `Ctrl+S` frequently
5. **Add your own notes** - Edit markdown cells to add annotations

## Examples and Use Cases

### Mission Planning
- Hohmann transfer calculations (Notebook 3)
- Lambert's problem for rendezvous (Notebook 3)
- Porkchop plots for launch windows (Notebook 5)
- Delta-v budgets (Notebook 3)

### Satellite Operations
- Orbit propagation with perturbations (Notebook 5)
- Ground track visualization (Notebook 4)
- Station-keeping requirements (Notebook 5)
- Constellation deployment (Notebook 5)

### Research & Analysis
- Monte Carlo uncertainty quantification (Notebook 5)
- Batch trajectory optimization (Notebook 5)
- High-fidelity propagation (Notebook 5)
- Performance benchmarking (Notebook 5)

### Education
- Teaching orbital mechanics (Notebooks 1-2)
- Visualizing orbital elements (Notebook 2)
- Demonstrating conservation laws (Notebook 2)
- Interactive mission design (Notebooks 3-4)

## Performance Notes

Astrora's Rust backend provides exceptional performance:

| Operation | Astrora (Rust) | Pure Python | Speedup |
|-----------|----------------|-------------|---------|
| State vector conversion | ~2 μs | ~50 μs | **25x** |
| RK4 propagation step | ~5 μs | ~100 μs | **20x** |
| Lambert solver | ~3 μs | ~100 μs | **33x** |
| Batch operations (1000x) | ~2 ms | ~100 ms | **50x** |
| Monte Carlo (10000 samples) | ~0.5 s | ~10 s | **20x** |

**Key insight:** Rust shines for batch operations and repeated calculations!

## Contributing

Found an error? Have a suggestion? Want to add a notebook?

1. **Report issues:** Open an issue on GitHub
2. **Suggest improvements:** Pull requests welcome!
3. **Share your notebooks:** We'd love to see your work
4. **Ask questions:** GitHub Discussions or issues

## Additional Resources

### Documentation
- [Astrora Main README](../README.md) - Project overview
- [Installation Guide](../INSTALLATION.md) - Detailed setup instructions
- [Quickstart Guide](../QUICKSTART.md) - Text-based quickstart
- [Advanced Usage](../ADVANCED_USAGE.md) - Advanced techniques reference
- [Performance Guide](../PERFORMANCE.md) - Optimization strategies
- [Migration Guide](../MIGRATION.md) - For poliastro/hapsira users

### Python Examples
- [examples/](../examples/) - Standalone Python scripts
- Shows same concepts in script format
- Useful for automation and batch processing

### Academic References
- Curtis, "Orbital Mechanics for Engineering Students" (4th ed., 2014)
- Vallado, "Fundamentals of Astrodynamics and Applications" (4th ed., 2013)
- Prussing & Conway, "Orbital Mechanics" (2nd ed., 2012)
- Bate, Mueller, White, "Fundamentals of Astrodynamics" (1971)

### Online Resources
- [Orbital Mechanics for Engineering Students](https://orbital-mechanics.space/)
- [NASA JPL Solar System Dynamics](https://ssd.jpl.nasa.gov/)
- [Astropy Documentation](https://docs.astropy.org/)
- [AeroRust Community](https://aerorust.org/)

## License

These notebooks are part of the Astrora project and are licensed under the MIT License.

Copyright (c) 2024 Astrora Contributors

## Acknowledgments

- Inspired by [poliastro](https://github.com/poliastro/poliastro) (archived October 2023)
- Built with [PyO3](https://pyo3.rs/), [maturin](https://www.maturin.rs/), and [Rust](https://www.rust-lang.org/)
- Uses algorithms from Curtis, Vallado, and other astrodynamics literature
- Leverages [hifitime](https://docs.rs/hifitime/), [nalgebra](https://docs.rs/nalgebra/), and Rust scientific computing ecosystem

---

**Happy learning! 🚀 If you find these notebooks useful, please star the project on GitHub!**
