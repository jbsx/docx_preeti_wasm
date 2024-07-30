use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;
use std::panic;

//mod tests;
use htmlescape::decode_html;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Map {
    character_map: HashMap<String, String>,
    post_rules: Vec<Vec<String>>,
}

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn preeti_to_unicode(input: String, preeti_json: String) -> String {
    let rules: Map = serde_json::from_str(&preeti_json).unwrap();

    //normalise html entities
    let normalised_input = decode_html(&input).unwrap_or(input);

    //convert
    let mut res = String::new();
    for i in normalised_input.split("") {
        res.push_str(rules.character_map.get(i).unwrap_or(&"".to_owned()));
    }

    //post rules
    for i in &rules.post_rules {
        let re = Regex::new(&i[0]).unwrap();
        res = re.replace_all(&res, &i[1]).to_string();
    }

    return res;
}
