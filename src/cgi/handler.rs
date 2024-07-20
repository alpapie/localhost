use std::{
    collections::HashMap,
    env,
    process::{Child, Command, Output, Stdio},
};

use crate::response::response::Response;

// REQUEST will be imported from request mod
pub struct Request {}

pub struct CGIHandler <'a>{
    script_path: String,
    cgi:&'a str
}

impl CGIHandler<'_> {
    pub fn new(script_path: String) -> Self {

        CGIHandler { script_path, cgi:"" }
    }

    // pub fn handle_request(&self, request: &Request) -> Result<Response, String> {
    //     // self.setup_environment(request);
    //     match self.execute_script(request) {
    //         Ok(output) => self.parse_cgi_output(output),
    //         Err(e) => Err(format!("Failed to execute CGI script: {}", e)),
    //     }
    // }

    // fn setup_environment(&self, request: &Request) {
    //     // env::set_var("REQUEST_METHOD", &request.method);
    //     // env::set_var("PATH_INFO", &request.path);
    // }

    fn execute_script(&self, request: &Request) -> Result<Child, String> {
        let mut child = Command::new(&self.script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            // .env("REQUEST_METHOD", request.method)
            // .env("PATH_INFO", &request.path)
            .spawn()
            .expect("Failed to spawn CGI script");

        Ok(child)
    }

    fn parse_cgi_output(&self, mut child: Child) -> Result<String, String> {
        let output = child
            .wait_with_output()
            .map_err(|e| format!("Failed to read output: {}", e))?;
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut headers = HashMap::new();
        let mut body = String::new();
        let mut status_code: u16 = 200;

        for line in output_str.lines() {
            if line.starts_with("Status:") {
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    if let Ok(s) = parts[1].parse() {
                        status_code = s;
                    }
                }
            } else if line.contains(":") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    headers.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            } else if line.trim().is_empty() {
                // End of headers, remaining part is body
                body = line.to_string();
            } else {
                body.push_str(line);
                body.push('\n');
            }
        }
        Ok(body)
    }

    pub fn find_path(&self) -> Option<&str>{
        match self.cgi {
            "py" | "python"=> Some("/path/to/script"),
            "php"=> Some("/path/to/script"),
            _ => None
        }
    }
}
