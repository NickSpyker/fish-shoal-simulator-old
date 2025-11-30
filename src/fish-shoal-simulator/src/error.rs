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
    error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum Error {
    Create(String),
    Run(String),
    Config(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Fish Shoal Simulator failed to {}",
            match self {
                Self::Create(err) => format!("create: {err}"),
                Self::Run(err) => format!("run: {err}"),
                Self::Config(err) => format!("config: {err}"),
            }
        )
    }
}

impl error::Error for Error {}
