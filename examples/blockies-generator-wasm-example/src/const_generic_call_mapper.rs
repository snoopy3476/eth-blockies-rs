use eth_blockies::*;

pub const MIN_RESOLUTION: usize = 5;
pub const MAX_RESOLUTION: usize = 16;
pub const RESOLUTION_RANGE: core::ops::RangeInclusive<usize> = MIN_RESOLUTION..=MAX_RESOLUTION;

macro_rules! init_const_blockies_arr {

    // wrapper
    ( $func_name:ident ) => {
        init_const_blockies_arr!(@gen_arr
            $func_name, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        )
    };

    // gen array of Blockies call, with given resolutions (args)
    (@gen_arr $func_name:ident, $( $resol:literal ), *) => {
        [
            $(
                &Blockies::<$resol>::$func_name,
            )*
        ]
    };

}

pub type CallArr<S> =
    [&'static dyn Fn(S, (usize, usize), bool) -> String; MAX_RESOLUTION - MIN_RESOLUTION + 1];

pub const fn init_const_blockies_arr<S: SeedInput>() -> CallArr<S> {
    init_const_blockies_arr!(compressed_png_data_base64)
}
