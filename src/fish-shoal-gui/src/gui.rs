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

use crate::{Error, SideBar, Simulation};
use eframe::{
    egui::{Context, Vec2, ViewportBuilder, Visuals}, App, CreationContext, Frame,
    NativeOptions,
};
use fish_shoal_simulator::{Config, SimulatorOutput};
use std::sync::mpsc::{Receiver, Sender};

pub struct FishShoalGui {
    pub data_receiver: Receiver<SimulatorOutput>,
    pub config_sender: Sender<Config>,
    pub config: Config,
    pub screen: Vec2,
    initialized: bool,
}

impl FishShoalGui {
    pub fn new(data_receiver: Receiver<SimulatorOutput>, config_sender: Sender<Config>) -> Self {
        Self {
            data_receiver,
            config_sender,
            config: Config::default(),
            screen: Vec2::default(),
            initialized: false,
        }
    }

    pub fn run(self) -> Result<(), Error> {
        eframe::run_native(
            "Fish Shoal Simulator",
            NativeOptions {
                viewport: ViewportBuilder::default()
                    .with_min_inner_size([384.0, 216.0])
                    .with_maximized(true),
                centered: true,
                ..Default::default()
            },
            Box::new(|cc: &CreationContext| {
                cc.egui_ctx.set_visuals(Visuals::dark());
                Ok(Box::new(self))
            }),
        )
        .map_err(|err| Error::EFrame(err.to_string()))
    }
}

impl App for FishShoalGui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if self.config_sender.send(self.config).is_err() {
            return;
        }

        SideBar::render(self, ctx, frame);
        Simulation::render(self, ctx, frame);

        if !self.initialized {
            self.config.width = self.screen.x as usize;
            self.config.height = self.screen.y as usize;
            self.initialized = true;
        }

        ctx.request_repaint();
    }
}
