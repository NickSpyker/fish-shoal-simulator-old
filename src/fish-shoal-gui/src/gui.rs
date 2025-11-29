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

use crate::{error::EFrameError, Error};
use eframe::{
    egui::{CentralPanel, Context, Rect, SidePanel, Slider, Stroke, Vec2, ViewportBuilder}, epaint::{Color32, Pos2, StrokeKind}, App,
    Frame,
    NativeOptions,
};
use fish_shoal_simulator::{Config, SimulatorOutput};
use std::sync::mpsc::{Receiver, Sender};

pub struct FishShoalGui {
    data_receiver: Receiver<SimulatorOutput>,
    config_sender: Sender<Config>,
    config: Config,
    available_area: Vec2,
}

impl FishShoalGui {
    pub fn new(data_receiver: Receiver<SimulatorOutput>, config_sender: Sender<Config>) -> Self {
        Self {
            data_receiver,
            config_sender,
            config: Config::default(),
            available_area: Vec2::default(),
        }
    }

    pub fn run(self) -> Result<(), Error> {
        eframe::run_native(
            "Fish Shoal Simulator",
            NativeOptions {
                viewport: ViewportBuilder::default()
                    .with_min_inner_size([384.0, 216.0])
                    //.with_inner_size([1024.0, 576.0])
                    .with_maximized(true),
                centered: true,
                ..Default::default()
            },
            Box::new(|_| Ok(Box::new(self))),
        )
        .map_err(|err| Error::EFrame(EFrameError(err.to_string())))
    }
}

impl App for FishShoalGui {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if self.config_sender.send(self.config).is_err() {
            return;
        }

        SidePanel::left("sidebar")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Configuration");

                ui.separator();

                let mut width = self.config.width as u32;
                let max_width = self.available_area.x as u32;
                ui.add(Slider::new(&mut width, 100..=max_width).text("Width"));
                self.config.width = width as usize;

                let mut height = self.config.height as u32;
                let max_height = self.available_area.y as u32;
                ui.add(Slider::new(&mut height, 100..=max_height).text("Height"));
                self.config.height = height as usize;

                ui.separator();

                let mut nb_entities = self.config.nb_entities as u32;
                ui.add(Slider::new(&mut nb_entities, 0..=250_000).text("Entities"));
                self.config.nb_entities = nb_entities as usize;
            });

        CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            self.available_area = rect.size();

            let painter = ui.painter_at(rect);

            let margin_hor: f32 = (self.available_area.x - self.config.width as f32) / 2.0;
            let margin_ver: f32 = (self.available_area.y - self.config.height as f32) / 2.0;
            let top_left = Pos2::new(rect.min.x + margin_hor, rect.min.y + margin_ver);
            let config_rect = Rect::from_min_size(
                top_left,
                Vec2::new(self.config.width as f32, self.config.height as f32),
            );

            let origin = config_rect.left_top();

            painter.rect_stroke(
                config_rect,
                0.0,
                Stroke::new(1.0, Color32::WHITE),
                StrokeKind::Middle,
            );

            if let Ok(output) = self.data_receiver.recv() {
                for position in &output.positions {
                    let screen_x = origin.x + position.x;
                    let screen_y = origin.y + position.y;
                    painter.circle_filled(Pos2::new(screen_x, screen_y), 2.0, Color32::BLUE);
                }
            }
        });

        ctx.request_repaint();
    }
}
