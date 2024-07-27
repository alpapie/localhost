use serde::Deserialize;
use std::collections::HashMap;
use std::fs;



#[derive(Deserialize, Debug)]
pub struct HostConfig {
    pub servers: Vec<Config>,
}
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub server_name: String,
    pub server_address: String,
    pub ports: Vec<u16>,
    pub error_pages: Option<ErrorPages>,
    pub client_body_size_limit: usize,
    pub routes: Option<HashMap<String, RouteConfig>>,
    pub alias: String,
    pub upload_folder: String
}

#[derive(Deserialize, Debug)]
pub struct ErrorPages {
    pub error_400: Option<String>,
    pub error_403: Option<String>,
    pub error_404: Option<String>,
    pub error_405: Option<String>,
    pub error_413: Option<String>,
    pub error_500: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RouteConfig {
    pub accepted_methods: Vec<String>,
    pub redirections: Option<String>,
    pub root_directory: String,
    pub default_file: Option<String>,
    pub cgi: Option<String>,
    pub directory_listing: bool,
    pub setcookie:Option<bool>,
    pub auth:Option<bool>

}

impl Config {
    pub fn load_from_file(path: &str) -> Result<HostConfig, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: HostConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}
