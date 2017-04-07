extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use weld;

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Configuration {
    pub server: Server,
    pub database: Database,
}

impl Configuration {
    pub fn new(path: &String) -> Configuration {
        if path != "" {
            info!(weld::ROOT_LOGGER, "Configuration - Reading Path: {:?}", &path);
            let mut file = File::open(path).expect("Configuration - Error Can't read provided configuration. Terminating...");
            let mut contents = String::new();
            match file.read_to_string(&mut contents){
                Ok(size)=>{
                    if size == 0 {
					    panic!("Configuration - Error Empty File Terminating...");
				    }
                    let config: Configuration = serde_json::from_str(&contents).expect("Configuration - Error Invalid JSON format. Terminating...");
                    info!(weld::ROOT_LOGGER, "Configutation - Ok");
                    debug!(weld::ROOT_LOGGER, "{:?}", &config);
                    return config;
                }
                Err(e)=>{	
                    error!(weld::ROOT_LOGGER, "Configuration - Error : {}", e);
				panic!("Configuration - Error Terminating...");}
            }
        } else {
            return Configuration {
                       database: Database { path: "db.json".to_string() },
                       server: Server {
                           listeners: vec![Listener {
                                               port: 8080,
                                               host: "127.0.0.1".to_string(),
                                               ltype: ListenerType::http,
                                           }],
                       },
                   };
        }
    }
    pub fn load(&mut self, path: &String) {
        let configuration = Configuration::new(&path);
        self.server = configuration.server;
        self.database = configuration.database;
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Server {
    pub listeners: Vec<Listener>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub enum ListenerType {
    http,
    https,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Listener {
    #[serde(rename = "type")]
    pub ltype: ListenerType,
    pub host: String,
    pub port: i16,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Database {
    pub path: String,
}

