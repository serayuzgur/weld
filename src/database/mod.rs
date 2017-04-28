extern crate serde_json;
mod operations;
pub mod errors;

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
use self::errors::Errors;


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
            Err(e) => {
                panic!("Database - Error You can't mock API with it. Terminating...{}",
                       e)
            }

        }
        let new_data: serde_json::Value = serde_json::from_str(&contents)
            .expect("Invalid JSON format. Check provided db. Terminating...");
        self.data = new_data;
        info!(self.logger, "Database - Ok : {:?}", self.configuration.path);
    }

    pub fn decide_id(val: &String) -> i64 {
        match i64::from_str_radix(val.as_str(), 10) {
            Ok(parsed) => parsed,
            Err(_) => -1,
        }
    }

    pub fn get_object<'per_req>(keys: &mut Vec<String>,
                                json_object: &'per_req mut Value)
                                -> Result<&'per_req mut Value, Errors> {
        if keys.len() == 0 {
            return Ok(json_object);
        }
        let key = keys.remove(0);
        match json_object {
            &mut Array(ref mut array) => {
                let id = Self::decide_id(&key);
                if let Some(idx) = Database::find_index(&array, &id) {
                    if let Some(obj) = array.get_mut(idx) {
                        return Self::call_if_nec(keys, key, obj);
                    } else {
                        return Err(Errors::NotFound(format!("Read - Error  path: {:?} ", &key)));
                    }
                } else {
                    return Err(Errors::NotFound(format!("Read - Error  path: {:?} ", &key)));
                }
            }

            &mut Object(ref mut obj) => {
                if let Some(obj) = obj.get_mut(key.as_str()) {
                    return Self::call_if_nec(keys, key, obj);
                } else {
                    return Err(Errors::NotFound(format!("Read - Error  path: {:?} ", &key)));
                }
            }
            _ => {
                return Err(Errors::NotFound(format!("Read - Error  path: {:?} ", &key)));
            }
        };
    }
    pub fn call_if_nec<'per_req>(keys: &mut Vec<String>,
                                 key: String,
                                 value: &'per_req mut Value)
                                 -> Result<&'per_req mut Value, Errors> {
        if keys.len() == 0 {
            keys.push(key);
            return Ok(value);
        } else {
            return Self::get_object(keys, value);
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
}