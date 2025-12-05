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

use crate::FishShoalGui;
use eframe::{
    egui::Painter,
    emath::{Pos2, Vec2},
    epaint::{Color32, Shape, Stroke},
};
use std::f32::consts::PI;

pub struct AlgoRadiusFov;

impl AlgoRadiusFov {
    pub fn render(
        app: &mut FishShoalGui,
        position: Pos2,
        velocity: Vec2,
        painter: &Painter,
        filed: bool,
    ) {
        let base_angle: f32 = velocity.angle();
        let deg_to_rad: f32 = PI / 180.0;

        let draw_cone = |radius: f32, fov_deg: f32, color: Color32| {
            let half_rad: f32 = (fov_deg * deg_to_rad) / 2.0;

            let mut points: Vec<Pos2> = Vec::new();

            points.push(position);

            let steps: i32 = 20;
            for i in 0..=steps {
                let t: f32 = i as f32 / steps as f32;
                let angle: f32 = base_angle - half_rad + (2.0 * half_rad * t);
                let (s, c): (f32, f32) = angle.sin_cos();
                let p: Pos2 = position + Vec2::new(c, s) * radius;
                points.push(p);
            }

            points.push(position);

            if filed {
                painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
            } else {
                painter.add(Shape::line(points, Stroke::new(1.0, color)));
            }
        };

        let attraction_color: Color32 = Color32::from_rgba_unmultiplied(0, 255, 0, 80);
        draw_cone(
            app.config.attraction_radius,
            app.config.attraction_fov,
            attraction_color,
        );

        let alignment_color: Color32 = Color32::from_rgba_unmultiplied(0, 128, 255, 80);
        draw_cone(
            app.config.alignment_radius,
            app.config.alignment_fov,
            alignment_color,
        );

        let avoidance_color: Color32 = Color32::from_rgba_unmultiplied(255, 0, 0, 120);

        if filed {
            painter.circle_filled(position, app.config.avoidance_radius, avoidance_color);
        } else {
            painter.circle_stroke(
                position,
                app.config.avoidance_radius,
                Stroke::new(1.0, avoidance_color),
            );
        }
    }
}
