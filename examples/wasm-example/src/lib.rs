use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

struct HtmlElems {
    figure_blockies: HtmlElement,
    input_seed: HtmlInputElement,
    input_is_seed_ethaddr: HtmlInputElement,
    input_save_dim_width: HtmlInputElement,
    input_save_dim_height: HtmlInputElement,
    input_save_button: HtmlInputElement,
    a_save_trigger: HtmlElement,
    section_examples: HtmlElement,
}

struct HtmlElemsRef<'a> {
    figure_blockies: &'a HtmlElement,
    input_seed: &'a HtmlInputElement,
    input_is_seed_ethaddr: &'a HtmlInputElement,
    input_save_dim_width: &'a HtmlInputElement,
    input_save_dim_height: &'a HtmlInputElement,
    input_save_button: &'a HtmlInputElement,
    a_save_trigger: &'a HtmlElement,
    section_examples: &'a HtmlElement,
}

impl HtmlElems {
    pub fn new(
        figure_blockies: HtmlElement,
        input_seed: HtmlInputElement,
        input_is_seed_ethaddr: HtmlInputElement,
        input_save_dim_width: HtmlInputElement,
        input_save_dim_height: HtmlInputElement,
        input_save_button: HtmlInputElement,
        a_save_trigger: HtmlElement,
        section_examples: HtmlElement,
    ) -> Self {
        Self {
            figure_blockies,
            input_seed,
            input_is_seed_ethaddr,
            input_save_dim_width,
            input_save_dim_height,
            input_save_button,
            a_save_trigger,
            section_examples,
        }
    }

    pub fn get() -> HtmlElemsRef<'static> {
        unsafe {
            opt_unwrap(HTML_ELEMS.as_ref().and_then(|v| {
                Some(HtmlElemsRef {
                    figure_blockies: &v.figure_blockies,
                    input_seed: &v.input_seed,
                    input_is_seed_ethaddr: &v.input_is_seed_ethaddr,
                    input_save_dim_width: &v.input_save_dim_width,
                    input_save_dim_height: &v.input_save_dim_height,
                    input_save_button: &v.input_save_button,
                    a_save_trigger: &v.a_save_trigger,
                    section_examples: &v.section_examples,
                })
            }))
        }
    }
}

static mut HTML_ELEMS: Option<HtmlElems> = None;

// initialization //

#[wasm_bindgen]
pub fn init_blockies() {
    let doc = &opt_unwrap(window().and_then(|w| w.document()));
    unsafe {
        HTML_ELEMS = Some(HtmlElems::new(
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("figure-blockies-card"))
                    .clone()
                    .dyn_into::<HtmlElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("input-seed"))
                    .clone()
                    .dyn_into::<HtmlInputElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("input-is-seed-ethaddr"))
                    .clone()
                    .dyn_into::<HtmlInputElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("input-blockies-save-dim-width"))
                    .clone()
                    .dyn_into::<HtmlInputElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("input-blockies-save-dim-height"))
                    .clone()
                    .dyn_into::<HtmlInputElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("input-blockies-save-button"))
                    .clone()
                    .dyn_into::<HtmlInputElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("a-blockies-save-trigger"))
                    .clone()
                    .dyn_into::<HtmlElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("section-blockies-examples"))
                    .clone()
                    .dyn_into::<HtmlElement>(),
            ),
        ));
    }

    let elems = HtmlElems::get();

    //
    // predefined examples //
    //

    let blockies_examples = elems.section_examples;

    let children = blockies_examples.children();
    (0..children.length()).for_each(|i| {
        children.get_with_index(i).map(|e| e.remove());
    });

    [
        ("eth-blockies-rs", false),
        ("0x0000000000000000000000000000000000000000", true),
        ("0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC", true),
        ("0x*_This_invalid_address_is_marked_in_red*", true),
        ("0x4bbeEB066eD09B7AEd07bF39EEe0460DFa261520", true),
        ("0x4bbeEB066eD09B7AEd07bF39EEe0460DFa261520", false),
        ("plain text example", false),
        ("0x000a9a0c2fb94536452aba2d199f11c404d508d3", true),
        ("example-your-email-addr@example.com", false),
        ("0x068899cceb463ed483b79b565dde3bdbc90f598a", true),
        ("0x000eace0089e5d3c984bbd84bd4290426b0d71d1", true),
        ("0x0009a464895f4ddd47595da98d38e9e9ec110fff", true),
        ("0x01235557747af9cd120aca462dac992c329304bd", true),
        ("0x000b82bb1f7db0eed2a69b78ba4dc655ca8086d6", true),
        ("0x00112ba39c66a00926aa1852e6f721f4f6505e72", true),
        ("0x000f7474c7236159bf7d51e8d260c388f7567ea9", true),
        ("0x000217dd5ce5985880567e8832ecba9a4cec7bb6", true),
        ("0x00098e3e0fdb9ca774645eba75331af5c072f848", true),
        ("0x000738aa02f0a97baddb03aafba537ca1244ca7c", true),
        ("0x000bda3063fce7699bf70ce31ff7f8ff69d9ccb7", true),
        ("0x0002bad45b918ab01e931ab049806530180aed8a", true),
        ("0x0008a0d473810aa819b471eef3d95743eb32ea89", true),
        ("0x0008e430ca209924db554c8efe125479272538ea", true),
        ("0x07999deff8024f153d1a34bcd1372c7162f76d07", true),
    ]
    .iter()
    .for_each(|(addr, is_seed_ethaddr)| {
        opt_unwrap({
            let blockies_wrapper = build_blockies(doc, *is_seed_ethaddr);
            blockies_wrapper
                .children()
                .get_with_name("blockies-card")
                .and_then(|b| b.dyn_into::<HtmlElement>().ok())
                .and_then(|b| {
                    blockies_examples
                        .append_child(&blockies_wrapper)
                        .map(|_| refresh_blockies(&b, addr, *is_seed_ethaddr))
                        .ok()
                })
        });
    });

    // finalize initialization
    refresh_input_blockies();
    res_unwrap(elems.figure_blockies.class_list().remove_1("loading"));
}

// disable all inputs before generating blockies
#[wasm_bindgen]
pub fn prepare_save_blockies() {
    let elems = HtmlElems::get();

    res_unwrap(
        elems
            .figure_blockies
            .class_list()
            .add_1("loading")
            .map(|_| {
                [
                    elems.input_seed,
                    elems.input_is_seed_ethaddr,
                    elems.input_save_dim_width,
                    elems.input_save_dim_height,
                    elems.input_save_button,
                ]
                .iter()
                .for_each(|input| {
                    res_unwrap(input.set_attribute("disabled", ""));
                });
            }),
    );
}

// generate blockies, and re-enable disabled inputs
#[wasm_bindgen]
pub fn save_blockies() {
    let elems = HtmlElems::get();

    let seed = {
        use eth_blockies::*;
        match elems.input_is_seed_ethaddr.checked() {
            true => elems.input_seed.value().canonicalize_ethaddr(),
            false => elems.input_seed.value(),
        }
    };
    let dimension = (
        elems
            .input_save_dim_width
            .value()
            .parse::<usize>()
            .ok()
            .filter(|v| *v > 0) // invalid if 0
            .unwrap_or(512),
        elems
            .input_save_dim_height
            .value()
            .parse::<usize>()
            .ok()
            .filter(|v| *v > 0) // invalid if 0
            .unwrap_or(512),
    );

    let download_data = eth_blockies_data_uri(&seed, dimension);
    let download_fname = format!("eth-blockies-rs_{}.png", seed);

    // set data to trigger_save
    res_unwrap(
        {
            // data
            elems.a_save_trigger.set_attribute("href", &download_data)
        }
        .and_then(|_| {
            // filename
            elems
                .a_save_trigger
                .set_attribute("download", &download_fname)
        })
        .map(|_| {
            // trigger download
            elems.a_save_trigger.click()
        }),
    );

    [
        elems.input_seed,
        elems.input_is_seed_ethaddr,
        elems.input_save_dim_width,
        elems.input_save_dim_height,
        elems.input_save_button,
    ]
    .iter()
    .for_each(|input| res_unwrap(input.remove_attribute("disabled")));

    res_unwrap(elems.figure_blockies.class_list().remove_1("loading"));
}

#[wasm_bindgen]
pub fn refresh_input_blockies() {
    let elems = HtmlElems::get();

    // set/remove ethaddr according to is_ethaddr check
    match elems.input_is_seed_ethaddr.checked() {
        true => {
            res_unwrap(elems.figure_blockies.class_list().add_1("ethaddr"));
            res_unwrap(elems.input_seed.class_list().add_1("ethaddr"));
        }
        false => {
            res_unwrap(elems.figure_blockies.class_list().remove_1("ethaddr"));
            res_unwrap(elems.input_seed.class_list().remove_1("ethaddr"));
        }
    };

    let seed = elems.input_seed.value();

    refresh_blockies(
        &elems.figure_blockies,
        seed.as_str(),
        elems.input_is_seed_ethaddr.checked(),
    );
}

// callbacks //

fn build_blockies(doc: &Document, is_seed_ethaddr: bool) -> Element {
    new_elem(
        doc,
        "div",
        &[("class", Some("wrapper-blockies-card"))],
        None,
        &[
            new_elem(
                doc,
                "label",
                &[("class", Some("wrapper-is-seed-ethaddr"))],
                None,
                &[
                    new_elem(
                        doc,
                        "input",
                        &[
                            ("type", Some("checkbox")),
                            (
                                match is_seed_ethaddr {
                                    true => "checked",
                                    false => "data-not-checked",
                                },
                                None,
                            ),
                            ("disabled", None),
                        ],
                        None,
                        &[],
                    ),
                    new_elem(doc, "span", &[], Some("is ETH-addr"), &[]),
                ],
            ),
            new_elem(
                doc,
                "figure",
                &[
                    (
                        "class",
                        Some(match is_seed_ethaddr {
                            true => "blockies-card ethaddr",
                            false => "blockies-card",
                        }),
                    ),
                    ("name", Some("blockies-card")),
                ],
                None,
                &[
                    new_elem(
                        doc,
                        "a",
                        &[
                            ("name", Some("blockies-img-link")),
                            ("target", Some("_blank")),
                        ],
                        None,
                        &[new_elem(
                            doc,
                            "img",
                            &[("name", Some("blockies-img"))],
                            None,
                            &[],
                        )],
                    ),
                    new_elem(
                        doc,
                        "figcaption",
                        &[("name", Some("blockies-img-caption"))],
                        None,
                        &[],
                    ),
                ],
            ),
        ],
    )
}

fn refresh_blockies(blockies_card: &HtmlElement, seed_raw: &str, is_seed_ethaddr: bool) {
    use eth_blockies::*;
    let is_seed_valid = match is_seed_ethaddr {
        true => is_seed_valid_ethaddr(seed_raw),
        false => true,
    };
    let seed = match is_seed_ethaddr {
        true => seed_raw.canonicalize_ethaddr(),
        false => seed_raw.to_string(),
    };

    // get children elems
    let (img, img_link) = opt_unwrap(
        blockies_card
            .children()
            .get_with_name("blockies-img-link")
            .and_then(|img_link| {
                img_link
                    .children()
                    .get_with_name("blockies-img")
                    .map(|img| (img, img_link))
            }),
    );
    let caption = opt_unwrap(
        blockies_card
            .children()
            .get_with_name("blockies-img-caption"),
    );

    let dimension = {
        let rect = img.get_bounding_client_rect();

        (rect.width() as usize, rect.height() as usize)
    };

    // set/remove invalid-addr according to is_seed_valid
    res_unwrap(match is_seed_valid {
        true => blockies_card.class_list().remove_1("invalid-addr"),
        false => blockies_card.class_list().add_1("invalid-addr"),
    });

    // set link to etherscan.io, if seed == ethaddr && seed is valid ethereum address
    match is_seed_ethaddr && is_seed_valid {
        true => res_unwrap(img_link.set_attribute("href", &etherscan_link(&seed))),
        false => res_unwrap(img_link.remove_attribute("href")),
    }

    // set img src
    let blockies_data_uri = eth_blockies_data_uri(&seed, dimension);
    res_unwrap(img.set_attribute("src", &blockies_data_uri));

    caption.set_text_content(Some(&seed));
}

// other helpers

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
fn eth_blockies_data_uri(seed: &str, dimension: (usize, usize)) -> String {
    use eth_blockies::*;
    eth_blockies_png_data_base64(seed, dimension, true, true)
}

fn is_seed_valid_ethaddr(addr: &str) -> bool {
    match addr.get(0..2) {
        Some("0x") | Some("0X") => addr.get(2..),
        _ => addr.get(..),
    }
    .filter(|addr_wo_prefix| {
        addr_wo_prefix.len() == 40 && addr_wo_prefix.chars().all(|c| c.is_ascii_hexdigit())
    })
    .is_some()
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
