use mio::net::TcpStream;
use mio::Token;
use core::time;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

use crate::config::config::RouteConfig;
use crate::config::Config;
use crate::error::LogError;
use crate::request::parse_header::HttpRequest;
use crate::response::response::Response;
use uuid::Uuid;
use crate::request::parse_header::ParseError::*;

#[derive(Debug)]
pub struct ConnectionHandler<'a> {
    pub stream: TcpStream,
    token: Token,
    pub last_activity: Instant,
    pub config: &'a Config,
}

impl<'a> ConnectionHandler<'a> {
    pub fn new(stream: TcpStream, token: Token, config: &'a Config) -> Self {
        ConnectionHandler {
            stream,
            token,
            last_activity: Instant::now(),
            config,
        }
    }

    pub fn handle_event(&mut self, event: &mio::event::Event,session: &mut Vec<String>) -> bool {
        if event.is_readable() {
            match self.read_event() {
                Ok((head, body)) => {
                    if head.is_empty() {
                        return false;
                    }
                    let b_request = HttpRequest::parse(&head,&body,self.config);
                    match b_request {
                        Ok(request) => {
                            let mut max_redirect: u32 = 10;
                        // println!("head {:?} ->len({}) body {:?} ->({})",&request,head.len(),body,body.len());
                        if let Some(value) = self.get_response(request, &mut max_redirect,session) {
                            if max_redirect < 1 {
                                self.eror_ppage(310);
                            }
                            return value;
                        }
                        },
                        Err(request_error) => {
                            println!("Error parse request-> {:?}", request_error);
                            LogError::new(format!("Error parse request-> {:?}", request_error)).log();
                            return  match request_error  {
                               Tolong=>self.eror_ppage(413),
                                _=> self.eror_ppage(500)
                            };
                        }
                    }
                }
                Err(err) => {
                    println!("Error read request-> {:?}", err);
                    LogError::new(format!("Error read request-> {:?}", err)).log();
                    // return false;
                }
            }
        }
        true
    }

    fn eror_ppage(&mut self, status: u16) -> bool {
        let mut response = Response::new("".to_string());
        let res = response.response_error(status, self.config);
        self.write_event(&res);
        true
    }

    fn get_response(&mut self, mut request: HttpRequest, max_redirect: &mut u32,session: &mut Vec<String>) -> Option<bool> {
        let route = self.get_path(&request.path);
      
        if *max_redirect < 1 {
            return Some(true);
        }
        *max_redirect -= 1;
        if route.0 {
            if route.1.redirections.is_some() {
                request.path = route.1.redirections.unwrap();
                self.get_response(request, max_redirect,session);
                return Some(true);
            }
            if self.check(&request) {
                let path = match request.path.strip_prefix(&self.config.alias) {
                    Some(content) => content.to_owned(),
                    None => "".to_owned(),
                };
             
                let cookie=request.get_cookie("session_id");
                if route.1.auth.is_some() && !route.1.auth.unwrap() && (cookie.is_none() || !session.contains(&cookie.clone().unwrap()) ) {
                    println!("cookie {:?}",cookie.clone());
                    return Some(self.eror_ppage(403));
                }

                let mut sess_id=String::new();
                if route.1.setcookie.is_some() && route.1.setcookie.unwrap(){
                    sess_id= Uuid::new_v4().to_string();
                    session.push(sess_id.clone());
                }

                let mut response = Response::new(sess_id);
                if let Some(res) = response.response_200(route.1, path) {
                    self.write_event(&res);
                    return Some(true);
                }
            } else {
                return Some(self.eror_ppage(405));
            }
        } else {
            return Some(self.eror_ppage(404));
        }
        None
    }

    pub fn read_event(&mut self) -> Result<(String, Vec<u8>), u32> {
        let mut buffer = [0; 1024];
        let mut head = String::new();
        let mut body = Vec::new();
        let mut content_length = 0;

        // Get the head and first bytes of the body
        loop {
            match self.stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        return Ok((head, body));
                    }

                    match String::from_utf8(buffer[..bytes_read].to_vec()) {
                        Ok(chunk) => {
                            if let Some(index) = chunk.find("\r\n\r\n") {
                                // Split head and body when finding the double CRLF (Carriage Return Line Feed)
                                head.push_str(&chunk[..index]);
                                body.extend(&buffer[index + 4..bytes_read]);

                                // Extract Content-Length
                                if let Some(cl_index) = head.to_lowercase().find("content-length:") {
                                    let cl_str = &head[cl_index + 15..];
                                    if let Some(cl_end) = cl_str.find("\r\n") {
                                        content_length = cl_str[..cl_end].trim().parse().unwrap_or(0);
                                    }
                                }
                                break;
                            } else {
                                // If no double CRLF found, add the entire chunk to the head
                                head.push_str(&chunk);
                            }
                        }
                        Err(_) => {
                            let rest;
                            unsafe {
                                rest = String::from_utf8_unchecked(buffer.to_vec());
                            }
                            let index = rest.find("\r\n\r\n").unwrap_or(0);
                            head.push_str(rest.split_at(index).0);
                            if index == 0 {
                                body.extend(&buffer[index..bytes_read]);
                            } else {
                                body.extend(&buffer[index + 4..bytes_read]);

                                // Extract Content-Length
                                if let Some(cl_index) = head.to_lowercase().find("content-length:") {
                                    let cl_str = &head[cl_index + 15..];
                                    if let Some(cl_end) = cl_str.find("\r\n") {
                                        content_length = cl_str[..cl_end].trim().parse().unwrap_or(0);
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::Interrupted => {
                    // Resource temporarily unavailable, so retry after a short delay
                    std::thread::sleep(Duration::from_millis(50));
                    continue;
                }
                Err(_) => return Err(line!()),
            }
        }

        // Read the rest of the body
        while body.len() < content_length {
            match self.stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                    body.extend_from_slice(&buffer[..bytes_read]);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::Interrupted => {
                    // Resource temporarily unavailable, so retry after a short delay
                    std::thread::sleep(Duration::from_millis(50));
                    continue;
                }
                Err(_) => return Ok((head, body)),
            }
        }

        Ok((head, body))
    }

    pub fn write_event(&mut self, data: &str) {
        match self.stream.write_all(data.as_bytes()) {
            Ok(_) => println!("Response sent successfully"),
            Err(err) => {
                LogError::new(format!("Error writing response: {:?}", err)).log();
            }
        }
        let _ = self.stream.flush();
    }

    pub fn get_path(&self, path: &String) -> (bool, RouteConfig) {
        if path.starts_with(&self.config.alias) || &self.config.alias == path {
            if let Some(config) = &self.config.routes {
                match config.get(path) {
                    Some(route) => return (true, route.clone()),
                    None => return (false, RouteConfig::default()),
                }
            }
        }
        (false, RouteConfig::default())
    }

    pub fn check(&mut self, request: &HttpRequest) -> bool {
        let get_path = self.get_path(&request.path.clone());

        if get_path.0 {
            return get_path.1.accepted_methods.contains(&request.method);
        }
        false
    }
}
