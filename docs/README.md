# Astrora Documentation

This directory contains the Sphinx documentation for Astrora.

## Building the Documentation

### Prerequisites

Install documentation dependencies:

```bash
uv pip install -e ".[docs]"
```

### Build HTML Documentation

From the `docs/` directory:

```bash
# Using Makefile
make html

# Or directly with sphinx-build
sphinx-build -b html . _build/html
```

The HTML documentation will be generated in `_build/html/`.

### View Documentation

Open `_build/html/index.html` in your browser:

```bash
open _build/html/index.html  # macOS
xdg-open _build/html/index.html  # Linux
start _build/html/index.html  # Windows
```

### Clean Build Artifacts

```bash
make clean
```

## Documentation Structure

```
docs/
├── api/              # API reference documentation
│   ├── bodies.rst
│   ├── coordinates.rst
│   ├── core.rst
│   ├── maneuver.rst
│   ├── plotting.rst
│   ├── time.rst
│   ├── twobody.rst
│   └── util.rst
├── developer/        # Developer guides
├── examples/         # Example notebooks and scripts
├── user_guide/       # User guides and tutorials
├── conf.py          # Sphinx configuration
├── index.rst        # Documentation home page
└── Makefile         # Build automation

```

## Configuration

The documentation is configured in `conf.py` with:

- **Theme**: Read the Docs (sphinx_rtd_theme)
- **Extensions**:
  - `sphinx.ext.autodoc` - Auto-generate API docs from docstrings
  - `sphinx.ext.napoleon` - NumPy-style docstring support
  - `sphinx.ext.viewcode` - Source code links
  - `sphinx.ext.intersphinx` - Cross-references to other projects
  - `sphinx.ext.mathjax` - Mathematical notation
  - `sphinx_autodoc_typehints` - Type hint documentation
  - `numpydoc` - NumPy docstring processing

## Contributing

When adding new modules or functions:

1. Write comprehensive NumPy-style docstrings
2. Add the module to the appropriate `api/*.rst` file
3. Rebuild the documentation to verify it renders correctly
4. Check for warnings during the build

## Notes

- The documentation includes comprehensive API reference for all Python modules
- Rust documentation can be built separately with `cargo doc`
- All warnings during build are expected for functions not yet implemented
- The documentation is configured to work with Python 3.8+
