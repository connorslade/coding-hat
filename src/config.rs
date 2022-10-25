use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use simple_config_parser::Config as Cfg;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub workers: usize,

    pub docker_command: String,
    pub docker_timeout: String,
    pub tmp_folder: String,

    pub req_duration: Duration,
    pub problems_path: String,
}

impl Config {
    pub fn load<T: AsRef<Path>>(path: T) -> Self {
        let cfg = Cfg::new().file(path).unwrap();
        Self {
            host: get_config(&cfg, "host"),
            port: get_config(&cfg, "port"),
            database: get_config(&cfg, "database"),
            workers: get_config(&cfg, "workers"),

            docker_command: get_config(&cfg, "docker_command"),
            docker_timeout: get_config(&cfg, "docker_timeout"),
            tmp_folder: get_config(&cfg, "tmp_folder"),

            req_duration: Duration::from_secs(get_config(&cfg, "req_duration")),
            problems_path: get_config(&cfg, "problems_path"),
        }
    }
}

fn get_config<T: FromStr>(cfg: &Cfg, name: &str) -> T {
    cfg.get(name)
        .unwrap_or_else(|_| panic!("Error getting `{}` from Config", name))
}
