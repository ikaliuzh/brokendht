use serde::{Serialize, Deserialize};

// use std::hash::{Hash, Hasher};

pub type Buffer = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DhtAction {
	Insert { key: Key, value: Value },
	Get { key: Key },
	Delete { key: Key },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DhtResponse {
	Value {
		key: Key,
		value: Value
	},
	InsertAck {
		key: Key
	},
	Error {
		key: Key,
		error: DhtError
	},
	Uninitialized,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DhtError {
	Error { message: String},
}


#[derive(Hash, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Key {
	pub key: Buffer,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value{
	None,
	Value { content: Buffer },
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn key_hash_consistency() {
		let key =  Key { key: vec![1, 2, 3] };
		assert_eq!(8086395815454877121, key.hash()); 
	}
}