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

            let area: Rect = Self::build_area(app, rect);

            let painter: Painter = ui.painter_at(rect);
            painter.rect_stroke(
                area,
                0.0,
                Stroke::new(1.0, Color32::WHITE),
                StrokeKind::Middle,
            );

            Self::build_grid(app, area, &painter);

            if let Ok(output) = app.data_receiver.recv() {
                Entities::render_all(painter, output, area.left_top());
            }
        });
    }

    fn build_area(app: &mut FishShoalGui, rect: Rect) -> Rect {
        let margin_hor: f32 = (app.screen.x - app.config.width as f32) / 2.0;
        let margin_ver: f32 = (app.screen.y - app.config.height as f32) / 2.0;

        Rect::from_min_size(
            Pos2::new(rect.min.x + margin_hor, rect.min.y + margin_ver),
            Vec2::new(app.config.width as f32, app.config.height as f32),
        )
    }

    fn build_grid(app: &mut FishShoalGui, area: Rect, painter: &Painter) {
        let cell_size: f32 = app.config.attraction_radius;
        let stroke: Stroke = Stroke::new(0.05, Color32::GRAY);

        let mut x: f32 = area.min.x + cell_size;
        while x < area.max.x {
            painter.line_segment([Pos2::new(x, area.min.y), Pos2::new(x, area.max.y)], stroke);
            x += cell_size;
        }

        let mut y: f32 = area.min.y + cell_size;
        while y < area.max.y {
            painter.line_segment([Pos2::new(area.min.x, y), Pos2::new(area.max.x, y)], stroke);
            y += cell_size;
        }
    }
}
