use database::Database;
use database::Errors;
use database::errors::log_n_wrap;
use database::Errors::{NotFound, Duplicate};
use std::vec::Vec;
use serde_json::Value;
use serde_json;
use rand;

impl Database {
    /// Retuns the list of the tables ind the db json
    pub fn tables(&self) -> Vec<&String> {
        let map: &serde_json::Map<String, Value> = self.data
            .as_object()
            .expect("Database is invalid. You can't mock API with it. Terminating...");
        let mut keys = Vec::new();
        keys.extend(map.keys());
        keys
    }

    /// Reads the desired result with the given path.
    pub fn read(&mut self, keys: &mut Vec<String>) -> Result<Value, Errors> {
        let mut data = &mut self.data;
        match Self::get_object(keys, data) {
            Ok(obj) => Ok(obj.clone()),
            Err(ref msg) => Err(msg.clone()),
        }
    }

    /// Inserts the record to the desired place.
    pub fn insert(&mut self, keys: &mut Vec<String>, value: Value) -> Result<Value, Errors> {
        let mut data = &mut self.data;
        if let Ok(mut obj) = Self::get_object(keys, data) {
            // Path Found. It should be an array to accomplish an operation. Otherwise it must be an update not insert.
            if let Some(ref mut array) = obj.as_array_mut() {
                let mut id = rand::random();
                // If id comes with the record use it.
                if let Some(id_value) = value.get("id") {
                    if let Some(parsed) = id_value.as_i64() {
                        id = parsed;
                    }
                }
                let value_with_id = &mut value.clone();

                if let Some(obj_id) = value_with_id.as_object_mut() {
                    obj_id.insert("id".to_string(), serde_json::to_value(id).unwrap());
                }

                if let Some(idx) = Database::find_index(array, &id) {
                    log_n_wrap(&self.logger,
                               Duplicate(format!("Insert - Error  {:?}. \"id\" duplicates \
                                                  record at index: {:?}",
                                                 &value_with_id,
                                                 idx)))
                } else {
                    array.push(value_with_id.clone());
                    info!(&self.logger, "Insert - Ok id: {:?}", &id);
                    debug!(&self.logger, "Insert - Value  {}", &value_with_id);
                    Ok(value_with_id.clone())
                }
            } else {
                log_n_wrap(&self.logger,
                           Duplicate(format!("Insert - Error already has an object with the \
                                              given key: {:?}",
                                             keys)))
            }
        } else {
            log_n_wrap(&self.logger,
                       NotFound(format!("Insert - Error  {:?}. No record with the given path:",
                                        keys)))
        }
    }

    /// Updates the record with the given id.
    pub fn update(&mut self, keys: &mut Vec<String>, value: Value) -> Result<Value, Errors> {
        let mut data = &mut self.data;
        if let Ok(mut obj) = Self::get_object(keys, data) {
            match obj {
                &mut Value::Object(ref mut map) => {
                    // let id = map.get("id").clone();
                    if let Value::Object(value_map) = value {
                        for key in value_map.keys() {
                            if key != "id" {
                                map.insert(key.to_string(), value_map.get(key).unwrap().clone());
                            }
                        }
                    }
                    info!(&self.logger, "Updated - Ok id: {:?}", &map.get("id"));
                    debug!(&self.logger, "Updated - Value  {:?}", &map);
                }
                &mut Value::String(ref mut s) => {
                    if let Value::String(value_s) = value {
                        *s = value_s.clone();
                    }
                }
                &mut Value::Number(ref mut n) => {
                    if let Value::Number(value_n) = value {
                        *n = value_n;
                    }
                }
                &mut Value::Bool(ref mut b) => {
                    if let Value::Bool(value_s) = value {
                        *b = value_s.clone();
                    }
                }
                _ => {
                    return log_n_wrap(&self.logger,
                                      Duplicate(format!("Update - Error already has an object \
                                                         with the given key: {:?}",
                                                        keys)));
                }
            }
            return Ok(obj.clone());
        } else {
            log_n_wrap(&self.logger,
                       NotFound(format!("Update - Error  {:?}. No record with the given path:",
                                        keys)))
        }
    }

    /// Deletes the record with the given id.
    pub fn delete(&mut self, keys: &mut Vec<String>) -> Result<Value, Errors> {
        let mut data = &mut self.data;
        let last_idx = keys.len() - 1;
        let last_item = keys.remove(last_idx);
        let key = last_item.as_str();
        if let Ok(mut obj) = Self::get_object(keys, data) {
            match obj {
                &mut Value::Object(ref mut map) => {
                    if let Some(deleted) = map.remove(key) {
                        Ok(deleted.clone())
                    } else {
                        log_n_wrap(&self.logger, NotFound(format!("Table not found {}", &key)))
                    }
                }
                &mut Value::Array(ref mut array) => {
                    let id = Self::decide_id(&key.to_string());
                    if let Some(idx) = Database::find_index(array, &id) {
                        let value = array.remove(idx);
                        info!(&self.logger, "Delete - Ok  id: {:?}", &id);
                        debug!(&self.logger, "Delete - Value {}", &value);
                        Ok(value.clone())
                    } else {
                        log_n_wrap(&self.logger,
                                   NotFound(format!("Delete - Error  id: {:?}", &id)))
                    }
                }
                _ => {
                    log_n_wrap(&self.logger,
                               NotFound(format!("Delete - Error  path: {:?}", &key)))
                }
            }
        } else {
            log_n_wrap(&self.logger,
                       NotFound(format!("Delete - Error  path: {:?}", &key)))
        }
    }
}