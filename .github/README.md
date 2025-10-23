# GitHub Configuration

This directory contains GitHub-specific configuration files for the Astrora project.

## Directory Structure

```
.github/
├── workflows/           # GitHub Actions workflow definitions
│   ├── benchmark.yml           # Continuous benchmarking (runs on PRs and main)
│   ├── benchmark-full.yml      # Full benchmark suite (weekly schedule)
│   └── benchmark-rust.yml      # Rust Criterion benchmarks
├── BENCHMARK_CI.md     # Comprehensive benchmarking documentation
└── README.md           # This file
```

## Workflows

### Continuous Benchmarking Workflows

The project uses three separate benchmark workflows to track performance:

1. **`benchmark.yml`**: Fast Python benchmarks on every PR/push
   - Detects regressions quickly
   - Fails PRs with >20% performance loss
   - ~5 minutes runtime

2. **`benchmark-full.yml`**: Comprehensive benchmark suite
   - Runs all benchmark files
   - Scheduled weekly
   - Generates detailed reports
   - ~15-20 minutes runtime

3. **`benchmark-rust.yml`**: Rust-level performance tracking
   - Criterion benchmarks
   - Runs when Rust code changes
   - Isolates pure Rust performance
   - ~10-15 minutes runtime

### Quick Reference

```bash
# View workflow status
gh workflow list

# Run benchmark manually
gh workflow run benchmark.yml

# View recent runs
gh run list --workflow=benchmark.yml

# View logs for a run
gh run view <run-id>
```

## Setting Up Benchmark CI

### First-Time Setup

1. **Enable GitHub Pages** (for benchmark history dashboard):
   - Go to Settings → Pages
   - Source: Deploy from a branch
   - Branch: `gh-pages` / `root`

2. **Configure Permissions** (if needed):
   - Settings → Actions → General
   - Workflow permissions: Read and write permissions
   - Allow GitHub Actions to create pull requests: ✓

3. **Create gh-pages branch** (if doesn't exist):
   ```bash
   git checkout --orphan gh-pages
   git rm -rf .
   echo "# Benchmark Results" > index.html
   git add index.html
   git commit -m "Initialize gh-pages"
   git push origin gh-pages
   git checkout main
   ```

### Configuration

Key configuration options in workflow files:

```yaml
# Alert threshold (120% = 20% regression)
alert-threshold: '120%'

# Fail workflow on regression (PR only)
fail-on-alert: true

# Comment on PR with results
comment-on-alert: true

# CC team members on alerts
alert-comment-cc-users: '@poliastro-team'
```

## Benchmark Results

### Viewing Results

1. **Workflow artifacts**: Available for 90 days after run
   ```bash
   gh run view <run-id> --log
   ```

2. **GitHub Pages**: Interactive dashboard
   ```
   https://yourorg.github.io/astrora/dev/bench/
   ```

3. **PR comments**: Automatic comparison posted on PRs

### Understanding Alerts

When a benchmark regression is detected, you'll see:

```
⚠️ Performance Alert

Possible performance regression detected:
- test_cross_product_rust: 2.75 μs → 3.71 μs (↑35%)
- test_normalize_vector: 1.50 μs → 1.95 μs (↑30%)

2 benchmarks regressed, 10 remained stable.
```

**Actions to take:**
1. Review the PR changes for performance-impacting code
2. Run benchmarks locally to confirm
3. Profile the code to identify bottlenecks
4. Optimize or document intentional trade-offs

## Best Practices

### For Contributors

- **Always run benchmarks locally** before pushing performance-critical changes
- **Use release mode** for accurate results: `maturin develop --release`
- **Investigate failures** rather than disabling checks
- **Document trade-offs** if performance regression is intentional

### For Maintainers

- **Review benchmark trends** weekly via GitHub Pages
- **Adjust thresholds** if too many false positives occur
- **Add benchmarks** for new performance-critical features
- **Archive old artifacts** to save storage

## Troubleshooting

### Common Issues

**Issue**: Workflow fails with "Permission denied"
- **Fix**: Enable write permissions in Settings → Actions

**Issue**: No baseline found for comparison
- **Fix**: Run full benchmark suite once on main branch

**Issue**: Inconsistent results between local and CI
- **Fix**: Verify using release mode and same Python version

**Issue**: Benchmarks are flaky
- **Fix**: Increase `--benchmark-min-rounds` or `alert-threshold`

See `BENCHMARK_CI.md` for detailed troubleshooting guide.

## Adding New Workflows

To add a new workflow:

1. Create `workflows/new-workflow.yml`
2. Define triggers, jobs, and steps
3. Test locally with [act](https://github.com/nektos/act) (optional)
4. Push and verify in GitHub Actions tab

**Template:**

```yaml
name: My Workflow

on:
  push:
    branches: [main]

jobs:
  my-job:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run something
        run: echo "Hello"
```

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [pytest-benchmark](https://pytest-benchmark.readthedocs.io/)
- [Criterion.rs](https://bheisler.github.io/criterion.rs/)
- [github-action-benchmark](https://github.com/benchmark-action/github-action-benchmark)

## Maintenance

### Workflow Updates

Workflows use versioned actions (e.g., `actions/checkout@v4`). Update regularly:

```bash
# Check for outdated actions
gh extension install mheap/gh-action-update

# Update actions
gh action-update --update
```

### Storage Management

Benchmark artifacts are retained for 90 days. To adjust:

```yaml
retention-days: 30  # Reduce to save storage
```

Historical benchmark data in `gh-pages` branch should be periodically archived or cleaned.

---

For detailed documentation on continuous benchmarking, see [`BENCHMARK_CI.md`](./BENCHMARK_CI.md).
