// deprecated functions from v1.0.0
// contents of this file will be removed on next release

use alloc::{string::String, vec::Vec};

#[cfg(feature = "compressed_png")]
use crate::indexed_png;

use crate::{
    ansi_seq, BlockiesGenerator, BlockiesHelper, ColorClass, EthBlockies, Palette, RgbPalette,
    RgbPixel, SeedInput,
};

/// Get Ethereum blockies data
///
/// (This function is deprecated: Use [`EthBlockies::data`](BlockiesGenerator::data) instead)
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
#[doc(hidden)]
#[deprecated(since = "1.1.0", note = "Use `EthBlockies::data` instead")]
pub fn eth_blockies_data<S: SeedInput>(seed: S) -> EthBlockies<RgbPixel> {
    EthBlockies::data(seed)
}

/// Get Ethereum blockies data in mapped format with `map_fn`
///
/// (This function is deprecated: Use [`EthBlockies::data_mapped`](BlockiesGenerator::data_mapped) instead)
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
#[doc(hidden)]
#[deprecated(since = "1.1.0", note = "Use `EthBlockies::data_mapped` instead")]
pub fn eth_blockies_data_mapped<S: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
    seed: S,
    map_fn: F,
) -> EthBlockies<T> {
    EthBlockies::data_mapped(seed, map_fn)
}

/// Get Ethereum blockies data in indexed image format
///
/// (This function is deprecated: Use [`EthBlockies::indexed_data`](BlockiesGenerator::indexed_data) instead)
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
#[doc(hidden)]
#[deprecated(since = "1.1.0", note = "Use `EthBlockies::indexed_data` instead")]
pub fn eth_blockies_indexed_data<S: SeedInput>(seed: S) -> (RgbPalette, EthBlockies<ColorClass>) {
    EthBlockies::indexed_data(seed.as_seed_bytes())
}

/// Get Ethereum blockies data in indexed image format with mapped palette (`map_fn`)
///
/// (This function is deprecated: Use [`EthBlockies::indexed_data_mapped`](BlockiesGenerator::indexed_data_mapped) instead)
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
#[doc(hidden)]
#[deprecated(
    since = "1.1.0",
    note = "Use `EthBlockies::indexed_data_mapped` instead"
)]
pub fn eth_blockies_indexed_data_mapped<S: SeedInput, T: Clone, F: Fn(RgbPixel) -> T>(
    seed: S,
    map_fn: F,
) -> (Palette<T>, EthBlockies<ColorClass>) {
    EthBlockies::indexed_data_mapped(seed.as_seed_bytes(), map_fn)
}

/// Get Ethereum-style blockies data in ANSI sequence format
///
/// (This function is deprecated: Use [`EthBlockies::ansiseq_data`](BlockiesGenerator::ansiseq_data) instead)
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
#[doc(hidden)]
#[deprecated(since = "1.1.0", note = "Use `EthBlockies::ansiseq_data` instead")]
pub fn eth_blockies_ansiseq_data<S: SeedInput>(
    seed: S,
    dimension: (usize, usize),
    is_utf_8: bool,
) -> Vec<String> {
    let (palette, bitmap) = EthBlockies::indexed_data(seed);
    match is_utf_8 {
        true => ansi_seq::indexed_data_to_ansiseq_utf8(palette, bitmap.scale(dimension), dimension),
        false => {
            ansi_seq::indexed_data_to_ansiseq_ascii(palette, bitmap.scale(dimension), dimension)
        }
    }
}

/// Get Ethereum-style blockies data in indexed png format
///
/// (This function is deprecated: Use [`EthBlockies::png_data`](BlockiesGenerator::png_data) or [`EthBlockies::compressed_png_data`](BlockiesGenerator::compressed_png_data) instead
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
#[doc(hidden)]
#[cfg(feature = "compressed_png")]
#[cfg_attr(docsrs, doc(cfg(feature = "compressed_png")))]
#[deprecated(
    since = "1.1.0",
    note = "Use `EthBlockies::png_data` or `EthBlockies::compressed_png_data` instead"
)]
pub fn eth_blockies_png_data<S: SeedInput>(
    seed: S,
    dimension: (usize, usize),
    compressed_output: bool,
) -> Vec<u8> {
    let (palette, bitmap) = EthBlockies::indexed_data(seed);
    indexed_png::indexed_data_to_png(
        palette,
        bitmap.scale(dimension),
        dimension,
        compressed_output,
    )
}

/// Get Ethereum-style blockies data in base64 format of indexed png
///
/// (This function is deprecated: Use [`EthBlockies::png_data_base64`](BlockiesGenerator::png_data_base64) or [`EthBlockies::compressed_png_data_base64`](BlockiesGenerator::compressed_png_data_base64) instead)
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
#[doc(hidden)]
#[cfg(feature = "compressed_png")]
#[cfg_attr(docsrs, doc(cfg(feature = "compressed_png")))]
#[deprecated(
    since = "1.1.0",
    note = "Use `EthBlockies::png_data_base64` or `EthBlockies::compressed_png_data_base64` instead"
)]
pub fn eth_blockies_png_data_base64<S: SeedInput>(
    seed: S,
    dimension: (usize, usize),
    compressed_output: bool,
    data_uri_output: bool,
) -> String {
    let png_data = match compressed_output {
        true => EthBlockies::compressed_png_data(seed, dimension),
        false => EthBlockies::png_data(seed, dimension),
    };
    indexed_png::base64_wrapper(&png_data, data_uri_output)
}

#[doc(hidden)]
#[deprecated(since = "1.1.0", note = "Use `SeedInput` instead")]
/// Canonicalize input string seed into a well-formed Ethereum address seed
pub trait SeedString {
    #[doc(hidden)]
    /// Get reference of string slice inside
    fn as_seed_str(&self) -> &str;

    #[doc(hidden)]
    #[deprecated(since = "1.1.0", note = "Use `SeedInput::to_ethaddr_seed` instead")]
    /// Convert given Ethereum address string to well-formed Ethereum address seed:  
    /// * Format: `0x(hex_letters_lowercase)`
    ///
    /// Call this function on a seed string to generate a *standard* Ethereum blockies
    /// (commonly seen as icons of wallet addresses, on many other Ethereum platforms).
    ///
    /// (Using this on non-Ethereum-address string is not an error,
    ///  but it is highly likely that the returned result is meaningless)
    ///
    /// # Example
    ///
    /// * General usage
    /// ```
    /// use eth_blockies::SeedString;
    ///
    /// // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr_1 = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    ///     .canonicalize_ethaddr();
    ///
    /// assert_eq!(addr_1, "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    ///
    /// // "e686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr_2 = String::from("e686c14ff9c11038f2b1c9ad617f2346cfb817dc")
    ///     .canonicalize_ethaddr();
    ///
    /// assert_eq!(addr_1, addr_2);
    /// ```
    ///
    /// * Usage without `use eth_blockies::SeedString;` statement
    /// ```
    /// // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    /// // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    /// let addr = <dyn eth_blockies::SeedString>::
    ///     canonicalize_ethaddr(&"0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC");
    ///
    /// assert_eq!(addr, "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
    /// ```
    fn canonicalize_ethaddr(&self) -> String {
        #[allow(deprecated)]
        String::from_utf8_lossy(&self.as_seed_str().to_ethaddr_seed()).into_owned()
    }
}

// string-slice
#[allow(deprecated)]
impl SeedString for &str {
    fn as_seed_str(&self) -> &str {
        *self
    }
}

// string
#[allow(deprecated)]
impl SeedString for String {
    fn as_seed_str(&self) -> &str {
        (*self).as_str()
    }
}
#[allow(deprecated)]
impl SeedString for &String {
    fn as_seed_str(&self) -> &str {
        (**self).as_str()
    }
}
