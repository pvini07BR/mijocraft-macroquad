mod aabb;
mod chunk;
mod chunk_manager;
mod player;

use chunk::{CHUNK_AREA, TILE_SIZE};
use chunk_manager::ChunkManager;
use macroquad::prelude::*;

use player::Player;

#[macroquad::main("teste")]
async fn main() {
    let mut chunk_manager = ChunkManager::new();

    let mut blocks = [0; CHUNK_AREA];
    for i in 0..CHUNK_AREA {
        blocks[i] = rand::gen_range(0, 8);
    }

    chunk_manager.create_chunk(IVec2::ZERO, blocks).await;

    let mut player = Player::new(Vec2::ZERO);

    let mut camera = Camera2D {
        ..Default::default()
    };

    let mut zoom = 1.0;

    loop {
        player.input();
        player.update(&chunk_manager);

        if is_key_down(KeyCode::LeftControl) {
            if mouse_wheel().1 > 0.0 {
                zoom += 5.0 * get_frame_time();
            } else if mouse_wheel().1 < 0.0 {
                zoom -= 5.0 * get_frame_time();
            }
        }

        camera.zoom = vec2(
            ((screen_height() / screen_width()) / TILE_SIZE as f32) / 6.0,
            -((1.0 / TILE_SIZE as f32) / 6.0),
        ) * zoom;
        
        camera.target = player.get_position();

        clear_background(LIGHTGRAY);

        set_camera(&camera);

        chunk_manager.draw();
        player.draw();

        set_default_camera();
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0.0, 16.0, 24.0, BLACK);

        next_frame().await;
    }
}
