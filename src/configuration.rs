extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Configuration {
    pub server: Server,
}

impl Configuration {
    pub fn new(path: &String) -> Configuration {
        println!("Reading configuration : {:?}", &path);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        
        file.read_to_string(&mut contents);

        let config: Configuration = serde_json::from_str(&contents).unwrap();
        println!("Configutation Loaded {:?}", &config);
        
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

