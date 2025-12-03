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

use crate::{Chunks, Config, Position};
use shipyard::{EntityId, IntoIter, UniqueView, UniqueViewMut, View};

#[derive(Debug)]
pub struct LoadChunks;

impl LoadChunks {
    pub fn system(
        positions: View<Position>,
        cfg: UniqueView<Config>,
        mut chunks: UniqueViewMut<Chunks>,
    ) {
        chunks.clear();
        chunks.resize(cfg.attraction_radius);

        (&positions)
            .iter()
            .with_id()
            .for_each(|(id, pos): (EntityId, &Position)| {
                chunks.store(&pos.0, id.uindex() as u32);
            });
    }
}
