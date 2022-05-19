use eth_blockies::*;

mod arg_parser;
use arg_parser::*;
use std::{env, fs, io};

fn main() {
    match {
        parse_args()
            // run main routine
            .and_then(|(arg_list, mut opt_list)| {
                let mut arg_list = arg_list.iter();

                // initialize seed
                match (arg_list.next(), opt_list.get_opt("ethseed", false)?.0) {
                    (Some(seed), false) => Ok(seed.to_string()),
                    (Some(seed), true) => Ok(seed.canonicalize_ethaddr()),
                    (None, _) => Err(io::Error::new(io::ErrorKind::InvalidInput, "")),
                }
                // select output format
                .and_then(|seed| match arg_list.next().map(String::as_str) {
                    Some("ansi") | Some("a") | None => get_ansi(&seed, opt_list), // default
                    Some("image") | Some("i") => get_image(&seed, opt_list),
                    Some(input) => Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!(
                            "Invalid argument: Invalid output format: '{}'\n\
                             Use either 'ansi' or 'image' here.",
                            input
                        ),
                    )),
                })
            })
            // print message if error
            .map_err(|e| {
                match e.kind() {
                    io::ErrorKind::InvalidInput => {
                        match e.to_string().as_ref() {
                            "" => eprintln!("{}", help_msg()),
                            non_empty_input => eprintln!(
                                "{}\n\n* For detailed usages of this binary, \
                                 run this binary without any parameter.",
                                non_empty_input
                            ),
                        };
                    }
                    _ => {
                        eprintln!("{}", e.to_string());
                    }
                };
                e
            })
    } {
        // return exit code
        Ok(_) => std::process::exit(0),
        _ => std::process::exit(1),
    };
}

// print as ascii ansi output
fn get_ansi(seed: &str, opt_list: OptList) -> io::Result<()> {
    fn parse_opts(opt_list: OptList) -> io::Result<(bool, (usize, usize), Option<String>)> {
        let mut opt_list = opt_list.clone();
        // Some( is_ascii, (dim-width, dim-height), Some(file-out-name) )
        Ok((
            // type
            (!opt_list.get_opt("ascii", false)?.0),
            // dimension (width, height)
            opt_list
                .get_opt("dimension", true)?
                .1
                .as_deref()
                .unwrap_or("8x8") // default
                .split_once('x')
                .and_then(|(w, h)| w.parse().ok().zip(h.parse().ok()))
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid argument: Invalid ansi blockies dimension\n\
                     Type input in the form of '(width)x(height)'. (e.g. 8x8)",
                ))?,
            // name of file out
            opt_list.get_opt("outfile", true)?.1.map(String::from),
        ))
        // check if opt_list is empty. if not, abort parsing and return warning
        .and_then(|ret| opt_list.check_if_empty().map(|_| ret))
    }

    let (is_utf8, dimension, file_out) = parse_opts(opt_list)?;

    let ansi_string_data = eth_blockies_ansiseq_data(seed, dimension, is_utf8).join("\n");

    use io::Write;
    match file_out {
        Some(file) => fs::File::create(file)?.write_all(ansi_string_data.as_bytes())?,
        None => io::stdout().write_fmt(format_args!("{}\n", ansi_string_data))?,
    }

    Ok(())
}

// print as png format
fn get_image(seed: &str, opt_list: OptList) -> io::Result<()> {
    fn parse_opts(opt_list: OptList) -> io::Result<(bool, (usize, usize), Option<String>)> {
        let mut opt_list = opt_list.clone();
        // Some( is_uncompressed_png, (dim-width, dim-height), Some(file-out-name) )
        Ok((
            // type
            (!opt_list.get_opt("raw", false)?.0),
            // dimension (width, height)
            opt_list
                .get_opt("dimension", true)?
                .1
                .as_deref()
                .unwrap_or("128x128") // default
                .split_once('x')
                .and_then(|(w, h)| w.parse().ok().zip(h.parse().ok()))
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid argument: Invalid image dimension\n\
                     Type input in the form of '(width)x(height)'. (e.g. 128x128)",
                ))?,
            // name of file out
            opt_list.get_opt("outfile", true)?.1.map(String::from),
        ))
        // check if opt_list is empty. if not, abort parsing and return warning
        .and_then(|ret| opt_list.check_if_empty().map(|_| ret))
    }

    let (is_compressed, dimension, file_out) = parse_opts(opt_list)?;

    let img_png_data = eth_blockies_png_data(seed, dimension, is_compressed);

    use io::Write;
    match file_out {
        Some(file) => fs::File::create(file)?.write_all(&img_png_data)?,
        None => io::stdout().write_all(&img_png_data)?,
    }

    Ok(())
}

// print help message
fn help_msg() -> String {
    format!(
        "{pkg_name} v{pkg_ver}\n\
         A tool for generating Ethereum-style blocky identicon\n\
         Written by {pkg_author}\n\
         License: {pkg_license}\n\n\n\
         usage: {bin_name} <seed> [output-fmt (ansi|image)] [OPTIONS]...\n\n\
         <seed>                  Seed to generate blockies (e.g. Ethereum wallet address)\n\
         [output-fmt]  - ansi    (Default) Generate ansi sequence of blockies,\n\
         \0                        usually for printing to terminal\n\
         \0              - image   Generate png image data of blockies\n\n\
         [OPTIONS]:\n\n\
         \t-e --ethseed    Interpret seed string as Ethereum address,\n\
         \t                and canonicalize seed (to lowercase + set '0x' prefix)\n\
         \t                to get Ethereum blockies correctly\n\
         \t-a --ascii      (only for 'ansi' mode)   Get non-compact, big blockies\n\
         \t                                         with ascii (non-unicode)\n\
         \t-r --raw        (only for 'image' mode)  Get uncompressed, raw png image\n\n\
         \t-d --dimension=<WIDTH>x<HEIGHT>\n\
         \t                Dimensions of blockies in the form of '(width)x(height)'\n\
         \t                If not given, following is used (Default):\n\
         \t                - (only for 'ansi' mode)   '8x8'\n\
         \t                - (only for 'image' mode)  '128x128'\n\n\
         \t-o --outfile=<FILENAME>\n\
         \t                File name to write output\n\
         \t                If the parameter is not given, stdout is used (Default)\n\n\n\n\
         examples:\n\n\
         - Outputs from following commands are all the same:\n\
         $ {bin_name} {example_addr_canonical}\n\
         $ {bin_name} {example_addr_without_0x} -e\n\n\
         - Outputs from following commands are all the same:\n\
         $ {bin_name} \"generic_seed_not_ethaddr\"\n\
         $ {bin_name} \"generic_seed_not_ethaddr\" ansi --dimension=8x8\n\
         $ {bin_name} \"generic_seed_not_ethaddr\" a -d 8x8\n\n\
         - Outputs from following commands are all the same:\n\
         $ {bin_name} \"generic_seed\" image > blockies.png\n\
         $ {bin_name} \"generic_seed\" i -d128x128 -oblockies.png\n\
         $ {bin_name} \"generic_seed\" i -d 128x128 -o blockies.png\n",
        pkg_name = env!("CARGO_PKG_NAME"),
        pkg_ver = env!("CARGO_PKG_VERSION"),
        pkg_author = env!("CARGO_PKG_AUTHORS"),
        pkg_license = env!("CARGO_PKG_LICENSE"),
        bin_name = env!("CARGO_BIN_NAME"),
        example_addr_canonical = "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc",
        example_addr_without_0x = "e686c14FF9C11038F2B1c9aD617F2346CFB817dC",
    )
}
