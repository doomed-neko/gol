pub const FILL_CHANCE: f64 = 0.1;
pub const WINDOW_WIDTH: u32 = 720 * 2;
pub const WINDOW_HEIGHT: u32 = 480 * 2;
pub const TILE_SIZE: u32 = 20;
pub const ROWS: usize = (WINDOW_WIDTH / TILE_SIZE) as usize;
pub const COLS: usize = (WINDOW_HEIGHT / TILE_SIZE) as usize;
#[derive(Debug, Clone)]
pub struct Game {
    pub grid: Vec<bool>,
}

impl Game {
    pub fn new(grid: Vec<bool>) -> Self {
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

    pub fn next_gen(&mut self) {
        let new_gen: Vec<bool> = (0..(ROWS * COLS))
            .into_iter()
            .map(|x| self.next_cell_state(x))
            .collect();
        self.grid = new_gen;
    }
}
