use::std::thread;
use std::io::prelude::*;
use std::collections::{hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, Shutdown};

use crate::messages::*;
use crate::storage::*;


#[derive(Debug)]
pub struct DhtServerAddr {
    pub socket: SocketAddr,
}

impl DhtServerAddr {
    pub fn new(ipaddr: IpAddr, port: u16) -> DhtServerAddr {
        DhtServerAddr { socket: SocketAddr::new(ipaddr, port) }
    }

    pub fn new_local(port: u16) -> DhtServerAddr {
        DhtServerAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let _ = &self.socket.hash(&mut hasher);
        hasher.finish()
    }
}

pub struct Server {
    socket_addr: DhtServerAddr,
    fingertable: Vec<DhtServerAddr>, // fingertable[i] -> 2^i successor
    storage: StorageMap,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { 
            socket_addr: DhtServerAddr::new_local(port),
            fingertable: vec![],
            storage: StorageMap::new(),
        }
    }

    pub fn run(&self) {
        let conn_listener = TcpListener::bind(self.socket_addr.socket)
            .expect(&format!("Failed to connect to socket {}", self.socket_addr.socket));
        println!("Running on {}", self.socket_addr.socket);

        for stream in conn_listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let handle = thread::spawn(move||{
                         self.handle_connection(stream)
                    });
                    handle.join().unwrap();
                },
                Err(e) => {
                    println!("Error: {}", e);
                },
            }
        }

        drop(conn_listener);
    }


    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        
        match stream.read(&mut buffer) {
            Ok(_) => {
                let request: DhtAction = 
                    bincode::deserialize::<DhtAction>(&buffer).unwrap();


                &self.handle_request(request);
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

