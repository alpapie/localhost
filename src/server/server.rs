use std::process::exit;
use std::time::{Duration, Instant};
use std::{
    collections::HashMap, net::ToSocketAddrs
};

use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener, TcpStream};

use crate::config::config::HostConfig;
use crate::config::Config;
use crate::error::LogError;

use super::ConnectionHandler; // Assurez-vous que Config est correctement importé depuis votre code.

pub fn server_start() {
    let config_path = "config.json";
    let config = Config::load_from_file(config_path);
    match config {
        Ok(config) => {
            let mut listeners = create_listeners(&config);
            // if listeners.is_none() || listeners.as_ref().unwrap().is_empty() {
                
            // }
            let mut litenerss= match listeners {
                Some(list) => list,
                None => {
                    println!("error server: code 500");
                    LogError::new("error: no listener created ".to_string()).log();
                    exit(1);
                },
            };
            if litenerss.is_empty() {
                println!("error server: code 500 -> no server");
                LogError::new("error: no listener created ".to_string()).log();
                exit(1);
            }
            let mut server = Server::new(&mut litenerss);
            let mut events = Events::with_capacity(4096);
            
            loop {
                match server.poll.poll(&mut events, Some(Duration::from_millis(5000))) {
                    Ok(_) => {
                        for event in events.iter() {
                            let token = event.token();
                            if token.0 < server.listeners.len() {
                                let listener_info = &server.listeners[token.0];
                                match listener_info.listener.accept() {
                                    Ok((stream, _)) => {
                                       server.handle_new_connection(stream,listener_info.config);
                                    }
                                    Err(e) => {
                                        println!("error: new connection error - {e}");
                                        LogError::new(format!("Error accepting connection: {:?}", e)).log();
                                    },
                                }
                            }
                            if let Some(handler) = server.connection_handlers.get_mut(&token) {
                                if !handler.handle_event(event,&mut server.sessions){
                                   server.poll.registry()
                                    .deregister(&mut handler.stream)
                                    .expect("Failed to deregister stream");
                                    server.connection_handlers.remove(&token);
                                }
                            } 
                        }
                        server.handle_timeout()
                    },
                    Err(e) => {
                        println!("error: sever 500 - {e}");
                        LogError::new(format!("error: {e}")).log();
                    },
                };
            }
        }
        Err(err) => {
            println!("error: {err}");
            LogError::new(format!("error: {err}")).log();
        }
    }
}

#[derive(Debug)]
struct ListenerInfo <'a> {
    listener: TcpListener,
    config: &'a Config,
}

fn create_listeners(config: &HostConfig) -> Option<Vec<ListenerInfo>> {
    let mut listeners  = Vec::new();
    for host in &config.servers {
        let mut list_try_server=Vec::new();
        for port in &host.ports {
            let _hoster = format!("{}:{}", host.server_address,port);
            let adress = match _hoster.to_socket_addrs() {
                Ok(mut addr) => match addr.next() {
                    Some(socket_addr) => socket_addr,
                    None =>  {
                        println!("error lors de la conection");
                        LogError::new("error lors de la conection".to_string()).log();
                        list_try_server=Vec::new();
                        break;
                    },
                },
                Err(e) =>  {
                    println!("{}", format!("error lors de la conection {e}"));
                    LogError::new(format!("error lors de la conection {e}")).log();
                    list_try_server=Vec::new();
                    break;
                },
            };

            match TcpListener::bind(adress) {
                Ok(listener) => {
                    list_try_server.push(ListenerInfo{
                        listener,
                        config:host
                    })
                },
                Err(e) => {
                    println!("{}",format!("Error: {}. Unable to listen to: {}", e, adress));
                    LogError::new(format!("Error: {}. Unable to listen to: {}", e, adress)).log();
                    list_try_server=Vec::new();
                    break;

                }
            }
        }
        listeners.extend(list_try_server)
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
   listeners: &'a mut Vec< ListenerInfo<'a>>,
   pub poll: Poll,
   pub connection_handlers: HashMap<Token, ConnectionHandler<'a>>,
   pub next_token: usize,
   pub sessions:Vec<String>
}

impl <'a> Server <'a>  {
    fn new(listeners: &'a mut Vec< ListenerInfo<'a>>) -> Self {

        let poll = Poll::new().expect("Failed to create Poll instance");
        let mut token_id=0;
        
          for (index, listener) in listeners.iter_mut().enumerate() {
            token_id+=1;
            let token = Token(index);
            if poll.registry().register(&mut listener.listener, token, Interest::READABLE).is_err(){
                LogError::new("Error: 500 register connection listener error".to_string()).log();
            };
        }

        Server {
            listeners,
            poll,
            connection_handlers: HashMap::new(),
            next_token:token_id,
            sessions: Vec::new()
        }
    }

    pub fn handle_new_connection(&mut self, stream: TcpStream, config: &'a Config) {
        // set_linger_option(&stream, linger_duration).expect("Failed to set linger option");

        if let Err(e) = stream.set_ttl(60) {
            println!("error: timeout - {e}");
            LogError::new(format!("Error: {e}")).log();
        }
        self.next_token += 1;
        let token = Token(self.next_token);
        let mut handler = ConnectionHandler::new(stream, token,config);
        self.poll.registry().register(&mut handler.stream, token, Interest::READABLE | Interest::WRITABLE).unwrap();
        self.connection_handlers.insert(token, handler);
    }

    fn handle_timeout(&mut self) {
        let now = Instant::now();
        let timeout_duration = Duration::from_millis(3000);

        // Remove connections that timed out from `connections` HashMap
        self.connection_handlers.retain(|_, conn| {
            if now.duration_since(conn.last_activity) > timeout_duration {
                self.poll
                    .registry()
                    .deregister(&mut conn.stream)
                    .expect("Failed to deregister stream due to timeout");
                false
            } else {
                true
            }
        });
    }
}


// use socket2::{SockRef, Socket};

// fn set_linger_option(stream: &TcpStream, linger_duration: Option<Duration>) -> std::io::Result<()> {
//     #[cfg(unix)]
//     let socket = unsafe { Socket::from_raw_fd(stream.as_raw_fd()) };
//     #[cfg(windows)]
//     let socket = unsafe { Socket::from_raw_socket(stream.as_raw_socket()) };

//     SockRef::from(&socket).set_linger(linger_duration)?;
//     std::mem::forget(socket); // Important to avoid closing the file descriptor
//     Ok(())
// }