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
use serde_json::Value::{Array, Object};
use serde_json::Map;
use rand;


/// This is a very  simple database struct for a json db.
/// It works really simple. Loads all data to memory.
/// Does all the operations in the memory and writes the final object to the file at the end.
// TODO: Use serde better for indexed updates over file.
#[derive(Debug)]
pub struct Database {
    logger: Logger,
    configuration: configuration::Database,
    data: serde_json::Value,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Errors {
    NotFound(String),
    Duplicate(String),
    BadData(String),
}
impl Database {
    /// Creates an instance of the Database.
    pub fn new(configuration: &configuration::Database) -> Database {
        let path: String = configuration.path.clone();
        Database {
            logger: ROOT_LOGGER.new(o!("database.path"=>path)),
            configuration: configuration.clone(),
            data: serde_json::Value::Null,
        }
    }
    pub fn set_configuration(&mut self, configuration: &configuration::Database) {
        let path: String = configuration.path.clone();
        self.logger = ROOT_LOGGER.new(o!("database.path"=>path));
        self.configuration = configuration.clone();
        self.data = serde_json::Value::Null;
    }
    /// Parses the file and loads in to the memory.
    /// You have to call this before doing any set of operations.
    /// All failed operations results with panic because there is no meaning to continue without a proper db.
    pub fn open(&mut self) {
        info!(self.logger,
              "Database - Connecting : {:?}",
              self.configuration.path);
        let mut file = File::open(&self.configuration.path)
            .expect("Database - Error Can't read. Terminating...");
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(usize) => {
                if usize == 0 {
                    panic!("Database - Error It is empty. You can't mock API with it. \
                            Terminating...");
                }
            }
            Err(e) => panic!("Database - Error You can't mock API with it. Terminating..."),

        }
        let new_data: serde_json::Value = serde_json::from_str(&contents)
            .expect("Invalid JSON format. Check provided db. Terminating...");
        self.data = new_data;
        info!(self.logger, "Database - Ok : {:?}", self.configuration.path);
    }

    pub fn decide_id(val: &String) -> i64 {
        match i64::from_str_radix(val.as_str(), 10) {
            Ok(parsed) => parsed,
            Err(e) => -1,
        }
    }

    pub fn get_object(&self, keys: &mut Vec<String>, json_object: &Value) -> Result<Value, Errors> {
        let key = keys.remove(0);
        let mut value = Value::Null;
        match json_object {
            &Array(ref array) => {
                let id = Self::decide_id(&key);
                match Database::find_index(&array, &id) {
                    None => {
                        return Self::error(&self.logger,
                                           Errors::NotFound(format!("Read - Error  path: {:?} ",
                                                                    &key)));
                    }
                    Some(idx) => {
                        match array.get(idx) {
                            Some(obj) => {
                                value = obj.clone();
                            }
                            None => {
                                return Self::error(&self.logger,
                                                   Errors::NotFound(format!("Read - Error  \
                                                                             path: {:?} ",
                                                                            &key)));
                            }
                        }
                    }
                }
            }
            &Object(ref obj) => {
                value = match obj.get(key.as_str()) {
                    Some(obj) => obj.clone(),
                    None => {
                        return Self::error(&self.logger,
                                           Errors::NotFound(format!("Read - Error  path: {:?} ",
                                                                    &key)));
                    }
                }
            }
            _ => {
                return Self::error(&self.logger,
                                   Errors::NotFound(format!("Read - Error  path: {:?} ", &key)));
            }
        };
        if keys.len() == 0 {
            info!(&self.logger, "Read - Ok  path: {:?} last:{:?}", &keys, &key);
            debug!(&self.logger, "Read - Value {}", &value);
            return Ok(value);
        } else {
            // Consume key
            return self.get_object(keys, &mut value);
        }
    }

    /// Reads the desired result with the given path.
    pub fn read(&self, keys: &mut Vec<String>) -> Result<Value, Errors> {
        let ref data = self.data;
        match &self.get_object(keys, data) {
            &Ok(ref obj) => Ok(obj.clone()),
            &Err(ref msg) => Err(msg.clone()),
        }
    }

    /// Inserts the record to the desired place.
    pub fn insert(&mut self, key: &str, value: &mut Map<String, Value>) -> Result<Value, Errors> {
        let ref mut data = self.data;
        let db_option = data.as_object_mut();
        let db: &mut Map<String, Value> =
            db_option.expect("Database is invalid. You can't mock API with it. Terminating...");
        match db.get_mut(key) {
            Some(en_map) => {
                let array: &mut Vec<Value> = en_map.as_array_mut()
                    .expect("Table is invalid. For now it can only be Array<Map>. Terminating...");
                let mut id = rand::random();
                // If id comes with the record use it.
                match value.get("id") {
                    Some(id_value) => {
                        match id_value.as_i64() {
                            Some(parsed) => id = parsed,
                            None => {}
                        }
                    }
                    None => {}
                }

                value.insert("id".to_string(), serde_json::to_value(id).unwrap());

                match Database::find_index(array, &id) {
                    None => {
                        let as_value = serde_json::to_value(&value).unwrap();
                        array.push(as_value.clone());
                        info!(&self.logger, "Insert - Ok id: {:?}", &id);
                        debug!(&self.logger, "Insert - Value  {}", &as_value);
                        return Ok(as_value.clone());
                    }
                    Some(idx) => {
                        Self::error(&self.logger,
                                    Errors::Duplicate(format!("Insert - Error  {:?}. \"id\" \
                                                               duplicates record at index: {:?}",
                                                              &value,
                                                              idx)))
                    }
                }
            }
            None => {
                Self::error(&self.logger,
                            Errors::NotFound(format!("Table not found {}", &key)))
            }
        }
    }

    /// Updates the record with the given id.
    pub fn update(&mut self, key: &str, value: Map<String, Value>) -> Result<Value, Errors> {
        let ref mut data = self.data;
        let db: &mut Map<String, Value> = data.as_object_mut()
            .expect("Database is invalid. You can't mock API with it. Terminating...");
        match db.get_mut(key) {
            Some(en_map) => {
                let array: &mut Vec<Value> = en_map.as_array_mut()
                    .expect("Table is invalid. For now it can only be Array<Map>. Terminating...");
                match value.get("id") {
                    Some(id_value) => {
                        match id_value.as_i64() {
                            Some(id) => {
                                match Database::find_index(array, &id) {
                                    None => {
                                        Self::error(&self.logger,
                                                    Errors::NotFound(format!("Update - Error  \
                                                                              {:?}. No record \
                                                                              with the given \
                                                                              \"id\"",
                                                                             id)))
                                    }
                                    Some(idx) => {
                                        {
                                            let old_value = array.get_mut(idx).unwrap();
                                            let old_map = old_value.as_object_mut()
                                                .unwrap();
                                            for key in value.keys() {
                                                old_map.insert(key.to_string(),
                                                               value.get(key).unwrap().clone());
                                            }
                                        }
                                        let new_value = array.get(idx).unwrap();
                                        info!(&self.logger, "Updated - Ok id: {:?}", &id);
                                        debug!(&self.logger, "Updated - Value  {}", &new_value);
                                        return Ok(new_value.clone());
                                    }
                                }
                            }
                            None => {
                                Self::error(&self.logger,
                                            Errors::BadData(format!("Update - Error  {:?}. id \
                                                                     column is not valid. Must \
                                                                     be compatible with i64",
                                                                    &value)))
                            }
                        }
                    }
                    None => {
                        Self::error(&self.logger,
                                    Errors::BadData(format!("Update - Error  {:?}. id column is \
                                                             not valid. Must be compatible with \
                                                             i64",
                                                            &value)))
                    }

                }
            }
            None => Err(Errors::NotFound(format!("Table not found {}", &key))),

        }
    }

    /// Deletes the record with the given id.
    pub fn delete(&mut self, key: &str, id: &i64) -> Result<Value, Errors> {
        let ref mut data = self.data;
        let db: &mut Map<String, Value> = data.as_object_mut()
            .expect("Database is invalid. You can't mock API with it. Terminating...");
        match db.get_mut(key) {
            Some(en_map) => {
                let array: &mut Vec<Value> = en_map.as_array_mut()
                    .expect("Table is invalid. For now it can only be Array<Map>. Terminating...");
                match Database::find_index(array, &id) {
                    None => {
                        Self::error(&self.logger,
                                    Errors::NotFound(format!("Delete - Error  id: {:?}", &id)))
                    }
                    Some(idx) => {
                        let value = array.remove(idx);
                        info!(&self.logger, "Delete - Ok  id: {:?}", &id);
                        debug!(&self.logger, "Delete - Value {}", &value);
                        return Ok(value.clone());
                    }
                }
            }
            None => {
                Self::error(&self.logger,
                            Errors::NotFound(format!("Table not found {}", &key)))
            }
        }
    }

    /// Flush all the changes to the file.
    pub fn flush(&mut self) {
        let new_db = &serde_json::to_string(&self.data).unwrap();
        debug!(&self.logger, "Flush - Started");
        let bytes = new_db.as_bytes();
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.configuration.path)
            .unwrap();
        match file.set_len(0) {
            Ok(_) => {
                match file.write_all(bytes) {
                    Ok(_) => {
                        let result = file.sync_all();
                        info!(&self.logger,
                              "Flush - Ok File {:?} Result: {:?}",
                              &file,
                              &result);
                    }
                    Err(e) => {
                        error!(&self.logger,
                               "Flush - Error Can't write file File: {:?} Error: {:?}",
                               &file,
                               e)
                    }
                }
            }
            Err(e) => {
                error!(&self.logger,
                       "Flush - Error Can't set file size File: {:?} Error {:?}",
                       &file,
                       e)
            }
        }
    }

    /// Get the list of the tables
    pub fn tables(&self) -> Vec<&String> {
        let map: &serde_json::Map<String, Value> = self.data
            .as_object()
            .expect("Database is invalid. You can't mock API with it. Terminating...");
        let mut keys = vec![];
        keys.extend(map.keys());
        keys
    }

    /// Find the index of the element with the given target id.
    fn find_index(vec: &Vec<Value>, target: &i64) -> Option<usize> {
        let mut index = 0;
        for value in vec.iter() {
            let map = value.as_object().unwrap();
            let id = map.get("id")
                .unwrap()
                .as_i64()
                .unwrap();
            if id.eq(target) {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn error(logger: &Logger, error: Errors) -> Result<Value, Errors> {
        error!(logger, "{:?}", error);
        return Err(error);
    }
}