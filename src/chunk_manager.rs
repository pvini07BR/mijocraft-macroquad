use crate::{aabb::Aabb, chunk::{Chunk, CHUNK_AREA, CHUNK_WIDTH, TILE_SIZE}};
use macroquad::prelude::*;
use std::collections::HashMap;

use ::rand::prelude::*;

pub struct ChunkManager {
    chunks: HashMap<IVec2, Chunk>,
}

impl ChunkManager {
    pub fn new() -> ChunkManager {
        ChunkManager {
            chunks: HashMap::<IVec2, Chunk>::new(),
        }
    }

    pub fn draw(&self, camera: &Camera2D) {
        // Only render chunks that are inside the screen!!!
        // ================================================

        const CHUNK_PIXEL_SIZE: f32 = CHUNK_WIDTH as f32 * TILE_SIZE as f32;

        let screen_center = camera.screen_to_world(Vec2::ZERO);
        let bottom_right_screen_corner = camera.screen_to_world(vec2(screen_width(), screen_height()));
        let screen_aabb = Aabb::new(screen_center, (bottom_right_screen_corner - screen_center).abs());

        for chunk in self.chunks.values() {
            let to_block = chunk.position * CHUNK_WIDTH as i32;
            let to_pixel = to_block * TILE_SIZE as i32;

            let chunk_aabb = Aabb::new(to_pixel.as_vec2() + Vec2::splat(CHUNK_PIXEL_SIZE/2.0), Vec2::splat(CHUNK_PIXEL_SIZE/2.0));
            
            if screen_aabb.intersects(&chunk_aabb) {
                chunk.draw();
            }
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

    pub fn get_loaded_chunks_amount(&self) -> usize {
        return self.chunks.len();
    }

    pub async fn create_chunk(&mut self, chunk_position: IVec2, blocks: [usize; CHUNK_AREA]) {
        self.chunks
            .insert(chunk_position, Chunk::new(chunk_position, blocks).await);
    }

    pub fn delete_chunk(&mut self, chunk_position: IVec2) {
        self.chunks.remove(&chunk_position);
    }

    pub async fn generate_chunk(&mut self, pos: IVec2) {
        let mut blocks = [0; CHUNK_AREA];
        for i in 0..CHUNK_AREA {
            blocks[i] = thread_rng().gen_range(0..8);
        }

        self.create_chunk(pos, blocks).await;
    }

    pub async fn load_chunks_area(&mut self, first_pos: IVec2, second_pos: IVec2) {
        for y in second_pos.y..(first_pos.y+1) {
            for x in first_pos.x..(second_pos.x+1) {
                if !self.chunks.contains_key(&ivec2(x, y)) {
                    self.generate_chunk(ivec2(x, y)).await;
                }
            }
        }
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
