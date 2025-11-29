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

use std::{
    any::Any,
    error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum Error {
    EFrame(eframe::Error),
    Simulator(fish_shoal_simulator::Error),
    Thread(Box<dyn Any + Send + 'static>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fish Shoal GUI error caused by {}",
            match self {
                Self::EFrame(source) => format!("eframe: {source}"),
                Self::Simulator(source) => format!("simulator: {source}"),
                Self::Thread(source) => format!("thread: {:?}", source),
            }
        )
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::EFrame(source) => Some(source),
            Self::Simulator(source) => Some(source),
            Self::Thread(_) => None,
        }
    }
}
