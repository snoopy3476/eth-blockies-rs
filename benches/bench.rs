#[macro_use]
extern crate bencher;

fn bench_eth_blockies_data(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies::eth_blockies_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    });
}

fn bench_indexed_data_to_png(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies::eth_blockies_png_data(
            "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc",
            (128, 128),
        );
    });
}

benchmark_group!(benches, bench_eth_blockies_data, bench_indexed_data_to_png);
benchmark_main!(benches);
