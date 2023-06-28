#[macro_use]
extern crate bencher;
use eth_blockies::*;

#[cfg(feature = "compressed_png")]
benchmark_main!(
    group_basic_data,
    group_indexed_data,
    group_png_data,
    group_png_compressed_data,
    group_png_base64_data,
    group_ansiseq_ascii_data,
    group_ansiseq_utf8_data
);
#[cfg(not(feature = "compressed_png"))]
benchmark_main!(
    group_basic_data,
    group_indexed_data,
    group_png_data,
    group_png_base64_data,
    group_ansiseq_ascii_data,
    group_ansiseq_utf8_data
);

benchmark_group!(
    group_basic_data,
    bench_data,
    bench_data_flatten,
    bench_data_mapped
);
benchmark_group!(
    group_indexed_data,
    bench_indexed_data,
    bench_indexed_data_scale_008,
    bench_indexed_data_scale_100,
    bench_indexed_data_scale_128,
    bench_indexed_data_scale_512
);
benchmark_group!(
    group_png_data,
    bench_processed_png_data_008,
    bench_processed_png_data_100,
    bench_processed_png_data_128,
    bench_processed_png_data_512
);
benchmark_group!(
    group_png_base64_data,
    bench_processed_png_data_base64_008,
    bench_processed_png_data_base64_100,
    bench_processed_png_data_base64_128,
    bench_processed_png_data_base64_512
);
#[cfg(feature = "compressed_png")]
benchmark_group!(
    group_png_compressed_data,
    bench_processed_png_data_compressed_008,
    bench_processed_png_data_compressed_100,
    bench_processed_png_data_compressed_128,
    bench_processed_png_data_compressed_512
);
benchmark_group!(
    group_ansiseq_ascii_data,
    bench_processed_ansiseq_data_ascii_08,
    bench_processed_ansiseq_data_ascii_31,
    bench_processed_ansiseq_data_ascii_32
);
benchmark_group!(
    group_ansiseq_utf8_data,
    bench_processed_ansiseq_data_utf8_08,
    bench_processed_ansiseq_data_utf8_31,
    bench_processed_ansiseq_data_utf8_32
);

const SEED_ADDR: &str = "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc";

fn bench_data(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        EthBlockies::data(SEED_ADDR);
    });
}

fn bench_data_flatten(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        EthBlockies::data(SEED_ADDR).flatten();
    });
}

fn bench_data_mapped(bench: &mut bencher::Bencher) {
    fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
        (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    }

    bench.iter(|| {
        EthBlockies::data_mapped(SEED_ADDR, rgb_to_grayscale);
    });
}

fn bench_indexed_data(bench: &mut bencher::Bencher) {
    bench.iter(|| {
        EthBlockies::indexed_data(SEED_ADDR);
    });
}

// bench_indexed_data_scale
fn bench_indexed_data_scale(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        let (_palette, data) = EthBlockies::indexed_data(SEED_ADDR);
        data.scale(dimension).concat();
    });
}
fn bench_indexed_data_scale_008(bench: &mut bencher::Bencher) {
    bench_indexed_data_scale(bench, (8, 8));
}
fn bench_indexed_data_scale_100(bench: &mut bencher::Bencher) {
    bench_indexed_data_scale(bench, (100, 100));
}
fn bench_indexed_data_scale_128(bench: &mut bencher::Bencher) {
    bench_indexed_data_scale(bench, (128, 128));
}
fn bench_indexed_data_scale_512(bench: &mut bencher::Bencher) {
    bench_indexed_data_scale(bench, (512, 512));
}

// bench_processed_png_data
fn bench_processed_png_data(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        EthBlockies::png_data(SEED_ADDR, dimension);
    });
}
fn bench_processed_png_data_008(bench: &mut bencher::Bencher) {
    bench_processed_png_data(bench, (8, 8));
}
fn bench_processed_png_data_100(bench: &mut bencher::Bencher) {
    bench_processed_png_data(bench, (100, 100));
}
fn bench_processed_png_data_128(bench: &mut bencher::Bencher) {
    bench_processed_png_data(bench, (128, 128));
}
fn bench_processed_png_data_512(bench: &mut bencher::Bencher) {
    bench_processed_png_data(bench, (512, 512));
}

// bench_indexed_data_to_compressed_png
#[cfg(feature = "compressed_png")]
fn bench_processed_png_data_compressed(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        EthBlockies::compressed_png_data(SEED_ADDR, dimension);
    });
}
#[cfg(feature = "compressed_png")]
fn bench_processed_png_data_compressed_008(bench: &mut bencher::Bencher) {
    bench_processed_png_data_compressed(bench, (8, 8));
}
#[cfg(feature = "compressed_png")]
fn bench_processed_png_data_compressed_100(bench: &mut bencher::Bencher) {
    bench_processed_png_data_compressed(bench, (100, 100));
}
#[cfg(feature = "compressed_png")]
fn bench_processed_png_data_compressed_128(bench: &mut bencher::Bencher) {
    bench_processed_png_data_compressed(bench, (128, 128));
}
#[cfg(feature = "compressed_png")]
fn bench_processed_png_data_compressed_512(bench: &mut bencher::Bencher) {
    bench_processed_png_data_compressed(bench, (512, 512));
}

// bench_processed_png_data_base64
fn bench_processed_png_data_base64(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        EthBlockies::png_data_base64(SEED_ADDR, dimension, false);
    });
}

fn bench_processed_png_data_base64_008(bench: &mut bencher::Bencher) {
    bench_processed_png_data_base64(bench, (8, 8));
}
fn bench_processed_png_data_base64_100(bench: &mut bencher::Bencher) {
    bench_processed_png_data_base64(bench, (100, 100));
}
fn bench_processed_png_data_base64_128(bench: &mut bencher::Bencher) {
    bench_processed_png_data_base64(bench, (128, 128));
}
fn bench_processed_png_data_base64_512(bench: &mut bencher::Bencher) {
    bench_processed_png_data_base64(bench, (512, 512));
}

fn bench_processed_ansiseq_data_ascii(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        EthBlockies::ansiseq_data(SEED_ADDR, dimension, false);
    });
}
fn bench_processed_ansiseq_data_ascii_08(bench: &mut bencher::Bencher) {
    bench_processed_ansiseq_data_ascii(bench, (8, 8));
}
fn bench_processed_ansiseq_data_ascii_31(bench: &mut bencher::Bencher) {
    bench_processed_ansiseq_data_ascii(bench, (31, 31));
}
fn bench_processed_ansiseq_data_ascii_32(bench: &mut bencher::Bencher) {
    bench_processed_ansiseq_data_ascii(bench, (32, 32));
}

fn bench_processed_ansiseq_data_utf8(bench: &mut bencher::Bencher, dimension: (usize, usize)) {
    bench.iter(|| {
        EthBlockies::ansiseq_data(SEED_ADDR, dimension, true);
    });
}
fn bench_processed_ansiseq_data_utf8_08(bench: &mut bencher::Bencher) {
    bench_processed_ansiseq_data_utf8(bench, (8, 8));
}
fn bench_processed_ansiseq_data_utf8_31(bench: &mut bencher::Bencher) {
    bench_processed_ansiseq_data_utf8(bench, (31, 31));
}
fn bench_processed_ansiseq_data_utf8_32(bench: &mut bencher::Bencher) {
    bench_processed_ansiseq_data_utf8(bench, (32, 32));
}
