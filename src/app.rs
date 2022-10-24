use std::collections::HashMap;

use parking_lot::{Mutex, RwLock};
use rusqlite::Connection;

use crate::{config::Config, problem::Problem};

pub struct App {
    /// System config
    pub config: Config,
    /// Database
    pub db: Mutex<Connection>,

    /// Problem Map
    /// ID -> PROBLEM
    pub problems: RwLock<HashMap<String, Problem>>,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load("./data/config.cfg");
        App {
            db: Mutex::new(Connection::open(&config.database).unwrap()),
            problems: RwLock::new(HashMap::new()),
            config,
        }
    }
}
