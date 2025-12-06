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

use super::AlgoRadiusFov;
use crate::FishShoalGui;
use eframe::emath::Rect;
use eframe::{
    egui::{Align, Context, Layout, RichText, SidePanel, Slider},
    emath::Vec2,
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
                ui.heading(RichText::new("Information").size(14.0));
                let mouse_statue: String = if let Some([mx, my]) = app.config.mouse_pos {
                    format!("{{{mx:.0}, {my:.0}}}")
                } else {
                    "outside".to_string()
                };
                ui.label(RichText::new(format!("• Mouse: {mouse_statue}")));
                let focused_fish_id: String = if let Some(id) = app.focused_fish_id {
                    id.to_string()
                } else {
                    "none".to_string()
                };
                ui.horizontal(|ui| {
                    ui.label(format!("• Focused fish: {focused_fish_id}"));
                    if app.focused_fish_id.is_some() {
                        if ui.button("Stop").clicked() {
                            app.focused_fish_id = None;
                        }
                    }
                });
                if let Some(fish) = &app.focused_fish_data {
                    ui.label(format!(
                        "    • position: {{{:.0}, {:.0}}}",
                        fish.position[0], fish.position[1]
                    ));
                    ui.label(format!(
                        "    • velocity: {{{:.2}, {:.2}}}",
                        fish.velocity[0], fish.velocity[1]
                    ));
                    ui.label(format!("    • speed:    {:.0}", fish.speed));
                } else {
                    ui.label("    • position: none");
                    ui.label("    • velocity: none");
                    ui.label("    • speed:    none");
                }

                ui.separator();
                ui.heading(RichText::new("Simulation").size(14.0));
                ui.horizontal(|ui| {
                    if ui.selectable_label(!app.config.paused, "Run").clicked() {
                        app.config.paused = false;
                    }
                    if ui.selectable_label(app.config.paused, "Pause").clicked() {
                        app.config.paused = true;
                    }
                });

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

                ui.separator();
                ui.heading(RichText::new("Shoal behavior radius").size(14.0));
                ui.add(
                    Slider::new(&mut app.config.attraction_radius, 3.0..=100.0).text("Attraction"),
                );
                app.config.alignment_radius = app
                    .config
                    .alignment_radius
                    .clamp(2.0, app.config.attraction_radius - 1.0);
                ui.add(Slider::new(&mut app.config.alignment_radius, 2.0..=99.0).text("Alignment"));
                app.config.avoidance_radius = app
                    .config
                    .avoidance_radius
                    .clamp(1.0, app.config.alignment_radius - 1.0);
                ui.add(Slider::new(&mut app.config.avoidance_radius, 1.0..=98.0).text("Avoidance"));
                app.config.alignment_radius = app
                    .config
                    .alignment_radius
                    .clamp(app.config.avoidance_radius + 1.0, 99.0);
                app.config.attraction_radius = app
                    .config
                    .attraction_radius
                    .clamp(app.config.alignment_radius + 1.0, 100.0);

                ui.separator();
                ui.heading(RichText::new("Shoal behavior fov").size(14.0));
                ui.add(
                    Slider::new(&mut app.config.attraction_fov, 0.0..=360.0)
                        .suffix("°")
                        .text("Attraction"),
                );
                ui.add(
                    Slider::new(&mut app.config.alignment_fov, 0.0..=360.0)
                        .suffix("°")
                        .text("Alignment"),
                );

                ui.vertical(|sub_ui| {
                    let rect: Rect = sub_ui.max_rect();
                    AlgoRadiusFov::render(
                        app,
                        rect.center(),
                        Vec2::new(0.0, -1.0),
                        &sub_ui.painter(),
                        true,
                    );
                });

                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    ui.add_space(10.0);

                    if !app.is_latest_version {
                        if let Some(download_link) = &app.latest_version_download_link {
                            ui.hyperlink_to(
                                RichText::new(">> download latest <<").size(10.0),
                                download_link,
                            );
                        }
                    }

                    ui.label(RichText::new(&app.version_msg).size(10.0));
                });
            });
    }
}
