/*
* Copyright 2025 Nicolas Spijkerman
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http:
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

use super::SchoolingConfig;
use crate::{Scalar, Vec2};
use rand::Rng;
use shipyard::EntityId;
use std::{
    collections::HashMap,
    f32::consts::{PI, TAU},
};

// See ./docs/schooling_mechanism_in_fish.pdf
#[derive(Debug)]
pub struct SchoolingMechanism {
    position: Vec2,
    velocity: Vec2,
    speed: Scalar,
    others_positions: HashMap<EntityId, Vec2>,
    others_velocities: HashMap<EntityId, Vec2>,
    others_speeds: HashMap<EntityId, Scalar>,
    cfg: SchoolingConfig,
}

impl SchoolingMechanism {
    pub fn setup(
        position: Vec2,
        velocity: Vec2,
        speed: Scalar,
        others_positions: HashMap<EntityId, Vec2>,
        others_velocities: HashMap<EntityId, Vec2>,
        others_speeds: HashMap<EntityId, Scalar>,
        cfg: SchoolingConfig,
    ) -> Self {
        Self {
            position,
            velocity,
            speed,
            others_positions,
            others_velocities,
            others_speeds,
            cfg,
        }
    }

    pub fn set_behavior(&self, velocity: &mut Vec2, speed: &mut Scalar) {
        *velocity = self.velocity;
        *speed = self.speed * 100.0;
    }

    pub fn update(&mut self, rng: &mut impl Rng) {
        // 1. Update Speed (Stochastic Gamma/Erlang Distribution)
        self.speed = self.generate_random_speed(rng);

        // 2. Identify Neighbors in Visual Field
        struct Candidate {
            id: EntityId,
            dist: f32,
            vec_to: Vec2,
            angle_diff: f32, // Relative to current heading
        }

        let current_heading: f32 = self.velocity.y.atan2(self.velocity.x);
        let visual_limit: f32 = self.cfg.visual_field / 2.0;

        let mut candidates: Vec<Candidate> = Vec::with_capacity(self.others_positions.len());

        for (&id, &pos) in &self.others_positions {
            let vec_to: Vec2 = pos - self.position;
            let dist: f32 = vec_to.length();

            if dist <= 0.001 {
                continue; // Skip extremely close overlaps
            }

            let angle_to: f32 = vec_to.y.atan2(vec_to.x);
            let angle_diff: f32 = Self::wrap_angle(angle_to - current_heading);

            // Check if within Visual Field (Angle AR)
            if angle_diff.abs() <= visual_limit {
                candidates.push(Candidate {
                    id,
                    dist,
                    vec_to,
                    angle_diff,
                });
            }
        }

        // 3. Determine Reference Individual and Interaction Mode
        let target_angle: f32;
        let angle_std_dev: f32;

        // Filter for "Sector" (within interaction radius RC)
        let mut sector_neighbors: Vec<&Candidate> = candidates
            .iter()
            .filter(|c| c.dist <= self.cfg.attraction_radius)
            .collect();

        if !sector_neighbors.is_empty() {
            // --- CASE A: Neighbors in Sector ---

            // Sort by absolute angle difference (closest to current heading first)
            sector_neighbors
                .sort_by(|a, b| a.angle_diff.abs().partial_cmp(&b.angle_diff.abs()).unwrap());

            // Keep only up to 4 nearest-in-angle neighbors [schooling_mechanism_in_fish.pdf][web:1]
            let count: usize = sector_neighbors.len().min(4);
            let top_neighbors: &[&Candidate] = &sector_neighbors[0..count];

            // Calculate Weights: W_{j+1} = RF * W_j
            let rf: f32 = self.cfg.reference_factor;
            let mut weights: Vec<f32> = Vec::with_capacity(count);
            let mut weight_sum: f32 = 0.0;
            let mut current_w: f32 = 1.0;

            for _ in 0..count {
                weights.push(current_w);
                weight_sum += current_w;
                current_w *= rf;
            }

            // Select Reference Neighbor (Roulette Wheel Selection)
            let r: f32 = rng.random_range(0.0..weight_sum);
            let mut acc: f32 = 0.0;
            let mut selected_idx: usize = 0;
            for (i, &w) in weights.iter().enumerate() {
                acc += w;
                if r <= acc {
                    selected_idx = i;
                    break;
                }
            }
            let reference: &Candidate = top_neighbors[selected_idx];

            // Determine Interaction based on Distance
            if reference.dist < self.cfg.avoidance_radius {
                // -- Avoidance --
                // Turn 90 degrees away from neighbor
                // Paper: "depending upon which heading forms the smaller angle"
                // We effectively want to steer normal to the neighbor's direction relative to us.
                let neighbor_angle: f32 = reference.vec_to.y.atan2(reference.vec_to.x);

                let turn_left: f32 = Self::wrap_angle(neighbor_angle + PI / 2.0);
                let turn_right: f32 = Self::wrap_angle(neighbor_angle - PI / 2.0);

                // Choose the turn direction closer to current heading to maintain flow
                if Self::wrap_angle(turn_left - current_heading).abs()
                    < Self::wrap_angle(turn_right - current_heading).abs()
                {
                    target_angle = turn_left;
                } else {
                    target_angle = turn_right;
                }
                angle_std_dev = self.cfg.avoidance_attraction_standard_deviation;
            } else if reference.dist < self.cfg.alignment_radius {
                // -- Parallel Orientation --
                // Match neighbor's velocity heading
                if let Some(other_vel) = self.others_velocities.get(&reference.id) {
                    target_angle = other_vel.y.atan2(other_vel.x);
                } else {
                    // Fallback if velocity missing (shouldn't happen)
                    target_angle = current_heading;
                }
                angle_std_dev = self.cfg.alignment_standard_deviation;
            } else {
                // -- Approach -- (Between Alignment and Attraction Radius)
                // Head towards neighbor
                target_angle = reference.vec_to.y.atan2(reference.vec_to.x);
                angle_std_dev = self.cfg.avoidance_attraction_standard_deviation;
            }
        } else if !candidates.is_empty() {
            // --- CASE B: No Neighbors in Sector, but some in Visual Cone (Far away) ---
            // Paper: "approach motion toward a nearer neighbor is set up"

            // Find nearest by distance
            let nearest: &Candidate = candidates
                .iter()
                .min_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap())
                .unwrap();

            target_angle = nearest.vec_to.y.atan2(nearest.vec_to.x);
            angle_std_dev = self.cfg.avoidance_attraction_standard_deviation;
        } else {
            // --- CASE C: No Neighbors Visible ---
            // Random movement (Independent)
            target_angle = rng.random_range(0.0..TAU);
            angle_std_dev = 0.0; // No deviation on random walk, just pick one
        }

        // 4. Apply Gaussian Noise to Direction
        let final_angle: f32 = if angle_std_dev > 0.0 {
            // Box-Muller or similar for Normal Distribution
            // Standard Rust `rand` doesn't have `gen_normal` easily without `rand_distr`.
            // We approximate or use Box-Muller manually.
            let u1: f32 = rng.random_range(0.0..1.0);
            let u2: f32 = rng.random_range(0.0..1.0);

            // Standard Normal Z
            let z: f32 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();

            target_angle + z * angle_std_dev
        } else {
            target_angle
        };

        // 5. Update Velocity Vector (Normalized)
        let (sin, cos): (f32, f32) = final_angle.sin_cos();
        self.velocity = Vec2::new(cos, sin);
        // Ensure it is normalized (though sin/cos is unit length)
        self.velocity.normalize();
    }

    fn generate_random_speed(&self, rng: &mut impl Rng) -> Scalar {
        let k: i32 = self.cfg.gamma_dist_k as i32;
        let a: f32 = self.cfg.gamma_dist_a;

        if k <= 0 {
            return Scalar::ZERO;
        }

        // Erlang distribution generation (Sum of K exponential)
        // X = -1/A * ln( product(U_i) )
        let mut log_prod: f32 = 0.0;
        for _ in 0..k {
            let u: f32 = rng.random_range(1e-6..1.0); // Avoid log(0)
            log_prod += u.ln();
        }

        let speed_val: f32 = -log_prod / a;
        Scalar::new(speed_val)
    }

    #[inline]
    fn wrap_angle(angle: f32) -> f32 {
        let mut a: f32 = angle;
        while a > PI {
            a -= TAU;
        }
        while a < -PI {
            a += TAU;
        }
        a
    }
}
