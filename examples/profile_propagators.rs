//! Simple profiling harness for orbital propagators
//!
//! This binary runs propagators many times for profiling purposes.
//! Run with: cargo flamegraph --example profile_propagators --no-default-features

use nalgebra::Vector3;
use astrora_core::core::constants::{GM_EARTH, R_EARTH, J2_EARTH};
use astrora_core::propagators::perturbations::propagate_j2_rk4;
use std::time::Instant;

fn main() {
    println!("Orbital Propagator Profiling Harness");
    println!("=====================================\n");

    // LEO satellite initial state (ISS-like orbit)
    // Altitude: ~400 km, inclination: ~51.6°
    let r_mag = R_EARTH + 400e3;
    let v_circular = (GM_EARTH / r_mag).sqrt();

    let r0 = Vector3::new(r_mag, 0.0, 0.0);
    let v0 = Vector3::new(0.0, v_circular, 0.0);

    // Test Case 1: Short propagations (1 orbit = ~90 minutes)
    println!("Running short propagations (1 orbit)...");
    let start = Instant::now();
    let dt = 90.0 * 60.0; // 90 minutes in seconds

    for i in 0..5_000 {
        // Slight variations to prevent over-optimization
        let r = r0 + Vector3::new(i as f64, 0.0, 0.0);
        let _ = propagate_j2_rk4(&r, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(100));
    }

    let elapsed = start.elapsed();
    println!("  Completed 5,000 propagations in {:.2?}", elapsed);
    println!("  Average: {:.2?} per propagation", elapsed / 5_000);

    // Test Case 2: Medium propagations (1 day)
    println!("\nRunning medium propagations (1 day)...");
    let start = Instant::now();
    let dt = 24.0 * 3600.0; // 1 day in seconds

    for i in 0..1_000 {
        let r = r0 + Vector3::new(i as f64 * 10.0, 0.0, 0.0);
        let _ = propagate_j2_rk4(&r, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(500));
    }

    let elapsed = start.elapsed();
    println!("  Completed 1,000 propagations in {:.2?}", elapsed);
    println!("  Average: {:.2?} per propagation", elapsed / 1_000);

    // Test Case 3: Long propagations with many steps
    println!("\nRunning long propagations (1 week, high accuracy)...");
    let start = Instant::now();
    let dt = 7.0 * 24.0 * 3600.0; // 1 week in seconds

    for i in 0..200 {
        let r = r0 + Vector3::new(i as f64 * 100.0, 0.0, 0.0);
        let _ = propagate_j2_rk4(&r, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(2000));
    }

    let elapsed = start.elapsed();
    println!("  Completed 200 propagations in {:.2?}", elapsed);
    println!("  Average: {:.2?} per propagation", elapsed / 200);

    // Test Case 4: High-step-count test (maximum accuracy)
    println!("\nRunning high-accuracy propagations (1 orbit, 10,000 steps)...");
    let start = Instant::now();
    let dt = 90.0 * 60.0; // 90 minutes

    for i in 0..500 {
        let r = r0 + Vector3::new(i as f64, 0.0, 0.0);
        let _ = propagate_j2_rk4(&r, &v0, dt, GM_EARTH, J2_EARTH, R_EARTH, Some(10_000));
    }

    let elapsed = start.elapsed();
    println!("  Completed 500 propagations in {:.2?}", elapsed);
    println!("  Average: {:.2?} per propagation", elapsed / 500);

    println!("\nProfiling complete!");
}
