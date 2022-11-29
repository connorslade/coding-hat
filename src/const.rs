use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use serde_json::{from_str, Value};

use crate::langs;

pub struct LangInfo {
    pub image_name: String,
    pub source_path: String,
    pub info_index: usize,
}

// defult ~1 month
pub const VALID_SESSION_LENGTH: u64 = 60 * 60 * 24 * 30;

lazy_static! {
    pub static ref LANGS: HashMap<String, LangInfo> = {
        let raw_langs: Value = from_str(&fs::read_to_string("langs/languages.json").unwrap())
            .expect("Error parsing langs/languages.json");
        let mut langs = HashMap::new();

        for i in raw_langs.as_array().unwrap() {
            let name = i.get("name").unwrap().as_str().unwrap().to_owned();

            let info = langs::LANGS
                .iter()
                .enumerate()
                .find(|(_, i)| i.name() == name)
                .unwrap_or_else(|| panic!("Language `{}` not implemented", name));

            langs.insert(
                name.to_owned(),
                LangInfo {
                    image_name: i.get("imageName").unwrap().as_str().unwrap().to_owned(),
                    source_path: i.get("sourcePath").unwrap().as_str().unwrap().to_owned(),
                    info_index: info.0,
                },
            );
        }

        langs
    };
}

pub fn init() {
    lazy_static::initialize(&LANGS);

    println!("✍  Loaded {} languages", LANGS.len());
}
