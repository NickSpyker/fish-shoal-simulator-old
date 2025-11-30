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

use crate::{Config, Position};
use rayon::prelude::*;
use shipyard::{IntoIter, UniqueView, ViewMut};

pub struct OutOfBound;

impl OutOfBound {
    pub fn system(mut positions: ViewMut<Position>, cfg: UniqueView<Config>) {
        let width: f32 = cfg.width as f32;
        let height: f32 = cfg.height as f32;

        (&mut positions).par_iter().for_each(|pos| {
            if pos.x <= 0.0 {
                pos.x = width - 1.0;
            } else if pos.x >= width {
                pos.x = 1.0;
            }

            if pos.y <= 0.0 {
                pos.y = height - 1.0;
            } else if pos.y >= height {
                pos.y = 1.0;
            }
        });
    }
}
