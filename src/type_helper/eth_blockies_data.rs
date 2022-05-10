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
/// This can be generated by performing [`serialize`](EthBlockiesSerializer::serialize)
/// on [`EthBlockies`] type.
pub type EthBlockiesSerial<T> = [T; BLOCKIES_SIZE * BLOCKIES_SIZE];

/// Trait for serializing 2D array
pub trait EthBlockiesSerializer<T: Clone> {
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
}
impl<T: Clone> EthBlockiesSerializer<T> for EthBlockies<T> {
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
