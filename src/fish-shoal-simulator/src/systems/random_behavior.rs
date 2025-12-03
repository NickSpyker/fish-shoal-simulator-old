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

use crate::{Config, Scalar, Speed, Stress, TargetSpeed, TargetVelocity, Vec2, Velocity};
use rand::{rngs::ThreadRng, Rng};
use rayon::prelude::*;
use shipyard::{IntoIter, UniqueView, View, ViewMut};

#[derive(Debug)]
pub struct RandomBehavior;

impl RandomBehavior {
    pub fn system(
        velocities: View<Velocity>,
        mut target_velocities: ViewMut<TargetVelocity>,
        speeds: View<Speed>,
        mut target_speeds: ViewMut<TargetSpeed>,
        mut stress: ViewMut<Stress>,
        cfg: UniqueView<Config>,
    ) {
        (
            &velocities,
            &mut target_velocities,
            &speeds,
            &mut target_speeds,
            &mut stress,
        )
            .par_iter()
            .for_each(|(vel, target_vel, speed, target_speed, stress)| {
                let mut rng: ThreadRng = rand::rng();

                if vel.0 == target_vel.0 && rng.random_bool(cfg.direction_change_prob) {
                    let random_direction: Vec2 = Vec2::random_dir(&mut rng);
                    target_vel.0 = target_vel
                        .0
                        .lerp(random_direction, rng.random_range(0.0..1.0));
                }

                if speed.0 == target_speed.0 && rng.random_bool(cfg.speed_change_prob) {
                    let random_speed: Scalar = Scalar::new_random(&mut rng, 10.0..100.0);
                    target_speed.0 = target_speed
                        .0
                        .lerp(random_speed, rng.random_range(0.0..1.0));
                }

                if rng.random_bool(cfg.stress_change_prob) {
                    stress.0 = Scalar::new_random(&mut rng, 0.1..0.5);
                }
            });
    }
}
