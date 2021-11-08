use messages::*;

pub mod messages;
// pub mod server;

fn main() {
    let act : DhtAction = DhtAction::Insert {
        key: Key {key: b"banana".to_vec() },
        val: Value::Value{ content: b"yellow".to_vec() },
    };
    println!("{:?}", bincode::deserialize::<DhtAction>(&bincode::serialize(&act).unwrap()));
}