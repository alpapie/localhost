use std::process::exit;
use std::{
    collections::HashMap, net::ToSocketAddrs
};

use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};

use crate::config::Config;
use crate::error::LogError;

use super::ConnectionHandler; // Assurez-vous que Config est correctement importé depuis votre code.

pub fn server_start() {
    let config_path = "config.json";
    let config = Config::load_from_file(config_path);
    match config {
        Ok(config) => {
            let mut listeners: Option<Vec<TcpListener>> = create_listeners(config.ports);
            if listeners.is_none() || listeners.as_ref().unwrap().is_empty() {
                listeners = Some(vec![create_default_listener().unwrap()]);
            }
            let mut litenerss= match listeners {
                Some(list) => list,
                None => {
                    println!("error server: code 500");
                    exit(0);
                },
            };
            let mut server = Server::new(&mut litenerss);
            let mut events = Events::with_capacity(1024);
            
            loop {
                server.poll.poll(&mut events, None).unwrap();
    
                for event in events.iter() {
                    let token = event.token();
                    // println!("NEW REQUEST {token:?}");
                    if let Some(handler) = server.connection_handlers.get_mut(&token) {
                        // println!("connection existant {handler:?}");
                        handler.handle_event(&mut server.poll, event);
                    } else if token.0 < server.listeners.len() {
                        let listener = &mut server.listeners[token.0];
                        match listener.accept() {
                            Ok((stream, _addr)) => {
                                // println!("nouvel connection {_addr:?}");
                                server.handle_new_connection(stream);
                            }
                            Err(e) => {
                                LogError::new(format!("Error accepting connection: {:?}", e)).log();
                            },
                        }
                    }
                }
            }

        }
        Err(err) => {
            println!("{err}")
        }
    }
}


fn create_listeners(ports: Vec<u16>) -> Option<Vec<TcpListener>> {
    let mut listeners = Vec::new();
    for port in ports {
        let h_port = format!("127.0.0.1:{}", port);
        let adress = match h_port.to_socket_addrs() {
            Ok(mut addr) => match addr.next() {
                Some(socket_addr) => socket_addr,
                None => return {
                    LogError::new(format!("error lors de la conection")).log();
                    None
                },
            },
            Err(e) => return {
                LogError::new(format!("error lors de la conection {e}")).log();
                None
            },
        };

        match TcpListener::bind(adress) {
            Ok(listener) => listeners.push(listener),
            Err(e) => {
                LogError::new(format!("Error: {}. Unable to listen to: {}", e, h_port)).log();
                return None
            }
        }
    }
    Some(listeners)
}

fn create_default_listener() -> Result<TcpListener, String> {
    let default_address = "127.0.0.1:9999";
    let addr = match default_address.to_socket_addrs() {
        Ok(mut addr) => match addr.next() {
            Some(socket_addr) => socket_addr,
            None => return Err(format!("No valid address found for {}", default_address)),
        },
        Err(e) => return Err(format!("Error resolving address {}: {}", default_address, e)),
    };

    match TcpListener::bind(addr) {
        Ok(listener) => Ok(listener),
        Err(e) => Err(format!("Error binding to address {}: {}", default_address, e)),
    }
}

 #[derive(Debug)]
pub struct Server<'a> {
   pub listeners: &'a mut Vec< TcpListener>,
   pub poll: Poll,
   pub connection_handlers: HashMap<Token, ConnectionHandler>,
   pub next_token: usize,
}

impl <'a> Server <'a>  {
    pub fn new(listeners: &'a mut Vec< TcpListener>) -> Self {

        let poll = Poll::new().unwrap();
        
          for (index, listener) in listeners.iter_mut().enumerate() {
            let token = Token(index);
            poll.registry().register(listener, token, Interest::READABLE).unwrap();
        }

        Server {
            listeners,
            poll,
            connection_handlers: HashMap::new(),
            next_token:0

        }
    }

    pub fn handle_new_connection(&mut self, stream: TcpStream) {
        let token = Token(self.next_token + self.listeners.len());
        self.next_token += 1;
        let mut handler = ConnectionHandler::new(stream, token);
        self.poll.registry().register(&mut handler.stream, token, Interest::READABLE | Interest::WRITABLE).unwrap();
        self.connection_handlers.insert(token, handler);
    }
}

