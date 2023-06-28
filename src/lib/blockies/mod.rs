mod blockies_base;
mod key_generator;

use crate::global_type_helper::{ColorClass, RgbPalette};
pub use blockies_base::{Blockies, BlockiesHelper};

pub fn new_blockies<const R: usize>(seed: &[u8]) -> (RgbPalette, Blockies<R, ColorClass>) {
    let mut keygen = key_generator::KeyGenerator::new(seed);

    // initialize palette
    let palette = {
        let (color, bgcolor, spotcolor) =
            { (keygen.next_rgb(), keygen.next_rgb(), keygen.next_rgb()) };
        [bgcolor, color, spotcolor]
    };

    // initialize bitmap
    let bitmap = Blockies::new(|(x, _)| match x < (R + 1) / 2 {
        true => keygen.next_colorclass(),
        false => ColorClass::BgColor, // dummy: right half is not used
    })
    .map_2d_with_ref(|src_arr, (x, y)| {
        // class
        match x < (R + 1) / 2 {
            // left half: use itself
            true => src_arr[y][x].clone(),
            // right half: use corresponding flipped left half
            false => src_arr[y][R - 1 - x].clone(),
        }
    });

    (palette, bitmap)
}
