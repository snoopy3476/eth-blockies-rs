# eth-blockies-rs
![binary-example.png](https://github.com/snoopy3476/eth-blockies-rs/blob/b2f9fef9367d8df311f2e03bb9b719c243244096/asset/binary-example.png?raw=true)

A lightweight library in pure Rust to get Ethereum-style blocky identicon data,
which can be used for generating blockies icon images, printing to terminal, etc.

Useful when getting raw RGB data of Ethereum-style blockies, as well as complete png image files.

Supports general Rust bin/lib, and WebAssembly (wasm) target.



## Live Demo

![library-example.png](https://github.com/snoopy3476/eth-blockies-rs/blob/b2f9fef9367d8df311f2e03bb9b719c243244096/asset/library-example.png?raw=true)

* Check for the [**wasm demo** (Ethereum-style Blockies Generator)](https://snoopy3476.github.io/eth-blockies-rs) to generate your custom blockies.



## Library



### Documentation

* Check for the [full library documentation on **docs.rs**](https://docs.rs/eth-blockies/1.0.0/eth_blockies/) for details.



### Prerequisites
* Add a dependency to `Cargo.toml` of a Rust crate
```toml
[dependencies]
eth-blockies = "1.0.0"
```



### Library Usage Examples



* Get raw blockies data, from various seed types
  * To generate a *standard* Ethereum blockies (commonly seen as icons of wallet addresses on other Ethereum platforms),
    the input address seed **MUST** be canonicalized to `0x(hex_letters_lowercase)` using `canonicalize_ethaddr()`, as `addr` below.
```rust
use eth_blockies::*;

// blockies from general string seed
{
    let seed = "eth-blockies-rs";

    // general string seed: used as it is
    let blockies_data_from_string = eth_blockies_data(seed);
}

// blockies from general byte-array seed
{
    let seed: &[u8] = &[
        0x0c, 0x93, 0xa3, 0x2e, 0xe5, 0x2b, 0xf6, 0x43,
        0x66, 0xdb, 0xdc, 0xd7, 0xed, 0xde, 0x00, 0x78,
    ];

    // general byte-array seed: used as it is
    let blockies_data_from_byte_arr = eth_blockies_data(seed);
}

// blockies from Ethereum address seed
{
    // "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    // -> "0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
        .canonicalize_ethaddr();

    assert_eq!(
        addr,
        b"0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc"
    );

    // Ethereum address seed: canonicalized before use
    let blockies_data_from_eth_addr = eth_blockies_data(addr);
}
```


* Get raw blockies data, in various forms
```rust
use eth_blockies::*;

let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    .canonicalize_ethaddr();


// get 2D array of (r, g, b)
// dimension: 8 x 8 (nested-array of RgbPixel)
{
    let blockies_data_rgb = eth_blockies_data(&seed);
}


// get 1D array of (r, g, b)
// dimension: 64 x 1 (array of RgbPixel)
{
    let blockies_data_rgb = eth_blockies_data(&seed).flatten();
}


// get 2D array of grayscale
{
    fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
        (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114)
            as u8
    }

    let blockies_data_grayscale =
        eth_blockies_data_mapped(&seed, rgb_to_grayscale);
}


// get (color palette, 2D array of color indices for each pixel)
{
    let (color_palette, palette_idx_bitmap) =
        eth_blockies_indexed_data(&seed);

    assert_eq!(
        color_palette[palette_idx_bitmap[0][0]],
        (132, 222, 77)
    );
}
```



* Write a generated blockies png data to file `test-raw.png`,
  on general Rust binary/library target
```rust
use eth_blockies::*;

let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
    .canonicalize_ethaddr();
let dimension = (128, 128); // multiples of 8 recommended
let compressed_output = true; // false for an uncompressed png
let img_png_data =
    eth_blockies_png_data(seed, dimension, compressed_output);


use std::io::Write;
std::fs::File::create("test-raw.png").unwrap()
    .write_all(&img_png_data);
```



* Generate an html `img` blockies element, on wasm target
```rust
// addr to blockies data uri scheme,
// which can be used directly in img elem 'src' or css 'url()'
fn eth_blockies_data_uri_scheme(addr: &str) -> String {
    use eth_blockies::*;

    let addr_input = addr.canonicalize_ethaddr();
    let dimension = (128, 128);
    let compressed_output = true;
    let data_uri_output = true;

    eth_blockies_png_data_base64(
        addr_input,
        dimension,
        compressed_output,
        data_uri_output
    )
}

use web_sys::*;

let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC";

window()
    .and_then(|w| w.document())
    .and_then(|doc| doc.body().zip(doc.create_element("img").ok()))
    .and_then(|(body, img)| {
        // create a new img html element with generated data_uri
        img.set_attribute("src", &eth_blockies_data_uri_scheme(addr))
            // then attach to body
            .and_then(|_| body.append_child(&img))
            .ok()
    });
```



## Binary



### Binary Usage
```text
usage: eth-blockies <seed> [output-fmt (ansi|image)] [OPTIONS]...

<seed>                  Seed to generate blockies (e.g. Ethereum wallet address)
[output-fmt]  - ansi    (Default) Generate ansi sequence of blockies,
                        usually for printing to terminal
              - image   Generate png image data of blockies

[OPTIONS]:

        -e --ethseed    Interpret seed string as Ethereum address,
                        and canonicalize seed (to lowercase + set '0x' prefix)
                        to get Ethereum blockies correctly
        -a --ascii      (only for 'ansi' mode)   Get non-compact, big blockies
                                                 with ascii (non-unicode)
        -r --raw        (only for 'image' mode)  Get uncompressed, raw png image

        -d --dimension=<WIDTH>x<HEIGHT>
                        Dimensions of blockies in the form of '(width)x(height)'
                        If not given, following is used (Default):
                        - (only for 'ansi' mode)   '8x8'
                        - (only for 'image' mode)  '128x128'

        -o --outfile=<FILENAME>
                        File name to write output
                        If the parameter is not given, stdout is used (Default)



examples:

- Outputs from following commands are all the same:
$ eth-blockies 0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc
$ eth-blockies e686c14FF9C11038F2B1c9aD617F2346CFB817dC -e

- Outputs from following commands are all the same:
$ eth-blockies "generic_seed_not_ethaddr"
$ eth-blockies "generic_seed_not_ethaddr" ansi --dimension=8x8
$ eth-blockies "generic_seed_not_ethaddr" a -d 8x8

- Outputs from following commands are all the same:
$ eth-blockies "generic_seed" image > blockies.png
$ eth-blockies "generic_seed" i -d128x128 -oblockies.png
$ eth-blockies "generic_seed" i -d 128x128 -o blockies.png
```



## Author
Kim Hwiwon \<kim.hwiwon@outlook.com\>



## License
The MIT License (MIT)
