use database::Database;
use database::Errors;
use std::vec::Vec;
use serde_json::Value;
use serde_json;
use service::query_api::Queries;
use database::query_api;

impl Database {
    /// Retuns the list of the tables (outmost keys) from the database.
    pub fn tables(&self) -> Vec<&String> {
        let map: &serde_json::Map<String, Value> = self.data
            .as_object()
            .expect("Database is invalid. You can't mock API with it. Terminating...");
        let mut keys = Vec::new();
        keys.extend(map.keys());
        keys
    }

    /// Reads the desired result with the given path.
    pub fn read(&mut self,
                keys: &mut Vec<String>,
                queries: Option<Queries>)
                -> Result<Value, Errors> {
        let mut data = &mut self.data;
        // TODO: If path is db return db
        match Self::get_object(keys, data) {
            Ok(obj) => {
                println!("{:?}", queries);
                if let Some(q) = queries {
                    let clone = &mut obj.clone();
                    query_api::filter::apply(clone, &q);
                    query_api::q::apply(clone, &q);
                    return Ok(clone.clone());
                }
                return Ok(obj.clone());

            }
            Err(ref msg) => Err(msg.clone()),
        }
    }
}