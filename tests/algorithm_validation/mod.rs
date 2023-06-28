// These tests are disabled (ignored) by default.
// To test this, first place testcases inside subdir 'testcase',
// and run testmod 'algorithm_validation' with --ignored flag

#[cfg(test)]
mod algorithm_validation {
    use std::io;

    #[ignore]
    #[test]
    fn blockies_eth() -> io::Result<()> {
        test::seed_io_test::<8, true>()
    }
    #[ignore]
    #[test]
    fn blockies_r05() -> io::Result<()> {
        test::seed_io_test::<5, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r06() -> io::Result<()> {
        test::seed_io_test::<6, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r07() -> io::Result<()> {
        test::seed_io_test::<7, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r08() -> io::Result<()> {
        test::seed_io_test::<8, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r09() -> io::Result<()> {
        test::seed_io_test::<9, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r10() -> io::Result<()> {
        test::seed_io_test::<10, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r11() -> io::Result<()> {
        test::seed_io_test::<11, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r12() -> io::Result<()> {
        test::seed_io_test::<12, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r13() -> io::Result<()> {
        test::seed_io_test::<13, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r14() -> io::Result<()> {
        test::seed_io_test::<14, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r15() -> io::Result<()> {
        test::seed_io_test::<15, false>()
    }
    #[ignore]
    #[test]
    fn blockies_r16() -> io::Result<()> {
        test::seed_io_test::<16, false>()
    }

    mod test {
        use eth_blockies::*;
        use std::{fs, io};

        const RES_DIR: &str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/algorithm_validation/testcase"
        );

        pub fn seed_io_test<const R: usize, const E: bool>() -> io::Result<()> {
            let fname = format!(
                "{}/R{}{}.txt",
                RES_DIR,
                R,
                match E {
                    true => "_eth",
                    false => "",
                }
            );
            if std::path::Path::new(&fname).exists() {
                eprintln!("seed_io_test<{}>(): fname: {}", R, fname);
                fs::read_to_string(fname)?
                    .lines()
                    .map(parse_line::<R>)
                    .collect::<io::Result<()>>()?;
            } else {
                panic!("No testfile {}", fname);
            }
            Ok(())
        }

        // try to parse test io line
        fn parse_line<const R: usize>(line: &str) -> io::Result<()> {
            match line.is_empty() || line.starts_with("#") {
                true => Ok(()),

                // if line should be parsed && tested
                false => {
                    let mut line_iter = line.split_ascii_whitespace();

                    // split line
                    match (
                        line_iter.next(),
                        line_iter
                            .next()
                            .and_then(|output| parse_expected_output(output)),
                    ) {
                        // if line parsed successfully
                        (Some(input), Some(parsed_expected_output)) => {
                            // compare bitmap outputs

                            println!("Validating line... [{}]", line);

                            let generated_output = Blockies::<R>::data(input);

                            match generated_output.eq(&parsed_expected_output) {
                                true => io::Result::Ok(()),
                                false => Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!(
                                    "Expected output & generated output not match! (line: [{}])",
                                    line
                                ),
                                )),
                            }
                        }
                        _ => Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Failed to parse line! (line: [{}])", line),
                        )),
                    }
                }
            }
        }

        //
        // parse given expected output string
        fn parse_expected_output<const R: usize>(
            expected_output_str: &str,
        ) -> Option<Blockies<R, RgbPixel>> {
            let output_raw = expected_output_str.as_bytes();
            if output_raw.get(6)?.ne(&b',')
                || output_raw.get(13)?.ne(&b',')
                || output_raw.get(20)?.ne(&b'=')
                || output_raw.len().ne(&(21 + R * ((R + 1) / 2)))
            {
                return None;
            }

            let (c0_str, c1_str, c2_str, bitmap_half) = (
                &output_raw[0..6],
                &output_raw[7..13],
                &output_raw[14..20],
                &output_raw[21..]
                    .iter()
                    .map(|c| (c - b'0').try_into())
                    .collect::<Result<Vec<ColorClass>, _>>()
                    .ok()?,
            );

            // parse hex color code to rgb
            fn color_hex_to_rgb(hex_bytestr: &[u8]) -> Option<eth_blockies::RgbPixel> {
                Some((
                    u8::from_str_radix(&String::from_utf8_lossy(&hex_bytestr[0..2]), 16).ok()?,
                    u8::from_str_radix(&String::from_utf8_lossy(&hex_bytestr[2..4]), 16).ok()?,
                    u8::from_str_radix(&String::from_utf8_lossy(&hex_bytestr[4..6]), 16).ok()?,
                ))
            }

            let palette = [
                color_hex_to_rgb(c0_str)?,
                color_hex_to_rgb(c1_str)?,
                color_hex_to_rgb(c2_str)?,
            ];

            Some(Blockies::<R, RgbPixel>::new(|(x, y)| {
                match x < (R + 1) / 2 {
                    true => palette[bitmap_half[x + ((R + 1) / 2) * y]],
                    false => palette[bitmap_half[(R - x - 1) + ((R + 1) / 2) * y]],
                }
            }))
        }
    }
}
