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

use crate::{Entities, FishShoalGui};
use eframe::{
    egui::{CentralPanel, Context, Painter},
    emath::{Pos2, Rect, Vec2},
    epaint::{Color32, Stroke, StrokeKind},
    Frame,
};

pub struct Simulation;

impl Simulation {
    pub(crate) fn render(app: &mut FishShoalGui, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let rect: Rect = ui.max_rect();
            app.screen = rect.size();

            let margin_hor: f32 = (app.screen.x - app.config.width as f32) / 2.0;
            let margin_ver: f32 = (app.screen.y - app.config.height as f32) / 2.0;
            let top_left: Pos2 = Pos2::new(rect.min.x + margin_hor, rect.min.y + margin_ver);
            let config_rect: Rect = Rect::from_min_size(
                top_left,
                Vec2::new(app.config.width as f32, app.config.height as f32),
            );

            let painter: Painter = ui.painter_at(rect);

            painter.rect_stroke(
                config_rect,
                0.0,
                Stroke::new(1.0, Color32::WHITE),
                StrokeKind::Middle,
            );

            if let Ok(output) = app.data_receiver.recv() {
                Entities::render_all(painter, output, config_rect.left_top());
            }
        });
    }
}
