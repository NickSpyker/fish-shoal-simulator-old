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

use crate::{Config, DeltaTime, Error, Fish, Position, SimulatorOutput, SystemBundle};
use shipyard::{IntoIter, View, World};
use std::cmp::Ordering;

#[derive(Default)]
pub struct FishShoalSimulator {
    entities: World,
    config: Config,
}

impl FishShoalSimulator {
    pub fn new() -> Result<Self, Error> {
        let world = World::new();
        SystemBundle::build(&world).map_err(|err| Error::Create(err.to_string()))?;
        world.add_unique(DeltaTime::new());
        Ok(Self {
            entities: world,
            config: Config::default(),
        })
    }

    pub fn setup(&mut self, config: Config) {
        self.config.width = config.width;
        self.config.height = config.height;

        match config.nb_entities.cmp(&self.config.nb_entities) {
            Ordering::Greater => {
                let to_add: usize = config.nb_entities - self.config.nb_entities;
                Fish::add(
                    &mut self.entities,
                    to_add,
                    self.config.width,
                    self.config.height,
                );
                self.config.nb_entities = config.nb_entities;
            }
            Ordering::Less => {
                let to_remove: usize = self.config.nb_entities - config.nb_entities;
                Fish::remove(&mut self.entities, to_remove);
                self.config.nb_entities = config.nb_entities;
            }
            _ => (),
        }
    }

    pub fn run<F>(&mut self, mut io: F) -> Result<(), Error>
    where
        F: FnMut(SimulatorOutput) -> Config + 'static,
    {
        SystemBundle::run(&self.entities).map_err(|err| Error::Run(err.to_string()))?;
        let mut config = Config::default();
        self.entities.run(|positions: View<Position>| {
            config = io(SimulatorOutput {
                positions: positions.iter().map(|&position| position).collect(),
            });
        });
        if config != self.config {
            self.setup(config);
        }
        Ok(())
    }
}
