use criterion::{Criterion, criterion_group, criterion_main};
use gol::{COLS, Game, ROWS};
use rand::random_bool;

fn bench_10k(c: &mut Criterion) {
    let (cols, rows) = (100, 100);
    let mut buf = vec![false; rows * cols];
    buf.fill_with(|| random_bool(0.05));
    let mut game = Game::new(buf, cols, rows);
    c.bench_function("10k", |b| b.iter(|| game.next_gen()));
}

fn bench_cols_by_rows(c: &mut Criterion) {
    let mut buf = vec![false; COLS * ROWS];
    buf.fill_with(|| random_bool(0.05));
    let mut game = Game::new(buf, COLS, ROWS);
    c.bench_function("ROWS*COLS", |b| b.iter(|| game.next_gen()));
}

criterion_group!(benches, bench_10k, bench_cols_by_rows);
criterion_main!(benches);
