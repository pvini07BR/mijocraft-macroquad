use std::f32::consts::FRAC_PI_2;

use macroquad::prelude::*;

use crate::{
    chunk::{ChunkLayer, TILE_SIZE},
    chunk_manager::ChunkManager,
    collision::{self, bounding_box::AxisAlignedRectangle},
};

pub struct Player {
    pub velocity: Vec2,
    pub floored: bool,
    pub direction: isize,
    pub sprite_rotation: f32,
    pub noclip: bool,
    pub bounding_box: AxisAlignedRectangle,
}

impl Player {
    pub fn new(center_pos: Vec2) -> Player {
        Player {
            velocity: Vec2::ZERO,
            floored: false,
            direction: 0,
            sprite_rotation: 0.0,
            noclip: false,
            bounding_box: AxisAlignedRectangle {
                center_pos,
                size: Vec2::splat(TILE_SIZE as f32 - 8.0),
            },
        }
    }

    pub fn input(&mut self) {
        let mut speed = 10.0 * TILE_SIZE as f32;
        if is_key_down(KeyCode::LeftControl) {
            speed *= 4.0;
        }
        if is_key_down(KeyCode::LeftShift) {
            speed /= 4.0;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.velocity.x = speed;
            self.direction = 1;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.velocity.x = -speed;
            self.direction = -1;
        } else {
            self.velocity.x = 0.0;
            if self.floored {
                self.direction = 0;
            }
        }

        if self.noclip {
            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                self.velocity.y = speed;
            } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                self.velocity.y = -speed
            } else {
                self.velocity.y = 0.0;
            }

            self.velocity = self.velocity.normalize_or_zero() * speed;
        } else {
            if self.floored {
                if is_key_down(KeyCode::Space)
                    || is_key_down(KeyCode::Up)
                    || is_key_down(KeyCode::W)
                {
                    self.velocity.y += 16.0 * TILE_SIZE as f32;
                }
            }
        }

        if is_key_pressed(KeyCode::F) {
            self.noclip = !self.noclip;
            self.velocity = Vec2::ZERO;
            self.floored = false;
        }
    }

    pub fn update(&mut self, chunk_manager: &ChunkManager) {
        const GRAVITY_ACCEL: f32 = 98.07;
        const TERMINAL_GRAVITY: f32 = 530.0;

        // Apply gravity
        if !self.noclip {
            if self.velocity.y > -TERMINAL_GRAVITY {
                self.velocity.y -= (GRAVITY_ACCEL * TILE_SIZE as f32) * get_frame_time();
            } else if self.velocity.y < -TERMINAL_GRAVITY {
                self.velocity.y = -TERMINAL_GRAVITY;
            }
        }

        self.move_player(chunk_manager);

        if !self.floored {
            self.sprite_rotation -= (9.6 * get_frame_time()) * self.direction as f32;
        } else {
            self.sprite_rotation = (self.sprite_rotation / FRAC_PI_2).round() * FRAC_PI_2;
        }
    }

    // Why is the player climbing walls??? :sob:
    // =========================================

    fn move_player(&mut self, chunk_manager: &ChunkManager) {
        if self.noclip {
            self.bounding_box.center_pos += self.velocity * get_frame_time();
            return;
        }
        let collision::RectangleCorners {
            top_right,
            top_left,
            bottom_right,
            bottom_left,
        } = self.bounding_box.as_drectangle().corners();

        self.floored = false;

        let get_corner_overlap =
            |corner: Vec2, add_x: bool, add_y: bool, velocity: Vec2| -> Option<(Vec2, Vec2)> {
                let next_frame = corner + (velocity * get_frame_time());
                if chunk_manager.get_block((next_frame / TILE_SIZE as f32).floor().as_ivec2(), ChunkLayer::FOREGROUND) > 0 {
                    let to_block = (next_frame / TILE_SIZE as f32).floor() * TILE_SIZE as f32;
                    let added = to_block
                        + vec2(
                            TILE_SIZE as f32 * add_x as i32 as f32,
                            TILE_SIZE as f32 * add_y as i32 as f32,
                        );
                    return Some((Vec2::abs(next_frame - added), added));
                } else {
                    return None;
                }
            };

        let mut solve_collision = |corner: Vec2, add_x: bool, add_y: bool| {
            let Some((overlap, collided_block_pos)) =
                get_corner_overlap(corner, add_x, add_y, self.velocity)
            else {
                return;
            };
            let min = overlap.x.min(overlap.y);
            if min == overlap.x {
                self.velocity.x = 0.0;
                self.bounding_box.center_pos.x -= corner.x - collided_block_pos.x;
            } else if min == overlap.y {
                self.velocity.y = 0.0;
                self.bounding_box.center_pos.y -= corner.y - collided_block_pos.y;
                if add_y {
                    self.floored = true
                }
            }
        };

        solve_collision(bottom_left, true, true);
        solve_collision(bottom_right, false, true);
        solve_collision(top_right, false, false);
        solve_collision(top_left, true, false);

        self.bounding_box.center_pos += self.velocity * get_frame_time();
    }

    pub fn draw(&self) {
        self.bounding_box
            .as_drectangle()
            .draw_center_rotated(RED, self.sprite_rotation);
    }

    pub fn get_position(&self) -> Vec2 {
        self.bounding_box.center_pos
    }
}
