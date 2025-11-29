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

use crate::{FishShoalGui, UiComponent, Utils};
use eframe::{
    egui::{CentralPanel, Context},
    emath::{Pos2, Rect, Vec2},
    epaint::{Color32, Stroke, StrokeKind},
    Frame,
};

pub struct Simulation;

impl UiComponent for Simulation {
    fn render(app: &mut FishShoalGui, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            app.available_area = rect.size();

            let painter = ui.painter_at(rect);

            let margin_hor: f32 = (app.available_area.x - app.config.width as f32) / 2.0;
            let margin_ver: f32 = (app.available_area.y - app.config.height as f32) / 2.0;
            let top_left = Pos2::new(rect.min.x + margin_hor, rect.min.y + margin_ver);
            let config_rect = Rect::from_min_size(
                top_left,
                Vec2::new(app.config.width as f32, app.config.height as f32),
            );

            let origin = config_rect.left_top();

            painter.rect_stroke(
                config_rect,
                0.0,
                Stroke::new(1.0, Color32::WHITE),
                StrokeKind::Middle,
            );

            if let Ok(output) = app.data_receiver.recv() {
                let count: usize = output.positions.len();
                for i in 0..count {
                    let position = output.positions[i];
                    let velocity = output.velocities[i];
                    let speed = output.speeds[i];

                    let screen_x = origin.x + position.x;
                    let screen_y = origin.y + position.y;

                    let color = Utils::speed_to_color(speed.0);

                    painter.circle_filled(Pos2::new(screen_x, screen_y), 2.0, color);
                }
            }
        });
    }
}
