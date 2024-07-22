use std::fs;

use server::server_start;

mod cgi;
mod config;
mod error;
mod request;
mod response;
mod server;

fn main() {
    server_start()
}
