//! Null geodesic integration in Schwarzschild spacetime
//!
//! Numerically integrates photon trajectories around non-rotating, uncharged,
//! spherically symmetric masses using the Binet equation:
//!
//! ```text
//! d²u/dφ² + u = 3GM/c² · u²    where u = 1/r
//! ```
//!
//! Rewritten as a first-order system for RK4 integration:
//! ```text
//! y₁ = u,      dy₁/dφ = y₂
//! y₂ = du/dφ,  dy₂/dφ = -y₁ + 3GM/c² · y₁²
//! ```
//!
//! # References
//! - Wald, R.M. (1984). *General Relativity*. Ch. 6.
//! - Carroll, S.M. (2004). *Spacetime and Geometry*. Ch. 5, 7.

use nalgebra as na;

use crate::core::constants::{schwarzschild_radius, C};
use crate::core::error::{PoliastroError, PoliastroResult};

/// 2-element state vector for the geodesic ODE: [u, du/dφ]
/// where u = 1/r (inverse radial coordinate in meters⁻¹).
pub type GeodesicState = na::SVector<f64, 2>;

/// Result of a Schwarzschild null geodesic integration.
#[derive(Debug, Clone, PartialEq)]
pub struct SchwarzschildGeodesicResult {
    /// Radial positions along the path (meters), one per recorded step.
    pub radii: Vec<f64>,
    /// Azimuthal angles along the path (radians).
    pub phi_values: Vec<f64>,
    /// Total deflection angle (radians) — deviation from straight-line path.
    pub deflection_angle: f64,
    /// Closest approach distance to the massive body (meters).
    pub closest_approach: f64,
    /// Whether the photon was captured (crossed the Schwarzschild radius).
    pub captured: bool,
    /// Number of integration steps actually taken.
    pub steps_taken: usize,
}

/// Null geodesic integrator for Schwarzschild spacetime.
pub struct SchwarzschildGeodesic;

impl SchwarzschildGeodesic {
    /// Binet equation right-hand side for the geodesic ODE.
    ///
    /// Given state [u, du/dφ], returns [du/dφ, d²u/dφ²]:
    /// - dy₁/dφ = y₂
    /// - dy₂/dφ = -y₁ + (3GM/c²) · y₁²
    ///
    /// The term `3GM/c² · u²` is the general-relativistic correction.
    /// Without it, the equation reduces to `d²u/dφ² + u = 0` (straight line).
    #[inline]
    pub fn binet_rhs(gm: f64, state: &GeodesicState) -> GeodesicState {
        let u = state[0];
        let du_dphi = state[1];
        let gr_factor = 3.0 * gm / (C * C);

        GeodesicState::new(du_dphi, -u + gr_factor * u * u)
    }

    /// Single RK4 step for the 2-component geodesic ODE.
    ///
    /// Standalone implementation optimized for the 2-element state vector.
    /// Uses `SVector<f64, 2>` (stack-allocated) for zero heap overhead.
    #[inline]
    pub fn rk4_step(gm: f64, _phi: f64, state: &GeodesicState, h: f64) -> GeodesicState {
        let k1 = Self::binet_rhs(gm, state);
        let k2 = Self::binet_rhs(gm, &(state + k1 * (h / 2.0)));
        let k3 = Self::binet_rhs(gm, &(state + k2 * (h / 2.0)));
        let k4 = Self::binet_rhs(gm, &(state + k3 * h));

        state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (h / 6.0)
    }

    /// Integrate a null geodesic in Schwarzschild spacetime.
    ///
    /// Given a gravitational parameter and impact parameter, numerically integrates
    /// the photon trajectory from a large initial distance through the region of
    /// strongest curvature.
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter GM of the massive body (m³/s²)
    /// * `impact_parameter` - Perpendicular distance from the asymptotic straight-line
    ///   trajectory to the body center (meters). Must be positive.
    /// * `phi_range` - Total azimuthal angle to integrate over (radians).
    ///   Use `std::f64::consts::PI` for a half-orbit (typical for deflection).
    ///   Larger values for near-capture orbits that may loop.
    /// * `num_steps` - Number of integration steps. More steps = higher accuracy.
    ///
    /// # Returns
    /// `SchwarzschildGeodesicResult` with the path, deflection angle, and metadata.
    ///
    /// # Errors
    /// - `InvalidParameter` if `gm < 0`, `impact_parameter <= 0`, or `num_steps == 0`
    /// - `GeodesicIntegrationFailure` if the integration produces NaN or diverges
    pub fn integrate(
        gm: f64,
        impact_parameter: f64,
        phi_range: f64,
        num_steps: usize,
    ) -> PoliastroResult<SchwarzschildGeodesicResult> {
        // Input validation
        if gm < 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "gm",
                gm,
                "must be non-negative",
            ));
        }
        if impact_parameter <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "impact_parameter",
                impact_parameter,
                "must be positive",
            ));
        }
        if num_steps == 0 {
            return Err(PoliastroError::invalid_parameter(
                "num_steps",
                num_steps as f64,
                "must be > 0",
            ));
        }
        if phi_range <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "phi_range",
                phi_range,
                "must be positive",
            ));
        }

        let r_s = schwarzschild_radius(gm);
        let h = phi_range / num_steps as f64;

        // Initial conditions for the Binet equation scattering problem:
        //
        // For gm = 0, the exact solution is u(φ) = sin(φ)/b, giving:
        //   u(0) = 0 (photon at r = ∞), du/dφ(0) = 1/b (approaching, u increasing)
        //   u(π/2) = 1/b (closest approach at r = b)
        //   u(π) = 0 (photon exits to r = ∞)
        //
        // With gravity (gm > 0), the trajectory bends more, and the exit
        // occurs at φ = π + δ where δ is the deflection angle.
        //
        // We start with u ≈ 0 (practically at infinity).
        let u_initial = 0.0;
        let du_dphi_initial = 1.0 / impact_parameter;

        let mut state = GeodesicState::new(u_initial, du_dphi_initial);
        let mut phi = 0.0;

        let mut radii = Vec::with_capacity(num_steps / 10 + 1);
        let mut phi_values = Vec::with_capacity(num_steps / 10 + 1);
        let mut closest_approach = f64::INFINITY;
        let mut captured = false;
        let mut steps_taken = 0;
        let mut phi_turning_point: Option<f64> = None;

        // Record initial point (at infinity — use a large representative radius)
        let r_display_max = impact_parameter * 100.0;
        radii.push(r_display_max);
        phi_values.push(phi);

        for step in 0..num_steps {
            let prev_du = state[1];
            state = Self::rk4_step(gm, phi, &state, h);
            phi += h;
            steps_taken = step + 1;

            let u = state[0];
            let du = state[1];

            // Check for numerical issues
            if !u.is_finite() || !du.is_finite() {
                return Err(PoliastroError::geodesic_integration_failure(format!(
                    "NaN or infinity at step {step}, phi = {phi:.6}"
                )));
            }

            // Detect turning point: du/dφ changes sign from positive to negative.
            // This is closest approach. Use linear interpolation for sub-step accuracy.
            if phi_turning_point.is_none() && prev_du > 0.0 && du <= 0.0 {
                // Linear interpolation: φ_turn ≈ φ_prev + h * prev_du / (prev_du - du)
                let frac = prev_du / (prev_du - du);
                phi_turning_point = Some(phi - h + h * frac);
            }

            if u > 0.0 {
                let r = 1.0 / u;

                if r < closest_approach {
                    closest_approach = r;
                }

                // Capture check: photon crosses Schwarzschild radius
                if r_s > 0.0 && r <= r_s {
                    captured = true;
                    radii.push(r);
                    phi_values.push(phi);
                    break;
                }

                // Record every 10th point to keep output manageable
                if step % 10 == 0 {
                    radii.push(r);
                    phi_values.push(phi);
                }
            } else {
                // u crossed zero — photon has exited to infinity on the far side.
                radii.push(r_display_max);
                phi_values.push(phi);
                break;
            }
        }

        // Compute deflection angle using the symmetry of the Schwarzschild problem.
        //
        // The Schwarzschild geodesic is symmetric about the turning point.
        // For a straight line (gm = 0), the turning point is at φ = π/2, and
        // the total sweep is π (from φ=0 to φ=π).
        //
        // With gravity, the turning point shifts to φ_turn > π/2, and by symmetry
        // the total sweep is 2·φ_turn. The deflection is:
        //   δ = 2·φ_turn - π
        //
        // This is much more accurate than trying to detect the exact exit angle,
        // because the turning point (du/dφ = 0) is well-defined even for tiny deflections.
        let deflection_angle = if captured {
            f64::INFINITY
        } else if let Some(phi_turn) = phi_turning_point {
            2.0 * phi_turn - std::f64::consts::PI
        } else {
            // No turning point found — photon may not have reached closest approach
            // (phi_range too small or captured before turning)
            0.0
        };

        Ok(SchwarzschildGeodesicResult {
            radii,
            phi_values,
            deflection_angle,
            closest_approach,
            captured,
            steps_taken,
        })
    }

    /// Integrate and return Cartesian coordinates for rendering.
    ///
    /// Converts the (r, φ) trajectory to (x, y, z) coordinates in the orbital
    /// plane (z = 0). The massive body is at the origin.
    ///
    /// # Arguments
    /// Same as [`integrate`](Self::integrate).
    ///
    /// # Returns
    /// Vec of [x, y, z] points in meters.
    pub fn integrate_to_cartesian(
        gm: f64,
        impact_parameter: f64,
        phi_range: f64,
        num_steps: usize,
    ) -> PoliastroResult<Vec<[f64; 3]>> {
        let result = Self::integrate(gm, impact_parameter, phi_range, num_steps)?;

        let points: Vec<[f64; 3]> = result
            .radii
            .iter()
            .zip(result.phi_values.iter())
            .map(|(&r, &phi)| [r * phi.cos(), r * phi.sin(), 0.0])
            .collect();

        Ok(points)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::{GM_SGR_A_STAR, GM_SUN, R_SUN};
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    #[test]
    fn test_binet_rhs_flat_space() {
        // With gm = 0, the GR correction vanishes: d²u/dφ² + u = 0
        let state = GeodesicState::new(0.001, -0.0005);
        let rhs = SchwarzschildGeodesic::binet_rhs(0.0, &state);
        assert_relative_eq!(rhs[0], state[1], epsilon = 1e-15);
        assert_relative_eq!(rhs[1], -state[0], epsilon = 1e-15);
    }

    #[test]
    fn test_binet_rhs_with_gravity() {
        // With gm > 0, there's a positive correction term 3GM/c² · u²
        let gm = GM_SUN;
        let u = 1.0 / R_SUN; // at the Sun's surface
        let state = GeodesicState::new(u, 0.0);
        let rhs = SchwarzschildGeodesic::binet_rhs(gm, &state);

        let gr_correction = 3.0 * gm / (C * C) * u * u;
        assert_relative_eq!(rhs[1], -u + gr_correction, epsilon = 1e-25);
        // The GR correction should be small but positive
        assert!(gr_correction > 0.0);
        assert!(gr_correction < u); // correction << main term for the Sun
    }

    #[test]
    fn test_flat_space_straight_line() {
        // With gm = 0, the photon should travel in a straight line (zero deflection)
        let result = SchwarzschildGeodesic::integrate(
            0.0, // no gravity
            1e9, // 1 million km impact parameter
            PI,  // half orbit
            10_000,
        )
        .unwrap();

        assert!(!result.captured);
        assert!(
            result.deflection_angle < 1e-6,
            "Flat space deflection should be ~0, got {}",
            result.deflection_angle
        );
    }

    #[test]
    fn test_sun_deflection_order_of_magnitude() {
        // Photon grazing the Sun's limb should deflect ~1.75 arcseconds
        let result = SchwarzschildGeodesic::integrate(
            GM_SUN, R_SUN, // impact parameter = solar radius
            PI, 100_000,
        )
        .unwrap();

        assert!(!result.captured);
        // Convert to arcseconds: 1 radian = 206265 arcsec
        let deflection_arcsec = result.deflection_angle * 206265.0;
        // Should be in the right ballpark (1-3 arcsec)
        assert!(
            deflection_arcsec > 0.5 && deflection_arcsec < 5.0,
            "Sun limb deflection should be ~1.75 arcsec, got {deflection_arcsec:.4} arcsec"
        );
    }

    #[test]
    fn test_deflection_decreases_with_distance() {
        // Deflection should decrease as impact parameter increases
        let result_close =
            SchwarzschildGeodesic::integrate(GM_SUN, R_SUN * 2.0, PI, 50_000).unwrap();

        let result_far =
            SchwarzschildGeodesic::integrate(GM_SUN, R_SUN * 10.0, PI, 50_000).unwrap();

        assert!(
            result_close.deflection_angle > result_far.deflection_angle,
            "Closer pass should deflect more: close={}, far={}",
            result_close.deflection_angle,
            result_far.deflection_angle
        );
    }

    #[test]
    fn test_capture_below_critical_impact() {
        // For Sgr A*, photons with b < b_crit should be captured
        // b_crit = 3√3 · GM/c² ≈ 5.196 · GM/c²
        let b_crit = 3.0 * 3.0_f64.sqrt() * GM_SGR_A_STAR / (C * C);

        let result = SchwarzschildGeodesic::integrate(
            GM_SGR_A_STAR,
            b_crit * 0.5, // well below critical
            2.0 * PI,
            100_000,
        )
        .unwrap();

        assert!(
            result.captured,
            "Photon below critical impact parameter should be captured"
        );
    }

    #[test]
    fn test_escape_above_critical_impact() {
        // Photons with b >> b_crit should escape
        let b_crit = 3.0 * 3.0_f64.sqrt() * GM_SGR_A_STAR / (C * C);

        let result = SchwarzschildGeodesic::integrate(
            GM_SGR_A_STAR,
            b_crit * 5.0, // well above critical
            PI,
            100_000,
        )
        .unwrap();

        assert!(
            !result.captured,
            "Photon well above critical impact parameter should escape"
        );
    }

    #[test]
    fn test_cartesian_output() {
        let points =
            SchwarzschildGeodesic::integrate_to_cartesian(GM_SUN, R_SUN * 10.0, PI, 10_000)
                .unwrap();

        assert!(!points.is_empty());
        // All z coordinates should be zero (equatorial plane)
        for p in &points {
            assert_relative_eq!(p[2], 0.0, epsilon = 1e-15);
        }
        // Points should be at reasonable distances
        for p in &points {
            let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
            assert!(r > 0.0, "Point should have positive radius");
        }
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(SchwarzschildGeodesic::integrate(-1.0, 1e9, PI, 1000).is_err());
        assert!(SchwarzschildGeodesic::integrate(GM_SUN, 0.0, PI, 1000).is_err());
        assert!(SchwarzschildGeodesic::integrate(GM_SUN, -1.0, PI, 1000).is_err());
        assert!(SchwarzschildGeodesic::integrate(GM_SUN, 1e9, PI, 0).is_err());
        assert!(SchwarzschildGeodesic::integrate(GM_SUN, 1e9, -1.0, 1000).is_err());
    }

    #[test]
    fn test_closest_approach_less_than_initial() {
        let result = SchwarzschildGeodesic::integrate(GM_SUN, R_SUN * 5.0, PI, 50_000).unwrap();

        // Closest approach should be approximately the impact parameter
        // (exact for gm=0, close for weak fields)
        assert!(
            result.closest_approach < R_SUN * 10.0,
            "Closest approach should be near the impact parameter"
        );
        assert!(
            result.closest_approach > R_SUN * 0.1,
            "Closest approach should be reasonable"
        );
    }
}
