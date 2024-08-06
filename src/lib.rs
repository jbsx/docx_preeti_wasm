//mod tests;

use std::collections::HashMap;
use std::error::Error;
use std::io::{Cursor, Write};
use std::{io::Read, panic};
use wasm_bindgen::prelude::*;

use anyhow::Result;
use htmlescape::decode_html;
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
    pre_rules: Vec<Vec<String>>,
    character_map: HashMap<String, String>,
    post_rules: Vec<Vec<String>>,
}

static PREETI_RULES: Lazy<Map> = Lazy::new(|| {
    return serde_json::from_str(std::include_str!("preeti.json")).unwrap();
});

static UNICODE_RULES: Lazy<Map> = Lazy::new(|| {
    return serde_json::from_str(std::include_str!("unicode.json")).unwrap();
});

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

fn convert_xml_string_preeti(input: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut reader = Reader::from_str(&input);
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut is_preeti = false;

    loop {
        match reader.read_event() {
            Ok(Event::Text(e)) => {
                if is_preeti {
                    let converted = preeti_to_unicode(e.unescape()?.to_string());
                    let elem = BytesText::new(&converted);
                    writer.write_event(Event::Text(elem))?;

                    is_preeti = false;
                } else {
                    writer.write_event(Event::Text(e))?;
                }
            }
            Ok(Event::Empty(e)) => {
                if &e.name() == &QName(b"w:rFonts") {
                    let e_buf = &e.to_vec();
                    let streeng = String::from_utf8_lossy(e_buf);
                    if streeng.contains("w:ascii=\"Preeti\"") {
                        is_preeti = true;

                        writer.write_event(Event::Empty(BytesStart::new(
                            &streeng.replace("Preeti", "Arial"),
                        )))?;
                    } else {
                        writer.write_event(Event::Empty(e))?;
                    }
                } else {
                    writer.write_event(Event::Empty(e))?;
                }
            }
            Ok(Event::End(e)) => {
                if &e.name() == &QName(b"w:r") || &e.name() == &QName(b"w:pPr") {
                    is_preeti = false;
                }
                writer.write_event(Event::End(e))?;
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                writer.write_event(e)?;
            }
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
    }

    let converted_file = writer.into_inner().into_inner();
    return Ok(converted_file);
}

fn convert_xml_string_unicode(input: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut reader = Reader::from_str(&input);
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    loop {
        match reader.read_event() {
            Ok(Event::Text(e)) => {
                let converted = unicode_to_preeti(e.unescape()?.to_string());
                let elem = BytesText::new(&converted);
                writer.write_event(Event::Text(elem))?;
            }
            Ok(Event::Eof) => break,
            Ok(e) => {
                writer.write_event(e)?;
            }
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
    }

    let converted_file = writer.into_inner().into_inner();
    return Ok(converted_file);
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
    for i in &PREETI_RULES.post_rules {
        let re = Regex::new(&i[0]).unwrap();
        res = re.replace_all(&res, &i[1]).to_string();
    }

    return res;
}

#[wasm_bindgen]
pub fn preeti_to_unicode_docx(input: Vec<u8>) -> Vec<u8> {
    let file = Cursor::new(input);
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut streeng_file = String::new();
    let _ = archive
        .by_name("word/document.xml")
        .unwrap()
        .read_to_string(&mut streeng_file);

    let converted = convert_xml_string_preeti(streeng_file).unwrap();

    let buf = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(buf);
    for i in archive.to_owned().file_names() {
        match i {
            "word/document.xml" => {
                let _ = writer
                    .start_file(
                        "word/document.xml",
                        zip::write::SimpleFileOptions::default(),
                    )
                    .unwrap();
                let _ = writer.write(&converted);
                let _ = writer.flush().unwrap();
            }
            _ => {
                let f = archive.by_name(i).unwrap();
                let _ = writer.raw_copy_file(f).unwrap();
            }
        }
    }
    let res = writer
        .finish_into_readable()
        .unwrap()
        .into_inner()
        .into_inner();

    return res;
}

#[wasm_bindgen]
pub fn unicode_to_preeti(input: String) -> String {
    //normalise html entities
    let mut normalised_input: String = decode_html(&input).unwrap_or(input);

    //pre rules
    for i in &UNICODE_RULES.pre_rules {
        let re = Regex::new(&i[0]).unwrap();
        normalised_input = re.replace_all(&normalised_input, &i[1]).to_string();
    }

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
                        }
                    }
                }
                idx += 1;
                continue;
            }

            if idx < chars.len() - 2 {
                if chars[idx + 2] == 'ि' {
                    if "WERTYUXASDGHJK:ZVN".contains(curr) {
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
                                idx += 2;
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
                                idx += 2;
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
                                        res.push_str(&format!("l{}{}", t, &chars[idx + 2]));
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

        //println!("{:?} --- {}", curr, &res);
    }

    //post rules
    for i in &UNICODE_RULES.post_rules {
        let re = Regex::new(&i[0]).unwrap();
        res = re.replace_all(&res, &i[1]).to_string();
    }

    return res;
}

#[wasm_bindgen]
pub fn unicode_to_preeti_docx(input: Vec<u8>) -> Vec<u8> {
    let file = Cursor::new(input);
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut streeng_file = String::new();
    let _ = archive
        .by_name("word/document.xml")
        .unwrap()
        .read_to_string(&mut streeng_file);

    let converted = convert_xml_string_unicode(streeng_file).unwrap();

    let buf = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(buf);
    for i in archive.to_owned().file_names() {
        match i {
            "word/document.xml" => {
                let _ = writer
                    .start_file(
                        "word/document.xml",
                        zip::write::SimpleFileOptions::default(),
                    )
                    .unwrap();
                let _ = writer.write(&converted);
                let _ = writer.flush().unwrap();
            }
            _ => {
                let f = archive.by_name(i).unwrap();
                let _ = writer.raw_copy_file(f).unwrap();
            }
        }
    }
    let res = writer
        .finish_into_readable()
        .unwrap()
        .into_inner()
        .into_inner();

    return res;
}
