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

use crate::{DeltaTime, Position, Speed, Velocity};
use rayon::prelude::*;
use shipyard::{IntoIter, UniqueView, View, ViewMut};

pub struct Motion;

impl Motion {
    pub fn system(
        mut positions: ViewMut<Position>,
        velocities: View<Velocity>,
        speeds: View<Speed>,
        delta_time: UniqueView<DeltaTime>,
    ) {
        let dt: DeltaTime = *delta_time;

        (&mut positions, &velocities, &speeds)
            .par_iter()
            .for_each(|(pos, vel, speed)| {
                pos.x += dt * (*speed * vel.dx).0;
                pos.y += dt * (*speed * vel.dy).0;
            })
    }
}
