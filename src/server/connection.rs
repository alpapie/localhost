use std::io::{ BufReader, Error, ErrorKind, Read, Write};
use std::time::Instant;
use mio:: Token;
use mio::net::TcpStream;

use crate::config::config::RouteConfig;
use crate::config::Config;
use crate::error::LogError;
use crate::request::parse_header::HttpRequest;
use crate::response::response::Response;

#[derive(Debug)]
pub struct ConnectionHandler <'a>{
    pub stream: TcpStream,
    pub token: Token,
    pub last_activity: Instant,
    pub config: &'a Config
}

impl <'a> ConnectionHandler <'a>{
    pub fn new(stream: TcpStream, token: Token,config: &'a Config) -> Self {
        ConnectionHandler { stream, token,last_activity: Instant::now(),config }
    }

    pub fn handle_event(&mut self, event: &mio::event::Event) -> bool{
        if event.is_readable() {
            match self.read_event(){
                Ok((head,body)) => {
                    let b_request= HttpRequest::parse(&head);
                    if let Ok(request) =b_request{
                        if self.check(&request){
                            let route=self.get_path(&request.path);
                            let mut response=Response::new();
                            if let Some(res)=response.response_200(route.1,request.path){
                                print!("header : {:?}",res);
                                self.write_event(&res);
                                return true
                            }
                        }
                    }
                    let mut response=Response::new();
                    let res=response.response_error(404,self.config);
                    self.write_event(&res);
                    return true;
                },
                Err(err) => {
                    println!("Error read request-> {:?}", err);
                    LogError::new(format!("Error read request-> {:?}", err)).log();
                },
            } 
        }
        println!("reade erroeoeoeoeoeo");
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

    pub fn write_event(&mut self,data : &str) {
        match self.stream.write_all(data.as_bytes()) {
            Ok(_) => println!("Response sent successfully"),
            Err(err) => {
                LogError::new(format!("Error writing response: {:?}", err)).log();
            }
        }
    }

    pub fn get_path(&self, path: &String )->(bool,RouteConfig){
        if let  Some(config)=&self.config.routes {
            match config.get(path) {
                Some(route) =>{
                   return  (true, route.clone())
                },
                None => return  (false,RouteConfig::default()),
            }
        }
        return (false,RouteConfig::default())
    }

    pub fn check(&mut self,request: &HttpRequest )->bool{
        let get_path=self.get_path(&request.path.clone());
        if get_path.0 {
           return get_path.1.accepted_methods.contains(&request.method)
        }
        self.write_event("404");
        return false
    }

    pub fn check_body_size(self,config: Config,body_size: usize)->bool{
        config.client_body_size_limit==body_size
    }
}
