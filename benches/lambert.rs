// Comprehensive benchmark suite for Lambert problem solver
//
// This benchmark suite evaluates the performance of the Lambert solver for:
// 1. Single Lambert solutions (various transfer types)
// 2. Batch operations (for porkchop plot generation)
// 3. Multi-revolution solutions
// 4. Different orbit types and transfer scenarios
//
// Run with: cargo bench --bench lambert
// For comparison with Python: python benchmarks/lambert_benchmark.py

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration,
    AxisScale,
};
use nalgebra::Vector3;
use astrora_core::core::constants::{GM_EARTH, GM_SUN};
use astrora_core::maneuvers::{Lambert, TransferKind};
use std::f64::consts::PI;

//=============================================================================
// Helper Functions to Create Test Cases
//=============================================================================

/// LEO to LEO transfer (quarter orbit)
fn leo_to_leo_quarter() -> (Vector3<f64>, Vector3<f64>, f64, f64) {
    let r_leo: f64 = 7000e3; // 7000 km altitude
    let r1 = Vector3::new(r_leo, 0.0, 0.0);
    let r2 = Vector3::new(0.0, r_leo, 0.0);

    // Quarter orbit time
    let period = 2.0 * PI * (r_leo.powi(3) / GM_EARTH).sqrt();
    let tof = period / 4.0;

    (r1, r2, tof, GM_EARTH)
}

/// LEO to GEO transfer (Hohmann-like)
fn leo_to_geo_transfer() -> (Vector3<f64>, Vector3<f64>, f64, f64) {
    let r_leo: f64 = 7000e3;
    let r_geo: f64 = 42164e3; // GEO altitude

    let r1 = Vector3::new(r_leo, 0.0, 0.0);
    let r2 = Vector3::new(0.0, r_geo, 0.0);

    // Approximate Hohmann transfer time
    let a_transfer: f64 = (r_leo + r_geo) / 2.0;
    let tof = PI * (a_transfer.powi(3) / GM_EARTH).sqrt();

    (r1, r2, tof, GM_EARTH)
}

/// LEO to MEO transfer
fn leo_to_meo_transfer() -> (Vector3<f64>, Vector3<f64>, f64, f64) {
    let r_leo: f64 = 7000e3;
    let r_meo: f64 = 20000e3; // MEO altitude

    let r1 = Vector3::new(r_leo, 0.0, 0.0);
    let r2 = Vector3::new(0.0, r_meo, 0.0);

    let a_transfer: f64 = (r_leo + r_meo) / 2.0;
    let tof = PI * (a_transfer.powi(3) / GM_EARTH).sqrt();

    (r1, r2, tof, GM_EARTH)
}

/// Complex 3D transfer
fn complex_3d_transfer() -> (Vector3<f64>, Vector3<f64>, f64, f64) {
    let r1 = Vector3::new(5000e3, 10000e3, 2100e3);
    let r2 = Vector3::new(-14600e3, 2500e3, 7000e3);
    let tof = 3600.0; // 1 hour

    (r1, r2, tof, GM_EARTH)
}

/// Earth to Mars-like interplanetary transfer (simplified)
fn interplanetary_like_transfer() -> (Vector3<f64>, Vector3<f64>, f64, f64) {
    let r_earth: f64 = 149.6e9; // ~1 AU
    let r_mars: f64 = 227.9e9;  // ~1.52 AU

    let r1 = Vector3::new(r_earth, 0.0, 0.0);
    let r2 = Vector3::new(0.0, r_mars, 0.0);

    // Approximate Hohmann transfer time (~8.5 months)
    let a_transfer: f64 = (r_earth + r_mars) / 2.0;
    let tof = PI * (a_transfer.powi(3) / GM_SUN).sqrt();

    (r1, r2, tof, GM_SUN)
}

//=============================================================================
// Benchmark 1: Single Lambert Solutions (Various Transfer Types)
//=============================================================================

fn bench_lambert_single_solves(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lambert_Single_Solves");

    // LEO to LEO (fast, simple circular transfer)
    group.bench_function("LEO_to_LEO_quarter", |b| {
        let (r1, r2, tof, mu) = leo_to_leo_quarter();
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::Auto),
                black_box(0),
            ))
        })
    });

    // LEO to GEO (Hohmann-like transfer)
    group.bench_function("LEO_to_GEO_Hohmann", |b| {
        let (r1, r2, tof, mu) = leo_to_geo_transfer();
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::Auto),
                black_box(0),
            ))
        })
    });

    // LEO to MEO transfer
    group.bench_function("LEO_to_MEO", |b| {
        let (r1, r2, tof, mu) = leo_to_meo_transfer();
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::Auto),
                black_box(0),
            ))
        })
    });

    // Complex 3D transfer (Vallado example)
    group.bench_function("Complex_3D_Vallado", |b| {
        let (r1, r2, tof, mu) = complex_3d_transfer();
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::ShortWay),
                black_box(0),
            ))
        })
    });

    // Interplanetary-like transfer
    group.bench_function("Interplanetary_Like", |b| {
        let (r1, r2, tof, mu) = interplanetary_like_transfer();
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::Auto),
                black_box(0),
            ))
        })
    });

    group.finish();
}

//=============================================================================
// Benchmark 2: Batch Operations (Porkchop Plot Simulation)
//=============================================================================

fn bench_lambert_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lambert_Batch_Operations");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let (r1, r2, tof_base, mu) = leo_to_geo_transfer();

    // Test different batch sizes (simulating porkchop plots of varying resolution)
    let batch_sizes = vec![10, 50, 100, 500, 1000];

    for &size in &batch_sizes {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_solves", size)),
            &size,
            |b, &size| {
                // Create array of TOFs (varying ±20% from base)
                let tofs: Vec<f64> = (0..size)
                    .map(|i| {
                        let factor = 0.8 + 0.4 * (i as f64 / size as f64);
                        tof_base * factor
                    })
                    .collect();

                b.iter(|| {
                    black_box(Lambert::solve_batch(
                        black_box(r1),
                        black_box(r2),
                        black_box(&tofs),
                        black_box(mu),
                        black_box(TransferKind::Auto),
                        black_box(0),
                    ))
                })
            },
        );
    }

    group.finish();
}

//=============================================================================
// Benchmark 3: Parallel Batch Operations (Porkchop Plot with Parallelization)
//=============================================================================

fn bench_lambert_parallel_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lambert_Parallel_Batch");
    group.sample_size(10); // Reduce sample size for expensive benchmarks

    let (r1_base, r2_base, tof_base, mu) = leo_to_geo_transfer();

    // Simulate realistic porkchop plot grid sizes
    let grid_sizes = vec![100, 500, 1000, 2500];

    for &size in &grid_sizes {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}_grid", size, size)),
            &size,
            |b, &size| {
                // Create grids of positions and times
                let mut r1s = Vec::with_capacity(size);
                let mut r2s = Vec::with_capacity(size);
                let mut tofs = Vec::with_capacity(size);

                for i in 0..size {
                    let factor_r1 = 0.9 + 0.2 * (i as f64 / size as f64);
                    let factor_r2 = 0.9 + 0.2 * ((size - i) as f64 / size as f64);
                    let factor_tof = 0.8 + 0.4 * (i as f64 / size as f64);

                    r1s.push(r1_base * factor_r1);
                    r2s.push(r2_base * factor_r2);
                    tofs.push(tof_base * factor_tof);
                }

                b.iter(|| {
                    black_box(Lambert::solve_batch_parallel(
                        black_box(&r1s),
                        black_box(&r2s),
                        black_box(&tofs),
                        black_box(mu),
                        black_box(TransferKind::Auto),
                        black_box(0),
                    ))
                })
            },
        );
    }

    group.finish();
}

//=============================================================================
// Benchmark 4: Multi-Revolution Solutions
//=============================================================================

fn bench_lambert_multi_revolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lambert_Multi_Revolution");

    let r_leo: f64 = 7000e3;
    let r1 = Vector3::new(r_leo, 0.0, 0.0);
    let r2 = Vector3::new(0.0, r_leo, 0.0);
    let period: f64 = 2.0 * PI * (r_leo.powi(3) / GM_EARTH).sqrt();

    // Zero revolutions (baseline)
    group.bench_function("0_revolutions", |b| {
        let tof = period / 4.0;
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(GM_EARTH),
                black_box(TransferKind::Auto),
                black_box(0),
            ))
        })
    });

    // 1 revolution
    group.bench_function("1_revolution", |b| {
        let tof = 4.5 * period;
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(GM_EARTH),
                black_box(TransferKind::Auto),
                black_box(1),
            ))
        })
    });

    // 2 revolutions
    group.bench_function("2_revolutions", |b| {
        let tof = 9.0 * period;
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(GM_EARTH),
                black_box(TransferKind::Auto),
                black_box(2),
            ))
        })
    });

    group.finish();
}

//=============================================================================
// Benchmark 5: Transfer Direction Comparison
//=============================================================================

fn bench_lambert_transfer_directions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lambert_Transfer_Directions");

    let (r1, r2, tof, mu) = leo_to_leo_quarter();

    // Auto direction
    group.bench_function("Auto", |b| {
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::Auto),
                black_box(0),
            ))
        })
    });

    // Short-way
    group.bench_function("ShortWay", |b| {
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::ShortWay),
                black_box(0),
            ))
        })
    });

    // Long-way
    group.bench_function("LongWay", |b| {
        b.iter(|| {
            black_box(Lambert::solve(
                black_box(r1),
                black_box(r2),
                black_box(tof),
                black_box(mu),
                black_box(TransferKind::LongWay),
                black_box(0),
            ))
        })
    });

    group.finish();
}

//=============================================================================
// Benchmark 6: Porkchop Plot Realistic Scenario
//=============================================================================

fn bench_lambert_porkchop_realistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lambert_Porkchop_Realistic");
    group.sample_size(10);

    // Simulate a 50x50 porkchop plot (2500 Lambert solves)
    // This is typical for mission analysis tools
    let grid_size = 50;
    let total_solves = grid_size * grid_size;

    let (r1_base, r2_base, tof_base, mu) = leo_to_geo_transfer();

    group.bench_function(format!("{}x{}_porkchop", grid_size, grid_size), |b| {
        // Pre-generate the grid
        let mut r1s = Vec::with_capacity(total_solves);
        let mut r2s = Vec::with_capacity(total_solves);
        let mut tofs = Vec::with_capacity(total_solves);

        for i in 0..grid_size {
            for j in 0..grid_size {
                // Vary departure position (r1) with time
                let factor_r1 = 0.95 + 0.1 * (i as f64 / grid_size as f64);
                // Vary arrival position (r2) with time
                let factor_r2 = 0.95 + 0.1 * (j as f64 / grid_size as f64);
                // Vary TOF across the grid
                let factor_tof = 0.7 + 0.6 * ((i + j) as f64 / (2.0 * grid_size as f64));

                r1s.push(r1_base * factor_r1);
                r2s.push(r2_base * factor_r2);
                tofs.push(tof_base * factor_tof);
            }
        }

        b.iter(|| {
            black_box(Lambert::solve_batch_parallel(
                black_box(&r1s),
                black_box(&r2s),
                black_box(&tofs),
                black_box(mu),
                black_box(TransferKind::Auto),
                black_box(0),
            ))
        })
    });

    group.finish();
}

//=============================================================================
// Configure criterion groups
//=============================================================================

criterion_group!(
    benches,
    bench_lambert_single_solves,
    bench_lambert_batch_operations,
    bench_lambert_parallel_batch,
    bench_lambert_multi_revolution,
    bench_lambert_transfer_directions,
    bench_lambert_porkchop_realistic,
);

criterion_main!(benches);
