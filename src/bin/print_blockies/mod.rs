use std::{fs, io};

use crate::arg_parser::CmdOpt;
use crate::{
    arg_parser::OptList,
    bin_error::{BinError, BinResult},
    const_generic_call_mapper,
};
use eth_blockies::SeedInput;

// print as ascii ansi output
pub fn print_ansi(blockies_size: usize, seed: &[u8], opt_list: OptList) -> BinResult<()> {
    fn parse_opts(
        opt_list: OptList,
        default_blockies_size: usize,
    ) -> BinResult<(bool, (usize, usize), Option<String>)> {
        let mut opt_list = opt_list.clone();
        // Some( is_ascii, (dim-width, dim-height), Some(file-out-name) )
        Ok((
            // type
            (!opt_list.get_opt("ascii", false)?.0),
            // dimension (width, height)
            opt_list
                .get_opt("dimension", true)?
                .1
                .unwrap_or(format!("{0}x{0}", default_blockies_size)) // default
                .split_once('x')
                .and_then(|(w, h)| w.parse().ok().zip(h.parse().ok()))
                .ok_or(BinError::InvalidInput(format!(
                    "Invalid argument: Invalid ansi blockies dimension\n\
                     Type input in the form of '(width)x(height)'. (e.g. {0}x{0})",
                    default_blockies_size
                )))?,
            // name of file out
            opt_list.get_opt("outfile", true)?.1.map(String::from),
        ))
        // check if opt_list is empty. if not, abort parsing and return warning
        .and_then(|ret| opt_list.check_if_empty().map(|_| ret))
    }

    let (is_utf8, dimension, file_out) = parse_opts(opt_list, blockies_size)?;

    let ansi_string_data =
        const_generic_call_mapper::gen_ansiseq(blockies_size, seed, dimension, is_utf8).join("\n");

    use io::Write;
    match file_out {
        Some(file) => {
            fs::File::create(file).and_then(|mut f| f.write_all(ansi_string_data.as_bytes()))
        }
        None => io::stdout().write_fmt(format_args!("{}\n", ansi_string_data)),
    }
    .map_err(|e| BinError::InvalidInput(e.to_string()))?;

    Ok(())
}

// print as png format
pub fn print_image(blockies_size: usize, seed: &[u8], opt_list: OptList) -> BinResult<()> {
    fn parse_opts(opt_list: OptList) -> BinResult<(bool, (usize, usize), Option<String>)> {
        let mut opt_list = opt_list.clone();
        // Some( is_uncompressed_png, (dim-width, dim-height), Some(file-out-name) )
        Ok((
            // type
            {
                #[cfg(feature = "compressed_png")]
                {
                    !opt_list.get_opt("raw", false)?.0
                }
                #[cfg(not(feature = "compressed_png"))]
                {
                    false
                }
            },
            // dimension (width, height)
            opt_list
                .get_opt("dimension", true)?
                .1
                .as_deref()
                .unwrap_or("128x128") // default
                .split_once('x')
                .and_then(|(w, h)| w.parse().ok().zip(h.parse().ok()))
                .ok_or(BinError::InvalidInput(
                    "Invalid argument: Invalid image dimension\n\
                     Type input in the form of '(width)x(height)'. (e.g. 128x128)"
                        .to_owned(),
                ))?,
            // name of file out
            opt_list.get_opt("outfile", true)?.1.map(String::from),
        ))
        // check if opt_list is empty. if not, abort parsing and return warning
        .and_then(|ret| opt_list.check_if_empty().map(|_| ret))
    }

    let (is_compressed, dimension, file_out) = parse_opts(opt_list)?;

    let img_png_data = match is_compressed {
        true => {
            #[cfg(feature = "compressed_png")]
            fn call_png<I: SeedInput>(
                blockies_size: usize,
                seed: I,
                dimension: (usize, usize),
            ) -> Vec<u8> {
                const_generic_call_mapper::gen_comp_image(blockies_size, seed, dimension)
            }
            #[cfg(not(feature = "compressed_png"))]
            fn call_png<I: SeedInput>(
                blockies_size: usize,
                seed: I,
                dimension: (usize, usize),
            ) -> Vec<u8> {
                const_generic_call_mapper::gen_image(blockies_size, seed, dimension)
            }

            call_png(blockies_size, seed, dimension)
        }
        false => const_generic_call_mapper::gen_image(blockies_size, seed, dimension),
    };

    use io::Write;
    match file_out {
        Some(file) => fs::File::create(file).and_then(|mut f| f.write_all(&img_png_data)),
        None => io::stdout().write_all(&img_png_data),
    }
    .map_err(|e| BinError::InvalidInput(e.to_string()))?;

    Ok(())
}
