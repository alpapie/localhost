use std::fs;

use server::server_start;

mod config;
mod request;
mod server;
mod error;
mod response;
mod cgi;


fn main()  {
    server_start()
}
