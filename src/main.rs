mod aabb;
mod chunk;
mod chunk_manager;
mod player;

use aabb::Aabb;
use chunk::TILE_SIZE;
use chunk_manager::{get_chunk_position, ChunkManager};
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
    let mut mouse_pos: Option<Vec2> = None;
    let mut block_mouse_pos: Option<IVec2> = None;

    let mut debug_f3: bool = false;

    loop {
        player.input();
        player.update(&chunk_manager);

        if is_key_pressed(KeyCode::F3) {
            debug_f3 = !debug_f3;
        }

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

        // Only get position when mouse moves inside the window
        if mouse_delta_position().length() > 0.0 {
            mouse_pos = Some(vec2(mouse_position().0, mouse_position().1));
        }

        let screen_top_left =
            camera.screen_to_world(vec2(screen_width() / 2.0, screen_height() / 2.0));
        let screen_bottom_right = camera.screen_to_world(vec2(screen_width(), screen_height()));
        let screen_aabb = Aabb::new(
            screen_top_left,
            (screen_bottom_right - screen_top_left).abs(),
        );

        chunk_manager.load_chunks_on_screen(&screen_aabb).await;

        if let Some(pos) = mouse_pos {
            let world_pos = camera.screen_to_world(pos);
            let block_pos = (world_pos / TILE_SIZE as f32).floor().as_ivec2();

            if is_mouse_button_pressed(MouseButton::Left) {
                chunk_manager.set_block(block_pos, 0);
            }
            if is_mouse_button_pressed(MouseButton::Right) {
                chunk_manager.set_block(block_pos, 1);
            }

            block_mouse_pos = Some(block_pos);
        }

        clear_background(Color::from_hex(0x628fd9));

        set_camera(&camera);

        chunk_manager.draw(&screen_aabb, debug_f3);
        player.draw();

        if let Some(pos) = block_mouse_pos {
            draw_rectangle(
                pos.x as f32 * TILE_SIZE as f32,
                pos.y as f32 * TILE_SIZE as f32,
                TILE_SIZE as f32,
                TILE_SIZE as f32,
                Color::new(1.0, 1.0, 1.0, 0.5),
            );
        }
        //screen_aabb.debug_draw(BLUE);

        set_default_camera();
        if !debug_f3 {
            draw_text("Press F3 for debug", 8.0, 24.0, 32.0, WHITE);
        } else {
            let mut strings = vec![
                format!("FPS: {}", get_fps()),
                format!("Position: {}", player.aabb.position),
                format!("Zoom: {}x", zoom),
                "\n".to_string(),
                format!("Block position: {}", (player.aabb.position / TILE_SIZE as f32).floor().as_ivec2()),
                format!("Chunk position: {}", get_chunk_position((player.aabb.position / TILE_SIZE as f32).floor().as_ivec2())),
                "\n".to_string(),
                format!(
                    "Loaded Chunks: {}",
                    chunk_manager.get_loaded_chunks_amount()
                ),
            ];
            
            if let Some(pos) = block_mouse_pos {
                strings.insert(2, format!("Cursor position: {}", pos));
            }

            let mut cur_y = 0.0;
            for s in strings {
                let font_size = 32;
                let size = measure_text(s.as_str(), None, font_size, 1.0);
                cur_y += font_size as f32;
                if s != "\n" {
                    let margin = 5.0;
                    let pos = vec2(margin, cur_y);

                    draw_rectangle(pos.x - margin, (pos.y - size.offset_y) - margin, size.width + margin, size.height + (margin*2.0), color_u8!(0.0, 0.0, 0.0, 128.0));
                    draw_text(s.as_str(), pos.x, pos.y, font_size as f32, WHITE);
                }
            }
        }

        next_frame().await;
    }
}
