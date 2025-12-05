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

pub type FocusedFishId = usize;

pub struct FocusedFishData {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub speed: f32,
}

impl FocusedFishData {
    pub fn new(position: [f32; 2], velocity: [f32; 2], speed: f32) -> Self {
        Self {
            position,
            velocity,
            speed,
        }
    }
}
