extern crate serde_json;

use slog::Logger;
use configuration;
use weld::ROOT_LOGGER;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use serde_json::Value;
use serde_json::Map;



/// This is a very primitive implementation for a json db.
///Reads all the data to the memory, manipulates it and writes back.
// TODO: Use serde better for indexed updates over file.
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
    pub fn insert(&mut self, key: &str, value: Map<String, Value>) {
        let ref mut data = self.data;
        let db_option = data.as_object_mut();
        let db: &mut Map<String, Value> = db_option.unwrap();
        let en_map = db.get_mut(key).unwrap();
        let array: &mut Vec<Value> = en_map.as_array_mut().unwrap();
        array.push(serde_json::to_value(&value).unwrap());
        info!(&self.logger, "Inserted  {:?}", &value);
    }

    /// Updates the record with the given id.
    pub fn update(&self) {}

    /// Deletes the record with the given id.
    pub fn delete(&self) {}

    pub fn flush(&mut self) {
        let new_db = &serde_json::to_string(&self.data).unwrap();
        info!(&self.logger, "New Array {:?}", &new_db);

        let bytes = new_db.as_bytes();
        info!(&self.logger, "Flushing changes to the Database");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.configuration.path)
            .unwrap();
        file.set_len(0);
        file.write_all(bytes);
        let result = file.sync_all();
        info!(&self.logger, "Operation {:?} file {:?}", &result, &file);
    }

    /// Get the list of the tables
    pub fn tables(&self) -> Vec<&String> {
        let map: &serde_json::Map<String, Value> = self.data.as_object().unwrap();
        let mut keys = vec![];
        keys.extend(map.keys());
        keys
    }
}

