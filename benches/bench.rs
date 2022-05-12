#[macro_use]
extern crate bencher;
use eth_blockies::*;

benchmark_group!(
    benches,
    bench_eth_blockies_data,
    bench_eth_blockies_data_serialized,
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
    bench_indexed_data_to_png_008_external_crate,
    bench_indexed_data_to_png_100_external_crate,
    bench_indexed_data_to_png_128_external_crate,
    bench_indexed_data_to_png_512_external_crate,
    bench_indexed_data_to_png_base64_008,
    bench_indexed_data_to_png_base64_100,
    bench_indexed_data_to_png_base64_128,
    bench_indexed_data_to_png_base64_512,
);
benchmark_main!(benches);

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

// bench_eth_blockies_indexed_data_scale
fn bench_eth_blockies_indexed_data_scale(bench: &mut bencher::Bencher, dimension: (u32, u32)) {
    bench.iter(|| {
        let (_palette, data) =
            eth_blockies_indexed_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
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

// bench_indexed_data_to_png
fn bench_indexed_data_to_png(bench: &mut bencher::Bencher, dimension: (u32, u32)) {
    bench.iter(|| {
        eth_blockies_png_data("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc", dimension);
    });
}
fn bench_indexed_data_to_png_008(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (8, 8));
}
fn bench_indexed_data_to_png_100(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (100, 100));
}
fn bench_indexed_data_to_png_128(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (128, 128));
}
fn bench_indexed_data_to_png_512(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png(bench, (512, 512));
}

// bench_indexed_data_to_png_external_crate
fn bench_indexed_data_to_png_external_crate(bench: &mut bencher::Bencher, dimension: (u32, u32)) {
    fn external_crate_png(palette_data: &[u8], bitmap_data: &[u8], dimension: (u32, u32)) {
        let buf_writer = std::io::Cursor::new(vec![0; 1]);
        let mut png_encoder = png::Encoder::new(buf_writer, dimension.0, dimension.1);

        png_encoder.set_color(png::ColorType::Indexed);
        png_encoder.set_depth(png::BitDepth::Eight);
        png_encoder.set_compression(png::Compression::Default);
        png_encoder.set_palette(palette_data);

        let mut png_writer = png_encoder.write_header().unwrap();
        png_writer.write_image_data(bitmap_data).unwrap();
    }

    bench.iter(|| {
        let (color_palette, palette_idx_bitmap) = eth_blockies_indexed_data_mapped(
            "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc",
            |colorclass| colorclass.into(),
        );

        external_crate_png(
            color_palette.serialize().as_slice(),
            palette_idx_bitmap.scale(dimension).concat().as_slice(),
            dimension,
        );
    });
}
fn bench_indexed_data_to_png_008_external_crate(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_external_crate(bench, (8, 8));
}
fn bench_indexed_data_to_png_100_external_crate(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_external_crate(bench, (100, 100));
}
fn bench_indexed_data_to_png_128_external_crate(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_external_crate(bench, (128, 128));
}
fn bench_indexed_data_to_png_512_external_crate(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_external_crate(bench, (512, 512));
}

// bench_indexed_data_to_png_base64
fn bench_indexed_data_to_png_base64(bench: &mut bencher::Bencher, dimension: (u32, u32)) {
    bench.iter(|| {
        eth_blockies_png_data_base64("0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc", dimension);
    });
}
fn bench_indexed_data_to_png_base64_008(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (8, 8));
}
fn bench_indexed_data_to_png_base64_100(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (100, 100));
}
fn bench_indexed_data_to_png_base64_128(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (128, 128));
}
fn bench_indexed_data_to_png_base64_512(bench: &mut bencher::Bencher) {
    bench_indexed_data_to_png_base64(bench, (512, 512));
}
