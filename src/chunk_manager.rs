use crate::{aabb::Aabb, chunk::{Chunk, CHUNK_AREA, CHUNK_WIDTH, TILE_SIZE}};
use macroquad::prelude::*;
use noise::{Fbm, HybridMulti, NoiseFn, Perlin, Worley};
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

    pub fn draw(&self, screen_aabb: &Aabb) {
        // Only render chunks that are inside the screen!!!
        // ================================================
        for chunk in self.chunks.values() {
            if screen_aabb.intersects(&chunk.aabb) {
                chunk.draw();
            }
            //chunk.aabb.debug_draw(RED);
        }
    }

    pub fn set_block(&mut self, block_position: IVec2, block_type: usize) {
        let chunk_position = get_chunk_position(block_position);
        let Some(chunk) = self.chunks.get_mut(&chunk_position) else { return; };
        let relative_coords = get_relative_position(block_position, chunk_position);
        chunk.blocks[get_index_from_position(relative_coords)] = block_type;
        chunk.remesh();
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
        self.chunks.shrink_to_fit();
    }

    pub async fn generate_chunk(&mut self, pos: IVec2) {
        let mut blocks: [usize; 256] = [0; CHUNK_AREA];

        for y in 0..CHUNK_WIDTH {
            for x in 0..CHUNK_WIDTH {
                let index = get_index_from_position(uvec2(x as u32, y as u32));
                let global_pos = ivec2((pos.x * CHUNK_WIDTH as i32) + x as i32, (pos.y * CHUNK_WIDTH as i32) + y as i32);

                let mut noise = HybridMulti::<Perlin>::default();
                noise.frequency = 0.25;
                noise.lacunarity = 3.0;

                let s = ((noise.get([global_pos.x as f64 / CHUNK_WIDTH as f64, global_pos.x as f64 / CHUNK_WIDTH as f64, global_pos.x as f64 / CHUNK_WIDTH as f64]) + 0.5) * CHUNK_WIDTH as f64).round() as i32;

                if global_pos.y == s {
                    blocks[index] = 1;
                } else if global_pos.y < s && global_pos.y >= s-25 {
                    blocks[index] = 2;
                } else if global_pos.y < s-25 {
                    blocks[index] = 3;
                }
            }
        }

        self.create_chunk(pos, blocks).await;
    }

    pub async fn load_chunks_on_screen(&mut self, screen_aabb: &Aabb) {
        let top_left = screen_aabb.position - screen_aabb.half_size;
        let bottom_right = screen_aabb.position + screen_aabb.half_size;

        let top_left_block = (top_left / TILE_SIZE as f32).floor().as_ivec2();
        let bottom_right_block = (bottom_right / TILE_SIZE as f32).floor().as_ivec2();

        let top_left_chunk = get_chunk_position(top_left_block);
        let bottom_right_chunk = get_chunk_position(bottom_right_block);

        for y in top_left_chunk.y..=bottom_right_chunk.y {
            for x in top_left_chunk.x..=bottom_right_chunk.x {
                if !self.chunks.contains_key(&ivec2(x, y)) {
                    self.generate_chunk(ivec2(x, y)).await;
                }
            }
        }

        self.unload_unseen_chunks(screen_aabb);
    }

    pub fn unload_unseen_chunks(&mut self, screen_aabb: &Aabb) {
        let mut chunk_poses_to_delete: Vec<IVec2> = vec![];
        for (chunk_pos, chunk) in self.chunks.iter() {
            if !screen_aabb.intersects(&chunk.aabb) { chunk_poses_to_delete.push(*chunk_pos); }
        }

        for pos in chunk_poses_to_delete { self.delete_chunk(pos); }
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
