# eth-blockies-rs
A lightweight library in pure Rust to get Ethereum blockies raw data (or png images),
which can be used for creating blockies icon images, printing to terminal, etc.

Supports wide range of targets including Rust bin/lib, and WebAssembly (wasm).



## Live Demo

* Check for the following wasm demo to generate your address blockies:  
[**DEMO**: Ethereum Blockies Generator](https://snoopy3476.github.io/eth-blockies-rs)



## Include to a Rust Cargo crate
* `Cargo.toml`
```toml
[dependencies]
eth-blockies = "0.9.1"
```



## Usage Examples


* Get a raw blockies data
```rust
use eth_blockies::*;

let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
     .addr_canonicalize();


// get 2D array of (r, g, b)
{
    let blockies_data_rgb = eth_blockies_data(&addr);
}


// get 2D array of grayscale
{
    fn rgb_to_grayscale((r, g, b): RgbPixel) -> u8 {
        (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) as u8
    }
    
    let blockies_data_grayscale =
        eth_blockies_data_mapped(&addr, rgb_to_grayscale);
}


// get (color palette, palette index of each pixel)
{
    let (color_palette, palette_idx_bitmap) =
        eth_blockies_indexed_data(&addr);
}
```



* Write a generated blockies to png file `text.png`, on Rust binary/library target
```rust
use eth_blockies::*;

let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
     .addr_canonicalize();
let dimension = (128, 128);
let img_png_data = eth_blockies_png_data(addr, dimension);

use std::io::Write;
std::fs::File::create("test.png").unwrap().write_all(&img_png_data);
```



* Generate a html `img` element of a generated blockies, on wasm target
```rust
// addr to blockies data_uri,
// which can be used directly in img elem 'src' or css 'url()'
fn eth_blockies_data_uri(addr: &str) -> Option<String> {
    use eth_blockies::*;

    let img_data_base64 =
        eth_blockies_png_data_base64(
            addr.addr_canonicalize(),
            (8, 8)
        );

    String::from_utf8(img_data_base64)
        .map(|data| "data:image/png;base64,".to_owned() + &data)
        .ok()
}

use web_sys::*;

let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC";

window()
    .and_then(|w| w.document())
    .and_then(|doc| doc.body().zip(doc.create_element("img").ok()))
    .and_then(|(body, img)| {
        // set data uri to img src
        eth_blockies_data_uri(addr)
            .and_then(|data_uri|
                img.set_attribute("src", &data_uri).ok()
            );

        img.set_attribute(
            "style",
            concat!(
                // no blur on scaling
                "image-rendering: pixelated !important; ",
                "width: 120px; height: 120px;",
            ),
        );

        body.append_child(&img).ok()
    });
```



## Author
Kim Hwiwon \<kim.hwiwon@outlook.com\>



## License
The MIT License (MIT)
