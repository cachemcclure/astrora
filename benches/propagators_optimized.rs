//! Benchmarks comparing optimized (static) vs standard (dynamic) propagators
//!
//! This benchmark suite measures the performance improvement from Phase 1 optimizations:
//! - Stack-allocated vectors (SVector) vs heap-allocated (DVector)
//! - Zero-allocation integrators vs standard allocating integrators
//!
//! Expected speedup: 3-5x for standard 6-DOF propagation

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use astrora_core::core::constants::{GM_EARTH, R_EARTH, J2_EARTH};
use astrora_core::core::integrators_static::{StateVector6, propagate_rk4_final_only};
use astrora_core::propagators::perturbations_static::{j2_dynamics, j2_perturbation_static, Vector3Static};
use astrora_core::core::numerical::rk4_step;
use astrora_core::propagators::perturbations::j2_perturbation;
use nalgebra as na;

// ============================================================================
// Helper Functions
// ============================================================================

/// Create initial state for circular LEO orbit
fn circular_leo_state_static() -> StateVector6 {
    let r0 = 7000e3; // 7000 km altitude
    let v0 = (GM_EARTH / r0).sqrt();
    StateVector6::new(r0, 0.0, 0.0, 0.0, v0, 0.0)
}

fn circular_leo_state_dynamic() -> na::DVector<f64> {
    let r0 = 7000e3;
    let v0 = (GM_EARTH / r0).sqrt();
    na::DVector::from_vec(vec![r0, 0.0, 0.0, 0.0, v0, 0.0])
}

/// Two-body dynamics (dynamic vectors)
fn two_body_dynamics_dynamic(
    _t: f64,
    state: &na::DVector<f64>,
) -> na::DVector<f64> {
    let x = state[0];
    let y = state[1];
    let z = state[2];
    let vx = state[3];
    let vy = state[4];
    let vz = state[5];

    let r_vec = na::Vector3::new(x, y, z);
    let r = r_vec.norm();
    let a = -GM_EARTH / (r * r * r) * r_vec;

    na::DVector::from_vec(vec![vx, vy, vz, a[0], a[1], a[2]])
}

/// Two-body dynamics (static vectors)
fn two_body_dynamics_static(_t: f64, state: &StateVector6) -> StateVector6 {
    let x = state[0];
    let y = state[1];
    let z = state[2];
    let vx = state[3];
    let vy = state[4];
    let vz = state[5];

    let r_vec = Vector3Static::new(x, y, z);
    let r = r_vec.norm();
    let a = -GM_EARTH / (r * r * r) * r_vec;

    StateVector6::new(vx, vy, vz, a[0], a[1], a[2])
}

/// J2-perturbed dynamics (dynamic vectors)
fn j2_dynamics_dynamic(
    _t: f64,
    state: &na::DVector<f64>,
) -> na::DVector<f64> {
    let x = state[0];
    let y = state[1];
    let z = state[2];
    let vx = state[3];
    let vy = state[4];
    let vz = state[5];

    let r_vec = na::Vector3::new(x, y, z);
    let r = r_vec.norm();

    // Two-body acceleration
    let a_twobody = -GM_EARTH / (r * r * r) * r_vec;

    // J2 perturbation
    let a_j2 = j2_perturbation(&r_vec, GM_EARTH, J2_EARTH, R_EARTH);

    // Total acceleration
    let a_total = a_twobody + a_j2;

    na::DVector::from_vec(vec![vx, vy, vz, a_total[0], a_total[1], a_total[2]])
}

// ============================================================================
// Benchmarks
// ============================================================================

fn bench_two_body_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_body_propagation");

    // Benchmark parameters: (name, steps)
    let cases = vec![
        ("1_orbit_100steps", 100),
        ("1_orbit_1000steps", 1000),
        ("1_day_500steps", 500),
        ("1_week_2000steps", 2000),
    ];

    for (name, steps) in cases {
        // Calculate time span based on steps
        let period = 2.0 * std::f64::consts::PI * (7000e3_f64.powi(3) / GM_EARTH).sqrt();
        let t_final = match name {
            "1_orbit_100steps" | "1_orbit_1000steps" => period,
            "1_day_500steps" => 86400.0,
            "1_week_2000steps" => 604800.0,
            _ => period,
        };

        // Dynamic (heap-allocated) version
        group.bench_with_input(
            BenchmarkId::new("dynamic", name),
            &(steps, t_final),
            |b, &(steps, t_final)| {
                let state0 = circular_leo_state_dynamic();
                b.iter(|| {
                    let h = t_final / steps as f64;
                    let mut state = state0.clone();
                    let mut t = 0.0;

                    for _ in 0..steps {
                        state = rk4_step(two_body_dynamics_dynamic, t, &state, h);
                        t += h;
                    }
                    black_box(state)
                });
            },
        );

        // Static (stack-allocated) version
        group.bench_with_input(
            BenchmarkId::new("static", name),
            &(steps, t_final),
            |b, &(steps, t_final)| {
                let state0 = circular_leo_state_static();
                b.iter(|| {
                    black_box(propagate_rk4_final_only(
                        two_body_dynamics_static,
                        0.0,
                        &state0,
                        t_final,
                        steps,
                    ))
                });
            },
        );
    }

    group.finish();
}

fn bench_j2_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("j2_propagation");

    let cases = vec![
        ("1_orbit_100steps", 100),
        ("1_orbit_1000steps", 1000),
        ("1_day_500steps", 500),
    ];

    for (name, steps) in cases {
        let period = 2.0 * std::f64::consts::PI * (7000e3_f64.powi(3) / GM_EARTH).sqrt();
        let t_final = match name {
            "1_orbit_100steps" | "1_orbit_1000steps" => period,
            "1_day_500steps" => 86400.0,
            _ => period,
        };

        // Dynamic (heap-allocated) version with J2
        group.bench_with_input(
            BenchmarkId::new("dynamic", name),
            &(steps, t_final),
            |b, &(steps, t_final)| {
                let state0 = circular_leo_state_dynamic();
                b.iter(|| {
                    let h = t_final / steps as f64;
                    let mut state = state0.clone();
                    let mut t = 0.0;

                    for _ in 0..steps {
                        state = rk4_step(j2_dynamics_dynamic, t, &state, h);
                        t += h;
                    }
                    black_box(state)
                });
            },
        );

        // Static (stack-allocated) version with J2
        group.bench_with_input(
            BenchmarkId::new("static", name),
            &(steps, t_final),
            |b, &(steps, t_final)| {
                let state0 = circular_leo_state_static();
                b.iter(|| {
                    let dynamics = j2_dynamics(GM_EARTH, J2_EARTH, R_EARTH);
                    black_box(propagate_rk4_final_only(
                        dynamics,
                        0.0,
                        &state0,
                        t_final,
                        steps,
                    ))
                });
            },
        );
    }

    group.finish();
}

fn bench_perturbation_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("perturbation_functions");

    let r_dynamic = na::Vector3::new(7000e3, 0.0, 1000e3);
    let r_static = Vector3Static::new(7000e3, 0.0, 1000e3);

    // J2 perturbation comparison
    group.bench_function("j2_dynamic", |b| {
        b.iter(|| {
            black_box(j2_perturbation(&r_dynamic, GM_EARTH, J2_EARTH, R_EARTH))
        });
    });

    group.bench_function("j2_static", |b| {
        b.iter(|| {
            black_box(j2_perturbation_static(&r_static, GM_EARTH, J2_EARTH, R_EARTH))
        });
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    bench_two_body_propagation,
    bench_j2_propagation,
    bench_perturbation_functions,
);
criterion_main!(benches);
