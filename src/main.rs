use eth_blockies::*;

use std::{env, fs, io};

fn main() -> io::Result<()> {
    let mut args = env::args();

    match args.nth(1).as_deref() {
        Some("image") => {
            eprintln!("image mode!");

            image_mode(args)
        }
        Some("console") => {
            eprintln!("console mode!");

            console_mode(args)
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "")),
    }?;

    Ok(())
}

// print as file output (png format)
fn image_mode(args: env::Args) -> io::Result<()> {
    fn parse_args(mut args: env::Args) -> io::Result<(String, (u32, u32), Option<String>)> {
        // Some( addr, (dim-width, dim-height), Some(file-out-name) )
        Ok((
            // addr
            args.next().ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid argument: Missing address",
            ))?,
            // dimension (width, height)
            args.next()
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid argument: Missing dimension",
                ))?
                .split_once('x')
                .and_then(|(w, h)| w.parse().ok().zip(h.parse().ok()))
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid argument: Invalid dimension",
                ))?,
            // name of file out
            args.next(),
        ))
    }
    let (addr, dimension, file_out) = parse_args(args)?;

    let img_png_data = eth_blockies_png_data(addr.addr_canonicalize(), dimension);

    use io::Write;
    match file_out {
        Some(file) => fs::File::create(file)?.write_all(&img_png_data)?,
        None => io::stdout().write_all(&img_png_data)?,
    }

    Ok(())
}

// print as ascii console output
fn console_mode(_args: env::Args) -> io::Result<()> {
    /*
    fn parse_args(mut args: env::Args) -> io::Result<(String, (u32, u32), String)> {
        // Some( addr, (dim-width, dim-height), Some(mode) )
        Ok((
            // addr
            args.next().ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid argument: Missing address",
            ))?,
            // dimension (width, height)
            args.next()
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid argument: Missing dimension",
                ))?
                .split_once('x')
                .and_then(|(w, h)| w.parse().ok().zip(h.parse().ok()))
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid argument: Invalid dimension",
                ))?,
            // name of file out
            args.next().unwrap_or("".to_string()),
        ))
    }
    let (addr, dimension, mode) = parse_args(args)?;

    let ansi_string_data = eth_blockies_ansiseq_data(addr.addr_canonicalize(), dimension, true);

    use io::Write;
    io::stdout().write_all(&ansi_string_data)
     */
    Ok(())
}
