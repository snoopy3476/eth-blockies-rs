use eth_blockies::SeedInput;

use std::env;

mod arg_parser;
use arg_parser::*;
mod bin_error;
use bin_error::BinError;
use print_blockies::{print_ansi, print_image};
mod const_generic_call_mapper;
use const_generic_call_mapper::{MAX_BLOCKIES_SIZE, MIN_BLOCKIES_SIZE};
mod print_blockies;

fn main() {
    match {
        parse_args()
            // run main routine
            .and_then(|(arg_list, mut opt_list)| {
                let mut arg_list = arg_list.iter();

                // help message option
                match opt_list.get_opt("help", false)?.0 {
                    true => Err(BinError::HelpArgument),
                    false => Ok(()),
                }
                // initialize seed
                .and_then(
                    |()| match (arg_list.next(), opt_list.get_opt("ethseed", false)?.0) {
                        (Some(seed), false) => Ok(seed.as_bytes().to_vec()),
                        (Some(seed), true) => Ok(seed.to_ethaddr_seed().to_vec()),
                        (None, _) => Err(BinError::NoArgument),
                    },
                )
                // blockies size
                .and_then(|seed| {
                    Ok((
                        seed,
                        opt_list
                            .get_opt("size", true)?
                            .1
                            .unwrap_or(format!("{0}", 8)) // default
                            .parse()
                            .ok()
                            .filter(|s| (MIN_BLOCKIES_SIZE..=MAX_BLOCKIES_SIZE).contains(s))
                            .ok_or(BinError::InvalidInput(format!(
                                "Invalid argument: Invalid blockies size\n\
                                     Enter integer in range {}-{}.",
                                MIN_BLOCKIES_SIZE, MAX_BLOCKIES_SIZE
                            )))?,
                    ))
                })
                // select output format
                .and_then(|(seed, blockies_size)| {
                    match arg_list.next().map(String::as_str) {
                        Some("ansi") | Some("a") | None => {
                            print_ansi(blockies_size, &seed, opt_list)
                        } // default
                        Some("image") | Some("i") => print_image(blockies_size, &seed, opt_list),
                        Some(input) => Err(BinError::InvalidInput(format!(
                            "Invalid argument: Invalid output format: '{}'\n\
                                 Use either 'ansi' or 'image' here.",
                            input
                        ))),
                    }
                })
            })
            // print message if error
            .map_err(|e| {
                match e {
                    BinError::NoArgument => {
                        eprintln!("{}", bin_usage());
                    }
                    BinError::HelpArgument => {
                        eprintln!("{}", help_msg());
                    }
                    BinError::InvalidInput(err_msg) => {
                        eprintln!(
                            "{}\n\n* For detailed usages of this binary, \
                                 run this binary with -h (--help) option.",
                            err_msg
                        )
                    }
                };
            })
    } {
        // return exit code
        Ok(_) => std::process::exit(0),
        _ => std::process::exit(1),
    };
}

// print help message
fn help_msg() -> String {
    format!(
        "{pkg_name} v{pkg_ver} {feature_variation}\n\
         A simple tool to generate Ethereum blockies (Ethereum-style blocky identicon)\n\
         Written by {pkg_author}\n\
         License: {pkg_license}\n\n\n\
         {bin_usage}\n\n\
         <seed>                 Seed to generate blockies (e.g. Ethereum wallet address)\n\n\
         [output-fmt]  - ansi   (Default) Generate ansi sequence of blockies,\n\
         \0                       usually for printing to terminal\n\
         \0              - image  Generate png image data of blockies\n\n\
         [OPTIONS...]:\n\n\
         \t-e --ethseed   Interpret seed string as Ethereum address,\n\
         \t               and canonicalize seed (to lowercase + set '0x' prefix)\n\
         \t               to get Ethereum blockies correctly\n\
         \t-a --ascii     (only for 'ansi' mode)   Get non-compact, big blockies\n\
         \t                                        with ascii (non-unicode)\n\
         {raw_arg_msg}\n\
         \t-s --size=<BLOCKIES_SIZE> \n\
         \t               Blockies size: # of elems per side (1-32) (Default: '8')\n\n\
         \t-d --dimension=<WIDTH>x<HEIGHT>\n\
         \t               Dimensions of output in the form of '(width)x(height)'\n\
         \t               If not given, following is used (Default):\n\
         \t               - ('ansi' mode)   '(blockies_size)x(blockies_size)'\n\
         \t               - ('image' mode)  '128x128'\n\n\
         \t-o --outfile=<FILENAME>\n\
         \t               File name to write output\n\
         \t               If the parameter is not given, stdout is used (Default)\n\n\n\n\
         examples:\n\n\
         - Outputs from following commands are all the same:\n\
         $ {bin_name} {example_addr_canonical}\n\
         $ {bin_name} {example_addr_without_0x} -e\n\n\
         - Outputs from following commands are all the same:\n\
         $ {bin_name} \"generic_seed_not_ethaddr\" --size=15\n\
         $ {bin_name} \"generic_seed_not_ethaddr\" ansi --size=15 --dimension=15x15\n\
         $ {bin_name} \"generic_seed_not_ethaddr\" a -s 15 -d 15x15\n\n\
         - Outputs from following commands are all the same:\n\
         $ {bin_name} \"generic_seed\" image > blockies.png\n\
         $ {bin_name} \"generic_seed\" i -d128x128 -oblockies.png\n\
         $ {bin_name} \"generic_seed\" i -d 128x128 -o blockies.png\n",
        pkg_name = env!("CARGO_PKG_NAME"),
        pkg_ver = env!("CARGO_PKG_VERSION"),
        feature_variation = {
            #[cfg(feature = "compressed_png")]
            {
                ""
            }
            #[cfg(not(feature = "compressed_png"))]
            {
                "(compressed_png disabled)"
            }
        },
        pkg_author = env!("CARGO_PKG_AUTHORS"),
        pkg_license = env!("CARGO_PKG_LICENSE"),
        bin_usage = bin_usage(),
        bin_name = env!("CARGO_BIN_NAME"),
        example_addr_canonical = "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc",
        example_addr_without_0x = "e686c14FF9C11038F2B1c9aD617F2346CFB817dC",
        raw_arg_msg = {
            #[cfg(feature = "compressed_png")]
            {
                "\t-r --raw       (only for 'image' mode)  Get uncompressed, raw png image\n"
            }
            #[cfg(not(feature = "compressed_png"))]
            {
                ""
            }
        }
    )
}

fn bin_usage() -> String {
    format!(
        "usage: {bin_name} <seed> [output-fmt (ansi|image)] [OPTIONS...]",
        bin_name = env!("CARGO_BIN_NAME"),
    )
}
