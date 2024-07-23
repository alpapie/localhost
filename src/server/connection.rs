use mio::net::TcpStream;
use mio::Token;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::time::Instant;

use crate::config::config::RouteConfig;
use crate::config::Config;
use crate::error::LogError;
use crate::request::parse_header::HttpRequest;
use crate::response::response::Response;

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

    pub fn handle_event(&mut self, event: &mio::event::Event) -> bool {
        if event.is_readable() {
            match self.read_event() {
                Ok((head, body)) => {
                    if head.is_empty() {
                        return false;
                    }
                    let b_request = HttpRequest::parse(&head);
                    if let Ok(request) = b_request {
                        let mut max_redirect: u32 = 10;
                        // println!("head {} ->len({}) body {:?} ->({})",head,head.len(),body,body.len());
                        if let Some(value) = self.get_response(request, &mut max_redirect) {
                            if max_redirect < 1 {
                                self.eror_ppage(310);
                            }
                            return value;
                        }
                    }
                    return self.eror_ppage(400);
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
        let mut response = Response::new();
        let res = response.response_error(status, self.config);
        self.write_event(&res);
        true
    }

    fn get_response(&mut self, mut request: HttpRequest, max_redirect: &mut u32) -> Option<bool> {
        let route = self.get_path(&request.path);

        if *max_redirect < 1 {
            return Some(true);
        }
        *max_redirect -= 1;
        if route.0 {

            if route.1.redirections.is_some() {
                request.path = route.1.redirections.unwrap();
                self.get_response(request, max_redirect);
                return Some(true);
            }
            if self.check(&request) {
                let mut response = Response::new();
                let path = match request.path.strip_prefix(&self.config.alias) {
                    Some(content) => content.to_owned(),
                    None => "".to_owned(),
                };
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

    pub fn read_event(&mut self) -> Result<(String, Vec<u8>), Error> {
        let mut buf_reader = BufReader::new(&mut self.stream);
        let mut head = String::new();
        let mut body = Vec::new();
        let mut content_length: Option<usize> = None;

        // Read headers
        loop {
            let mut line = String::new();
            match buf_reader.read_line(&mut line) {
                Ok(0) => break, // End of stream
                Ok(_) => {
                    if line == "\r\n" {
                        break; // End of headers
                    }
                    if line.starts_with("Content-Length:") {
                        content_length = line[15..].trim().parse().ok();
                    }
                    head.push_str(&line);
                }
                Err(err) => return Err(err),
            }
        }
        if let Some(length) = content_length {
            let mut buffer = vec![0; length];
            buf_reader.read_exact(&mut buffer)?;
            body = buffer;
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

    pub fn check_body_size(self, config: Config, body_size: usize) -> bool {
        config.client_body_size_limit == body_size
    }
}
