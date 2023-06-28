# eth-blockies-rs
![binary-example.png](https://github.com/snoopy3476/eth-blockies-rs/blob/b2f9fef9367d8df311f2e03bb9b719c243244096/asset/binary-example.png?raw=true)

A lightweight library in pure Rust to get raw data of Ethereum-style blocky identicon,
which can be used for generating blockies icon images, printing to terminal, etc.

Useful when getting raw RGB data of Ethereum-style blockies, as well as complete png image files.

Supports general Rust bin/lib, and WebAssembly (wasm) target.



## Live Demo

![library-example.png](https://github.com/snoopy3476/eth-blockies-rs/blob/b2f9fef9367d8df311f2e03bb9b719c243244096/asset/library-example.png?raw=true)

* Check for the [**wasm demo** (Ethereum-style Blockies Generator)](https://snoopy3476.github.io/eth-blockies-rs) to generate your custom blockies.



## Library



### Documentation

* Check for the [full library documentation on **docs.rs**](https://docs.rs/eth-blockies/1.1/) for details.



### Prerequisites

* Add a dependency to `Cargo.toml` of a Rust crate
  ```toml
  [dependencies]
  eth-blockies = "1.1"
  ```
  ...or without `compressed_png` default crate feature:
  ```toml
  [dependencies]
  eth-blockies = { version = "1.1", default-features = false }
  ```


### Library Basic Usage


1. Define a blockies type (size) to use

   * Make an alias type of [`Blockies`](https://docs.rs/eth-blockies/1.1/eth_blockies/type.Blockies.html)
   ```rust
   use eth_blockies::{Blockies, BlockiesGenerator};
 
   // Blockies < size (const), T >
   type Icon<T> = Blockies<15, T>;
   ```
  
   * *Cf)* For Ethereum address blockies, [`EthBlockies`](https://docs.rs/eth-blockies/1.1/eth_blockies/type.EthBlockies.html) is predefined as follows
   ```rust
   // use statement for Ethereum address blockies
   use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};
   //   type 'EthBlockies<T>' is predefined as 'Blockies<8, T>'
   ```   
---


2. Select an input seed type

   * Check for [`SeedInput`](https://docs.rs/eth-blockies/1.1/eth_blockies/trait.SeedInput.html) to get full list of input seed types
   ```rust
   // generate blockies from various input type
   let from_string = Icon::data("eth-blockies".to_string());
   let from_byte_vec = Icon::data(vec![0x0c, 0x93, 0xa3, 0x2e]);
   ```
  
   * *Cf)* For Ethereum address seeds, apply [`to_ethaddr_seed()`](https://docs.rs/eth-blockies/1.1/eth_blockies/trait.SeedInput.html#method.to_ethaddr_seed) before passing as input seed
   ```rust
   // generate Ethereum address blockies from various input type

   let seed_from_str = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
       .to_ethaddr_seed();
   let from_str = EthBlockies::data(seed_from_str);

   let seed_from_bytes = [
         0xe6, 0x86, 0xc1, 0x4f, 0xf9, 0xc1, 0x10, 0x38, 0xf2, 0xb1,
         0xc9, 0xad, 0x61, 0x7f, 0x23, 0x46, 0xcf, 0xb8, 0x17, 0xdc,
       ].to_ethaddr_seed();
   let from_bytes = EthBlockies::data(seed_from_bytes);
   ```
---


3. Select an output data type

   * Check for [`BlockiesGenerator`](https://docs.rs/eth-blockies/1.1/eth_blockies/trait.BlockiesGenerator.html) to get full list of output data types
   ```rust
   // generate blockies in various forms
   let in_rgb_2d_arr = Icon::data("eth-blockies");
   let in_indexed_2d_arr = Icon::indexed_data("eth-blockies");
   let in_gray_2d_arr = Icon::data_mapped("eth-blockies", to_gray);
   let in_png_data_vec = Icon::png_data("eth-blockies", (128, 128));
   ```


### Example

* Generate an Ethereum address blockies (with [`to_ethaddr_seed()`](https://docs.rs/eth-blockies/1.1/eth_blockies/trait.SeedInput.html#method.to_ethaddr_seed))

  ```rust
  use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};

  let seed = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC"
      .to_ethaddr_seed(); // required for Ethereum address blockies

  // 8x8 2D-array of (r, g, b)
  {
      let eth_blockies_from_addr = EthBlockies::data(&seed);
  }

  // uncompressed png data in byte vector
  {
      let eth_blockies_png_from_addr =
          EthBlockies::png_data(&seed, (128, 128));
          // use below for compressed png
          // EthBlockies::compressed_png_data(&seed, (128, 128));

      // write data as png file
      use std::io::Write;
      std::fs::File::create("eth-blockies.png").unwrap()
          .write_all(&eth_blockies_png_from_addr);
  }
  ```


* Generate an html `img` blockies element, on wasm target
  ```rust
  // addr to blockies data uri scheme,
  // which can be used directly in img elem 'src' or css 'url()'
  fn eth_blockies_data_uri(addr: &str) -> String {
     use eth_blockies::{EthBlockies, SeedInput, BlockiesGenerator};

     let addr_input = addr.to_ethaddr_seed();
     let output_dim = (128, 128);
     let data_uri_output = true;

     EthBlockies::png_data_base64(
        addr_input, output_dim, data_uri_output)
  }

  use web_sys::*;

  let addr = "0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC";

  window()
     .and_then(|w| w.document())
     .and_then(|doc| doc.body().zip(doc.create_element("img").ok()))
     .and_then(|(body, img)| {
        // create a new img html element with generated data_uri
        img.set_attribute("src", &eth_blockies_data_uri(addr))
           // then attach to body
           .and_then(|_| body.append_child(&img))
           .ok()
     });
  ```


### Cargo Features

* `compressed_png` (Enabled by default)
  * This feature enables following functions:
    * [`compressed_png_data()`](https://docs.rs/eth-blockies/1.1/eth_blockies/trait.BlockiesGenerator.html#tymethod.compressed_png_data)
    * [`compressed_png_data_base64()`](https://docs.rs/eth-blockies/1.1/eth_blockies/trait.BlockiesGenerator.html#tymethod.compressed_png_data_base64)
  * This feature adds a following external dependency:
    * [`deflate`](https://docs.rs/deflate/) crate
  * If png compression is not needed,
    disable this feature as follows when adding the crate:
    * E.g.
      * Shell: `cargo add eth-blockies@1.1 --no-default-features`
      * Cargo.toml: `eth-blockies = { version = "1.1", default-features = false }`




## Binary



### Install

```console
$ cargo install eth-blockies
```



### Binary Usage
```text
usage: eth-blockies <seed> [output-fmt (ansi|image)] [OPTIONS...]

<seed>                 Seed to generate blockies (e.g. Ethereum wallet address)

[output-fmt]  - ansi   (Default) Generate ansi sequence of blockies,
                       usually for printing to terminal
              - image  Generate png image data of blockies

[OPTIONS...]:

        -e --ethseed   Interpret seed string as Ethereum address,
                       and canonicalize seed (to lowercase + set '0x' prefix)
                       to get Ethereum blockies correctly
        -a --ascii     (only for 'ansi' mode)   Get non-compact, big blockies
                                                with ascii (non-unicode)
        -r --raw       (only for 'image' mode)  Get uncompressed, raw png image

        -s --size=<BLOCKIES_SIZE>
                       Blockies size: # of elems per side (1-32) (Default: '8')

        -d --dimension=<WIDTH>x<HEIGHT>
                       Dimensions of output in the form of '(width)x(height)'
                       If not given, following is used (Default):
                       - ('ansi' mode)   '(blockies_size)x(blockies_size)'
                       - ('image' mode)  '128x128'

        -o --outfile=<FILENAME>
                       File name to write output
                       If the parameter is not given, stdout is used (Default)



examples:

- Outputs from following commands are all the same:
$ eth-blockies 0xe686c14ff9c11038f2b1c9ad617f2346cfb817dc
$ eth-blockies e686c14FF9C11038F2B1c9aD617F2346CFB817dC -e

- Outputs from following commands are all the same:
$ eth-blockies "generic_seed_not_ethaddr" --size=15
$ eth-blockies "generic_seed_not_ethaddr" ansi --size=15 --dimension=15x15
$ eth-blockies "generic_seed_not_ethaddr" a -s 15 -d 15x15

- Outputs from following commands are all the same:
$ eth-blockies "generic_seed" image > blockies.png
$ eth-blockies "generic_seed" i -d128x128 -oblockies.png
$ eth-blockies "generic_seed" i -d 128x128 -o blockies.png
```



## Author
Kim Hwiwon \<kim.hwiwon@outlook.com\>



## License
The MIT License (MIT)
