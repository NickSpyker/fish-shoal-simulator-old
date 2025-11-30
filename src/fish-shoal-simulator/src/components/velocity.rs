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
use std::ops::{Add, Mul, Sub};

#[derive(Component, Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Add<Velocity> for Velocity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl Sub<Velocity> for Velocity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

impl Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl Velocity {
    pub fn new() -> Self {
        let mut rng: ThreadRng = rand::rng();

        let dx: f32 = rng.random_range(-1.0..=1.0);
        let dy: f32 = rng.random_range(-1.0..=1.0);

        Self { dx, dy }.normalize()
    }

    pub fn normalize(&mut self) -> Self {
        let len: f32 = self.dx.hypot(self.dy);

        if len > 0.0 {
            self.dx /= len;
            self.dy /= len;
        }

        self.clone()
    }

    pub fn lerp(&mut self, to: &Self, factor: f32) -> Self {
        let new_vec: Self = *self + (*to - *self) * factor;
        *self = new_vec;
        self.normalize()
    }
}
