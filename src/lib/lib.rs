//! A lightweight library in pure Rust to get Ethereum-style blocky identicon data,
//! which can be used for generating blockies icon images, printing to terminal, etc.
//!
//! Useful when getting raw RGB data of Ethereum-style blockies, as well as complete png image files.
//!
//!
//! # Basic Usage
//!
//!
//! 1. Define a blockies type (size) to use
//!
//!    * Make an alias type of [`Blockies`]
//!    ```
//!    use eth_blockies::{Blockies, BlockiesGenerator};
//!  
//!    // Blockies < size (const), T >
//!    type Icon<T> = Blockies<15, T>;
//!    ```
//!   
//!    * *Cf)* For Ethereum address blockies, [`EthBlockies`] is predefined as follows
//!    ```
//!    // use statement for Ethereum address blockies
//!    use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
//!    //   type 'EthBlockies<T>' is predefined as 'Blockies<8, T>'
//!    ```   
//! ---
//!
//!
//! 2. Select an input seed type
//!
//!    * Check for [`SeedInput`] to get full list of input seed types
//!    ```
//!    # use eth_blockies::{Blockies, BlockiesGenerator};
//!    #
//!    # type Icon<T> = Blockies<15, T>;
//!    #
//!    // generate blockies from various input type
//!    let from_string = Icon::data("eth-blockies".to_string());
//!    let from_byte_vec = Icon::data(vec![0x0c, 0x93, 0xa3, 0x2e]);
//!    ```
//!   
//!    * *Cf)* For Ethereum address seeds, apply [`to_ethaddr_seed()`](global_type_helper::SeedInput::to_ethaddr_seed) before passing as input seed
//!    ```
//!    # use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
//!    #
//!    // generate Ethereum address blockies from various input type
//!
//!    let seed_from_str = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!        .to_ethaddr_seed();
//!    let from_str = EthBlockies::data(seed_from_str);
//!
//!    let seed_from_bytes = [
//!          0xe6, 0x86, 0xc1, 0x4f, 0xf9, 0xc1, 0x10, 0x38, 0xf2, 0xb1,
//!          0xc9, 0xad, 0x61, 0x7f, 0x23, 0x46, 0xcf, 0xb8, 0x17, 0xdc,
//!        ].to_ethaddr_seed();
//!    let from_bytes = EthBlockies::data(seed_from_bytes);
//!    ```
//! ---
//!
//!
//! 3. Select an output data type
//!
//!    * Check for [`BlockiesGenerator`] to get full list of output data types
//!    ```
//!    # use eth_blockies::{Blockies, BlockiesGenerator};
//!    #
//!    # type Icon<T> = Blockies<15, T>;
//!    #
//!    # fn to_gray((r, g, b): (u8, u8, u8)) -> u8 {
//!    #     (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
//!    # }
//!    #
//!    // generate blockies in various forms
//!    let in_rgb_2d_arr = Icon::data("eth-blockies");
//!    let in_indexed_2d_arr = Icon::indexed_data("eth-blockies");
//!    let in_gray_2d_arr = Icon::data_mapped("eth-blockies", to_gray);
//!    let in_png_data_vec = Icon::png_data("eth-blockies", (128, 128));
//!    ```
//!
//!
//! # Example
//!
//! * Generate an Ethereum address blockies (with [`to_ethaddr_seed()`](global_type_helper::SeedInput::to_ethaddr_seed))
//!
//!   ```no_run
//!   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
//!
//!   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//!       .to_ethaddr_seed(); // required for Ethereum address blockies
//!
//!   // 8x8 2D-array of (r, g, b)
//!   {
//!       let eth_blockies_from_addr = EthBlockies::data(&seed);
//!   }
//!
//!   // uncompressed png data in byte vector
//!   {
//!       let eth_blockies_png_from_addr =
//!           EthBlockies::png_data(&seed, (128, 128));
//!           // use below for compressed png
//!           // EthBlockies::compressed_png_data(&seed, (128, 128));
//!
//!       // write data as png file
//!       use std::io::Write;
//!       std::fs::File::create("eth-blockies.png").unwrap()
//!           .write_all(&eth_blockies_png_from_addr);
//!   }
//!   ```
//!
//!
//! * Generate an html `img` blockies element, on wasm target
//!   ```ignore
//!   // addr to blockies data uri scheme,
//!   // which can be used directly in img elem 'src' or css 'url()'
//!   fn eth_blockies_data_uri(addr: &str) -> String {
//!      use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
//!
//!      let addr_input = addr.to_ethaddr_seed();
//!      let output_dim = (128, 128);
//!      let data_uri_output = true;
//!
//!      EthBlockies::png_data_base64(
//!         addr_input, output_dim, data_uri_output)
//!   }
//!
//!   use web_sys::*;
//!
//!   let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC";
//!
//!   window()
//!      .and_then(|w| w.document())
//!      .and_then(|doc| doc.body().zip(doc.create_element("img").ok()))
//!      .and_then(|(body, img)| {
//!         // create a new img html element with generated data_uri
//!         img.set_attribute("src", &eth_blockies_data_uri(addr))
//!            // then attach to body
//!            .and_then(|_| body.append_child(&img))
//!            .ok()
//!      });
//!   ```
//!
//!
//! # Cargo Features
//!
//! * `compressed_png` (Enabled by default)
//!   * This feature enables following functions:
//!     * [`compressed_png_data()`](BlockiesGenerator::compressed_png_data)
//!     * [`compressed_png_data_base64()`](BlockiesGenerator::compressed_png_data_base64)
//!   * This feature adds a following external dependency:
//!     * [`deflate`] crate
//!   * If png compression is not needed,
//!     disable this feature as follows when adding the crate:
//!     * E.g.
//!       * Shell: `cargo add eth-blockies@1.1 --no-default-features`
//!       * Cargo.toml: `eth-blockies = { version = "1.1", default-features = false }`

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod blockies;
pub use blockies::{Blockies, BlockiesHelper};
mod global_type_helper;
pub use global_type_helper::*;
mod data_encoder;
use data_encoder::*;

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// ( Alias of [`Blockies`]`<8, T>` ) Predefined [`Blockies`] for identicon of Ethereum address
pub type EthBlockies<T> = Blockies<8, T>;

/// Trait for generating a new [`Blockies`]
///
/// Used for generating data of a new blocky identicon in various form, including:
/// * Raw blockies data
/// * Terminal printable string (ANSI sequence)
/// * Image file data (png)
pub trait BlockiesGenerator<const S: usize> {
    /// Generate an Ethereum-style blockies data
    ///
    /// # Arguments
    ///
    /// * `seed` - Input seed
    ///
    /// # Return
    ///
    /// * Blockies RGB data, in 2D [`RgbPixel`] array
    ///
    /// # Example
    ///
    /// * Generate RGB blockies data
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<15, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///
    ///   // generate blockies
    ///   let icon_data_rgb = Blockies::<15>::data(seed);
    ///   let icon_data_rgb_alias = Identicon::data(seed);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(icon_data_rgb, icon_data_rgb_alias);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///
    ///   // generate blockies
    ///   let blockies_data_rgb = EthBlockies::data(addr);
    ///
    ///   // test
    ///   {
    ///       const P: eth_blockies::RgbPalette =
    ///           [(38, 173, 52), (132, 222, 77), (4, 201, 40)];
    ///
    ///       assert_eq!(blockies_data_rgb, [
    ///               [ P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1] ],
    ///               [ P[1], P[0], P[0], P[2], P[2], P[0], P[0], P[1] ],
    ///               [ P[2], P[1], P[1], P[0], P[0], P[1], P[1], P[2] ],
    ///               [ P[0], P[0], P[2], P[0], P[0], P[2], P[0], P[0] ],
    ///               [ P[1], P[0], P[1], P[2], P[2], P[1], P[0], P[1] ],
    ///               [ P[1], P[2], P[1], P[2], P[2], P[1], P[2], P[1] ],
    ///               [ P[0], P[2], P[1], P[2], P[2], P[1], P[2], P[0] ],
    ///               [ P[1], P[0], P[0], P[1], P[1], P[0], P[0], P[1] ],
    ///           ]);
    ///   }
    ///   ```
    ///
    fn data<I: SeedInput>(seed: I) -> Blockies<S, RgbPixel>;

    /// Generate an Ethereum-style blockies data, mapping each RGB color with `map_fn`
    ///
    /// Same with [`BlockiesGenerator::data`],  
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
    /// * Mapped blockies data, in 2D `T` array  
    ///   (`T`: type of each element returned by `map_fn`)
    ///
    /// # Example
    ///
    /// * Generate grayscale blockies data
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<19, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///   fn to_gray((r, g, b): (u8, u8, u8)) -> u8 {
    ///       (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    ///   }
    ///  
    ///   // generate blockies
    ///   let icon_data_gray = Blockies::<19>::data_mapped(seed, to_gray);
    ///   let icon_data_gray_alias = Identicon::data_mapped(seed, to_gray);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(icon_data_gray, icon_data_gray_alias);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///   fn to_gray((r, g, b): (u8, u8, u8)) -> u8 {
    ///       (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    ///   }
    ///
    ///   // generate blockies
    ///   let blockies_data_gray = EthBlockies::data_mapped(seed, to_gray);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(blockies_data_gray, [
    ///              [ 178, 178, 178, 178, 178, 178, 178, 178 ],
    ///              [ 178, 118, 118, 123, 123, 118, 118, 178 ],
    ///              [ 123, 178, 178, 118, 118, 178, 178, 123 ],
    ///              [ 118, 118, 123, 118, 118, 123, 118, 118 ],
    ///              [ 178, 118, 178, 123, 123, 178, 118, 178 ],
    ///              [ 178, 123, 178, 123, 123, 178, 123, 178 ],
    ///              [ 118, 123, 178, 123, 123, 178, 123, 118 ],
    ///              [ 178, 118, 118, 178, 178, 118, 118, 178 ],
    ///          ]);
    ///   }
    ///   ```
    ///
    fn data_mapped<I: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
        seed: I,
        map_fn: F,
    ) -> Blockies<S, T>;

    /// Generate an Ethereum-style blockies data in indexed image format
    ///
    /// # Arguments
    ///
    /// * `seed` - Input seed
    ///
    /// # Return
    ///
    /// * A tuple of [(](tuple) `RgbPalette`, `Blockies<S, ColorClass>` [)](tuple)  
    ///   * `RgbPalette`: Array of [`RgbPixel`]  
    ///   * `Blockies<S, ColorClass>`: 2D array of palette indices
    ///
    /// # Example
    ///
    /// * Get RGB blockies data, composed of (RGB palette, palette indices for each element)
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<6, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///
    ///   // generate blockies
    ///   let (rgb_palette, palette_idx_bitmap) =
    ///       Blockies::<6>::indexed_data(seed);
    ///   let (rgb_palette_alias, palette_idx_bitmap_alias) =
    ///       Identicon::indexed_data(seed);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(rgb_palette, rgb_palette_alias);
    ///       assert_eq!(palette_idx_bitmap, palette_idx_bitmap_alias);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///
    ///   // generate blockies
    ///   let (palette, palette_idx_bitmap) =
    ///       EthBlockies::indexed_data(seed);
    ///
    ///   // test
    ///   {
    ///       use eth_blockies::ColorClass;
    ///
    ///       // get (r, g, b) from palette
    ///       assert_eq!(palette[ColorClass::BgColor], (38, 173, 52));
    ///       assert_eq!(palette[ColorClass::Color], (132, 222, 77));
    ///       assert_eq!(palette[ColorClass::SpotColor], (4, 201, 40));
    ///
    ///       // get color class from pixels
    ///       assert_eq!(palette_idx_bitmap[0][0], ColorClass::Color);
    ///       assert_eq!(palette_idx_bitmap[2][0], ColorClass::SpotColor);
    ///       assert_eq!(palette_idx_bitmap[1][1], ColorClass::BgColor);
    ///
    ///       // get (r, g, b) from pixels
    ///       assert_eq!(palette[palette_idx_bitmap[0][0]], (132, 222, 77));
    ///       assert_eq!(palette[palette_idx_bitmap[2][0]], (4, 201, 40));
    ///       assert_eq!(palette[palette_idx_bitmap[1][1]], (38, 173, 52));
    ///   }
    ///   ```
    ///
    fn indexed_data<I: SeedInput>(seed: I) -> (RgbPalette, Blockies<S, ColorClass>);

    /// Generate an Ethereum-style blockies data in indexed image format, mapping each RGB palette color with `map_fn`
    ///
    /// Same with [`BlockiesGenerator::indexed_data`],  
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
    /// * A tuple of [(](tuple) `Palette<T>`, `Blockies<S, ColorClass>` [)](tuple)  
    ///   * `Palette<T>`: Array of `T` returned by `map_fn`  
    ///   * `Blockies<S, ColorClass>`: 2D array of palette indices
    ///
    /// # Example
    ///
    /// * Get grayscale blockies data, composed of (grayscale palette, palette indices for each element)
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<6, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///   fn to_gray((r, g, b): (u8, u8, u8)) -> u8 {
    ///       (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    ///   }
    ///  
    ///   // generate blockies
    ///   let (gray_palette, palette_idx_bitmap) =
    ///       Blockies::<6>::indexed_data_mapped(seed, to_gray);
    ///   let (gray_palette_alias, palette_idx_bitmap_alias) =
    ///       Identicon::indexed_data_mapped(seed, to_gray);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(gray_palette, gray_palette_alias);
    ///       assert_eq!(palette_idx_bitmap, palette_idx_bitmap_alias);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///   fn to_gray((r, g, b): (u8, u8, u8)) -> u8 {
    ///       (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    ///   }
    ///
    ///   // generate blockies
    ///   let (palette, palette_idx_bitmap) =
    ///       EthBlockies::indexed_data_mapped(seed, to_gray);
    ///
    ///   // test
    ///   {
    ///       use eth_blockies::ColorClass;
    ///
    ///       // get grayscale value from palette
    ///       assert_eq!(palette[ColorClass::BgColor], 118);
    ///       assert_eq!(palette[ColorClass::Color], 178);
    ///       assert_eq!(palette[ColorClass::SpotColor], 123);
    ///
    ///       // get color class from pixels
    ///       assert_eq!(palette_idx_bitmap[0][0], ColorClass::Color);
    ///       assert_eq!(palette_idx_bitmap[2][0], ColorClass::SpotColor);
    ///       assert_eq!(palette_idx_bitmap[1][1], ColorClass::BgColor);
    ///
    ///       // get grayscale value from pixels
    ///       assert_eq!(palette[palette_idx_bitmap[0][0]], 178);
    ///       assert_eq!(palette[palette_idx_bitmap[2][0]], 123);
    ///       assert_eq!(palette[palette_idx_bitmap[1][1]], 118);
    ///   }
    ///   ```
    ///
    fn indexed_data_mapped<I: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
        seed: I,
        map_fn: F,
    ) -> (Palette<T>, Blockies<S, ColorClass>);

    /// Generate an Ethereum-style blockies data in ANSI sequence format (terminal-printable)
    ///
    /// # Arguments
    ///
    /// * `seed` - Input seed
    /// * `output_dim` - (width, height) of output png binary data.
    ///                  Multiples of `const S` ([`SIZE`](BlockiesHelper::SIZE)) recommended for both width and height.
    /// * `is_utf8` - Determine whether to print using UTF-8 characters.
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
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<20, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///   let output_dim = (40, 40); // multiples of size recommended
    ///   let is_utf8 = true; // if false, print in less-compact format
    ///
    ///   // generate blockies
    ///   let icon_ansiseq_string_joined =
    ///       Blockies::<20>::ansiseq_data(seed, output_dim, is_utf8)
    ///       .join("\n");
    ///   let icon_ansiseq_string_alias_joined =
    ///       Identicon::ansiseq_data(seed, output_dim, is_utf8)
    ///       .join("\n");
    ///
    ///   // test
    ///   {
    ///       assert_eq!(icon_ansiseq_string_joined,
    ///                  icon_ansiseq_string_alias_joined);
    ///
    ///       println!("{}", icon_ansiseq_string_joined);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    ///       .to_ethaddr_seed();
    ///   let output_dim = (8, 8); // multiples of size recommended
    ///   let is_utf8 = true; // if false, print in less-compact format
    ///
    ///   // generate blockies
    ///   let ansi_string_joined = EthBlockies::ansiseq_data(
    ///       seed, output_dim, is_utf8).join("\n");
    ///
    ///   // print
    ///   {
    ///       println!("{}", ansi_string_joined);
    ///
    ///       // print to terminal in different manner
    ///       // use std::io::Write;
    ///       // writeln!(std::io::stdout(), "{}", ansi_string_joined);
    ///   }
    ///   ```
    fn ansiseq_data<I: SeedInput>(
        seed: I,
        output_dim: (usize, usize),
        is_utf8: bool,
    ) -> Vec<String>;

    /// Generate an Ethereum-style blockies data in uncompressed indexed png format
    ///
    /// # Arguments
    ///
    /// * `seed` - Input seed
    /// * `output_dim` - (width, height) of output png binary data.
    ///                  Multiples of `const S` ([`SIZE`](BlockiesHelper::SIZE)) recommended for both width and height.
    ///
    /// # Return
    ///
    /// * A byte vector of png binary data
    ///
    /// # Example
    ///
    /// * Get uncompressed png data of RGB blockies
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<11, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///   let output_dim = (64, 64); // multiples of size recommended
    ///
    ///   // generate blockies
    ///   let icon_png_data = Blockies::<11>::png_data(seed, output_dim);
    ///   let icon_png_data_alias = Identicon::png_data(seed, output_dim);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(icon_png_data, icon_png_data_alias);
    ///
    ///       // uncomment below to write to file
    ///       // use std::io::Write;
    ///       // std::fs::File::create("icon.png").unwrap()
    ///       //     .write_all(&icon_png_data);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///   let output_dim = (16, 16); // multiples of size recommended
    ///
    ///   // generate blockies
    ///   let img_png_data = EthBlockies::png_data(seed, output_dim);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(img_png_data,
    ///           b"\x89\x50\x4e\x47\x0d\x0a\x1a\x0a\x00\x00\x00\x0d\x49\
    ///             \x48\x44\x52\x00\x00\x00\x10\x00\x00\x00\x10\x02\x03\
    ///             \x00\x00\x00\x62\x9d\x17\xf2\x00\x00\x00\x09\x50\x4c\
    ///             \x54\x45\x26\xad\x34\x84\xde\x4d\x04\xc9\x28\xed\xf2\
    ///             \x1a\xc2\x00\x00\x00\x5b\x49\x44\x41\x54\x78\x01\x01\
    ///             \x50\x00\xaf\xff\x00\x55\x55\x55\x55\x00\x55\x55\x55\
    ///             \x55\x00\x50\x0a\xa0\x05\x00\x50\x0a\xa0\x05\x00\xa5\
    ///             \x50\x05\x5a\x00\xa5\x50\x05\x5a\x00\x00\xa0\x0a\x00\
    ///             \x00\x00\xa0\x0a\x00\x00\x50\x5a\xa5\x05\x00\x50\x5a\
    ///             \xa5\x05\x00\x5a\x5a\xa5\xa5\x00\x5a\x5a\xa5\xa5\x00\
    ///             \x0a\x5a\xa5\xa0\x00\x0a\x5a\xa5\xa0\x00\x50\x05\x50\
    ///             \x05\x00\x50\x05\x50\x05\x10\x15\x13\xed\x46\x70\x22\
    ///             \x4a\x00\x00\x00\x00\x49\x45\x4e\x44\xae\x42\x60\x82");
    ///
    ///       // uncomment below to write to file
    ///       // use std::io::Write;
    ///       // std::fs::File::create("test.png").unwrap()
    ///       //     .write_all(&img_png_data);
    ///   }
    ///   ```
    fn png_data<I: SeedInput>(seed: I, output_dim: (usize, usize)) -> Vec<u8>;

    /// Generate an Ethereum-style blockies data in compressed indexed png format
    ///
    /// # Arguments
    ///
    /// * `seed` - Input seed
    /// * `output_dim` - (width, height) of output png binary data.
    ///                  Multiples of `const S` ([`SIZE`](BlockiesHelper::SIZE)) recommended for both width and height.
    ///
    /// # Return
    ///
    /// * A byte vector of png binary data
    ///
    /// # Example
    ///
    /// * Get compressed png data of RGB blockies
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<9, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///   let output_dim = (64, 64); // multiples of size recommended
    ///
    ///   // generate blockies
    ///   let icon_compressed_png_data =
    ///       Blockies::<9>::compressed_png_data(seed, output_dim);
    ///   let icon_compressed_png_data_alias =
    ///       Identicon::compressed_png_data(seed, output_dim);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(icon_compressed_png_data,
    ///                  icon_compressed_png_data_alias);
    ///
    ///       // uncomment below to write to file
    ///       // use std::io::Write;
    ///       // std::fs::File::create("icon.png").unwrap()
    ///       //     .write_all(&icon_compressed_png_data);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///   let output_dim = (16, 16); // multiples of size recommended
    ///
    ///   // generate blockies
    ///   let img_png_data =
    ///       EthBlockies::compressed_png_data(seed, output_dim);
    ///
    ///   // print
    ///   {
    ///       // uncomment below to write to file
    ///       // use std::io::Write;
    ///       // std::fs::File::create("test.png").unwrap()
    ///       //     .write_all(&img_png_data);
    ///   }
    ///   ```
    #[cfg(feature = "compressed_png")]
    #[cfg_attr(docsrs, doc(cfg(feature = "compressed_png")))]
    fn compressed_png_data<I: SeedInput>(seed: I, output_dim: (usize, usize)) -> Vec<u8>;

    /// Generate an Ethereum-style blockies data in base64 format of uncompressed indexed png
    ///
    /// # Arguments
    ///
    /// * `seed` - Input seed
    /// * `output_dim` - (width, height) of output png binary data.
    ///                  Multiples of `const S` ([`SIZE`](BlockiesHelper::SIZE)) recommended for both width and height.
    /// * `data_uri_output` - Determine if the result output is prefixed with data URI scheme
    ///
    /// # Return
    ///
    /// * A string of base64-encoded png data
    ///
    /// # Example
    ///
    /// * Get uncompressed png data of RGB blockies in base64 format
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<7, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///   let output_dim = (64, 64); // multiples of size recommended
    ///   let data_uri = false; // true: prepend "data:image/png;base64,"
    ///
    ///   // generate blockies
    ///   let icon_png_data_base64_string =
    ///       Blockies::<7>::png_data_base64(seed, output_dim, data_uri);
    ///   let icon_png_data_base64_alias_string =
    ///       Identicon::png_data_base64(seed, output_dim, data_uri);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(icon_png_data_base64_string,
    ///                  icon_png_data_base64_alias_string);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///   let output_dim = (16, 16); // multiples of size recommended
    ///
    ///   // generate blockies
    ///   {
    ///       // base64 data only
    ///       let data_uri = false;
    ///       let img_png_data_base64_string =
    ///           EthBlockies::png_data_base64(&seed, output_dim, data_uri);
    ///
    ///       // test
    ///       assert_eq!(img_png_data_base64_string,
    ///           "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQAgMAAABinRfyAAAACVBMVEU\
    ///            mrTSE3k0EySjt8hrCAAAAW0lEQVR4AQFQAK//AFVVVVUAVVVVVQBQCq\
    ///            AFAFAKoAUApVAFWgClUAVaAACgCgAAAKAKAABQWqUFAFBapQUAWlqlp\
    ///            QBaWqWlAApapaAAClqloABQBVAFAFAFUAUQFRPtRnAiSgAAAABJRU5E\
    ///            rkJggg==");
    ///   }
    ///   {
    ///       // base64 data with data uri scheme prefix
    ///       let data_uri = true;
    ///       let img_png_data_base64_uri_scheme_string =
    ///           EthBlockies::png_data_base64(&seed, output_dim, data_uri);
    ///
    ///       // test
    ///       assert_eq!(img_png_data_base64_uri_scheme_string,
    ///           "data:image/png;base64,\
    ///            iVBORw0KGgoAAAANSUhEUgAAABAAAAAQAgMAAABinRfyAAAACVBMVEU\
    ///            mrTSE3k0EySjt8hrCAAAAW0lEQVR4AQFQAK//AFVVVVUAVVVVVQBQCq\
    ///            AFAFAKoAUApVAFWgClUAVaAACgCgAAAKAKAABQWqUFAFBapQUAWlqlp\
    ///            QBaWqWlAApapaAAClqloABQBVAFAFAFUAUQFRPtRnAiSgAAAABJRU5E\
    ///            rkJggg==");
    ///   }
    ///   ```
    fn png_data_base64<I: SeedInput>(
        seed: I,
        output_dim: (usize, usize),
        data_uri_output: bool,
    ) -> String;

    /// Generate an Ethereum-style blockies data in base64 format of compressed indexed png
    ///
    /// # Arguments
    ///
    /// * `seed` - Input seed
    /// * `output_dim` - (width, height) of output png binary data.
    ///                  Multiples of `const S` ([`SIZE`](BlockiesHelper::SIZE)) recommended for both width and height.
    /// * `data_uri_output` - Determine if the result output is prefixed with data URI scheme
    ///
    /// # Return
    ///
    /// * A string of base64-encoded png data
    ///
    /// # Example
    ///
    /// * Get compressed png data of RGB blockies in base64 format
    ///
    ///   * General identicon
    ///   ```
    ///   use eth_blockies::{Blockies, BlockiesGenerator};
    ///   type Identicon<T> = Blockies<12, T>; // user-defined blockies type
    ///
    ///   // args
    ///   let seed = "general string seed";
    ///   let output_dim = (64, 64); // multiples of size recommended
    ///   let data_uri = false; // true: prepend "data:image/png;base64,"
    ///
    ///   // generate blockies
    ///   let icon_png_data_base64_string =
    ///       Blockies::<12>::compressed_png_data_base64(
    ///           seed, output_dim, data_uri);
    ///   let icon_png_data_base64_alias_string =
    ///       Identicon::compressed_png_data_base64(
    ///           seed, output_dim, data_uri);
    ///
    ///   // test
    ///   {
    ///       assert_eq!(icon_png_data_base64_string,
    ///                  icon_png_data_base64_alias_string);
    ///   }
    ///   ```
    ///
    ///   * Ethereum blockies
    ///   ```
    ///   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
    ///
    ///   // args
    ///   let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///       .to_ethaddr_seed();
    ///   let output_dim = (16, 16); // multiples of size recommended
    ///
    ///   // generate blockies
    ///   {
    ///       // base64 data only
    ///       let data_uri_output = false;
    ///       let img_png_data_base64_string =
    ///           EthBlockies::compressed_png_data_base64(
    ///               &seed, output_dim, data_uri_output);
    ///   }
    ///   {
    ///       // base64 data with data uri scheme prefix
    ///       let data_uri_output = true;
    ///       let img_png_data_base64_uri_scheme_string =
    ///           EthBlockies::compressed_png_data_base64(
    ///               &seed, output_dim, data_uri_output);
    ///   }
    ///   ```
    #[cfg(feature = "compressed_png")]
    #[cfg_attr(docsrs, doc(cfg(feature = "compressed_png")))]
    fn compressed_png_data_base64<I: SeedInput>(
        seed: I,
        output_dim: (usize, usize),
        data_uri_output: bool,
    ) -> String;
}

impl<const S: usize> BlockiesGenerator<S> for Blockies<S> {
    fn data<I: SeedInput>(seed: I) -> Blockies<S, RgbPixel> {
        Blockies::data_mapped(seed, |rgb_pixel| rgb_pixel)
    }

    fn data_mapped<I: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
        seed: I,
        map_fn: F,
    ) -> Blockies<S, T> {
        let (palette, class_bitmap) = Blockies::indexed_data_mapped(seed, map_fn);

        class_bitmap.map_2d(|class, _| palette[class].clone())
    }

    fn indexed_data<I: SeedInput>(seed: I) -> (RgbPalette, Blockies<S, ColorClass>) {
        blockies::new_blockies(seed.as_seed_bytes())
    }

    fn indexed_data_mapped<I: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
        seed: I,
        map_fn: F,
    ) -> (Palette<T>, Blockies<S, ColorClass>) {
        let (rgb_palette, class_bitmap) = Blockies::indexed_data(seed.as_seed_bytes());

        (rgb_palette.map(map_fn), class_bitmap)
    }

    fn ansiseq_data<I: SeedInput>(
        seed: I,
        output_dim: (usize, usize),
        is_utf8: bool,
    ) -> Vec<String> {
        let (palette, bitmap) = Blockies::<S>::indexed_data(seed);
        match is_utf8 {
            true => ansi_seq::indexed_data_to_ansiseq_utf8(
                palette,
                bitmap.scale(output_dim),
                output_dim,
            ),
            false => ansi_seq::indexed_data_to_ansiseq_ascii(
                palette,
                bitmap.scale(output_dim),
                output_dim,
            ),
        }
    }

    fn png_data<I: SeedInput>(seed: I, output_dim: (usize, usize)) -> Vec<u8> {
        let (palette, bitmap) = Blockies::<S>::indexed_data(seed);
        indexed_png::indexed_data_to_png(palette, bitmap.scale(output_dim), output_dim, false)
    }

    #[cfg(feature = "compressed_png")]
    fn compressed_png_data<I: SeedInput>(seed: I, output_dim: (usize, usize)) -> Vec<u8> {
        let (palette, bitmap) = Blockies::<S>::indexed_data(seed);
        indexed_png::indexed_data_to_png(palette, bitmap.scale(output_dim), output_dim, true)
    }

    fn png_data_base64<I: SeedInput>(
        seed: I,
        output_dim: (usize, usize),
        data_uri_output: bool,
    ) -> String {
        indexed_png::base64_wrapper(&Blockies::<S>::png_data(seed, output_dim), data_uri_output)
    }

    #[cfg(feature = "compressed_png")]
    fn compressed_png_data_base64<I: SeedInput>(
        seed: I,
        output_dim: (usize, usize),
        data_uri_output: bool,
    ) -> String {
        indexed_png::base64_wrapper(
            &Blockies::<S>::compressed_png_data(seed, output_dim),
            data_uri_output,
        )
    }
}

// deprecated functions from v1.0.0
mod compat;
pub use compat::*;
