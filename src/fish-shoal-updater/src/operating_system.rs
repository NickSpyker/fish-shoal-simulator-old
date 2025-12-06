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

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct OperatingSystem {
    pub os_name: String,
    pub bin_name: String,
}

impl Display for OperatingSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.os_name)
    }
}

impl OperatingSystem {
    pub fn get() -> Self {
        #[cfg(target_os = "windows")]
        return Self {
            os_name: "windows".to_string(),
            bin_name: "windows.exe".to_string(),
        };

        #[cfg(target_os = "linux")]
        return Self {
            os_name: "linux".to_string(),
            bin_name: "linux".to_string(),
        };

        #[cfg(target_os = "macos")]
        return Self {
            os_name: "macos".to_string(),
            bin_name: "macos".to_string(),
        };

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        return Self {
            os_name: "unsupported os".to_string(),
            bin_name: "unsupported operating system".to_string(),
        };
    }
}
