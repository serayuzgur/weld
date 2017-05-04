use database::Database;
use database::Errors;
use std::vec::Vec;
use serde_json::Value;
use serde_json;
use service::query_api::Queries;

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
        println!("{:?}", queries);
        // TODO: If path is db return db
        match Self::get_object(keys, data) {
            Ok(obj) => Ok(obj.clone()),
            Err(ref msg) => Err(msg.clone()),
        }
        // TODO:
        // Get the result
        // If it is List than do the ops
        // filter & operations & full text
        // Sort
        // Paginate
        // Slice
    }
}