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
    rl.set_window_min_size(500, 300);
    rl.set_target_fps(args.fps);
    while !rl.window_should_close() {
        // quit
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            break;
        }
        handle_keyboard(&mut rl, &mut game, &mut args);
        handle_window_resize(&mut rl, &mut args, &mut game);
        handle_mouse_clicks(&rl, args, &mut game);

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        game.alive_cells().into_iter().for_each(|idx| {
            draw_cell(&mut game, args, &mut d, idx);
        });
        if args.show_stats {
            draw_stats(&game, args, &mut d);
        }
    }
}
