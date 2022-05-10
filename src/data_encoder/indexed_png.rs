use crate::{ColorClass, EthBlockies, Palette};

extern crate alloc;
use alloc::vec::Vec;

/// Convert indexed raw data to indexed png data
pub fn indexed_data_to_png(
    indexed_data: (Palette, EthBlockies<ColorClass>),
    dimension: (u32, u32),
) -> Vec<u8> {
    const PNG_HEADER: &[u8] = b"\x89PNG\r\n\x1a\n";
    const BIT_DEPTH: u8 = 2;

    let mut ret_data: Vec<u8> = Vec::from(PNG_HEADER);
    ret_data.append(&mut ihdr_chunk(dimension, BIT_DEPTH));
    ret_data.append(&mut plte_chunk(indexed_data.0));
    ret_data.append(&mut idat_chunk(indexed_data.1, dimension, BIT_DEPTH));
    ret_data.append(&mut iend_chunk());

    ret_data
}

/// Convert indexed raw data to indexed png data in base64 format
pub fn indexed_data_to_png_base64(
    indexed_data: (Palette, EthBlockies<ColorClass>),
    dimension: (u32, u32),
) -> Vec<u8> {
    base64(&indexed_data_to_png(indexed_data, dimension))
}

fn ihdr_chunk(dimension: (u32, u32), bit_depth: u8) -> Vec<u8> {
    const CHUNK_TYPE: &[u8] = b"IHDR";

    const COLOR_TYPE: u8 = 3;
    const COMPRESSION_METHOD: u8 = 0;
    const FILTER_METHOD: u8 = 0;
    const INTERLACE_METHOD: u8 = 0;

    let mut chunk_data: Vec<u8> = CHUNK_TYPE.to_vec();

    chunk_data.extend_from_slice(&dimension.0.to_be_bytes());
    chunk_data.extend_from_slice(&dimension.1.to_be_bytes());

    chunk_data.extend_from_slice(&bit_depth.to_be_bytes());
    chunk_data.extend_from_slice(&[
        COLOR_TYPE,
        COMPRESSION_METHOD,
        FILTER_METHOD,
        INTERLACE_METHOD,
    ]);

    pack_png_chunk(&mut chunk_data)
}

fn plte_chunk(palette: Palette) -> Vec<u8> {
    const CHUNK_TYPE: &[u8] = b"PLTE";

    let mut chunk_data: Vec<u8> = CHUNK_TYPE.to_vec();

    chunk_data.extend_from_slice(&palette.map(|(r, g, b)| [r, g, b]).concat());

    pack_png_chunk(&mut chunk_data)
}

fn idat_chunk(data: EthBlockies<ColorClass>, dimension: (u32, u32), bit_depth: u8) -> Vec<u8> {
    const CHUNK_TYPE: &[u8] = b"IDAT";
    const CM: u8 = 8;
    const CINFO: u8 = 7;
    const COMPRESSION_METHOD: &[u8] = &[CINFO << 4 | CM];
    const FCHECK: u8 = 1;
    const FDICT: u8 = 0;
    const FLEVEL: u8 = 0;
    const FLG: &[u8] = &[FLEVEL << 6 | FDICT << 5 | FCHECK];

    const IS_LAST_BLOCK: &[u8] = &[1];
    const FILTER_TYPE: &[u8] = &[0];
    let sample_per_byte: usize = (u8::BITS as u8 / bit_depth) as usize;

    let mut chunk_data: Vec<u8> = CHUNK_TYPE.to_vec();

    let scale = (
        dimension.0 as f64 / data[0].len() as f64,
        dimension.1 as f64 / data.len() as f64,
    );
    chunk_data.extend_from_slice(COMPRESSION_METHOD);
    chunk_data.extend_from_slice(FLG);
    chunk_data.extend_from_slice(IS_LAST_BLOCK);

    // build image block
    let mut img_data = (0..dimension.1).map(|scanline| scanline as f64).fold(
        Vec::<u8>::new(),
        |mut ret_vec, scanline| {
            ret_vec.extend_from_slice(FILTER_TYPE);

            ret_vec.append(
                &mut (0..dimension.0)
                    .collect::<Vec<u32>>()
                    .as_slice()
                    .chunks(sample_per_byte)
                    .map(|x_chunk| {
                        x_chunk
                            .iter()
                            // scale to new dimension
                            .map(|x| *x as f64)
                            .map(|pixel_i| {
                                data[(scanline / scale.1) as usize][(pixel_i / scale.0) as usize]
                            })
                            // pack to a byte
                            .map(|colorclass| colorclass as u8)
                            .enumerate()
                            .fold(0_u8, |byte, (i, class)| {
                                (class << (u8::BITS as u8 - bit_depth - (i as u8 * bit_depth)))
                                    | byte
                            })
                    })
                    .collect::<Vec<u8>>(),
            );
            ret_vec
        },
    );

    let img_data_len: u16 = (img_data.len()) as u16;
    chunk_data.extend_from_slice(&img_data_len.to_le_bytes());
    chunk_data.extend_from_slice(&(!img_data_len).to_le_bytes());

    img_data.extend_from_slice(&adler32(&img_data).to_be_bytes());
    chunk_data.append(&mut img_data);

    pack_png_chunk(&mut chunk_data)
}

fn iend_chunk() -> Vec<u8> {
    const CHUNK_TYPE: &[u8] = b"IEND";

    let mut chunk_data: Vec<u8> = CHUNK_TYPE.to_vec();

    pack_png_chunk(&mut chunk_data)
}

fn pack_png_chunk(chunk_data: &mut Vec<u8>) -> Vec<u8> {
    let crc_val = crc32(&chunk_data).to_be_bytes();
    let mut chunk: Vec<u8> = Vec::new();

    chunk.extend_from_slice(&(chunk_data.len() as u32 - 4).to_be_bytes());
    chunk.append(chunk_data);
    chunk.extend_from_slice(&crc_val);

    chunk
}

// http://www.libpng.org/pub/png/spec/1.2/PNG-CRCAppendix.html
fn crc32(buf: &[u8]) -> u32 {
    const CRC32_TABLE: &[u32; 256] = &[
        0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419, 0x706af48f, 0xe963a535,
        0x9e6495a3, 0x0edb8832, 0x79dcb8a4, 0xe0d5e91e, 0x97d2d988, 0x09b64c2b, 0x7eb17cbd,
        0xe7b82d07, 0x90bf1d91, 0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de, 0x1adad47d,
        0x6ddde4eb, 0xf4d4b551, 0x83d385c7, 0x136c9856, 0x646ba8c0, 0xfd62f97a, 0x8a65c9ec,
        0x14015c4f, 0x63066cd9, 0xfa0f3d63, 0x8d080df5, 0x3b6e20c8, 0x4c69105e, 0xd56041e4,
        0xa2677172, 0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b, 0x35b5a8fa, 0x42b2986c,
        0xdbbbc9d6, 0xacbcf940, 0x32d86ce3, 0x45df5c75, 0xdcd60dcf, 0xabd13d59, 0x26d930ac,
        0x51de003a, 0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599, 0xb8bda50f,
        0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924, 0x2f6f7c87, 0x58684c11, 0xc1611dab,
        0xb6662d3d, 0x76dc4190, 0x01db7106, 0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f,
        0x9fbfe4a5, 0xe8b8d433, 0x7807c9a2, 0x0f00f934, 0x9609a88e, 0xe10e9818, 0x7f6a0dbb,
        0x086d3d2d, 0x91646c97, 0xe6635c01, 0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e,
        0x6c0695ed, 0x1b01a57b, 0x8208f4c1, 0xf50fc457, 0x65b0d9c6, 0x12b7e950, 0x8bbeb8ea,
        0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65, 0x4db26158, 0x3ab551ce,
        0xa3bc0074, 0xd4bb30e2, 0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb, 0x4369e96a,
        0x346ed9fc, 0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5, 0xaa0a4c5f, 0xdd0d7cc9,
        0x5005713c, 0x270241aa, 0xbe0b1010, 0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409,
        0xce61e49f, 0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17, 0x2eb40d81,
        0xb7bd5c3b, 0xc0ba6cad, 0xedb88320, 0x9abfb3b6, 0x03b6e20c, 0x74b1d29a, 0xead54739,
        0x9dd277af, 0x04db2615, 0x73dc1683, 0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8,
        0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1, 0xf00f9344, 0x8708a3d2, 0x1e01f268,
        0x6906c2fe, 0xf762575d, 0x806567cb, 0x196c3671, 0x6e6b06e7, 0xfed41b76, 0x89d32be0,
        0x10da7a5a, 0x67dd4acc, 0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5, 0xd6d6a3e8,
        0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1, 0xa6bc5767, 0x3fb506dd, 0x48b2364b,
        0xd80d2bda, 0xaf0a1b4c, 0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef,
        0x4669be79, 0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236, 0xcc0c7795, 0xbb0b4703,
        0x220216b9, 0x5505262f, 0xc5ba3bbe, 0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7,
        0xb5d0cf31, 0x2cd99e8b, 0x5bdeae1d, 0x9b64c2b0, 0xec63f226, 0x756aa39c, 0x026d930a,
        0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713, 0x95bf4a82, 0xe2b87a14, 0x7bb12bae,
        0x0cb61b38, 0x92d28e9b, 0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21, 0x86d3d2d4, 0xf1d4e242,
        0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1, 0x18b74777, 0x88085ae6,
        0xff0f6a70, 0x66063bca, 0x11010b5c, 0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45,
        0xa00ae278, 0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7, 0x4969474d,
        0x3e6e77db, 0xaed16a4a, 0xd9d65adc, 0x40df0b66, 0x37d83bf0, 0xa9bcae53, 0xdebb9ec5,
        0x47b2cf7f, 0x30b5ffe9, 0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605,
        0xcdd70693, 0x54de5729, 0x23d967bf, 0xb3667a2e, 0xc4614ab8, 0x5d681b02, 0x2a6f2b94,
        0xb40bbe37, 0xc30c8ea1, 0x5a05df1b, 0x2d02ef8d,
    ];

    buf.iter().fold(0xffffffff_u32, |c, b| {
        CRC32_TABLE[((c ^ (*b as u32)) & 0xff) as usize] ^ (c >> 8)
    }) ^ 0xffffffff_u32
}

fn adler32(buf: &[u8]) -> u32 {
    const MOD_ADLER: u32 = 65521;
    const MAX_CHUNK: usize = 5552;

    let (a, b) = buf
        .chunks(MAX_CHUNK)
        .fold((1_u32, 0_u32), |(a, b), idx_data_tuple| {
            let (a_new, b_new) = idx_data_tuple
                .iter()
                .fold((a, b), |(a_inner, b_inner), elem| {
                    (a_inner + *elem as u32, a_inner + b_inner + *elem as u32)
                });

            (a_new % MOD_ADLER, b_new % MOD_ADLER)
        });

    b << 16 | a
}

fn base64(buf: &[u8]) -> Vec<u8> {
    const BASE64_TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    const BASE64_EMPTY: u8 = b'=';

    buf.chunks(3).fold(Vec::<u8>::new(), |mut vec, bytes| {
        let bits = u32::from_be_bytes([
            0_u8,
            *bytes.get(0).expect("base64"),
            *bytes.get(1).unwrap_or(&0_u8),
            *bytes.get(2).unwrap_or(&0_u8),
        ]);

        vec.extend_from_slice(&[
            BASE64_TABLE[(bits >> 18 & 0b111111_u32) as usize],
            BASE64_TABLE[(bits >> 12 & 0b111111_u32) as usize],
            match bytes.get(1).is_some() {
                true => BASE64_TABLE[(bits >> 6 & 0b111111_u32) as usize],
                false => BASE64_EMPTY,
            },
            match bytes.get(2).is_some() {
                true => BASE64_TABLE[(bits & 0b111111_u32) as usize],
                false => BASE64_EMPTY,
            },
        ]);
        vec
    })
}
