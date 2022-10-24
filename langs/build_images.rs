// cargo-deps: serde_json = "1.0.79"

use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

use serde_json::{from_str, Value};

fn main() {
    let base_dir = env::current_dir().unwrap();
    let langs = load_langs("languages.json");

    for i in langs {
        println!("[BUILDNIG] {}", i.0);
        env::set_current_dir(base_dir.join(i.1)).unwrap();

        let run = Command::new("docker")
            .args(["build", "-t", &i.2, "."])
            .status()
            .unwrap();

        if !run.success() {
            println!("[ERROR] exiting");
            process::exit(-1);
        }
    }
}

fn load_langs<T: AsRef<Path>>(file: T) -> Vec<(String, String, String)> {
    let raw_langs: Value =
        from_str(&fs::read_to_string(file).unwrap()).expect("Error parsing langs/languages.json");
    let mut out = Vec::new();

    for i in raw_langs.as_array().unwrap().to_owned() {
        out.push((
            i.get("name").unwrap().as_str().unwrap().to_owned(),
            i.get("path").unwrap().as_str().unwrap().to_owned(),
            i.get("imageName").unwrap().as_str().unwrap().to_owned(),
        ));
    }

    out
}