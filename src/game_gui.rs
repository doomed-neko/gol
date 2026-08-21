use std::ops::Add as _;

use raylib::prelude::*;

use crate::{args::GameArgs, game::Game};

pub fn handle_keyboard(rl: &mut RaylibHandle, game: &mut Game, args: &mut GameArgs) {
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
        *game = Game::new_random(args.cols(), args.rows(), args.fill_chance);
    }
    //
    // increment cell count
    if rl.is_key_down(KeyboardKey::KEY_PERIOD) {
        args.tile_size = args
            .tile_size
            .add(1)
            .min(args.window_height.min(args.window_width));
        *game = Game::new_random(args.cols(), args.rows(), args.fill_chance);
    }

    if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
        args.fps = 10;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
        args.fps = 20;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
        args.fps = 30;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
        args.fps = 40;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
        args.fps = 50;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SIX) {
        args.fps = 60;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) {
        args.fps = 70;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) {
        args.fps = 80;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_NINE) {
        args.fps = 90;
        rl.set_target_fps(args.fps);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
        args.fps = 100;
        rl.set_target_fps(args.fps);
    }

    // FPS -1 (one at a time)
    if rl.is_key_pressed(KeyboardKey::KEY_MINUS) {
        args.fps = args.fps.saturating_sub(1).max(1);
        rl.set_target_fps(args.fps);
    }

    // FPS +1 (one at a time)
    if rl.is_key_pressed(KeyboardKey::KEY_EQUAL) {
        args.fps += 1;
        rl.set_target_fps(args.fps);
    }

    // FPS -1 (hold)
    if rl.is_key_down(KeyboardKey::KEY_LEFT_BRACKET) {
        args.fps = args.fps.saturating_sub(1).max(1);
        rl.set_target_fps(args.fps);
    }

    // FPS +1 (hold)
    if rl.is_key_down(KeyboardKey::KEY_RIGHT_BRACKET) {
        args.fps += 1;
        rl.set_target_fps(args.fps);
    }

    // FPS unlimited
    if rl.is_key_down(KeyboardKey::KEY_U) {
        args.fps = 0;
        rl.set_target_fps(args.fps);
    }

    // clear (kill all cells)
    if rl.is_key_pressed(KeyboardKey::KEY_C) {
        game.grid.fill(false);
    }

    // randomize
    if rl.is_key_down(KeyboardKey::KEY_R) {
        rl.set_target_fps(50.max(args.fps));
        game.grid.fill_with(|| rand::random_bool(args.fill_chance));
        game.generation = 0;
    }
    if rl.is_key_released(KeyboardKey::KEY_R) {
        rl.set_target_fps(args.fps);
    }
}

pub fn draw_stats(game: &Game, args: GameArgs, d: &mut RaylibDrawHandle) {
    let fps = if args.fps > 0 {
        args.fps.to_string()
    } else {
        "unlimited".to_string()
    };
    d.draw_text(
        &format!(
            "Generation: {generation}\nCell count: {cell_count}\nFPS target: {fps}",
            generation = game.generation,
            cell_count = args.cell_count(),
        ),
        (args.window_width as f64 * 0.02) as i32,
        ((args.window_height - 80) as f32 * 0.99) as i32,
        25,
        Color::WHITE,
    );
}

pub fn handle_window_resize(rl: &RaylibHandle, args: &mut GameArgs, game: &mut Game) {
    if rl.is_window_resized() {
        args.window_height = rl.get_screen_height() as u32;
        args.window_width = rl.get_screen_width() as u32;
        *game = Game::new_random(args.cols(), args.rows(), args.fill_chance);
    }
}

pub fn handle_mouse_clicks(rl: &RaylibHandle, args: GameArgs, game: &mut Game, camera: Camera2D) {
    let mouse_pos = rl.get_mouse_position();
    let mouse_pos = rl.get_screen_to_world2D(mouse_pos, camera);
    let x = mouse_pos.x as i32 / args.tile_size as i32;
    let y = mouse_pos.y as i32 / args.tile_size as i32;
    let index = game.index_from_cords(y, x);
    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        game.grid[index] = true;
    } else if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
        game.grid[index] = false;
    }
}

pub fn draw_cell(game: &mut Game, args: GameArgs, d: &mut RaylibDrawHandle, idx: usize) {
    let (col, row) = game.cords_from_index(idx);
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
