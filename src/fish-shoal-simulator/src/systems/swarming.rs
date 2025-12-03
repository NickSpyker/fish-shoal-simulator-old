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

use crate::{Chunks, Config, Idle, NeighborCount, Position, TargetSpeed, TargetVelocity, Vec2};
use rayon::prelude::*;
use shipyard::{EntityId, Get, IntoIter, UniqueView, View, ViewMut};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Swarming;

impl Swarming {
    pub fn system(
        positions: View<Position>,
        mut velocities: ViewMut<TargetVelocity>,
        mut speeds: ViewMut<TargetSpeed>,
        mut neighbor_counts: ViewMut<NeighborCount>,
        mut idles: ViewMut<Idle>,
        cfg: UniqueView<Config>,
        chunks: UniqueView<Chunks>,
    ) {
        let all_velocities: HashMap<EntityId, TargetVelocity> = (&velocities)
            .iter()
            .with_id()
            .map(|(id, vel)| (id, *vel))
            .collect();

        let all_speeds: HashMap<EntityId, TargetSpeed> = (&speeds)
            .iter()
            .with_id()
            .map(|(id, speed)| (id, *speed))
            .collect();

        (
            &positions,
            &mut velocities,
            &mut speeds,
            &mut neighbor_counts,
            &mut idles,
        )
            .par_iter()
            .for_each(|(pos, vel, speed, neighbor_count, idle)| {
                let neighbors: HashSet<EntityId> = chunks.load(&pos.0);
                neighbor_count.0 = neighbors.len();
                if neighbor_count.0 == 0 {
                    idle.0 = true;
                    return;
                }
                idle.0 = false;

                let neighbor_positions: HashMap<EntityId, &Position> = neighbors
                    .par_iter()
                    .filter_map(|&neighbor_id| {
                        Some((neighbor_id, positions.get(neighbor_id).ok()?))
                    })
                    .collect();

                let mut avoidance_neighbors: Vec<(EntityId, &Position)> = neighbor_positions
                    .iter()
                    .filter_map(|(&neighbor_id, &neighbor_pos)| {
                        if pos.0.distance(neighbor_pos.0) <= cfg.avoidance_radius {
                            Some((neighbor_id, neighbor_pos))
                        } else {
                            None
                        }
                    })
                    .collect();

                avoidance_neighbors.sort_unstable_by_key(|(_, neighbor_pos)| {
                    (pos.0.distance(neighbor_pos.0) * 1000.0) as u32
                });
                avoidance_neighbors.truncate(6);

                if !avoidance_neighbors.is_empty() {
                    let mut avoidance_positions: Vec2 = Vec2::ZERO;
                    for (_, neighbor_pos) in &avoidance_neighbors {
                        avoidance_positions += neighbor_pos.0;
                    }
                    avoidance_positions /= avoidance_neighbors.len() as f32;
                    let avoidance_direction: Vec2 = (pos.0 - avoidance_positions).normalized();
                    if avoidance_direction != Vec2::ZERO {
                        vel.0 = avoidance_direction
                    }
                    return;
                }

                // TODO: add alignment and attraction behavior
            });
    }
}
