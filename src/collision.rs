use macroquad::prelude::*;

use crate::{chunk::{ChunkLayer, TILE_SIZE}, chunk_manager::ChunkManager};

pub struct RectangleCorners {
    pub bottom_left: Vec2,
    pub bottom_right: Vec2,
    pub top_left: Vec2,
    pub top_right: Vec2,
}

/* Rectangle defined by its dimensions
 * and the position of its top left corner. */
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub pos_bottom_left: Vec2,
    pub size: Vec2,
}
impl Rectangle {
    pub fn draw(&self, color: Color) {
        let p = self.pos_bottom_left;
        let d = self.size;
        draw_rectangle(p.x, p.y, d.x, d.y, color)
    }
    pub fn draw_lines(&self, thickness: f32, color: Color) {
        let p = self.pos_bottom_left;
        let d = self.size;
        draw_rectangle_lines(p.x, p.y, d.x, d.y, thickness, color)
    }
    pub fn draw_ex(&self, params: DrawRectangleParams) {
        let p = self.pos_bottom_left;
        let d = self.size;
        draw_rectangle_ex(p.x, p.y, d.x, d.y, params);
    }
    pub fn corners(&self) -> RectangleCorners {
        RectangleCorners {
            bottom_left: self.pos_bottom_left,
            top_left: self.pos_bottom_left + self.size.with_x(0.0),
            bottom_right: self.pos_bottom_left + self.size.with_y(0.0),
            top_right: self.pos_bottom_left + self.size,
        }
    }
    fn intersects(&self, other: Rectangle) -> bool {
        let corners = |r: Rectangle| (r.pos_bottom_left, r.pos_bottom_left + r.size);
        let (a_bottom_left, a_top_right) = corners(*self);
        let (b_bottom_left, b_top_right) = corners(other);
        // https://silentmatt.com/rectangle-intersection/
        a_bottom_left.cmple(b_top_right).all() && a_top_right.cmpge(b_bottom_left).all()
    }
    fn center(&self) -> Vec2 {
        self.pos_bottom_left + 0.5 * self.size
    }
    pub fn draw_center_rotated(&self, color: Color, rotation: f32) {
        let c = self.center();
        let d = self.size;
        draw_rectangle_ex(
            c.x,
            c.y,
            d.x,
            d.y,
            DrawRectangleParams {
                offset: Vec2::splat(0.5),
                color,
                rotation,
            },
        );
    }
}

// The function arguments are in world units, but the function itself only works with block units.
// The returned position is also in block units.
// If you wish to draw the intersection point or use it in any other context, you shall multiply it by "TILE_SIZE as f32".
// 
// There may be a way of making this raycasting algorithm to use world units instead,
// however I don't understand how this algorithm works so I decided to not touch it.
// - pvini07BR
pub fn cast_ray_blocks(chunk_manager: &ChunkManager, layer: ChunkLayer, mut ray_origin: Vec2, mut ray_end: Vec2, maximum_ray_distance: f32) -> Option<Vec2> {
    ray_origin /= TILE_SIZE as f32;
    ray_end /= TILE_SIZE as f32;

    let raydir = (ray_end - ray_origin).normalize();

    let step_size = vec2(
        f32::sqrt(1.0 + f32::powf(raydir.y / raydir.x, 2.0)),
        f32::sqrt(1.0 + f32::powf(raydir.x / raydir.y, 2.0))
    );

    let mut block_pos: IVec2 = ray_origin.floor().as_ivec2();
    let mut ray_length = Vec2::ZERO;
    let mut vstep = IVec2::ZERO;

    if raydir.x < 0.0 {
        vstep.x = -1;
        ray_length.x = (ray_origin.x - block_pos.x as f32) * step_size.x;
    } else {
        vstep.x = 1;
        ray_length.x = ((block_pos.x + 1) as f32 - ray_origin.x) * step_size.x;
    }

    if raydir.y < 0.0 {
        vstep.y = -1;
        ray_length.y = (ray_origin.y - block_pos.y as f32) * step_size.y;
    } else {
        vstep.y = 1;
        ray_length.y = ((block_pos.y + 1) as f32 - ray_origin.y) * step_size.y;
    }

    let mut block_found: bool = false;
    let mut distance: f32 = 0.0;

    while !block_found && distance < maximum_ray_distance {
        if ray_length.x < ray_length.y {
            block_pos.x += vstep.x;
            distance = ray_length.x;
            ray_length.x += step_size.x;
        } else {
            block_pos.y += vstep.y;
            distance = ray_length.y;
            ray_length.y += step_size.y;
        }

        block_found = chunk_manager.get_block(block_pos, layer) > 0;
    }

    if block_found {
        return Some(ray_origin + raydir * distance);
    } else {
        return None; 
    }
}

pub mod bounding_box {
    use super::*;
    #[derive(Debug)]
    pub struct AxisAlignedRectangle {
        /* Global position of the center of the bounding box */
        pub center_pos: Vec2,
        pub size: Vec2,
    }

    impl AxisAlignedRectangle {
        pub fn as_drectangle(&self) -> Rectangle {
            Rectangle {
                pos_bottom_left: self.center_pos - self.size * 0.5,
                size: self.size,
            }
        }
        pub fn intersects(&self, other: &AxisAlignedRectangle) -> bool {
            self.as_drectangle().intersects(other.as_drectangle())
        }
        #[allow(dead_code)]
        pub fn debug_draw(&self, color: Color) {
            self.as_drectangle().draw(Color { a: 0.5, ..color });
            self.as_drectangle().draw_lines(2.0, color)
        }
    }
}
