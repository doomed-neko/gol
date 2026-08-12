use std::{
    hash::RandomState,
    io::{stdin, stdout, Write},
    process::{Command, ExitCode, Stdio},
    thread::sleep,
    time::Duration,
};

const ROWS: usize = 10;
const COLS: usize = 50;
struct Game {
    grid: Vec<bool>,
}

impl Game {
    fn new(grid: Vec<bool>) -> Self {
        // let mut grid: Vec<bool> = Vec::with_capacity(ROWS * COLS);
        // grid.fill(false);
        return Self { grid };
    }
    fn next_gen(&mut self) {
        let mut new_gen: Vec<bool> = vec![false; ROWS * COLS];
        new_gen.fill(false);
        for row in 0..ROWS {
            for col in 0..COLS {
                let mut neighbors = 0;
                let idx = row * COLS + col;
                // left and right
                neighbors += self.grid.get(idx + 1).unwrap_or(&false).to_owned() as i32;
                neighbors += self
                    .grid
                    .get(idx.wrapping_sub(1))
                    .unwrap_or(&false)
                    .to_owned() as i32;

                // above and below
                neighbors += self
                    .grid
                    .get(idx.wrapping_sub(COLS))
                    .unwrap_or(&false)
                    .to_owned() as i32;
                neighbors += self.grid.get(idx + COLS).unwrap_or(&false).to_owned() as i32;

                // upper diagonals
                neighbors += self
                    .grid
                    .get(idx.wrapping_sub(COLS + 1))
                    .unwrap_or(&false)
                    .to_owned() as i32;
                neighbors += self
                    .grid
                    .get(idx.wrapping_sub(COLS - 1))
                    .unwrap_or(&false)
                    .to_owned() as i32;

                // lower diagonals
                neighbors += self.grid.get(idx + COLS + 1).unwrap_or(&false).to_owned() as i32;
                neighbors += self
                    .grid
                    .get(idx.wrapping_sub(COLS - 1))
                    .unwrap_or(&false)
                    .to_owned() as i32;
                let current = self.grid[idx];
                if !current && neighbors == 3 {
                    new_gen[idx] = true;
                } else {
                    if neighbors < 2 || neighbors > 3 {
                        new_gen[idx] = false;
                    } else {
                        new_gen[idx] = true;
                    }
                }
            }
        }
        self.grid = new_gen;
    }
    fn print(&self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                let idx = row * COLS + col;
                let ch = if self.grid[idx] { "#" } else { " " };
                print!("{ch}");
            }
            println!()
        }
    }
}
fn main() -> ExitCode {
    Command::new("clear").stdout(Stdio::inherit()).output().ok();
    println!("Welcome to the game of life, please fill the grid. \nit's a {ROWS} by {COLS} grid, you will enter each row and then the simulation will start");
    println!("enter any char for a filled cell and a space for an empty cell, \neach line should be {COLS} long and should end with a newline");
    let mut grid: Vec<bool> = vec![false; ROWS * COLS];
    for row in 0..ROWS {
        let mut buf = String::with_capacity(COLS + 1);
        stdin().read_line(&mut buf).expect("read line");
        buf.pop();
        if buf.len() > COLS {
            println!("Bad column!");
            return ExitCode::FAILURE;
        }
        let mut col = 0;
        for c in buf.chars() {
            if c == ' ' {
                grid[row * COLS + col] = false;
            } else {
                grid[row * COLS + col] = true;
            }
            col += 1;
        }
    }
    let mut game = Game::new(grid);
    loop {
        Command::new("clear").stdout(Stdio::inherit()).output().ok();
        stdout().flush().ok();
        game.print();
        sleep(Duration::from_millis(300));
        game.next_gen();
    }
}
