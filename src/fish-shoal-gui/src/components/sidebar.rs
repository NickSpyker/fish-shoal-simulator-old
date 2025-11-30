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

use crate::{FishShoalGui, UiComponent};
use eframe::{egui::Context, Frame};
use egui::{SidePanel, Slider};

pub struct SideBar;

impl UiComponent for SideBar {
    fn render(app: &mut FishShoalGui, ctx: &Context, _frame: &mut Frame) {
        SidePanel::left("sidebar")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Configuration");

                ui.separator();

                let mut width = app.config.width as u32;
                let max_width = app.available_area.x as u32;
                ui.add(Slider::new(&mut width, 100..=max_width).text("Width"));
                app.config.width = width as usize;

                let mut height = app.config.height as u32;
                let max_height = app.available_area.y as u32;
                ui.add(Slider::new(&mut height, 100..=max_height).text("Height"));
                app.config.height = height as usize;

                ui.separator();

                let mut nb_entities = app.config.nb_entities as u32;
                ui.add(Slider::new(&mut nb_entities, 0..=10_000).text("Entities"));
                app.config.nb_entities = nb_entities as usize;
            });
    }
}
