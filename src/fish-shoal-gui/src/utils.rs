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
    pub fn density_to_color(density: usize) -> Color32 {
        let d: f32 = density.clamp(0, 6) as f32 / 6.0;

        let r: f32 = 255.0 * d;
        let b: f32 = 255.0 - r;
        let g: f32 = 255.0 - (r - b).abs();

        Color32::from_rgb(r as u8, g as u8, b as u8)
    }
}
