#[macro_use]
extern crate bencher;
extern crate difference_of_squares as squares;

use bencher::Bencher;

const ITERATIONS: usize = 100_000;

fn bench_square_fold(bench: &mut Bencher) {
    bench.iter(|| squares::fold::sum_of_squares(ITERATIONS))
}

fn bench_square_map(bench: &mut Bencher) {
    bench.iter(|| squares::map::sum_of_squares(ITERATIONS))
}

fn bench_square_for(bench: &mut Bencher) {
    bench.iter(|| squares::for_loop::sum_of_squares(ITERATIONS))
}

fn bench_sum_for(bench: &mut Bencher) {
    bench.iter(|| squares::for_loop::square_of_sum(ITERATIONS))
}

fn bench_sum_map(bench: &mut Bencher) {
    bench.iter(|| squares::map::square_of_sum(ITERATIONS))
}

benchmark_group!(
    benches,
    bench_square_for,
    bench_square_fold,
    bench_square_map,
    bench_sum_for,
    bench_sum_map
);
benchmark_main!(benches);
