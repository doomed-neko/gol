use criterion::{Criterion, criterion_group, criterion_main};
use gol::game::Game;

fn bench_1k(c: &mut Criterion) {
    let mut game = Game::new_random(100, 10, 0.3);
    c.bench_function("ROWS*COLS", |b| b.iter(|| game.next_gen()));
}

fn bench_5k(c: &mut Criterion) {
    let mut game = Game::new_random(100, 10, 0.3);
    c.bench_function("ROWS*COLS", |b| b.iter(|| game.next_gen()));
}

fn bench_10k(c: &mut Criterion) {
    let (cols, rows) = (100, 100);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("10k", |b| b.iter(|| game.next_gen()));
}

criterion_group!(benches, bench_1k, bench_5k, bench_10k);
criterion_main!(benches);
