#[macro_use]
extern crate bencher;
use eth_blockies::*;

const SEED_ADDR: &str = "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc";

benchmark_group!(
    benches,
    bench_eth_blockies_data,
    bench_eth_blockies_data_flatten,
    bench_eth_blockies_data_mapped,
    bench_eth_blockies_indexed_data,
    bench_eth_blockies_indexed_data_scale_008,
    bench_eth_blockies_indexed_data_scale_100,
    bench_eth_blockies_indexed_data_scale_128,
    bench_eth_blockies_indexed_data_scale_512,
    bench_indexed_data_to_png_008,
    bench_indexed_data_to_png_100,
    bench_indexed_data_to_png_128,
    bench_indexed_data_to_png_512,
    bench_indexed_data_to_png_base64_008,
    bench_indexed_data_to_png_base64_100,
    bench_indexed_data_to_png_base64_128,
    bench_indexed_data_to_png_base64_512,
    bench_indexed_data_to_png_compressed_008,
    bench_indexed_data_to_png_compressed_100,
    bench_indexed_data_to_png_compressed_128,
    bench_indexed_data_to_png_compressed_512,
    bench_indexed_data_to_ansiseq_ascii_08,
    bench_indexed_data_to_ansiseq_ascii_31,
    bench_indexed_data_to_ansiseq_ascii_32,
    bench_indexed_data_to_ansiseq_utf8_08,
    bench_indexed_data_to_ansiseq_utf8_31,
    bench_indexed_data_to_ansiseq_utf8_32
);
benchmark_main!(benches);

fn bench_eth_blockies_data(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_data(SEED_ADDR);
    });
}

fn bench_eth_blockies_data_flatten(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_data(SEED_ADDR).flatten();
    });
}

fn bench_eth_blockies_data_mapped(bench: &mut bencher::Bencher) {
    fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
        (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    }

    bench.iter(|| {
        eth_blockies_data_mapped(SEED_ADDR, rgb_to_grayscale);
    });
}

fn bench_eth_blockies_indexed_data(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        eth_blockies_indexed_data(SEED_ADDR);
    });
}

// bench_eth_blockies_indexed_data_scale
fn bench_eth_blockies_indexed_data_scale(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        let (_palette, data) = eth_blockies_indexed_data(SEED_ADDR);
        data.scale(dimension).concat();
    });
}
fn bench_eth_blockies_indexed_data_scale_008(bench: &mut bencher::Bencher) {
    bench_eth_blockies_indexed_data_scale(bench, (8, 8));
}
fn bench_eth_blockies_indexed_data_scale_100(bench: &mut bencher::Bencher) {
    bench_eth_blockies_indexed_data_scale(bench, (100, 100));
}
fn bench_eth_blockies_indexed_data_scale_128(bench: &mut bencher::Bencher) {
    bench_eth_blockies_indexed_data_scale(bench, (128, 128));
}
fn bench_eth_blockies_indexed_data_scale_512(bench: &mut bencher::Bencher) {
    bench_eth_blockies_indexed_data_scale(bench, (512, 512));
}

// bench_indexed_data_to_png_uncompressed
fn bench_indexed_data_to_png(
    bench: &mut bencher::Bencher,
    dimension: (usize, usize),
    is_compressed: bool,
) {
    bench.iter(|| {
        eth_blockies_png_data(SEED_ADDR, dimension, is_compressed);
    });
}
fn bench_indexed_data_to_png_008(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (8, 8), false);
}
fn bench_indexed_data_to_png_100(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (100, 100), false);
}
fn bench_indexed_data_to_png_128(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (128, 128), false);
}
fn bench_indexed_data_to_png_512(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (512, 512), false);
}
fn bench_indexed_data_to_png_compressed_008(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (8, 8), true);
}
fn bench_indexed_data_to_png_compressed_100(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (100, 100), true);
}
fn bench_indexed_data_to_png_compressed_128(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (128, 128), true);
}
fn bench_indexed_data_to_png_compressed_512(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (512, 512), true);
}

// bench_indexed_data_to_png_base64
fn bench_indexed_data_to_png_base64(
    bench: &mut bencher::Bencher,
    dimension: (usize, usize),
    is_compressed: bool,
) {
    bench.iter(|| {
        eth_blockies_png_data_base64(SEED_ADDR, dimension, is_compressed, false);
    });
}
fn bench_indexed_data_to_png_base64_008(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (8, 8), false);
}
fn bench_indexed_data_to_png_base64_100(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (100, 100), false);
}
fn bench_indexed_data_to_png_base64_128(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (128, 128), false);
}
fn bench_indexed_data_to_png_base64_512(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (512, 512), false);
}

fn bench_indexed_data_to_ansiseq_ascii(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        eth_blockies_ansiseq_data(SEED_ADDR, dimension, false);
    });
}
fn bench_indexed_data_to_ansiseq_ascii_08(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_ansiseq_ascii(bench, (8, 8));
}
fn bench_indexed_data_to_ansiseq_ascii_31(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_ansiseq_ascii(bench, (31, 31));
}
fn bench_indexed_data_to_ansiseq_ascii_32(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_ansiseq_ascii(bench, (32, 32));
}

fn bench_indexed_data_to_ansiseq_utf8(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        eth_blockies_ansiseq_data(SEED_ADDR, dimension, true);
    });
}
fn bench_indexed_data_to_ansiseq_utf8_08(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_ansiseq_utf8(bench, (8, 8));
}
fn bench_indexed_data_to_ansiseq_utf8_31(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_ansiseq_utf8(bench, (31, 31));
}
fn bench_indexed_data_to_ansiseq_utf8_32(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_ansiseq_utf8(bench, (32, 32));
}
