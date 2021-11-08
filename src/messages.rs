pub type Buffer = Vecc<u8>;

#[derive(Debug, PartialEq)]
pub enum DhtAction {
	Insert {key: Key, val: Value},
	Lookup {key: Key },
	Delete {key: Key },

	Uninitialized,
}

// impl From<String> for DhtAction {
// 	fn from(data: String) -> DhtAction {
// 		let mut data = data.split_whitespace();

// 		let act = data.next().unwrap();
// 		let key = data.next().unwrap().parse::<i32>().unwrap();
		
// 		match act {
// 			"0" | "insert" => { // insert
// 				DhtAction::Insert{ key, val: data.next().unwrap().parse::<i32>().unwrap() }
// 			}, 
// 			"1" | "lookup" => { // lookup
// 				DhtAction::Lookup{ key }
// 			},
// 			"2" | "delete" => { // delete
// 				DhtAction::Delete{ key } 
// 			}
// 			_ => {
// 				DhtAction::Uninitialized
// 			}
// 		}
// 	}
// }


#[derive(Debug, PartialEq)]
pub struct Key{

}

#[derive(Debug, PartialEq)]
pub enum Value{

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dhtaction_into() {
		let res: DhtAction = String::from("insert 25 16").into();

		assert_eq!(DhtAction::Insert{key: 25, val: 16}, res);
	}

}