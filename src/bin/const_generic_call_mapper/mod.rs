use eth_blockies::*;

pub const MIN_BLOCKIES_SIZE: usize = 1;
pub const MAX_BLOCKIES_SIZE: usize = 32;

macro_rules! init_const_blockies_arr {

    // wrapper
    ( $func_name:ident ) => {
        init_const_blockies_arr!(@gen_arr
            $func_name, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
        )
    };

    // gen array of Blockies call, with given blockies_sizes (args)
    (@gen_arr $func_name:ident, $( $size:literal ), *) => {
        [
            $(
                &Blockies::<$size>::$func_name,
            )*
        ]
    };

}

pub fn gen_ansiseq<I: SeedInput>(
    blockies_size: usize,
    seed: I,
    dimension: (usize, usize),
    is_utf8: bool,
) -> Vec<String> {
    let func_list: [&dyn Fn(I, (usize, usize), bool) -> Vec<String>;
        MAX_BLOCKIES_SIZE - MIN_BLOCKIES_SIZE + 1] = init_const_blockies_arr!(ansiseq_data);

    func_list[blockies_size - 1](seed, dimension, is_utf8)
}

pub fn gen_image<I: SeedInput>(
    blockies_size: usize,
    seed: I,
    dimension: (usize, usize),
) -> Vec<u8> {
    let func_list: [&dyn Fn(I, (usize, usize)) -> Vec<u8>;
        MAX_BLOCKIES_SIZE - MIN_BLOCKIES_SIZE + 1] = init_const_blockies_arr!(png_data);

    func_list[blockies_size - 1](seed, dimension)
}

#[cfg(feature = "compressed_png")]
pub fn gen_comp_image<I: SeedInput>(
    blockies_size: usize,
    seed: I,
    dimension: (usize, usize),
) -> Vec<u8> {
    let func_list: [&dyn Fn(I, (usize, usize)) -> Vec<u8>;
        MAX_BLOCKIES_SIZE - MIN_BLOCKIES_SIZE + 1] = init_const_blockies_arr!(compressed_png_data);

    func_list[blockies_size - 1](seed, dimension)
}
