pub struct Config {
    host: String,
    port: Vec<u8>,
}

use http::{Method, Request, Response, StatusCode};

struct ServerConfig<'a> {
    server_address: &'a str,
    port: Vec<u32>,
    body_size: u32,
    error_page: &'a str,
    default_server: String,
    routes: Vec<Route<'a>>,
}

struct Route<'a> {
    path: &'a str,
    methods: Vec<Method>,
}
