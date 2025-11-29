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

use rand::{rngs::ThreadRng, Rng};
use shipyard::Component;

#[derive(Component, Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }

    pub fn new_zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn new_random() -> Self {
        let mut rng: ThreadRng = rand::rng();

        let dx: f32 = rng.random_range(-1.0..=1.0);
        let dy: f32 = rng.random_range(-1.0..=1.0);

        Self::new(dx, dy).normalized()
    }

    pub fn normalize(&mut self) {
        let len: f32 = self.dx.hypot(self.dy);
        if len > 0.0 {
            self.dx /= len;
            self.dy /= len;
        }
    }

    pub fn normalized(&self) -> Self {
        let mut vec: Self = self.clone();
        vec.normalize();
        vec
    }
}
