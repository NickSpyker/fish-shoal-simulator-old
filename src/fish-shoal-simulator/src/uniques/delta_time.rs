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

use shipyard::Unique;
use std::{
    ops::Mul,
    time::{Duration, Instant},
};

#[derive(Unique, Debug, Copy, Clone)]
pub struct DeltaTime {
    last_time: Instant,
    delta: Duration,
}

impl DeltaTime {
    pub fn calc(&mut self) {
        let now: Instant = Instant::now();
        self.delta = now - self.last_time;
        self.last_time = now;
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self {
            last_time: Instant::now(),
            delta: Duration::default(),
        }
    }
}

impl Mul<f32> for DeltaTime {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.delta.as_secs_f32() * rhs
    }
}

impl Mul<DeltaTime> for f32 {
    type Output = f32;

    fn mul(self, rhs: DeltaTime) -> Self::Output {
        self * rhs.delta.as_secs_f32()
    }
}
