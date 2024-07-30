use macroquad::prelude::*;

use crate::{aabb::Aabb, chunk::TILE_SIZE, chunk_manager::ChunkManager};

pub struct Player {
    pub velocity: Vec2,
    pub noclip: bool,
    pub aabb: Aabb,
}

impl Player {
    pub fn new(position: Vec2) -> Player {
        Player {
            velocity: Vec2::ZERO,
            noclip: false,
            aabb: Aabb {
                position,
                half_size: Vec2::splat((TILE_SIZE as f32 - 8.0) / 2.0),
            },
        }
    }

    pub fn input(&mut self) {
        let mut speed = 5.0 * TILE_SIZE as f32;
        if is_key_down(KeyCode::LeftControl) {
            speed *= 4.0;
        }
        if is_key_down(KeyCode::LeftShift) {
            speed /= 4.0;
        }

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

        if is_key_pressed(KeyCode::F) {
            self.noclip = !self.noclip;
        }

        self.velocity = self.velocity.normalize_or_zero() * speed;
    }

    pub fn update(&mut self, chunk_manager: &ChunkManager) {
        if self.noclip {
            self.aabb.position += self.velocity * get_frame_time();
            return;
        }

        let bottom_left = self.aabb.position - self.aabb.half_size;
        let bottom_right = self.aabb.position + vec2(self.aabb.half_size.x, -self.aabb.half_size.y);
        let top_right = self.aabb.position + self.aabb.half_size;
        let top_left = self.aabb.position + vec2(-self.aabb.half_size.x, self.aabb.half_size.y);

        let get_corner_overlap = |corner: Vec2, add_x: bool, add_y: bool, velocity: Vec2| -> Option<(Vec2, Vec2)> {
            let next_frame = corner + (velocity * get_frame_time());
            if chunk_manager.get_block((next_frame / TILE_SIZE as f32).floor().as_ivec2()) > 0 {
                let to_block = (next_frame / TILE_SIZE as f32).floor() * TILE_SIZE as f32;
                let added = to_block + vec2(TILE_SIZE as f32 * add_x as i32 as f32, TILE_SIZE as f32 * add_y as i32 as f32);
                return Some((Vec2::abs(next_frame - added), added));
            } else {
                return None;
            }
        };

        let mut solve_collision = |corner: Vec2, add_x: bool, add_y: bool| {
            if let Some((overlap, collided_block_pos)) = get_corner_overlap(corner, add_x, add_y, self.velocity) {
                //println!("{}", collided_block_pos);
                let min = overlap.x.min(overlap.y);
                if min == overlap.x {
                    self.velocity.x = 0.0;
                    self.aabb.position.x -= corner.x - collided_block_pos.x;
                } else if min == overlap.y {
                    self.velocity.y = 0.0;
                    self.aabb.position.y -= corner.y - collided_block_pos.y;
                }
            }
        };

        solve_collision(bottom_left, true, true);
        solve_collision(bottom_right, false, true);
        solve_collision(top_right, false, false);
        solve_collision(top_left, true, false);

        self.aabb.position += self.velocity * get_frame_time();
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
