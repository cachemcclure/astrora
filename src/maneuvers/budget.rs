//! Delta-v budget tracking and propellant mass calculations
//!
//! A delta-v budget is an estimate of the total change in velocity required for a space
//! mission. It is calculated as the sum of delta-v required to perform each propulsive
//! maneuver needed during the mission.
//!
//! # Theory
//!
//! Delta-v is a scalar quantity that determines the propellant requirements for a mission.
//! The relationship between delta-v, spacecraft mass, and propellant is given by the
//! **Tsiolkovsky rocket equation**:
//!
//! ```text
//! Δv = Isp × g₀ × ln(m₀/m_f)
//! ```
//!
//! Where:
//! - Isp = specific impulse of the propulsion system (s)
//! - g₀ = standard gravity (9.80665 m/s²)
//! - m₀ = initial mass (including propellant)
//! - m_f = final mass (dry mass after propellant consumed)
//!
//! Rearranging to find propellant mass:
//! ```text
//! m_propellant = m_dry × (exp(Δv/(Isp × g₀)) - 1)
//! ```
//!
//! # Mission Design
//!
//! A typical delta-v budget enumerates:
//! 1. Various classes of maneuvers (orbital transfers, plane changes, rendezvous, etc.)
//! 2. Delta-v per maneuver
//! 3. Number of each maneuver type required
//! 4. Contingency margins for uncertainties
//!
//! The total is calculated by summing all individual maneuvers, similar to a financial budget.
//!
//! # Applications
//!
//! - Mission feasibility analysis
//! - Propellant requirements estimation
//! - Launch vehicle selection
//! - Trade studies between different mission profiles
//! - Verification that mission is achievable with given spacecraft
//!
//! # References
//! - <https://en.wikipedia.org/wiki/Delta-v_budget>
//! - <https://en.wikipedia.org/wiki/Tsiolkovsky_rocket_equation>
//! - Curtis, H. D. (2013). Orbital Mechanics for Engineering Students. Ch. 11
//! - Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications. Ch. 6

use crate::core::{PoliastroError, PoliastroResult};

/// Standard Earth gravity for propulsion calculations (m/s²)
const STANDARD_GRAVITY: f64 = 9.80665;

/// Individual maneuver entry in a delta-v budget
///
/// Represents a single impulsive maneuver with associated metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct DeltaVManeuver {
    /// Name or description of the maneuver (e.g., "Hohmann transfer to GEO")
    pub name: String,
    /// Delta-v required for this maneuver (m/s)
    pub delta_v: f64,
    /// Optional notes or additional information
    pub notes: Option<String>,
}

impl DeltaVManeuver {
    /// Create a new maneuver entry
    ///
    /// # Arguments
    /// * `name` - Description of the maneuver
    /// * `delta_v` - Delta-v required (m/s)
    ///
    /// # Returns
    /// A new `DeltaVManeuver` instance
    ///
    /// # Errors
    /// Returns error if delta_v is negative or NaN
    pub fn new(name: impl Into<String>, delta_v: f64) -> PoliastroResult<Self> {
        if !delta_v.is_finite() {
            return Err(PoliastroError::invalid_parameter(
                "delta_v",
                delta_v,
                "must be finite",
            ));
        }
        if delta_v < 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "delta_v",
                delta_v,
                "cannot be negative",
            ));
        }
        Ok(Self {
            name: name.into(),
            delta_v,
            notes: None,
        })
    }

    /// Create a new maneuver with notes
    ///
    /// # Arguments
    /// * `name` - Description of the maneuver
    /// * `delta_v` - Delta-v required (m/s)
    /// * `notes` - Additional notes or context
    pub fn with_notes(
        name: impl Into<String>,
        delta_v: f64,
        notes: impl Into<String>,
    ) -> PoliastroResult<Self> {
        let mut maneuver = Self::new(name, delta_v)?;
        maneuver.notes = Some(notes.into());
        Ok(maneuver)
    }
}

/// Delta-v budget for mission planning
///
/// Accumulates and tracks all maneuvers required for a space mission.
/// Provides methods for calculating total delta-v, propellant requirements,
/// and applying contingency margins.
#[derive(Debug, Clone, PartialEq)]
pub struct DeltaVBudget {
    /// Mission name or identifier
    pub mission_name: String,
    /// List of all maneuvers in the budget
    pub maneuvers: Vec<DeltaVManeuver>,
    /// Contingency margin as a fraction (e.g., 0.1 for 10%)
    pub contingency_margin: f64,
}

impl DeltaVBudget {
    /// Create a new empty delta-v budget
    ///
    /// # Arguments
    /// * `mission_name` - Name or identifier for the mission
    ///
    /// # Returns
    /// A new empty `DeltaVBudget` with 0% contingency margin
    pub fn new(mission_name: impl Into<String>) -> Self {
        Self {
            mission_name: mission_name.into(),
            maneuvers: Vec::new(),
            contingency_margin: 0.0,
        }
    }

    /// Create a budget with a contingency margin
    ///
    /// # Arguments
    /// * `mission_name` - Name or identifier for the mission
    /// * `contingency_margin` - Margin as fraction (e.g., 0.1 for 10%)
    ///
    /// # Returns
    /// A new `DeltaVBudget` with specified contingency
    ///
    /// # Errors
    /// Returns error if margin is negative or > 1.0
    pub fn with_contingency(
        mission_name: impl Into<String>,
        contingency_margin: f64,
    ) -> PoliastroResult<Self> {
        if !contingency_margin.is_finite() {
            return Err(PoliastroError::invalid_parameter(
                "contingency_margin",
                contingency_margin,
                "must be finite",
            ));
        }
        if !(0.0..=1.0).contains(&contingency_margin) {
            return Err(PoliastroError::out_of_range(
                "contingency_margin",
                contingency_margin,
                0.0,
                1.0,
            ));
        }
        Ok(Self {
            mission_name: mission_name.into(),
            maneuvers: Vec::new(),
            contingency_margin,
        })
    }

    /// Add a maneuver to the budget
    ///
    /// # Arguments
    /// * `maneuver` - The maneuver to add
    pub fn add_maneuver(&mut self, maneuver: DeltaVManeuver) {
        self.maneuvers.push(maneuver);
    }

    /// Add a maneuver by name and delta-v
    ///
    /// # Arguments
    /// * `name` - Description of the maneuver
    /// * `delta_v` - Delta-v required (m/s)
    ///
    /// # Errors
    /// Returns error if delta_v is invalid
    pub fn add(&mut self, name: impl Into<String>, delta_v: f64) -> PoliastroResult<()> {
        let maneuver = DeltaVManeuver::new(name, delta_v)?;
        self.add_maneuver(maneuver);
        Ok(())
    }

    /// Add a maneuver with notes
    ///
    /// # Arguments
    /// * `name` - Description of the maneuver
    /// * `delta_v` - Delta-v required (m/s)
    /// * `notes` - Additional notes
    pub fn add_with_notes(
        &mut self,
        name: impl Into<String>,
        delta_v: f64,
        notes: impl Into<String>,
    ) -> PoliastroResult<()> {
        let maneuver = DeltaVManeuver::with_notes(name, delta_v, notes)?;
        self.add_maneuver(maneuver);
        Ok(())
    }

    /// Calculate total delta-v (sum of all maneuvers)
    ///
    /// # Returns
    /// Total delta-v in m/s (without contingency)
    pub fn total_delta_v(&self) -> f64 {
        self.maneuvers.iter().map(|m| m.delta_v).sum()
    }

    /// Calculate total delta-v including contingency margin
    ///
    /// # Returns
    /// Total delta-v with contingency in m/s
    pub fn total_with_contingency(&self) -> f64 {
        self.total_delta_v() * (1.0 + self.contingency_margin)
    }

    /// Calculate propellant mass required using Tsiolkovsky rocket equation
    ///
    /// # Arguments
    /// * `dry_mass` - Spacecraft dry mass (kg)
    /// * `specific_impulse` - Engine specific impulse (s)
    ///
    /// # Returns
    /// Propellant mass required (kg) including contingency
    ///
    /// # Errors
    /// Returns error if inputs are invalid
    ///
    /// # Example
    /// ```
    /// # use poliastro::maneuvers::DeltaVBudget;
    /// # fn example() -> poliastro::core::PoliastroResult<()> {
    /// let mut budget = DeltaVBudget::new("LEO to GEO");
    /// budget.add("Hohmann transfer", 3896.0)?;
    ///
    /// // Calculate propellant for 1000 kg spacecraft with 300s Isp engine
    /// let propellant = budget.propellant_mass(1000.0, 300.0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn propellant_mass(&self, dry_mass: f64, specific_impulse: f64) -> PoliastroResult<f64> {
        if !dry_mass.is_finite() || dry_mass <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "dry_mass",
                dry_mass,
                "must be positive and finite",
            ));
        }
        if !specific_impulse.is_finite() || specific_impulse <= 0.0 {
            return Err(PoliastroError::invalid_parameter(
                "specific_impulse",
                specific_impulse,
                "must be positive and finite",
            ));
        }

        let total_dv = self.total_with_contingency();
        let exhaust_velocity = specific_impulse * STANDARD_GRAVITY;

        // Tsiolkovsky: m_prop = m_dry × (exp(Δv/v_e) - 1)
        let mass_ratio = (total_dv / exhaust_velocity).exp();
        let propellant = dry_mass * (mass_ratio - 1.0);

        Ok(propellant)
    }

    /// Calculate propellant mass fraction (propellant / total mass)
    ///
    /// # Arguments
    /// * `dry_mass` - Spacecraft dry mass (kg)
    /// * `specific_impulse` - Engine specific impulse (s)
    ///
    /// # Returns
    /// Propellant mass fraction (0.0 to 1.0)
    pub fn propellant_fraction(
        &self,
        dry_mass: f64,
        specific_impulse: f64,
    ) -> PoliastroResult<f64> {
        let propellant = self.propellant_mass(dry_mass, specific_impulse)?;
        let total_mass = dry_mass + propellant;
        Ok(propellant / total_mass)
    }

    /// Calculate total spacecraft mass (dry + propellant)
    ///
    /// # Arguments
    /// * `dry_mass` - Spacecraft dry mass (kg)
    /// * `specific_impulse` - Engine specific impulse (s)
    ///
    /// # Returns
    /// Total initial mass (kg)
    pub fn total_mass(&self, dry_mass: f64, specific_impulse: f64) -> PoliastroResult<f64> {
        let propellant = self.propellant_mass(dry_mass, specific_impulse)?;
        Ok(dry_mass + propellant)
    }

    /// Set contingency margin
    ///
    /// # Arguments
    /// * `margin` - Contingency as fraction (e.g., 0.1 for 10%)
    ///
    /// # Errors
    /// Returns error if margin is invalid
    pub fn set_contingency(&mut self, margin: f64) -> PoliastroResult<()> {
        if !margin.is_finite() {
            return Err(PoliastroError::invalid_parameter(
                "margin",
                margin,
                "must be finite",
            ));
        }
        if !(0.0..=1.0).contains(&margin) {
            return Err(PoliastroError::out_of_range(
                "margin",
                margin,
                0.0,
                1.0,
            ));
        }
        self.contingency_margin = margin;
        Ok(())
    }

    /// Get number of maneuvers in budget
    pub fn num_maneuvers(&self) -> usize {
        self.maneuvers.len()
    }

    /// Check if budget is empty
    pub fn is_empty(&self) -> bool {
        self.maneuvers.is_empty()
    }

    /// Clear all maneuvers from budget
    pub fn clear(&mut self) {
        self.maneuvers.clear();
    }
}

/// Result of delta-v budget calculation for Python API
///
/// Contains comprehensive budget information including propellant requirements.
#[derive(Debug, Clone, PartialEq)]
pub struct DeltaVBudgetResult {
    /// Mission name
    pub mission_name: String,
    /// Total delta-v (m/s) without contingency
    pub total_delta_v: f64,
    /// Contingency margin (fraction)
    pub contingency_margin: f64,
    /// Total delta-v with contingency (m/s)
    pub total_with_contingency: f64,
    /// Number of maneuvers
    pub num_maneuvers: usize,
    /// Dry mass of spacecraft (kg) - optional
    pub dry_mass: Option<f64>,
    /// Specific impulse (s) - optional
    pub specific_impulse: Option<f64>,
    /// Propellant mass required (kg) - optional
    pub propellant_mass: Option<f64>,
    /// Propellant mass fraction - optional
    pub propellant_fraction: Option<f64>,
    /// Total initial mass (kg) - optional
    pub total_mass: Option<f64>,
}

impl DeltaVBudget {
    /// Generate a result summary for Python API
    ///
    /// # Arguments
    /// * `dry_mass` - Optional spacecraft dry mass (kg)
    /// * `specific_impulse` - Optional engine Isp (s)
    ///
    /// # Returns
    /// A `DeltaVBudgetResult` with computed values
    pub fn result(
        &self,
        dry_mass: Option<f64>,
        specific_impulse: Option<f64>,
    ) -> PoliastroResult<DeltaVBudgetResult> {
        let mut result = DeltaVBudgetResult {
            mission_name: self.mission_name.clone(),
            total_delta_v: self.total_delta_v(),
            contingency_margin: self.contingency_margin,
            total_with_contingency: self.total_with_contingency(),
            num_maneuvers: self.num_maneuvers(),
            dry_mass,
            specific_impulse,
            propellant_mass: None,
            propellant_fraction: None,
            total_mass: None,
        };

        // Calculate propellant if both mass and Isp provided
        if let (Some(dm), Some(isp)) = (dry_mass, specific_impulse) {
            result.propellant_mass = Some(self.propellant_mass(dm, isp)?);
            result.propellant_fraction = Some(self.propellant_fraction(dm, isp)?);
            result.total_mass = Some(self.total_mass(dm, isp)?);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maneuver_creation() {
        let maneuver = DeltaVManeuver::new("Test", 1000.0).unwrap();
        assert_eq!(maneuver.name, "Test");
        assert_eq!(maneuver.delta_v, 1000.0);
        assert_eq!(maneuver.notes, None);
    }

    #[test]
    fn test_maneuver_with_notes() {
        let maneuver = DeltaVManeuver::with_notes("Test", 1000.0, "Important").unwrap();
        assert_eq!(maneuver.notes, Some("Important".to_string()));
    }

    #[test]
    fn test_maneuver_negative_delta_v() {
        let result = DeltaVManeuver::new("Test", -100.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_budget_creation() {
        let budget = DeltaVBudget::new("Test Mission");
        assert_eq!(budget.mission_name, "Test Mission");
        assert_eq!(budget.maneuvers.len(), 0);
        assert_eq!(budget.contingency_margin, 0.0);
    }

    #[test]
    fn test_budget_with_contingency() {
        let budget = DeltaVBudget::with_contingency("Test", 0.15).unwrap();
        assert_eq!(budget.contingency_margin, 0.15);
    }

    #[test]
    fn test_add_maneuver() {
        let mut budget = DeltaVBudget::new("Test");
        budget.add("Burn 1", 1000.0).unwrap();
        budget.add("Burn 2", 500.0).unwrap();

        assert_eq!(budget.num_maneuvers(), 2);
        assert_eq!(budget.total_delta_v(), 1500.0);
    }

    #[test]
    fn test_contingency_calculation() {
        let mut budget = DeltaVBudget::with_contingency("Test", 0.1).unwrap();
        budget.add("Burn", 1000.0).unwrap();

        assert_eq!(budget.total_delta_v(), 1000.0);
        assert_eq!(budget.total_with_contingency(), 1100.0);
    }

    #[test]
    fn test_propellant_mass_calculation() {
        let mut budget = DeltaVBudget::new("Test");
        budget.add("Burn", 3000.0).unwrap();

        // For 1000 kg spacecraft with 300s Isp
        let propellant = budget.propellant_mass(1000.0, 300.0).unwrap();

        // Expected: m_prop = 1000 × (exp(3000/(300×9.80665)) - 1)
        //         = 1000 × (exp(1.0194) - 1) ≈ 1000 × 1.772 ≈ 1772 kg
        assert!((propellant - 1772.0).abs() < 10.0);
    }

    #[test]
    fn test_propellant_fraction() {
        let mut budget = DeltaVBudget::new("Test");
        budget.add("Burn", 3000.0).unwrap();

        let fraction = budget.propellant_fraction(1000.0, 300.0).unwrap();

        // With ~1772 kg propellant and 1000 kg dry mass:
        // fraction = 1772 / (1000 + 1772) ≈ 0.639
        assert!((fraction - 0.639).abs() < 0.01);
    }

    #[test]
    fn test_leo_to_geo_mission() {
        // Realistic LEO to GEO mission
        let mut budget = DeltaVBudget::with_contingency("LEO to GEO", 0.1).unwrap();

        budget.add("Hohmann transfer burn 1", 2440.0).unwrap();
        budget.add("Hohmann transfer burn 2", 1475.0).unwrap();
        budget.add("Station keeping (per year)", 50.0).unwrap();

        assert_eq!(budget.num_maneuvers(), 3);

        let total = budget.total_delta_v();
        assert!((total - 3965.0).abs() < 1.0); // ~3965 m/s

        let with_margin = budget.total_with_contingency();
        assert!((with_margin - 4361.5).abs() < 1.0); // ~4362 m/s
    }

    #[test]
    fn test_budget_result_without_propellant() {
        let mut budget = DeltaVBudget::new("Test");
        budget.add("Burn", 1000.0).unwrap();

        let result = budget.result(None, None).unwrap();

        assert_eq!(result.mission_name, "Test");
        assert_eq!(result.total_delta_v, 1000.0);
        assert_eq!(result.num_maneuvers, 1);
        assert!(result.propellant_mass.is_none());
    }

    #[test]
    fn test_budget_result_with_propellant() {
        let mut budget = DeltaVBudget::new("Test");
        budget.add("Burn", 1000.0).unwrap();

        let result = budget.result(Some(1000.0), Some(300.0)).unwrap();

        assert!(result.propellant_mass.is_some());
        assert!(result.propellant_fraction.is_some());
        assert!(result.total_mass.is_some());
    }

    #[test]
    fn test_clear_budget() {
        let mut budget = DeltaVBudget::new("Test");
        budget.add("Burn 1", 1000.0).unwrap();
        budget.add("Burn 2", 500.0).unwrap();

        assert!(!budget.is_empty());
        budget.clear();
        assert!(budget.is_empty());
        assert_eq!(budget.total_delta_v(), 0.0);
    }

    #[test]
    fn test_invalid_contingency() {
        let result = DeltaVBudget::with_contingency("Test", 1.5);
        assert!(result.is_err());

        let result = DeltaVBudget::with_contingency("Test", -0.1);
        assert!(result.is_err());
    }
}
