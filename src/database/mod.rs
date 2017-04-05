extern crate serde_json;

use slog::Logger;
use configuration;
use weld::ROOT_LOGGER;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use serde_json::Value;



#[derive(Debug)]
pub struct Database<'a> {
    logger: Logger,
    configuration: &'a configuration::Database,
    data: serde_json::Value,
}

impl<'a> Database<'a> {
    pub fn new(configuration: &'a configuration::Database) -> Database {
        let path: String = configuration.path.clone();
        Database {
            logger: ROOT_LOGGER.new(o!("database.path"=>path)),
            configuration: &configuration,
            data: serde_json::Value::Null,
        }
    }
    /// Opens a file channel.
    pub fn open(&mut self) {
        info!(self.logger,
              "Connecting database : {:?}",
              self.configuration.path);
        let mut file = File::open(&self.configuration.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        let new_data: serde_json::Value = serde_json::from_str(&contents).unwrap();
        debug!(self.logger, "{}", &new_data);
        self.data = new_data;
    }

    /// Closes the file channel.
    pub fn close(&self) {}

    /// Reads the desired level (all , entity, filters...).
    pub fn read(&self) {}

    /// Inserts the record to the desired place.
    pub fn insert(&self) {}

    /// Updates the record with the given id.
    pub fn update(&self) {}

    /// Deletes the record with the given id.
    pub fn delete(&self) {}

    /// Get the list of the tables
    pub fn tables(&self) -> Vec<&String> {
        let map: &serde_json::Map<String, Value> = self.data.as_object().unwrap();
        let mut keys = vec![];
        keys.extend(map.keys());
        keys
    }
}

