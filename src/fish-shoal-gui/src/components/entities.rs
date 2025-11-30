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

use crate::Utils;
use eframe::{
    egui::{Painter, Shape, Stroke},
    emath::{Pos2, Vec2},
    epaint::Color32,
};
use fish_shoal_simulator::SimulatorOutput;

pub struct Entities;

impl Entities {
    pub fn render_all(painter: Painter, data: SimulatorOutput, origin: Pos2) {
        for i in 0..data.positions.len() {
            Self::render_entity(i, &painter, &data, origin);
        }
    }

    fn render_entity(idx: usize, painter: &Painter, data: &SimulatorOutput, origin: Pos2) {
        let position: [f32; 2] = data.positions[idx];
        let velocity: [f32; 2] = data.velocities[idx];
        let speed: f32 = data.speeds[idx];

        let screen_pos: Pos2 = origin + Vec2::new(position[0], position[1]);
        let color: Color32 = Utils::speed_to_color(speed);

        if speed > 0.1 {
            let vel_vec: Vec2 = Vec2::new(velocity[0], velocity[1]);

            let direction: Vec2 = vel_vec.normalized();

            let size: f32 = 6.0;
            let width: f32 = 3.0;

            let nose: Pos2 = screen_pos + direction * size;

            let right: Vec2 = Vec2::new(direction.y, -direction.x) * width;
            let tail_base: Pos2 = screen_pos - direction * (size * 0.5);

            let corner_left: Pos2 = tail_base - right;
            let corner_right: Pos2 = tail_base + right;

            painter.add(Shape::convex_polygon(
                vec![nose, corner_right, corner_left],
                color,
                Stroke::NONE,
            ));
        } else {
            painter.circle_filled(screen_pos, 2.0, color);
        }
    }
}
