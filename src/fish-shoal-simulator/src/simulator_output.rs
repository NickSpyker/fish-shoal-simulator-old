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

#[derive(Debug, Default)]
pub struct SimulatorOutput {
    pub ids: Vec<usize>,
    pub positions: Vec<[f32; 2]>,
    pub velocities: Vec<[f32; 2]>,
    pub speeds: Vec<f32>,
    pub densities: Vec<usize>,
}

impl SimulatorOutput {
    pub(crate) fn build(
        position_view: View<Position>,
        velocity_view: View<Velocity>,
        speed_view: View<Speed>,
        density_view: View<Density>,
    ) -> Self {
        let mut ids: Vec<usize> = Vec::new();
        let mut positions: Vec<[f32; 2]> = Vec::new();
        let mut velocities: Vec<[f32; 2]> = Vec::new();
        let mut speeds: Vec<f32> = Vec::new();
        let mut densities: Vec<usize> = Vec::new();

        (&position_view, &velocity_view, &speed_view, &density_view)
            .iter()
            .with_id()
            .for_each(|(id, (pos, vel, speed, density))| {
                ids.push(id.uindex());
                positions.push(pos.0.into());
                velocities.push(vel.0.into());
                speeds.push(speed.0.into());
                densities.push(density.value);
            });

        Self {
            ids,
            positions,
            velocities,
            speeds,
            densities,
        }
    }
}
