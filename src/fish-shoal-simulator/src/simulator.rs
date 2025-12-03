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
    entities::Fish, systems::*, Chunks, Config, DeltaTime, Error, Density, Position, SimulatorOutput,
    Speed, Velocity,
};
use shipyard::{
    error::{AddWorkload, RunWorkload},
    Workload,
    {IntoIter, UniqueView, UniqueViewMut, View, World},
};
use std::{cmp::Ordering, mem};

#[derive(Debug)]
pub struct FishShoalSimulator {
    world: World,
    paused: bool,
}

impl FishShoalSimulator {
    pub fn new() -> Result<Self, Error> {
        let mut world: World = World::default();
        let cfg: Config = Config::default();

        world.add_unique(Config::default());
        world.add_unique(DeltaTime::default());
        world.add_unique(Chunks::new(cfg.attraction_radius));

        Fish::add(&mut world, cfg.entity_count, cfg);

        Workload::new("sim")
            .with_system(CalculateDeltaTime::system)
            .with_system(LoadChunks::system)
            .with_barrier()
            .with_system(Motion::system)
            .with_system(OutOfBound::system)
            .with_system(LerpToTarget::system)
            .with_system(RandomBehavior::system)
            .with_system(Swarming::system)
            .add_to_world(&world)
            .map_err(|err: AddWorkload| Error::Create(err.to_string()))?;

        Ok(Self {
            world,
            paused: false,
        })
    }

    pub fn run<F>(&mut self, mut io: F) -> Result<(), Error>
    where
        F: FnMut(SimulatorOutput) -> Config + 'static,
    {
        if !self.paused {
            self.world
                .run_workload("sim")
                .map_err(|err: RunWorkload| Error::Run(err.to_string()))?;
        }

        let mut new_cfg: Config = Config::default();

        self.world.run(
            |positions: View<Position>,
             velocities: View<Velocity>,
             speeds: View<Speed>,
             neighbor_counts: View<Density>| {
                new_cfg = io(SimulatorOutput {
                    positions: positions.iter().map(|position| position.0.into()).collect(),
                    velocities: velocities
                        .iter()
                        .map(|velocity| velocity.0.into())
                        .collect(),
                    speeds: speeds.iter().map(|speed| speed.0.value).collect(),
                    densities: neighbor_counts
                        .iter()
                        .map(|neighbor_count| neighbor_count.value)
                        .collect(),
                });
            },
        );

        if self.world.run(|cfg: UniqueView<Config>| *cfg != new_cfg) {
            self.update_config(new_cfg);
        }

        self.paused = new_cfg.paused;

        Ok(())
    }

    pub fn update_config(&mut self, new_cfg: Config) {
        let old_cfg: Config = self
            .world
            .run(|mut cfg: UniqueViewMut<Config>| mem::replace(&mut *cfg, new_cfg));

        match new_cfg.entity_count.cmp(&old_cfg.entity_count) {
            Ordering::Greater => {
                let to_add: usize = new_cfg.entity_count - old_cfg.entity_count;
                Fish::add(&mut self.world, to_add, new_cfg);
            }
            Ordering::Less => {
                let to_remove: usize = old_cfg.entity_count - new_cfg.entity_count;
                Fish::remove(&mut self.world, to_remove);
            }
            _ => (),
        }
    }
}
