use std::env;

use gol::{CELL_COUNT, COLS, FILL_CHANCE, Game, ROWS, TILE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use raylib::prelude::*;

fn main() {
    let mut grid: Vec<bool> = vec![false; ROWS * COLS];
    grid.fill_with(|| rand::random_bool(FILL_CHANCE));
    let mut game = Game::new(grid, COLS, ROWS);

    if let Some(arg) = env::args().nth(1)
        && arg == "nogui"
    {
        for _ in 0..100 {
            game.next_gen();
        }
        return;
    }
    println!("SIMULATING {CELL_COUNT} cells");
    game.next_gen();

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("rust+raylib Game Of Life")
        .build();
    let mut fps = 10;
    rl.set_target_fps(fps);
    while !rl.window_should_close() {
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
            game.grid.fill_with(|| rand::random_bool(FILL_CHANCE));
            game.generation = 0;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let x = mouse_pos.x as i32 / TILE_SIZE as i32;
            let y = mouse_pos.y as i32 / TILE_SIZE as i32;
            let index = game.index_from_cords(y, x);
            game.grid[index] = true;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_pos = rl.get_mouse_position();
            let x = mouse_pos.x as i32 / TILE_SIZE as i32;
            let y = mouse_pos.y as i32 / TILE_SIZE as i32;
            let index = game.index_from_cords(y, x);
            game.grid[index] = false;
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for row in 0..ROWS {
            for col in 0..COLS {
                let idx = row * COLS + col;
                if game.grid[idx] {
                    d.draw_rectangle(
                        row as i32 * TILE_SIZE as i32,
                        col as i32 * TILE_SIZE as i32,
                        TILE_SIZE as i32,
                        TILE_SIZE as i32,
                        Color::GREEN,
                    );
                }
            }
        }
        d.draw_text(
            &format!("Generation: {}\nFPS target: {fps}", game.generation),
            10,
            900,
            25,
            Color::WHITE,
        );
    }
}
