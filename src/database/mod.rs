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
	pub fn set_configuration(&mut self,configuration: &configuration::Database) {
		let path: String = configuration.path.clone();
		self.logger =  ROOT_LOGGER.new(o!("database.path"=>path));
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
		let mut file =
			File::open(&self.configuration.path).expect("Database - Error Can't read. Terminating...");
		let mut contents = String::new();
		match file.read_to_string(&mut contents) {
			Ok(usize) => {
				if usize == 0 {
					panic!("Database - Error It is empty. You can't mock API with it. Terminating...");
				}
			}
			Err(e) => {
				error!(self.logger, "Database - Error {}", e);
				panic!("Database - Error You can't mock API with it. Terminating...");
			}
		}
		let new_data: serde_json::Value = serde_json::from_str(&contents).expect("Invalid JSON format. Check provided db. Terminating...");
		debug!(self.logger, "{}", &new_data);
		self.data = new_data;
		info!(self.logger,
			  "Database - Ok : {:?}",
			  self.configuration.path);
	}

	/// Reads the desired result with the given id.
	pub fn read(&mut self, key: &str, id: &i64) -> Option<Value> {
		let ref mut data = self.data;
		let db_option = data.as_object_mut();
		let db: &mut Map<String, Value> = db_option.expect("Database is invalid. You can't mock API with it. Terminating...");
		match db.get_mut(key){
			Some(en_map) => {
				let array: &mut Vec<Value> = en_map.as_array_mut().expect("Table is invalid. For now it can only be Array<Map>. Terminating...");
				match Database::find_index(array, &id) {
					None => {
						error!(&self.logger, "Read - Error  id: {:?}", &id);
						return None;
					}
					Some(idx) => {
						match array.get(idx) {
							Some(value) => {
								info!(&self.logger, "Read - Ok  id: {:?}", &id);
								debug!(&self.logger, "Read - Value {}", &value);
								return Some(value.clone());
							}
							None => {
								error!(&self.logger, "Read - Error  id: {:?}", &id);
								return None;
							}
						}
					}
				}
			}
			None => {
				error!(self.logger,"Table not found {}",&key);
				return None;
			}
		}	
	}

	/// Inserts the record to the desired place.
	pub fn insert(&mut self, key: &str, value: Map<String, Value>) -> Option<Value> {
		let ref mut data = self.data;
		let db_option = data.as_object_mut();
		let db: &mut Map<String, Value> = db_option.expect("Database is invalid. You can't mock API with it. Terminating...");
		match db.get_mut(key){
			Some(en_map) => {        
				let array: &mut Vec<Value> = en_map.as_array_mut().expect("Table is invalid. For now it can only be Array<Map>. Terminating...");
				let mut id = rand::random();
				//If id comes with the record use it.
				match value.get("id"){
					Some(value)=>{
						match value.as_i64(){
							Some(parsed)=> {id = parsed;}
							None=>{}
						}
					}
					None=>{}
				}
				match Database::find_index(array, &id) {
					None => {
						let as_value = serde_json::to_value(&value).unwrap();
						array.push(as_value.clone());
						debug!(&self.logger, "Inserted  {:?}", &value);
						return Some(as_value);
					}
					Some(idx) => {
						error!(&self.logger,
							"Failed Insert  {:?}. \"id\" duplicates record at index: {:?}",
							&value,
							idx);
							return None;
					}
				}
		   }
			None => {
				error!(self.logger,"Table not found {}",&key);
				return None;
			}
		}
	}

	/// Updates the record with the given id.
	pub fn update(&mut self, key: &str, value: Map<String, Value>) {
		let ref mut data = self.data;
		let db_option = data.as_object_mut();
		let db: &mut Map<String, Value> = db_option.expect("Database is invalid. You can't mock API with it. Terminating...");
		match db.get_mut(key){
			Some(en_map) => {                  
				let array: &mut Vec<Value> = en_map.as_array_mut().expect("Table is invalid. For now it can only be Array<Map>. Terminating...");
				match value.get("id"){
					Some(id_value)=>{
						match id_value.as_i64(){
							Some(id)=> {
								match Database::find_index(array, &id) {
									None => {
										error!(&self.logger,
											"Failed update  {:?}. No record with the given \"id\"",
											&value);
									}
									Some(idx) => {
										let old_value = array.get_mut(idx)
											.unwrap()
											.as_object_mut()
											.unwrap();
										for key in value.keys() {
											old_value.insert(key.to_string(), value.get(key).unwrap().clone());
										}
										info!(&self.logger, "Updated - Ok id: {:?}", &id);
										debug!(&self.logger, "Updated - Value  {:?}", &value);
									}
								}
							}
							None=>{
								error!(&self.logger,
									"Update - Error  {:?}. id column is not valid. Must be compatible with i64",
									&value);
							}
						}
					}
					None=>{
					  error!(&self.logger,
						"Update - Error  {:?}. id column is not available. Must have an i64 compatible id",
						&value);  
					}
				}
			}
			None=>{}
		}
	}

	/// Deletes the record with the given id.
	pub fn delete(&mut self, key: &str, id: &i64)-> Option<Value> {
		let ref mut data = self.data;
		let db_option = data.as_object_mut();
		let db: &mut Map<String, Value> = db_option.expect("Database is invalid. You can't mock API with it. Terminating...");
		match db.get_mut(key){
			Some(en_map) => {
				let array: &mut Vec<Value> = en_map.as_array_mut().expect("Table is invalid. For now it can only be Array<Map>. Terminating...");
				match Database::find_index(array, &id) {
					None => {
						error!(&self.logger, "Delete - Error  id: {:?}", &id);
						return None;
					}
					Some(idx) => {
						let value= array.remove(idx);
						info!(&self.logger, "Delete - Ok  id: {:?}", &id);
						debug!(&self.logger, "Delete - Value {}", &value);
						return Some(value.clone());	
					}
				}
			}
			None => {
				error!(self.logger,"Table not found {}",&key);
				return None;
			}
		}
	}

	pub fn flush(&mut self) {
		let new_db = &serde_json::to_string(&self.data).unwrap();
		debug!(&self.logger, "Flush -  Starded Data: {}", &new_db);
		let bytes = new_db.as_bytes();
		let mut file = OpenOptions::new()
			.read(true)
			.write(true)
			.open(&self.configuration.path)
			.unwrap();
		match file.set_len(0){
			Ok(_)=>{
				match file.write_all(bytes){
					Ok(_)=>{
						let result = file.sync_all();
						info!(&self.logger, "Flush - Ok  File {:?} Result: {:?}", &file, &result);
					}
					Err(e)=>{
						error!(&self.logger, "Flush - Error Can't write file File: {:?} Error: {:?}", &file,e)
					}
				}
			}
			Err(e)=>{
				error!(&self.logger, "Flush - Error Can't set file size File: {:?} Error {:?}", &file, e)
			}
		}
	}

	/// Get the list of the tables
	pub fn tables(&self) -> Vec<&String> {
		let map: &serde_json::Map<String, Value> = self.data.as_object().expect("Database is invalid. You can't mock API with it. Terminating...");
		let mut keys = vec![];
		keys.extend(map.keys());
		keys
	}

	/// Find the index of the element with the given target id.
	fn find_index(vec: &mut Vec<Value>, target: &i64) -> Option<usize> {
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

