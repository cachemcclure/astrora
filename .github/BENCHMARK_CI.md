# Continuous Benchmarking Documentation

This document explains how continuous benchmarking is set up in the Astrora project to track performance regressions automatically.

## Overview

Astrora uses **continuous benchmarking** to ensure that code changes don't introduce performance regressions. We track both Python (pytest-benchmark) and Rust (Criterion) benchmarks across commits.

### Goals

1. **Detect regressions early**: Automatically identify performance drops before merging
2. **Track performance trends**: Monitor performance improvements/degradations over time
3. **Prevent performance debt**: Make performance a first-class concern in code review
4. **Historical tracking**: Maintain benchmark history for long-term analysis

## GitHub Actions Workflows

### 1. Continuous Benchmarking (`benchmark.yml`)

**Triggers:**
- Push to `main` branch
- Pull requests to `main`
- Manual trigger via workflow_dispatch

**What it does:**
- Runs Python benchmarks from `tests/benchmark_numpy_overhead.py`
- Generates JSON output with `--benchmark-json`
- Stores results in `gh-pages` branch for historical tracking
- **For PRs**: Compares against `main` branch and fails if regression > 20%
- **For main**: Updates benchmark history

**Regression threshold:** 120% (alerts if new code is 20% slower)

**Usage:**
```bash
# Workflow runs automatically on PR
# Manual trigger:
gh workflow run benchmark.yml
```

### 2. Full Benchmark Suite (`benchmark-full.yml`)

**Triggers:**
- Weekly schedule (Mondays at 00:00 UTC)
- Manual trigger via workflow_dispatch

**What it does:**
- Runs **all** benchmark files:
  - `benchmark_numpy_overhead.py` - Python/Rust data transfer overhead
  - `benchmark_batch_anomaly.py` - Batch anomaly conversion performance
  - `benchmark_parallel_batch.py` - Parallel processing benchmarks
- Combines results into single report
- Stores comprehensive historical data
- Generates comparison reports

**Usage:**
```bash
# Manual trigger for comprehensive benchmarking
gh workflow run benchmark-full.yml
```

### 3. Rust Benchmarks (`benchmark-rust.yml`)

**Triggers:**
- Push to `main` (when Rust code changes)
- Pull requests (when Rust code changes)
- Manual trigger

**What it does:**
- Runs Criterion benchmarks via `cargo bench`
- Processes Criterion output format
- Tracks Rust-level performance separately from Python interface
- Useful for isolating pure Rust performance

**Usage:**
```bash
# Manual trigger:
gh workflow run benchmark-rust.yml
```

## Running Benchmarks Locally

### Python Benchmarks

```bash
# Activate environment
source .venv/bin/activate

# Build in release mode (IMPORTANT for accurate benchmarks)
maturin develop --release

# Run single benchmark file
pytest tests/benchmark_numpy_overhead.py --benchmark-only

# Run with detailed output
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-verbose

# Save baseline for comparison
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-save=baseline

# Compare against saved baseline
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-compare=baseline

# Run all benchmarks
pytest tests/ -m benchmark --benchmark-only
```

### Rust Benchmarks

```bash
# Run all Criterion benchmarks
cargo bench

# Run specific benchmark group
cargo bench propagators

# View HTML reports
open target/criterion/report/index.html
```

## Understanding Results

### pytest-benchmark Output

```
-------------------------------------------------------------------------------------- benchmark: 5 tests --------------------------------------------------------------------------------------
Name (time in us)                    Min                 Max                Mean            StdDev              Median               IQR            Outliers       OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_cross_product_rust            2.5000 (1.0)        4.5000 (1.0)        2.7500 (1.0)     0.2500 (1.0)        2.7000 (1.0)      0.3000 (1.0)         50;0  363,636.3636 (1.0)        1000           1
test_cross_product_numpy          68.5000 (27.40)    85.0000 (18.89)     71.2500 (25.91)   2.5000 (10.00)     70.5000 (26.11)    2.0000 (6.67)         10;5   14,035.8880 (0.04)        100           1
```

**Key metrics:**
- **Min/Max/Mean**: Time taken per operation
- **Median**: Middle value (less affected by outliers)
- **OPS**: Operations per second (higher is better)
- **Ratio in parentheses**: Performance comparison (27.40x means 27x faster)

### Regression Alerts

When a PR introduces a performance regression > 20%, the workflow will:

1. **Fail the check** (for PRs only)
2. **Add a comment** to the PR with details:
   ```
   Performance Alert! ⚠️

   test_cross_product_rust regressed by 35%
   - Previous: 2.75 μs
   - Current:  3.71 μs
   ```
3. **CC relevant team members** for review

## Best Practices

### 1. Always Benchmark in Release Mode

Debug builds are 10-100x slower and don't reflect real performance:

```bash
# ❌ Wrong - debug mode
maturin develop
pytest tests/benchmark_numpy_overhead.py --benchmark-only

# ✅ Correct - release mode
maturin develop --release
pytest tests/benchmark_numpy_overhead.py --benchmark-only
```

### 2. Warm Up Before Benchmarking

Use `--benchmark-warmup=on` to avoid cold-start effects:

```bash
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-warmup=on
```

### 3. Run Multiple Rounds

For statistical significance, use minimum 5 rounds:

```bash
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-min-rounds=10
```

### 4. Compare Apples to Apples

When comparing performance:
- Use same Python version
- Use same Rust toolchain version
- Use same CPU (or same runner type in CI)
- Ensure no other processes interfere

### 5. Investigate Before Dismissing

If benchmarks fail in CI but pass locally:
- Check if you're using release mode
- Verify CPU architecture differences (CI uses x86-64, you might use ARM)
- Check for system load differences
- Review the benchmark history trend

## Benchmark History & Trends

### Viewing Historical Data

Benchmark results are stored in the `gh-pages` branch:

```bash
# Clone gh-pages branch
git clone -b gh-pages https://github.com/yourorg/astrora.git astrora-benchmarks
cd astrora-benchmarks/dev/bench

# View JSON data
cat benchmarks.json
```

### GitHub Pages Dashboard

If GitHub Pages is enabled, view results at:
```
https://yourorg.github.io/astrora/dev/bench/
```

This provides:
- Interactive charts
- Historical trends
- Per-commit performance tracking

## Troubleshooting

### Issue: Benchmarks flaky in CI

**Solution:** Increase minimum rounds and use relative thresholds:

```yaml
# In workflow file
--benchmark-min-rounds=10
alert-threshold: '150%'  # Allow more variance
```

### Issue: False positives on small performance changes

**Solution:** Increase alert threshold for less sensitive tests:

```yaml
alert-threshold: '130%'  # Only alert on 30%+ regressions
```

### Issue: Criterion results not processed

**Solution:** Check that `target/criterion` directory exists and contains results:

```bash
ls -la target/criterion/
```

### Issue: Benchmark comparison fails with "no baseline"

**Solution:** Ensure baseline exists in gh-pages branch or create one:

```bash
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-save=baseline
```

## Adding New Benchmarks

### Python Benchmarks

1. Create a new test file: `tests/benchmark_myfeature.py`

2. Write benchmark functions:

```python
import pytest
from astrora._core import my_function

def test_my_function_performance(benchmark):
    """Benchmark my_function with typical inputs."""
    result = benchmark(my_function, arg1, arg2)
    assert result is not None
```

3. Add to full benchmark workflow:

```yaml
# In .github/workflows/benchmark-full.yml
- name: Run my feature benchmarks
  run: |
    source .venv/bin/activate
    pytest tests/benchmark_myfeature.py \
      --benchmark-only \
      --benchmark-json=benchmark_myfeature.json
```

### Rust Benchmarks

1. Create benchmark file: `benches/my_benchmark.rs`

2. Write Criterion benchmarks:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use astrora_core::my_module::my_function;

fn benchmark_my_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| my_function(black_box(arg1), black_box(arg2)))
    });
}

criterion_group!(benches, benchmark_my_function);
criterion_main!(benches);
```

3. Register in `Cargo.toml`:

```toml
[[bench]]
name = "my_benchmark"
harness = false
```

## References

- [pytest-benchmark documentation](https://pytest-benchmark.readthedocs.io/)
- [Criterion documentation](https://bheisler.github.io/criterion.rs/book/)
- [github-action-benchmark](https://github.com/benchmark-action/github-action-benchmark)
- [GitHub Actions documentation](https://docs.github.com/en/actions)

## Performance Targets

Based on our benchmarks, Astrora achieves:

- **Python/Rust overhead**: < 10% for batch operations
- **Cross products**: 27x faster than NumPy
- **Vector operations**: 2-7x faster than NumPy
- **Overall workflow**: 5-10x faster than pure Python implementations

See `NUMPY_PERFORMANCE_ANALYSIS.md` and `HAPSIRA_PERFORMANCE_COMPARISON.md` for detailed results.
