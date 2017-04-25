use database::Database;
use database::errors::Errors;
use std::vec::Vec;
use serde_json::Value;
use serde_json;
use rand;

impl Database {
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
        match Self::get_object(keys, data) {
            Ok(mut obj) => {
                // Path Found. It should be an array to accomplish an operation. Otherwise it must be an update not insert.
                match obj.as_array_mut() {
                    Some(ref mut array) => {
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
                        let value_with_id = &mut value.clone();
                        match value_with_id.as_object_mut() {
                            Some(obj_id) => {
                                obj_id.insert("id".to_string(), serde_json::to_value(id).unwrap());
                            }
                            None => {}
                        }

                        match Database::find_index(array, &id) {
                            None => {
                                array.push(value_with_id.clone());
                                info!(&self.logger, "Insert - Ok id: {:?}", &id);
                                debug!(&self.logger, "Insert - Value  {}", &value_with_id);
                                return Ok(value_with_id.clone());
                            }
                            Some(idx) => {
                                return Self::error(&self.logger,
                                                   Errors::Duplicate(format!("Insert - Error  \
                                                                              {:?}. \"id\" \
                                                                              duplicates \
                                                                              record at index: \
                                                                              {:?}",
                                                                             &value_with_id,
                                                                             idx)));
                            }
                        }

                    }
                    None => {
                        return Self::error(&self.logger,
                                           Errors::Duplicate(format!("Insert - Error \
                                                                      already has an object \
                                                                      with the given key: {:?}",
                                                                     keys)));
                    }
                }
            }
            Err(ref msg) => Err(msg.clone()),
        }
    }

    /// Updates the record with the given id.
    pub fn update(&mut self, keys: &mut Vec<String>, value: Value) -> Result<Value, Errors> {
        let mut data = &mut self.data;
        match Self::get_object(keys, data) {
            Ok(mut obj) => {
                match obj {
                    &mut Value::Object(ref mut map) => {
                        // let id = map.get("id").clone();
                        match value {
                            Value::Object(value_map) => {
                                for key in value_map.keys() {
                                    if key != "id" {
                                        map.insert(key.to_string(),
                                                   value_map.get(key).unwrap().clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                        info!(&self.logger, "Updated - Ok id: {:?}", &map.get("id"));
                        debug!(&self.logger, "Updated - Value  {:?}", &map);
                    }
                    &mut Value::String(ref mut s) => {
                        match value {
                            Value::String(value_s) => {
                                *s = value_s.clone();
                            }
                            _ => {}
                        }
                    }
                    &mut Value::Number(ref mut n) => {
                        match value {
                            Value::Number(value_n) => {
                                *n = value_n;
                            }
                            _ => {}
                        }
                    }
                    &mut Value::Bool(ref mut b) => {
                        match value {
                            Value::Bool(value_s) => {
                                *b = value_s.clone();
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        return Self::error(&self.logger,
                                           Errors::Duplicate(format!("Update - Error \
                                                                      already has an object \
                                                                      with the given key: {:?}",
                                                                     keys)));
                    }
                }
                return Ok(obj.clone());
            }
            Err(_) => {
                Self::error(&self.logger,
                            Errors::NotFound(format!("Update - Error  {:?}. No record with the \
                                                      given path:",
                                                     keys)))
            }
        }
    }

    /// Deletes the record with the given id.
    pub fn delete(&mut self, keys: &mut Vec<String>) -> Result<Value, Errors> {
        let mut data = &mut self.data;
        let last_idx = keys.len() - 1;
        let last_item = keys.remove(last_idx);
        let key = last_item.as_str();
        match Self::get_object(keys, data) {
            Ok(mut obj) => {
                match obj {
                    &mut Value::Object(ref mut map) => {
                        match map.remove(key) {
                            Some(deleted) => return Ok(deleted.clone()),
                            None => {
                                return Self::error(&self.logger,
                                                   Errors::NotFound(format!("Table not found {}",
                                                                            &key)))
                            }
                        }
                    }
                    &mut Value::Array(ref mut array) => {
                        let id = Self::decide_id(&key.to_string());
                        match Database::find_index(array, &id) {
                            None => {
                                Self::error(&self.logger,
                                            Errors::NotFound(format!("Delete - Error  id: {:?}",
                                                                     &id)))
                            }
                            Some(idx) => {
                                let value = array.remove(idx);
                                info!(&self.logger, "Delete - Ok  id: {:?}", &id);
                                debug!(&self.logger, "Delete - Value {}", &value);
                                return Ok(value.clone());
                            }
                        }
                    }
                    _ => {
                        Self::error(&self.logger,
                                    Errors::NotFound(format!("Delete - Error  path: {:?}", &key)))
                    }
                }
            }
            Err(_) => {
                Self::error(&self.logger,
                            Errors::NotFound(format!("Delete - Error  path: {:?}", &key)))
            }
        }
    }
}