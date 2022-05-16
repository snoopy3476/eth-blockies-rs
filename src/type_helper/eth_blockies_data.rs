use alloc::vec::Vec;
use core::{
    convert::TryFrom,
    mem::{transmute_copy, MaybeUninit},
    ops::{Index, IndexMut},
    ptr::addr_of_mut,
};

/// Dimension of Ethereum blockies data
const BLOCKIES_SIZE: usize = 8;

/// Ethereum blockies data with type `T`
pub type EthBlockies<T> = [EthBlockiesRow<T>; BLOCKIES_SIZE];
/// Ethereum blockies row data with type `T`
pub type EthBlockiesRow<T> = [T; BLOCKIES_SIZE];
/// Ethereum blockies serialized data with type `T`
///
/// This can be generated by performing [`serialize`](EthBlockiesHelper::serialize)
/// on [`EthBlockies`] type.
pub type EthBlockiesSerial<T> = [T; BLOCKIES_SIZE * BLOCKIES_SIZE];

/// Trait for [`EthBlockies`]
pub trait EthBlockiesHelper<T: Clone> {
    /// Flatten 2D array to 1D array
    ///
    /// # Return
    /// * 1D array of `T`
    ///
    /// # Example
    /// ```
    /// use eth_blockies::*;
    /// const COLORS: Palette = [(38, 173, 52), (132, 222, 77), (4, 201, 40)];
    ///
    /// // original data (2d)
    /// let blockies_data_rgb_2d: EthBlockies<RgbPixel> = [ [
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
    ///     ], ];
    ///
    /// // serialize data to 1d array
    /// let blockies_data_rgb_1d: EthBlockiesSerial<RgbPixel>
    ///     = blockies_data_rgb_2d.serialize();
    ///
    ///
    /// assert_eq!(blockies_data_rgb_1d, [
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
    fn serialize(self) -> EthBlockiesSerial<T>;

    /// Scale `(8, 8)` 2d array to given dimension
    ///
    /// * Note that this function does not perform any kind of pixel blending at edge.  
    ///   Therefore, dimension lower than `(100, 100)` may generate unbalanced image **if both `(width, height)` are not multiples of 8**.
    ///
    /// # Return
    /// * 2D vector of `T`
    ///
    /// # Example
    fn scale(self, dimension: (u32, u32)) -> Vec<Vec<T>>;
}
impl<T: Clone> EthBlockiesHelper<T> for EthBlockies<T> {
    // In many cases, this func is expected to have zero-impact
    //   after compilation & optimization
    fn serialize(self) -> EthBlockiesSerial<T> {
        // initialize ret_arr using MaybeUninit
        {
            let mut ret_arr_uninit: MaybeUninit<EthBlockiesSerial<T>> = MaybeUninit::uninit();

            self.iter().enumerate().for_each(|(idx_row, row)| {
                row.iter().enumerate().for_each(|(idx, v)| unsafe {
                    addr_of_mut!((*ret_arr_uninit.as_mut_ptr())[idx_row * BLOCKIES_SIZE + idx])
                        .write_unaligned(v.clone());
                });
            });

            unsafe { transmute_copy(&ret_arr_uninit) }
        }
    }

    fn scale(self, dimension: (u32, u32)) -> Vec<Vec<T>> {
        // if nothing to scale data
        if dimension == (BLOCKIES_SIZE as u32, BLOCKIES_SIZE as u32) {
            self.iter().map(|row| Vec::from(row.as_slice())).collect()
        }
        // if both width && height is multiples of BLOCKIES_SIZE
        else if dimension.0.overflowing_rem(BLOCKIES_SIZE as u32).0 == 0
            && dimension.1.overflowing_rem(BLOCKIES_SIZE as u32).0 == 0
        {
            let scale = (
                dimension.0 / BLOCKIES_SIZE as u32,
                dimension.1 / BLOCKIES_SIZE as u32,
            );
            self.iter().fold(Vec::<Vec<T>>::new(), |mut ret_vec, row| {
                let new_row: Vec<T> = row.iter().fold(Vec::<T>::new(), |mut ret_row, elem| {
                    (0..scale.0).for_each(|_| ret_row.push(elem.clone()));
                    ret_row
                });
                (0..scale.1).for_each(|_| ret_vec.push(new_row.clone()));
                ret_vec
            })
        }
        // for other dimensions: general case
        else {
            // calculate expected vs actual pixels for each step,
            // then returns array that shows if each step needs extra elem
            fn calculate_extra_elem_needed(
                pixels_per_class: f64,
                scale: u32,
            ) -> [bool; BLOCKIES_SIZE] {
                (0..BLOCKIES_SIZE)
                    .fold(
                        ([false; BLOCKIES_SIZE], 0_i32),
                        |(mut ret_arr, pixels_diff_prev), idx| {
                            let pixels_diff =
                                ((pixels_per_class - scale as f64) * (idx + 1) as f64) as i32;

                            if pixels_diff != pixels_diff_prev {
                                ret_arr[idx] = true;
                            }

                            (ret_arr, pixels_diff)
                        },
                    )
                    .0
            }

            let scale = (
                dimension.0 / BLOCKIES_SIZE as u32,
                dimension.1 / BLOCKIES_SIZE as u32,
            );
            let extra_elem_needed = (
                calculate_extra_elem_needed(dimension.0 as f64 / BLOCKIES_SIZE as f64, scale.0),
                calculate_extra_elem_needed(dimension.1 as f64 / BLOCKIES_SIZE as f64, scale.1),
            );

            self.iter()
                .enumerate()
                .fold(Vec::<Vec<T>>::new(), |mut ret_vec, (idx_row, row)| {
                    let new_row: Vec<T> =
                        row.iter()
                            .enumerate()
                            .fold(Vec::<T>::new(), |mut ret_row, (idx, elem)| {
                                (0..scale.0).for_each(|_| ret_row.push(elem.clone()));
                                if extra_elem_needed.0[idx] {
                                    ret_row.push(elem.clone());
                                }
                                ret_row
                            });

                    (0..scale.1).for_each(|_| ret_vec.push(new_row.clone()));
                    if extra_elem_needed.1[idx_row] {
                        ret_vec.push(new_row.clone());
                    }

                    ret_vec
                })
        }
    }
}

/// Type of color in Ethereum blockies
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ColorClass {
    /// Color 1
    BgColor = 0,
    /// Color 2
    Color = 1,
    /// Color 3
    SpotColor = 2,
}

/// Array for [`ColorClass`], which can be accessed like a map
///
/// # Example
/// ```
/// use eth_blockies::*;
///
/// let colorclass_map: ColorClassArrayMap<&str> = ["first_elem", "second_elem", "third_elem"];
///
/// assert_eq!(colorclass_map[ColorClass::BgColor], "first_elem");
/// assert_eq!(colorclass_map[ColorClass::Color], "second_elem");
/// assert_eq!(colorclass_map[ColorClass::SpotColor], "third_elem");
/// ```
pub type ColorClassArrayMap<T> = [T; 3];

impl TryFrom<u8> for ColorClass {
    type Error = ();

    fn try_from(raw_value: u8) -> Result<Self, Self::Error> {
        match raw_value {
            0..=2 => Ok(unsafe { core::mem::transmute(raw_value) }),
            _ => Err(()),
        }
    }
}

impl Into<u8> for ColorClass {
    fn into(self) -> u8 {
        self as u8
    }
}

impl<T> Index<ColorClass> for ColorClassArrayMap<T> {
    type Output = T;
    fn index(&self, colorclass: ColorClass) -> &Self::Output {
        unsafe { self.get_unchecked(colorclass as usize) }
    }
}
impl<T> IndexMut<ColorClass> for ColorClassArrayMap<T> {
    fn index_mut(&mut self, colorclass: ColorClass) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(colorclass as usize) }
    }
}

impl<T> Index<&ColorClass> for ColorClassArrayMap<T> {
    type Output = T;
    fn index(&self, colorclass: &ColorClass) -> &Self::Output {
        unsafe { self.get_unchecked(*colorclass as usize) }
    }
}
impl<T> IndexMut<&ColorClass> for ColorClassArrayMap<T> {
    fn index_mut(&mut self, colorclass: &ColorClass) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(*colorclass as usize) }
    }
}

/// Unit RGB pixel data
pub type RgbPixel = (u8, u8, u8);
const RGBPIXEL_SIZE: usize = 3;

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
const PALETTE_SIZE: usize = 3;

/// Ethereum blockies palette serialized data with type `T`
///
/// This can be generated by performing [`serialize`](PaletteHelper::serialize)
/// on [`Palette`] type.
pub type PaletteSerial = [u8; RGBPIXEL_SIZE * PALETTE_SIZE];

/// Trait for [`Palette`]
pub trait PaletteHelper {
    /// Flatten 2D array to 1D array
    ///
    /// # Return
    /// * 1D array of `T`
    ///
    /// # Example
    /// ```
    /// use eth_blockies::*;
    /// const COLORS: Palette = [(38, 173, 52), (132, 222, 77), (4, 201, 40)];
    ///
    /// // serialize palette to 1d byte array
    /// let palette_1d: PaletteSerial
    ///     = COLORS.serialize();
    ///
    ///
    /// assert_eq!(palette_1d, [38, 173, 52, 132, 222, 77, 4, 201, 40]);
    /// ```
    fn serialize(self) -> PaletteSerial;
}
impl PaletteHelper for Palette {
    fn serialize(self) -> PaletteSerial {
        // initialize ret_arr using MaybeUninit
        {
            let mut ret_arr_uninit: MaybeUninit<PaletteSerial> = MaybeUninit::uninit();

            self.iter().enumerate().for_each(|(idx_row, row)| unsafe {
                addr_of_mut!((*ret_arr_uninit.as_mut_ptr())[idx_row * PALETTE_SIZE + 0])
                    .write_unaligned(row.0);
                addr_of_mut!((*ret_arr_uninit.as_mut_ptr())[idx_row * PALETTE_SIZE + 1])
                    .write_unaligned(row.1);
                addr_of_mut!((*ret_arr_uninit.as_mut_ptr())[idx_row * PALETTE_SIZE + 2])
                    .write_unaligned(row.2);
            });

            unsafe { transmute_copy(&ret_arr_uninit) }
        }
    }
}