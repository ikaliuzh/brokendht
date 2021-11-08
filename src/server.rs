use std::net::{TcpListener, TcpStream, Shutdown};
use::std::thread;
use std::io::prelude::*;
use std::collections::HashMap;
use std::str;

use super::messages::*;

pub struct Server<'a> {
    socket_addr: &'a str,
    data: HashMap<String, String>,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { 
            socket_addr,
            data: HashMap<Key, Value>::new(),
        }
    }

    pub fn run(&self) {
        let conn_listener = TcpListener::bind(self.socket_addr)
            .expect(&format!("Failed to connect to socket {}", self.socket_addr));
        println!("Running on {}", self.socket_addr);

        for stream in conn_listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move||{
                        handle_connection(stream)
                    });
                },
                Err(e) => {
                    println!("Error: {}", e);
                },
            }
        }

        drop(conn_listener);
    }


    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        
        match stream.read(&mut buffer) {
            Ok(_) => {
                let request: DhtAction = match
                    bincode::deserialize::<DhtAction>(&buffer) {
                        Ok(m) => m,
                        Err(_) => {
                            println!("Failed to deserialize the request");
                        }
                    };


                handle_request(request);
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
            DhtAction::Insert{ key, val } => {
                self.insert(key, val)
            },
            DhtAction::Lookup{ key } => {
                self.lookup(key)
            },
            DhtAction::Delete { key } => {
                self.delete(key)
            }
        }
    }

}

