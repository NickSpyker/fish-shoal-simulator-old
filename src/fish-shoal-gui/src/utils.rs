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

use eframe::epaint::Color32;

pub struct Utils;

impl Utils {
    pub fn speed_to_color(speed: f32) -> Color32 {
        let s: f32 = speed.clamp(0.0, 100.0) / 100.0;

        let r: u8 = (255.0 * s) as u8;
        let g: u8 = 0u8;
        let b: u8 = (255.0 * (1.0 - s)) as u8;

        Color32::from_rgb(r, g, b)
    }
}
