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

use crate::{DeltaTime, Speed, Stress, TargetSpeed, TargetVelocity, Velocity};
use rayon::prelude::*;
use shipyard::{IntoIter, UniqueView, View, ViewMut};

#[derive(Debug)]
pub struct LerpToTarget;

impl LerpToTarget {
    pub fn system(
        mut velocities: ViewMut<Velocity>,
        target_velocities: View<TargetVelocity>,
        mut speeds: ViewMut<Speed>,
        target_speeds: View<TargetSpeed>,
        stress: View<Stress>,
        delta_time: UniqueView<DeltaTime>,
    ) {
        let dt: DeltaTime = *delta_time;

        (
            &mut velocities,
            &target_velocities,
            &mut speeds,
            &target_speeds,
            &stress,
        )
            .par_iter()
            .for_each(|(vel, target_vel, speed, target_speed, stress)| {
                let factor: f32 = dt * stress.0.value * 10.0;

                vel.0 = vel.0.lerp(target_vel.0, factor);
                speed.0 = speed.0.lerp(target_speed.0, factor);
            })
    }
}
