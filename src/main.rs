mod aabb;
mod chunk;
mod chunk_manager;
mod player;

use aabb::Aabb;
use chunk::TILE_SIZE;
use chunk_manager::ChunkManager;
use macroquad::prelude::*;

use player::Player;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "mijocraft".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    };
    // Some(0) if you want to disable VSync
    // None if you want to enable VSync
    conf.platform.swap_interval = Some(0);
    return conf;
}


#[macroquad::main(window_conf)]
async fn main() {
    let tex = load_texture("assets/textures/blocks.png").await.unwrap();
    tex.set_filter(FilterMode::Nearest);
    
    let mut chunk_manager = ChunkManager::new(tex);

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

        let screen_top_left = camera.screen_to_world(vec2(screen_width()/2.0, screen_height()/2.0));
        let screen_bottom_right = camera.screen_to_world(vec2(screen_width(), screen_height()));
        let screen_aabb = Aabb::new(screen_top_left, (screen_bottom_right - screen_top_left).abs());

        chunk_manager.load_chunks_on_screen(&screen_aabb).await;

        let world_pos = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1));
        let block_pos = (world_pos / TILE_SIZE as f32).floor().as_ivec2();

        if is_mouse_button_pressed(MouseButton::Left) { chunk_manager.set_block(block_pos, 0); }
        if is_mouse_button_pressed(MouseButton::Right) { chunk_manager.set_block(block_pos, 1); }
        
        clear_background(LIGHTGRAY);
        
        set_camera(&camera);
        
        chunk_manager.draw(&screen_aabb);
        player.draw();
        
        draw_rectangle(block_pos.x as f32 * TILE_SIZE as f32, block_pos.y as f32 * TILE_SIZE as f32, TILE_SIZE as f32, TILE_SIZE as f32, Color::new(1.0, 1.0, 1.0, 0.5));
        //screen_aabb.debug_draw(BLUE);

        set_default_camera();
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0.0, 16.0, 24.0, BLACK);
        draw_text(format!("Loaded Chunks: {}", chunk_manager.get_loaded_chunks_amount()).as_str(), 0.0, 30.0, 24.0, BLACK);
        
        next_frame().await;
    }
}
