use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

mod const_generic_call_mapper;

struct HtmlElems {
    figure_blockies: HtmlElement,
    input_seed: HtmlInputElement,
    input_genseed_button: HtmlInputElement,
    input_is_seed_ethaddr: HtmlInputElement,
    select_resol: HtmlSelectElement,
    input_save_dim_width: HtmlInputElement,
    input_save_dim_height: HtmlInputElement,
    input_save_button: HtmlInputElement,
    a_save_trigger: HtmlElement,
    section_examples: HtmlElement,
}

struct HtmlElemsRef<'a> {
    figure_blockies: &'a HtmlElement,
    input_seed: &'a HtmlInputElement,
    input_genseed_button: &'a HtmlInputElement,
    input_is_seed_ethaddr: &'a HtmlInputElement,
    select_resol: &'a HtmlSelectElement,
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
        input_genseed_button: HtmlInputElement,
        input_is_seed_ethaddr: HtmlInputElement,
        select_resol: HtmlSelectElement,
        input_save_dim_width: HtmlInputElement,
        input_save_dim_height: HtmlInputElement,
        input_save_button: HtmlInputElement,
        a_save_trigger: HtmlElement,
        section_examples: HtmlElement,
    ) -> Self {
        Self {
            figure_blockies,
            input_seed,
            input_genseed_button,
            input_is_seed_ethaddr,
            select_resol,
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
                    input_genseed_button: &v.input_genseed_button,
                    input_is_seed_ethaddr: &v.input_is_seed_ethaddr,
                    select_resol: &v.select_resol,
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

const CALL_ARR: const_generic_call_mapper::CallArr<Vec<u8>> =
    const_generic_call_mapper::init_const_blockies_arr();

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
                opt_unwrap(doc.get_element_by_id("input-genseed-button"))
                    .clone()
                    .dyn_into::<HtmlInputElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("input-is-seed-ethaddr"))
                    .clone()
                    .dyn_into::<HtmlInputElement>(),
            ),
            res_unwrap(
                opt_unwrap(doc.get_element_by_id("select-resol"))
                    .clone()
                    .dyn_into::<HtmlSelectElement>(),
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
        (b"eth-blockies-rs".to_vec(), Some(8)),
        (b"plain text example".to_vec(), Some(gen_random_resol())),
        (
            b"example-your-email-addr@example.com".to_vec(),
            Some(gen_random_resol()),
        ),
        (gen_random_str(), Some(gen_random_resol())),
        (gen_random_str(), Some(gen_random_resol())),
        (gen_random_str(), Some(gen_random_resol())),
        (
            b"0x4bbeEB066eD09B7AEd07bF39EEe0460DFa261520".to_vec(),
            Some(8),
        ),
        (b"0x4bbeEB066eD09B7AEd07bF39EEe0460DFa261520".to_vec(), None),
        (b"0x*_This_invalid_address_is_marked_in_red*".to_vec(), None),
        (b"0x0000000000000000000000000000000000000000".to_vec(), None),
        (b"0xe686c14FF9C11038F2B1c9aD617F2346CFB817dC".to_vec(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
        (gen_random_ethaddr(), None),
    ]
    .iter()
    .for_each(|(seed, resol_info)| {
        opt_unwrap({
            let (resolution, is_seed_ethaddr) = match resol_info {
                Some(r) => (*r, false),
                None => (8, true),
            };
            let blockies_wrapper = build_blockies(doc, is_seed_ethaddr, resolution);
            blockies_wrapper
                .children()
                .get_with_name("blockies-card")
                .and_then(|b| b.dyn_into::<HtmlElement>().ok())
                .and_then(|b| {
                    blockies_examples
                        .append_child(&blockies_wrapper)
                        .map(|_| refresh_blockies(&b, resolution, seed, is_seed_ethaddr))
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
                    elems.input_genseed_button,
                    elems.input_is_seed_ethaddr,
                    elems.input_save_dim_width,
                    elems.input_save_dim_height,
                    elems.input_save_button,
                ]
                .iter()
                .for_each(|input| {
                    res_unwrap(input.set_attribute("disabled", ""));
                });

                res_unwrap(elems.select_resol.set_attribute("disabled", ""));
            }),
    );
}

// generate blockies, and re-enable disabled inputs
#[wasm_bindgen]
pub fn save_blockies() {
    let elems = HtmlElems::get();

    let seed = elems.input_seed.value();

    let is_seed_ethaddr = elems.input_is_seed_ethaddr.checked();
    let is_seed_valid = match is_seed_ethaddr {
        true => is_seed_valid_ethaddr(seed.as_bytes()),
        false => true,
    };

    let seed_raw = {
        use eth_blockies::*;
        match is_seed_ethaddr && is_seed_valid {
            true => seed.to_ethaddr_seed().to_vec(),
            false => seed.as_bytes().to_owned(),
        }
    };

    let resolution_str = &elems.select_resol.value();
    let resolution = resolution_str
        .parse::<usize>()
        .ok()
        .filter(|v| const_generic_call_mapper::RESOLUTION_RANGE.contains(v)) // only valid in range
        .filter(|_| !is_seed_ethaddr)
        .zip(Some(resolution_str.to_owned()))
        .unwrap_or((8, "8".to_owned()));
    let dimension_str = (
        &elems.input_save_dim_width.value(),
        &elems.input_save_dim_height.value(),
    );
    let dimension = (
        dimension_str
            .0
            .parse::<usize>()
            .ok()
            .filter(|v| *v > 0) // invalid if 0
            .zip(Some(dimension_str.0.to_owned()))
            .unwrap_or((128, "128".to_owned())),
        dimension_str
            .1
            .parse::<usize>()
            .ok()
            .filter(|v| *v > 0) // invalid if 0
            .zip(Some(dimension_str.1.to_owned()))
            .unwrap_or((128, "128".to_owned())),
    );

    let download_data =
        eth_blockies_data_uri(resolution.0, &seed_raw, (dimension.0 .0, dimension.1 .0));
    let download_fname = match is_seed_ethaddr && is_seed_valid {
        true => [
            "eth-blockies-rs_eth_d",
            &dimension.0 .1,
            "x",
            &dimension.1 .1,
            "_",
            &addr_to_cksumaddr(&res_unwrap(String::from_utf8(seed_raw))),
        ]
        .concat(),
        false => [
            "eth-blockies-rs_r",
            &resolution.1,
            "_d",
            &dimension.0 .1,
            "x",
            &dimension.1 .1,
            "_",
            &String::from_utf8_lossy(&seed_raw),
        ]
        .concat(),
    };

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
        elems.input_genseed_button,
        elems.input_is_seed_ethaddr,
        elems.input_save_dim_width,
        elems.input_save_dim_height,
        elems.input_save_button,
    ]
    .iter()
    .for_each(|input| res_unwrap(input.remove_attribute("disabled")));

    if !elems.input_is_seed_ethaddr.checked() {
        res_unwrap(elems.select_resol.remove_attribute("disabled"));
    }

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

            // select resolution box
            elems.select_resol.set_disabled(true);
        }
        false => {
            res_unwrap(elems.figure_blockies.class_list().remove_1("ethaddr"));
            res_unwrap(elems.input_seed.class_list().remove_1("ethaddr"));

            // select resolution box
            if !elems.figure_blockies.class_list().contains("loading") {
                elems.select_resol.set_disabled(false);
            }
        }
    };

    let resolution = elems
        .select_resol
        .value()
        .parse::<usize>()
        .ok()
        .filter(|v| const_generic_call_mapper::RESOLUTION_RANGE.contains(v)) // only valid in range
        .filter(|_| !elems.input_is_seed_ethaddr.checked())
        .unwrap_or(8);

    let seed = elems.input_seed.value();

    refresh_blockies(
        &elems.figure_blockies,
        resolution,
        seed.as_bytes(),
        elems.input_is_seed_ethaddr.checked(),
    );
}

#[wasm_bindgen]
pub fn gen_input_seed() {
    let elems = HtmlElems::get();

    elems
        .input_seed
        .set_value(&match elems.input_is_seed_ethaddr.checked() {
            true => res_unwrap(String::from_utf8(gen_random_ethaddr())),
            false => res_unwrap(String::from_utf8(gen_random_str())),
        });

    refresh_input_blockies();
}

// callbacks //

fn build_blockies(doc: &Document, is_seed_ethaddr: bool, resolution: usize) -> Element {
    new_elem(
        doc,
        "div",
        &[("class", Some("wrapper-blockies-card"))],
        None,
        &[
            new_elem(
                doc,
                "div",
                &[("class", Some("wrapper-resol"))],
                None,
                &[
                    new_elem(
                        doc,
                        "p",
                        &[],
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
                                            false => "",
                                        },
                                        None,
                                    ),
                                    ("disabled", None),
                                ],
                                None,
                                &[],
                            ),
                            new_elem(doc, "label", &[], Some(" ETH-addr"), &[]),
                        ],
                    ),
                    new_elem(
                        doc,
                        "p",
                        &[],
                        None,
                        &[
                            new_elem(doc, "span", &[], Some("Size: "), &[]),
                            new_elem(
                                doc,
                                "select",
                                &[("disabled", None)],
                                None,
                                &gen_select_options(doc, resolution),
                            ),
                        ],
                    ),
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

fn refresh_blockies(
    blockies_card: &HtmlElement,
    resolution: usize,
    seed_raw: &[u8],
    is_seed_ethaddr: bool,
) {
    use eth_blockies::*;
    let is_seed_valid = match is_seed_ethaddr {
        true => is_seed_valid_ethaddr(seed_raw),
        false => true,
    };
    let seed = match is_seed_ethaddr && is_seed_valid {
        true => seed_raw.to_ethaddr_seed().to_vec(),
        false => seed_raw.to_vec(),
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
        true => res_unwrap(
            img_link.set_attribute("href", &etherscan_link(&String::from_utf8_lossy(&seed))),
        ),
        false => res_unwrap(img_link.remove_attribute("href")),
    }

    // set img src
    let blockies_data_uri = eth_blockies_data_uri(resolution, &seed, dimension);
    res_unwrap(img.set_attribute("src", &blockies_data_uri));

    caption.set_text_content(
        String::from_utf8(seed)
            .ok()
            .map(|s| match is_seed_ethaddr && is_seed_valid {
                true => addr_to_cksumaddr(&s),
                false => s,
            })
            .as_deref(),
    );
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
        if !k.is_empty() {
            res_unwrap(elem.set_attribute(k, v.unwrap_or_default()));
        }
    });
    elem.set_text_content(text);
    children.iter().for_each(|child| {
        res_unwrap(elem.append_child(child));
    });
    elem
}

fn gen_select_options(doc: &Document, default: usize) -> Vec<Element> {
    const_generic_call_mapper::RESOLUTION_RANGE
        .map(|i| {
            let i_bstr = [
                {
                    let upper_val = i / 10;
                    match upper_val {
                        0 => b' ',
                        _ => upper_val as u8 + b'0',
                    }
                },
                (i % 10) as u8 + b'0',
            ];
            let val = String::from_utf8_lossy(&i_bstr);
            new_elem(
                doc,
                "option",
                &[
                    ("value", Some(&val)),
                    match i == default {
                        true => ("selected", None),
                        false => ("", None),
                    },
                ],
                Some(&val),
                &[],
            )
        })
        .collect()
}

fn gen_random_resol() -> usize {
    let mut num: [u8; 1] = [0; 1];
    res_unwrap(getrandom::getrandom(&mut num));

    // returns 4..20
    (num[0] as usize
        % (const_generic_call_mapper::MAX_RESOLUTION - const_generic_call_mapper::MIN_RESOLUTION
            + 1))
        + const_generic_call_mapper::MIN_RESOLUTION
}

fn gen_random_ethaddr() -> Vec<u8> {
    use eth_blockies::SeedInput;
    let mut bytes: [u8; 20] = [0; 20];
    res_unwrap(getrandom::getrandom(&mut bytes));
    bytes.to_ethaddr_seed().to_vec()
}

fn gen_random_str() -> Vec<u8> {
    const CHAR_LIST: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut bytes: [u8; 42] = [0; 42];
    res_unwrap(getrandom::getrandom(&mut bytes));
    bytes
        .iter()
        .map(|b| CHAR_LIST[(b & 0b00111111) as usize])
        .collect()
}

fn addr_to_cksumaddr(addr: &str) -> String {
    use tiny_keccak::{Hasher, Keccak};

    fn encode_hex(input: &[u8]) -> String {
        let mut ret: String = String::with_capacity(input.len() * 2);

        const HEX_TABLE: &[char; 16] = &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        input.iter().for_each(|byte| {
            ret.push(HEX_TABLE[(byte >> 4) as usize]);
            ret.push(HEX_TABLE[(byte & 0x0F) as usize]);
        });

        ret
    }

    let wallet_addr = addr.trim_start_matches("0x").to_ascii_lowercase();

    let mut addr_hashed = [0u8; 32];
    let mut keccak = Keccak::v256();
    keccak.update(wallet_addr.as_bytes());
    keccak.finalize(&mut addr_hashed);

    [
        "0x",
        &wallet_addr
            .chars()
            .zip(encode_hex(&addr_hashed).chars())
            .map(|(c, hash_c)| match c {
                'a'..='f' => match hash_c {
                    '8'..='9' | 'a'..='f' => c.to_ascii_uppercase(),
                    _ => c,
                },
                _ => c,
            })
            .collect::<String>(),
    ]
    .concat()
}

// Return data uri of Ethereum blockies for given address
fn eth_blockies_data_uri(resolution: usize, seed: &[u8], dimension: (usize, usize)) -> String {
    CALL_ARR[resolution - const_generic_call_mapper::MIN_RESOLUTION](
        seed.to_owned(),
        dimension,
        true,
    )
}

fn is_seed_valid_ethaddr(addr: &[u8]) -> bool {
    match addr.get(0..2) {
        Some(b"0x") | Some(b"0X") => addr.get(2..),
        _ => addr.get(..),
    }
    .filter(|addr_wo_prefix| {
        addr_wo_prefix.len() == 40 && addr_wo_prefix.iter().all(|c| c.is_ascii_hexdigit())
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
