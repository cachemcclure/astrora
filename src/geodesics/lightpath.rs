//! Composite light path builder
//!
//! Computes the full path of a photon from a source to a target, accounting for
//! gravitational lensing by intermediate massive bodies. Uses analytical weak-field
//! deflection for distant passes and numerical geodesic integration for close encounters.
//!
//! # Algorithm
//!
//! 1. Compute the straight-line path from source to target.
//! 2. For each lensing body, compute the impact parameter (perpendicular distance
//!    from the body to the straight-line path).
//! 3. Filter to bodies within an influence threshold.
//! 4. For each relevant body, compute the deflection (analytical or numerical).
//! 5. Apply cumulative deflections to build the curved path.
//! 6. Compute total path length and travel time.

use super::deflection::Deflection;
use crate::core::constants::{schwarzschild_radius, C};
use crate::core::error::{PoliastroError, PoliastroResult};

/// A massive body that can gravitationally deflect light.
#[derive(Debug, Clone, Copy)]
pub struct LensingBody {
    /// Position [x, y, z] in the working coordinate frame (meters).
    pub position: [f64; 3],
    /// Gravitational parameter GM (m³/s²).
    pub gm: f64,
}

/// Record of a deflection event along a light path.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeflectionEvent {
    /// Index of the body in the input slice.
    pub body_index: usize,
    /// Deflection angle (radians).
    pub deflection_angle: f64,
    /// Closest approach distance from the light path to the body center (meters).
    pub closest_approach: f64,
    /// Whether full numerical integration was used (vs. analytical).
    pub numerical: bool,
}

/// Complete result of a light path computation.
#[derive(Debug, Clone, PartialEq)]
pub struct LightPathResult {
    /// 3D points along the light path [x, y, z] in meters.
    pub points: Vec<[f64; 3]>,
    /// Total path length along the curved trajectory (meters).
    pub total_distance: f64,
    /// Light travel time along the path (seconds).
    pub travel_time: f64,
    /// Straight-line distance from source to target (meters).
    pub straight_line_distance: f64,
    /// All deflection events along the path, ordered by occurrence.
    pub deflections: Vec<DeflectionEvent>,
    /// Total accumulated deflection angle (radians).
    pub total_deflection: f64,
}

/// Composite light path calculator.
pub struct LightPath;

impl LightPath {
    /// Compute the light path from source to target with gravitational lensing.
    ///
    /// # Arguments
    /// * `source` - Source position [x, y, z] in meters
    /// * `target` - Target position [x, y, z] in meters
    /// * `bodies` - Massive bodies that may deflect the light
    /// * `points_per_segment` - Number of interpolation points per path segment
    ///
    /// # Returns
    /// `LightPathResult` with the path geometry and deflection details.
    ///
    /// # Errors
    /// - `InvalidParameter` if source and target are identical
    /// - Propagates errors from deflection calculations
    pub fn compute(
        source: [f64; 3],
        target: [f64; 3],
        bodies: &[LensingBody],
        points_per_segment: usize,
    ) -> PoliastroResult<LightPathResult> {
        let straight_line_distance = Self::distance(&source, &target);

        if straight_line_distance < 1.0 {
            return Err(PoliastroError::invalid_parameter(
                "source-target distance",
                straight_line_distance,
                "source and target must be at least 1 meter apart",
            ));
        }

        let points_per_segment = points_per_segment.max(2);

        // Find bodies that are close enough to the straight-line path to matter.
        // Threshold: impact parameter < 1000 × Schwarzschild radius (or 1 AU for safety).
        let mut relevant_bodies: Vec<(usize, f64, [f64; 3])> = Vec::new();

        for (i, body) in bodies.iter().enumerate() {
            let (impact_param, closest_point) =
                Self::closest_approach_to_line(&body.position, &source, &target);

            let r_s = schwarzschild_radius(body.gm);
            let threshold = if r_s > 0.0 {
                (1000.0 * r_s).max(1e12) // at least 1 million km
            } else {
                continue; // zero mass, skip
            };

            if impact_param < threshold && impact_param > 0.0 {
                relevant_bodies.push((i, impact_param, closest_point));
            }
        }

        // Sort by closest approach (nearest first)
        relevant_bodies.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // If no relevant bodies, return a straight line
        if relevant_bodies.is_empty() {
            let points = Self::interpolate_line(&source, &target, points_per_segment);
            return Ok(LightPathResult {
                points,
                total_distance: straight_line_distance,
                travel_time: straight_line_distance / C,
                straight_line_distance,
                deflections: Vec::new(),
                total_deflection: 0.0,
            });
        }

        // Compute deflections for each relevant body
        let mut deflections = Vec::new();
        let mut total_deflection = 0.0;

        for &(body_index, impact_param, _closest_point) in &relevant_bodies {
            let body = &bodies[body_index];

            let result = Deflection::deflection_adaptive(body.gm, impact_param, 50_000)?;

            if result.deflection_angle.is_finite() && result.deflection_angle > 0.0 {
                deflections.push(DeflectionEvent {
                    body_index,
                    deflection_angle: result.deflection_angle,
                    closest_approach: impact_param,
                    numerical: !result.is_weak_field,
                });
                total_deflection += result.deflection_angle;
            }
        }

        // Build the path. For small deflections (which is most cases), we construct
        // a path that bends at each deflection point. The bend direction is toward
        // the lensing body.
        let points = if deflections.is_empty() || total_deflection < 1e-15 {
            Self::interpolate_line(&source, &target, points_per_segment)
        } else {
            Self::build_deflected_path(
                &source,
                &target,
                bodies,
                &relevant_bodies,
                &deflections,
                points_per_segment,
            )
        };

        // Compute total path length from the points
        let total_distance = Self::path_length(&points);
        let travel_time = total_distance / C;

        Ok(LightPathResult {
            points,
            total_distance,
            travel_time,
            straight_line_distance,
            deflections,
            total_deflection,
        })
    }

    /// Estimate the maximum deflection without computing the full path.
    ///
    /// Useful for filtering: skip the full computation if max deflection is negligible.
    pub fn estimate_max_deflection(
        source: &[f64; 3],
        target: &[f64; 3],
        bodies: &[LensingBody],
    ) -> PoliastroResult<f64> {
        let mut max_deflection = 0.0;

        for body in bodies {
            let (impact_param, _) = Self::closest_approach_to_line(&body.position, source, target);
            if impact_param > 0.0 {
                let result = Deflection::einstein_deflection(body.gm, impact_param)?;
                if result.deflection_angle > max_deflection {
                    max_deflection = result.deflection_angle;
                }
            }
        }

        Ok(max_deflection)
    }

    // =========================================================================
    // Private helpers
    // =========================================================================

    /// Compute the closest approach of a point to a line segment.
    ///
    /// Returns (distance, closest_point_on_line).
    fn closest_approach_to_line(
        point: &[f64; 3],
        line_start: &[f64; 3],
        line_end: &[f64; 3],
    ) -> (f64, [f64; 3]) {
        let dx = [
            line_end[0] - line_start[0],
            line_end[1] - line_start[1],
            line_end[2] - line_start[2],
        ];
        let dp = [
            point[0] - line_start[0],
            point[1] - line_start[1],
            point[2] - line_start[2],
        ];

        let line_len_sq = dx[0] * dx[0] + dx[1] * dx[1] + dx[2] * dx[2];

        if line_len_sq < 1e-30 {
            // Degenerate line (start == end)
            let dist = Self::distance(point, line_start);
            return (dist, *line_start);
        }

        // Project point onto the infinite line: t = dot(dp, dx) / dot(dx, dx)
        let t = (dp[0] * dx[0] + dp[1] * dx[1] + dp[2] * dx[2]) / line_len_sq;
        // Clamp to [0, 1] for line segment
        let t_clamped = t.clamp(0.0, 1.0);

        let closest = [
            line_start[0] + t_clamped * dx[0],
            line_start[1] + t_clamped * dx[1],
            line_start[2] + t_clamped * dx[2],
        ];

        let dist = Self::distance(point, &closest);
        (dist, closest)
    }

    /// Euclidean distance between two 3D points.
    fn distance(a: &[f64; 3], b: &[f64; 3]) -> f64 {
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        let dz = a[2] - b[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Linearly interpolate between two points.
    fn interpolate_line(start: &[f64; 3], end: &[f64; 3], num_points: usize) -> Vec<[f64; 3]> {
        let n = num_points.max(2);
        (0..n)
            .map(|i| {
                let t = i as f64 / (n - 1) as f64;
                [
                    start[0] + t * (end[0] - start[0]),
                    start[1] + t * (end[1] - start[1]),
                    start[2] + t * (end[2] - start[2]),
                ]
            })
            .collect()
    }

    /// Build a deflected path that bends toward each lensing body at the
    /// closest approach point.
    fn build_deflected_path(
        source: &[f64; 3],
        target: &[f64; 3],
        bodies: &[LensingBody],
        relevant_bodies: &[(usize, f64, [f64; 3])],
        deflections: &[DeflectionEvent],
        points_per_segment: usize,
    ) -> Vec<[f64; 3]> {
        // Build waypoints: source → (bend points at each lensing body) → target
        // At each closest-approach point, offset the path toward the lensing body
        // by an amount proportional to the deflection angle.
        let mut waypoints: Vec<[f64; 3]> = Vec::new();
        waypoints.push(*source);

        for deflection in deflections {
            // Find the corresponding relevant body info
            let body_info = relevant_bodies
                .iter()
                .find(|(idx, _, _)| *idx == deflection.body_index);

            if let Some(&(body_idx, impact_param, closest_point)) = body_info {
                let body_pos = &bodies[body_idx].position;

                // Direction from closest point on path to the lensing body
                let to_body = [
                    body_pos[0] - closest_point[0],
                    body_pos[1] - closest_point[1],
                    body_pos[2] - closest_point[2],
                ];
                let to_body_len =
                    (to_body[0] * to_body[0] + to_body[1] * to_body[1] + to_body[2] * to_body[2])
                        .sqrt();

                if to_body_len > 0.0 {
                    // Offset the waypoint toward the body.
                    // The offset distance scales with impact_parameter × deflection_angle
                    // (this is the actual lateral displacement at the closest approach point).
                    let offset = impact_param * deflection.deflection_angle;
                    let unit = [
                        to_body[0] / to_body_len,
                        to_body[1] / to_body_len,
                        to_body[2] / to_body_len,
                    ];

                    waypoints.push([
                        closest_point[0] + offset * unit[0],
                        closest_point[1] + offset * unit[1],
                        closest_point[2] + offset * unit[2],
                    ]);
                }
            }
        }

        waypoints.push(*target);

        // Interpolate between consecutive waypoints
        let mut points = Vec::new();
        let pts_per = (points_per_segment / waypoints.len().saturating_sub(1)).max(2);

        for i in 0..waypoints.len() - 1 {
            let segment = Self::interpolate_line(&waypoints[i], &waypoints[i + 1], pts_per);
            if i == 0 {
                points.extend_from_slice(&segment);
            } else {
                // Skip the first point to avoid duplicates at waypoint junctions
                points.extend_from_slice(&segment[1..]);
            }
        }

        points
    }

    /// Compute total path length from a series of points.
    fn path_length(points: &[[f64; 3]]) -> f64 {
        points
            .windows(2)
            .map(|w| Self::distance(&w[0], &w[1]))
            .sum()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::constants::{AU, GM_SUN, R_SUN};
    use approx::assert_relative_eq;

    #[test]
    fn test_no_bodies_straight_line() {
        let source = [0.0, 0.0, 0.0];
        let target = [1e12, 0.0, 0.0];

        let result = LightPath::compute(source, target, &[], 100).unwrap();

        assert_relative_eq!(
            result.total_distance,
            result.straight_line_distance,
            epsilon = 1.0
        );
        assert!(result.deflections.is_empty());
        assert_relative_eq!(result.total_deflection, 0.0, epsilon = 1e-20);
        assert_relative_eq!(
            result.travel_time,
            result.total_distance / C,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_travel_time_equals_distance_over_c() {
        let source = [0.0, 0.0, 0.0];
        let target = [AU, 0.0, 0.0]; // 1 AU away

        let result = LightPath::compute(source, target, &[], 100).unwrap();

        // 1 AU / c ≈ 499 seconds
        assert_relative_eq!(result.travel_time, AU / C, epsilon = 0.1);
        assert!(result.travel_time > 498.0 && result.travel_time < 500.0);
    }

    #[test]
    fn test_sun_deflects_light() {
        // Light from a distant star passing near the Sun to reach Earth.
        // Path goes along y-axis; Sun is offset in x by 10 solar radii.
        let source = [0.0, 10.0 * AU, 0.0]; // far away in +y
        let target = [0.0, -AU, 0.0]; // Earth at -1 AU in y
        let sun = LensingBody {
            position: [R_SUN * 10.0, 0.0, 0.0], // Sun offset from the path
            gm: GM_SUN,
        };

        let result = LightPath::compute(source, target, &[sun], 200).unwrap();

        // Should have exactly one deflection event
        assert_eq!(result.deflections.len(), 1);
        assert_eq!(result.deflections[0].body_index, 0);
        assert!(result.deflections[0].deflection_angle > 0.0);
        // Deflection should be tiny (arcsecond scale when close to Sun)
        assert!(result.total_deflection < 0.01); // less than ~0.5 degrees
    }

    #[test]
    fn test_multiple_bodies() {
        let source = [0.0, 0.0, 0.0];
        let target = [3.0 * AU, 0.0, 0.0];

        let body1 = LensingBody {
            position: [AU, R_SUN * 10.0, 0.0],
            gm: GM_SUN,
        };
        let body2 = LensingBody {
            position: [2.0 * AU, -R_SUN * 10.0, 0.0],
            gm: GM_SUN,
        };

        let result = LightPath::compute(source, target, &[body1, body2], 200).unwrap();

        // Both bodies should contribute deflections
        assert_eq!(result.deflections.len(), 2);
        // Total deflection should be sum of individual deflections
        let sum: f64 = result.deflections.iter().map(|d| d.deflection_angle).sum();
        assert_relative_eq!(result.total_deflection, sum, epsilon = 1e-15);
    }

    #[test]
    fn test_distant_body_no_effect() {
        let source = [0.0, 0.0, 0.0];
        let target = [1e12, 0.0, 0.0];

        // Body very far from the path
        let body = LensingBody {
            position: [5e11, 1e18, 0.0], // 1e18 m away in y
            gm: GM_SUN,
        };

        let result = LightPath::compute(source, target, &[body], 100).unwrap();

        // Body too far away to affect the path
        assert!(result.deflections.is_empty() || result.total_deflection < 1e-20);
    }

    #[test]
    fn test_degenerate_source_target() {
        let source = [1e6, 0.0, 0.0];
        let target = [1e6, 0.0, 0.0]; // same as source

        assert!(LightPath::compute(source, target, &[], 100).is_err());
    }

    #[test]
    fn test_estimate_max_deflection() {
        let source = [0.0, 0.0, 0.0];
        let target = [0.0, 10.0 * AU, 0.0];

        let sun = LensingBody {
            position: [R_SUN * 2.0, 5.0 * AU, 0.0], // near the path midpoint
            gm: GM_SUN,
        };

        let max_defl = LightPath::estimate_max_deflection(&source, &target, &[sun]).unwrap();
        assert!(max_defl > 0.0);
    }

    #[test]
    fn test_closest_approach_to_line() {
        // Point directly above the midpoint of a line on the x-axis
        let point = [5.0, 3.0, 0.0];
        let start = [0.0, 0.0, 0.0];
        let end = [10.0, 0.0, 0.0];

        let (dist, closest) = LightPath::closest_approach_to_line(&point, &start, &end);
        assert_relative_eq!(dist, 3.0, epsilon = 1e-10);
        assert_relative_eq!(closest[0], 5.0, epsilon = 1e-10);
        assert_relative_eq!(closest[1], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_path_has_source_and_target() {
        let source = [0.0, 0.0, 0.0];
        let target = [1e12, 0.0, 0.0];

        let result = LightPath::compute(source, target, &[], 50).unwrap();

        // First point should be near source
        assert_relative_eq!(result.points[0][0], source[0], epsilon = 1.0);
        // Last point should be near target
        let last = result.points.last().unwrap();
        assert_relative_eq!(last[0], target[0], epsilon = 1.0);
    }
}
