use crate::chunk::{Chunk, CHUNK_AREA, CHUNK_WIDTH};
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct ChunkManager {
    chunks: HashMap<IVec2, Chunk>,
}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunks: HashMap::<IVec2, Chunk>::new(),
        }
    }

    pub fn draw(&self) {
        for chunk in self.chunks.values() {
            chunk.draw();
        }
    }

    pub fn get_block(&self, block_position: IVec2) -> usize {
        let chunk_position = get_chunk_position(block_position);
        let Some(chunk) = &self.chunks.get(&chunk_position) else {
            return 0;
        };
        let relative_coords = get_relative_position(block_position, chunk_position);
        return chunk.blocks[get_index_from_position(relative_coords)];
    }

    pub async fn create_chunk(&mut self, chunk_position: IVec2, blocks: [usize; CHUNK_AREA]) {
        self.chunks
            .insert(chunk_position, Chunk::new(chunk_position, blocks).await);
    }

    pub fn delete_chunk(&mut self, chunk_position: IVec2) {
        self.chunks.remove(&chunk_position);
    }
}

pub fn get_chunk_position(block_position: IVec2) -> IVec2 {
    return IVec2::new(
        (block_position.x as f32 / CHUNK_WIDTH as f32).floor() as i32,
        (block_position.y as f32 / CHUNK_WIDTH as f32).floor() as i32,
    );
}

pub fn get_relative_position(global_position: IVec2, chunk_position: IVec2) -> UVec2 {
    return UVec2::new(
        (global_position.x as f32 - (chunk_position.x as f32 * CHUNK_WIDTH as f32)) as u32,
        (global_position.y as f32 - (chunk_position.y as f32 * CHUNK_WIDTH as f32)) as u32,
    );
}

pub fn get_index_from_position(position: UVec2) -> usize {
    return position.x as usize + (position.y as usize * CHUNK_WIDTH);
}
