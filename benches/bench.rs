use criterion::{Criterion, criterion_group, criterion_main};
use gol::Game;
use rand::random_bool;

fn criterion_benchmark(c: &mut Criterion) {
    let mut buf = vec![false; 10000];
    buf.fill_with(|| random_bool(0.05));
    let mut game = Game::new(buf, 100, 100);
    c.bench_function("game", |b| b.iter(|| game.next_gen()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
