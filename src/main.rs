use crate::messages::*;
use crate::server::*;


use std::net::{IpAddr, Ipv4Addr};

pub mod messages;
pub mod server;
pub mod storage;


fn main() {
    let act : DhtAction = DhtAction::Insert {
        key: Key {key: b"banana".to_vec() },
        value: Value::Value{ content: b"yellow".to_vec() },
    };
    println!("{:?}\n", bincode::deserialize::<DhtAction>(&bincode::serialize(&act).unwrap()));


    let socket = DhtServerAddr::new( 
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080
        );

    println!("{:?}", socket.hash());
}