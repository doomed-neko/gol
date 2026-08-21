use criterion::{Criterion, criterion_group, criterion_main};
use gol::game::Game;

fn bench_1k(c: &mut Criterion) {
    let mut game = Game::new_random(100, 10, 0.3);
    c.bench_function("1k", |b| b.iter(|| game.next_gen()));
}

fn bench_5k(c: &mut Criterion) {
    let (cols, rows) = (100, 50);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("5k", |b| b.iter(|| game.next_gen()));
}

fn bench_10k(c: &mut Criterion) {
    let (cols, rows) = (100, 100);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("10k", |b| b.iter(|| game.next_gen()));
}

fn bench_50k(c: &mut Criterion) {
    let (cols, rows) = (100, 500);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("50", |b| b.iter(|| game.next_gen()));
}

fn bench_100k(c: &mut Criterion) {
    let (cols, rows) = (1000, 100);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("100k", |b| b.iter(|| game.next_gen()));
}

fn bench_500k(c: &mut Criterion) {
    let (cols, rows) = (1000, 500);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("500k", |b| b.iter(|| game.next_gen()));
}

fn bench_1m(c: &mut Criterion) {
    let (cols, rows) = (1000, 1000);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("1m", |b| b.iter(|| game.next_gen()));
}

fn bench_5m(c: &mut Criterion) {
    let (cols, rows) = (1000, 5000);
    let mut game = Game::new_random(cols, rows, 0.3);
    c.bench_function("5m", |b| b.iter(|| game.next_gen()));
}

criterion_group!(
    benches, bench_1k, bench_5k, bench_10k, bench_50k, bench_100k, bench_500k, bench_1m, bench_5m
);
criterion_main!(benches);
