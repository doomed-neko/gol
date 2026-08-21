use std::ops::Add;

use gol::{args::GameArgs, game::Game};
use raylib::prelude::*;

fn main() {
    let mut args = GameArgs::new();
    let mut grid: Vec<bool> = vec![false; args.rows() * args.cols()];
    grid.fill_with(|| rand::random_bool(args.fill_chance));
    let mut game = Game::new_random(args.cols(), args.rows(), args.fill_chance);

    if args.no_gui > 0 {
        for _ in 0..args.no_gui {
            game.next_gen();
        }
        return;
    }
    println!("SIMULATING {} cells", args.cell_count());
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
    let mut fps = 20;
    rl.set_target_fps(fps);
    while !rl.window_should_close() {
        if rl.is_window_resized() {
            args.window_height = rl.get_screen_height() as u32;
            args.window_width = rl.get_screen_width() as u32;
            game = Game::new_random(args.cols(), args.rows(), args.fill_chance);
        }
        // quit
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            break;
        }

        // tick next generation (hold)
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            game.next_gen();
        }

        // tick next generation (once)
        if rl.is_key_pressed(KeyboardKey::KEY_N) {
            game.next_gen();
        }

        // toggle stats text (once)
        if rl.is_key_pressed(KeyboardKey::KEY_I) {
            args.show_stats = !args.show_stats
        }

        // decrement cell count
        if rl.is_key_down(KeyboardKey::KEY_COMMA) {
            args.tile_size = args.tile_size.saturating_sub(1).max(1);
            game = Game::new_random(args.cols(), args.rows(), args.fill_chance);
        }
        //
        // increment cell count
        if rl.is_key_down(KeyboardKey::KEY_PERIOD) {
            args.tile_size = args
                .tile_size
                .add(1)
                .min(args.window_height.min(args.window_width));
            game = Game::new_random(args.cols(), args.rows(), args.fill_chance);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            fps = 10;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            fps = 20;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            fps = 30;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            fps = 40;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
            fps = 50;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SIX) {
            fps = 60;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) {
            fps = 70;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) {
            fps = 80;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_NINE) {
            fps = 90;
            rl.set_target_fps(fps);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
            fps = 100;
            rl.set_target_fps(fps);
        }

        // FPS -1 (one at a time)
        if rl.is_key_pressed(KeyboardKey::KEY_MINUS) {
            fps = fps.saturating_sub(1).max(1);
            rl.set_target_fps(fps);
        }

        // FPS +1 (one at a time)
        if rl.is_key_pressed(KeyboardKey::KEY_EQUAL) {
            fps += 1;
            rl.set_target_fps(fps);
        }

        // FPS -1 (hold)
        if rl.is_key_down(KeyboardKey::KEY_LEFT_BRACKET) {
            fps = fps.saturating_sub(1).max(1);
            rl.set_target_fps(fps);
        }

        // FPS +1 (hold)
        if rl.is_key_down(KeyboardKey::KEY_RIGHT_BRACKET) {
            fps += 1;
            rl.set_target_fps(fps);
        }

        // FPS unlimited
        if rl.is_key_down(KeyboardKey::KEY_U) {
            fps = u32::MAX;
            rl.set_target_fps(fps);
        }

        // clear (kill all cells)
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            game.grid.fill(false);
        }

        // randomize
        if rl.is_key_down(KeyboardKey::KEY_R) {
            rl.set_target_fps(50);
            game.grid.fill_with(|| rand::random_bool(args.fill_chance));
            game.generation = 0;
        }
        if rl.is_key_released(KeyboardKey::KEY_R) {
            rl.set_target_fps(fps);
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let x = mouse_pos.x as i32 / args.tile_size as i32;
            let y = mouse_pos.y as i32 / args.tile_size as i32;
            let index = game.index_from_cords(y, x);
            game.grid[index] = true;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_pos = rl.get_mouse_position();
            let x = mouse_pos.x as i32 / args.tile_size as i32;
            let y = mouse_pos.y as i32 / args.tile_size as i32;
            let index = game.index_from_cords(y, x);
            game.grid[index] = false;
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for row in 0..args.rows() {
            for col in 0..args.cols() {
                let idx = row * args.cols() + col;
                if game.grid[idx] {
                    d.draw_rectangle(
                        row as i32 * args.tile_size as i32,
                        col as i32 * args.tile_size as i32,
                        args.tile_size as i32,
                        args.tile_size as i32,
                        Color::GREEN,
                    );
                }
            }
        }
        if args.show_stats {
            d.draw_text(
                &format!(
                    "Generation: {generation}\nCell count: {cell_count}\nFPS target: {fps}",
                    generation = game.generation,
                    cell_count = args.cell_count()
                ),
                (args.window_width as f64 * 0.02) as i32,
                ((args.window_height - 80) as f32 * 0.99) as i32,
                // (args.window_height as f64 * 0.9) as i32,
                25,
                Color::WHITE,
            );
        }
    }
}
