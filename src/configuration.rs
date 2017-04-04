extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use slog::Logger;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Configuration {
    pub server: Server,
}

impl Configuration {
    pub fn new(path: &String, root_logger: &Logger) -> Configuration {
        info!(&root_logger, "Reading configuration : {:?}", &path);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents);

        let config: Configuration = serde_json::from_str(&contents).unwrap();
        info!(root_logger, "Configutation Loaded");
        debug!(root_logger, "{:?}", &config);

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

