//! Analytical weak-field gravitational deflection
//!
//! Provides fast analytical approximations for light deflection far from
//! massive bodies (weak-field regime), and adaptive dispatch to numerical
//! integration for close passes (strong-field regime).
//!
//! # Einstein Deflection Formula
//!
//! For a photon passing a mass M at impact parameter b >> r_s:
//! ```text
//! α = 4GM / (c² · b)
//! ```
//! This is accurate to first post-Newtonian order.
//!
//! # Shapiro Delay
//!
//! Additional travel time due to spacetime curvature:
//! ```text
//! Δt = (2GM/c³) · ln((r_source + r_target + d) / (r_source + r_target - d))
//! ```
//! where d is the straight-line distance between source and target.
//!
//! # References
//! - Einstein, A. (1916). "Die Grundlage der allgemeinen Relativitätstheorie."
//! - Shapiro, I.I. (1964). "Fourth test of general relativity." *Phys. Rev. Lett.* 13, 789.
//! - Will, C.M. (2014). "The confrontation between general relativity and experiment."

use super::schwarzschild::SchwarzschildGeodesic;
use crate::core::constants::{rad_to_arcsec, schwarzschild_radius, C, GM_SUN, R_SUN};
use crate::core::error::{PoliastroError, PoliastroResult};

/// Result of a gravitational deflection calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeflectionResult {
    /// Deflection angle (radians).
    pub deflection_angle: f64,
    /// Deflection angle (arcseconds).
    pub deflection_arcsec: f64,
    /// Impact parameter used (meters).
    pub impact_parameter: f64,
    /// Gravitational parameter of the deflecting body (m³/s²).
    pub gm: f64,
    /// Whether the weak-field analytical formula was used.
    pub is_weak_field: bool,
}

/// Analytical and adaptive gravitational deflection calculator.
pub struct Deflection;

impl Deflection {
    /// First-order Einstein deflection angle.
    ///
    /// Computes `α = 4GM / (c² · b)` — the standard weak-field result.
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter GM (m³/s²)
    /// * `impact_parameter` - Closest approach distance of the undeflected ray (m)
    ///
    /// # Errors
    /// Returns `InvalidParameter` if `gm < 0` or `impact_parameter <= 0`.
    pub fn einstein_deflection(
        gm: f64,
        impact_parameter: f64,
    ) -> PoliastroResult<DeflectionResult> {
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

        let deflection_angle = 4.0 * gm / (C * C * impact_parameter);
        let r_s = schwarzschild_radius(gm);
        let is_weak_field = r_s == 0.0 || impact_parameter > 100.0 * r_s;

        Ok(DeflectionResult {
            deflection_angle,
            deflection_arcsec: rad_to_arcsec(deflection_angle),
            impact_parameter,
            gm,
            is_weak_field,
        })
    }

    /// Solar limb deflection — the famous 1.75 arcsecond prediction.
    ///
    /// Convenience method computing `einstein_deflection(GM_SUN, R_SUN)`.
    /// Confirmed by Eddington's 1919 eclipse expedition and many subsequent measurements.
    pub fn sun_limb_deflection() -> DeflectionResult {
        // This cannot fail with valid constants
        Self::einstein_deflection(GM_SUN, R_SUN).unwrap()
    }

    /// Determine whether the weak-field approximation is adequate.
    ///
    /// Returns `true` if `impact_parameter > 100 · r_s`, where `r_s` is the
    /// Schwarzschild radius. The 100× threshold ensures the first-order Einstein
    /// formula is accurate to better than 0.01%.
    #[inline]
    pub fn is_weak_field(gm: f64, impact_parameter: f64) -> bool {
        let r_s = schwarzschild_radius(gm);
        r_s == 0.0 || impact_parameter > 100.0 * r_s
    }

    /// Adaptive deflection — uses analytical or numerical method as appropriate.
    ///
    /// For weak fields (`b > 100 · r_s`), uses the Einstein formula.
    /// For strong fields, uses full Schwarzschild null geodesic integration.
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter GM (m³/s²)
    /// * `impact_parameter` - Impact parameter (m)
    /// * `num_steps` - Number of integration steps for numerical mode (ignored in weak-field)
    pub fn deflection_adaptive(
        gm: f64,
        impact_parameter: f64,
        num_steps: usize,
    ) -> PoliastroResult<DeflectionResult> {
        if Self::is_weak_field(gm, impact_parameter) {
            Self::einstein_deflection(gm, impact_parameter)
        } else {
            // Strong field: use numerical integration
            let result = SchwarzschildGeodesic::integrate(
                gm,
                impact_parameter,
                std::f64::consts::PI * 2.0, // allow extra room for strong deflection
                num_steps,
            )?;

            let deflection_angle = if result.captured {
                f64::INFINITY
            } else {
                result.deflection_angle
            };

            Ok(DeflectionResult {
                deflection_angle,
                deflection_arcsec: rad_to_arcsec(deflection_angle),
                impact_parameter,
                gm,
                is_weak_field: false,
            })
        }
    }

    /// Shapiro time delay — additional light travel time due to spacetime curvature.
    ///
    /// When light passes near a massive body, it takes longer than the straight-line
    /// travel time. This excess is the Shapiro delay.
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter of the lensing body (m³/s²)
    /// * `r_source` - Distance from the lensing body to the light source (m)
    /// * `r_target` - Distance from the lensing body to the observer/target (m)
    /// * `impact_parameter` - Closest approach distance (m)
    ///
    /// # Returns
    /// Excess travel time in seconds.
    ///
    /// # References
    /// Shapiro, I.I. (1964). Approximate formula valid for `b << r_source, r_target`.
    pub fn shapiro_delay(
        gm: f64,
        r_source: f64,
        r_target: f64,
        impact_parameter: f64,
    ) -> PoliastroResult<f64> {
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
        if r_source <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_source",
                r_source,
                "must be positive",
            ));
        }
        if r_target <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "r_target",
                r_target,
                "must be positive",
            ));
        }

        // Shapiro delay formula:
        // Δt = (2GM/c³) · ln((4 · r_source · r_target) / (b²))
        // More precise form using the logarithmic expression
        let c3 = C * C * C;
        let delay = (2.0 * gm / c3)
            * ((4.0 * r_source * r_target) / (impact_parameter * impact_parameter)).ln();

        Ok(delay)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::GM_SGR_A_STAR;
    use approx::assert_relative_eq;

    #[test]
    fn test_sun_limb_deflection() {
        let result = Deflection::sun_limb_deflection();
        // Einstein's prediction: 1.7505 arcseconds
        assert_relative_eq!(result.deflection_arcsec, 1.7505, epsilon = 0.01);
        assert!(result.is_weak_field);
    }

    #[test]
    fn test_einstein_deflection_scaling_with_gm() {
        // Deflection should scale linearly with GM
        let result1 = Deflection::einstein_deflection(GM_SUN, 1e10).unwrap();
        let result2 = Deflection::einstein_deflection(2.0 * GM_SUN, 1e10).unwrap();
        assert_relative_eq!(
            result2.deflection_angle,
            2.0 * result1.deflection_angle,
            epsilon = 1e-15
        );
    }

    #[test]
    fn test_einstein_deflection_scaling_with_b() {
        // Deflection should scale inversely with impact parameter
        let result1 = Deflection::einstein_deflection(GM_SUN, 1e10).unwrap();
        let result2 = Deflection::einstein_deflection(GM_SUN, 2e10).unwrap();
        assert_relative_eq!(
            result2.deflection_angle,
            0.5 * result1.deflection_angle,
            epsilon = 1e-15
        );
    }

    #[test]
    fn test_zero_gm_zero_deflection() {
        let result = Deflection::einstein_deflection(0.0, 1e10).unwrap();
        assert_relative_eq!(result.deflection_angle, 0.0, epsilon = 1e-20);
    }

    #[test]
    fn test_is_weak_field_sun() {
        // Sun at its surface: r_s ≈ 2953 m, R_SUN ≈ 7e8 m
        // R_SUN / r_s ≈ 237,000 >> 100
        assert!(Deflection::is_weak_field(GM_SUN, R_SUN));
    }

    #[test]
    fn test_is_weak_field_sgr_a() {
        // Sgr A* at photon sphere: 1.5 * r_s ≈ 1.84e10 m
        // 1.5 * r_s / r_s = 1.5 < 100, so NOT weak field
        let r_s = schwarzschild_radius(GM_SGR_A_STAR);
        assert!(!Deflection::is_weak_field(GM_SGR_A_STAR, 1.5 * r_s));
        // But far from it:
        assert!(Deflection::is_weak_field(GM_SGR_A_STAR, 200.0 * r_s));
    }

    #[test]
    fn test_adaptive_weak_field_matches_analytical() {
        // In weak field, adaptive should give same result as analytical
        let analytical = Deflection::einstein_deflection(GM_SUN, R_SUN * 10.0).unwrap();
        let adaptive = Deflection::deflection_adaptive(GM_SUN, R_SUN * 10.0, 100_000).unwrap();
        assert_relative_eq!(
            analytical.deflection_angle,
            adaptive.deflection_angle,
            epsilon = 1e-15
        );
        assert!(adaptive.is_weak_field);
    }

    #[test]
    fn test_shapiro_delay_sun() {
        // Shapiro delay for light passing near the Sun:
        // Earth-Mars superior conjunction: ~240 μs measured by Cassini
        // Earth distance: ~1 AU, Mars at ~2.5 AU during superior conjunction
        let r_earth = 1.496e11; // 1 AU in meters
        let r_mars = 3.74e11; // ~2.5 AU in meters
        let delay = Deflection::shapiro_delay(GM_SUN, r_earth, r_mars, R_SUN).unwrap();

        // Should be on the order of 100-300 μs
        let delay_us = delay * 1e6;
        assert!(
            delay_us > 50.0 && delay_us < 500.0,
            "Shapiro delay should be ~200 μs, got {delay_us:.1} μs"
        );
    }

    #[test]
    fn test_shapiro_delay_increases_with_gm() {
        let r = 1e11;
        let b = 1e9;
        let delay1 = Deflection::shapiro_delay(GM_SUN, r, r, b).unwrap();
        let delay2 = Deflection::shapiro_delay(2.0 * GM_SUN, r, r, b).unwrap();
        assert_relative_eq!(delay2, 2.0 * delay1, epsilon = 1e-10);
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(Deflection::einstein_deflection(-1.0, 1e9).is_err());
        assert!(Deflection::einstein_deflection(GM_SUN, 0.0).is_err());
        assert!(Deflection::einstein_deflection(GM_SUN, -1.0).is_err());
        assert!(Deflection::shapiro_delay(GM_SUN, 0.0, 1e11, 1e9).is_err());
        assert!(Deflection::shapiro_delay(GM_SUN, 1e11, 0.0, 1e9).is_err());
        assert!(Deflection::shapiro_delay(GM_SUN, 1e11, 1e11, 0.0).is_err());
    }
}
