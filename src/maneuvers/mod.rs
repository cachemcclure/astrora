//! Orbital maneuvers module
//!
//! This module provides calculations for various orbital maneuvers including:
//! - Hohmann transfers (two-impulse orbital transfers)
//! - Bi-elliptic transfers (three-impulse orbital transfers)
//! - Plane change maneuvers
//! - Combined maneuvers
//! - Rendezvous and phasing orbits
//! - Gravity assists (planetary flybys)
//! - Delta-v budget tracking and propellant mass calculations
//! - Lambert's problem (orbital boundary value problem solver)
//!
//! # References
//! - Curtis, H. D. (2013). Orbital Mechanics for Engineering Students (3rd ed.). Ch. 5, 6, 8
//! - Vallado, D. A. (2013). Fundamentals of Astrodynamics and Applications (4th ed.). Ch. 6, 7, 8
//! - <https://orbital-mechanics.space/orbital-maneuvers/hohmann-transfer.html>
//! - <https://orbital-mechanics.space/orbital-maneuvers/bielliptic-hohmann-transfer.html>
//! - <https://orbital-mechanics.space/orbital-maneuvers/plane-change-maneuvers.html>
//! - <https://orbital-mechanics.space/orbital-maneuvers/phasing-maneuvers.html>
//! - <https://orbital-mechanics.space/interplanetary-maneuvers/gravity-assists.html>
//! - <https://orbital-mechanics.space/lamberts-problem/lamberts-problem.html>

pub mod bielliptic;
pub mod budget;
pub mod gravityassist;
pub mod hohmann;
pub mod lambert;
pub mod planechange;
pub mod rendezvous;

// Re-export main types
pub use bielliptic::{BiellipticTransfer, BiellipticTransferResult};
pub use budget::{DeltaVBudget, DeltaVBudgetResult, DeltaVManeuver};
pub use gravityassist::{
    BPlaneParameters, GravityAssist, GravityAssistResult, GravityAssistVelocities,
};
pub use hohmann::{HohmannTransfer, HohmannTransferResult};
pub use lambert::{Lambert, LambertSolution, TransferKind};
pub use planechange::{
    CombinedPlaneChangeResult, OptimalPlaneChangeResult, PlaneChange, PlaneChangeResult,
};
pub use rendezvous::{
    CoorbitalRendezvousResult, CoplanarRendezvousResult, PhasingOrbitResult, Rendezvous,
};
