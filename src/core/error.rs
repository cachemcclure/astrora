//! Error types for astrora astrodynamics calculations
//!
//! This module defines a comprehensive error hierarchy for orbital mechanics
//! computations, including numerical failures, physical constraint violations,
//! and conversion errors. All errors integrate seamlessly with PyO3 for
//! automatic conversion to Python exceptions.

use pyo3::exceptions::{PyRuntimeError, PyValueError, PyTypeError, PyArithmeticError};
use pyo3::PyErr;
use thiserror::Error;

/// Main error type for astrora operations
///
/// This enum covers all error cases that can occur during astrodynamics
/// calculations, from numerical convergence failures to physical constraint
/// violations.
#[derive(Debug, Error)]
pub enum PoliastroError {
    // ========================================================================
    // Numerical and Mathematical Errors
    // ========================================================================
    /// Iterative solver failed to converge within allowed iterations
    ///
    /// Common in Kepler's equation solving, Lambert's problem, and
    /// root-finding algorithms.
    #[error("Convergence failure: {method} failed to converge after {iterations} iterations (tolerance: {tolerance})")]
    ConvergenceFailure {
        method: String,
        iterations: usize,
        tolerance: f64,
    },

    /// Numerical value is invalid (NaN, Infinity, or denormal)
    #[error("Invalid numerical value encountered: {context} = {value}")]
    InvalidNumericalValue {
        context: String,
        value: f64,
    },

    /// Division by zero or near-zero value
    #[error("Division by zero or near-zero value in {context}: divisor = {divisor}")]
    DivisionByZero {
        context: String,
        divisor: f64,
    },

    /// Numerical instability detected
    #[error("Numerical instability detected in {operation}: {details}")]
    NumericalInstability {
        operation: String,
        details: String,
    },

    /// Matrix is singular or near-singular
    #[error("Singular matrix encountered in {context}: determinant = {determinant}")]
    SingularMatrix {
        context: String,
        determinant: f64,
    },

    // ========================================================================
    // Physical Constraint Violations
    // ========================================================================
    /// Physical parameter violates required constraints
    #[error("Invalid parameter '{parameter}': value {value} {constraint}")]
    InvalidParameter {
        parameter: String,
        value: f64,
        constraint: String,
    },

    /// Orbital eccentricity is out of valid range
    #[error("Invalid eccentricity: {value} (must be >= 0)")]
    InvalidEccentricity {
        value: f64,
    },

    /// Semi-major axis is invalid
    #[error("Invalid semi-major axis: {value} km (must be > 0 for elliptic orbits)")]
    InvalidSemiMajorAxis {
        value: f64,
    },

    /// Inclination is out of valid range [0, π]
    #[error("Invalid inclination: {value} rad (must be in [0, π])")]
    InvalidInclination {
        value: f64,
    },

    /// Angular value is out of valid range
    #[error("Invalid angle '{name}': {value} rad (expected range: [{min}, {max}])")]
    InvalidAngle {
        name: String,
        value: f64,
        min: f64,
        max: f64,
    },

    /// Orbit violates energy conservation
    #[error("Energy conservation violated: ΔE = {delta_energy} (tolerance: {tolerance})")]
    EnergyNotConserved {
        delta_energy: f64,
        tolerance: f64,
    },

    /// Orbit violates angular momentum conservation
    #[error("Angular momentum conservation violated: ΔL = {delta_momentum} (tolerance: {tolerance})")]
    MomentumNotConserved {
        delta_momentum: f64,
        tolerance: f64,
    },

    // ========================================================================
    // State and Coordinate Errors
    // ========================================================================
    /// State vector contains invalid values
    #[error("Invalid state vector: {reason}")]
    InvalidStateVector {
        reason: String,
    },

    /// Position vector is zero or near-zero
    #[error("Zero or near-zero position vector: |r| = {magnitude} km")]
    ZeroPosition {
        magnitude: f64,
    },

    /// Velocity vector is zero or near-zero
    #[error("Zero or near-zero velocity vector: |v| = {magnitude} km/s")]
    ZeroVelocity {
        magnitude: f64,
    },

    /// Coordinate transformation failed
    #[error("Coordinate transformation failed from {from_frame} to {to_frame}: {reason}")]
    TransformationFailure {
        from_frame: String,
        to_frame: String,
        reason: String,
    },

    /// Singularity in orbital element conversion
    ///
    /// Occurs for circular orbits (e=0), equatorial orbits (i=0), or
    /// critical inclination cases.
    #[error("Singularity in orbital elements: {singularity_type} (consider using {alternative})")]
    OrbitalSingularity {
        singularity_type: String,
        alternative: String,
    },

    // ========================================================================
    // Integration and Propagation Errors
    // ========================================================================
    /// Numerical integrator failed
    #[error("Integration failure in {integrator}: {reason}")]
    IntegrationFailure {
        integrator: String,
        reason: String,
    },

    /// Time step is too large for accurate integration
    #[error("Time step too large: {step_size} s (recommended: < {max_recommended} s for {orbit_type})")]
    TimeStepTooLarge {
        step_size: f64,
        max_recommended: f64,
        orbit_type: String,
    },

    /// Propagation diverged or became unstable
    #[error("Propagation diverged after {time} s: {reason}")]
    PropagationDivergence {
        time: f64,
        reason: String,
    },

    /// Propagation failed for a specific context
    #[error("Propagation failed in {context}: {source}")]
    PropagationFailed {
        context: String,
        source: Box<PoliastroError>,
    },

    // ========================================================================
    // Input Validation Errors
    // ========================================================================
    /// Input value is out of acceptable range
    #[error("Value out of range for '{parameter}': {value} (expected: [{min}, {max}])")]
    OutOfRange {
        parameter: String,
        value: f64,
        min: f64,
        max: f64,
    },

    /// Incompatible units provided
    #[error("Incompatible units: expected {expected}, got {actual}")]
    IncompatibleUnits {
        expected: String,
        actual: String,
    },

    /// Invalid time value or epoch
    #[error("Invalid time value: {reason}")]
    InvalidTime {
        reason: String,
    },

    /// Missing required parameter
    #[error("Missing required parameter: {parameter}")]
    MissingParameter {
        parameter: String,
    },

    // ========================================================================
    // Orbit Type Errors
    // ========================================================================
    /// Operation not supported for this orbit type
    #[error("Operation '{operation}' not supported for {orbit_type} orbits")]
    UnsupportedOrbitType {
        operation: String,
        orbit_type: String,
    },

    /// Orbit type cannot be determined
    #[error("Cannot determine orbit type: {reason}")]
    AmbiguousOrbitType {
        reason: String,
    },

    // ========================================================================
    // General Errors
    // ========================================================================
    /// Computation error with custom message
    #[error("Computation error: {message}")]
    ComputationError {
        message: String,
    },

    /// Not implemented yet
    #[error("Not implemented: {feature}")]
    NotImplemented {
        feature: String,
    },

    /// Internal error (bug in the library)
    #[error("Internal error: {message} (this is a bug, please report it)")]
    InternalError {
        message: String,
    },
}

/// Result type alias for astrora operations
///
/// This is a convenience alias for `Result<T, PoliastroError>` that should
/// be used throughout the library for consistency.
///
/// Note: The type is still called PoliastroError/PoliastroResult internally
/// for backward compatibility, but represents astrora's error handling.
pub type PoliastroResult<T> = Result<T, PoliastroError>;

// ============================================================================
// PyO3 Integration
// ============================================================================

impl From<PoliastroError> for PyErr {
    /// Convert PoliastroError to Python exception
    ///
    /// Maps different error variants to appropriate Python exception types:
    /// - Numerical errors → ArithmeticError
    /// - Invalid parameters → ValueError
    /// - Not implemented → NotImplementedError (via RuntimeError)
    /// - Internal errors → RuntimeError
    fn from(err: PoliastroError) -> PyErr {
        use PoliastroError::*;

        match err {
            // Numerical/mathematical errors → ArithmeticError
            ConvergenceFailure { .. }
            | DivisionByZero { .. }
            | NumericalInstability { .. }
            | SingularMatrix { .. } => PyArithmeticError::new_err(err.to_string()),

            // Invalid numerical values → ValueError
            InvalidNumericalValue { .. } => PyValueError::new_err(err.to_string()),

            // Physical constraint violations → ValueError
            InvalidParameter { .. }
            | InvalidEccentricity { .. }
            | InvalidSemiMajorAxis { .. }
            | InvalidInclination { .. }
            | InvalidAngle { .. }
            | EnergyNotConserved { .. }
            | MomentumNotConserved { .. } => PyValueError::new_err(err.to_string()),

            // State and coordinate errors → ValueError
            InvalidStateVector { .. }
            | ZeroPosition { .. }
            | ZeroVelocity { .. }
            | OrbitalSingularity { .. } => PyValueError::new_err(err.to_string()),

            // Transformation errors → RuntimeError
            TransformationFailure { .. } => PyRuntimeError::new_err(err.to_string()),

            // Integration errors → RuntimeError
            IntegrationFailure { .. }
            | PropagationDivergence { .. }
            | PropagationFailed { .. } => PyRuntimeError::new_err(err.to_string()),

            // Validation errors → ValueError
            OutOfRange { .. }
            | InvalidTime { .. }
            | MissingParameter { .. }
            | TimeStepTooLarge { .. } => PyValueError::new_err(err.to_string()),

            // Unit errors → TypeError
            IncompatibleUnits { .. } => PyTypeError::new_err(err.to_string()),

            // Orbit type errors → ValueError
            UnsupportedOrbitType { .. }
            | AmbiguousOrbitType { .. } => PyValueError::new_err(err.to_string()),

            // General errors → RuntimeError
            ComputationError { .. }
            | NotImplemented { .. }
            | InternalError { .. } => PyRuntimeError::new_err(err.to_string()),
        }
    }
}

// ============================================================================
// Convenience Constructors
// ============================================================================

impl PoliastroError {
    /// Create a convergence failure error
    pub fn convergence_failure(method: impl Into<String>, iterations: usize, tolerance: f64) -> Self {
        Self::ConvergenceFailure {
            method: method.into(),
            iterations,
            tolerance,
        }
    }

    /// Create an invalid parameter error
    pub fn invalid_parameter(
        parameter: impl Into<String>,
        value: f64,
        constraint: impl Into<String>,
    ) -> Self {
        Self::InvalidParameter {
            parameter: parameter.into(),
            value,
            constraint: constraint.into(),
        }
    }

    /// Create an out of range error
    pub fn out_of_range(parameter: impl Into<String>, value: f64, min: f64, max: f64) -> Self {
        Self::OutOfRange {
            parameter: parameter.into(),
            value,
            min,
            max,
        }
    }

    /// Create an invalid state vector error
    pub fn invalid_state(reason: impl Into<String>) -> Self {
        Self::InvalidStateVector {
            reason: reason.into(),
        }
    }

    /// Create a not implemented error
    pub fn not_implemented(feature: impl Into<String>) -> Self {
        Self::NotImplemented {
            feature: feature.into(),
        }
    }

    /// Create an internal error (indicates a bug)
    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_failure() {
        let err = PoliastroError::convergence_failure("Newton-Raphson", 100, 1e-10);
        assert!(err.to_string().contains("Newton-Raphson"));
        assert!(err.to_string().contains("100"));
    }

    #[test]
    fn test_invalid_parameter() {
        let err = PoliastroError::invalid_parameter("mass", -10.0, "must be positive");
        assert!(err.to_string().contains("mass"));
        assert!(err.to_string().contains("-10"));
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn test_out_of_range() {
        let err = PoliastroError::out_of_range("eccentricity", 1.5, 0.0, 1.0);
        assert!(err.to_string().contains("eccentricity"));
        assert!(err.to_string().contains("1.5"));
    }

    #[test]
    fn test_invalid_eccentricity() {
        let err = PoliastroError::InvalidEccentricity { value: -0.5 };
        assert!(err.to_string().contains("eccentricity"));
        assert!(err.to_string().contains("-0.5"));
    }

    #[test]
    fn test_division_by_zero() {
        let err = PoliastroError::DivisionByZero {
            context: "orbital period calculation".to_string(),
            divisor: 0.0,
        };
        assert!(err.to_string().contains("Division by zero"));
        assert!(err.to_string().contains("orbital period"));
    }

    #[test]
    fn test_orbital_singularity() {
        let err = PoliastroError::OrbitalSingularity {
            singularity_type: "circular orbit (e=0)".to_string(),
            alternative: "equinoctial elements".to_string(),
        };
        assert!(err.to_string().contains("Singularity"));
        assert!(err.to_string().contains("circular"));
        assert!(err.to_string().contains("equinoctial"));
    }

    #[test]
    fn test_not_implemented() {
        let err = PoliastroError::not_implemented("GPU acceleration");
        assert!(err.to_string().contains("Not implemented"));
        assert!(err.to_string().contains("GPU"));
    }

    #[test]
    fn test_internal_error() {
        let err = PoliastroError::internal("unexpected state in propagator");
        assert!(err.to_string().contains("Internal error"));
        assert!(err.to_string().contains("bug"));
    }

    #[test]
    fn test_energy_conservation() {
        let err = PoliastroError::EnergyNotConserved {
            delta_energy: 1e-6,
            tolerance: 1e-10,
        };
        let msg = err.to_string();
        assert!(msg.contains("Energy"));
        // Check for delta energy value (may be formatted as 0.000001 or 1e-6)
        assert!(msg.contains("0.000001") || msg.contains("1e-6"));
    }

    #[test]
    fn test_incompatible_units() {
        let err = PoliastroError::IncompatibleUnits {
            expected: "meters".to_string(),
            actual: "radians".to_string(),
        };
        assert!(err.to_string().contains("units"));
        assert!(err.to_string().contains("meters"));
        assert!(err.to_string().contains("radians"));
    }
}
