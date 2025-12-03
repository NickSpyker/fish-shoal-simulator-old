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

use crate::Vec2;
use shipyard::Unique;
use std::collections::{HashMap, HashSet};

#[derive(Unique, Debug, Default)]
pub struct Chunks {
    chunk_size: f32,
    chunks: HashMap<u32, HashSet<u32>>,
}

impl Chunks {
    pub fn new(chunk_size: f32) -> Self {
        Self {
            chunk_size,
            chunks: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.chunks.clear();
    }

    pub fn resize(&mut self, chunk_size: f32) {
        self.chunk_size = chunk_size;
    }

    pub fn store(&mut self, pos: &Vec2, id: u32) {
        let chunk_id: u32 = self.chunk_id_from_pos(pos);

        let is_unique: bool = self.chunks.entry(chunk_id).or_default().insert(id);
        debug_assert!(is_unique, "Entity {id} already exists in chunk");
    }

    pub fn remove(&mut self, pos: &Vec2, id: u32) {
        let chunk_id: u32 = self.chunk_id_from_pos(pos);

        if let Some(chunk) = self.chunks.get_mut(&chunk_id) {
            let existed: bool = chunk.remove(&id);
            debug_assert!(existed, "Entity {id} was not in chunk");

            if chunk.is_empty() {
                self.chunks.remove(&chunk_id);
            }
        }
    }

    pub fn load(&self, pos: &Vec2) -> HashSet<u32> {
        let (chunk_x, chunk_y): (u32, u32) = self.chunk_coords(pos);

        let mut data: HashSet<u32> = HashSet::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let x: u32 = (chunk_x as i32 + dx) as u32;
                let y: u32 = (chunk_y as i32 + dy) as u32;

                let chunk_id: u32 = Self::chunk_id_from_coords(x, y);

                if let Some(chunk) = self.chunks.get(&chunk_id) {
                    data.extend(chunk);
                }
            }
        }

        data
    }

    #[inline]
    fn chunk_id_from_pos(&self, pos: &Vec2) -> u32 {
        let (x, y): (u32, u32) = self.chunk_coords(pos);
        Self::chunk_id_from_coords(x, y)
    }

    #[inline]
    fn chunk_id_from_coords(x: u32, y: u32) -> u32 {
        let sum: u32 = x + y;
        (sum * (sum + 1)) / 2 + y
    }

    #[inline]
    fn chunk_coords(&self, pos: &Vec2) -> (u32, u32) {
        debug_assert!(pos.x >= 0.0, "Position {pos} is negative in x");
        debug_assert!(pos.y >= 0.0, "Position {pos} is negative in y");

        (
            (pos.x / self.chunk_size).floor() as u32,
            (pos.y / self.chunk_size).floor() as u32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Chunks;
    use crate::Vec2;
    use std::collections::HashSet;

    #[test]
    fn chunk_new() {
        let chunk_size: f32 = 32.0;
        let chunks_repository: Chunks = Chunks::new(chunk_size);

        assert_eq!(chunks_repository.chunk_size, chunk_size);
        assert!(chunks_repository.chunks.is_empty());
    }

    #[test]
    fn chunk_clear() {
        let chunk_size: f32 = 10.0;
        let mut chunks_repository: Chunks = Chunks::new(chunk_size);
        let entity_position: Vec2 = Vec2::new(5.0, 5.0);
        let entity_identifier: u32 = 1;

        chunks_repository.store(&entity_position, entity_identifier);
        assert!(!chunks_repository.chunks.is_empty());

        chunks_repository.clear();
        assert!(chunks_repository.chunks.is_empty());
    }

    #[test]
    fn chunk_resize() {
        let initial_chunk_size: f32 = 32.0;
        let target_chunk_size: f32 = 64.0;
        let mut chunks_repository: Chunks = Chunks::new(initial_chunk_size);

        chunks_repository.resize(target_chunk_size);

        assert_eq!(chunks_repository.chunk_size, target_chunk_size);
    }

    #[test]
    fn chunk_store() {
        let chunk_size: f32 = 10.0;
        let mut chunks_repository: Chunks = Chunks::new(chunk_size);
        let entity_position: Vec2 = Vec2::new(15.0, 15.0);
        let entity_identifier: u32 = 42;

        let expected_chunk_identifier: u32 = 4;

        chunks_repository.store(&entity_position, entity_identifier);

        let stored_chunk: &HashSet<u32> = chunks_repository
            .chunks
            .get(&expected_chunk_identifier)
            .expect("Chunk should exist after storage");

        assert!(stored_chunk.contains(&entity_identifier));
    }

    #[test]
    fn chunk_remove() {
        let chunk_size: f32 = 10.0;
        let mut chunks_repository: Chunks = Chunks::new(chunk_size);
        let entity_position: Vec2 = Vec2::new(5.0, 5.0);
        let entity_identifier: u32 = 100;

        chunks_repository.store(&entity_position, entity_identifier);
        chunks_repository.remove(&entity_position, entity_identifier);

        assert!(chunks_repository.chunks.is_empty());
    }

    #[test]
    fn chunk_load() {
        let chunk_size: f32 = 10.0;
        let mut chunks_repository: Chunks = Chunks::new(chunk_size);

        let position_center: Vec2 = Vec2::new(15.0, 15.0);
        let entity_center_id: u32 = 1;

        let position_neighbor: Vec2 = Vec2::new(5.0, 15.0);
        let entity_neighbor_id: u32 = 2;

        let position_far: Vec2 = Vec2::new(45.0, 45.0);
        let entity_far_id: u32 = 3;

        chunks_repository.store(&position_center, entity_center_id);
        chunks_repository.store(&position_neighbor, entity_neighbor_id);
        chunks_repository.store(&position_far, entity_far_id);

        let retrieved_entities: HashSet<u32> = chunks_repository.load(&position_center);

        assert!(retrieved_entities.contains(&entity_center_id));
        assert!(retrieved_entities.contains(&entity_neighbor_id));
        assert!(!retrieved_entities.contains(&entity_far_id));
    }

    #[test]
    fn chunk_id_from_pos() {
        let chunk_size: f32 = 10.0;
        let chunks_repository: Chunks = Chunks::new(chunk_size);
        let entity_position: Vec2 = Vec2::new(12.0, 12.0);

        let expected_identifier: u32 = 4;
        let actual_identifier: u32 = chunks_repository.chunk_id_from_pos(&entity_position);

        assert_eq!(actual_identifier, expected_identifier);
    }

    #[test]
    fn chunk_id_from_coords() {
        let coordinate_x: u32 = 1;
        let coordinate_y: u32 = 1;

        let expected_identifier: u32 = 4;
        let actual_identifier: u32 = Chunks::chunk_id_from_coords(coordinate_x, coordinate_y);

        assert_eq!(actual_identifier, expected_identifier);
    }

    #[test]
    fn chunk_coords() {
        let chunk_size: f32 = 16.0;
        let chunks_repository: Chunks = Chunks::new(chunk_size);
        let entity_position: Vec2 = Vec2::new(33.0, 15.0);

        let expected_coordinates: (u32, u32) = (2, 0);
        let actual_coordinates: (u32, u32) = chunks_repository.chunk_coords(&entity_position);

        assert_eq!(actual_coordinates, expected_coordinates);
    }
}
