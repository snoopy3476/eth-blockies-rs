//! A lightweight library in pure Rust to get Ethereum-style blocky identicon data,
//! which can be used for generating blockies icon images, printing to terminal, etc.
//!
//! Useful when getting raw RGB data of Ethereum-style blockies, as well as complete png image files.
//!
//!
//! # Example
//!
//! * Get raw blockies data, from various seed types
//!   * To generate a *standard* Ethereum blockies (commonly seen as icons of wallet addresses on other Ethereum platforms),
//!     the input address seed **MUST** be canonicalized to `0x(hex_letters_lowercase)` using [`canonicalize_ethaddr`](type_helper::SeedString::canonicalize_ethaddr), as `addr` below.
//! ```
//! use eth_blockies::*;
//!
//! // blockies from general string seed
//! {
//!     let seed = "eth-blockies-rs";
//!
//!     // general string seed: used as it is
//!     let blockies_data_from_string = eth_blockies_data(seed);
//! }
//!
//! // blockies from general byte-array seed
//! {
//!     let seed: &[u8] = &[
//!         0x0c, 0x93, 0xa3, 0x2e, 0xe5, 0x2b, 0xf6, 0x43,
//!         0x66, 0xdb, 0xdc, 0xd7, 0xed, 0xde, 0x00, 0x78,
//!     ];
//!
//!     // general byte-array seed: used as it is
//!     let blockies_data_from_byte_arr = eth_blockies_data(seed);
//! }
//!
//! // blockies from Ethereum address seed
//! {
//!     // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!     // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
//!     let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!         .canonicalize_ethaddr();
//!
//!     assert_eq!(
//!         addr,
//!         "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
//!     );
//!
//!     // Ethereum address seed: canonicalized before use
//!     let blockies_data_from_eth_addr = eth_blockies_data(addr);
//! }
//! ```
//!
//! * Get raw blockies data, in various forms
//! ```
//! use eth_blockies::*;
//!
//! let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!     .canonicalize_ethaddr();
//!
//!
//! // get 2D array of (r, g, b)
//! // dimension: 8 x 8 (nested-array of RgbPixel)
//! {
//!     let blockies_data_rgb = eth_blockies_data(&seed);
//! }
//!
//!
//! // get 1D array of (r, g, b)
//! // dimension: 64 x 1 (array of RgbPixel)
//! {
//!     let blockies_data_rgb = eth_blockies_data(&seed).flatten();
//! }
//!
//!
//! // get 2D array of grayscale
//! {
//!     fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
//!         (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114)
//!             as u8
//!     }
//!
//!     let blockies_data_grayscale =
//!         eth_blockies_data_mapped(&seed, rgb_to_grayscale);
//! }
//!
//!
//! // get (color palette, 2D array of color indices for each pixel)
//! {
//!     let (color_palette, palette_idx_bitmap) =
//!         eth_blockies_indexed_data(&seed);
//!
//!     assert_eq!(
//!         color_palette[palette_idx_bitmap[0][0]],
//!         (132, 222, 77)
//!     );
//! }
//! ```
//!
//!
//!
//! * Write a generated blockies png data to file `test-raw.png`,
//!   on general Rust binary/library target
//! ```
//! use eth_blockies::*;
//!
//! let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!     .canonicalize_ethaddr();
//! let dimension = (128, 128); // multiples of 8 recommended
//! let compressed_output = true; // false for an uncompressed png
//! let img_png_data =
//!     eth_blockies_png_data(seed, dimension, compressed_output);
//!
//! // uncomment below to write to file
//!
//! //use std::io::Write;
//! //std::fs::File::create("test-raw.png").unwrap()
//! //    .write_all(&img_png_data);
//! ```
//!
//!
//!
//! * Generate an html `img` blockies element, on wasm target
//! ```ignore
//! // addr to blockies data uri scheme,
//! // which can be used directly in img elem 'src' or css 'url()'
//! fn eth_blockies_data_uri_scheme(addr: &str) -> String {
//!     use eth_blockies::*;
//!
//!     let addr_input = addr.canonicalize_ethaddr();
//!     let dimension = (128, 128);
//!     let compressed_output = true;
//!     let data_uri_output = true;
//!
//!     eth_blockies_png_data_base64(
//!         addr_input,
//!         dimension,
//!         compressed_output,
//!         data_uri_output
//!     )
//! }
//!
//! use web_sys::*;
//!
//! let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC";
//!
//! window()
//!     .and_then(|w| w.document())
//!     .and_then(|doc| doc.body().zip(doc.create_element("img").ok()))
//!     .and_then(|(body, img)| {
//!         // create a new img html element with generated data_uri
//!         img.set_attribute("src", &eth_blockies_data_uri_scheme(addr))
//!             // then attach to body
//!             .and_then(|_| body.append_child(&img))
//!             .ok()
//!     });
//! ```

#![no_std]

mod type_helper;
pub use type_helper::*;
mod data_encoder;
use data_encoder::{ansi_seq, indexed_png};
mod blockies_generator;
use blockies_generator::BlockiesGenerator;

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Get Ethereum-style blockies data
///
/// # Arguments
///
/// * `seed` - Input seed
///
/// # Return
///
/// * Blockies RGB data, in the form of 2D [`RgbPixel`] array
///
/// # Example
///
/// * Get RGB blockies data
/// ```
/// use eth_blockies::*;
/// let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
///     .canonicalize_ethaddr();
///
/// // 2D array data
/// let blockies_data_rgb = eth_blockies_data(seed);
///
/// const P: RgbPalette = [(38, 173, 52), (132, 222, 77), (4, 201, 40)];
///
/// assert_eq!(blockies_data_rgb, [
///         [ P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1] ],
///         [ P[1], P[0], P[0], P[2], P[2], P[0], P[0], P[1] ],
///         [ P[2], P[1], P[1], P[0], P[0], P[1], P[1], P[2] ],
///         [ P[0], P[0], P[2], P[0], P[0], P[2], P[0], P[0] ],
///         [ P[1], P[0], P[1], P[2], P[2], P[1], P[0], P[1] ],
///         [ P[1], P[2], P[1], P[2], P[2], P[1], P[2], P[1] ],
///         [ P[0], P[2], P[1], P[2], P[2], P[1], P[2], P[0] ],
///         [ P[1], P[0], P[0], P[1], P[1], P[0], P[0], P[1] ],
///     ]);
/// ```
///
#[allow(dead_code)]
pub fn eth_blockies_data<S: SeedInput>(seed: S) -> EthBlockies<RgbPixel> {
    eth_blockies_data_mapped(seed, |rgb_pixel| rgb_pixel)
}

/// Get Ethereum-style blockies data in mapped format with `map_fn`
///
/// Same with [`eth_blockies_data`],  
/// except that each [`RgbPixel`] output is mapped
/// with the function `map_fn` \(element mapping function\)
///
/// # Arguments
///
/// * `seed` - Input seed
/// * `map_fn` - Mapping function for each [`RgbPixel`] element in the returned array
///
/// # Return
///
/// * Mapped blockies data, in the form of 2D `T` array  
///   (`T`: type of each element returned by `map_fn`)
///
/// # Example
///
/// * Get grayscale blockies data
/// ```
/// use eth_blockies::*;
///
/// fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
///     (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
/// }
///
/// let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
///     .canonicalize_ethaddr();
/// let blockies_data_grayscale = eth_blockies_data_mapped(seed, rgb_to_grayscale);
///
/// assert_eq!(blockies_data_grayscale, [
///        [ 178, 178, 178, 178, 178, 178, 178, 178 ],
///        [ 178, 118, 118, 123, 123, 118, 118, 178 ],
///        [ 123, 178, 178, 118, 118, 178, 178, 123 ],
///        [ 118, 118, 123, 118, 118, 123, 118, 118 ],
///        [ 178, 118, 178, 123, 123, 178, 118, 178 ],
///        [ 178, 123, 178, 123, 123, 178, 123, 178 ],
///        [ 118, 123, 178, 123, 123, 178, 123, 118 ],
///        [ 178, 118, 118, 178, 178, 118, 118, 178 ],
///    ]);
/// ```
///
#[allow(dead_code)]
pub fn eth_blockies_data_mapped<S: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
    seed: S,
    map_fn: F,
) -> EthBlockies<T> {
    let (palette, class_bitmap) = eth_blockies_indexed_data_mapped(seed, map_fn);

    class_bitmap.map_2d(|class, _| palette[class].clone())
}

/// Get Ethereum-style blockies data in indexed image format
///
/// # Arguments
///
/// * `seed` - Input seed
///
/// # Return
///
/// * A tuple of [(](tuple) `RgbPalette`, `EthBlockies<ColorClass>` [)](tuple)  
///   * `RgbPalette`: Array of [`RgbPixel`]  
///   * `EthBlockies<ColorClass>`: 2D array of palette indices
///
/// # Example
///
/// * Get RGB blockies data, composed of RGB palette + palette indices for each element
/// ```
/// use eth_blockies::*;
/// let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
///     .canonicalize_ethaddr();
/// let (color_palette, palette_idx_bitmap) = eth_blockies_indexed_data(seed);
///
///
/// // get (r, g, b) from palette
/// assert_eq!(color_palette[ColorClass::BgColor], (38, 173, 52));
/// assert_eq!(color_palette[ColorClass::Color], (132, 222, 77));
/// assert_eq!(color_palette[ColorClass::SpotColor], (4, 201, 40));
///
/// // get color class from pixels
/// assert_eq!(palette_idx_bitmap[0][0], ColorClass::Color);
/// assert_eq!(palette_idx_bitmap[2][0], ColorClass::SpotColor);
/// assert_eq!(palette_idx_bitmap[1][1], ColorClass::BgColor);
///
/// // get (r, g, b) from pixels
/// assert_eq!(color_palette[palette_idx_bitmap[0][0]], (132, 222, 77));
/// assert_eq!(color_palette[palette_idx_bitmap[2][0]], (4, 201, 40));
/// assert_eq!(color_palette[palette_idx_bitmap[1][1]], (38, 173, 52));
/// ```
///
#[allow(dead_code)]
pub fn eth_blockies_indexed_data<S: SeedInput>(seed: S) -> (RgbPalette, EthBlockies<ColorClass>) {
    eth_blockies_indexed_data_mapped(seed, |rgb_pixel| rgb_pixel)
}

/// Get Ethereum-style blockies data in indexed image format with mapped palette (`map_fn`)
///
/// Same with [`eth_blockies_indexed_data`],  
/// except that each [`RgbPixel`] output in [`Palette`] is mapped
/// with the function `map_fn` \(element mapping function\)
///
/// # Arguments
///
/// * `seed` - Input seed
/// * `map_fn` - Mapping function for each [`RgbPixel`] element in the returned array
///
/// # Return
///
/// * A tuple of [(](tuple) `Palette<T>`, `EthBlockies<ColorClass>` [)](tuple)  
///   * `Palette<T>`: Array of `T` returned by `map_fn`  
///   * `EthBlockies<ColorClass>`: 2D array of palette indices
///
/// # Example
///
/// * Get grayscale blockies data, composed of grayscale palette + palette indices for each element
/// ```
/// use eth_blockies::*;
///
/// fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
///     (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
/// }
///
/// let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
///     .canonicalize_ethaddr();
/// let (color_palette_grayscale, palette_idx_bitmap) =
///     eth_blockies_indexed_data_mapped(seed, rgb_to_grayscale);
///
///
/// // get (r, g, b) from palette
/// assert_eq!(color_palette_grayscale[ColorClass::BgColor], 118);
/// assert_eq!(color_palette_grayscale[ColorClass::Color], 178);
/// assert_eq!(color_palette_grayscale[ColorClass::SpotColor], 123);
///
/// // get color class from pixels
/// assert_eq!(palette_idx_bitmap[0][0], ColorClass::Color);
/// assert_eq!(palette_idx_bitmap[2][0], ColorClass::SpotColor);
/// assert_eq!(palette_idx_bitmap[1][1], ColorClass::BgColor);
///
/// // get (r, g, b) from pixels
/// assert_eq!(color_palette_grayscale[palette_idx_bitmap[0][0]], 178);
/// assert_eq!(color_palette_grayscale[palette_idx_bitmap[2][0]], 123);
/// assert_eq!(color_palette_grayscale[palette_idx_bitmap[1][1]], 118);
/// ```
///
#[allow(dead_code)]
pub fn eth_blockies_indexed_data_mapped<S: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
    seed: S,
    map_fn: F,
) -> (Palette<T>, EthBlockies<ColorClass>) {
    let mut keygen = BlockiesGenerator::new(seed.as_seed_bytes());

    // initialize palette
    let palette = {
        let (color, bgcolor, spotcolor) =
            { (keygen.next_rgb(), keygen.next_rgb(), keygen.next_rgb()) };
        [bgcolor, color, spotcolor].map(map_fn)
    };

    // initialize bitmap
    let bitmap = EthBlockies::new(|(x, _)| match x < (EthBlockies::<T>::DIMENSION.0 + 1) / 2 {
        true => keygen.next_colorclass(),
        false => ColorClass::BgColor, // dummy: right half is not used
    })
    .map_2d_with_ref(|src_arr, (x, y)| {
        // class
        match x < (EthBlockies::<T>::DIMENSION.0 + 1) / 2 {
            // left half: use itself
            true => src_arr[y][x].clone(),
            // right half: use corresponding flipped left half
            false => src_arr[y][EthBlockies::<T>::DIMENSION.0 - 1 - x].clone(),
        }
    });

    (palette, bitmap)
}

/// Get Ethereum-style blockies data in ANSI sequence format
///
/// # Arguments
///
/// * `seed` - Input seed
/// * `dimension` - (width, height) of output png binary data.
///                 Multiples of 8 recommended for both width and height.
/// * `is_utf_8` - Determine whether to print using UTF-8 characters.
///   * [`true`]: Output data contain UTF-8 characters,
///     which makes blockies printed in compact size.
///   * [`false`]: Output data do not contain any UTF-8 character,
///     but its size is bigger as one unit block is represented as two spaces ('0x20').
///
/// # Return
///
/// * A vector of ANSI sequnce string. Each string in the vector represents each line.
///
/// # Example
///
/// * Get RGB blockies data in ANSI sequence for terminal output
/// ```
/// use eth_blockies::*;
///
/// let seed = "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc";
/// let dimension = (8, 8);
/// let is_utf8 = true; // if false, print in less-compact format (pure ascii)
/// let ansi_string_data =
///     eth_blockies_ansiseq_data(seed.canonicalize_ethaddr(), dimension, is_utf8)
///         .join("\n");
///
/// println!("{}", ansi_string_data);
///
/// // use std::io::Write;
/// // writeln!(std::io::stdout(), "{}", ansi_string_data);
/// ```
#[allow(dead_code)]
pub fn eth_blockies_ansiseq_data<S: SeedInput>(
    seed: S,
    dimension: (usize, usize),
    is_utf_8: bool,
) -> Vec<String> {
    match is_utf_8 {
        true => ansi_seq::indexed_data_to_ansiseq_utf8(eth_blockies_indexed_data(seed), dimension),
        false => {
            ansi_seq::indexed_data_to_ansiseq_ascii(eth_blockies_indexed_data(seed), dimension)
        }
    }
}

/// Get Ethereum-style blockies data in indexed png format
///
/// # Arguments
///
/// * `seed` - Input seed
/// * `dimension` - (width, height) of output png binary data.
///                 Multiples of 8 recommended for both width and height.
/// * `compressed_output` - Determine if the result output png is compressed
///
/// # Return
///
/// * A byte vector of png binary data
///
/// # Example
///
/// * Get 16x16 uncompressed png data of RGB blockies
/// ```
/// use eth_blockies::*;
///
/// let seed = String::from("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC")
///     .canonicalize_ethaddr();
/// let dimension = (16, 16); // multiples of 8 recommended
/// let compressed_output = false; // true is recommended for normal use
/// let img_png_data = eth_blockies_png_data(seed, dimension, compressed_output);
///
/// assert_eq!(img_png_data,
///     b"\x89\x50\x4e\x47\x0d\x0a\x1a\x0a\x00\x00\x00\x0d\x49\x48\x44\x52\x00\
///       \x00\x00\x10\x00\x00\x00\x10\x02\x03\x00\x00\x00\x62\x9d\x17\xf2\x00\
///       \x00\x00\x09\x50\x4c\x54\x45\x26\xad\x34\x84\xde\x4d\x04\xc9\x28\xed\
///       \xf2\x1a\xc2\x00\x00\x00\x5b\x49\x44\x41\x54\x78\x01\x01\x50\x00\xaf\
///       \xff\x00\x55\x55\x55\x55\x00\x55\x55\x55\x55\x00\x50\x0a\xa0\x05\x00\
///       \x50\x0a\xa0\x05\x00\xa5\x50\x05\x5a\x00\xa5\x50\x05\x5a\x00\x00\xa0\
///       \x0a\x00\x00\x00\xa0\x0a\x00\x00\x50\x5a\xa5\x05\x00\x50\x5a\xa5\x05\
///       \x00\x5a\x5a\xa5\xa5\x00\x5a\x5a\xa5\xa5\x00\x0a\x5a\xa5\xa0\x00\x0a\
///       \x5a\xa5\xa0\x00\x50\x05\x50\x05\x00\x50\x05\x50\x05\x10\x15\x13\xed\
///       \x46\x70\x22\x4a\x00\x00\x00\x00\x49\x45\x4e\x44\xae\x42\x60\x82");
///
/// // uncomment below to write to file
///
/// // use std::io::Write;
/// // std::fs::File::create("test.png").unwrap().write_all(&img_png_data);
/// ```
#[allow(dead_code)]
pub fn eth_blockies_png_data<S: SeedInput>(
    seed: S,
    dimension: (usize, usize),
    compressed_output: bool,
) -> Vec<u8> {
    indexed_png::indexed_data_to_png(
        eth_blockies_indexed_data(seed),
        dimension,
        compressed_output,
    )
}

/// Get Ethereum-style blockies data in base64 format of indexed png
///
/// # Arguments
///
/// * `seed` - Input seed
/// * `dimension` - (width, height) of output png binary data.
///                 Multiples of 8 recommended for both width and height.
/// * `compressed_output` - Determine if the result output png is compressed
/// * `data_uri_output` - Determine if the result output is prefixed with data URI scheme
///
/// # Return
///
/// * A string of base64-encoded png data
///
/// # Example
///
/// * Get 16x16 uncompressed png data of RGB blockies in base64 format
/// ```
/// use eth_blockies::*;
///
/// let seed = String::from("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC")
///     .canonicalize_ethaddr();
/// let dimension = (16, 16); // multiples of 8 recommended
/// let compressed_output = false; // true is recommended for normal use
///
/// // base64 data only
/// let img_png_data =
///     eth_blockies_png_data_base64(&seed, dimension, compressed_output, false);
///
/// assert_eq!(img_png_data,
///     "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQAgMAAABinRfyAAAACVBMVEUmrTSE3k0EySjt8hrCAAAA\
///      W0lEQVR4AQFQAK//AFVVVVUAVVVVVQBQCqAFAFAKoAUApVAFWgClUAVaAACgCgAAAKAKAABQWqUF\
///      AFBapQUAWlqlpQBaWqWlAApapaAAClqloABQBVAFAFAFUAUQFRPtRnAiSgAAAABJRU5ErkJggg==");
///
/// // base64 data with data uri scheme prefix
/// let img_png_data_uri_scheme =
///     eth_blockies_png_data_base64(&seed, dimension, compressed_output, true);
///
/// assert_eq!(img_png_data_uri_scheme,
///     "data:image/png;base64,\
///      iVBORw0KGgoAAAANSUhEUgAAABAAAAAQAgMAAABinRfyAAAACVBMVEUmrTSE3k0EySjt8hrCAAAA\
///      W0lEQVR4AQFQAK//AFVVVVUAVVVVVQBQCqAFAFAKoAUApVAFWgClUAVaAACgCgAAAKAKAABQWqUF\
///      AFBapQUAWlqlpQBaWqWlAApapaAAClqloABQBVAFAFAFUAUQFRPtRnAiSgAAAABJRU5ErkJggg==");
///
/// // uncomment below to write to file
///
/// // use std::io::Write;
/// // let mut f = std::fs::File::create("test.png.base64").unwrap();
/// // f.write_all(img_png_data.as_bytes());
/// ```
#[allow(dead_code)]
pub fn eth_blockies_png_data_base64<S: SeedInput>(
    seed: S,
    dimension: (usize, usize),
    compressed_output: bool,
    data_uri_output: bool,
) -> String {
    String::from_utf8(
        [
            Vec::<u8>::from(match data_uri_output {
                true => b"data:image/png;base64,".as_ref(),
                false => b"".as_ref(),
            }),
            indexed_png::base64(&eth_blockies_png_data(seed, dimension, compressed_output)),
        ]
        .concat(),
    )
    .expect("unexpected internal error")
}
