//! Simple profiling harness for Lambert solver
//!
//! This binary runs the Lambert solver many times for profiling purposes.
//! Run with: cargo flamegraph --example profile_lambert --no-default-features
//!
//! Or build and run directly:
//!   cargo build --release --example profile_lambert --no-default-features
//!   cargo flamegraph --no-default-features -- target/release/examples/profile_lambert

use nalgebra::Vector3;
use astrora_core::core::constants::{GM_EARTH, GM_SUN};
use astrora_core::maneuvers::{Lambert, TransferKind};
use std::f64::consts::PI;
use std::time::Instant;

fn main() {
    println!("Lambert Solver Profiling Harness");
    println!("=================================\n");

    // Test Case 1: LEO to GEO transfer (common scenario)
    println!("Running LEO to GEO transfers...");
    let start = Instant::now();
    let mut total_dv = 0.0;

    for i in 0..10_000 {
        let r_leo = 7000e3 + (i as f64 * 10.0); // Slight variation
        let r_geo = 42164e3;

        let r1 = Vector3::new(r_leo, 0.0, 0.0);
        let r2 = Vector3::new(0.0, r_geo, 0.0);

        let a_transfer = (r_leo + r_geo) / 2.0;
        let tof = PI * (a_transfer.powi(3) / GM_EARTH).sqrt();

        match Lambert::solve(r1, r2, tof, GM_EARTH, TransferKind::Auto, 0) {
            Ok(solution) => {
                // Calculate delta-v (for realistic computation)
                let v_circular_leo = (GM_EARTH / r_leo).sqrt();
                let v_leo = Vector3::new(0.0, v_circular_leo, 0.0);
                total_dv += (solution.v1 - v_leo).norm();
            }
            Err(_) => {}
        }
    }

    let elapsed = start.elapsed();
    println!("  Completed 10,000 solves in {:.2?}", elapsed);
    println!("  Average: {:.2?} per solve", elapsed / 10_000);
    println!("  Total Δv: {:.2} m/s\n", total_dv);

    // Test Case 2: Complex 3D transfers
    println!("Running complex 3D transfers...");
    let start = Instant::now();

    for i in 0..5_000 {
        let angle = (i as f64) * 0.01;
        let r1 = Vector3::new(
            5000e3 * angle.cos(),
            10000e3 * angle.sin(),
            2100e3
        );
        let r2 = Vector3::new(
            -14600e3 * angle.sin(),
            2500e3 * angle.cos(),
            7000e3
        );
        let tof = 3600.0 + (i as f64);

        let _ = Lambert::solve(r1, r2, tof, GM_EARTH, TransferKind::ShortWay, 0);
    }

    let elapsed = start.elapsed();
    println!("  Completed 5,000 solves in {:.2?}", elapsed);
    println!("  Average: {:.2?} per solve", elapsed / 5_000);

    // Test Case 3: Interplanetary transfers
    println!("\nRunning interplanetary transfers...");
    let start = Instant::now();

    for i in 0..1_000 {
        let r_earth = 149.6e9;
        let r_mars = 227.9e9;

        let angle = (i as f64) * 0.1;
        let r1 = Vector3::new(r_earth * angle.cos(), r_earth * angle.sin(), 0.0);
        let r2 = Vector3::new(r_mars * (angle + PI/4.0).cos(), r_mars * (angle + PI/4.0).sin(), 0.0);

        let a_transfer = (r_earth + r_mars) / 2.0;
        let tof = PI * (a_transfer.powi(3) / GM_SUN).sqrt();

        let _ = Lambert::solve(r1, r2, tof, GM_SUN, TransferKind::Auto, 0);
    }

    let elapsed = start.elapsed();
    println!("  Completed 1,000 solves in {:.2?}", elapsed);
    println!("  Average: {:.2?} per solve\n", elapsed / 1_000);

    println!("Profiling complete!");
}
