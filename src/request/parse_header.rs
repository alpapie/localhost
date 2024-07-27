
use std::collections::HashMap;
use std::{str, fmt};
use serde_json::Value;
use std::error::Error;
use std::io::Read;
use uuid::Uuid;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::config::Config;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<HttpBody>,
}

#[derive(Debug)]
pub enum HttpBody {
    Json(Value),
    Form(HashMap<String, String>),
    Text(String),
    Multipart(Vec<MultipartPart>),
}

#[derive(Debug)]
pub struct MultipartPart {
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidRequestLine,
    MissingHeaderValue,
    UnsupportedMediaType,
    ChunkedBodyParseError,
    MultipartParseError,
    MissingContentDispositionHeader,
    MissingFilename,
    FileCreationFailed,
    FileWriteFailed,
    FileUploadError(String),
    Tolong
}

use ParseError::*;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

impl HttpRequest {
    pub fn parse(head: &str, request_body: &[u8], cfg: &Config) -> Result<HttpRequest, ParseError> {
        let mut lines = head.lines();
        let request_line = lines.next().ok_or(InvalidRequestLine)?;
        let mut parts = request_line.split_whitespace();

        let method = parts.next().ok_or(InvalidRequestLine)?.to_string();
        let path = parts.next().ok_or(InvalidRequestLine)?.to_string();
        let version = parts.next().ok_or(InvalidRequestLine)?.to_string();
        let mut headers = HashMap::new();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let mut header_parts = line.splitn(2, ':');
            let name = header_parts.next().unwrap().trim().to_string();
            let value = header_parts.next().ok_or(MissingHeaderValue)?.trim().to_string();
            headers.insert(name, value);
        }
        // println!("contnt_ln: {:?} body_len {:?} body size: {}", headers.get("content-length"), request_body.len(), cfg.client_body_size_limit);
        let content_type = headers.get("content-type").map(|ct| ct.to_lowercase());
        
        // Parse body based on method and content type
        
        let body = match method.as_str() {
            "GET" => None,
            "DELETE" => None,
            "POST" => {
                if request_body.len() <= cfg.client_body_size_limit{
                    
                    if let Some(content_type) = &content_type {
                        if content_type.contains("multipart/form-data") {
                            // Extract boundary parameter
                            let boundary = Self::parse_boundary(content_type).ok_or(MultipartParseError)?;
                            
                            let multipart_body = Self::parse_body_multipart(request_body, &boundary, &cfg.upload_folder)?;
                            Some(HttpBody::Multipart(multipart_body))
                        } else if content_type.contains("chunked") {
                            let chunked_body = Self::parse_body_chunked(request_body)?;
                            Some(Self::parse_body_by_type(&chunked_body, content_type)?)
                        } else {
                            Some(Self::parse_body_by_type(request_body, content_type)?)
                        }
                    } else {
                        None
                    }
                }else {
                    println!("can not get thie body client_body_size_limit");
                    return Err(Tolong)
                }
            },
            _ => None,
        };

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    fn parse_body_by_type(body: &[u8], content_type: &str) -> Result<HttpBody, ParseError> {
        match content_type {
            "application/json" => Self::parse_body_json(body),
            "application/x-www-form-urlencoded" => Self::parse_body_form(body),
            "text/plain" | "text/html" => Self::parse_body_text(body),
            _ => Err(UnsupportedMediaType),
        }
    }

    pub fn parse_body_chunked(body: &[u8]) -> Result<Vec<u8>, ParseError> {
        let mut stream = body;
        let mut body = Vec::new();

        loop {
            let mut size_buf = String::new();
            let mut reader = &mut stream;
            reader.read_to_string(&mut size_buf).map_err(|_| ChunkedBodyParseError)?;
            let size = usize::from_str_radix(size_buf.trim(), 16).map_err(|_| ChunkedBodyParseError)?;

            if size == 0 {
                break;
            }

            let mut data_chunk = vec![0; size];
            reader.read_exact(&mut data_chunk).map_err(|_| ChunkedBodyParseError)?;
            body.extend_from_slice(&data_chunk);

            let mut crlf = [0; 2];
            reader.read_exact(&mut crlf).map_err(|_| ChunkedBodyParseError)?;

            if &crlf != b"\r\n" {
                return Err(ChunkedBodyParseError);
            }
        }
        Ok(body)
    }

    fn parse_body_json(body: &[u8]) -> Result<HttpBody, ParseError> {
        let json = serde_json::from_slice(body).map_err(|_| UnsupportedMediaType)?;
        Ok(HttpBody::Json(json))
    }

    fn parse_body_form(body: &[u8]) -> Result<HttpBody, ParseError> {
        let form_data = serde_urlencoded::from_bytes(body).map_err(|_| UnsupportedMediaType)?;
        Ok(HttpBody::Form(form_data))
    }

    fn parse_body_text(body: &[u8]) -> Result<HttpBody, ParseError> {
        let text = String::from_utf8(body.to_vec()).map_err(|_| UnsupportedMediaType)?;
        Ok(HttpBody::Text(text))
    }

    fn parse_boundary(content_type: &str) -> Option<String> {
        content_type.split(';')
            .find(|s| s.trim().starts_with("boundary="))
            .and_then(|s| s.trim().strip_prefix("boundary="))
            .map(|s| s.to_string())
    }

    fn parse_body_multipart(body: &[u8], boundary: &str, upload_directory: &str) -> Result<Vec<MultipartPart>, ParseError> {
        let boundary = format!("--{}", boundary);
        let mut parts = Vec::new();
        let mut start = 0;
        
        while let Some(boundary_start) = body[start..].windows(boundary.len()).position(|window| window == boundary.as_bytes()) {
            start += boundary_start + boundary.len();
            if body[start..].starts_with(b"\r\n") {
                start += 2;
            }
    
            if body[start..].starts_with(b"--") {
                break;
            }
    
            let headers_end = body[start..].windows(4).position(|window| window == b"\r\n\r\n").ok_or(MultipartParseError)? + start;
            let headers_raw = &body[start..headers_end];
            let headers = Self::parse_headers(headers_raw)?;
    
            start = headers_end + 4; // Move past the "\r\n\r\n"
    
            let part_end = body[start..].windows(boundary.len()).position(|window| window == boundary.as_bytes()).unwrap_or(body.len() - start) + start;
            let part_body = body[start..part_end].to_vec();
    
            parts.push(MultipartPart { headers: headers.clone(), body: part_body.clone() });
            if let Some(content_disposition) = headers.get("Content-Disposition") {
                if content_disposition.contains("filename=") {
                    
                    upload_file(headers.clone(), part_body, upload_directory).map_err(|e| {
                        FileUploadError(e.to_string())
                    })?;
                }
            }
    
            start = part_end;
        }
    
        Ok(parts)
    }
    
    

    fn parse_headers(raw_headers: &[u8]) -> Result<HashMap<String, String>, ParseError> {
        let mut headers = HashMap::new();
        let headers_str = str::from_utf8(raw_headers).map_err(|_| MultipartParseError)?;

        for line in headers_str.lines() {
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap().trim().to_string();
            let value = parts.next().ok_or(MultipartParseError)?.trim().to_string();
            headers.insert(name, value);
        }

        Ok(headers)
    }
    pub fn get_cookie(&self,name: &str)->Option<String>{
        if let Some( cookie) = self.headers.get("Cookie"){
            for elem in cookie.split(';'){
                let kk: Vec<&str>=elem.split('=').collect();
                if kk.len()==2 && kk[0]==name{
                    return Some(kk[1].to_owned())
                }
            }
        }
        None
    }
}


fn upload_file(headers: HashMap<String, String>, body: Vec<u8>, directory: &str) -> Result<(), ParseError> {
    let content_disposition = headers.get("Content-Disposition")
        .ok_or(MissingContentDispositionHeader)?;
    let filename = content_disposition.split(';')
    .find_map(|part| {
        let part = part.trim();
        if part.starts_with("filename=") {
            Some(part["filename=".len()..].trim_matches('"').to_string())
        } else {
            None
        }
    })
    .ok_or(MissingFilename)?;

    let file_path = Path::new(directory).join(filename);
    
    let mut file = File::create(&file_path).map_err(|_| FileCreationFailed)?;
    // println!("this is the file {:?}", body);
    file.write_all(&body).map_err(|_| FileWriteFailed)?;
    
    // println!("File uploaded: {:?}", file_path);
    Ok(())
}
