use alloc::vec::Vec;

/// Ethereum-style blockies data of type `T`
///
/// An alias type of square 2d-array
///
/// # Generic Parameters
///
/// * `S` (const) - Size of blockies data (number of elements in both width and height)
///   * Equal to the `size` argument in other blockies implementation.
/// * `T` - Type of each element
pub type Blockies<const S: usize, T = ()> = [[T; S]; S];

/// Additional helper functions for generated [`Blockies`] objects
pub trait BlockiesHelper<const S: usize, T: Clone> {
    /// Size of [`Blockies`]
    ///
    /// Same as `const S` value (const generic)
    const SIZE: usize = S;

    #[doc(hidden)]
    #[deprecated(since = "1.1.0", note = "Use `BlockiesHelper::SIZE` instead")]
    /// (`width`, `height`) constants of [`Blockies`]
    const DIMENSION: (usize, usize) = (Self::SIZE, Self::SIZE);

    #[doc(hidden)]
    /// Create a new [`Blockies`] with given initialization function for each element
    ///
    /// # Arguments
    ///
    /// * `fn_init` - Initialization function, with following constraints:
    ///   * Arguments
    ///     * (`x`, `y`) - Coordinates, corresponding to the currently returned element
    ///   * Return
    ///     * An element of new [`Blockies`], for each (`x`, `y`)
    ///
    /// # Return
    ///
    /// * [`Blockies`], which has elements of return values from `fn_init`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// let coords_arr: Blockies<8, (u32, u32)> =
    ///     Blockies::new(|(x, y)| {
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
    fn new<F>(fn_init: F) -> Blockies<S, T>
    where
        F: FnMut((usize, usize)) -> T;

    #[doc(hidden)]
    /// Map a new [`Blockies`] from original [`Blockies`],
    /// with given initialization function for each element
    ///
    /// # Arguments
    ///
    /// * `fn_init` - Initialization function, with following constraints:
    ///   * Arguments
    ///     * `element` - Corresponding element of original [`Blockies`]
    ///                   for the given coordinates
    ///     * (`x`, `y`) - Coordinates, corresponding to the currently returned element
    ///   * Return
    ///     * A mapped element of new [`Blockies`], for each (`x`, `y`)
    ///
    /// # Return
    ///
    /// * [`Blockies`], which has elements of return values from `fn_init`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// let coords_arr: Blockies<8, (u32, u32)> = [
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
    /// let combined_idx_arr: Blockies<8, u32> =
    ///     coords_arr.map_2d(|orig_elem, (_x, _y)| {
    ///         orig_elem.1 * 8 + orig_elem.0
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
    fn map_2d<U: Clone, F>(&self, fn_init: F) -> Blockies<S, U>
    where
        F: FnMut(&T, (usize, usize)) -> U;

    #[doc(hidden)]
    /// Create a new [`Blockies`] from original [`Blockies`],
    /// with given initialization function
    /// (which gets reference of original 2d-array as an argument) for each element
    ///
    /// # Arguments
    ///
    /// * `fn_init` - Initialization function, with following constraints:
    ///   * Arguments
    ///     * `original_blockies` - Reference of original [`Blockies`]
    ///     * (`x`, `y`) - Coordinates, corresponding to the currently returned element
    ///   * Return
    ///     * A mapped element of new [`Blockies`], for each (`x`, `y`)
    ///
    /// # Return
    ///
    /// * [`Blockies`], which has elements of return values from `fn_init`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::*;
    ///
    /// let coords_arr: Blockies<8, (u32, u32)> = [
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
    /// let combined_idx_arr: Blockies<8, u32> =
    ///     coords_arr.map_2d_with_ref(|orig_2d_arr, (x, y)| {
    ///         orig_2d_arr[y][x].1 * 8 + orig_2d_arr[y][x].0
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
    fn map_2d_with_ref<U: Clone, F>(&self, fn_init: F) -> Blockies<S, U>
    where
        F: FnMut(&Blockies<S, T>, (usize, usize)) -> U;

    /// Flatten [`Blockies`] 2D data to 1D vector
    ///
    /// # Return
    ///
    /// * 1D vector of `T`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::{Blockies, BlockiesHelper};
    ///
    /// // original data (2d)
    /// let blockies_data_2d: Blockies<4, usize>  = [
    ///         [ 11, 12, 13, 14, ],
    ///         [ 21, 22, 23, 24, ],
    ///         [ 31, 32, 33, 34, ],
    ///         [ 41, 42, 43, 44, ],
    ///     ];
    ///
    /// // flatten data to 1d array
    /// let blockies_data_1d: Vec<usize> =
    ///     blockies_data_2d.flatten();
    ///
    /// assert_eq!(blockies_data_1d, vec![
    ///     11, 12, 13, 14, 21, 22, 23, 24,
    ///     31, 32, 33, 34, 41, 42, 43, 44,
    /// ]);
    /// ```
    fn flatten(self) -> Vec<T>;

    /// Scale [`Blockies`] data to given dimension
    ///
    /// * Note that this function does not perform any kind of pixel blending at edges.  
    ///   Therefore, lower dimension may generate unbalanced image,  
    ///   **only if both `(width, height)` are not multiples of [`SIZE`](Self::SIZE)**.
    ///
    /// # Arguments
    ///
    /// * `output_dim` - Width and height of output after scaling
    ///
    /// # Return
    ///
    /// * 2D vector of `T`
    ///
    /// # Example
    ///
    /// ```
    /// use eth_blockies::{Blockies, BlockiesHelper};
    ///
    /// // original data (2d)
    /// let blockies_data_4x4: Blockies<4, usize> = [
    ///         [ 11, 12, 13, 14, ],
    ///         [ 21, 22, 23, 24, ],
    ///         [ 31, 32, 33, 34, ],
    ///         [ 41, 42, 43, 44, ],
    ///     ];
    ///
    /// // scale: 4x4 -> 8x8
    /// let blockies_data_8x8: Vec<Vec<usize>> =
    ///     blockies_data_4x4.scale((8, 8));
    ///
    /// assert_eq!(blockies_data_8x8, vec![
    ///         vec![ 11, 11, 12, 12, 13, 13, 14, 14, ],
    ///         vec![ 11, 11, 12, 12, 13, 13, 14, 14, ],
    ///         vec![ 21, 21, 22, 22, 23, 23, 24, 24, ],
    ///         vec![ 21, 21, 22, 22, 23, 23, 24, 24, ],
    ///         vec![ 31, 31, 32, 32, 33, 33, 34, 34, ],
    ///         vec![ 31, 31, 32, 32, 33, 33, 34, 34, ],
    ///         vec![ 41, 41, 42, 42, 43, 43, 44, 44, ],
    ///         vec![ 41, 41, 42, 42, 43, 43, 44, 44, ],
    /// ]);
    /// ```
    fn scale(self, output_dim: (usize, usize)) -> Vec<Vec<T>>;
}
impl<T: Clone, const S: usize> BlockiesHelper<S, T> for Blockies<S, T> {
    fn new<F>(mut fn_init: F) -> Blockies<S, T>
    where
        F: FnMut((usize, usize)) -> T,
    {
        [[(); S]; S].map_2d_with_ref(|_, coord| fn_init(coord))
    }

    fn map_2d<U: Clone, F>(&self, mut fn_init: F) -> Blockies<S, U>
    where
        F: FnMut(&T, (usize, usize)) -> U,
    {
        Blockies::<S, U>::new(|(x, y)| fn_init(&self[y][x], (x, y)))
    }

    fn map_2d_with_ref<U: Clone, F>(&self, mut fn_init: F) -> Blockies<S, U>
    where
        F: FnMut(&Blockies<S, T>, (usize, usize)) -> U,
    {
        /*
        use core::mem::{transmute_copy, MaybeUninit};

        // This usage of uninit().assume_init() is safe
        //   because MaybeUninit does not have any initialization:
        // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
        let mut array_uninit: [[MaybeUninit<U>; S]; S] =
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
         */

        core::array::from_fn(|y| core::array::from_fn(|x| fn_init(self, (x, y))))
    }

    fn flatten(self) -> Vec<T> {
        // waiting for nightly-only feature 'generic_const_exprs' to be stable...
        // initialize ret_arr using MaybeUninit
        /*
        {
            let mut ret_arr_uninit: MaybeUninit<[T; 64]> = MaybeUninit::uninit();
            let ret_arr_ptr_casted: *mut T = ret_arr_uninit.as_mut_ptr().cast();

            self.iter().enumerate().for_each(|(idx_row, row)| {
                row.iter().enumerate().for_each(|(idx, elem)| unsafe {
                    ret_arr_ptr_casted
                        .add(idx_row * S + idx)
                        .write_unaligned(elem.clone());
                });
            });

            unsafe { ret_arr_uninit.assume_init() }.into()
        }
         */

        self.into_iter().flatten().collect()
    }

    fn scale(self, output_dim: (usize, usize)) -> Vec<Vec<T>> {
        // the base number of duplicates for each pixel
        let scale = (output_dim.0 / S, output_dim.1 / S);

        // if additional duplicates needed for each pixel
        let extra_elem_needed = {
            // calculate expected vs actual pixels for each step (== original pixel),
            //   then returns array that shows if each step needs extra elem
            fn calc_extra_elem_needed<const S: usize>(len: usize, scale: usize) -> [bool; S] {
                match len.overflowing_rem(S).0 == 0 {
                    true => [false; S],
                    false => {
                        let pixels_per_class = len as f64 / S as f64;
                        // calculate for each class element
                        (0..S)
                            .fold(
                                ([false; S], 0_isize),
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
                calc_extra_elem_needed::<S>(output_dim.0, scale.0),
                calc_extra_elem_needed::<S>(output_dim.1, scale.1),
            )
        };

        // template for vectors below
        let vec_template = (
            Vec::<T>::with_capacity(output_dim.0),      // for row
            Vec::<Vec<T>>::with_capacity(output_dim.1), // for 2d vec
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
