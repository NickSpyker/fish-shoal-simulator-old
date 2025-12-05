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
use shipyard::EntityId;
use std::collections::HashMap;

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

    pub fn avoidance(&mut self) -> bool {
        false
    }

    pub fn alignment(&mut self) -> bool {
        false
    }

    pub fn attraction(&mut self) -> bool {
        false
    }
}
