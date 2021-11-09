// use::std::thread;
use std::io::prelude::*;
use std::collections::{hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, Shutdown};

use crate::messages::*;
use crate::storage::*;


pub trait RingHash: Hash {
    fn ring_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let _ = &self.hash(&mut hasher);
        hasher.finish()
    }
}


impl RingHash for SocketAddr {} 
impl RingHash for Key {}

pub struct Server {
    socket_addr: SocketAddr,
    fingertable: Vec<SocketAddr>, // fingertable[i] -> 2^i successor
    storage: StorageMap,
}

impl Server {
    pub fn new(port: u16) -> Self {
        println!("Runninf on localhost:{}", port);
        Server { 
            socket_addr: SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port),
            fingertable: vec![],
            storage: StorageMap::new(),
        }
    }

    pub fn run(&mut self) {
        let conn_listener = TcpListener::bind(self.socket_addr)
            .expect(&format!("Failed to connect to socket {}", self.socket_addr));
        println!("Running on {}", self.socket_addr);

        for stream in conn_listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    //let handle = thread::spawn(move||{
                    self.handle_connection(stream);
                    //});
                    // handle.join().unwrap();
                },
                Err(e) => {
                    println!("Error: {}", e);
                },
            }
        }

        drop(conn_listener);
    }


    fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        
        match stream.read(&mut buffer) {
            Ok(_) => {
                let request: DhtAction = 
                    bincode::deserialize::<DhtAction>(&buffer).unwrap();

                let _ = &self.handle_request(request);
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }

    fn handle_request(&mut self, request: DhtAction) -> DhtResponse {
        match request {
            DhtAction::Insert{ key, value } => {
                self.insert(key, value)
            },
            DhtAction::Get{ key } => {
                self.get(key)
            },
            DhtAction::Delete { key } => {
                self.delete(key)
            }
        }
    }

    fn insert(&mut self, key: Key, value: Value) -> DhtResponse {
        DhtResponse::Uninitialized
    }

    fn get(&mut self, key: Key) -> DhtResponse {
        DhtResponse::Uninitialized
    }

    fn delete(&mut self, key: Key) -> DhtResponse {
        DhtResponse::Uninitialized
    }


}

