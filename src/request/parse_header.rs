use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use std::{fmt, str};

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
    JsonValueError, // Fixed typo
    ChunkError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

impl HttpRequest {
    pub fn parse(request: &str) -> Result<HttpRequest, ParseError> {
        let mut lines = request.lines();
        // Parse the request line
        let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;
        let mut parts = request_line.split_whitespace();

        let method = parts
            .next()
            .ok_or(ParseError::InvalidRequestLine)?
            .to_string();
        let path = parts
            .next()
            .ok_or(ParseError::InvalidRequestLine)?
            .to_string();
        let version = parts
            .next()
            .ok_or(ParseError::InvalidRequestLine)?
            .to_string();

        // Parse the headers
        let mut headers = HashMap::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let mut header_parts = line.splitn(2, ':');
            let name = header_parts.next().unwrap().trim().to_string();
            let value = header_parts
                .next()
                .ok_or(ParseError::MissingHeaderValue)?
                .trim()
                .to_string();
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

    pub fn parse_headers(request_headers: &[u8]) -> Result<HashMap<String, String>, ParseError> {
        let mut headers = HashMap::new();
        let request =
            str::from_utf8(request_headers).map_err(|_| ParseError::InvalidRequestLine)?;

        let mut lines = request.lines();
        let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;
        let mut parts = request_line.split_whitespace();

        // You may choose to store these values if needed
        // let _method = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        // let _path = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        // let _version = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut key_value = line.splitn(2, ':');
            let key = key_value.next().unwrap().trim().to_string();
            let value = key_value
                .next()
                .ok_or(ParseError::MissingHeaderValue)?
                .trim()
                .to_string();
            headers.insert(key, value);
        }

        Ok(headers)
    }

    // Cette fonction traite le body selon l'application json
    pub fn parse_body_json(&self, body: &[u8]) -> Result<Value, ParseError> {
        let request_body = str::from_utf8(body).map_err(|_| ParseError::JsonValueError)?;

        if self
            .headers
            .get("Content-Type")
            .map(|ct| ct == "application/json")
            .unwrap_or(false)
        {
            serde_json::from_str(request_body).map_err(|_| ParseError::JsonValueError)
        } else {
            Err(ParseError::JsonValueError)
        }
    }

    // Cette fonction traite les données post du body selon l'application x-www-form-urlencoded
    pub fn parse_body_form(&self, body: &[u8]) -> Result<HashMap<String, String>, ParseError> {
        let request_body = str::from_utf8(body).map_err(|_| ParseError::InvalidRequestLine)?;

        if self
            .headers
            .get("Content-Type")
            .map(|ct| ct == "application/x-www-form-urlencoded")
            .unwrap_or(false)
        {
            let mut form_data = HashMap::new();
            for pair in request_body.split('&') {
                let mut kv = pair.splitn(2, '=');
                if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                    form_data.insert(key.to_string(), value.to_string());
                }
            }
            Ok(form_data)
        } else {
            Err(ParseError::InvalidRequestLine) // Or another appropriate error
        }
    }

    pub fn parse_body_chunked(stream: &mut dyn BufRead) -> Result<Vec<u8>, ParseError> {
        let mut body = Vec::new();

        loop {
            let mut size_buf = String::new();
            stream
                .read_line(&mut size_buf)
                .map_err(|_| ParseError::ChunkError)?;
            let size =
                usize::from_str_radix(size_buf.trim(), 16).map_err(|_| ParseError::ChunkError)?;

            if size == 0 {
                break;
            }

            let mut data_chunk = vec![0; size];
            stream
                .read_exact(&mut data_chunk)
                .map_err(|_| ParseError::ChunkError)?;
            body.extend_from_slice(&data_chunk);

            let mut crlf = vec![0; 2];
            stream
                .read_exact(&mut crlf)
                .map_err(|_| ParseError::ChunkError)?;

            // Check if CRLF is correct
            if &crlf != b"\r\n" {
                return Err(ParseError::ChunkError);
            }
        }
        Ok(body)
    }
}
