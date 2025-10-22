//! State vector representations

use nalgebra as na;

/// Cartesian state vector (position and velocity)
#[derive(Debug, Clone, Copy)]
pub struct CartesianState {
    /// Position vector [x, y, z] in meters
    pub position: na::Vector3<f64>,
    /// Velocity vector [vx, vy, vz] in meters per second
    pub velocity: na::Vector3<f64>,
}

impl CartesianState {
    /// Create a new Cartesian state
    pub fn new(position: na::Vector3<f64>, velocity: na::Vector3<f64>) -> Self {
        Self { position, velocity }
    }

    /// Get the position vector
    pub fn position(&self) -> &na::Vector3<f64> {
        &self.position
    }

    /// Get the velocity vector
    pub fn velocity(&self) -> &na::Vector3<f64> {
        &self.velocity
    }

    /// Calculate orbital energy per unit mass (m^2/s^2)
    pub fn specific_energy(&self, mu: f64) -> f64 {
        let r = self.position.norm();
        let v = self.velocity.norm();
        0.5 * v * v - mu / r
    }

    /// Calculate specific angular momentum vector (m^2/s)
    pub fn specific_angular_momentum(&self) -> na::Vector3<f64> {
        self.position.cross(&self.velocity)
    }
}
