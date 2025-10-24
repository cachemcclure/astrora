//! Classical orbital elements and state vector conversions
//!
//! This module provides representations of classical (Keplerian) orbital elements
//! and conversion functions between Cartesian state vectors and orbital elements.
//!
//! # Conversions
//! - `rv_to_coe`: Convert position and velocity vectors to classical orbital elements
//! - `coe_to_rv`: Convert classical orbital elements to position and velocity vectors
//!
//! # Edge Cases
//! The conversions handle special cases including:
//! - Circular orbits (e ≈ 0)
//! - Equatorial orbits (i ≈ 0)
//! - Parabolic and hyperbolic trajectories

use crate::core::error::{PoliastroError, PoliastroResult};
use crate::core::linalg::{Vector3, Matrix3};
use pyo3::prelude::*;
use std::f64::consts::PI;

/// Default tolerance for orbital element singularity checks
pub const DEFAULT_TOL: f64 = 1e-8;

/// Classical (Keplerian) orbital elements
///
/// # Elements
/// - `a`: Semi-major axis (meters)
/// - `e`: Eccentricity (dimensionless, 0 ≤ e for bound orbits)
/// - `i`: Inclination (radians, 0 ≤ i ≤ π)
/// - `raan`: Right ascension of ascending node (Ω, radians, 0 ≤ Ω < 2π)
/// - `argp`: Argument of periapsis (ω, radians, 0 ≤ ω < 2π)
/// - `nu`: True anomaly (ν, radians, 0 ≤ ν < 2π)
#[pyclass(module = "astrora._core", name = "OrbitalElements")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitalElements {
    /// Semi-major axis (m)
    #[pyo3(get, set)]
    pub a: f64,
    /// Eccentricity (dimensionless)
    #[pyo3(get, set)]
    pub e: f64,
    /// Inclination (radians)
    #[pyo3(get, set)]
    pub i: f64,
    /// Right ascension of ascending node (radians)
    #[pyo3(get, set)]
    pub raan: f64,
    /// Argument of periapsis (radians)
    #[pyo3(get, set)]
    pub argp: f64,
    /// True anomaly (radians)
    #[pyo3(get, set)]
    pub nu: f64,
}

impl OrbitalElements {
    /// Create new orbital elements
    ///
    /// # Arguments
    /// * `a` - Semi-major axis (meters)
    /// * `e` - Eccentricity
    /// * `i` - Inclination (radians)
    /// * `raan` - Right ascension of ascending node (radians)
    /// * `argp` - Argument of periapsis (radians)
    /// * `nu` - True anomaly (radians)
    pub fn new(a: f64, e: f64, i: f64, raan: f64, argp: f64, nu: f64) -> Self {
        Self {
            a,
            e,
            i,
            raan,
            argp,
            nu,
        }
    }

    /// Calculate orbital period for elliptical orbits (seconds)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    ///
    /// # Returns
    /// Orbital period T = 2π√(a³/μ)
    ///
    /// # Errors
    /// Returns error if orbit is not elliptical (e >= 1)
    pub fn period(&self, mu: f64) -> PoliastroResult<f64> {
        if self.e >= 1.0 {
            return Err(PoliastroError::UnsupportedOrbitType {
                operation: "period calculation".into(),
                orbit_type: if self.e > 1.0 {
                    "hyperbolic"
                } else {
                    "parabolic"
                }
                .into(),
            });
        }
        Ok(2.0 * PI * (self.a.powi(3) / mu).sqrt())
    }

    /// Calculate periapsis distance (m)
    ///
    /// For elliptical orbits: r_p = a(1 - e)
    pub fn periapsis(&self) -> f64 {
        self.a * (1.0 - self.e)
    }

    /// Calculate apoapsis distance (m)
    ///
    /// For elliptical orbits: r_a = a(1 + e)
    ///
    /// # Note
    /// For hyperbolic orbits (e > 1), this returns a negative value
    /// which is physically meaningless (hyperbolic orbits have no apoapsis)
    pub fn apoapsis(&self) -> f64 {
        self.a * (1.0 + self.e)
    }

    /// Calculate semi-latus rectum (m)
    ///
    /// p = a(1 - e²)
    pub fn semi_latus_rectum(&self) -> f64 {
        self.a * (1.0 - self.e * self.e)
    }

    /// Calculate specific angular momentum magnitude (m²/s)
    ///
    /// h = √(μp) where p is the semi-latus rectum
    pub fn specific_angular_momentum_magnitude(&self, mu: f64) -> f64 {
        (mu * self.semi_latus_rectum()).sqrt()
    }
}

/// Modified Equinoctial Orbital Elements
///
/// A singularity-free representation of orbital elements that avoids singularities
/// for circular (e ≈ 0) and equatorial (i ≈ 0, i ≈ 90°) orbits.
///
/// # Elements
/// - `p`: Semi-latus rectum (meters)
/// - `f`: Eccentricity x-component: e·cos(ω + Ω)
/// - `g`: Eccentricity y-component: e·sin(ω + Ω)
/// - `h`: Inclination x-component: tan(i/2)·cos(Ω)
/// - `k`: Inclination y-component: tan(i/2)·sin(Ω)
/// - `L`: True longitude: Ω + ω + ν (radians)
///
/// # Singularities
/// - Standard formulation: Singularity at i = π (retrograde equatorial orbits)
/// - Free from singularities at e = 0 and i = 0, 90°
///
/// # References
/// Walker, M. J. H., Ireland, B., and Owens, J., "A Set of Modified Equinoctial
/// Orbital Elements", Celestial Mechanics, Vol. 36, pp. 409-419, 1985.
#[pyclass(module = "astrora._core", name = "EquinoctialElements")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquinoctialElements {
    /// Semi-latus rectum (m)
    #[pyo3(get, set)]
    pub p: f64,
    /// Eccentricity x-component (dimensionless)
    #[pyo3(get, set)]
    pub f: f64,
    /// Eccentricity y-component (dimensionless)
    #[pyo3(get, set)]
    pub g: f64,
    /// Inclination x-component (dimensionless)
    #[pyo3(get, set)]
    pub h: f64,
    /// Inclination y-component (dimensionless)
    #[pyo3(get, set)]
    pub k: f64,
    /// True longitude (radians)
    #[pyo3(get, set)]
    pub L: f64,
}

impl EquinoctialElements {
    /// Create new modified equinoctial elements
    ///
    /// # Arguments
    /// * `p` - Semi-latus rectum (meters)
    /// * `f` - Eccentricity x-component
    /// * `g` - Eccentricity y-component
    /// * `h` - Inclination x-component
    /// * `k` - Inclination y-component
    /// * `L` - True longitude (radians)
    pub fn new(p: f64, f: f64, g: f64, h: f64, k: f64, L: f64) -> Self {
        Self { p, f, g, h, k, L }
    }

    /// Calculate eccentricity from equinoctial elements
    ///
    /// e = √(f² + g²)
    pub fn eccentricity(&self) -> f64 {
        (self.f * self.f + self.g * self.g).sqrt()
    }

    /// Calculate semi-major axis for elliptical orbits (meters)
    ///
    /// a = p / (1 - e²)
    ///
    /// # Errors
    /// Returns error if orbit is not elliptical (e >= 1)
    pub fn semi_major_axis(&self) -> PoliastroResult<f64> {
        let e = self.eccentricity();
        if e >= 1.0 {
            return Err(PoliastroError::UnsupportedOrbitType {
                operation: "semi-major axis calculation".into(),
                orbit_type: if e > 1.0 {
                    "hyperbolic"
                } else {
                    "parabolic"
                }
                .into(),
            });
        }
        Ok(self.p / (1.0 - e * e))
    }

    /// Calculate inclination from equinoctial elements (radians)
    ///
    /// i = 2·arctan(√(h² + k²))
    pub fn inclination(&self) -> f64 {
        2.0 * (self.h * self.h + self.k * self.k).sqrt().atan()
    }

    /// Calculate orbital period for elliptical orbits (seconds)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    pub fn period(&self, mu: f64) -> PoliastroResult<f64> {
        let a = self.semi_major_axis()?;
        Ok(2.0 * PI * (a.powi(3) / mu).sqrt())
    }
}

/// Convert Cartesian state vectors to classical orbital elements
///
/// Implements the RV2COE algorithm from Vallado's "Fundamentals of Astrodynamics and Applications"
///
/// # Arguments
/// * `r` - Position vector (m)
/// * `v` - Velocity vector (m/s)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// Classical orbital elements (a, e, i, Ω, ω, ν)
///
/// # Edge Cases
/// - **Circular orbits** (e < tol): ω is set to 0, ν measured from ascending node
/// - **Equatorial orbits** (i < tol): Ω is set to 0, ω measured from x-axis
/// - **Circular equatorial**: Both Ω and ω set to 0, ν measured from x-axis
///
/// # Errors
/// Returns error if the orbit is not physically valid
///
/// # Example
/// ```
/// use astrora_core::core::elements::rv_to_coe;
/// use astrora_core::core::linalg::Vector3;
/// use astrora_core::core::constants::GM_EARTH;
///
/// let r = Vector3::new(7000e3, 0.0, 0.0);
/// let v = Vector3::new(0.0, 7.5e3, 0.0);
/// let elements = rv_to_coe(&r, &v, GM_EARTH, 1e-8).unwrap();
/// ```
pub fn rv_to_coe(
    r: &Vector3,
    v: &Vector3,
    mu: f64,
    tol: f64,
) -> PoliastroResult<OrbitalElements> {
    // Step 1: Calculate magnitudes
    let r_mag = r.norm();
    let v_mag = v.norm();

    // Radial velocity (component of velocity in radial direction)
    let v_r = r.dot(v) / r_mag;

    // Step 2: Specific angular momentum
    let h_vec = r.cross(v);
    let h_mag = h_vec.norm();

    if h_mag < tol {
        return Err(PoliastroError::InvalidStateVector {
            reason: format!("orbit is degenerate (zero angular momentum): |h| = {h_mag}"),
        });
    }

    // Step 3: Inclination
    let i = (h_vec.z / h_mag).acos();

    // Step 4: Node line vector (N = K × h, where K = [0, 0, 1])
    let k_vec = Vector3::new(0.0, 0.0, 1.0);
    let n_vec = k_vec.cross(&h_vec);
    let n_mag = n_vec.norm();

    // Step 5: Eccentricity vector and magnitude
    // e = (1/μ)[(v² - μ/r)r - (r·v)v]
    let ecc_vec = ((v_mag * v_mag - mu / r_mag) * r - (r.dot(v)) * v) / mu;
    let e = ecc_vec.norm();

    // Step 6: Specific orbital energy
    let energy = v_mag * v_mag / 2.0 - mu / r_mag;

    // Step 7: Semi-major axis
    let a = if e.abs() < 1.0 - tol {
        // Elliptical orbit
        -mu / (2.0 * energy)
    } else if e.abs() < 1.0 + tol {
        // Parabolic orbit (e ≈ 1)
        return Err(PoliastroError::UnsupportedOrbitType {
                operation: "rv_to_coe".into(),
                orbit_type: "parabolic (e ≈ 1)".into(),
            });
    } else {
        // Hyperbolic orbit (e > 1)
        -mu / (2.0 * energy) // Note: a will be negative for hyperbolic orbits
    };

    // Step 8: Right ascension of ascending node (RAAN, Ω)
    let raan = if i.abs() < tol || (PI - i).abs() < tol {
        // Equatorial orbit: RAAN is undefined, set to 0
        0.0
    } else if n_mag < tol {
        // Should not happen if i is not zero, but check anyway
        0.0
    } else {
        // General case
        let raan_base = (n_vec.x / n_mag).acos();
        // Quadrant check: if N_y < 0, RAAN is in 3rd or 4th quadrant
        if n_vec.y >= 0.0 {
            raan_base
        } else {
            2.0 * PI - raan_base
        }
    };

    // Step 9: Argument of periapsis (ω)
    let argp = if e < tol {
        // Circular orbit: ω is undefined, set to 0
        0.0
    } else if i.abs() < tol || (PI - i).abs() < tol {
        // Equatorial orbit: measure ω from x-axis
        let argp_base = (ecc_vec.x / e).acos();
        if ecc_vec.y >= 0.0 {
            argp_base
        } else {
            2.0 * PI - argp_base
        }
    } else if n_mag < tol {
        // Should not happen, but handle gracefully
        0.0
    } else {
        // General case: ω measured from ascending node
        let argp_base = (n_vec.dot(&ecc_vec) / (n_mag * e)).acos();
        // Quadrant check: if e_z < 0, ω is in 3rd or 4th quadrant
        if ecc_vec.z >= 0.0 {
            argp_base
        } else {
            2.0 * PI - argp_base
        }
    };

    // Step 10: True anomaly (ν)
    let nu = if e < tol {
        // Circular orbit: ν measured from ascending node (or x-axis if equatorial)
        if i.abs() < tol || (PI - i).abs() < tol {
            // Circular equatorial: measure from x-axis
            let nu_base = (r.x / r_mag).acos();
            if r.y >= 0.0 {
                nu_base
            } else {
                2.0 * PI - nu_base
            }
        } else if n_mag < tol {
            0.0
        } else {
            // Circular inclined: measure from ascending node
            let nu_base = (n_vec.dot(r) / (n_mag * r_mag)).acos();
            if r.z >= 0.0 {
                nu_base
            } else {
                2.0 * PI - nu_base
            }
        }
    } else {
        // Eccentric orbit: ν measured from periapsis
        let nu_base = (ecc_vec.dot(r) / (e * r_mag)).acos();
        // Quadrant check: if v_r < 0, we're past apoapsis
        if v_r >= 0.0 {
            nu_base
        } else {
            2.0 * PI - nu_base
        }
    };

    Ok(OrbitalElements::new(a, e, i, raan, argp, nu))
}

/// Convert classical orbital elements to Cartesian state vectors
///
/// Implements the COE2RV algorithm using perifocal frame and rotation matrices
///
/// # Arguments
/// * `elements` - Classical orbital elements
/// * `mu` - Standard gravitational parameter (m³/s²)
///
/// # Returns
/// (position, velocity) tuple in inertial frame
///
/// # Algorithm
/// 1. Compute position and velocity in perifocal frame (PQW)
/// 2. Apply rotation matrices to transform to inertial frame (IJK)
/// 3. Rotation sequence: Z(Ω) → X(i) → Z(ω)
///
/// # Example
/// ```
/// use astrora_core::core::elements::{OrbitalElements, coe_to_rv};
/// use astrora_core::core::constants::GM_EARTH;
///
/// let elements = OrbitalElements::new(
///     7000e3,    // a: 7000 km
///     0.01,      // e: slight eccentricity
///     0.0,       // i: equatorial
///     0.0,       // Ω: RAAN
///     0.0,       // ω: arg of periapsis
///     0.0,       // ν: true anomaly
/// );
/// let (r, v) = coe_to_rv(&elements, GM_EARTH);
/// ```
pub fn coe_to_rv(elements: &OrbitalElements, mu: f64) -> (Vector3, Vector3) {
    let OrbitalElements {
        a,
        e,
        i,
        raan,
        argp,
        nu,
    } = *elements;

    // Calculate semi-latus rectum
    let p = a * (1.0 - e * e);

    // Calculate position magnitude in orbit
    let r_mag = p / (1.0 + e * nu.cos());

    // Position in perifocal frame (PQW coordinates)
    let r_pqw = Vector3::new(r_mag * nu.cos(), r_mag * nu.sin(), 0.0);

    // Velocity in perifocal frame
    // v_p = (μ/h) * [-sin(ν), e + cos(ν), 0]
    // where h = √(μp)
    let h = (mu * p).sqrt();
    let v_pqw = (mu / h) * Vector3::new(-nu.sin(), e + nu.cos(), 0.0);

    // Create rotation matrices for transformation from perifocal to inertial
    // Rotation sequence: R = R_z(-Ω) * R_x(-i) * R_z(-ω)
    // Note: nalgebra uses right-handed rotations, so we use negative angles
    // to rotate FROM perifocal TO inertial

    // R_z(ω): Rotation about z-axis by argument of periapsis
    let r_z_argp = Matrix3::new(
        argp.cos(),
        -argp.sin(),
        0.0,
        argp.sin(),
        argp.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    );

    // R_x(i): Rotation about x-axis by inclination
    let r_x_i = Matrix3::new(1.0, 0.0, 0.0, 0.0, i.cos(), -i.sin(), 0.0, i.sin(), i.cos());

    // R_z(Ω): Rotation about z-axis by RAAN
    let r_z_raan = Matrix3::new(
        raan.cos(),
        -raan.sin(),
        0.0,
        raan.sin(),
        raan.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    );

    // Combined rotation: R = R_z(Ω) * R_x(i) * R_z(ω)
    let rotation = r_z_raan * r_x_i * r_z_argp;

    // Transform position and velocity to inertial frame
    let r_inertial = rotation * r_pqw;
    let v_inertial = rotation * v_pqw;

    (r_inertial, v_inertial)
}

/// Convert classical orbital elements to modified equinoctial elements
///
/// # Arguments
/// * `elements` - Classical orbital elements (a, e, i, Ω, ω, ν)
///
/// # Returns
/// Modified equinoctial elements (p, f, g, h, k, L)
///
/// # Formulas
/// - p = a(1 - e²)
/// - f = e·cos(ω + Ω)
/// - g = e·sin(ω + Ω)
/// - h = tan(i/2)·cos(Ω)
/// - k = tan(i/2)·sin(Ω)
/// - L = Ω + ω + ν
///
/// # Example
/// ```
/// use astrora_core::core::elements::{OrbitalElements, coe_to_equinoctial};
///
/// let elements = OrbitalElements::new(
///     7000e3,    // a: 7000 km
///     0.01,      // e: slight eccentricity
///     0.05,      // i: 5° inclination
///     0.1,       // Ω: RAAN
///     0.2,       // ω: arg of periapsis
///     0.3,       // ν: true anomaly
/// );
/// let eq_elements = coe_to_equinoctial(&elements);
/// ```
pub fn coe_to_equinoctial(elements: &OrbitalElements) -> EquinoctialElements {
    let OrbitalElements {
        a,
        e,
        i,
        raan,
        argp,
        nu,
    } = *elements;

    // Semi-latus rectum
    let p = a * (1.0 - e * e);

    // Eccentricity components
    let omega_plus_raan = argp + raan;
    let f = e * omega_plus_raan.cos();
    let g = e * omega_plus_raan.sin();

    // Inclination components
    let tan_half_i = (i / 2.0).tan();
    let h = tan_half_i * raan.cos();
    let k = tan_half_i * raan.sin();

    // True longitude
    let L = raan + argp + nu;

    EquinoctialElements::new(p, f, g, h, k, L)
}

/// Convert modified equinoctial elements to classical orbital elements
///
/// # Arguments
/// * `elements` - Modified equinoctial elements (p, f, g, h, k, L)
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// Classical orbital elements (a, e, i, Ω, ω, ν)
///
/// # Formulas
/// - e = √(f² + g²)
/// - i = 2·arctan(√(h² + k²))
/// - Ω = atan2(k, h)
/// - ω = atan2(g, f) - Ω
/// - ν = L - atan2(g, f)
/// - a = p / (1 - e²)
///
/// # Edge Cases
/// - **Circular orbits** (e < tol): ω set to 0, ν measured from ascending node
/// - **Equatorial orbits** (i < tol): Ω set to 0, ω measured from x-axis
///
/// # Errors
/// Returns error if orbit is not elliptical (e >= 1) or has other invalid properties
///
/// # Example
/// ```
/// use astrora_core::core::elements::{EquinoctialElements, equinoctial_to_coe};
///
/// let eq_elements = EquinoctialElements::new(
///     6.93e6,    // p: semi-latus rectum
///     0.01,      // f: ecc x-component
///     0.0,       // g: ecc y-component
///     0.044,     // h: inc x-component
///     0.0,       // k: inc y-component
///     0.5,       // L: true longitude
/// );
/// let elements = equinoctial_to_coe(&eq_elements, 1e-8).unwrap();
/// ```
pub fn equinoctial_to_coe(
    elements: &EquinoctialElements,
    tol: f64,
) -> PoliastroResult<OrbitalElements> {
    let EquinoctialElements {
        p,
        f,
        g,
        h,
        k,
        L,
    } = *elements;

    // Calculate eccentricity
    let e = (f * f + g * g).sqrt();

    // Check for parabolic/hyperbolic orbits
    if e >= 1.0 - tol {
        return Err(PoliastroError::UnsupportedOrbitType {
            operation: "equinoctial_to_coe".into(),
            orbit_type: if e > 1.0 + tol {
                "hyperbolic (e > 1)"
            } else {
                "parabolic (e ≈ 1)"
            }
            .into(),
        });
    }

    // Calculate semi-major axis
    let a = p / (1.0 - e * e);

    // Calculate inclination
    let i = 2.0 * (h * h + k * k).sqrt().atan();

    // Handle edge cases
    let (raan, argp, nu) = if i.abs() < tol {
        // Equatorial orbit: Ω undefined, set to 0
        let raan = 0.0;

        if e < tol {
            // Circular equatorial: both Ω and ω undefined
            let argp = 0.0;
            let nu = normalize_angle(L);
            (raan, argp, nu)
        } else {
            // Eccentric equatorial: ω measured from x-axis
            let argp = normalize_angle(g.atan2(f));
            let nu = normalize_angle(L - argp);
            (raan, argp, nu)
        }
    } else if e < tol {
        // Circular inclined: ω undefined
        let raan = normalize_angle(k.atan2(h));
        let argp = 0.0;
        let nu = normalize_angle(L - raan);
        (raan, argp, nu)
    } else {
        // General case
        let raan = normalize_angle(k.atan2(h));
        let argp_plus_raan = g.atan2(f);
        let argp = normalize_angle(argp_plus_raan - raan);
        let nu = normalize_angle(L - argp_plus_raan);
        (raan, argp, nu)
    };

    Ok(OrbitalElements::new(a, e, i, raan, argp, nu))
}

/// Convert Cartesian state vectors to modified equinoctial elements
///
/// # Arguments
/// * `r` - Position vector (m)
/// * `v` - Velocity vector (m/s)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// Modified equinoctial elements (p, f, g, h, k, L)
///
/// # Strategy
/// Converts via classical orbital elements: rv → coe → equinoctial
///
/// # Example
/// ```
/// use astrora_core::core::elements::rv_to_equinoctial;
/// use astrora_core::core::linalg::Vector3;
/// use astrora_core::core::constants::GM_EARTH;
///
/// let r = Vector3::new(7000e3, 0.0, 0.0);
/// let v = Vector3::new(0.0, 7.5e3, 0.0);
/// let eq_elements = rv_to_equinoctial(&r, &v, GM_EARTH, 1e-8).unwrap();
/// ```
pub fn rv_to_equinoctial(
    r: &Vector3,
    v: &Vector3,
    mu: f64,
    tol: f64,
) -> PoliastroResult<EquinoctialElements> {
    let coe = rv_to_coe(r, v, mu, tol)?;
    Ok(coe_to_equinoctial(&coe))
}

/// Convert modified equinoctial elements to Cartesian state vectors
///
/// # Arguments
/// * `elements` - Modified equinoctial elements (p, f, g, h, k, L)
/// * `mu` - Standard gravitational parameter (m³/s²)
/// * `tol` - Tolerance for singularity checks (default: 1e-8)
///
/// # Returns
/// (position, velocity) tuple in inertial frame
///
/// # Strategy
/// Converts via classical orbital elements: equinoctial → coe → rv
///
/// # Example
/// ```
/// use astrora_core::core::elements::{EquinoctialElements, equinoctial_to_rv};
/// use astrora_core::core::constants::GM_EARTH;
///
/// let eq_elements = EquinoctialElements::new(
///     6.93e6, 0.01, 0.0, 0.044, 0.0, 0.5
/// );
/// let (r, v) = equinoctial_to_rv(&eq_elements, GM_EARTH, 1e-8).unwrap();
/// ```
pub fn equinoctial_to_rv(
    elements: &EquinoctialElements,
    mu: f64,
    tol: f64,
) -> PoliastroResult<(Vector3, Vector3)> {
    let coe = equinoctial_to_coe(elements, tol)?;
    Ok(coe_to_rv(&coe, mu))
}

/// Normalize angle to [0, 2π) range
#[inline]
fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % (2.0 * PI);
    if normalized < 0.0 {
        normalized += 2.0 * PI;
    }
    normalized
}

// ============================================================================
// Python Bindings
// ============================================================================

#[pymethods]
impl OrbitalElements {
    /// Create new orbital elements
    ///
    /// # Arguments
    /// * `a` - Semi-major axis (meters)
    /// * `e` - Eccentricity
    /// * `i` - Inclination (radians)
    /// * `raan` - Right ascension of ascending node (radians)
    /// * `argp` - Argument of periapsis (radians)
    /// * `nu` - True anomaly (radians)
    #[new]
    fn py_new(a: f64, e: f64, i: f64, raan: f64, argp: f64, nu: f64) -> Self {
        Self::new(a, e, i, raan, argp, nu)
    }

    /// Calculate orbital period for elliptical orbits (seconds)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    fn orbital_period(&self, mu: f64) -> PyResult<f64> {
        self.period(mu).map_err(|e| e.into())
    }

    /// Calculate periapsis distance (m)
    #[getter]
    fn periapsis_distance(&self) -> f64 {
        self.periapsis()
    }

    /// Calculate apoapsis distance (m)
    #[getter]
    fn apoapsis_distance(&self) -> f64 {
        self.apoapsis()
    }

    /// Calculate semi-latus rectum (m)
    #[getter]
    fn p(&self) -> f64 {
        self.semi_latus_rectum()
    }

    /// Calculate specific angular momentum magnitude (m²/s)
    fn h_magnitude(&self, mu: f64) -> f64 {
        self.specific_angular_momentum_magnitude(mu)
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "OrbitalElements(a={:.3e} m, e={:.6}, i={:.6} rad, Ω={:.6} rad, ω={:.6} rad, ν={:.6} rad)",
            self.a, self.e, self.i, self.raan, self.argp, self.nu
        )
    }

    fn __str__(&self) -> String {
        format!(
            "a = {:.3} km\ne = {:.6}\ni = {:.3}°\nΩ = {:.3}°\nω = {:.3}°\nν = {:.3}°",
            self.a / 1000.0,
            self.e,
            self.i.to_degrees(),
            self.raan.to_degrees(),
            self.argp.to_degrees(),
            self.nu.to_degrees()
        )
    }
}

#[pymethods]
impl EquinoctialElements {
    /// Create new modified equinoctial elements
    ///
    /// # Arguments
    /// * `p` - Semi-latus rectum (meters)
    /// * `f` - Eccentricity x-component
    /// * `g` - Eccentricity y-component
    /// * `h` - Inclination x-component
    /// * `k` - Inclination y-component
    /// * `L` - True longitude (radians)
    #[new]
    fn py_new(p: f64, f: f64, g: f64, h: f64, k: f64, L: f64) -> Self {
        Self::new(p, f, g, h, k, L)
    }

    /// Calculate eccentricity
    #[getter]
    fn eccentricity_value(&self) -> f64 {
        self.eccentricity()
    }

    /// Calculate semi-major axis for elliptical orbits (meters)
    #[getter]
    fn semi_major_axis_value(&self) -> PyResult<f64> {
        self.semi_major_axis().map_err(|e| e.into())
    }

    /// Calculate inclination (radians)
    #[getter]
    fn inclination_value(&self) -> f64 {
        self.inclination()
    }

    /// Calculate orbital period for elliptical orbits (seconds)
    ///
    /// # Arguments
    /// * `mu` - Standard gravitational parameter (m³/s²)
    fn orbital_period(&self, mu: f64) -> PyResult<f64> {
        self.period(mu).map_err(|e| e.into())
    }

    /// Convert to classical orbital elements
    ///
    /// # Arguments
    /// * `tol` - Tolerance for singularity checks (default: 1e-8)
    #[pyo3(signature = (tol=None))]
    fn to_classical(&self, tol: Option<f64>) -> PyResult<OrbitalElements> {
        equinoctial_to_coe(self, tol.unwrap_or(DEFAULT_TOL)).map_err(|e| e.into())
    }

    /// Create from classical orbital elements
    ///
    /// # Arguments
    /// * `elements` - Classical orbital elements
    #[staticmethod]
    fn from_classical(elements: &OrbitalElements) -> Self {
        coe_to_equinoctial(elements)
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "EquinoctialElements(p={:.3e} m, f={:.6}, g={:.6}, h={:.6}, k={:.6}, L={:.6} rad)",
            self.p, self.f, self.g, self.h, self.k, self.L
        )
    }

    fn __str__(&self) -> String {
        let e = self.eccentricity();
        let i = self.inclination();
        format!(
            "p = {:.3} km\nf = {:.6}\ng = {:.6}\nh = {:.6}\nk = {:.6}\nL = {:.6} rad\n(derived: e = {:.6}, i = {:.3}°)",
            self.p / 1000.0,
            self.f,
            self.g,
            self.h,
            self.k,
            self.L,
            e,
            i.to_degrees()
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::GM_EARTH;
    use approx::assert_relative_eq;

    #[test]
    fn test_orbital_elements_period() {
        // ISS-like orbit: a ≈ 6778 km, e ≈ 0.0001
        let elements = OrbitalElements::new(6778e3, 0.0001, 0.0, 0.0, 0.0, 0.0);
        let period = elements.period(GM_EARTH).unwrap();

        // ISS orbital period is about 92 minutes
        let expected_period = 92.0 * 60.0; // seconds
        assert_relative_eq!(period, expected_period, epsilon = 100.0); // Within 100 seconds
    }

    #[test]
    fn test_orbital_elements_periapsis_apoapsis() {
        let a = 7000e3; // 7000 km
        let e = 0.1; // 10% eccentricity
        let elements = OrbitalElements::new(a, e, 0.0, 0.0, 0.0, 0.0);

        let r_p = elements.periapsis();
        let r_a = elements.apoapsis();

        assert_relative_eq!(r_p, a * (1.0 - e), epsilon = 1.0);
        assert_relative_eq!(r_a, a * (1.0 + e), epsilon = 1.0);
        assert_relative_eq!(r_p, 6300e3, epsilon = 1.0);
        assert_relative_eq!(r_a, 7700e3, epsilon = 1.0);
    }

    #[test]
    fn test_rv_to_coe_circular_equatorial() {
        // Circular equatorial orbit at 7000 km altitude
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v_mag = (GM_EARTH / 7000e3).sqrt(); // Circular velocity
        let v = Vector3::new(0.0, v_mag, 0.0);

        let elements = rv_to_coe(&r, &v, GM_EARTH, DEFAULT_TOL).unwrap();

        // Check semi-major axis
        assert_relative_eq!(elements.a, 7000e3, epsilon = 1.0);

        // Check eccentricity (should be very small)
        assert!(elements.e < DEFAULT_TOL);

        // Check inclination (should be near 0 for equatorial)
        assert!(elements.i < DEFAULT_TOL);
    }

    #[test]
    fn test_rv_to_coe_elliptical() {
        // Elliptical orbit
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v = Vector3::new(0.0, 8000.0, 0.0); // Higher than circular velocity

        let elements = rv_to_coe(&r, &v, GM_EARTH, DEFAULT_TOL).unwrap();

        // Check that eccentricity is positive (elliptical)
        assert!(elements.e > 0.0);
        assert!(elements.e < 1.0); // Must be elliptical
    }

    #[test]
    fn test_rv_to_coe_inclined() {
        // Inclined circular orbit
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v_mag = (GM_EARTH / 7000e3).sqrt();
        // Velocity at 45° inclination
        let angle = PI / 4.0;
        let v = Vector3::new(0.0, v_mag * angle.cos(), v_mag * angle.sin());

        let elements = rv_to_coe(&r, &v, GM_EARTH, DEFAULT_TOL).unwrap();

        // Check inclination is close to 45°
        assert_relative_eq!(elements.i, PI / 4.0, epsilon = 1e-6);
    }

    #[test]
    fn test_coe_to_rv_circular_equatorial() {
        // Circular equatorial orbit
        let elements = OrbitalElements::new(7000e3, 0.0, 0.0, 0.0, 0.0, 0.0);

        let (r, v) = coe_to_rv(&elements, GM_EARTH);

        // Position should be along x-axis (at periapsis, nu=0)
        assert_relative_eq!(r.norm(), 7000e3, epsilon = 1.0);
        assert_relative_eq!(r.x, 7000e3, epsilon = 1.0);
        assert_relative_eq!(r.y, 0.0, epsilon = 1.0);
        assert_relative_eq!(r.z, 0.0, epsilon = 1.0);

        // Velocity should be circular
        let v_expected = (GM_EARTH / 7000e3).sqrt();
        assert_relative_eq!(v.norm(), v_expected, epsilon = 1.0);
    }

    #[test]
    fn test_coe_to_rv_elliptical() {
        // Elliptical orbit: a = 8000 km, e = 0.1
        let elements = OrbitalElements::new(8000e3, 0.1, 0.0, 0.0, 0.0, 0.0);

        let (r, _v) = coe_to_rv(&elements, GM_EARTH);

        // At periapsis (nu = 0), r = a(1-e)
        let r_expected = 8000e3 * (1.0 - 0.1);
        assert_relative_eq!(r.norm(), r_expected, epsilon = 1.0);
    }

    #[test]
    fn test_roundtrip_conversion_circular() {
        // Test rv -> coe -> rv for circular orbit
        let r_orig = Vector3::new(7000e3, 0.0, 0.0);
        let v_mag = (GM_EARTH / 7000e3).sqrt();
        let v_orig = Vector3::new(0.0, v_mag, 0.0);

        // Convert to COE
        let elements = rv_to_coe(&r_orig, &v_orig, GM_EARTH, DEFAULT_TOL).unwrap();

        // Convert back to rv
        let (r_new, v_new) = coe_to_rv(&elements, GM_EARTH);

        // Check that we get back the original vectors
        assert_relative_eq!(r_new.x, r_orig.x, epsilon = 1.0);
        assert_relative_eq!(r_new.y, r_orig.y, epsilon = 1.0);
        assert_relative_eq!(r_new.z, r_orig.z, epsilon = 1.0);

        assert_relative_eq!(v_new.x, v_orig.x, epsilon = 0.1);
        assert_relative_eq!(v_new.y, v_orig.y, epsilon = 0.1);
        assert_relative_eq!(v_new.z, v_orig.z, epsilon = 0.1);
    }

    #[test]
    fn test_roundtrip_conversion_elliptical() {
        // Test rv -> coe -> rv for elliptical orbit
        let r_orig = Vector3::new(7000e3, 0.0, 0.0);
        let v_orig = Vector3::new(0.0, 8000.0, 0.0);

        // Convert to COE
        let elements = rv_to_coe(&r_orig, &v_orig, GM_EARTH, DEFAULT_TOL).unwrap();

        // Convert back to rv
        let (r_new, v_new) = coe_to_rv(&elements, GM_EARTH);

        // Check roundtrip accuracy
        assert_relative_eq!(r_new.x, r_orig.x, epsilon = 10.0);
        assert_relative_eq!(r_new.y, r_orig.y, epsilon = 10.0);
        assert_relative_eq!(r_new.z, r_orig.z, epsilon = 10.0);

        assert_relative_eq!(v_new.x, v_orig.x, epsilon = 1.0);
        assert_relative_eq!(v_new.y, v_orig.y, epsilon = 1.0);
        assert_relative_eq!(v_new.z, v_orig.z, epsilon = 1.0);
    }

    #[test]
    fn test_roundtrip_conversion_inclined() {
        // Test rv -> coe -> rv for inclined orbit
        let r_orig = Vector3::new(7000e3, 0.0, 0.0);
        let v_mag = (GM_EARTH / 7000e3).sqrt();
        let angle = PI / 3.0; // 60° inclination
        let v_orig = Vector3::new(0.0, v_mag * angle.cos(), v_mag * angle.sin());

        // Convert to COE
        let elements = rv_to_coe(&r_orig, &v_orig, GM_EARTH, DEFAULT_TOL).unwrap();

        // Convert back to rv
        let (r_new, v_new) = coe_to_rv(&elements, GM_EARTH);

        // Check roundtrip accuracy
        assert_relative_eq!(r_new.norm(), r_orig.norm(), epsilon = 10.0);
        assert_relative_eq!(v_new.norm(), v_orig.norm(), epsilon = 1.0);

        // Check inclination is preserved
        let h_new = r_new.cross(&v_new);
        let i_check = (h_new.z / h_new.norm()).acos();
        assert_relative_eq!(i_check, angle, epsilon = 1e-6);
    }

    #[test]
    fn test_semi_latus_rectum() {
        let elements = OrbitalElements::new(7000e3, 0.1, 0.0, 0.0, 0.0, 0.0);
        let p = elements.semi_latus_rectum();

        let expected = 7000e3 * (1.0 - 0.1 * 0.1);
        assert_relative_eq!(p, expected, epsilon = 1.0);
    }

    #[test]
    fn test_rv_to_coe_rejects_zero_angular_momentum() {
        // Radial trajectory (zero angular momentum)
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v = Vector3::new(1000.0, 0.0, 0.0); // Radial velocity only

        let result = rv_to_coe(&r, &v, GM_EARTH, DEFAULT_TOL);
        assert!(result.is_err());
    }

    // ============================================================================
    // Equinoctial Elements Tests
    // ============================================================================

    #[test]
    fn test_coe_to_equinoctial_circular_equatorial() {
        // Circular equatorial orbit
        let coe = OrbitalElements::new(7000e3, 0.0, 0.0, 0.0, 0.0, 0.0);
        let eq = coe_to_equinoctial(&coe);

        // For circular equatorial orbit:
        // f = g = 0 (e = 0)
        // h = k = 0 (i = 0)
        assert_relative_eq!(eq.f, 0.0, epsilon = DEFAULT_TOL);
        assert_relative_eq!(eq.g, 0.0, epsilon = DEFAULT_TOL);
        assert_relative_eq!(eq.h, 0.0, epsilon = DEFAULT_TOL);
        assert_relative_eq!(eq.k, 0.0, epsilon = DEFAULT_TOL);
        assert_relative_eq!(eq.p, 7000e3, epsilon = 1.0);
    }

    #[test]
    fn test_coe_to_equinoctial_elliptical() {
        // Elliptical orbit: a = 8000 km, e = 0.1
        let coe = OrbitalElements::new(8000e3, 0.1, 0.0, 0.0, 0.0, 0.0);
        let eq = coe_to_equinoctial(&coe);

        // Check semi-latus rectum
        let p_expected = 8000e3 * (1.0 - 0.1 * 0.1);
        assert_relative_eq!(eq.p, p_expected, epsilon = 1.0);

        // Check eccentricity components
        let e_recovered = (eq.f * eq.f + eq.g * eq.g).sqrt();
        assert_relative_eq!(e_recovered, 0.1, epsilon = 1e-10);

        // For equatorial orbit, h = k = 0
        assert_relative_eq!(eq.h, 0.0, epsilon = DEFAULT_TOL);
        assert_relative_eq!(eq.k, 0.0, epsilon = DEFAULT_TOL);
    }

    #[test]
    fn test_coe_to_equinoctial_inclined() {
        // Inclined circular orbit: i = 30°
        let inc = PI / 6.0; // 30 degrees
        let coe = OrbitalElements::new(7000e3, 0.0, inc, 0.0, 0.0, 0.0);
        let eq = coe_to_equinoctial(&coe);

        // Check inclination recovery
        let i_recovered = 2.0 * (eq.h * eq.h + eq.k * eq.k).sqrt().atan();
        assert_relative_eq!(i_recovered, inc, epsilon = 1e-10);

        // For circular orbit, f = g = 0
        assert_relative_eq!(eq.f, 0.0, epsilon = DEFAULT_TOL);
        assert_relative_eq!(eq.g, 0.0, epsilon = DEFAULT_TOL);
    }

    #[test]
    fn test_equinoctial_to_coe_circular_equatorial() {
        // Circular equatorial orbit in equinoctial elements
        let eq = EquinoctialElements::new(7000e3, 0.0, 0.0, 0.0, 0.0, 0.5);
        let coe = equinoctial_to_coe(&eq, DEFAULT_TOL).unwrap();

        // Check semi-major axis
        assert_relative_eq!(coe.a, 7000e3, epsilon = 1.0);

        // Check eccentricity (should be very small)
        assert!(coe.e < DEFAULT_TOL);

        // Check inclination (should be near 0)
        assert!(coe.i < DEFAULT_TOL);
    }

    #[test]
    fn test_equinoctial_to_coe_elliptical() {
        // Elliptical orbit
        let eq = EquinoctialElements::new(7.92e6, 0.1, 0.0, 0.0, 0.0, 0.5);
        let coe = equinoctial_to_coe(&eq, DEFAULT_TOL).unwrap();

        // Check eccentricity
        assert_relative_eq!(coe.e, 0.1, epsilon = 1e-10);

        // Check semi-major axis: a = p / (1 - e²)
        let a_expected = 7.92e6 / (1.0 - 0.1 * 0.1);
        assert_relative_eq!(coe.a, a_expected, epsilon = 1.0);
    }

    #[test]
    fn test_roundtrip_coe_equinoctial_circular() {
        // Test COE -> Equinoctial -> COE for circular orbit
        let coe_orig = OrbitalElements::new(7000e3, 0.0, 0.0, 0.0, 0.0, 0.5);
        let eq = coe_to_equinoctial(&coe_orig);
        let coe_new = equinoctial_to_coe(&eq, DEFAULT_TOL).unwrap();

        assert_relative_eq!(coe_new.a, coe_orig.a, epsilon = 1.0);
        assert_relative_eq!(coe_new.e, coe_orig.e, epsilon = 1e-10);
        assert_relative_eq!(coe_new.i, coe_orig.i, epsilon = 1e-10);
        assert_relative_eq!(coe_new.nu, coe_orig.nu, epsilon = 1e-8);
    }

    #[test]
    fn test_roundtrip_coe_equinoctial_elliptical() {
        // Test COE -> Equinoctial -> COE for elliptical orbit
        let coe_orig = OrbitalElements::new(8000e3, 0.1, PI/4.0, 0.5, 0.3, 0.7);
        let eq = coe_to_equinoctial(&coe_orig);
        let coe_new = equinoctial_to_coe(&eq, DEFAULT_TOL).unwrap();

        assert_relative_eq!(coe_new.a, coe_orig.a, epsilon = 1.0);
        assert_relative_eq!(coe_new.e, coe_orig.e, epsilon = 1e-10);
        assert_relative_eq!(coe_new.i, coe_orig.i, epsilon = 1e-10);
        assert_relative_eq!(coe_new.raan, coe_orig.raan, epsilon = 1e-8);
        assert_relative_eq!(coe_new.argp, coe_orig.argp, epsilon = 1e-8);
        assert_relative_eq!(coe_new.nu, coe_orig.nu, epsilon = 1e-8);
    }

    #[test]
    fn test_rv_to_equinoctial_circular() {
        // Circular orbit
        let r = Vector3::new(7000e3, 0.0, 0.0);
        let v_mag = (GM_EARTH / 7000e3).sqrt();
        let v = Vector3::new(0.0, v_mag, 0.0);

        let eq = rv_to_equinoctial(&r, &v, GM_EARTH, DEFAULT_TOL).unwrap();

        // Check eccentricity (should be very small)
        let e = eq.eccentricity();
        assert!(e < DEFAULT_TOL);

        // Check semi-major axis
        let a = eq.semi_major_axis().unwrap();
        assert_relative_eq!(a, 7000e3, epsilon = 1.0);
    }

    #[test]
    fn test_equinoctial_to_rv_circular() {
        // Circular orbit
        let eq = EquinoctialElements::new(7000e3, 0.0, 0.0, 0.0, 0.0, 0.0);
        let (r, v) = equinoctial_to_rv(&eq, GM_EARTH, DEFAULT_TOL).unwrap();

        // Check radius
        assert_relative_eq!(r.norm(), 7000e3, epsilon = 1.0);

        // Check velocity magnitude (circular)
        let v_expected = (GM_EARTH / 7000e3).sqrt();
        assert_relative_eq!(v.norm(), v_expected, epsilon = 1.0);
    }

    #[test]
    fn test_roundtrip_rv_equinoctial() {
        // Test rv -> equinoctial -> rv roundtrip
        let r_orig = Vector3::new(7000e3, 0.0, 0.0);
        let v_orig = Vector3::new(0.0, 8000.0, 0.0);

        let eq = rv_to_equinoctial(&r_orig, &v_orig, GM_EARTH, DEFAULT_TOL).unwrap();
        let (r_new, v_new) = equinoctial_to_rv(&eq, GM_EARTH, DEFAULT_TOL).unwrap();

        assert_relative_eq!(r_new.x, r_orig.x, epsilon = 10.0);
        assert_relative_eq!(r_new.y, r_orig.y, epsilon = 10.0);
        assert_relative_eq!(r_new.z, r_orig.z, epsilon = 10.0);

        assert_relative_eq!(v_new.x, v_orig.x, epsilon = 1.0);
        assert_relative_eq!(v_new.y, v_orig.y, epsilon = 1.0);
        assert_relative_eq!(v_new.z, v_orig.z, epsilon = 1.0);
    }

    #[test]
    fn test_equinoctial_elements_eccentricity() {
        let eq = EquinoctialElements::new(7000e3, 0.06, 0.08, 0.0, 0.0, 0.0);
        let e = eq.eccentricity();

        // e = √(f² + g²) = √(0.06² + 0.08²) = √(0.0036 + 0.0064) = √0.01 = 0.1
        assert_relative_eq!(e, 0.1, epsilon = 1e-10);
    }

    #[test]
    fn test_equinoctial_elements_inclination() {
        // For i = 60°, tan(i/2) = tan(30°) = 1/√3 ≈ 0.577
        let tan_half_i = (PI / 6.0).tan(); // 30°
        let eq = EquinoctialElements::new(7000e3, 0.0, 0.0, tan_half_i, 0.0, 0.0);
        let i = eq.inclination();

        assert_relative_eq!(i, PI / 3.0, epsilon = 1e-10); // 60°
    }

    #[test]
    fn test_equinoctial_elements_period() {
        // Circular orbit at 7000 km
        let eq = EquinoctialElements::new(7000e3, 0.0, 0.0, 0.0, 0.0, 0.0);
        let period = eq.period(GM_EARTH).unwrap();

        // Expected period ≈ 97 minutes (more accurate for 7000 km altitude)
        let expected_period = 97.0 * 60.0;
        assert_relative_eq!(period, expected_period, epsilon = 100.0);
    }

    #[test]
    fn test_equinoctial_singularity_free_near_circular() {
        // Near-circular orbit (e = 1e-10) - would be problematic for classical elements
        let coe = OrbitalElements::new(7000e3, 1e-10, PI/4.0, 0.5, 0.0, 0.3);
        let eq = coe_to_equinoctial(&coe);

        // Should successfully convert back
        let coe_recovered = equinoctial_to_coe(&eq, DEFAULT_TOL).unwrap();
        assert_relative_eq!(coe_recovered.e, 1e-10, epsilon = 1e-12);
        assert_relative_eq!(coe_recovered.a, 7000e3, epsilon = 1.0);
    }

    #[test]
    fn test_equinoctial_singularity_free_near_equatorial() {
        // Near-equatorial orbit (i = 1e-10) - would be problematic for classical elements
        let coe = OrbitalElements::new(7000e3, 0.1, 1e-10, 0.0, 0.3, 0.5);
        let eq = coe_to_equinoctial(&coe);

        // Should successfully convert back
        let coe_recovered = equinoctial_to_coe(&eq, DEFAULT_TOL).unwrap();
        assert_relative_eq!(coe_recovered.e, 0.1, epsilon = 1e-10);
        assert_relative_eq!(coe_recovered.i, 1e-10, epsilon = 1e-12);
    }
}
