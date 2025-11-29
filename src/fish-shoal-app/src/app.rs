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

pub struct FishShoalApp {
    sim_config_sender: Sender<Config>,
    sim_config_receiver: Receiver<Config>,
    sim_data_sender: Sender<SimulatorOutput>,
    sim_data_receiver: Receiver<SimulatorOutput>,
}

impl FishShoalApp {
    pub fn new() -> Self {
        let (sim_config_sender, sim_config_receiver) = mpsc::channel();
        let (sim_data_sender, sim_data_receiver) = mpsc::channel();
        Self {
            sim_config_sender,
            sim_config_receiver,
            sim_data_sender,
            sim_data_receiver,
        }
    }

    pub fn run(self) -> Result<(), Error> {
        let mut sim = FishShoalSimulator::new().map_err(Error::Simulator)?;
        let gui = FishShoalGui::new(self.sim_data_receiver, self.sim_config_sender);

        let sim_thread: JoinHandle<Result<(), Error>> = thread::spawn(move || {
            let data_sender = self.sim_data_sender;
            let config_receiver = self.sim_config_receiver;

            let mut is_running: bool = true;
            while is_running {
                let data_sender = data_sender.clone();

                let config: Config = config_receiver.recv().map_err(Error::Receiver)?;
                is_running = config.is_running;

                sim.run(move |output: SimulatorOutput| {
                    data_sender
                        .send(output)
                        .expect("Fish Shoal App crash caused by simulator data sender");
                    config
                })
                .map_err(Error::Simulator)?;
            }
            Ok(())
        });

        gui.run().map_err(Error::Gui)?;
        sim_thread.join().map_err(Error::Thread)?
    }
}
