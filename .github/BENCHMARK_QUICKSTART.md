# Benchmark CI - Quick Reference

A cheat sheet for running and interpreting benchmarks in Astrora.

## TL;DR

```bash
# Before benchmarking (CRITICAL!)
maturin develop --release

# Run benchmarks locally
pytest tests/benchmark_numpy_overhead.py --benchmark-only

# Save baseline and compare
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-save=baseline
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-compare=baseline
```

## When Benchmarks Run

| Event | Workflow | What Happens |
|-------|----------|--------------|
| Push to main | `benchmark.yml` | Fast Python benchmarks, store results |
| PR to main | `benchmark.yml` | Compare against main, fail if >20% slower |
| Weekly (Monday) | `benchmark-full.yml` | All benchmarks, comprehensive report |
| Rust code change | `benchmark-rust.yml` | Criterion benchmarks |

## Local Benchmarking

### Python Benchmarks

```bash
# 1. Build in release mode (otherwise results are meaningless)
source .venv/bin/activate
maturin develop --release

# 2. Run specific benchmark
pytest tests/benchmark_numpy_overhead.py --benchmark-only

# 3. Run all benchmarks
pytest tests/ -m benchmark --benchmark-only

# 4. With detailed output
pytest tests/benchmark_numpy_overhead.py --benchmark-only --benchmark-verbose

# 5. Compare against baseline
pytest tests/benchmark_numpy_overhead.py --benchmark-only \
    --benchmark-save=my-baseline \
    --benchmark-compare=my-baseline
```

### Rust Benchmarks

```bash
# Run all Rust benchmarks
cargo bench

# Run specific benchmark group
cargo bench propagators

# Open HTML report
open target/criterion/report/index.html
```

## Reading Benchmark Output

```
Name (time in μs)         Min      Max     Mean    StdDev   Median     OPS
---------------------------------------------------------------------------
cross_product_rust       2.50     4.50    2.75     0.25     2.70   363,636
cross_product_numpy     68.50    85.00   71.25     2.50    70.50    14,036

Rust is 27.40x faster ← Look here!
```

**What to look for:**
- **OPS (operations/sec)**: Higher is better
- **Mean/Median**: Average time (lower is better)
- **Ratio**: Speedup factor (e.g., 27.40x faster)
- **StdDev**: Consistency (lower is more stable)

## CI Behavior

### Pull Requests

✅ **PASS**: New code is same speed or faster
```
All benchmarks within 20% of baseline
```

⚠️ **FAIL**: Performance regression detected
```
Performance regression detected:
- test_foo: 2.75 μs → 3.71 μs (↑35%)

Action required: Investigate or optimize
```

### Main Branch

Every push to main:
- Runs benchmarks
- Updates historical data
- Stores in `gh-pages` branch
- No pass/fail, just records

## Common Issues

### ❌ "Benchmarks are slow locally"

**Cause**: Built in debug mode
```bash
# Check mode
cargo metadata --format-version 1 | grep '"profile"'

# Fix: rebuild in release
maturin develop --release
```

### ❌ "CI fails but local passes"

**Causes:**
1. Different CPU architecture (CI = x86-64, Mac = ARM)
2. Not using release mode locally
3. System load differences

**Fix:**
```bash
# Ensure using release mode
maturin develop --release

# Run with more rounds for stability
pytest tests/benchmark_numpy_overhead.py --benchmark-only \
    --benchmark-min-rounds=10
```

### ❌ "Benchmarks are flaky"

**Fix**: Increase minimum rounds and warmup
```bash
pytest tests/benchmark_numpy_overhead.py --benchmark-only \
    --benchmark-warmup=on \
    --benchmark-min-rounds=10
```

## Best Practices

### ✅ DO

- Build in release mode: `maturin develop --release`
- Warm up before benchmarking: `--benchmark-warmup=on`
- Run multiple rounds: `--benchmark-min-rounds=5`
- Close other applications during benchmarking
- Use same Python version as CI (3.11)

### ❌ DON'T

- Benchmark in debug mode (10-100x slower)
- Compare different CPU architectures directly
- Ignore regression alerts without investigation
- Benchmark with heavy system load
- Make performance conclusions from single run

## Workflow Commands

```bash
# List workflows
gh workflow list

# Run benchmark manually
gh workflow run benchmark.yml

# View recent runs
gh run list --workflow=benchmark.yml --limit 5

# View specific run logs
gh run view <run-id> --log

# Download artifacts
gh run download <run-id>
```

## Interpreting Regressions

When you see a regression alert:

1. **Verify**: Run benchmarks locally in release mode
2. **Profile**: Use flamegraphs to identify bottleneck
   ```bash
   cargo flamegraph --bench propagators
   ```
3. **Analyze**: Is this expected? (e.g., added features)
4. **Optimize**: Fix if unintentional
5. **Document**: Explain if intentional trade-off

## Performance Targets

| Operation | Target Speedup | Actual |
|-----------|----------------|--------|
| Cross products | >20x | 27.5x ✅ |
| Vector normalization | >5x | 7.3x ✅ |
| Dot products | >2x | 2-4x ✅ |
| Matrix operations | >1x | 1.5-6.8x ✅ |
| Overall workflow | 5-10x | 5-10x ✅ |

## Getting Help

- **Documentation**: `.github/BENCHMARK_CI.md` (comprehensive guide)
- **Workflow files**: `.github/workflows/benchmark*.yml`
- **Benchmark tests**: `tests/benchmark_*.py`
- **Issues**: Create issue with `benchmark` label

## Quick Diagnosis Decision Tree

```
Is benchmark slow?
├─ Debug mode? → maturin develop --release
├─ Cold start? → Add --benchmark-warmup=on
├─ Flaky results? → Add --benchmark-min-rounds=10
└─ CI vs local mismatch? → Check CPU architecture

Is CI failing?
├─ Real regression? → Profile and optimize
├─ Intentional? → Document and adjust threshold
├─ Flaky? → Increase alert-threshold
└─ False positive? → Review baseline data
```

## Advanced Options

### Save Multiple Baselines

```bash
# Save different scenarios
pytest tests/benchmark_numpy_overhead.py --benchmark-only \
    --benchmark-save=before-optimization

# Make changes...

pytest tests/benchmark_numpy_overhead.py --benchmark-only \
    --benchmark-save=after-optimization

# Compare
pytest-benchmark compare before-optimization after-optimization
```

### Benchmark Specific Tests

```bash
# Use -k for pattern matching
pytest tests/ -k "cross_product" --benchmark-only

# Combine with markers
pytest tests/ -m "benchmark and not slow" --benchmark-only
```

### Generate Reports

```bash
# HTML report
pytest tests/benchmark_numpy_overhead.py --benchmark-only \
    --benchmark-save=results \
    --benchmark-save-data

# View with pytest-benchmark compare
pytest-benchmark compare results --histogram
```

---

**Remember**: Always use `maturin develop --release` before benchmarking! 🚀
