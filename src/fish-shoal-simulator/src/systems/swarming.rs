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

use crate::{
    algo::SchoolingMechanism, Chunks, Config, Density, /* Scalar, */ Position, Social, Stress,
    TargetSpeed, TargetVelocity, Vec2,
};
use shipyard::{EntityId, IntoIter, UniqueView, View, ViewMut};
use std::collections::{HashMap, HashSet};

macro_rules! collect_components {
    ($c:ident) => {
        $c.iter().with_id().map(|(i, v)| (i, v.0)).collect()
    };
}

macro_rules! neighbors {
    ($o:ident, $c:ident) => {
        $o.iter().filter_map(|&i| Some((i, *$c.get(&i)?))).collect()
    };
}

#[derive(Debug)]
pub struct Swarming;

impl Swarming {
    pub fn system(
        positions: View<Position>,
        mut velocities: ViewMut<TargetVelocity>,
        mut speeds: ViewMut<TargetSpeed>,
        mut stress: ViewMut<Stress>,
        mut densities: ViewMut<Density>,
        mut socials: ViewMut<Social>,
        chunks: UniqueView<Chunks>,
        cfg: UniqueView<Config>,
    ) {
        let others_positions: HashMap<EntityId, Vec2> = collect_components!(positions);
        let others_velocities: HashMap<EntityId, Vec2> = collect_components!(velocities);
        // let others_speeds: HashMap<EntityId, Scalar> = collect_components!(speeds);

        (
            &positions,
            &mut velocities,
            &mut speeds,
            &mut stress,
            &mut densities,
            &mut socials,
        )
            .iter()
            .with_id()
            .for_each(|(id, (pos, vel, speed, stress, density, social))| {
                let mut neighbors: HashSet<EntityId> = chunks.load_chunk(&pos.0);
                neighbors.remove(&id);

                density.set(neighbors.len());
                if density.value < SchoolingMechanism::MAX_NEIGHBORS {
                    neighbors.extend(chunks.load_neighbors(&pos.0));
                }
                neighbors.remove(&id);

                density.set(neighbors.len());
                if density.is_zero() {
                    social.set_alone();
                    speed.0.value = 50.0;
                    stress.0.value = 0.1;
                    return;
                }
                social.set_grouped();

                let mut algo: SchoolingMechanism = SchoolingMechanism::setup(
                    pos.0,
                    vel.0,
                    speed.0,
                    stress.0,
                    neighbors!(neighbors, others_positions),
                    neighbors!(neighbors, others_velocities),
                    // neighbors!(neighbors, others_speeds),
                    cfg.avoidance_radius,
                    cfg.alignment_radius,
                    cfg.attraction_radius,
                );

                if !algo.avoidance() {
                    if !algo.alignment() {
                        algo.attraction();
                    }
                }

                algo.set_behavior(&mut vel.0, &mut speed.0, &mut stress.0);
            });
    }
}
