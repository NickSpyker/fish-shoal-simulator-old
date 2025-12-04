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

use crate::{Density, Position, Speed, Velocity};
use shipyard::{IntoIter, View};

macro_rules! collect_components {
    ($view:expr, $map:expr) => {
        $view.iter().map($map).collect()
    };
}

#[derive(Debug, Default)]
pub struct SimulatorOutput {
    pub positions: Vec<[f32; 2]>,
    pub velocities: Vec<[f32; 2]>,
    pub speeds: Vec<f32>,
    pub densities: Vec<usize>,
}

impl SimulatorOutput {
    pub(crate) fn build(
        positions: View<Position>,
        velocities: View<Velocity>,
        speeds: View<Speed>,
        densities: View<Density>,
    ) -> Self {
        Self {
            positions: collect_components!(positions, |pos| pos.0.into()),
            velocities: collect_components!(velocities, |vel| vel.0.into()),
            speeds: collect_components!(speeds, |speed| speed.0.value),
            densities: collect_components!(densities, |density| density.value),
        }
    }
}
