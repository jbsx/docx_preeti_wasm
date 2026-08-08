mod tests;

use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::{io::Read, panic};
use wasm_bindgen::prelude::*;

use anyhow::Result;
use htmlescape::decode_html;
use js_sys::{SharedArrayBuffer, Uint8Array};
use once_cell::sync::Lazy;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use regex::Regex;
use serde::{Deserialize, Serialize};

extern crate console_error_panic_hook;

#[derive(Serialize, Deserialize, Debug)]
struct Map {
    character_map: HashMap<String, String>,
    post_rules: Vec<Vec<String>>,
}

static PREETI_RULES: Lazy<Map> = Lazy::new(|| {
    return serde_json::from_str(std::include_str!("preeti.json")).unwrap();
});

static UNICODE_RULES: Lazy<Map> = Lazy::new(|| {
    return serde_json::from_str(std::include_str!("unicode.json")).unwrap();
});

static PREETI_POST_RULES: Lazy<Vec<(Regex, String)>> = Lazy::new(|| {
    return PREETI_RULES
        .post_rules
        .iter()
        .map(|i| (Regex::new(&i[0]).unwrap(), i[1].clone()))
        .collect();
});

static UNICODE_POST_RULES: Lazy<Vec<(Regex, String)>> = Lazy::new(|| {
    return UNICODE_RULES
        .post_rules
        .iter()
        .map(|i| (Regex::new(&i[0]).unwrap(), i[1].clone()))
        .collect();
});

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone, Copy)]
enum Direction {
    PreetiToUnicode,
    UnicodeToPreeti,
}

fn convert_xml_string(
    input: &str,
    loading: Option<SharedArrayBuffer>,
    direction: Direction,
) -> Result<Vec<u8>> {
    let mut reader = Reader::from_str(input);
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut convert = false;

    let mut completion = 0;
    loop {
        //progress bar
        if !input.is_empty() {
            let curr = (reader.buffer_position() * 100) / input.len();
            if curr != completion {
                completion = curr;
                if let Some(s) = &loading {
                    let load_percent = Uint8Array::new(s);
                    load_percent.set_index(0, curr as u8);
                }
            }
        }

        match reader.read_event() {
            Ok(Event::Text(e)) => {
                if convert {
                    let text = e.unescape()?.to_string();
                    let converted = match direction {
                        Direction::PreetiToUnicode => preeti_to_unicode(text),
                        Direction::UnicodeToPreeti => unicode_to_preeti(text),
                    };
                    let elem = BytesText::new(&converted);
                    writer.write_event(Event::Text(elem))?;

                    if matches!(direction, Direction::PreetiToUnicode) {
                        convert = false;
                    }
                } else {
                    writer.write_event(Event::Text(e))?;
                }
            }
            Ok(Event::Empty(e)) => {
                if &e.name() == &QName(b"w:rFonts") {
                    let e_buf = &e.to_vec();
                    let fonts_str = String::from_utf8_lossy(e_buf);
                    let fonts = match direction {
                        Direction::PreetiToUnicode => {
                            if !fonts_str.contains("w:ascii=\"Preeti\"") {
                                writer.write_event(Event::Empty(e))?;
                                continue;
                            }
                            fonts_str.replace("Preeti", "Arial")
                        }
                        Direction::UnicodeToPreeti => {
                            let mut fonts = fonts_str.to_string();
                            if let Some(from) = fonts.find("w:ascii=\"") {
                                let to = fonts[from + 9..]
                                    .find('"')
                                    .ok_or(anyhow::anyhow!("malformed w:rFonts element"))?;
                                fonts.replace_range(from + 9..from + 9 + to, "Preeti");
                            }
                            fonts
                        }
                    };

                    convert = true;

                    writer.write_event(Event::Empty(BytesStart::new(fonts)))?;
                } else {
                    writer.write_event(Event::Empty(e))?;
                }
            }
            Ok(Event::End(e)) => {
                if &e.name() == &QName(b"w:r") || &e.name() == &QName(b"w:pPr") {
                    convert = false;
                }
                writer.write_event(Event::End(e))?;
            }
            Ok(Event::Eof) => {
                writer.write_event(Event::Eof)?;
                break;
            }
            Ok(e) => {
                writer.write_event(e)?;
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "XML error at position {}: {:?}",
                    reader.buffer_position(),
                    e
                ))
            }
        }
    }

    return Ok(writer.into_inner().into_inner());
}

fn convert_docx(
    input: Vec<u8>,
    loading: Option<SharedArrayBuffer>,
    direction: Direction,
) -> Result<Vec<u8>> {
    let file = Cursor::new(input);
    let mut archive = zip::ZipArchive::new(file)?;

    let mut document_xml = String::new();
    archive
        .by_name("word/document.xml")?
        .read_to_string(&mut document_xml)?;

    let converted = convert_xml_string(&document_xml, loading, direction)?;

    let names: Vec<String> = archive.file_names().map(|s| s.to_owned()).collect();

    let buf = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(buf);
    for name in names {
        if name == "word/document.xml" {
            writer.start_file(
                "word/document.xml",
                zip::write::SimpleFileOptions::default(),
            )?;
            writer.write_all(&converted)?;
        } else {
            let f = archive.by_name(&name)?;
            writer.raw_copy_file(f)?;
        }
    }

    return Ok(writer.finish()?.into_inner());
}

#[wasm_bindgen]
pub fn preeti_to_unicode(input: String) -> String {
    //normalise html entities
    let normalised_input = decode_html(&input).unwrap_or(input);

    //convert
    let mut res = String::new();
    for i in normalised_input.split("") {
        res.push_str(&PREETI_RULES.character_map.get(i).unwrap_or(&i.to_owned()));
    }

    //post rules
    for (re, replacement) in PREETI_POST_RULES.iter() {
        res = re.replace_all(&res, replacement).to_string();
    }

    return res;
}

#[wasm_bindgen]
pub fn preeti_to_unicode_docx(
    input: Vec<u8>,
    loading: Option<SharedArrayBuffer>,
) -> std::result::Result<Vec<u8>, JsError> {
    return convert_docx(input, loading, Direction::PreetiToUnicode)
        .map_err(|e| JsError::new(&e.to_string()));
}

pub fn normalise_unicode(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut idx = 0;
    let mut res = String::new();

    while idx < chars.len() {
        if chars[idx] == ':' {
            res.push('M');
            idx += 1;
            continue;
        }

        if idx + 2 < chars.len() {
            if chars[idx] != 'र' {
                if chars[idx + 1] == '्' && !" ।,".contains(chars[idx + 2]) {
                    if chars[idx + 2] != 'र' {
                        match UNICODE_RULES.character_map.get(&chars[idx].to_string()) {
                            Some(c) => {
                                if "wertyuxasdghjkzvn".contains(c) {
                                    res.push_str(&c.to_uppercase());
                                    idx += 2;
                                    continue;
                                } else if chars[idx] == 'स' {
                                    res.push(':');
                                    idx += 2;
                                    continue;
                                } else if chars[idx] == 'ष' {
                                    res.push('i');
                                    idx += 2;
                                    continue;
                                }
                            }
                            None => {
                                res.push(chars[idx]);
                                idx += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        if idx >= 1 && idx + 1 < chars.len() {
            if chars[idx - 1] != 'र' && chars[idx] == '्' && chars[idx + 1] == 'र' {
                if !"टठड".contains(chars[idx - 1]) {
                    res.push('|');
                    idx += 2;
                    continue;
                } else {
                    res.push('«');
                    idx += 2;
                    continue;
                }
            }
        }
        res.push(chars[idx]);
        idx += 1;
    }

    return res.replace("त|", "q");
}

#[wasm_bindgen]
pub fn unicode_to_preeti(input: String) -> String {
    //normalise html entities
    let normalised_input: String = normalise_unicode(decode_html(&input).unwrap_or(input));

    //convert
    let mut res = String::new();
    let mut idx = 0;
    let chars = normalised_input.chars().collect::<Vec<char>>();

    while idx < chars.len() {
        let curr = chars[idx];

        if idx < chars.len() - 1 {
            if chars[idx + 1] == 'ि' {
                if curr == 'q' {
                    res.push_str("lq");
                } else {
                    match UNICODE_RULES.character_map.get(&curr.to_string()) {
                        Some(t) => {
                            res.push_str(&format!("l{}", t));
                        }
                        None => {
                            res.push_str(&curr.to_string());
                            idx += 1;
                            continue;
                        }
                    }
                }
                idx += 2;
                continue;
            }

            if idx < chars.len() - 2 {
                if chars[idx + 2] == 'ि' {
                    if "WERTYUXASDGHJK:ZVN".contains(curr) {
                        if chars[idx + 1] == 'q' {
                            res.push_str(&format!("l{}q", curr));
                            idx += 3;
                            continue;
                        }
                        match UNICODE_RULES.character_map.get(&chars[idx + 1].to_string()) {
                            Some(t) => {
                                if t != "q" {
                                    match UNICODE_RULES
                                        .character_map
                                        .get(&chars[idx + 1].to_string())
                                    {
                                        Some(t) => {
                                            res.push_str(&format!("l{}{}", curr, t));
                                        }
                                        None => {
                                            res.push_str(&curr.to_string());
                                            idx += 1;
                                            continue;
                                        }
                                    }
                                } else {
                                    res.push_str(&format!("l{}{}", curr, chars[idx + 1]));
                                }
                                idx += 3;
                                continue;
                            }
                            None => {
                                res.push_str(&curr.to_string());
                                idx += 1;
                                continue;
                            }
                        }
                    }
                }

                if idx < chars.len() - 3 {
                    if chars[idx + 1] == '्' && curr == 'र' {
                        if chars[idx + 3] == 'ा'
                            || chars[idx + 3] == 'ो'
                            || chars[idx + 3] == 'ौ'
                            || chars[idx + 3] == 'े'
                            || chars[idx + 3] == 'ै'
                            || chars[idx + 3] == 'ी'
                        {
                            match UNICODE_RULES.character_map.get(&chars[idx + 2].to_string()) {
                                Some(p2) => {
                                    match UNICODE_RULES
                                        .character_map
                                        .get(&chars[idx + 3].to_string())
                                    {
                                        Some(p3) => {
                                            res.push_str(&format!("{}{}{{", p2, p3));
                                            idx += 4;
                                            continue;
                                        }
                                        None => {
                                            res.push_str(&curr.to_string());
                                            idx += 1;
                                            continue;
                                        }
                                    }
                                }
                                None => {
                                    res.push_str(&curr.to_string());
                                    idx += 1;
                                    continue;
                                }
                            }
                        } else if chars[idx + 3] == 'ि' {
                            match UNICODE_RULES.character_map.get(&chars[idx + 2].to_string()) {
                                Some(p2) => {
                                    match UNICODE_RULES
                                        .character_map
                                        .get(&chars[idx + 3].to_string())
                                    {
                                        Some(p3) => {
                                            res.push_str(&format!("{}{}{{", p3, p2));
                                            idx += 4;
                                            continue;
                                        }
                                        None => {
                                            res.push_str(&curr.to_string());
                                            idx += 1;
                                            continue;
                                        }
                                    }
                                }
                                None => {
                                    res.push_str(&curr.to_string());
                                    idx += 1;
                                    continue;
                                }
                            }
                        }

                        match UNICODE_RULES.character_map.get(&chars[idx + 2].to_string()) {
                            Some(t) => {
                                res.push_str(&format!("{}{{", t));
                                idx += 3;
                                continue;
                            }
                            None => {
                                res.push_str(&curr.to_string());
                                idx += 1;
                                continue;
                            }
                        }
                    }

                    if chars[idx + 3] == 'ि' {
                        if chars[idx + 2] == '|' || chars[idx + 2] == '«' {
                            if "WERTYUXASDGHJK:ZVNIi".contains(curr) {
                                match UNICODE_RULES.character_map.get(&chars[idx + 1].to_string()) {
                                    Some(t) => {
                                        res.push_str(&format!("l{}{}{}", curr, t, &chars[idx + 2]));
                                        idx += 4;
                                        continue;
                                    }
                                    None => {
                                        res.push_str(&curr.to_string());
                                        idx += 1;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        match UNICODE_RULES.character_map.get(&curr.to_string()) {
            Some(t) => {
                res.push_str(t);
            }
            None => {
                res.push_str(&curr.to_string());
            }
        }
        idx += 1;
    }

    //post rules
    for (re, replacement) in UNICODE_POST_RULES.iter() {
        res = re.replace_all(&res, replacement).to_string();
    }

    return res;
}

#[wasm_bindgen]
pub fn unicode_to_preeti_docx(
    input: Vec<u8>,
    loading: Option<SharedArrayBuffer>,
) -> std::result::Result<Vec<u8>, JsError> {
    return convert_docx(input, loading, Direction::UnicodeToPreeti)
        .map_err(|e| JsError::new(&e.to_string()));
}
