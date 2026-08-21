use gol::{
    args::GameArgs,
    game::Game,
    game_gui::{draw_cell, draw_stats, handle_keyboard, handle_mouse_clicks, handle_window_resize},
};
use raylib::prelude::*;

fn main() {
    let mut args = GameArgs::new();
    let mut game = Game::new_random(args.cols(), args.rows(), args.fill_chance);

    if args.no_gui > 0 {
        for _ in 0..args.no_gui {
            game.next_gen();
        }
        return;
    }
    game.next_gen();

    let mut builder = raylib::init();
    builder
        .size(args.window_width as i32, args.window_height as i32)
        .resizable()
        .title("Game Of Life");
    if args.vsync {
        builder.vsync();
    }
    let (mut rl, thread) = builder.build();
    let mut camera = Camera2D {
        target: Vector2::new(0.0, 0.0), // World coordinate the camera points at
        offset: Vector2::new(0.0, 0.0), // Screen offset
        rotation: 0.0,
        zoom: 1.0,
    };
    rl.set_window_min_size(500, 300);
    rl.set_target_fps(args.fps);
    while !rl.window_should_close() {
        // quit
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            break;
        }
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            let delta = rl.get_mouse_delta();
            camera.target.x -= delta.x / camera.zoom;
            camera.target.y -= delta.y / camera.zoom;
        }
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            let mouse_pos = rl.get_mouse_position();
            let mouse_world_pos = rl.get_screen_to_world2D(mouse_pos, camera);
            camera.offset = mouse_pos;
            camera.target = mouse_world_pos;
            let zoom_factor = 1.1f32.powf(wheel);
            camera.zoom = (camera.zoom * zoom_factor).clamp(0.1, 50.0);
        }
        handle_keyboard(&mut rl, &mut game, &mut args);
        handle_window_resize(&rl, &mut args, &mut game);
        handle_mouse_clicks(&rl, args, &mut game, camera);

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_mode2D(camera, |mut d| {
            game.alive_cells().into_iter().for_each(|idx| {
                draw_cell(&mut game, args, &mut d, idx);
            });
        });
        if args.show_stats {
            draw_stats(&game, args, &mut d);
        }
    }
}
