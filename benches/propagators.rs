// Comprehensive benchmark suite for orbit propagators and integrators
//
// This benchmark suite evaluates:
// 1. Different integrator methods (RK4, DOPRI5, DOP853)
// 2. Various tolerance settings for adaptive integrators
// 3. Different step sizes for fixed-step integrators
// 4. Multiple propagation scenarios (Keplerian, J2, complex perturbations)
//
// Run with: cargo bench --bench propagators
// For detailed output: cargo bench --bench propagators -- --verbose

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration,
    AxisScale,
};
use hifitime::{Duration, Epoch};
use nalgebra as na;
use astrora_core::core::constants::{GM_EARTH, R_EARTH, J2_EARTH};
use astrora_core::core::state::CartesianState;
use astrora_core::propagators::perturbations::{
    j2_perturbation, propagate_j2_rk4, propagate_j2_dopri5,
};
use astrora_core::propagators::keplerian::propagate_keplerian;

/// LEO satellite initial state (ISS-like orbit)
/// Altitude: ~400 km, inclination: ~51.6°
fn leo_initial_state() -> CartesianState {
    let r_mag = R_EARTH + 400e3; // 400 km altitude
    let v_circular = (GM_EARTH / r_mag).sqrt();

    // Inclined circular orbit
    let inclination = 51.6_f64.to_radians();

    CartesianState::new(
        na::Vector3::new(r_mag * inclination.cos(), 0.0, r_mag * inclination.sin()),
        na::Vector3::new(0.0, v_circular, 0.0),
    )
}

/// GEO satellite initial state
/// Altitude: ~35,786 km (geostationary)
fn geo_initial_state() -> CartesianState {
    let r_mag = R_EARTH + 35786e3;
    let v_circular = (GM_EARTH / r_mag).sqrt();

    CartesianState::new(
        na::Vector3::new(r_mag, 0.0, 0.0),
        na::Vector3::new(0.0, v_circular, 0.0),
    )
}

/// Highly elliptical orbit (Molniya-like)
/// Perigee: 500 km, Apogee: 39,750 km, inclination: 63.4°
fn heo_initial_state() -> CartesianState {
    let r_perigee = R_EARTH + 500e3;
    let r_apogee = R_EARTH + 39750e3;
    let a = (r_perigee + r_apogee) / 2.0;
    let e = (r_apogee - r_perigee) / (r_apogee + r_perigee);

    // Velocity at perigee
    let v_perigee = ((GM_EARTH / a) * (1.0 + e) / (1.0 - e)).sqrt();

    // Inclined orbit (Molniya: 63.4° minimizes precession)
    let inclination = 63.4_f64.to_radians();

    CartesianState::new(
        na::Vector3::new(r_perigee * inclination.cos(), 0.0, r_perigee * inclination.sin()),
        na::Vector3::new(0.0, v_perigee, 0.0),
    )
}

//=============================================================================
// Benchmark 1: RK4 with Different Step Sizes
//=============================================================================

fn bench_j2_rk4_step_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("J2_RK4_Step_Sizes");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let state = leo_initial_state();
    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let duration = Duration::from_hours(24.0); // 1 day propagation

    // Test different numbers of steps (which determines step size)
    // Fewer steps = larger step size = faster but less accurate
    // More steps = smaller step size = slower but more accurate
    let step_counts = vec![
        100,   // ~864 seconds per step (14.4 minutes)
        500,   // ~173 seconds per step (2.88 minutes)
        1000,  // ~86 seconds per step (1.44 minutes)
        2000,  // ~43 seconds per step
        5000,  // ~17 seconds per step
        10000, // ~8.6 seconds per step
    ];

    for &steps in &step_counts {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_steps", steps)),
            &steps,
            |b, &steps| {
                b.iter(|| {
                    black_box(propagate_j2_rk4(
                        black_box(&state.position()),
                        black_box(&state.velocity()),
                        black_box(duration.to_seconds()),
                        black_box(GM_EARTH),
                        black_box(J2_EARTH),
                        black_box(R_EARTH),
                        black_box(Some(steps)),
                    ))
                })
            },
        );
    }

    group.finish();
}

//=============================================================================
// Benchmark 2: DOPRI5 with Different Tolerances
//=============================================================================

fn bench_j2_dopri5_tolerances(c: &mut Criterion) {
    let mut group = c.benchmark_group("J2_DOPRI5_Tolerances");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let state = leo_initial_state();
    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let duration = Duration::from_hours(24.0);

    // Test different tolerance settings
    // Looser tolerance = larger steps = faster but less accurate
    // Tighter tolerance = smaller steps = slower but more accurate
    let tolerances = vec![
        1e-4,  // Very loose (quick but inaccurate)
        1e-6,  // Loose (fast, reasonable for preliminary analysis)
        1e-8,  // Default (good balance)
        1e-10, // Tight (high accuracy, slower)
        1e-12, // Very tight (maximum precision, much slower)
    ];

    for &tol in &tolerances {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("tol_{:.0e}", tol)),
            &tol,
            |b, &tol| {
                b.iter(|| {
                    black_box(propagate_j2_dopri5(
                        black_box(&state.position()),
                        black_box(&state.velocity()),
                        black_box(duration.to_seconds()),
                        black_box(GM_EARTH),
                        black_box(J2_EARTH),
                        black_box(R_EARTH),
                        black_box(Some(tol)),
                    ))
                })
            },
        );
    }

    group.finish();
}

//=============================================================================
// Benchmark 3: Integrator Method Comparison (RK4 vs DOPRI5)
//=============================================================================

fn bench_integrator_comparison_leo(c: &mut Criterion) {
    let mut group = c.benchmark_group("Integrator_Comparison_LEO");

    let state = leo_initial_state();
    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let duration = Duration::from_hours(24.0);

    // RK4 with moderate step count (balanced)
    group.bench_function("RK4_1000steps", |b| {
        b.iter(|| {
            black_box(propagate_j2_rk4(
                black_box(&state),
                black_box(epoch),
                black_box(duration),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(1000),
            ))
        })
    });

    // RK4 with high step count (accurate)
    group.bench_function("RK4_10000steps", |b| {
        b.iter(|| {
            black_box(propagate_j2_rk4(
                black_box(&state),
                black_box(epoch),
                black_box(duration),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(10000),
            ))
        })
    });

    // DOPRI5 with default tolerance
    group.bench_function("DOPRI5_tol_1e-8", |b| {
        b.iter(|| {
            black_box(propagate_j2_dopri5(
                black_box(&state.position()),
                black_box(&state.velocity()),
                black_box(duration.to_seconds()),
                black_box(GM_EARTH),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(Some(1e-8)),
            ))
        })
    });

    // DOPRI5 with tight tolerance
    group.bench_function("DOPRI5_tol_1e-10", |b| {
        b.iter(|| {
            black_box(propagate_j2_dopri5(
                black_box(&state.position()),
                black_box(&state.velocity()),
                black_box(duration.to_seconds()),
                black_box(GM_EARTH),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(Some(1e-10)),
            ))
        })
    });

    group.finish();
}

//=============================================================================
// Benchmark 4: Orbit Type Comparison
//=============================================================================

fn bench_orbit_types_dopri5(c: &mut Criterion) {
    let mut group = c.benchmark_group("Orbit_Types_DOPRI5");

    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let duration = Duration::from_hours(24.0);
    let tol = 1e-8;

    // LEO: Fast orbital period (~90 min), strong J2 effect
    group.bench_function("LEO", |b| {
        let state = leo_initial_state();
        b.iter(|| {
            black_box(propagate_j2_dopri5(
                black_box(&state.position()),
                black_box(&state.velocity()),
                black_box(duration.to_seconds()),
                black_box(GM_EARTH),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(Some(tol)),
            ))
        })
    });

    // GEO: Slow orbital period (~24 hours), weak J2 effect
    group.bench_function("GEO", |b| {
        let state = geo_initial_state();
        b.iter(|| {
            black_box(propagate_j2_dopri5(
                black_box(&state.position()),
                black_box(&state.velocity()),
                black_box(duration.to_seconds()),
                black_box(GM_EARTH),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(Some(tol)),
            ))
        })
    });

    // HEO: Variable speed, complex dynamics
    group.bench_function("HEO_Molniya", |b| {
        let state = heo_initial_state();
        b.iter(|| {
            black_box(propagate_j2_dopri5(
                black_box(&state.position()),
                black_box(&state.velocity()),
                black_box(duration.to_seconds()),
                black_box(GM_EARTH),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(Some(tol)),
            ))
        })
    });

    group.finish();
}

//=============================================================================
// Benchmark 5: Propagation Duration Scaling
//=============================================================================

fn bench_propagation_durations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Propagation_Durations");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let state = leo_initial_state();
    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let tol = 1e-8;

    // Test different propagation durations
    let durations_hours = vec![1.0, 6.0, 12.0, 24.0, 72.0]; // 1 hour to 3 days

    for &hours in &durations_hours {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}h", hours)),
            &hours,
            |b, &hours| {
                let duration = Duration::from_hours(hours);
                b.iter(|| {
                    black_box(propagate_j2_dopri5(
                        black_box(&state.position()),
                        black_box(&state.velocity()),
                        black_box(duration.to_seconds()),
                        black_box(GM_EARTH),
                        black_box(J2_EARTH),
                        black_box(R_EARTH),
                        black_box(Some(tol)),
                    ))
                })
            },
        );
    }

    group.finish();
}

//=============================================================================
// Benchmark 6: Keplerian vs J2 Propagation
//=============================================================================

fn bench_keplerian_vs_j2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Keplerian_vs_J2");

    let state = leo_initial_state();
    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let duration = Duration::from_hours(24.0);

    // Keplerian (two-body only, fast analytic solution)
    group.bench_function("Keplerian", |b| {
        b.iter(|| {
            black_box(propagate_keplerian(
                black_box(&state),
                black_box(epoch),
                black_box(duration),
            ))
        })
    });

    // J2 with RK4 (numerical integration with perturbation)
    group.bench_function("J2_RK4_1000steps", |b| {
        b.iter(|| {
            black_box(propagate_j2_rk4(
                black_box(&state),
                black_box(epoch),
                black_box(duration),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(1000),
            ))
        })
    });

    // J2 with DOPRI5 (adaptive integration)
    group.bench_function("J2_DOPRI5_tol_1e-8", |b| {
        b.iter(|| {
            black_box(propagate_j2_dopri5(
                black_box(&state.position()),
                black_box(&state.velocity()),
                black_box(duration.to_seconds()),
                black_box(GM_EARTH),
                black_box(J2_EARTH),
                black_box(R_EARTH),
                black_box(Some(1e-8)),
            ))
        })
    });

    group.finish();
}

//=============================================================================
// Benchmark 7: Tolerance vs Accuracy Trade-off
//=============================================================================

fn bench_tolerance_accuracy_tradeoff(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tolerance_Accuracy_Tradeoff");
    group.sample_size(20); // Reduce sample size for thorough benchmarks

    let state = leo_initial_state();
    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let duration = Duration::from_hours(24.0);

    // Range of tolerances to test
    let tolerances = vec![
        1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12,
    ];

    for &tol in &tolerances {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:.0e}", tol)),
            &tol,
            |b, &tol| {
                b.iter(|| {
                    black_box(propagate_j2_dopri5(
                        black_box(&state.position()),
                        black_box(&state.velocity()),
                        black_box(duration.to_seconds()),
                        black_box(GM_EARTH),
                        black_box(J2_EARTH),
                        black_box(R_EARTH),
                        black_box(Some(tol)),
                    ))
                })
            },
        );
    }

    group.finish();
}

//=============================================================================
// Benchmark 8: Step Size vs Performance for RK4
//=============================================================================

fn bench_rk4_step_size_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("RK4_Step_Size_Performance");
    group.sample_size(20);

    let state = leo_initial_state();
    let epoch = Epoch::from_gregorian_utc_hms(2025, 1, 1, 0, 0, 0);
    let duration = Duration::from_hours(24.0);

    // Test a wide range of step counts
    let step_counts = vec![
        50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000,
    ];

    for &steps in &step_counts {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}", steps)),
            &steps,
            |b, &steps| {
                b.iter(|| {
                    black_box(propagate_j2_rk4(
                        black_box(&state.position()),
                        black_box(&state.velocity()),
                        black_box(duration.to_seconds()),
                        black_box(GM_EARTH),
                        black_box(J2_EARTH),
                        black_box(R_EARTH),
                        black_box(Some(steps)),
                    ))
                })
            },
        );
    }

    group.finish();
}

// Configure criterion groups
criterion_group!(
    benches,
    bench_j2_rk4_step_sizes,
    bench_j2_dopri5_tolerances,
    bench_integrator_comparison_leo,
    bench_orbit_types_dopri5,
    bench_propagation_durations,
    bench_keplerian_vs_j2,
    bench_tolerance_accuracy_tradeoff,
    bench_rk4_step_size_performance,
);

criterion_main!(benches);
