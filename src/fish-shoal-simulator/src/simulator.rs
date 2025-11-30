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
    Config, DeltaTime, Error, Fish, Position, SimulatorOutput, Speed, SystemBundle, Velocity,
};
use shipyard::{
    error::{AddWorkload, RunWorkload},
    {IntoIter, UniqueView, UniqueViewMut, View, World},
};
use std::{cmp::Ordering, mem};

pub struct FishShoalSimulator {
    world: World,
}

impl FishShoalSimulator {
    pub fn new() -> Result<Self, Error> {
        let mut world: World = World::default();
        let cfg: Config = Config::default();

        world.add_unique(Config::default());
        world.add_unique(DeltaTime::default());

        Fish::add(&mut world, cfg.entity_count, cfg.width, cfg.height);
        SystemBundle::build(&world).map_err(|err: AddWorkload| Error::Create(err.to_string()))?;

        Ok(Self { world })
    }

    pub fn run<F>(&mut self, mut io: F) -> Result<(), Error>
    where
        F: FnMut(SimulatorOutput) -> Config + 'static,
    {
        SystemBundle::run(&self.world).map_err(|err: RunWorkload| Error::Run(err.to_string()))?;

        let mut new_cfg: Config = Config::default();

        self.world.run(
            |positions: View<Position>, velocities: View<Velocity>, speeds: View<Speed>| {
                new_cfg = io(SimulatorOutput {
                    positions: positions.iter().map(|&position| position).collect(),
                    velocities: velocities.iter().map(|&position| position).collect(),
                    speeds: speeds.iter().map(|&position| position).collect(),
                });
            },
        );

        if self.world.run(|cfg: UniqueView<Config>| *cfg == new_cfg) {
            self.update_config(new_cfg);
        }

        Ok(())
    }

    pub fn update_config(&mut self, new_cfg: Config) {
        let old_cfg: Config = self
            .world
            .run(|mut cfg: UniqueViewMut<Config>| mem::replace(&mut *cfg, new_cfg));

        match new_cfg.entity_count.cmp(&old_cfg.entity_count) {
            Ordering::Greater => {
                let to_add: usize = new_cfg.entity_count - old_cfg.entity_count;
                Fish::add(&mut self.world, to_add, old_cfg.width, old_cfg.height);
            }
            Ordering::Less => {
                let to_remove: usize = old_cfg.entity_count - new_cfg.entity_count;
                Fish::remove(&mut self.world, to_remove);
            }
            _ => (),
        }
    }
}
