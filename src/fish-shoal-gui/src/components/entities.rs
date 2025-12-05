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

use super::AlgoRadiusFov;
use crate::{FishShoalGui, FocusedFishData};
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
    pub fn render(
        app: &mut FishShoalGui,
        primary_pressed: bool,
        painter: Painter,
        data: SimulatorOutput,
        origin: Pos2,
    ) {
        for idx in 0..data.ids.len() {
            Self::render_entity(idx, app, primary_pressed, &painter, &data, origin);
        }

        if let Some([mx, my]) = app.config.mouse_pos {
            let pos: Pos2 = Pos2::new(mx, my);
            if let Some([omx, omy]) = app.old_mouse_pos {
                let old_pos: Pos2 = Pos2::new(omx, omy);
                let vel: Vec2 = pos - old_pos;
                let fish: Vec<Pos2> = Self::fish(pos, vel);
                painter.add(Shape::convex_polygon(fish, Color32::RED, Stroke::NONE));
            } else {
                painter.circle_filled(pos, 2.0, Color32::RED);
            };
        }
    }

    fn render_entity(
        idx: usize,
        app: &mut FishShoalGui,
        primary_pressed: bool,
        painter: &Painter,
        data: &SimulatorOutput,
        origin: Pos2,
    ) {
        let id: usize = data.ids[idx];
        let position: [f32; 2] = data.positions[idx];
        let velocity: [f32; 2] = data.velocities[idx];
        let density: usize = data.densities[idx];
        let speed: f32 = data.speeds[idx];

        let is_focused_fish: bool = Some(id) == app.focused_fish_id;
        if is_focused_fish {
            app.focused_fish_data = Some(FocusedFishData::new(position, velocity, speed));
        }

        let position: Pos2 = origin + Vec2::new(position[0], position[1]);
        let color: Color32 = Self::density_to_color(density);

        if speed > 0.1 {
            let velocity: Vec2 = Vec2::new(velocity[0], velocity[1]);

            let points: Vec<Pos2> = Self::fish(position, velocity);

            if primary_pressed && let Some([mx, my]) = app.config.mouse_pos {
                if position.distance(Pos2::new(mx, my)) <= FISH_LENGTH {
                    app.focused_fish_id = Some(id);
                }
            }

            painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
            if is_focused_fish {
                AlgoRadiusFov::render(app, position, velocity, painter, false);
            }
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
