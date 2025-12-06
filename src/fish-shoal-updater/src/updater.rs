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

use crate::{LatestRelease, OperatingSystem, Version};

#[derive(Debug)]
pub struct Updater {
    current_version: Version,
    latest_version: Version,
    operating_system: OperatingSystem,
}

impl Updater {
    fn build() -> Result<Self, String> {
        Ok(Self {
            current_version: Version::from_str(env!("CARGO_PKG_VERSION"))?,
            latest_version: Version::from_string(LatestRelease::fetch()?)?,
            operating_system: OperatingSystem::get(),
        })
    }

    pub fn check_version(msg: &mut String) -> bool {
        let updater: Self = match Self::build() {
            Ok(updater) => updater,
            Err(err) => {
                *msg = err;
                return false;
            }
        };

        let same_major: bool = updater.current_version.major == updater.latest_version.major;
        let same_minor: bool = updater.current_version.minor == updater.latest_version.minor;
        let same_patch: bool = updater.current_version.patch == updater.latest_version.patch;

        if same_major && same_minor && same_patch {
            *msg = "latest version".to_string();
            false
        } else {
            *msg = format!(
                "new version for {}, from {} to {}",
                updater.operating_system, updater.current_version, updater.latest_version
            );
            true
        }
    }

    pub fn get_latest_version_download_url() -> Option<String> {
        Some(LatestRelease::download_url(
            &Self::build().ok()?.operating_system.bin_name,
        ))
    }
}
