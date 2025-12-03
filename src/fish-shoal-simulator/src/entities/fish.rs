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
    Config, FishIdentifier, Social, Density, Position, Scalar, Speed, Stress, TargetSpeed,
    TargetVelocity, Vec2, Velocity,
};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use shipyard::{EntityId, IntoIter, View, World};

#[derive(Debug)]
pub struct Fish;

impl Fish {
    pub fn add(world: &mut World, amount: usize, cfg: Config) {
        let mut rng: ThreadRng = rand::rng();

        for _ in 0..amount {
            world.add_entity((
                FishIdentifier,
                Position(Vec2::new_random(
                    &mut rng,
                    0.0..cfg.width as f32,
                    0.0..cfg.height as f32,
                )),
                Velocity(Vec2::random_dir(&mut rng)),
                TargetVelocity(Vec2::random_dir(&mut rng)),
                Speed(Scalar::ZERO),
                TargetSpeed(Scalar::new_random(&mut rng, 10.0..100.0)),
                Stress(Scalar::new(0.1)),
                Density::default(),
                Social::default(),
            ));
        }
    }

    pub fn remove(world: &mut World, amount: usize) {
        let maybe_ids: Option<Vec<EntityId>> = world.run(|fish: View<FishIdentifier>| {
            let ids: Vec<EntityId> = (&fish).iter().with_id().map(|(id, _)| id).collect();

            if ids.is_empty() {
                return None;
            }

            let mut rng: ThreadRng = rand::rng();
            let count: usize = amount.min(ids.len());

            let mut indices: Vec<usize> = (0..ids.len()).collect();
            indices.shuffle(&mut rng);
            let chosen: Vec<EntityId> = indices.into_iter().take(count).map(|i| ids[i]).collect();

            Some(chosen)
        });

        if let Some(ids) = maybe_ids {
            for id in ids {
                let _ = world.delete_entity(id);
            }
        }
    }
}
