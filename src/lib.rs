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
//! * For all functions, each address argument for Ethereum wallet (`wallet_addr`) should be all lowercase, with leading '`0x`'.  
//!   This can be done with [`addr_canonicalize`](eth_wallet_addr::EthWalletAddr::addr_canonicalize).
//! ```
//! use eth_blockies::*;
//!
//! // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
//! // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
//! let addr = String::from("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC").addr_canonicalize();
//! assert_eq!(addr, "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc");
//!
//! let blockies_data = eth_blockies_data(addr);
//! ```

mod colorclass;
pub use colorclass::{ColorClass, ColorClassArrayMap};
mod eth_wallet_addr;
pub use eth_wallet_addr::EthWalletAddr;
mod indexed_png;
use indexed_png::*;

/// Unit RGB pixel data
pub type RgbPixel = (u8, u8, u8);

/// Dimension of Ethereum blockies data
const BLOCKIES_SIZE: usize = 8;

/// Ethereum blockies row data with type `T`
pub type EthBlockiesRow<T> = [T; BLOCKIES_SIZE];
/// Ethereum blockies data with type `T`
pub type EthBlockies<T> = [EthBlockiesRow<T>; BLOCKIES_SIZE];

/// Array map of colors composing Ethereum blockies
///
/// # Example
/// ```
/// use eth_blockies::*;
///
/// let colorclass_map: Palette = [(0, 0, 0), (127, 127, 127), (255, 255, 255)];
///
/// assert_eq!(colorclass_map[ColorClass::BgColor], (0, 0, 0));
/// assert_eq!(colorclass_map[ColorClass::Color], (127, 127, 127));
/// assert_eq!(colorclass_map[ColorClass::SpotColor], (255, 255, 255));
/// ```
pub type Palette = ColorClassArrayMap<RgbPixel>;

/// Get Ethereum blockies data
///
/// # Arguments
///
/// * `wallet_addr` - Wallet address
///
/// # Return
///
/// * 2D array of [`RgbPixel`]
///
#[allow(dead_code)]
pub fn eth_blockies_data<W: EthWalletAddr>(wallet_addr: W) -> EthBlockies<RgbPixel> {
    eth_blockies_data_mapped(wallet_addr, |rgb_pixel| rgb_pixel)
}

/// Get Ethereum blockies data in mapped format with `map_fn`
///
/// Same with [`eth_blockies_data`],  
/// except that each [`RgbPixel`] output is mapped
/// with the function argument `map_fn` \(input mapping function\)
///
/// # Arguments
///
/// * `wallet_addr` - Wallet address
/// * `map_fn` - Mapping function for each element in array return
///
/// # Return
///
/// * 2D array of `T`, which is returned by `map_fn`
///
#[allow(dead_code)]
pub fn eth_blockies_data_mapped<W: EthWalletAddr, T: Clone, F: Fn(RgbPixel) -> T>(
    wallet_addr: W,
    map_fn: F,
) -> EthBlockies<T> {
    let (palette, class_bitmap) = eth_blockies_indexed_data(wallet_addr);

    let mut ret_bitmap: EthBlockies<T> = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    ret_bitmap
        .iter_mut()
        .zip(class_bitmap)
        .for_each(|(ret_row, class_row)| {
            ret_row
                .iter_mut()
                .zip(class_row)
                .for_each(|(ret_elem, class)| {
                    *ret_elem = map_fn(palette[class]);
                });
        });

    ret_bitmap
}

/// Get Ethereum blockies data in indexed image format
///
/// # Arguments
///
/// * `wallet_addr` - Wallet address
///
/// # Return
///
/// * A tuple of [(](tuple) `Array of colors`, `2D array of color indices` [)](tuple)
///
#[allow(dead_code)]
pub fn eth_blockies_indexed_data<W: EthWalletAddr>(
    wallet_addr: W,
) -> (Palette, EthBlockies<ColorClass>) {
    let mut keygen = BlockiesGenerator::new(wallet_addr.addr_as_ref().as_bytes());

    let mut palette: Palette = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    [
        ColorClass::Color,
        ColorClass::BgColor,
        ColorClass::SpotColor,
    ]
    .iter()
    .for_each(|class| palette[class] = keygen.next_rgb());

    let mut bitmap: EthBlockies<ColorClass> =
        unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    bitmap
        .iter_mut()
        .map(|row| (row.len() / 2, row))
        .map(|(mid_idx, row)| row.split_at_mut(mid_idx))
        .for_each(|(left, right)| {
            left.iter_mut()
                .chain([ColorClass::BgColor].iter_mut()) // dummy, in case of odd width
                .zip(right.iter_mut().rev())
                .for_each(|(l, r)| {
                    let colorclass = keygen.next_colorclass();
                    *l = colorclass;
                    *r = colorclass;
                });
        });

    (palette, bitmap)
}

/// Get Ethereum blockies data in indexed png format
///
/// * For image file
/// ```
/// use eth_blockies::*;
///
/// let addr = String::from("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC")
///     .addr_canonicalize();
/// let img_png_data = eth_blockies_png_data(addr, (128, 128));
///
/// use std::io::Write;
/// std::fs::File::create("test.png").unwrap().write_all(&img_png_data);
/// ```
pub fn eth_blockies_png_data<W: EthWalletAddr>(wallet_addr: W, dimension: (u32, u32)) -> Vec<u8> {
    indexed_data_to_png(eth_blockies_indexed_data(wallet_addr), dimension)
}

/// Get Ethereum blockies data in base64 format of indexed png
///
/// ```
/// use eth_blockies::*;
///
/// let addr = String::from("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC")
///     .addr_canonicalize();
/// let img_png_data = eth_blockies_png_data_base64(addr, (8, 8));
///
/// use std::io::Write;
/// let mut f = std::fs::File::create("test.png.base64").unwrap();
/// f.write_all(b"data:image/png;base64,");
/// f.write_all(&img_png_data);
/// ```
pub fn eth_blockies_png_data_base64<W: EthWalletAddr>(
    wallet_addr: W,
    dimension: (u32, u32),
) -> Vec<u8> {
    indexed_data_to_png_base64(eth_blockies_indexed_data(wallet_addr), dimension)
}

/// Ethereum blockies generator, which stores necessary seeds for creating blockies
struct BlockiesGenerator {
    /// Seeds for generating ethereum blockies  
    /// (Named as "randseed" in original implementation)
    key_seeds: [i32; BlockiesGenerator::KEY_SEEDS_LEN],
    /// Current index of key_seeds to update when next_key runs
    key_seed_curidx: usize,
}

impl BlockiesGenerator {
    const KEY_SEEDS_LEN: usize = 4;

    /// Initialize new ethereum blockies generator using a given seed byte sequences
    fn new(seed: &[u8]) -> Self {
        Self {
            key_seeds: (seed.chunks(BlockiesGenerator::KEY_SEEDS_LEN).fold(
                [0_i32; BlockiesGenerator::KEY_SEEDS_LEN],
                |mut key_seeds_acc, seed_chunks| {
                    key_seeds_acc.iter_mut().zip(seed_chunks.iter()).for_each(
                        |(key_seed_cur, seed_char_cur)| {
                            *key_seed_cur = Self::key_seed_init(*key_seed_cur, *seed_char_cur)
                        },
                    );

                    key_seeds_acc
                },
            )),
            key_seed_curidx: 0,
        }
    }

    /// Update single element in key_seeds for initialization
    fn key_seed_init(key_seed_cur: i32, seed_char_cur: u8) -> i32 {
        (key_seed_cur << 5)
            .overflowing_sub(key_seed_cur)
            .0
            .overflowing_add(seed_char_cur as i32)
            .0
    }

    /// Get previous index of key_seeds
    fn idx_prev(idx: usize) -> usize {
        idx.overflowing_sub(1).0 % BlockiesGenerator::KEY_SEEDS_LEN
    }

    /// Get next index of key_seeds
    fn idx_next(idx: usize) -> usize {
        idx.overflowing_add(1).0 % BlockiesGenerator::KEY_SEEDS_LEN
    }

    /// Get next computed key using key_seeds, which is used for blockies generation
    /// Returns f64 in range: [0, 1]
    fn next_key(&mut self) -> f64 {
        self.key_seeds
            .get(Self::idx_prev(self.key_seed_curidx))
            .zip(self.key_seeds.get(self.key_seed_curidx))
            // calc new cur val
            .map(|(key_seed_prev, key_seed_cur)| {
                let tmp = *key_seed_cur ^ (*key_seed_cur << 11);
                *key_seed_prev ^ (*key_seed_prev >> 19) ^ tmp ^ (tmp >> 8)
            })
            // update self members
            .and_then(|key_seed_new_cur| {
                self.key_seeds
                    .get_mut(self.key_seed_curidx)
                    .map(|key_seed_cur_mut| {
                        self.key_seed_curidx = Self::idx_next(self.key_seed_curidx);

                        *key_seed_cur_mut = key_seed_new_cur;
                        key_seed_new_cur
                    })
            })
            // map to return val: map key_seed_new_cur in [0, 1] range
            .map(|key_seed_new_cur| {
                key_seed_new_cur.unsigned_abs() as f64 / ((i32::MAX as u32 + 1) as f64)
            })
            .expect("next_key")
    }

    /// Get next RGB pixel for palette using key_seeds
    fn next_rgb(&mut self) -> RgbPixel {
        fn hsl_to_rgb(hue: u32, saturation: f64, lightness: f64) -> RgbPixel {
            fn hue_to_rgb(p: f64, q: f64, t: u32) -> f64 {
                let t = match t {
                    0..=359 => t,
                    _ => t % 360,
                };

                return match t {
                    0..=60 => p + (q - p) * t as f64 / 60_f64,
                    0..=180 => q,
                    0..=240 => p + (q - p) * (4_f64 - t as f64 / 60_f64),
                    _ => p,
                };
            }

            let rgb_frac = match saturation == 0_f64 {
                true => (lightness, lightness, lightness),
                false => {
                    let q = match lightness < 0.5 {
                        true => lightness * (1_f64 + saturation),
                        false => lightness + saturation - lightness * saturation,
                    };
                    let p = 2_f64 * lightness - q;
                    (
                        hue_to_rgb(p, q, hue.overflowing_add(120).0),
                        hue_to_rgb(p, q, hue),
                        hue_to_rgb(p, q, hue.overflowing_add(240).0),
                    )
                }
            };

            (
                ((rgb_frac.0 * 255_f64) + 0.5_f64) as u8,
                ((rgb_frac.1 * 255_f64) + 0.5_f64) as u8,
                ((rgb_frac.2 * 255_f64) + 0.5_f64) as u8,
            )
        }

        hsl_to_rgb(
            (self.next_key() * 360_f64).floor() as u32,
            self.next_key() * 0.6_f64 + 0.4_f64,
            (self.next_key() + self.next_key() + self.next_key() + self.next_key()) * 0.25_f64,
        )
    }

    /// Get next color class for current pixel using key_seeds
    fn next_colorclass(&mut self) -> ColorClass {
        ((self.next_key() * 2.3_f64) as u8).try_into().unwrap()
    }
}
