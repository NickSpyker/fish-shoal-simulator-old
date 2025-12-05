/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http:
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use eframe::{
    egui::{Painter, Shape, Stroke},
    emath::{Pos2, Vec2},
    epaint::Color32,
};
use fish_shoal_simulator::SimulatorOutput;

const FISH_LENGTH: f32 = 10.0;
const FISH_HEAD_RADIUS: f32 = 3.0;

pub struct Entities;

impl Entities {
    pub fn render(painter: Painter, data: SimulatorOutput, origin: Pos2) {
        for i in 0..data.ids.len() {
            Self::render_entity(i, &painter, &data, origin);
        }
    }

    fn render_entity(idx: usize, painter: &Painter, data: &SimulatorOutput, origin: Pos2) {
        let position: [f32; 2] = data.positions[idx];
        let velocity: [f32; 2] = data.velocities[idx];
        let density: usize = data.densities[idx];
        let speed: f32 = data.speeds[idx];

        let position: Pos2 = origin + Vec2::new(position[0], position[1]);
        let color: Color32 = Self::density_to_color(density);

        if speed > 0.1 {
            let velocity: Vec2 = Vec2::new(velocity[0], velocity[1]);

            let points: Vec<Pos2> = Self::fish(position, velocity);

            painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
        } else {
            painter.circle_filled(position, 2.0, color);
        }
    }

    fn fish(position: Pos2, velocity: Vec2) -> Vec<Pos2> {
        let direction: Vec2 = velocity.normalized();

        let right: Vec2 = Vec2::new(direction.y, -direction.x);

        let head_center: Pos2 = position + direction * (FISH_LENGTH * 0.2);
        let tail_tip: Pos2 = head_center - direction * (FISH_LENGTH * 0.8);

        let diag_right: Vec2 = (direction + right).normalized();
        let diag_left: Vec2 = (direction - right).normalized();

        let nose_len: f32 = FISH_HEAD_RADIUS * 1.6;

        vec![
            tail_tip,
            head_center + right * FISH_HEAD_RADIUS,
            head_center + diag_right * FISH_HEAD_RADIUS,
            head_center + direction * nose_len,
            head_center + diag_left * FISH_HEAD_RADIUS,
            head_center - right * FISH_HEAD_RADIUS,
        ]
    }

    fn density_to_color(density: usize) -> Color32 {
        let d: f32 = density.clamp(0, 6) as f32 / 6.0;

        let r: f32 = 255.0 * d;
        let b: f32 = 255.0 - r;
        let g: f32 = 255.0 - (r - b).abs();

        Color32::from_rgb(r as u8, g as u8, b as u8)
    }
}
