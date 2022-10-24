use parking_lot::Mutex;
use rusqlite::Connection;

use crate::config::Config;

pub struct App {
    pub config: Config,
    pub db: Mutex<Connection>,
}

impl App {
    pub fn new() -> Self {
        let config = Config::load("./data/config.cfg");
        App {
            db: Mutex::new(Connection::open(&config.database).unwrap()),
            config,
        }
    }
}
