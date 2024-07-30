mod aabb;
mod chunk;
mod chunk_manager;
mod player;

use chunk::TILE_SIZE;
use chunk_manager::{get_chunk_position, ChunkManager};
use macroquad::prelude::*;

use player::Player;

#[macroquad::main("mijocraft")]
async fn main() {
    let mut chunk_manager = ChunkManager::new();

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
                zoom += 0.05;
                zoom = clamp(zoom, 0.05, 8.0);
            } else if mouse_wheel().1 < 0.0 {
                zoom -= 0.05;
                zoom = clamp(zoom, 0.05, 8.0);
            }
        }

        camera.zoom = vec2(
            ((screen_height() / screen_width()) / TILE_SIZE as f32) / 6.0,
            -((1.0 / TILE_SIZE as f32) / 6.0),
        ) * zoom;
        
        camera.target = player.get_position();

        let top_left = camera.screen_to_world(Vec2::ZERO);
        let bottom_right = camera.screen_to_world(vec2(screen_width(), screen_height()));

        
        let top_left_block = (top_left / TILE_SIZE as f32).floor().as_ivec2();
        let bottom_right_block = (bottom_right / TILE_SIZE as f32).floor().as_ivec2();
        
        let top_left_chunk = get_chunk_position(top_left_block);
        let bottom_right_chunk = get_chunk_position(bottom_right_block);

        chunk_manager.load_chunks_area(top_left_chunk, bottom_right_chunk).await;

        clear_background(LIGHTGRAY);

        set_camera(&camera);

        chunk_manager.draw(&camera);
        player.draw();

        let world_pos = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        draw_rectangle((world_pos.x / TILE_SIZE as f32).floor() * TILE_SIZE as f32, (world_pos.y / TILE_SIZE as f32).floor() * TILE_SIZE as f32, TILE_SIZE as f32, TILE_SIZE as f32, Color::new(1.0, 1.0, 1.0, 0.5));

        set_default_camera();
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0.0, 16.0, 24.0, BLACK);

        next_frame().await;
    }
}
