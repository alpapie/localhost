use std::collections::HashMap;
use std::fmt;
use serde_json::Value;
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
#[derive(Debug)]
pub enum ParseError {
    InvalidRequestLine,
    MissingHeaderValue,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl HttpRequest {
    pub fn parse(request: &str) -> Result<HttpRequest, ParseError> {
        let mut lines = request.lines();
        // Parse the request line
        let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        let path = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        let version = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        // Parse the headers
        let mut headers = HashMap::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let mut header_parts = line.splitn(2, ':');
            let name = header_parts.next().unwrap().trim().to_string();
            let value = header_parts.next().ok_or(ParseError::MissingHeaderValue)?.trim().to_string();
            headers.insert(name, value);
        }
        // Parse the body
        let body = lines.collect::<Vec<&str>>().join("\n");
        let body = if body.is_empty() { None } else { Some(body) };
        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }
    pub fn json_body(&self) -> Option<serde_json::Value> {
        if let Some(body) = &self.body {
            if self.headers.get("Content-Type").map(|ct| ct == "application/json").unwrap_or(false) {
                return serde_json::from_str(body).ok();
            }
        }
        None
    }
    pub fn form_body(&self) -> Option<HashMap<String, String>> {
        if let Some(body) = &self.body {
            if self.headers.get("Content-Type").map(|ct| ct == "application/x-www-form-urlencoded").unwrap_or(false) {
                let mut form_data = HashMap::new();
                for pair in body.split('&') {
                    let mut kv = pair.splitn(2, '=');
                    if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                        form_data.insert(key.to_string(), value.to_string());
                    }
                }
                return Some(form_data);
            }
        }
        None
    }
}