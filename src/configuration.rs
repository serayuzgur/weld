//! # configuration
//! This module holds all necessary structs and fn implementations for reading and parsing configuration JSON's.
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use weld;

/// Root struct. All application configuration must be defined in here.
#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Configuration {
    /// Server configuration
    pub server: Server,

    /// Database configuration
    pub database: Database,
}

impl Configuration {
    /// Creates an instance with default configuration.
    pub fn new() -> Configuration {
        Configuration {
            database: Database {
                path: "db.json".to_string(),
                default_pk: "id".to_string(),
            },
            server: Server {
                port: 8080,
                host: "127.0.0.1".to_string(),
            },
        }
    }

    /// Loads the file at the given path to the instance.
    pub fn load(&mut self, path: &str) {
        info!(weld::ROOT_LOGGER,
              "Configuration - Reading Path: {:?}",
              &path);
        let mut file = File::open(path)
            .expect("Configuration - Error Can't read provided configuration. Terminating...");
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(size) => {
                if size == 0 {
                    panic!("Configuration - Error Empty File Terminating...");
                }
                let config: Configuration = serde_json::from_str(&contents)
                    .expect("Configuration - Error Invalid JSON format. Terminating...");
                info!(weld::ROOT_LOGGER, "Configuration - Ok");
                debug!(weld::ROOT_LOGGER, "{:?}", &config);
                self.server = config.server;
                self.database = config.database;
            }
            Err(e) => {
                error!(weld::ROOT_LOGGER, "Configuration - Error : {}", e);
                panic!("Configuration - Error Terminating...");
            }
        }


    }
}


/// Server configuration. All server configuration must be defined in here.
#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Server {
    /// Host/IP address to listen
    pub host: String,

    /// Port to listen
    pub port: i16,
}

/// Database configuration. All database configuration must be defined in here.
#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
pub struct Database {
    /// Path of the desired database json file. It could be both relative or absolute.
    pub path: String,

    /// Default key to use as the PK(Primary Key) for the tables.
    pub default_pk: String,
}
