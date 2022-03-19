// Use `wee_alloc` as the global allocator.
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

#[wasm_bindgen]
pub fn init_blockies() {
    let doc = &opt_unwrap(window().and_then(|w| w.document()));

    //
    // user address input //
    //

    attach_cb_user_input(&opt_unwrap(doc.get_element_by_id("user-addr-input")));

    //
    // predefined examples //
    //

    let blockies_examples = opt_unwrap(doc.get_element_by_id("blockies-examples"));

    let children = blockies_examples.children();
    (0..children.length()).for_each(|i| {
        children.get_with_index(i).map(|e| e.remove());
    });

    // addr list of https://github.com/MyCryptoHQ/ethereum-blockies-base64/blob/c1a77ebd5c0b23cd085bf4ced19f69b2a165a2d1/src/example/testAddresses.json
    [
        "0x4bbeEB066eD09B7AEd07bF39EEe0460DFa261520",
        "0x000528583ba0c881f4d26a1ff50886fc89efc03f",
        "0x01122df2b7d1c0a6ad94589da045af3885bedbbc",
        "0x000ba5e704c33c58b5e7949f67344821fa54bd29",
        "0x000587ac53175fc48d7b3e36d9c62f87275e1f2f ", //
        "0x000a9a0c2fb94536452aba2d199f11c404d508d3",
        "0x12333e7c757cf270bd55bf988ca267014aaa463c ", //
        "0x068899cceb463ed483b79b565dde3bdbc90f598a",
        "0x000eace0089e5d3c984bbd84bd4290426b0d71d1",
        "0x0009a464895f4ddd47595da98d38e9e9ec110fff ", //
        "0x01235557747af9cd120aca462dac992c329304bd",
        "0x000b82bb1f7db0eed2a69b78ba4dc655ca8086d6 ", //
        "0x00112ba39c66a00926aa1852e6f721f4f6505e72",
        "0x000f7474c7236159bf7d51e8d260c388f7567ea9",
        "0x000217dd5ce5985880567e8832ecba9a4cec7bb6",
        "0x00098e3e0fdb9ca774645eba75331af5c072f848 ", //
        "0x000738aa02f0a97baddb03aafba537ca1244ca7c",
        "0x000bda3063fce7699bf70ce31ff7f8ff69d9ccb7",
        "0x0002bad45b918ab01e931ab049806530180aed8a",
        "0x0008a0d473810aa819b471eef3d95743eb32ea89",
        "0x0008e430ca209924db554c8efe125479272538ea ", //
        "0x07999deff8024f153d1a34bcd1372c7162f76d07",
        "0x0009e6974cab530b6545da1e3d8354ff5f059a9f ", //
        "0x0001c190d5f71d37113c043498f8d69cd59bb7ba",
    ]
    .iter()
    .for_each(|addr| {
        res_unwrap(blockies_examples.append_child(&build_blockies(doc, addr)));
    });
}

fn attach_cb_user_input(input: &Element) {
    let input = opt_unwrap(input.dyn_ref::<HtmlInputElement>());
    let cb_input = input.clone();
    let cb_blockies = opt_unwrap(
        input
            .next_element_sibling()
            .and_then(|blockies_wrapper| blockies_wrapper.first_element_child()),
    );

    let refresh_cb_closure = move || refresh_blockies(&cb_blockies, &cb_input.value());
    refresh_cb_closure();
    let refresh_cb_closure_js = Closure::wrap(Box::new(refresh_cb_closure) as Box<dyn FnMut()>);
    input.set_oninput(Some(refresh_cb_closure_js.as_ref().unchecked_ref()));

    refresh_cb_closure_js.forget();
}

fn build_blockies(doc: &Document, addr: &str) -> Element {
    let blockies = new_elem(
        doc,
        "figure",
        &[],
        None,
        &[
            new_elem(
                doc,
                "a",
                &[("target", Some("_blank"))],
                None,
                &[new_elem(
                    doc,
                    "img",
                    &[
                        // pixelated: no blur when upscale
                        ("style", Some("image-rendering: pixelated !important")),
                    ],
                    None,
                    &[],
                )],
            ),
            new_elem(doc, "figcaption", &[], None, &[]),
        ],
    );
    refresh_blockies(&blockies, addr);

    blockies
}

fn refresh_blockies(blockies: &Element, addr: &str) {
    use eth_blockies::*;
    let is_addr_valid = is_addr_valid(addr);
    let addr_canon = addr.addr_canonicalize();

    let (img_link, img, caption) = (
        opt_unwrap(blockies.children().get_with_index(0)).clone(),
        opt_unwrap(
            blockies
                .children()
                .get_with_index(0)
                .and_then(|a| a.first_element_child()),
        )
        .clone(),
        opt_unwrap(blockies.children().get_with_index(1)).clone(),
    );

    res_unwrap(blockies.set_attribute(
        "class",
        match is_addr_valid {
            true => "blockies",
            false => "blockies invalid-addr",
        },
    ));

    match is_addr_valid {
        true => res_unwrap(img_link.set_attribute("href", &etherscan_link(&addr_canon))),
        false => res_unwrap(img_link.remove_attribute("href")),
    }
    res_unwrap(img.set_attribute("src", &eth_blockies_data_uri(&addr_canon)));

    caption.set_text_content(Some(&addr.replace(" ", "▒")));
}

fn new_elem(
    doc: &Document,
    name: &str,
    attrs: &[(&str, Option<&str>)],
    text: Option<&str>,
    children: &[Element],
) -> Element {
    let elem = res_unwrap(doc.create_element(name));
    attrs.iter().for_each(|(k, v)| {
        res_unwrap(elem.set_attribute(k, v.unwrap_or_default()));
    });
    elem.set_text_content(text);
    children.iter().for_each(|child| {
        res_unwrap(elem.append_child(child));
    });
    elem
}

/// Return data uri of Ethereum blockies for given address
fn eth_blockies_data_uri(addr: &str) -> String {
    use eth_blockies::*;
    "data:image/png;base64,".to_owned()
        + &res_unwrap(String::from_utf8(eth_blockies_png_data_base64(
            addr,
            (8, 8),
        )))
}

fn is_addr_valid(addr: &str) -> bool {
    addr.len().eq(&42)
        && (addr
            .get(0..2)
            .map(|prefix| prefix.eq("0x") || prefix.eq("0X"))
            .unwrap_or(false))
        && (addr
            .get(2..)
            .map(|hex_addr| hex_addr.chars().all(|c| c.is_ascii_hexdigit()))
            .unwrap_or(false))
}

fn etherscan_link(addr: &str) -> String {
    "https://etherscan.io/address/".to_owned() + addr
}

//
// *_unwrap: used for bin size reduction, instead of unwrap/expect
//   https://rustwasm.github.io/docs/book/reference/code-size.html#optimizing-builds-for-code-size
#[inline]
pub fn opt_unwrap<T>(o: Option<T>) -> T {
    use std::process;
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}
#[inline]
pub fn res_unwrap<T, E>(r: Result<T, E>) -> T {
    use std::process;
    match r {
        Ok(t) => t,
        Err(_) => process::abort(),
    }
}
