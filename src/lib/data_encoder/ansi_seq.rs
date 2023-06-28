use crate::global_type_helper::{ColorClass, Palette, RgbPalette, RgbPixel};
use alloc::{string::String, vec, vec::Vec};

const COLOR_RESET_BYTES: usize = 4;

const COLOR_SEQ_ASCII_BYTES: usize = 19;
const BLOCK_BYTES_ASCII: usize = 2;
const COLOR_BLOCK_ASCII_BYTES: usize = COLOR_SEQ_ASCII_BYTES + BLOCK_BYTES_ASCII;
type UnitBlockAscii = [u8; BLOCK_BYTES_ASCII];

const COLOR_SEQ_UNICODE_BYTES: usize = 38;
const BLOCK_BYTES_UNICODE: usize = 3;
const COLOR_BLOCK_UNICODE_BYTES: usize = COLOR_SEQ_UNICODE_BYTES + BLOCK_BYTES_UNICODE;
type UnitBlockUnicode = [u8; BLOCK_BYTES_UNICODE];

/// Convert indexed raw data to ansiseq ascii string
pub fn indexed_data_to_ansiseq_ascii(
    indexed_palette: RgbPalette,
    indexed_bitmap: Vec<Vec<ColorClass>>,
    dimension: (usize, usize),
) -> Vec<String> {
    const ASCII_BLOCK: &UnitBlockAscii = b"  ";

    // get palette sequence for each possible color
    let palette_sequence: Palette<[u8; COLOR_BLOCK_ASCII_BYTES]> =
        indexed_palette.map(|palette_color| color_block_ascii(Some(palette_color), ASCII_BLOCK));

    // get output for each line
    indexed_bitmap
        .iter()
        .map(|row| {
            let blocks_part_bytes = COLOR_BLOCK_ASCII_BYTES * dimension.0;
            let mut row_arr = vec![0_u8; blocks_part_bytes + COLOR_RESET_BYTES];

            let (row_arr_blocks, row_arr_reset) = row_arr.split_at_mut(blocks_part_bytes);

            row.iter()
                .zip(row_arr_blocks.chunks_mut(COLOR_BLOCK_ASCII_BYTES))
                .for_each(|(class, row_arr_chunk)| {
                    row_arr_chunk.copy_from_slice(&palette_sequence[class]);
                });

            row_arr_reset.copy_from_slice(&color_reset());

            // using unchecked ver, as built string is always valid ascii (utf8)
            unsafe { String::from_utf8_unchecked(row_arr) }
            // String::from_utf8(row_arr).expect("unexpected internal error")
        })
        .collect()
}

/// Convert indexed raw data to ansiseq utf8 string
pub fn indexed_data_to_ansiseq_utf8(
    indexed_palette: RgbPalette,
    indexed_bitmap: Vec<Vec<ColorClass>>,
    dimension: (usize, usize),
) -> Vec<String> {
    const UTF8_BLOCK_LOWER: &UnitBlockUnicode = b"\xe2\x96\x84";
    const UTF8_BLOCK_UPPER: &UnitBlockUnicode = b"\xe2\x96\x80";

    // ensure that above chars are valid utf8
    alloc::str::from_utf8(UTF8_BLOCK_LOWER).expect("unexpected internal error");
    alloc::str::from_utf8(UTF8_BLOCK_UPPER).expect("unexpected internal error");

    // get palette sequence that is used when both upper/lower block parts exist in a line
    let palette_sequence: Palette<Palette<[u8; COLOR_BLOCK_UNICODE_BYTES]>> =
        indexed_palette.map(|palette_color_upper| {
            indexed_palette.map(|palette_color_lower| {
                color_block_unicode(
                    Some(palette_color_lower),
                    Some(palette_color_upper),
                    UTF8_BLOCK_LOWER,
                )
            })
        });
    // get palette sequence that is used when only upper block parts exist in a line
    let palette_sequence_upperonly: Palette<[u8; COLOR_BLOCK_UNICODE_BYTES]> =
        indexed_palette.map(|palette_color_upper| {
            color_block_unicode(Some(palette_color_upper), None, UTF8_BLOCK_UPPER)
        });

    // try chunk two lines, and output as one packed line
    indexed_bitmap
        .chunks(2)
        .map(|row_chunks| {
            let blocks_part_bytes = COLOR_BLOCK_UNICODE_BYTES * dimension.0;
            let mut row_arr = vec![0_u8; blocks_part_bytes + COLOR_RESET_BYTES];

            let (row_arr_blocks, row_arr_reset) = row_arr.split_at_mut(blocks_part_bytes);
            let row_arr_blocks_chunks = row_arr_blocks.chunks_mut(COLOR_BLOCK_UNICODE_BYTES);

            match row_chunks.get(1) {
                // if both upper & lower line exists
                Some(row_lower) => {
                    row_chunks[0]
                        .iter()
                        .zip(row_lower.iter())
                        .zip(row_arr_blocks_chunks)
                        .for_each(|((class_upper, class_lower), row_arr_chunk)| {
                            row_arr_chunk
                                .copy_from_slice(&palette_sequence[class_upper][class_lower]);
                        });
                }
                // if only upper line exists
                None => {
                    row_chunks[0].iter().zip(row_arr_blocks_chunks).for_each(
                        |(class_upper, row_arr_chunk)| {
                            row_arr_chunk.copy_from_slice(&palette_sequence_upperonly[class_upper]);
                        },
                    );
                }
            };

            row_arr_reset.copy_from_slice(&color_reset());

            // using unchecked ver, as built string is always valid utf8
            unsafe { String::from_utf8_unchecked(row_arr) }
            // String::from_utf8(row_arr).expect("unexpected internal error")
        })
        .collect()
}

#[inline(always)]
fn color_block_ascii(
    bgcolor: Option<RgbPixel>,
    block: &UnitBlockAscii,
) -> [u8; COLOR_BLOCK_ASCII_BYTES] {
    let mut ret_arr = [0_u8; COLOR_BLOCK_ASCII_BYTES];

    let (bg_part, block_part) = ret_arr.split_at_mut(19);

    bgcolor.map(|bg| {
        write_color_ansiseq_to_bytebuf(bg_part, bg, false);
    });
    block_part.copy_from_slice(block);

    ret_arr
}

#[inline(always)]
fn color_block_unicode(
    fgcolor: Option<RgbPixel>,
    bgcolor: Option<RgbPixel>,
    block: &UnitBlockUnicode,
) -> [u8; COLOR_BLOCK_UNICODE_BYTES] {
    let mut ret_arr = [0_u8; COLOR_BLOCK_UNICODE_BYTES];

    let (fg_part, rem) = ret_arr.split_at_mut(19);
    let (bg_part, block_part) = rem.split_at_mut(19);

    fgcolor.map(|fg| {
        write_color_ansiseq_to_bytebuf(fg_part, fg, true);
    });
    bgcolor.map(|bg| {
        write_color_ansiseq_to_bytebuf(bg_part, bg, false);
    });
    block_part.copy_from_slice(block);

    ret_arr
}

#[inline(always)]
fn color_reset() -> [u8; COLOR_RESET_BYTES] {
    *b"\x1b[0m"
}

#[inline(always)]
fn write_u8_to_bytebuf(buf: &mut [u8], num: u8) {
    buf[2] = num % 10 + b'0';
    let num = num / 10;
    buf[1] = num % 10 + b'0';
    let num = num / 10;
    buf[0] = num % 10 + b'0';
}

#[inline(always)]
fn write_color_ansiseq_to_bytebuf(buf: &mut [u8], rgb: RgbPixel, is_fg: bool) {
    buf.copy_from_slice(match is_fg {
        true => b"\x1b[38;2;000;000;000m",
        false => b"\x1b[48;2;000;000;000m",
    });

    let (r_buf, rem) = buf.split_at_mut(7).1.split_at_mut(3);
    write_u8_to_bytebuf(r_buf, rgb.0);
    let (g_buf, rem) = rem.split_at_mut(1).1.split_at_mut(3);
    write_u8_to_bytebuf(g_buf, rgb.1);
    let (b_buf, _rem) = rem.split_at_mut(1).1.split_at_mut(3);
    write_u8_to_bytebuf(b_buf, rgb.2);
}
