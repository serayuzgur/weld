use database::Database;
use database::Errors;
use database::errors::log_n_wrap;
use database::Errors::NotFound;
use std::vec::Vec;
use serde_json::Value;

impl Database {
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