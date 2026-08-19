use std::env;

use gol::{COLS, FILL_CHANCE, Game, ROWS, TILE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use rand::random_bool;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Rect};

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
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust+sdl2 Game Of Life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::GREEN);
        for row in 0..ROWS {
            for col in 0..COLS {
                let idx = row * COLS + col;
                if game.grid[idx] {
                    canvas
                        .fill_rect(Rect::new(
                            row as i32 * TILE_SIZE as i32,
                            col as i32 * TILE_SIZE as i32,
                            TILE_SIZE,
                            TILE_SIZE,
                        ))
                        .ok();
                }
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    game.next_gen();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    game.grid.fill_with(|| random_bool(FILL_CHANCE));
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    mut x,
                    mut y,
                    ..
                } => {
                    x /= TILE_SIZE as i32;
                    y /= TILE_SIZE as i32;
                    let index = game.index_from_cords(y, x);
                    game.grid[index] = !game.grid[index];
                }
                _ => {}
            }
        }
        canvas.present();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
    }
}
