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

use crate::{Scalar, Vec2};
use shipyard::EntityId;
use std::collections::HashMap;

// See ./docs/schooling_mechanism_in_fish.pdf
#[derive(Debug)]
pub struct SchoolingMechanism {
    position: Vec2,
    velocity: Vec2,
    speed: Scalar,
    stress: Scalar,
    others_positions: HashMap<EntityId, Vec2>,
    others_velocities: HashMap<EntityId, Vec2>,
    // others_speeds: HashMap<EntityId, Scalar>,
    avoidance_radius: f32,
    alignment_radius: f32,
    attraction_radius: f32,
}

impl SchoolingMechanism {
    pub const MAX_NEIGHBORS: usize = 6;

    pub fn setup(
        position: Vec2,
        velocity: Vec2,
        speed: Scalar,
        stress: Scalar,
        others_positions: HashMap<EntityId, Vec2>,
        others_velocities: HashMap<EntityId, Vec2>,
        // others_speeds: HashMap<EntityId, Scalar>,
        avoidance_radius: f32,
        alignment_radius: f32,
        attraction_radius: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            speed,
            stress,
            others_positions,
            others_velocities,
            // others_speeds,
            avoidance_radius,
            alignment_radius,
            attraction_radius,
        }
    }

    pub fn set_behavior(&self, velocity: &mut Vec2, speed: &mut Scalar, stress: &mut Scalar) {
        *velocity = self.velocity;
        *speed = self.speed;
        *stress = self.stress;
    }

    pub fn avoidance(&mut self) -> bool {
        let mut position_to_avoid: Vec2 = Vec2::ZERO;

        let mut count: f32 = 0.0;
        for (_, &other_position) in &self.others_positions {
            if self.position.distance(other_position) <= self.avoidance_radius {
                position_to_avoid += other_position;
                count += 1.0;
                position_to_avoid /= count;
                if count as usize >= Self::MAX_NEIGHBORS {
                    break;
                }
            }
        }

        if position_to_avoid != Vec2::ZERO {
            self.velocity = (self.position - position_to_avoid).normalized();
            self.stress.value = 0.95;
            self.speed.value = 100.0;
            true
        } else {
            false
        }
    }

    pub fn alignment(&mut self) -> bool {
        let mut velocity_to_align: Vec2 = Vec2::ZERO;

        let mut count: f32 = 0.0;
        for (other_id, &other_position) in &self.others_positions {
            if self.position.distance(other_position) <= self.avoidance_radius {
                continue;
            }
            if self.position.distance(other_position) <= self.alignment_radius {
                let other_velocity: Vec2 = self.others_velocities[other_id];
                velocity_to_align += other_velocity;
                count += 1.0;
                velocity_to_align /= count;
                if count as usize >= Self::MAX_NEIGHBORS {
                    break;
                }
            }
        }

        if velocity_to_align != Vec2::ZERO {
            self.velocity = velocity_to_align.normalized();
            self.stress.value = 0.33;
            self.speed.value = 75.0;
            true
        } else {
            false
        }
    }

    pub fn attraction(&mut self) -> bool {
        let mut position_to_join: Vec2 = Vec2::ZERO;

        let mut count: f32 = 0.0;
        for (_, &other_position) in &self.others_positions {
            let avoid: bool = self.position.distance(other_position) <= self.avoidance_radius;
            let align: bool = self.position.distance(other_position) <= self.alignment_radius;
            if avoid || align {
                continue;
            }
            if self.position.distance(other_position) <= self.attraction_radius {
                position_to_join += other_position;
                count += 1.0;
                position_to_join /= count;
                if count as usize >= Self::MAX_NEIGHBORS {
                    break;
                }
            }
        }

        if position_to_join != Vec2::ZERO {
            self.velocity = (position_to_join - self.position).normalized();
            self.stress.value = 0.5;
            self.speed.value = 100.0;
            true
        } else {
            false
        }
    }
}
