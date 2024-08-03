use macroquad::models::Vertex;
use macroquad::prelude::*;

use crate::collision::bounding_box::AxisAlignedRectangle;

pub const TILE_SIZE: usize = 32;
pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_AREA: usize = CHUNK_WIDTH * CHUNK_WIDTH;
pub const BLOCK_COUNT: usize = 8;

const CHUNK_PIXEL_SIZE: f32 = CHUNK_WIDTH as f32 * TILE_SIZE as f32;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ChunkLayer {
    FOREGROUND,
    BACKGROUND
}

impl std::fmt::Display for ChunkLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkLayer::FOREGROUND => write!(f, "Foreground"),
            ChunkLayer::BACKGROUND => write!(f, "Background")
        }
    }
}

impl ChunkLayer {
    pub fn flip(&self) -> ChunkLayer {
        return match self {
            ChunkLayer::FOREGROUND => ChunkLayer::BACKGROUND,
            ChunkLayer::BACKGROUND => ChunkLayer::FOREGROUND
        }
    }
}

pub struct Chunk {
    pub position: IVec2,
    pub foreground_blocks: [usize; CHUNK_AREA],
    pub background_blocks: [usize; CHUNK_AREA],
    pub foreground_mesh: Mesh,
    pub background_mesh: Mesh,
    pub aabb: AxisAlignedRectangle,
}

impl Chunk {
    pub async fn new(position: IVec2, foreground_blocks: [usize; CHUNK_AREA], background_blocks: [usize; CHUNK_AREA], texture_atlas: Texture2D) -> Chunk {
        const CHUNK_WORLD_WIDTH: f32 = CHUNK_WIDTH as f32 * TILE_SIZE as f32;
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

        let bottom_left_worldpos = position.as_vec2() * CHUNK_WORLD_WIDTH;
        let size = Vec2::splat(CHUNK_WORLD_WIDTH);
        let chunk_aabb = AxisAlignedRectangle {
            center_pos: bottom_left_worldpos + size * 0.5,
            size,
        };

        let mut new_chunk = Chunk {
            foreground_blocks,
            background_blocks,
            position,
            aabb: chunk_aabb,
            foreground_mesh: Mesh {
                indices: indices.to_vec(),
                vertices: vec![],
                texture: Some(texture_atlas.clone()),
            },
            background_mesh: Mesh {
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
        let mut foreground_vertices = [Vertex {
            position: Vec3::ZERO,
            uv: Vec2::ZERO,
            color: WHITE,
        }; CHUNK_AREA * 4];
        let mut background_vertices = [Vertex {
            position: Vec3::ZERO,
            uv: Vec2::ZERO,
            color: GRAY,
        }; CHUNK_AREA * 4];

        for y in 0..16 {
            for x in 0..16 {
                let index: usize = x + (y * 16);
                let vert_index = index * 4;
                if self.foreground_blocks[index] > 0 || self.background_blocks[index] > 0 {
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
                    let set_vertex_values = |blocks: &[usize; CHUNK_AREA], vertices: &mut [Vertex; CHUNK_AREA * 4]| {
                        // It needs to not consider the block ID number 0 because it's just air
                        let block_uv_index = block_uv_unit * (blocks[index] - 1) as f32;
    
                        vertices[vert_index].position = p(false, false);
                        vertices[vert_index].uv = Vec2::new(block_uv_index, 1.0);
    
                        vertices[vert_index + 1].position = p(true, false);
                        vertices[vert_index + 1].uv = Vec2::new(block_uv_index + block_uv_unit, 1.0);
    
                        vertices[vert_index + 2].position = p(true, true);
                        vertices[vert_index + 2].uv = Vec2::new(block_uv_index + block_uv_unit, 0.0);
    
                        vertices[vert_index + 3].position = p(false, true);
                        vertices[vert_index + 3].uv = Vec2::new(block_uv_index, 0.0);
                    };

                    if self.foreground_blocks[index] > 0 {
                        set_vertex_values(&self.foreground_blocks, &mut foreground_vertices);
                    }
                    if self.background_blocks[index] > 0 {
                        set_vertex_values(&self.background_blocks, &mut background_vertices);
                    }
                }
            }
        }

        self.foreground_mesh.vertices = foreground_vertices.to_vec();
        self.background_mesh.vertices = background_vertices.to_vec();
    }

    pub fn draw(&self, debug: bool) {
        draw_mesh(&self.background_mesh);
        draw_mesh(&self.foreground_mesh);

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
