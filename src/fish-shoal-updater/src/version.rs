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
    fmt::{self, Display, Formatter},
    num::ParseIntError,
};

#[derive(Debug)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn from_str(value: &str) -> Result<Self, String> {
        let version: Vec<&str> = value.split('.').collect();
        if version.len() != 3 {
            return Err(format!("Invalid version: {value}"));
        }

        let major: usize = version[0]
            .parse()
            .map_err(|err: ParseIntError| format!("Invalid major version {value}: {err}"))?;

        let minor: usize = version[1]
            .parse()
            .map_err(|err: ParseIntError| format!("Invalid minor version {value}: {err}"))?;

        let patch: usize = version[2]
            .parse()
            .map_err(|err: ParseIntError| format!("Invalid patch version: {value}: {err}"))?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    pub fn from_string(value: String) -> Result<Self, String> {
        Self::from_str(&value)
    }
}
