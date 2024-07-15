use std::fs;

use server::server_start;

mod config;
mod request;
mod server;
mod error;
mod response;
fn main()  {
    server_start()
}
