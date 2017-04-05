extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use weld;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Configuration {
    pub server: Server,
    pub database: Database,
}

impl Configuration {
    pub fn new(path: &String) -> Configuration {
        info!(weld::ROOT_LOGGER, "Reading configuration : {:?}", &path);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents);

        let config: Configuration = serde_json::from_str(&contents).unwrap();
        info!(weld::ROOT_LOGGER, "Configutation Loaded");
        debug!(weld::ROOT_LOGGER, "{:?}", &config);

        return config;
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Server {
    pub listeners: Vec<Listener>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum ListenerType {
    http,
    https,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Listener {
    #[serde(rename = "type")]
    pub ltype: ListenerType,
    pub host: String,
    pub port: i16,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Database {
    pub path: String,
}

