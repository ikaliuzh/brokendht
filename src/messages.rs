use serde::{Serialize, Deserialize};
use bincode;

pub type Buffer = Vec<u8>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DhtAction {
	Insert { key: Key, val: Value },
	Lookup { key: Key },
	Delete { key: Key },

	Uninitialized,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Key{
	pub key: Buffer,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Value{
	None,
	Value { content: Buffer },
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dhtaction() {
		
	}

}