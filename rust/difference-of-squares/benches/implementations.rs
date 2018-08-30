#[macro_use]
extern crate bencher;
extern crate difference_of_squares as squares;

use bencher::Bencher;

const ITERATIONS: usize = 100_000;

fn bench_square_fold(bench: &mut Bencher) {
    bench.iter(|| squares::sum_of_squares_fold(ITERATIONS))
}

fn bench_square_map(bench: &mut Bencher) {
    bench.iter(|| squares::sum_of_squares(ITERATIONS))
}

fn bench_square_for(bench: &mut Bencher) {
    bench.iter(|| squares::sum_of_squares_for(ITERATIONS))
}

fn bench_sum_for(bench: &mut Bencher) {
    bench.iter(|| squares::square_of_sum_for(ITERATIONS))
}

fn bench_sum_sum(bench: &mut Bencher) {
    bench.iter(|| squares::square_of_sum(ITERATIONS))
}

benchmark_group!(
    benches,
    bench_square_for,
    bench_square_fold,
    bench_square_map,
    bench_sum_for,
    bench_sum_sum
);
benchmark_main!(benches);
