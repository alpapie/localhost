use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::time::Instant;
use mio::{Poll, Token};
use mio::net::TcpStream;
use std::io::{ Result as IoResult};

use crate::error::LogError;

#[derive(Debug)]
pub struct ConnectionHandler {
    pub stream: TcpStream,
    pub token: Token,
    pub last_activity: Instant,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream, token: Token) -> Self {
        ConnectionHandler { stream, token,last_activity: Instant::now() }
    }

    pub fn handle_event(&mut self, event: &mio::event::Event) -> bool{
        if event.is_readable() {
            match self.read_event(){
                Ok((head,body)) => {
                    // read requet
                    self.write_event();
                },
                Err(err) => {
                    println!("Error read request {:?}", err);
                    LogError::new(format!("Error read request {:?}", err)).log();
                },
            } 
        }
        // if event.is_writable() {
        //     self.write_event();
        // }
       return true
    }

    pub fn read_event(&mut self) -> Result<(String, Vec<u8>),Error>{
        let mut buffer = [0; 1024];
        let mut head = String::new();
        let mut body = Vec::new();
        let mut is_body = false;
        let mut buf_reader = BufReader::new(&mut self.stream);

        loop {
            match buf_reader.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        if is_body {
                            break;
                        } else {
                            return Err(Error::new(ErrorKind::Interrupted,"parse data error"));
                        }
                    }
                    let chunk = match String::from_utf8(buffer[..bytes_read].to_vec()) {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            String::from_utf8_lossy(&buffer[..bytes_read]).into()
                        },
                    };

                    if let Some(index) = chunk.find("\r\n\r\n") {
                        if !is_body {
                            head.push_str(&chunk[..index]);
                            body.extend_from_slice(&buffer[index + 4..bytes_read]);
                            is_body = true;
                        } else {
                            body.extend_from_slice(&buffer[..bytes_read]);
                        }
                        break;
                    } else {
                        if is_body {
                            body.extend_from_slice(&buffer[..bytes_read]);
                        } else {
                            head.push_str(&chunk);
                        }
                    }
                },
                Err(err) => return  Err(err),
            }
        }
        while let Ok(bytes_read) = buf_reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            body.extend_from_slice(&buffer[..bytes_read]);
        }
       Ok((head,body))
    }

    pub fn write_event(&mut self) {
        let response = b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nalpapierer";
        match self.stream.write_all(response) {
            Ok(_) => println!("Response sent successfully"),
            Err(err) => {
                LogError::new(format!("Error writing response: {:?}", err)).log();
            }
        }
    }
}
