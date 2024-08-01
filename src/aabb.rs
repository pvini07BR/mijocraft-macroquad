use macroquad::prelude::*;

pub struct Aabb {
    pub position: Vec2,
    pub half_size: Vec2,
}

impl Aabb {
    pub fn new(position: Vec2, half_size: Vec2) -> Aabb {
        return Aabb {
            position,
            half_size,
        };
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        // collision x-axis?
        let collision_x = (self.position.x + self.half_size.x)
            >= (other.position.x - other.half_size.x)
            && (other.position.x + other.half_size.x) >= (self.position.x - self.half_size.x);
        // collision y-axis?
        let collision_y = (self.position.y + self.half_size.y)
            >= (other.position.y - other.half_size.y)
            && (other.position.y + other.half_size.y) >= (self.position.y - self.half_size.y);
        // collision only if on both axes
        return collision_x && collision_y;
    }

    pub fn debug_draw(&self, color: Color) {
        draw_rectangle(
            self.position.x - self.half_size.x,
            self.position.y - self.half_size.y,
            self.half_size.x * 2.0,
            self.half_size.y * 2.0,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: 0.5,
            },
        );
        draw_rectangle_lines(
            self.position.x - self.half_size.x,
            self.position.y - self.half_size.y,
            self.half_size.x * 2.0,
            self.half_size.y * 2.0,
            2.0,
            color,
        );
    }
}
