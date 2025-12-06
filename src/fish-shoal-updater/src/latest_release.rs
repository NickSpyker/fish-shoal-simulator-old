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

use dotenv_codegen::dotenv;
use reqwest::{
    Error,
    blocking::{Client, Response},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LatestRelease;

impl LatestRelease {
    pub fn fetch() -> Result<String, String> {
        let client: Client = Client::new();

        let response: Response = client
            .get(dotenv!("URL"))
            .send()
            .map_err(|err: Error| format!("error sending request: {err}"))?;

        match response
            .url()
            .path()
            .rsplit_once(dotenv!("URL_VERSION_DELIMITER"))
        {
            Some((_, version)) => Ok(version.to_string()),
            None => Err("error parsing latest release version".to_string()),
        }
    }

    pub fn download_url(bin_name: &str) -> String {
        format!("{}{}{bin_name}", dotenv!("URL"), dotenv!("DOWNLOAD"))
    }
}
