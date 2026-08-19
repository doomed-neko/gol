use rand::random_bool;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Rect};

const FILL_CHANCE: f64 = 0.1;
const WINDOW_WIDTH: u32 = 720 * 2;
const WINDOW_HEIGHT: u32 = 480 * 2;
const TILE_SIZE: u32 = 20;
const ROWS: usize = (WINDOW_WIDTH / TILE_SIZE) as usize;
const COLS: usize = (WINDOW_HEIGHT / TILE_SIZE) as usize;
struct Game {
    grid: Vec<bool>,
}

impl Game {
    fn new(grid: Vec<bool>) -> Self {
        Self { grid }
    }

    pub fn index_from_cords(&self, x: i32, y: i32) -> usize {
        y as usize * COLS + x as usize
    }

    pub fn cords_from_index(&self, index: usize) -> (usize, usize) {
        let col = index % COLS;
        let row = index / COLS;

        (col, row)
    }

    fn next_cell_state(&self, index: usize) -> bool {
        let current = self.grid[index];
        let (col, row) = {
            let (col, row) = self.cords_from_index(index);
            (col as i32, row as i32)
        };
        let mut n = 0;
        for x in [-1, 0, 1] {
            for y in [-1, 0, 1] {
                if x == 0 && y == 0 {
                    continue;
                }

                if row + y >= ROWS as i32 || row + y < 0 || col + x >= COLS as i32 || col + x < 0 {
                    continue;
                }
                if self.grid[self.index_from_cords(col + x, row + y) as usize] {
                    n += 1
                }
            }
        }
        if current && (2..=3).contains(&n) {
            return true;
        }
        if !current && n == 3 {
            return true;
        }
        false
    }

    fn next_gen(&mut self) {
        let new_gen: Vec<bool> = (0..(ROWS * COLS))
            .into_iter()
            .map(|x| self.next_cell_state(x))
            .collect();
        self.grid = new_gen;
    }
}
fn main() {
    let mut grid: Vec<bool> = vec![false; ROWS * COLS];
    grid.fill_with(|| rand::random_bool(FILL_CHANCE));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust+sdl2 Game Of Life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut game = Game::new(grid);
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
