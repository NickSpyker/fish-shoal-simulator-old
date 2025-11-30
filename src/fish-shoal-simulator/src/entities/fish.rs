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

use crate::{FishIdentifier, Position, Speed, Stress, TargetSpeed, TargetVelocity, Velocity};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use shipyard::{EntityId, IntoIter, View, World};

pub struct Fish;

impl Fish {
    pub fn add(world: &mut World, amount: usize, area_width: usize, area_height: usize) {
        for _ in 0..amount {
            world.add_entity((
                FishIdentifier,
                Position::new_random(0.0, area_width as f32, 0.0, area_height as f32),
                Velocity::new(),
                TargetVelocity::new(),
                Speed::new_zero(),
                TargetSpeed::new_random(10.0, 100.0),
                Stress::default(),
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
