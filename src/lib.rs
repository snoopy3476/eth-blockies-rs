//! A lightweight library to get Ethereum blockies raw data,
//! which can be used for creating blockies icon images, printing to terminal, etc.
//!
//! Useful when getting raw data of Ethereum blockies, not a full image file.
//!
//! * Rust implementation of following JavaScript package: <https://www.npmjs.com/package/ethereum-blockies-base64>
//!
//!
//! # Example
//!
//! * For all functions, each address argument for Ethereum (`eth_addr`) should be all lowercase, with leading '`0x`'.  
//!   This can be done with [`addr_canonicalize`](type_helper::eth_addr::EthAddr::addr_canonicalize).
//! ```
//! use eth_blockies::*;
//!
//! // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//! // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
//! let addr =
//!     "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC".to_owned()
//!     .addr_canonicalize();
//! assert_eq!(addr, "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
//!
//! let blockies_data = eth_blockies_data(addr);
//! ```
//!
//! * Get a raw blockies data
//! ```
//! use eth_blockies::*;
//!
//! let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!      .addr_canonicalize();
//!
//!
//! // get 2D array of (r, g, b)
//! {
//!     let blockies_data_rgb = eth_blockies_data(&addr);
//! }
//!
//!
//! // get 1D array of (r, g, b)
//! {
//!     let blockies_data_rgb = eth_blockies_data(&addr).serialize();
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
//!         eth_blockies_data_mapped(&addr, rgb_to_grayscale);
//! }
//!
//!
//! // get (color palette, palette index of each pixel)
//! {
//!     let (color_palette, palette_idx_bitmap) =
//!         eth_blockies_indexed_data(&addr);
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
//! * Write a generated blockies uncompressed png data to file `text.png`,
//!   on general Rust binary/library target
//! ```
//! use eth_blockies::*;
//!
//! let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!      .addr_canonicalize();
//! let dimension = (128, 128);
//! let img_png_data = eth_blockies_png_data(addr, dimension);
//!
//! // uncomment below to write to file
//!
//! // use std::io::Write;
//! // std::fs::File::create("test.png").unwrap()
//! //     .write_all(&img_png_data);
//! ```
//!
//!
//!
//! * Generate an html `img` element of a generated blockies, on wasm target  
//!   
//!   * First generate 8x8 image data string,
//!     then tell a browser to scale up properly when showing  
//!     (using style properties `width`, `height` and `image-rendering: pixelated`)
//! ```ignore
//! // addr to blockies data_uri,
//! // which can be used directly in img elem 'src' or css 'url()'
//! fn eth_blockies_data_uri(addr: &str) -> Option<String> {
//!     use eth_blockies::*;
//!
//!     let img_data_base64 =
//!         eth_blockies_png_data_base64(
//!             addr.addr_canonicalize(),
//!             (8, 8)
//!         );
//!
//!     String::from_utf8(img_data_base64)
//!         .map(|data| "data:image/png;base64,".to_owned() + &data)
//!         .ok()
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
//!         // set data uri to img src
//!         eth_blockies_data_uri(addr)
//!             .and_then(|data_uri|
//!                 img.set_attribute("src", &data_uri).ok()
//!             );
//!
//!         img.set_attribute(
//!             "style",
//!             concat!(
//!                 // no blur on scaling
//!                 "image-rendering: pixelated !important; ",
//!                 "width: 120px; height: 120px;",
//!             ),
//!         );
//!
//!         body.append_child(&img).ok()
//!     });
//! ```

#![no_std]

mod type_helper;
pub use type_helper::{eth_addr::EthAddr, eth_blockies_data::*};
mod data_encoder;
use data_encoder::indexed_png::*;
mod blockies_generator;
use blockies_generator::BlockiesGenerator;

extern crate alloc;
use alloc::vec::Vec;
use core::{
    mem::{transmute, MaybeUninit},
    ptr::addr_of_mut,
};

/// Get Ethereum blockies data
///
/// # Arguments
///
/// * `eth_addr` - Ethereum address
///
/// # Return
///
/// * 2D array of [`RgbPixel`]
///
/// # Example
/// ```
/// use eth_blockies::*;
/// let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
///     .addr_canonicalize();
///
/// // 2D array data
/// let blockies_data_rgb = eth_blockies_data(addr);
///
/// // 1D array (serial) data
/// let blockies_serial_data_rgb = blockies_data_rgb.serialize();
///
/// const COLORS: Palette = [(38, 173, 52), (132, 222, 77), (4, 201, 40)];
///
/// assert_eq!(blockies_data_rgb, [ [
///         COLORS[1], COLORS[1], COLORS[1], COLORS[1],
///         COLORS[1], COLORS[1], COLORS[1], COLORS[1],
///     ], [
///         COLORS[1], COLORS[0], COLORS[0], COLORS[2],
///         COLORS[2], COLORS[0], COLORS[0], COLORS[1],
///     ], [
///         COLORS[2], COLORS[1], COLORS[1], COLORS[0],
///         COLORS[0], COLORS[1], COLORS[1], COLORS[2],
///     ], [
///         COLORS[0], COLORS[0], COLORS[2], COLORS[0],
///         COLORS[0], COLORS[2], COLORS[0], COLORS[0],
///     ], [
///         COLORS[1], COLORS[0], COLORS[1], COLORS[2],
///         COLORS[2], COLORS[1], COLORS[0], COLORS[1],
///     ], [
///         COLORS[1], COLORS[2], COLORS[1], COLORS[2],
///         COLORS[2], COLORS[1], COLORS[2], COLORS[1],
///     ], [
///         COLORS[0], COLORS[2], COLORS[1], COLORS[2],
///         COLORS[2], COLORS[1], COLORS[2], COLORS[0],
///     ], [
///         COLORS[1], COLORS[0], COLORS[0], COLORS[1],
///         COLORS[1], COLORS[0], COLORS[0], COLORS[1],
///     ], ]
/// );
///
/// assert_eq!(blockies_serial_data_rgb, [
///     COLORS[1], COLORS[1], COLORS[1], COLORS[1],
///     COLORS[1], COLORS[1], COLORS[1], COLORS[1],
///
///     COLORS[1], COLORS[0], COLORS[0], COLORS[2],
///     COLORS[2], COLORS[0], COLORS[0], COLORS[1],
///
///     COLORS[2], COLORS[1], COLORS[1], COLORS[0],
///     COLORS[0], COLORS[1], COLORS[1], COLORS[2],
///
///     COLORS[0], COLORS[0], COLORS[2], COLORS[0],
///     COLORS[0], COLORS[2], COLORS[0], COLORS[0],
///
///     COLORS[1], COLORS[0], COLORS[1], COLORS[2],
///     COLORS[2], COLORS[1], COLORS[0], COLORS[1],
///
///     COLORS[1], COLORS[2], COLORS[1], COLORS[2],
///     COLORS[2], COLORS[1], COLORS[2], COLORS[1],
///
///     COLORS[0], COLORS[2], COLORS[1], COLORS[2],
///     COLORS[2], COLORS[1], COLORS[2], COLORS[0],
///
///     COLORS[1], COLORS[0], COLORS[0], COLORS[1],
///     COLORS[1], COLORS[0], COLORS[0], COLORS[1],
/// ]);
/// ```
///
#[allow(dead_code)]
pub fn eth_blockies_data<W: EthAddr>(eth_addr: W) -> EthBlockies<RgbPixel> {
    eth_blockies_data_mapped(eth_addr, |rgb_pixel| rgb_pixel)
}

/// Get Ethereum blockies data in mapped format with `map_fn`
///
/// Same with [`eth_blockies_data`],  
/// except that each [`RgbPixel`] output is mapped
/// with the function argument `map_fn` \(input mapping function\)
///
/// # Arguments
///
/// * `eth_addr` - Ethereum address
/// * `map_fn` - Mapping function for each element in array return
///
/// # Return
///
/// * 2D array of `T`, which is returned by `map_fn`
///
/// # Example
/// ```
/// use eth_blockies::*;
///
/// fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
///     (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
/// }
///
/// let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
///     .addr_canonicalize();
/// let blockies_data_grayscale = eth_blockies_data_mapped(addr, rgb_to_grayscale);
///
/// assert_eq!(blockies_data_grayscale, [
///        [178, 178, 178, 178, 178, 178, 178, 178],
///        [178, 118, 118, 123, 123, 118, 118, 178],
///        [123, 178, 178, 118, 118, 178, 178, 123],
///        [118, 118, 123, 118, 118, 123, 118, 118],
///        [178, 118, 178, 123, 123, 178, 118, 178],
///        [178, 123, 178, 123, 123, 178, 123, 178],
///        [118, 123, 178, 123, 123, 178, 123, 118],
///        [178, 118, 118, 178, 178, 118, 118, 178],
///    ]);
/// ```
///
#[allow(dead_code)]
pub fn eth_blockies_data_mapped<W: EthAddr, T: Clone, F: Fn(RgbPixel) -> T>(
    eth_addr: W,
    map_fn: F,
) -> EthBlockies<T> {
    let (palette, class_bitmap) = eth_blockies_indexed_data(eth_addr);

    // initialize ret_bitmap using MaybeUninit
    {
        let mut ret_bitmap_uninit: MaybeUninit<EthBlockies<T>> = MaybeUninit::uninit();

        class_bitmap
            .iter()
            .enumerate()
            .for_each(|(idx_row, class_row)| {
                class_row.iter().enumerate().for_each(|(idx, class)| {
                    let value = map_fn(palette[class]);

                    unsafe {
                        addr_of_mut!(
                            // calculate current bitmap ptr address
                            (*ret_bitmap_uninit.as_mut_ptr())[idx_row][idx]
                        )
                        .write_unaligned(value)
                    };
                });
            });

        unsafe { ret_bitmap_uninit.assume_init() }
    }
}

/// Get Ethereum blockies data in indexed image format
///
/// # Arguments
///
/// * `eth_addr` - Ethereum address
///
/// # Return
///
/// * A tuple of [(](tuple) `Array of colors`, `2D array of color indices` [)](tuple)
///
/// # Example
/// ```
/// use eth_blockies::*;
/// let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
///     .addr_canonicalize();
/// let (color_palette, palette_idx_bitmap) = eth_blockies_indexed_data(addr);
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
pub fn eth_blockies_indexed_data<W: EthAddr>(eth_addr: W) -> (Palette, EthBlockies<ColorClass>) {
    let mut keygen = BlockiesGenerator::new(eth_addr.addr_as_ref().as_bytes());

    // initialize palette using MaybeUninit
    let palette = {
        let mut palette_uninit: MaybeUninit<Palette> = MaybeUninit::uninit();
        [
            ColorClass::Color,
            ColorClass::BgColor,
            ColorClass::SpotColor,
        ]
        .iter()
        .for_each(|class| unsafe {
            addr_of_mut!((*palette_uninit.as_mut_ptr())[class]).write_unaligned(keygen.next_rgb())
        });
        unsafe { palette_uninit.assume_init() }
    };

    // initialize bitmap using MaybeUninit
    let bitmap = {
        // MaybeUninit::uninit().assume_init() here is safe:
        //   https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut bitmap_uninit: EthBlockies<MaybeUninit<ColorClass>> =
            unsafe { MaybeUninit::uninit().assume_init() };

        bitmap_uninit
            .iter_mut()
            .map(|row| (row.len() / 2, row))
            .map(|(mid_idx, row)| row.split_at_mut(mid_idx))
            .for_each(|(left, right)| {
                left.iter_mut()
                    .chain(
                        // dummy chain on left, in case of odd width
                        [MaybeUninit::uninit()].iter_mut(),
                    )
                    .zip(right.iter_mut().rev())
                    .for_each(|(l, r)| {
                        let colorclass = keygen.next_colorclass();
                        l.write(colorclass);
                        r.write(colorclass);
                    });
            });

        unsafe { transmute(bitmap_uninit) }
    };

    (palette, bitmap)
}

/// Get Ethereum blockies data in uncompressed indexed png format
///
/// * Note that this function is not suitable for high resolution blockies image, as a generated png is uncompressed
///
/// # Arguments
///
/// * `eth_addr` - Ethereum address
/// * `dimension` - (width, height) of output png binary data
///
/// # Return
///
/// * A byte vector of png binary data
///
/// # Example
///
/// * For image file
/// ```
/// use eth_blockies::*;
///
/// let addr = String::from("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC")
///     .addr_canonicalize();
/// let img_png_data = eth_blockies_png_data(addr, (128, 128));
///
/// // uncomment below to write to file
///
/// // use std::io::Write;
/// // std::fs::File::create("test.png").unwrap().write_all(&img_png_data);
/// ```
pub fn eth_blockies_png_data<W: EthAddr>(eth_addr: W, dimension: (u32, u32)) -> Vec<u8> {
    indexed_data_to_png(eth_blockies_indexed_data(eth_addr), dimension)
}

/// Get Ethereum blockies data in base64 format of uncompressed indexed png
///
/// * Note that this function is not suitable for high resolution blockies image, as a generated png is uncompressed
///
/// # Arguments
///
/// * `eth_addr` - Ethereum address
/// * `dimension` - (width, height) of output png binary data
///
/// # Return
///
/// * A byte vector (ASCII string) of base64-encoded png binary data
///
/// # Example
///
/// ```
/// use eth_blockies::*;
///
/// let addr = String::from("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC")
///     .addr_canonicalize();
/// let img_png_data = eth_blockies_png_data_base64(addr, (8, 8));
///
/// // uncomment below to write to file
///
/// // use std::io::Write;
/// // let mut f = std::fs::File::create("test.png.base64").unwrap();
/// // f.write_all(b"data:image/png;base64,");
/// // f.write_all(&img_png_data);
/// ```
pub fn eth_blockies_png_data_base64<W: EthAddr>(eth_addr: W, dimension: (u32, u32)) -> Vec<u8> {
    indexed_data_to_png_base64(eth_blockies_indexed_data(eth_addr), dimension)
}
