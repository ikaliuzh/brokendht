use std::collections::HashMap;
use std::sync::Mutex;

use crate::messages::{Key, Value};


#[derive(Debug)]
pub struct StorageMap {
	pub data: Mutex<HashMap<Key, Value>> 
}

impl StorageMap { 
	pub fn new() -> StorageMap {
		StorageMap { data: Mutex::new(HashMap::new()) }
	}

	pub fn insert(&self, key: Key, value: Value) {
		let mut data_guard = self.data.lock().unwrap();

		data_guard.insert(key, value); 
	}

	pub fn get(&self, key: Key) -> Option<Value> {
		let data_guard = self.data.lock().unwrap();

		match data_guard.get(&key) {
			Some(v) => Some(v.to_owned()),
			None => None
		}	
	}

	pub fn delete(&self, key: Key) {
		let mut data_guard = self.data.lock().unwrap();

		data_guard.remove(&key);	
	}
}