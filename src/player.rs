use macroquad::prelude::*;

use crate::{aabb::Aabb, chunk::TILE_SIZE, chunk_manager::ChunkManager};

pub struct Player {
    pub velocity: Vec2,
    pub aabb: Aabb,
}

impl Player {
    pub fn new(position: Vec2) -> Player {
        Player {
            velocity: Vec2::ZERO,
            aabb: Aabb {
                position,
                half_size: Vec2::splat((TILE_SIZE as f32 - 8.0) / 2.0),
            },
        }
    }

    pub fn input(&mut self) {
        let vel = 5.0 * TILE_SIZE as f32;

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.velocity.x = 1.0;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.velocity.x = -1.0;
        } else {
            self.velocity.x = 0.0;
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.velocity.y = 1.0;
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.velocity.y = -1.0;
        } else {
            self.velocity.y = 0.0;
        }

        self.velocity = self.velocity.normalize() * vel;
    }

    pub fn update(&mut self, chunk_manager: &ChunkManager) {
        let bottom_left = self.aabb.position - self.aabb.half_size;
        let bottom_right = self.aabb.position + vec2(self.aabb.half_size.x, -self.aabb.half_size.y);
        let top_right = self.aabb.position + self.aabb.half_size;
        let top_left = self.aabb.position + vec2(-self.aabb.half_size.x, self.aabb.half_size.y);

        let do_the_col =  |is_x: bool, velocity: &Vec2, first: &Vec2, second: &Vec2, add: bool| -> Option<f32> {
            let next_first = if is_x { first.x + (velocity.x * get_frame_time()) } else { first.y + (velocity.y * get_frame_time()) };
            let next_second = if is_x { second.x + (velocity.x * get_frame_time()) } else { second.y + (velocity.y * get_frame_time()) };

            let block_next_first = (next_first / TILE_SIZE as f32).floor();
            let block_next_second = (next_second / TILE_SIZE as f32).floor();

            if is_x {
                if chunk_manager.get_block(ivec2(block_next_first as i32, (first.y / TILE_SIZE as f32).floor() as i32)) {
                    let gap = ((block_next_first * TILE_SIZE as f32) + (TILE_SIZE as f32 * add as i32 as f32)) - first.x;
                    return Some(gap);
                } else if chunk_manager.get_block(ivec2(block_next_second as i32, (second.y / TILE_SIZE as f32).floor() as i32)) {
                    let gap = ((block_next_second * TILE_SIZE as f32) + (TILE_SIZE as f32 * add as i32 as f32)) - second.x;
                    return Some(gap);
                } else {
                    return None;
                }
            } else {
                if chunk_manager.get_block(ivec2((first.x / TILE_SIZE as f32).floor() as i32, block_next_first as i32)) {
                    let gap = ((block_next_first * TILE_SIZE as f32) + (TILE_SIZE as f32 * add as i32 as f32)) - first.y;
                    return Some(gap);
                } else if chunk_manager.get_block(ivec2((second.x / TILE_SIZE as f32).floor() as i32, block_next_second as i32)) {
                    let gap = ((block_next_second * TILE_SIZE as f32) + (TILE_SIZE as f32 * add as i32 as f32)) - second.y;
                    return Some(gap);
                } else {
                    return None;
                }
            }
        };

        if self.velocity.x > 0.0 {
            if let Some(gap) = do_the_col(true, &self.velocity, &bottom_right, &top_right, false) {
                self.velocity.x = 0.0;
                self.aabb.position.x += gap - 0.0001;
            } else {
                self.aabb.position.x += self.velocity.x * get_frame_time();
            }
        } else if self.velocity.x < 0.0 {
            if let Some(gap) = do_the_col(true, &self.velocity, &bottom_left, &top_left, true) {
                self.velocity.x = 0.0;
                self.aabb.position.x += gap;
            } else {
                self.aabb.position.x += self.velocity.x * get_frame_time();
            }
        }

        if self.velocity.y > 0.0 {
            if let Some(gap) = do_the_col(false, &self.velocity, &top_left, &top_right, false) {
                self.velocity.y = 0.0;
                self.aabb.position.y += gap - 0.0001;
            } else {
                self.aabb.position.y += self.velocity.y * get_frame_time();
            }
        } else if self.velocity.y < 0.0 {
            if let Some(gap) = do_the_col(false, &self.velocity, &bottom_left, &bottom_right, true) {
                self.velocity.y = 0.0;
                self.aabb.position.y += gap;
            } else {
                self.aabb.position.y += self.velocity.y * get_frame_time();
            }
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.aabb.position.x - self.aabb.half_size.x,
            self.aabb.position.y - self.aabb.half_size.y,
            self.aabb.half_size.x + self.aabb.half_size.x,
            self.aabb.half_size.y + self.aabb.half_size.x,
            RED,
        );
    }

    pub fn get_position(&self) -> Vec2 {
        self.aabb.position
    }
}
