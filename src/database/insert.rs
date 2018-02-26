use database::Database;
use database::Errors;
use database::errors::log_n_wrap;
use database::Errors::{NotFound, Conflict};
use std::vec::Vec;
use serde_json::Value;
use serde_json;
use rand;

impl Database {
    /// Inserts the record to the given path.
    pub fn insert(&mut self, keys: &mut Vec<String>, value: Value) -> Result<Value, Errors> {
        let data = &mut self.data;
        if let Ok(obj) = Self::get_object(keys, data) {
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
                // TODO: random id conflict must be resolved.
                if let Some(idx) = Database::find_index(array, &id) {
                    log_n_wrap(&self.logger,
                               Conflict(format!("Insert - Error  {:?}. \"id\" duplicates \
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
                           Conflict(format!("Insert - Error already has an object with the \
                                              given key: {:?}",
                                             keys)))
            }
        } else {
            log_n_wrap(&self.logger,
                       NotFound(format!("Insert - Error  {:?}. No record with the given path:",
                                        keys)))
        }
    }
}