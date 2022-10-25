use std::collections::HashMap;
use std::fs;

use parking_lot::Mutex;
use rusqlite::Connection;

use crate::{config::Config, problem::Problem};

pub struct App {
    /// System config
    pub config: Config,
    /// Database
    pub db: Mutex<Connection>,

    /// Problem Map
    /// ID -> PROBLEM
    pub problems: HashMap<String, Problem>,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load("./data/config.cfg");
        let mut problems = HashMap::new();

        for i in fs::read_dir(&config.problems_path)
            .unwrap()
            .map(|x| x.unwrap())
            .filter(|x| {
                x.path().is_file() && x.path().extension().and_then(|x| x.to_str()) == Some("prb")
            })
        {
            let name = i.file_name().to_string_lossy().to_string();
            let raw = fs::read_to_string(i.path()).unwrap();
            problems.insert(
                name.rsplit_once('.').unwrap().0.to_owned(),
                Problem::load(raw, &name),
            );
        }

        println!(
            "📜 Loaded {} problem{}",
            problems.len(),
            if problems.len() == 1 { "" } else { "s" }
        );

        App {
            db: Mutex::new(Connection::open(&config.database).unwrap()),
            problems,
            config,
        }
    }
}
