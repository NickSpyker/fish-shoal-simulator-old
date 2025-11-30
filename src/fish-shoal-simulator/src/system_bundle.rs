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

use crate::{CalculateDeltaTime, LerpToTarget, Motion, OutOfBound, RandomBehavior};
use shipyard::{
    error::{AddWorkload, RunWorkload}, Workload,
    World,
};

pub struct SystemBundle;

impl SystemBundle {
    const LABEL: &str = "systems";

    pub fn run(world: &World) -> Result<(), RunWorkload> {
        world.run_workload(Self::LABEL)
    }

    pub fn build(world: &World) -> Result<(), AddWorkload> {
        Workload::new(Self::LABEL)
            .with_system(CalculateDeltaTime::system)
            .with_barrier()
            .with_system(Motion::system)
            .with_system(OutOfBound::system)
            .with_system(LerpToTarget::system)
            .with_system(RandomBehavior::system)
            .add_to_world(&world)
    }
}
