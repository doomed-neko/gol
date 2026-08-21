#[derive(Debug, Clone)]
pub struct Game {
    pub grid: Vec<bool>,
    pub cols: usize,
    pub rows: usize,
    pub generation: usize,
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        let grid: Vec<bool> = vec![false; cols * rows];
        Self {
            grid,
            cols,
            rows,
            generation: 0,
        }
    }
    pub fn from_vec(grid: Vec<bool>, cols: usize, rows: usize) -> Self {
        Self {
            grid,
            cols,
            rows,
            generation: 0,
        }
    }
    pub fn new_random(cols: usize, rows: usize, fill_chance: f64) -> Self {
        let mut grid = vec![false; cols * rows];
        grid.fill_with(|| rand::random_bool(fill_chance));

        Self {
            grid,
            cols,
            rows,
            generation: 0,
        }
    }
    pub fn index_from_cords(&self, x: i32, y: i32) -> usize {
        let x = x.rem_euclid(self.cols as i32);
        let y = y.rem_euclid(self.rows as i32);

        y as usize * self.cols + x as usize
    }

    pub fn cords_from_index(&self, index: usize) -> (usize, usize) {
        let col = index % self.cols;
        let row = index / self.cols;

        (col, row)
    }

    pub fn next_cell_state(&self, index: usize) -> bool {
        let current = self.grid[index];
        let (col, row) = {
            let (col, row) = self.cords_from_index(index);
            (col as i32, row as i32)
        };
        let mut neighbor_count = 0;
        for x in [-1, 0, 1] {
            for y in [-1, 0, 1] {
                if x == 0 && y == 0 {
                    continue;
                }

                if row + y >= self.rows as i32
                    || row + y < 0
                    || col + x >= self.cols as i32
                    || col + x < 0
                {
                    continue;
                }
                if self.grid[self.index_from_cords(col + x, row + y)] {
                    neighbor_count += 1
                }
            }
        }
        if current && (2..=3).contains(&neighbor_count) {
            return true;
        }
        if !current && neighbor_count == 3 {
            return true;
        }
        false
    }

    pub fn next_gen(&mut self) {
        let new_gen: Vec<bool> = (0..(self.rows * self.cols))
            .into_iter()
            .map(|x| self.next_cell_state(x))
            .collect();
        self.grid = new_gen;
        self.generation += 1;
    }

    pub fn alive_cells(&self) -> Vec<usize> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(idx, &x)| if x { Some(idx) } else { None })
            .collect()
    }
}
