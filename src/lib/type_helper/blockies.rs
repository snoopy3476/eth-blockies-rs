use alloc::vec::Vec;
use core::mem::{transmute_copy, MaybeUninit};

/// Dimension of Ethereum-style blockies data
const BLOCKIES_SIZE: usize = 8;

/// Ethereum-style blockies data of type `T`
pub type EthBlockies<T> = EthBlockiesRowList<EthBlockiesRow<T>>;
/// Ethereum-style blockies column (line) data with type `T`
type EthBlockiesRowList<T> = [T; BLOCKIES_SIZE];
/// Ethereum-style blockies row data with type `T`
type EthBlockiesRow<T> = [T; BLOCKIES_SIZE];
/// Ethereum-style blockies flattened data of type `T`
///
/// This can be generated by performing [`flatten`](BlockiesHelper::flatten)
/// on [`EthBlockies`] type.
pub type FlatEthBlockies<T> = [T; BLOCKIES_SIZE * BLOCKIES_SIZE];

/// Trait for [`EthBlockies`]
pub trait BlockiesHelper<T: Clone> {
    /// (`width`, `height`) constants of [`EthBlockies`]
    const DIMENSION: (usize, usize) = (BLOCKIES_SIZE, BLOCKIES_SIZE);

    /// Create a new [`EthBlockies`] with given initialization function for each element
    ///
    /// # Arguments
    ///
    /// * `fn_init` - Initialization function, with following constraints:
    ///   * Arguments
    ///     * (`x`, `y`) - Coordinates, corresponding to the currently returned element
    ///   * Return
    ///     * An element of new [`EthBlockies`], for each (`x`, `y`)
    ///
    /// # Return
    ///
    /// * [`EthBlockies`], which has elements of return values from `fn_init`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// let coords_arr: EthBlockies<(u32, u32)> =
    ///     EthBlockies::new(|(x, y)| {
    ///         (x as u32, y as u32)
    ///     });
    ///
    /// assert_eq!(coords_arr, [
    ///     [ (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0) ],
    ///     [ (0,1), (1,1), (2,1), (3,1), (4,1), (5,1), (6,1), (7,1) ],
    ///     [ (0,2), (1,2), (2,2), (3,2), (4,2), (5,2), (6,2), (7,2) ],
    ///     [ (0,3), (1,3), (2,3), (3,3), (4,3), (5,3), (6,3), (7,3) ],
    ///     [ (0,4), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4) ],
    ///     [ (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (6,5), (7,5) ],
    ///     [ (0,6), (1,6), (2,6), (3,6), (4,6), (5,6), (6,6), (7,6) ],
    ///     [ (0,7), (1,7), (2,7), (3,7), (4,7), (5,7), (6,7), (7,7) ],
    /// ]);
    /// ```
    fn new<F>(fn_init: F) -> EthBlockies<T>
    where
        F: FnMut((usize, usize)) -> T;

    /// Map a new [`EthBlockies`] from original [`EthBlockies`],
    /// with given initialization function for each element
    ///
    /// # Arguments
    ///
    /// * `fn_init` - Initialization function, with following constraints:
    ///   * Arguments
    ///     * `element` - Corresponding element of original [`EthBlockies`]
    ///                   for the given coordinates
    ///     * (`x`, `y`) - Coordinates, corresponding to the currently returned element
    ///   * Return
    ///     * A mapped element of new [`EthBlockies`], for each (`x`, `y`)
    ///
    /// # Return
    ///
    /// * [`EthBlockies`], which has elements of return values from `fn_init`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// let coords_arr: EthBlockies<(u32, u32)> = [
    ///     [ (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0) ],
    ///     [ (0,1), (1,1), (2,1), (3,1), (4,1), (5,1), (6,1), (7,1) ],
    ///     [ (0,2), (1,2), (2,2), (3,2), (4,2), (5,2), (6,2), (7,2) ],
    ///     [ (0,3), (1,3), (2,3), (3,3), (4,3), (5,3), (6,3), (7,3) ],
    ///     [ (0,4), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4) ],
    ///     [ (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (6,5), (7,5) ],
    ///     [ (0,6), (1,6), (2,6), (3,6), (4,6), (5,6), (6,6), (7,6) ],
    ///     [ (0,7), (1,7), (2,7), (3,7), (4,7), (5,7), (6,7), (7,7) ],
    /// ];
    ///
    /// let combined_idx_arr: EthBlockies<u32> =
    ///     coords_arr.map_2d(|orig_elem, (_x, _y)| {
    ///         orig_elem.1 * EthBlockies::<u32>::DIMENSION.1 as u32
    ///             + orig_elem.0
    ///     });
    ///
    ///
    /// assert_eq!(combined_idx_arr, [
    ///     [   0,   1,   2,   3,   4,   5,   6,   7 ],
    ///     [   8,   9,  10,  11,  12,  13,  14,  15 ],
    ///     [  16,  17,  18,  19,  20,  21,  22,  23 ],
    ///     [  24,  25,  26,  27,  28,  29,  30,  31 ],
    ///     [  32,  33,  34,  35,  36,  37,  38,  39 ],
    ///     [  40,  41,  42,  43,  44,  45,  46,  47 ],
    ///     [  48,  49,  50,  51,  52,  53,  54,  55 ],
    ///     [  56,  57,  58,  59,  60,  61,  62,  63 ],
    /// ]);
    /// ```
    fn map_2d<U: Clone, F>(&self, fn_init: F) -> EthBlockies<U>
    where
        F: FnMut(&T, (usize, usize)) -> U;

    /// Create a new [`EthBlockies`] from original [`EthBlockies`],
    /// with given initialization function
    /// (which gets reference of original 2d-array as an argument) for each element
    ///
    /// # Arguments
    ///
    /// * `fn_init` - Initialization function, with following constraints:
    ///   * Arguments
    ///     * `original_blockies` - Reference of original [`EthBlockies`]
    ///     * (`x`, `y`) - Coordinates, corresponding to the currently returned element
    ///   * Return
    ///     * A mapped element of new [`EthBlockies`], for each (`x`, `y`)
    ///
    /// # Return
    ///
    /// * [`EthBlockies`], which has elements of return values from `fn_init`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// let coords_arr: EthBlockies<(u32, u32)> = [
    ///     [ (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0) ],
    ///     [ (0,1), (1,1), (2,1), (3,1), (4,1), (5,1), (6,1), (7,1) ],
    ///     [ (0,2), (1,2), (2,2), (3,2), (4,2), (5,2), (6,2), (7,2) ],
    ///     [ (0,3), (1,3), (2,3), (3,3), (4,3), (5,3), (6,3), (7,3) ],
    ///     [ (0,4), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4) ],
    ///     [ (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (6,5), (7,5) ],
    ///     [ (0,6), (1,6), (2,6), (3,6), (4,6), (5,6), (6,6), (7,6) ],
    ///     [ (0,7), (1,7), (2,7), (3,7), (4,7), (5,7), (6,7), (7,7) ],
    /// ];
    ///
    /// let combined_idx_arr: EthBlockies<u32> =
    ///     coords_arr.map_2d_with_ref(|orig_2d_arr, (x, y)| {
    ///         orig_2d_arr[y][x].1 * EthBlockies::<u32>::DIMENSION.1 as u32
    ///             + orig_2d_arr[y][x].0
    ///     });
    ///
    ///
    /// assert_eq!(combined_idx_arr, [
    ///     [   0,   1,   2,   3,   4,   5,   6,   7 ],
    ///     [   8,   9,  10,  11,  12,  13,  14,  15 ],
    ///     [  16,  17,  18,  19,  20,  21,  22,  23 ],
    ///     [  24,  25,  26,  27,  28,  29,  30,  31 ],
    ///     [  32,  33,  34,  35,  36,  37,  38,  39 ],
    ///     [  40,  41,  42,  43,  44,  45,  46,  47 ],
    ///     [  48,  49,  50,  51,  52,  53,  54,  55 ],
    ///     [  56,  57,  58,  59,  60,  61,  62,  63 ],
    /// ]);
    /// ```
    fn map_2d_with_ref<U: Clone, F>(&self, fn_init: F) -> EthBlockies<U>
    where
        F: FnMut(&EthBlockies<T>, (usize, usize)) -> U;

    /// Flatten 2D array to 1D array
    ///
    /// # Return
    ///
    /// * 1D array of `T`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    /// const P: RgbPalette = [(38, 173, 52), (132, 222, 77), (4, 201, 40)];
    ///
    /// // original data (2d)
    /// let blockies_data_rgb_2d: EthBlockies<RgbPixel> = [
    ///         [ P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1] ],
    ///         [ P[1], P[0], P[0], P[2], P[2], P[0], P[0], P[1] ],
    ///         [ P[2], P[1], P[1], P[0], P[0], P[1], P[1], P[2] ],
    ///         [ P[0], P[0], P[2], P[0], P[0], P[2], P[0], P[0] ],
    ///         [ P[1], P[0], P[1], P[2], P[2], P[1], P[0], P[1] ],
    ///         [ P[1], P[2], P[1], P[2], P[2], P[1], P[2], P[1] ],
    ///         [ P[0], P[2], P[1], P[2], P[2], P[1], P[2], P[0] ],
    ///         [ P[1], P[0], P[0], P[1], P[1], P[0], P[0], P[1] ],
    ///     ];
    ///
    /// // flatten data to 1d array
    /// let blockies_data_rgb_1d: FlatEthBlockies<RgbPixel>
    ///     = blockies_data_rgb_2d.flatten();
    ///
    /// assert_eq!(blockies_data_rgb_1d, [
    ///     P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1],
    ///     P[1], P[0], P[0], P[2], P[2], P[0], P[0], P[1],
    ///     P[2], P[1], P[1], P[0], P[0], P[1], P[1], P[2],
    ///     P[0], P[0], P[2], P[0], P[0], P[2], P[0], P[0],
    ///     P[1], P[0], P[1], P[2], P[2], P[1], P[0], P[1],
    ///     P[1], P[2], P[1], P[2], P[2], P[1], P[2], P[1],
    ///     P[0], P[2], P[1], P[2], P[2], P[1], P[2], P[0],
    ///     P[1], P[0], P[0], P[1], P[1], P[0], P[0], P[1],
    /// ]);
    /// ```
    fn flatten(self) -> FlatEthBlockies<T>;

    /// Scale `(8, 8)` 2d array to given dimension
    ///
    /// * Note that this function does not perform any kind of pixel blending at edges.  
    ///   Therefore, dimension lower than `(100, 100)` may generate unbalanced image **if both `(width, height)` are not multiples of 8**.
    ///
    /// # Return
    /// * 2D vector of `T`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    /// const P: RgbPalette = [(38, 173, 52), (132, 222, 77), (4, 201, 40)];
    ///
    /// // original data (2d)
    /// let blockies_data_rgb: EthBlockies<RgbPixel> = [
    ///         [ P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1] ],
    ///         [ P[1], P[0], P[0], P[2], P[2], P[0], P[0], P[1] ],
    ///         [ P[2], P[1], P[1], P[0], P[0], P[1], P[1], P[2] ],
    ///         [ P[0], P[0], P[2], P[0], P[0], P[2], P[0], P[0] ],
    ///         [ P[1], P[0], P[1], P[2], P[2], P[1], P[0], P[1] ],
    ///         [ P[1], P[2], P[1], P[2], P[2], P[1], P[2], P[1] ],
    ///         [ P[0], P[2], P[1], P[2], P[2], P[1], P[2], P[0] ],
    ///         [ P[1], P[0], P[0], P[1], P[1], P[0], P[0], P[1] ],
    ///     ];
    ///
    /// // scale: 8x8 -> 16x16
    /// let blockies_data_rgb_scaled: Vec<Vec<RgbPixel>>
    ///     = blockies_data_rgb.scale((16, 16));
    ///
    /// assert_eq!(blockies_data_rgb_scaled, [ [
    ///         P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1],
    ///         P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1],
    ///     ], [
    ///         P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1],
    ///         P[1], P[1], P[1], P[1], P[1], P[1], P[1], P[1],
    ///     ], [
    ///         P[1], P[1], P[0], P[0], P[0], P[0], P[2], P[2],
    ///         P[2], P[2], P[0], P[0], P[0], P[0], P[1], P[1],
    ///     ], [
    ///         P[1], P[1], P[0], P[0], P[0], P[0], P[2], P[2],
    ///         P[2], P[2], P[0], P[0], P[0], P[0], P[1], P[1],
    ///     ], [
    ///         P[2], P[2], P[1], P[1], P[1], P[1], P[0], P[0],
    ///         P[0], P[0], P[1], P[1], P[1], P[1], P[2], P[2],
    ///     ], [
    ///         P[2], P[2], P[1], P[1], P[1], P[1], P[0], P[0],
    ///         P[0], P[0], P[1], P[1], P[1], P[1], P[2], P[2],
    ///     ], [
    ///         P[0], P[0], P[0], P[0], P[2], P[2], P[0], P[0],
    ///         P[0], P[0], P[2], P[2], P[0], P[0], P[0], P[0],
    ///     ], [
    ///         P[0], P[0], P[0], P[0], P[2], P[2], P[0], P[0],
    ///         P[0], P[0], P[2], P[2], P[0], P[0], P[0], P[0],
    ///     ], [
    ///         P[1], P[1], P[0], P[0], P[1], P[1], P[2], P[2],
    ///         P[2], P[2], P[1], P[1], P[0], P[0], P[1], P[1],
    ///     ], [
    ///         P[1], P[1], P[0], P[0], P[1], P[1], P[2], P[2],
    ///         P[2], P[2], P[1], P[1], P[0], P[0], P[1], P[1],
    ///     ], [
    ///         P[1], P[1], P[2], P[2], P[1], P[1], P[2], P[2],
    ///         P[2], P[2], P[1], P[1], P[2], P[2], P[1], P[1],
    ///     ], [
    ///         P[1], P[1], P[2], P[2], P[1], P[1], P[2], P[2],
    ///         P[2], P[2], P[1], P[1], P[2], P[2], P[1], P[1],
    ///     ], [
    ///         P[0], P[0], P[2], P[2], P[1], P[1], P[2], P[2],
    ///         P[2], P[2], P[1], P[1], P[2], P[2], P[0], P[0],
    ///     ], [
    ///         P[0], P[0], P[2], P[2], P[1], P[1], P[2], P[2],
    ///         P[2], P[2], P[1], P[1], P[2], P[2], P[0], P[0],
    ///     ], [
    ///         P[1], P[1], P[0], P[0], P[0], P[0], P[1], P[1],
    ///         P[1], P[1], P[0], P[0], P[0], P[0], P[1], P[1],
    ///     ], [
    ///         P[1], P[1], P[0], P[0], P[0], P[0], P[1], P[1],
    ///         P[1], P[1], P[0], P[0], P[0], P[0], P[1], P[1],
    ///     ], ]
    /// );
    /// ```
    fn scale(self, dimension: (usize, usize)) -> Vec<Vec<T>>;
}
impl<T: Clone> BlockiesHelper<T> for EthBlockies<T> {
    fn new<F>(mut fn_init: F) -> EthBlockies<T>
    where
        F: FnMut((usize, usize)) -> T,
    {
        [[0_u8; BLOCKIES_SIZE]; BLOCKIES_SIZE].map_2d_with_ref(|_, coord| fn_init(coord))
    }

    fn map_2d<U: Clone, F>(&self, mut fn_init: F) -> EthBlockies<U>
    where
        F: FnMut(&T, (usize, usize)) -> U,
    {
        EthBlockies::<U>::new(|(x, y)| fn_init(&self[y][x], (x, y)))
    }

    fn map_2d_with_ref<U: Clone, F>(&self, mut fn_init: F) -> EthBlockies<U>
    where
        F: FnMut(&EthBlockies<T>, (usize, usize)) -> U,
    {
        // This usage of uninit().assume_init() is safe
        //   because MaybeUninit does not have any initialization:
        // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut array_uninit: EthBlockies<MaybeUninit<U>> =
            unsafe { MaybeUninit::uninit().assume_init() };

        array_uninit
            .iter_mut()
            .enumerate()
            .for_each(|(y, row_uninit)| {
                row_uninit
                    .iter_mut()
                    .enumerate()
                    .for_each(|(x, elem_uninit)| {
                        elem_uninit.write(fn_init(self, (x, y)));
                    });
            });

        unsafe { transmute_copy(&array_uninit) }
    }

    // In many cases, this func is expected to have zero-impact
    //   after compilation & optimization
    fn flatten(self) -> FlatEthBlockies<T> {
        // initialize ret_arr using MaybeUninit
        {
            let mut ret_arr_uninit: MaybeUninit<FlatEthBlockies<T>> = MaybeUninit::uninit();
            let ret_arr_ptr_casted: *mut T = ret_arr_uninit.as_mut_ptr().cast();

            self.iter().enumerate().for_each(|(idx_row, row)| {
                row.iter().enumerate().for_each(|(idx, elem)| unsafe {
                    ret_arr_ptr_casted
                        .add(idx_row * BLOCKIES_SIZE + idx)
                        .write_unaligned(elem.clone());
                });
            });

            unsafe { ret_arr_uninit.assume_init() }
        }
    }

    fn scale(self, dimension: (usize, usize)) -> Vec<Vec<T>> {
        // the base number of duplicates for each pixel
        let scale = (dimension.0 / BLOCKIES_SIZE, dimension.1 / BLOCKIES_SIZE);

        // if additional duplicates needed for each pixel
        let extra_elem_needed = {
            // calculate expected vs actual pixels for each step (== original pixel),
            //   then returns array that shows if each step needs extra elem
            fn calc_extra_elem_needed(len: usize, scale: usize) -> [bool; BLOCKIES_SIZE] {
                match len.overflowing_rem(BLOCKIES_SIZE).0 == 0 {
                    true => [false; BLOCKIES_SIZE],
                    false => {
                        let pixels_per_class = len as f64 / BLOCKIES_SIZE as f64;
                        // calculate for each class element
                        (0..BLOCKIES_SIZE)
                            .fold(
                                ([false; BLOCKIES_SIZE], 0_isize),
                                |(mut is_extra_needed, pixels_diff_prev), idx| {
                                    // if [pixels_diff] => (
                                    //   rounded difference of
                                    //   'expected ending pixel for current elem'
                                    //   and
                                    //   'ending pixel when duplicate
                                    //    each elem by factor of [scale]'
                                    // )
                                    // changes, current class elem needs extra elem

                                    let pixels_diff = ((pixels_per_class - scale as f64)
                                        * (idx + 1) as f64
                                        + 0.5_f64)
                                        as isize;

                                    is_extra_needed[idx] = pixels_diff != pixels_diff_prev;

                                    (is_extra_needed, pixels_diff)
                                },
                            )
                            .0
                    }
                }
            }

            (
                calc_extra_elem_needed(dimension.0, scale.0),
                calc_extra_elem_needed(dimension.1, scale.1),
            )
        };

        // template for vectors below
        let vec_template = (
            Vec::<T>::with_capacity(dimension.0),      // for row
            Vec::<Vec<T>>::with_capacity(dimension.1), // for 2d vec
        );

        // build scaled 2d vec
        self.iter()
            .enumerate()
            .fold(vec_template.1, |mut ret_vec, (idx_row, row)| {
                // build a scaled row for the current source row
                let new_row: Vec<T> = row.iter().enumerate().fold(
                    vec_template.0.clone(),
                    |mut ret_row, (idx, elem)| {
                        // duplicate n elems at the end of the row
                        // (n: scale.0 + extra_elem_needed.0[idx])
                        ret_row.resize(
                            ret_row.len() + scale.0 + extra_elem_needed.0[idx] as usize,
                            elem.clone(),
                        );
                        ret_row
                    },
                );

                // duplicate new rows by n, and append at the end
                // (n: scale.1 + extra_elem_needed.1[idx])
                ret_vec.resize(
                    ret_vec.len() + scale.1 + extra_elem_needed.1[idx_row] as usize,
                    new_row.clone(),
                );

                ret_vec
            })
    }
}
