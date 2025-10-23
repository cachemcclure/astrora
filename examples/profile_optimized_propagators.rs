//! Profiling harness for optimized (static) propagators
//!
//! This example is designed to be profiled with cargo-flamegraph to identify
//! any remaining performance bottlenecks in the zero-allocation propagators.
//!
//! Run with:
//! ```bash
//! cargo flamegraph --example profile_optimized_propagators --no-default-features \
//!   --output optimized_propagators_flamegraph.svg
//! ```

use astrora_core::core::constants::{GM_EARTH, R_EARTH, J2_EARTH};
use astrora_core::core::integrators_static::{StateVector6, propagate_rk4_final_only};
use astrora_core::propagators::perturbations_static::j2_dynamics;

fn main() {
    println!("Optimized Propagator Profiling Harness");
    println!("========================================\n");

    // Initial state: circular LEO orbit
    let r0 = 7000e3;
    let v0 = (GM_EARTH / r0).sqrt();
    let state0 = StateVector6::new(r0, 0.0, 0.0, 0.0, v0, 0.0);

    // Orbital period
    let period = 2.0 * std::f64::consts::PI * (r0.powi(3) / GM_EARTH).sqrt();

    println!("Running high-accuracy J2 propagations...");
    println!("  Orbit: LEO circular at {} km", r0 / 1000.0);
    println!("  Period: {:.2} seconds ({:.2} minutes)", period, period / 60.0);
    println!();

    // J2 dynamics
    let dynamics = j2_dynamics(GM_EARTH, J2_EARTH, R_EARTH);

    // Run many propagations for profiling
    let num_propagations = 10_000;
    let steps_per_orbit = 1000;

    println!("Propagating {} orbits with {} steps each...", num_propagations, steps_per_orbit);

    let start = std::time::Instant::now();

    for i in 0..num_propagations {
        let state_final = propagate_rk4_final_only(
            &dynamics,
            0.0,
            &state0,
            period,
            steps_per_orbit,
        );

        // Prevent optimization from eliminating the computation
        if i == 0 {
            let r_final = state_final.fixed_rows::<3>(0).norm();
            println!("  First orbit final radius: {:.3} km", r_final / 1000.0);
        }
    }

    let elapsed = start.elapsed();
    let total_steps = num_propagations * steps_per_orbit;

    println!("\n✅ Completed {} propagations in {:.3} seconds", num_propagations, elapsed.as_secs_f64());
    println!("   Total integration steps: {}", total_steps);
    println!("   Average time per propagation: {:.3} µs", elapsed.as_micros() as f64 / num_propagations as f64);
    println!("   Average time per step: {:.3} ns", elapsed.as_nanos() as f64 / total_steps as f64);
    println!();
    println!("Profiling complete. Check the flamegraph for hotspots.");
}
