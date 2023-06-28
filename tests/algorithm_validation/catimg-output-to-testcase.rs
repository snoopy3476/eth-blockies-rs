// catimg output to test-io string
//
// usage
// $ (command-to-print-blockies-png-data) | catimg -r1 -w(Resolution * 2) - | gen-test-io

use std::{
    env::args,
    io::{stderr, stdin, BufRead, Write},
};
fn main() {
    let print_orig_output = args().skip(1).next().map(|s| s.eq("-r")).unwrap_or(false);

    let (stdin, mut stderr) = (stdin(), stderr());
    let mut buf = Vec::<u8>::new();
    let mut buf_orig = Vec::<u8>::new();

    let mut color: (Option<[u8; 6]>, Option<[u8; 6]>, Option<[u8; 6]>) = (None, None, None);
    let mut bitmap_str = Vec::<u8>::new();

    loop {
        match stdin.lock().read_until(b'\n', &mut buf) {
            Ok(0) | Err(_) => return,
            Ok(_) => {
                buf_orig.extend_from_slice(&buf);

                let mut buf_cur = buf.as_slice();

                // start a new blockies test-io string if prefix appears
                if let Some(new_buf_cur) = buf_cur.strip_prefix(b"\x1b[s\x1b[?25l") {
                    buf_cur = new_buf_cur;
                }
                // finalize a blockies test-io string if suffix appears
                if let Some(new_buf_cur) = buf_cur.strip_suffix(b"\x1b[?25h") {
                    // write original blockies output to terminal first
                    if print_orig_output {
                        stderr.write_all(&buf_orig).ok();
                        buf_orig.clear();
                        stderr.flush().ok();
                    }

                    // write built test-io string
                    buf_cur = new_buf_cur;
                    println!(
                        "{},{},{}={}",
                        String::from_utf8_lossy(&color.0.expect("invalid format: no color found")),
                        String::from_utf8_lossy(&color.1.unwrap_or(*b"000000")),
                        String::from_utf8_lossy(&color.2.unwrap_or(*b"000000")),
                        String::from_utf8_lossy(&bitmap_str)
                    );
                    bitmap_str.clear();
                }

                // remove reset sequence
                if let Some(new_buf_cur) = buf_cur.strip_suffix(b"\x1b[m") {
                    buf_cur = new_buf_cur;
                }

                let mut bitmap_line_cur = Vec::<u8>::new();
                buf_cur
                    .split(|b| [b' ', b'\n'].contains(b))
                    .filter(|seq| !(seq.is_empty() || seq.eq(b"\x1b[m")))
                    .for_each(|color_seq| {
                        // color_seq: b"\x1b[0;48;2;R;G;Bm"

                        //println!("color_seq: {}", color_seq.escape_ascii().to_string());

                        // parse current color from sequence
                        let color_now = color_seq
                            .strip_prefix(b"\x1b[0;48;2;")
                            .and_then(|seq| seq.strip_suffix(b"m"))
                            .and_then(|seq| {
                                let mut seq_iter = seq.split(|b| b == &b';');
                                match (seq_iter.next(), seq_iter.next(), seq_iter.next()) {
                                    (Some(r_seq), Some(g_seq), Some(b_seq)) => {
                                        Some(bstr_rgb_to_hex(r_seq, g_seq, b_seq))
                                    }
                                    _ => None,
                                }
                            })
                            .expect("parse error: rgb value");

                        // find color idx from global colors:
                        //   if not found, add new to global colors and return its idx
                        let idx_char = match color {
                            (Some(c), _, _) if c == color_now => b'0',
                            (_, Some(c), _) if c == color_now => b'1',
                            (_, _, Some(c)) if c == color_now => b'2',
                            _ => match color {
                                (None, _, _) => {
                                    color.0 = Some(color_now);
                                    b'0'
                                }
                                (_, None, _) => {
                                    color.1 = Some(color_now);
                                    b'1'
                                }
                                (_, _, None) => {
                                    color.2 = Some(color_now);
                                    b'2'
                                }
                                _ => {
                                    panic!("invalid format: colors more than 3");
                                }
                            },
                        };

                        bitmap_line_cur.push(idx_char);
                    });

                // check if decalcomania
                if bitmap_line_cur
                    .iter()
                    .zip(bitmap_line_cur.iter().rev())
                    .any(|(f, r)| f != r)
                {
                    panic!("invalid format: decalcomania check failed");
                }

                bitmap_line_cur.truncate((bitmap_line_cur.len() + 1) / 2);
                bitmap_str.append(&mut bitmap_line_cur);

                buf.clear();
            }
        }
    }
}

#[inline(always)]
fn bstr_rgb_to_hex(r: &[u8], g: &[u8], b: &[u8]) -> [u8; 6] {
    fn bstr_dec_to_hex(input: &[u8]) -> [u8; 2] {
        const HEX_TABLE: &[u8] = b"0123456789abcdef";

        if !input.iter().all(u8::is_ascii_digit) {
            panic!("invalid format: rgb value parsed, but not digit");
        }

        let num_val = input.iter().fold(0_u8, |acc, x| acc * 10 + (x - b'0'));

        [
            HEX_TABLE[(num_val >> 4) as usize],
            HEX_TABLE[(num_val & 0xF) as usize],
        ]
    }

    let (r_hex, g_hex, b_hex) = (bstr_dec_to_hex(r), bstr_dec_to_hex(g), bstr_dec_to_hex(b));
    [r_hex[0], r_hex[1], g_hex[0], g_hex[1], b_hex[0], b_hex[1]]
}
