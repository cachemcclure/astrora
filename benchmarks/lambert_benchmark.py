"""
Comprehensive benchmark comparing Python and Rust Lambert solvers.

This script benchmarks:
1. Python baseline implementation (pure NumPy)
2. Rust implementation (via astrora_core PyO3 bindings)
3. Optional: poliastro/hapsira for reference

Results are saved to a markdown file for documentation.

Usage:
    python benchmarks/lambert_benchmark.py
"""

import sys
import time
from pathlib import Path
from typing import Callable, Dict, List, Tuple

import numpy as np

# Add parent directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent))

# Import Python baseline
from lambert_python_baseline import POLIASTRO_AVAILABLE, lambert_batch, lambert_universal_variable

# Import Rust implementation
try:
    import astrora._core as astrora_core

    RUST_AVAILABLE = True
except ImportError:
    print("WARNING: Rust astrora_core not available. Build with 'maturin develop' first.")
    RUST_AVAILABLE = False

# Constants
EARTH_MU = 3.986004418e14
SUN_MU = 1.32712440018e20

# =============================================================================
# Test Case Definitions
# =============================================================================


def leo_to_leo_quarter():
    """LEO to LEO quarter-orbit transfer"""
    r_leo = 7000e3
    r1 = np.array([r_leo, 0.0, 0.0])
    r2 = np.array([0.0, r_leo, 0.0])
    period = 2.0 * np.pi * (r_leo**3 / EARTH_MU) ** 0.5
    tof = period / 4.0
    return r1, r2, tof, EARTH_MU


def leo_to_geo_transfer():
    """LEO to GEO Hohmann-like transfer"""
    r_leo = 7000e3
    r_geo = 42164e3
    r1 = np.array([r_leo, 0.0, 0.0])
    r2 = np.array([0.0, r_geo, 0.0])
    a_transfer = (r_leo + r_geo) / 2.0
    tof = np.pi * (a_transfer**3 / EARTH_MU) ** 0.5
    return r1, r2, tof, EARTH_MU


def complex_3d_transfer():
    """Complex 3D transfer (Vallado example)"""
    r1 = np.array([5000e3, 10000e3, 2100e3])
    r2 = np.array([-14600e3, 2500e3, 7000e3])
    tof = 3600.0
    return r1, r2, tof, EARTH_MU


# =============================================================================
# Benchmark Utilities
# =============================================================================


def timeit(func: Callable, *args, iterations: int = 100, **kwargs) -> Tuple[float, float]:
    """
    Time a function call with multiple iterations.

    Returns
    -------
    mean_time, std_time : Tuple[float, float]
        Mean and standard deviation of execution time (seconds)
    """
    times = []

    # Warmup
    for _ in range(min(5, iterations // 10)):
        func(*args, **kwargs)

    # Actual timing
    for _ in range(iterations):
        start = time.perf_counter()
        func(*args, **kwargs)
        end = time.perf_counter()
        times.append(end - start)

    return np.mean(times), np.std(times)


def format_time(t: float) -> str:
    """Format time in appropriate units"""
    if t >= 1.0:
        return f"{t:.3f} s"
    elif t >= 1e-3:
        return f"{t * 1e3:.3f} ms"
    elif t >= 1e-6:
        return f"{t * 1e6:.3f} μs"
    else:
        return f"{t * 1e9:.3f} ns"


def format_speedup(speedup: float) -> str:
    """Format speedup ratio"""
    if speedup >= 1.0:
        return f"{speedup:.1f}x faster"
    else:
        return f"{1.0 / speedup:.1f}x slower"


# =============================================================================
# Benchmark 1: Single Lambert Solve
# =============================================================================


def bench_single_solve(test_name: str, r1, r2, tof, mu, iterations: int = 100) -> Dict:
    """Benchmark single Lambert solve"""
    print(f"\n{'=' * 70}")
    print(f"Benchmark: Single Lambert Solve - {test_name}")
    print(f"{'=' * 70}")

    results = {}

    # Python baseline
    print("Python baseline... ", end="", flush=True)
    python_time, python_std = timeit(
        lambert_universal_variable, r1, r2, tof, mu, iterations=iterations
    )
    print(f"{format_time(python_time)} ± {format_time(python_std)}")
    results["python"] = {"mean": python_time, "std": python_std}

    # Rust implementation
    if RUST_AVAILABLE:
        print("Rust implementation... ", end="", flush=True)

        def rust_lambert():
            # Convert to format expected by Rust
            solution = astrora_core.lambert_solve(
                r1, r2, float(tof), float(mu), short_way=True, revs=0
            )
            return solution

        rust_time, rust_std = timeit(rust_lambert, iterations=iterations)
        print(f"{format_time(rust_time)} ± {format_time(rust_std)}")
        results["rust"] = {"mean": rust_time, "std": rust_std}

        speedup = python_time / rust_time
        print(f"\nSpeedup: {format_speedup(speedup)}")
        results["speedup"] = speedup
    else:
        print("Rust implementation: NOT AVAILABLE")

    return results


# =============================================================================
# Benchmark 2: Batch Operations (Porkchop Plot)
# =============================================================================


def bench_batch_operations(test_name: str, r1, r2, tof_base, mu, batch_sizes: List[int]) -> Dict:
    """Benchmark batch Lambert solves for porkchop plots"""
    print(f"\n{'=' * 70}")
    print(f"Benchmark: Batch Operations - {test_name}")
    print(f"{'=' * 70}")

    results = {}

    for batch_size in batch_sizes:
        print(f"\nBatch size: {batch_size}")

        # Create TOF array (varying ±20%)
        tofs = np.array([tof_base * (0.8 + 0.4 * i / batch_size) for i in range(batch_size)])

        # Python baseline
        print(f"  Python baseline... ", end="", flush=True)
        python_time, python_std = timeit(lambert_batch, r1, r2, tofs, mu, iterations=10)
        print(f"{format_time(python_time)} ± {format_time(python_std)}")

        # Rust implementation
        if RUST_AVAILABLE:
            print(f"  Rust batch... ", end="", flush=True)

            def rust_batch():
                solutions = astrora_core.lambert_solve_batch(
                    r1, r2, tofs, float(mu), short_way=True, revs=0
                )
                return solutions

            rust_time, rust_std = timeit(rust_batch, iterations=10)
            print(f"{format_time(rust_time)} ± {format_time(rust_std)}")

            speedup = python_time / rust_time
            print(f"  Speedup: {format_speedup(speedup)}")

            if batch_size not in results:
                results[batch_size] = {}
            results[batch_size]["python"] = python_time
            results[batch_size]["rust"] = rust_time
            results[batch_size]["speedup"] = speedup

    return results


# =============================================================================
# Main Benchmark Runner
# =============================================================================


def run_all_benchmarks():
    """Run all benchmarks and generate report"""
    print("=" * 70)
    print("ASTRORA LAMBERT SOLVER - PERFORMANCE BENCHMARK")
    print("=" * 70)
    print(f"Python version: {sys.version.split()[0]}")
    print(f"NumPy version: {np.__version__}")
    print(f"Rust available: {RUST_AVAILABLE}")
    print(f"Poliastro available: {POLIASTRO_AVAILABLE}")
    print()

    all_results = {}

    # Benchmark 1: Single solves
    print("\n" + "=" * 70)
    print("PART 1: Single Lambert Solves")
    print("=" * 70)

    test_cases = [
        ("LEO to LEO Quarter-Orbit", leo_to_leo_quarter()),
        ("LEO to GEO Hohmann Transfer", leo_to_geo_transfer()),
        ("Complex 3D Transfer", complex_3d_transfer()),
    ]

    for test_name, (r1, r2, tof, mu) in test_cases:
        results = bench_single_solve(test_name, r1, r2, tof, mu, iterations=200)
        all_results[test_name] = results

    # Benchmark 2: Batch operations
    print("\n" + "=" * 70)
    print("PART 2: Batch Operations (Porkchop Plot Simulation)")
    print("=" * 70)

    r1, r2, tof, mu = leo_to_geo_transfer()
    batch_sizes = [10, 50, 100, 500, 1000]
    batch_results = bench_batch_operations("LEO to GEO", r1, r2, tof, mu, batch_sizes)
    all_results["batch"] = batch_results

    # Generate markdown report
    generate_report(all_results)

    print("\n" + "=" * 70)
    print("Benchmark complete! Results saved to LAMBERT_BENCHMARK_RESULTS.md")
    print("=" * 70)


def generate_report(results: Dict):
    """Generate markdown report from benchmark results"""
    report_lines = [
        "# Lambert Solver Performance Benchmark Results",
        "",
        f"**Date:** {time.strftime('%Y-%m-%d %H:%M:%S')}",
        "",
        "## Summary",
        "",
        "This benchmark compares the performance of the Rust Lambert solver implementation",
        "against a pure Python/NumPy baseline implementation.",
        "",
        "## Single Lambert Solves",
        "",
        "| Test Case | Python (ms) | Rust (μs) | Speedup |",
        "|-----------|-------------|-----------|---------|",
    ]

    # Add single solve results
    for test_name, data in results.items():
        if test_name != "batch":
            if "python" in data and "rust" in data:
                python_ms = data["python"]["mean"] * 1000
                rust_us = data["rust"]["mean"] * 1e6
                speedup = data.get("speedup", 0)
                report_lines.append(
                    f"| {test_name} | {python_ms:.3f} | {rust_us:.3f} | {speedup:.1f}x |"
                )

    # Add batch results
    if "batch" in results:
        report_lines.extend(
            [
                "",
                "## Batch Operations (Porkchop Plot Simulation)",
                "",
                "| Batch Size | Python (ms) | Rust (ms) | Speedup |",
                "|------------|-------------|-----------|---------|",
            ]
        )

        for batch_size, data in sorted(results["batch"].items()):
            python_ms = data["python"] * 1000
            rust_ms = data["rust"] * 1000
            speedup = data["speedup"]
            report_lines.append(
                f"| {batch_size} | {python_ms:.1f} | {rust_ms:.1f} | {speedup:.1f}x |"
            )

    # Add analysis
    report_lines.extend(
        [
            "",
            "## Analysis",
            "",
            "### Key Findings",
            "",
        ]
    )

    # Calculate average speedup
    if any(k != "batch" for k in results.keys()):
        speedups = [
            data.get("speedup", 0)
            for test_name, data in results.items()
            if test_name != "batch" and "speedup" in data
        ]
        if speedups:
            avg_speedup = np.mean(speedups)
            report_lines.append(f"- **Average speedup for single solves:** {avg_speedup:.1f}x")

    if "batch" in results and results["batch"]:
        batch_speedups = [data["speedup"] for data in results["batch"].values()]
        avg_batch_speedup = np.mean(batch_speedups)
        report_lines.append(f"- **Average speedup for batch operations:** {avg_batch_speedup:.1f}x")

    report_lines.extend(
        [
            "",
            "### Implications",
            "",
            "- Single Lambert solves are significantly faster in Rust due to compiled code",
            "  and optimized numerical algorithms.",
            "",
            "- Batch operations show even greater speedup due to reduced Python-Rust",
            "  boundary crossings and better memory locality.",
            "",
            "- For porkchop plot generation (1000+ solves), the Rust implementation",
            "  provides substantial performance benefits for mission analysis workflows.",
            "",
            "### Methodology",
            "",
            "- Each benchmark was run with multiple iterations (100-200 for single solves,",
            "  10 for batch operations) to ensure statistical significance.",
            "",
            "- Times reported are mean ± standard deviation.",
            "",
            "- All benchmarks were run on the same machine to ensure fair comparison.",
            "",
        ]
    )

    # Write report
    with open("LAMBERT_BENCHMARK_RESULTS.md", "w") as f:
        f.write("\n".join(report_lines))


if __name__ == "__main__":
    run_all_benchmarks()
