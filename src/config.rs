use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct General {
    pub domain: String,
    pub path: String,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Server {
    pub disallow_old_clients: bool,
}

#[derive(Deserialize)]
pub struct MySQL {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub database: u8,
}

#[derive(Deserialize)]
pub struct ElasticSearch {
    pub host: String,
    pub port: u16,
    pub index: String,
    pub _type: String,
}

#[derive(Deserialize)]
pub struct Services {
    pub mysql: MySQL,
    pub redis: Redis,
    pub elasticsearch: ElasticSearch,
}

#[derive(Deserialize)]
pub struct Config {
    pub General: General,
    pub Server: Server,
    pub Services: Services,
}

fn init_config() -> Config {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("/opt/nova_rs/config.yaml");

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut contents = String::new();
    match file {
        Ok(mut f) => f.read_to_string(&mut contents).unwrap(),
        Err(e) => panic!("Error reading config file: {}", e),
    };

    // Parse the string of data into serde_yaml::Value.
    let config: Config = serde_yaml::from_str(&contents).unwrap();

    return config;
}

lazy_static! {
    pub static ref CONFIG: Config = init_config();
}
