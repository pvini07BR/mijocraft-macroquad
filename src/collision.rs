use macroquad::prelude::*;

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
