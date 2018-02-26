use database::Database;
use database::Errors;
use database::errors::log_n_wrap;
use database::Errors::{Conflict, NotFound};
use std::vec::Vec;
use serde_json::Value;

impl Database {
    /// Updates the record with the given path.
    pub fn update(&mut self, keys: &mut Vec<String>, value: Value) -> Result<Value, Errors> {
        let data = &mut self.data;
        if let Ok(obj) = Self::get_object(keys, data) {
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
                &mut Value::String(ref mut s) => if let Value::String(value_s) = value {
                    *s = value_s.clone();
                },
                &mut Value::Number(ref mut n) => if let Value::Number(value_n) = value {
                    *n = value_n;
                },
                &mut Value::Bool(ref mut b) => if let Value::Bool(value_s) = value {
                    *b = value_s.clone();
                },
                _ => {
                    //TODO:: Check error message for the case
                    return log_n_wrap(
                        &self.logger,
                        Conflict(format!(
                            "Update - Error already has an object \
                             with the given key: {:?}",
                            keys
                        )),
                    );
                }
            }
            return Ok(obj.clone());
        } else {
            log_n_wrap(
                &self.logger,
                NotFound(format!(
                    "Update - Error  {:?}. No record with the given path:",
                    keys
                )),
            )
        }
    }
}
