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

use shipyard::Component;

#[derive(Component, Debug, Default)]
pub struct Social {
    pub is_in_group: bool,
}

impl Social {
    pub fn set_grouped(&mut self) {
        self.is_in_group = true;
    }

    pub fn set_alone(&mut self) {
        self.is_in_group = false;
    }
}
