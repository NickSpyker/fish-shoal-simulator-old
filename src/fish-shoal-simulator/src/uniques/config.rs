/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use shipyard::Unique;

#[derive(Unique, Debug, Copy, Clone, PartialEq)]
pub struct Config {
    pub mouse_pos: Option<[f32; 2]>,
    pub mouse_vel: Option<[f32; 2]>,
    pub mouse_speed: Option<f32>,

    pub running: bool,
    pub paused: bool,

    pub width: usize,
    pub height: usize,

    pub entity_count: usize,

    pub direction_change_prob: f64,
    pub speed_change_prob: f64,
    pub stress_change_prob: f64,

    pub attraction_fov: f32,
    pub alignment_fov: f32,

    pub attraction_radius: f32,
    pub alignment_radius: f32,
    pub avoidance_radius: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mouse_pos: None,
            mouse_vel: None,
            mouse_speed: None,

            running: true,
            paused: false,

            width: 1_920,
            height: 1_080,

            entity_count: 500,

            direction_change_prob: 0.1,
            speed_change_prob: 0.05,
            stress_change_prob: 0.001,

            attraction_fov: 100.0,
            alignment_fov: 250.0,

            attraction_radius: 50.0,
            alignment_radius: 30.0,
            avoidance_radius: 10.0,
        }
    }
}
