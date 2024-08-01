use macroquad::models::Vertex;
use macroquad::prelude::*;

use crate::aabb::Aabb;

pub const TILE_SIZE: usize = 32;
pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_AREA: usize = CHUNK_WIDTH * CHUNK_WIDTH;
pub const BLOCK_COUNT: usize = 8;

const CHUNK_PIXEL_SIZE: f32 = CHUNK_WIDTH as f32 * TILE_SIZE as f32;

pub struct Chunk {
    pub position: IVec2,
    pub blocks: [usize; 256],
    pub mesh: Mesh,
    pub aabb: Aabb,
}

impl Chunk {
    pub async fn new(position: IVec2, blocks: [usize; 256], texture_atlas: Texture2D) -> Chunk {
        let mut indices = [0; CHUNK_AREA * 6];
        let mut offset: usize = 0;
        for i in (0..(256 * 6)).step_by(6) {
            indices[i + 0] = 0 + offset as u16;
            indices[i + 1] = 1 + offset as u16;
            indices[i + 2] = 2 + offset as u16;

            indices[i + 3] = 2 + offset as u16;
            indices[i + 4] = 3 + offset as u16;
            indices[i + 5] = 0 + offset as u16;

            offset += 4;
        }

        let to_block = position * CHUNK_WIDTH as i32;
        let to_pixel = to_block * TILE_SIZE as i32;
        let chunk_aabb = Aabb::new(
            to_pixel.as_vec2() + Vec2::splat(CHUNK_PIXEL_SIZE / 2.0),
            Vec2::splat(CHUNK_PIXEL_SIZE / 2.0),
        );

        let mut new_chunk = Chunk {
            blocks,
            position,
            aabb: chunk_aabb,
            mesh: Mesh {
                indices: indices.to_vec(),
                vertices: vec![],
                texture: Some(texture_atlas),
            },
        };
        new_chunk.remesh();
        return new_chunk;
    }

    // This function is causing some weird high memory usage
    // that is still unknown.
    pub fn remesh(&mut self) {
        let mut vertices = [Vertex {
            position: Vec3::ZERO,
            uv: Vec2::ZERO,
            color: Color::default(),
        }; CHUNK_AREA * 4];

        for y in 0..16 {
            for x in 0..16 {
                let index: usize = x + (y * 16);
                let vert_index = index * 4;
                if self.blocks[index] > 0 {
                    let pos_template = |pos: usize, x: bool| {
                        pos as f32 * TILE_SIZE as f32 + (x as usize * TILE_SIZE) as f32
                    };
                    let p = |a: bool, b: bool| {
                        Vec3::new(
                            ((self.position.x * CHUNK_WIDTH as i32) * TILE_SIZE as i32) as f32
                                + pos_template(x, a),
                            ((self.position.y * CHUNK_WIDTH as i32) * TILE_SIZE as i32) as f32
                                + pos_template(y, b),
                            0.0,
                        )
                    };

                    let block_uv_unit = 1.0 / BLOCK_COUNT as f32;
                    // It needs to not consider the block ID number 0 because it's just air
                    let block_uv_index = block_uv_unit * (self.blocks[index] - 1) as f32;

                    vertices[vert_index] = Vertex {
                        position: p(false, false),
                        uv: Vec2::new(block_uv_index, 1.0),
                        color: WHITE,
                    };
                    vertices[vert_index + 1] = Vertex {
                        position: p(true, false),
                        uv: Vec2::new(block_uv_index + block_uv_unit, 1.0),
                        color: WHITE,
                    };
                    vertices[vert_index + 2] = Vertex {
                        position: p(true, true),
                        uv: Vec2::new(block_uv_index + block_uv_unit, 0.0),
                        color: WHITE,
                    };
                    vertices[vert_index + 3] = Vertex {
                        position: p(false, true),
                        uv: Vec2::new(block_uv_index, 0.0),
                        color: WHITE,
                    };
                }
            }
        }

        self.mesh.vertices = vertices.to_vec();
    }

    pub fn draw(&self, debug: bool) {
        draw_mesh(&self.mesh);

        if debug {
            for y in 0..CHUNK_WIDTH {
                draw_line(
                    self.position.x as f32 * CHUNK_PIXEL_SIZE,
                    (self.position.y as f32 * CHUNK_PIXEL_SIZE) + (y as f32 * TILE_SIZE as f32),
                    (self.position.x as f32 * CHUNK_PIXEL_SIZE) + CHUNK_PIXEL_SIZE,
                    (self.position.y as f32 * CHUNK_PIXEL_SIZE) + (y as f32 * TILE_SIZE as f32),
                    2.0,
                    color_u8!(255.0, 255.0, 255.0, 128.0),
                );
            }
            for x in 0..CHUNK_WIDTH {
                draw_line(
                    (self.position.x as f32 * CHUNK_PIXEL_SIZE) + (x as f32 * TILE_SIZE as f32),
                    self.position.y as f32 * CHUNK_PIXEL_SIZE,
                    (self.position.x as f32 * CHUNK_PIXEL_SIZE) + (x as f32 * TILE_SIZE as f32),
                    (self.position.y as f32 * CHUNK_PIXEL_SIZE) + CHUNK_PIXEL_SIZE,
                    2.0,
                    color_u8!(255.0, 255.0, 255.0, 128.0),
                );
            }

            draw_line(
                self.position.x as f32 * CHUNK_PIXEL_SIZE,
                self.position.y as f32 * CHUNK_PIXEL_SIZE,
                (self.position.x as f32 * CHUNK_PIXEL_SIZE) + CHUNK_PIXEL_SIZE,
                self.position.y as f32 * CHUNK_PIXEL_SIZE,
                5.0,
                BLUE,
            );
            draw_line(
                self.position.x as f32 * CHUNK_PIXEL_SIZE,
                self.position.y as f32 * CHUNK_PIXEL_SIZE,
                self.position.x as f32 * CHUNK_PIXEL_SIZE,
                (self.position.y as f32 * CHUNK_PIXEL_SIZE) + CHUNK_PIXEL_SIZE,
                5.0,
                RED,
            );
        }
    }
}
