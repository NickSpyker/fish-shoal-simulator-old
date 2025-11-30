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

use crate::error::Error;
use fish_shoal_gui::FishShoalGui;
use fish_shoal_simulator::{Config, FishShoalSimulator, SimulatorOutput};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

pub struct FishShoalApp;

impl FishShoalApp {
    pub fn run() -> Result<(), Error> {
        let mut sim: FishShoalSimulator = FishShoalSimulator::new().map_err(Error::Simulator)?;

        let (cfg_sender, cfg_receiver): (Sender<Config>, Receiver<Config>) =
            mpsc::channel::<Config>();

        let (data_sender, data_receiver): (Sender<SimulatorOutput>, Receiver<SimulatorOutput>) =
            mpsc::channel::<SimulatorOutput>();

        let gui: FishShoalGui = FishShoalGui::new(data_receiver, cfg_sender);

        let sim_thread: JoinHandle<Result<(), Error>> = thread::spawn(move || {
            loop {
                let cfg: Config = match cfg_receiver.recv() {
                    Ok(cfg) => cfg,
                    Err(_) => break,
                };

                let data_sender: Sender<SimulatorOutput> = data_sender.clone();

                sim.run(move |output: SimulatorOutput| {
                    if data_sender.send(output).is_err() {
                        return Config {
                            running: false,
                            ..Default::default()
                        };
                    }
                    cfg
                })
                .map_err(Error::Simulator)?;
            }
            Ok(())
        });

        gui.run().map_err(Error::Gui)?;
        sim_thread.join().map_err(Error::Thread)?
    }
}
