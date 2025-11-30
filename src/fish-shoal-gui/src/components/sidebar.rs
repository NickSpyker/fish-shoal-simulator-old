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
    egui::{Context, RichText, SidePanel, Slider},
    Frame,
};

pub struct SideBar;

impl SideBar {
    pub(crate) fn render(app: &mut FishShoalGui, ctx: &Context, _frame: &mut Frame) {
        SidePanel::left("sidebar")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Configuration");

                ui.separator();
                ui.heading(RichText::new("Entities").size(14.0));
                ui.add(Slider::new(&mut app.config.entity_count, 0..=10_000).text("Count"));

                ui.separator();
                ui.heading(RichText::new("Area").size(14.0));
                ui.add(
                    Slider::new(&mut app.config.width, 100..=app.screen.x as usize).text("Width"),
                );
                ui.add(
                    Slider::new(&mut app.config.height, 100..=app.screen.y as usize).text("Height"),
                );

                ui.separator();
                ui.heading(RichText::new("Idle behavior change probability").size(14.0));
                let mut dir_change_prob: f64 = app.config.direction_change_prob * 100.0;
                let mut speed_change_prob: f64 = app.config.speed_change_prob * 100.0;
                let mut stress_change_proba: f64 = app.config.stress_change_prob * 100.0;
                ui.add(
                    Slider::new(&mut dir_change_prob, 0.0..=100.0)
                        .suffix(" %")
                        .text("Direction"),
                );
                ui.add(
                    Slider::new(&mut speed_change_prob, 0.0..=100.0)
                        .suffix(" %")
                        .text("Speed"),
                );
                ui.add(
                    Slider::new(&mut stress_change_proba, 0.0..=100.0)
                        .suffix(" %")
                        .text("Stress"),
                );
                app.config.direction_change_prob = dir_change_prob / 100.0;
                app.config.speed_change_prob = speed_change_prob / 100.0;
                app.config.stress_change_prob = stress_change_proba / 100.0;
            });
    }
}
