#[macro_use]
extern crate bencher;
use eth_blockies::*;

fn bench_eth_blockies_data(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    });
}
fn bench_eth_blockies_data_serialized(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc").serialize();
    });
}

fn bench_eth_blockies_data_mapped(bench: &mut bencher::Bencher) {
    fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
        (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    }

    bench.iter(|| {
        eth_blockies_data_mapped(
            "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc",
            rgb_to_grayscale,
        );
    });
}

fn bench_eth_blockies_indexed_data(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_indexed_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    });
}

fn bench_indexed_data_to_png_8(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_png_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc", (8, 8));
    });
}

fn bench_indexed_data_to_png_base64_8(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_png_data_base64("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc", (8, 8));
    });
}

fn bench_indexed_data_to_png_128(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_png_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc", (128, 128));
    });
}

fn bench_indexed_data_to_png_base64_128(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_png_data_base64("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc", (128, 128));
    });
}

benchmark_group!(
    benches,
    bench_eth_blockies_data,
    bench_eth_blockies_data_serialized,
    bench_eth_blockies_data_mapped,
    bench_eth_blockies_indexed_data,
    bench_indexed_data_to_png_8,
    bench_indexed_data_to_png_base64_8,
    bench_indexed_data_to_png_128,
    bench_indexed_data_to_png_base64_128
);
benchmark_main!(benches);
