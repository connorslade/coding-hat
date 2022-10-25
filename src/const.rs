use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use serde_json::{from_str, Value};

lazy_static! {
    pub static ref LANGS: HashMap<String, (String, String)> = {
        let raw_langs: Value = from_str(&fs::read_to_string("langs/languages.json").unwrap())
            .expect("Error parsing langs/languages.json");
        let mut langs = HashMap::new();

        for i in raw_langs.as_array().unwrap() {
            langs.insert(
                i.get("name").unwrap().as_str().unwrap().to_owned(),
                (
                    i.get("imageName").unwrap().as_str().unwrap().to_owned(),
                    i.get("sourcePath").unwrap().as_str().unwrap().to_owned(),
                ),
            );
        }

        langs
    };
}
