//! Black hole geometry utilities
//!
//! Provides calculations for characteristic radii and geometric properties
//! of Schwarzschild black holes: photon sphere, ISCO, shadow radius,
//! Einstein ring, and trajectory classification.
//!
//! # Key Radii (Schwarzschild Black Hole)
//!
//! | Radius | Formula | Meaning |
//! |--------|---------|---------|
//! | Schwarzschild | r_s = 2GM/c² | Event horizon |
//! | Photon sphere | r_ph = 1.5 r_s | Unstable circular photon orbit |
//! | ISCO | r_isco = 3 r_s | Innermost stable circular orbit |
//! | Shadow | r_sh = √27 GM/c² | Apparent dark region for distant observer |
//! | Critical impact | b_crit = 3√3 GM/c² | Capture threshold |
//!
//! # References
//! - Event Horizon Telescope Collaboration (2022). Sgr A* results.
//! - Einstein, A. (1936). "Lens-like action of a star." *Science* 84, 506.

use crate::core::constants::{schwarzschild_radius, C, GM_SGR_A_STAR};
use crate::core::error::{PoliastroError, PoliastroResult};

/// Complete geometry of a Schwarzschild black hole.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlackHoleGeometry {
    /// Gravitational parameter GM (m³/s²).
    pub gm: f64,
    /// Schwarzschild radius (meters): r_s = 2GM/c².
    pub schwarzschild_radius: f64,
    /// Photon sphere radius (meters): r_ph = 1.5 · r_s = 3GM/c².
    pub photon_sphere_radius: f64,
    /// Innermost stable circular orbit radius (meters): r_isco = 3 · r_s = 6GM/c².
    pub isco_radius: f64,
    /// Critical impact parameter (meters): b_crit = 3√3 · GM/c².
    /// Photons with b < b_crit are captured.
    pub critical_impact_parameter: f64,
    /// Shadow radius (meters): r_shadow = √27 · GM/c².
    /// The apparent radius of the dark region as seen by a distant observer.
    pub shadow_radius: f64,
}

/// Classification of a photon trajectory near a Schwarzschild black hole.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrajectoryType {
    /// Photon will be captured (b < b_critical).
    Capture,
    /// Photon at the unstable photon sphere orbit (b ≈ b_critical).
    UnstableOrbit,
    /// Photon deflects and escapes (b > b_critical).
    Deflection,
}

/// Photon sphere and black hole geometry calculator.
pub struct PhotonSphere;

impl PhotonSphere {
    /// Compute the complete geometry for a Schwarzschild black hole.
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter GM (m³/s²). Must be positive.
    ///
    /// # Errors
    /// Returns `InvalidParameter` if `gm <= 0`.
    pub fn geometry(gm: f64) -> PoliastroResult<BlackHoleGeometry> {
        if gm <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "gm",
                gm,
                "must be positive",
            ));
        }

        let r_s = schwarzschild_radius(gm);
        let gm_over_c2 = gm / (C * C);

        Ok(BlackHoleGeometry {
            gm,
            schwarzschild_radius: r_s,
            photon_sphere_radius: 1.5 * r_s,
            isco_radius: 3.0 * r_s,
            critical_impact_parameter: 3.0 * 3.0_f64.sqrt() * gm_over_c2,
            shadow_radius: 27.0_f64.sqrt() * gm_over_c2,
        })
    }

    /// Convenience: Sgr A* geometry using the library's built-in constants.
    pub fn sgr_a_star() -> BlackHoleGeometry {
        Self::geometry(GM_SGR_A_STAR).unwrap()
    }

    /// Classify a photon trajectory given its impact parameter.
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter GM (m³/s²)
    /// * `impact_parameter` - Perpendicular distance from asymptotic ray to body center (m)
    ///
    /// # Returns
    /// - `Capture` if b < b_crit
    /// - `UnstableOrbit` if b ≈ b_crit (within 0.1%)
    /// - `Deflection` if b > b_crit
    pub fn classify_trajectory(gm: f64, impact_parameter: f64) -> PoliastroResult<TrajectoryType> {
        if gm <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "gm",
                gm,
                "must be positive",
            ));
        }
        if impact_parameter <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "impact_parameter",
                impact_parameter,
                "must be positive",
            ));
        }

        let b_crit = 3.0 * 3.0_f64.sqrt() * gm / (C * C);
        let ratio = impact_parameter / b_crit;

        if (ratio - 1.0).abs() < 0.001 {
            Ok(TrajectoryType::UnstableOrbit)
        } else if ratio < 1.0 {
            Ok(TrajectoryType::Capture)
        } else {
            Ok(TrajectoryType::Deflection)
        }
    }

    /// Einstein ring angular radius for a perfectly aligned point source.
    ///
    /// ```text
    /// θ_E = √(4GM · D_LS / (c² · D_L · D_S))
    /// ```
    ///
    /// where:
    /// - D_L = distance from observer to lens
    /// - D_S = distance from observer to source
    /// - D_LS = D_S - D_L (distance from lens to source)
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter of the lens (m³/s²)
    /// * `d_lens` - Distance from observer to lens (m)
    /// * `d_source` - Distance from observer to source (m). Must be > d_lens.
    ///
    /// # Returns
    /// Angular radius in radians.
    ///
    /// # References
    /// Einstein, A. (1936). *Science* 84, 506.
    pub fn einstein_ring_radius(gm: f64, d_lens: f64, d_source: f64) -> PoliastroResult<f64> {
        if gm <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "gm",
                gm,
                "must be positive",
            ));
        }
        if d_lens <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "d_lens",
                d_lens,
                "must be positive",
            ));
        }
        if d_source <= d_lens {
            return Err(PoliastroError::invalid_parameter(
                "d_source",
                d_source,
                "must be greater than d_lens",
            ));
        }

        let d_ls = d_source - d_lens;
        let theta = (4.0 * gm * d_ls / (C * C * d_lens * d_source)).sqrt();

        Ok(theta)
    }

    /// Angular radius of the black hole shadow as seen from a given distance.
    ///
    /// For a distant observer: `θ_shadow ≈ r_shadow / d`
    ///
    /// # Arguments
    /// * `gm` - Gravitational parameter (m³/s²)
    /// * `observer_distance` - Distance from the black hole (m)
    ///
    /// # Returns
    /// Angular radius in radians.
    pub fn shadow_angular_radius(gm: f64, observer_distance: f64) -> PoliastroResult<f64> {
        if gm <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "gm",
                gm,
                "must be positive",
            ));
        }
        if observer_distance <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "observer_distance",
                observer_distance,
                "must be positive",
            ));
        }

        let r_shadow = 27.0_f64.sqrt() * gm / (C * C);
        Ok(r_shadow / observer_distance)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::{DISTANCE_SGR_A_STAR, GM_SUN, R_SCHWARZSCHILD_SGR_A_STAR};
    use approx::assert_relative_eq;

    #[test]
    fn test_sgr_a_star_geometry() {
        let geom = PhotonSphere::sgr_a_star();

        // Schwarzschild radius should match the precomputed constant
        assert_relative_eq!(
            geom.schwarzschild_radius,
            R_SCHWARZSCHILD_SGR_A_STAR,
            epsilon = 1.0 // within 1 meter
        );

        // Sgr A* r_s should be ~12 million km
        assert!(geom.schwarzschild_radius > 1e10);
        assert!(geom.schwarzschild_radius < 2e10);
    }

    #[test]
    fn test_radius_relationships() {
        let geom = PhotonSphere::geometry(GM_SGR_A_STAR).unwrap();

        // Photon sphere = 1.5 × Schwarzschild
        assert_relative_eq!(
            geom.photon_sphere_radius,
            1.5 * geom.schwarzschild_radius,
            epsilon = 1e-6
        );

        // ISCO = 3 × Schwarzschild
        assert_relative_eq!(
            geom.isco_radius,
            3.0 * geom.schwarzschild_radius,
            epsilon = 1e-6
        );

        // Ordering: r_s < r_ph < r_isco
        assert!(geom.schwarzschild_radius < geom.photon_sphere_radius);
        assert!(geom.photon_sphere_radius < geom.isco_radius);
    }

    #[test]
    fn test_critical_impact_parameter() {
        let geom = PhotonSphere::geometry(GM_SGR_A_STAR).unwrap();

        // b_crit = 3√3 · GM/c² = (3√3/2) · r_s
        let expected = 3.0 * 3.0_f64.sqrt() * GM_SGR_A_STAR / (C * C);
        assert_relative_eq!(geom.critical_impact_parameter, expected, epsilon = 1.0);

        // b_crit should be larger than the photon sphere radius
        assert!(geom.critical_impact_parameter > geom.photon_sphere_radius);
    }

    #[test]
    fn test_shadow_radius() {
        let geom = PhotonSphere::geometry(GM_SGR_A_STAR).unwrap();

        // Shadow radius = √27 · GM/c²
        let expected = 27.0_f64.sqrt() * GM_SGR_A_STAR / (C * C);
        assert_relative_eq!(geom.shadow_radius, expected, epsilon = 1.0);

        // Shadow radius should equal critical impact parameter for Schwarzschild
        // b_crit = 3√3 · GM/c², r_shadow = √27 · GM/c² = 3√3 · GM/c²
        assert_relative_eq!(
            geom.shadow_radius,
            geom.critical_impact_parameter,
            epsilon = 1.0
        );
    }

    #[test]
    fn test_sun_photon_sphere_tiny() {
        let geom = PhotonSphere::geometry(GM_SUN).unwrap();
        // Sun's Schwarzschild radius ≈ 2953 m, photon sphere ≈ 4430 m
        assert!(geom.photon_sphere_radius < 5000.0);
        assert!(geom.photon_sphere_radius > 4000.0);
    }

    #[test]
    fn test_trajectory_classification() {
        let geom = PhotonSphere::geometry(GM_SGR_A_STAR).unwrap();
        let b_crit = geom.critical_impact_parameter;

        assert_eq!(
            PhotonSphere::classify_trajectory(GM_SGR_A_STAR, b_crit * 0.5).unwrap(),
            TrajectoryType::Capture
        );
        assert_eq!(
            PhotonSphere::classify_trajectory(GM_SGR_A_STAR, b_crit * 1.0005).unwrap(),
            TrajectoryType::UnstableOrbit
        );
        assert_eq!(
            PhotonSphere::classify_trajectory(GM_SGR_A_STAR, b_crit * 2.0).unwrap(),
            TrajectoryType::Deflection
        );
    }

    #[test]
    fn test_einstein_ring_radius() {
        // A star behind Sgr A* as seen from Earth
        let d_lens = DISTANCE_SGR_A_STAR; // ~26,670 ly
        let d_source = 2.0 * d_lens; // source twice as far

        let theta = PhotonSphere::einstein_ring_radius(GM_SGR_A_STAR, d_lens, d_source).unwrap();

        // Should be a small angle (microarcseconds for stellar-mass lenses,
        // milliarcseconds for supermassive)
        assert!(theta > 0.0);
        assert!(theta < 0.01); // less than ~0.5 degrees
    }

    #[test]
    fn test_sgr_a_shadow_angular_size() {
        // EHT measured Sgr A* shadow as ~52 microarcseconds
        let theta =
            PhotonSphere::shadow_angular_radius(GM_SGR_A_STAR, DISTANCE_SGR_A_STAR).unwrap();

        // Convert to microarcseconds
        let theta_uas = theta * 206265.0 * 1e6;

        // Should be in the right ballpark (30-80 μas)
        assert!(
            theta_uas > 20.0 && theta_uas < 100.0,
            "Sgr A* shadow should be ~52 μas, got {theta_uas:.1} μas"
        );
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(PhotonSphere::geometry(0.0).is_err());
        assert!(PhotonSphere::geometry(-1.0).is_err());
        assert!(PhotonSphere::classify_trajectory(0.0, 1e10).is_err());
        assert!(PhotonSphere::classify_trajectory(GM_SGR_A_STAR, 0.0).is_err());
        assert!(PhotonSphere::einstein_ring_radius(GM_SGR_A_STAR, 1e20, 0.5e20).is_err());
        assert!(PhotonSphere::shadow_angular_radius(0.0, 1e20).is_err());
    }
}
