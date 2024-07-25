// use serde_json::Value;
// use std::collections::HashMap;
// use std::error::Error;
// use std::io::BufRead;
// use std::{fmt str};

// #[derive(Debug)]
// pub struct HttpRequest {
//     pub method: String,
//     pub path: String,
//     pub version: String,
//     pub headers: HashMap<String, String>,
//     pub body: Option<String>,
// }

// #[derive(Debug)]
// pub enum ParseError {
//     InvalidRequestLine,
//     MissingHeaderValue,
//     JsonValueError, // Fixed typo
//     ChunkError,
// }

// impl fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl Error for ParseError {}

// impl HttpRequest {
//     pub fn parse(request: &str) -> Result<HttpRequest, ParseError> {
//         let mut lines = request.lines();
//         // Parse the request line
//         let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;
//         let mut parts = request_line.split_whitespace();

//         let method = parts
//             .next()
//             .ok_or(ParseError::InvalidRequestLine)?
//             .to_string();
//         let path = parts
//             .next()
//             .ok_or(ParseError::InvalidRequestLine)?
//             .to_string();
//         let version = parts
//             .next()
//             .ok_or(ParseError::InvalidRequestLine)?
//             .to_string();

//         // Parse the headers
//         let mut headers = HashMap::new();
//         for line in &mut lines {
//             if line.is_empty() {
//                 break;
//             }
//             let mut header_parts = line.splitn(2, ':');
//             let name = header_parts.next().unwrap().trim().to_string();
//             let value = header_parts
//                 .next()
//                 .ok_or(ParseError::MissingHeaderValue)?
//                 .trim()
//                 .to_string();
//             headers.insert(name, value);
//         }

//         // Parse the body
//         let body = lines.collect::<Vec<&str>>().join("\n");
//         let body = if body.is_empty() { None } else { Some(body) };

//         Ok(HttpRequest {
//             method,
//             path,
//             version,
//             headers,
//             body,
//         })
//     }

//     pub fn parse_headers(request_headers: &[u8]) -> Result<HashMap<String, String>, ParseError> {
//         let mut headers = HashMap::new();
//         let request =
//             str::from_utf8(request_headers).map_err(|_| ParseError::InvalidRequestLine)?;

//         let mut lines = request.lines();
//         let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;
//         let mut parts = request_line.split_whitespace();

//         // You may choose to store these values if needed
//         // let _method = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
//         // let _path = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
//         // let _version = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();

//         for line in lines.by_ref() {
//             if line.is_empty() {
//                 break;
//             }
//             let mut key_value = line.splitn(2, ':');
//             let key = key_value.next().unwrap().trim().to_string();
//             let value = key_value
//                 .next()
//                 .ok_or(ParseError::MissingHeaderValue)?
//                 .trim()
//                 .to_string();
//             headers.insert(key, value);
//         }

//         Ok(headers)
//     }

//     // Cette fonction traite le body selon l'application json
//     pub fn parse_body_json(&self, body: &[u8]) -> Result<Value, ParseError> {
//         let request_body = str::from_utf8(body).map_err(|_| ParseError::JsonValueError)?;

//         if self
//             .headers
//             .get("Content-Type")
//             .map(|ct| ct == "application/json")
//             .unwrap_or(false)
//         {
//             serde_json::from_str(request_body).map_err(|_| ParseError::JsonValueError)
//         } else {
//             Err(ParseError::JsonValueError)
//         }
//     }

//     // Cette fonction traite les données post du body selon l'application x-www-form-urlencoded
//     pub fn parse_body_form(&self, body: &[u8]) -> Result<HashMap<String, String>, ParseError> {
//         let request_body = str::from_utf8(body).map_err(|_| ParseError::InvalidRequestLine)?;

//         if self
//             .headers
//             .get("Content-Type")
//             .map(|ct| ct == "application/x-www-form-urlencoded")
//             .unwrap_or(false)
//         {
//             let mut form_data = HashMap::new();
//             for pair in request_body.split('&') {
//                 let mut kv = pair.splitn(2, '=');
//                 if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
//                     form_data.insert(key.to_string(), value.to_string());
//                 }
//             }
//             Ok(form_data)
//         } else {
//             Err(ParseError::InvalidRequestLine) // Or another appropriate error
//         }
//     }

//     pub fn parse_body_chunked(stream: &mut dyn BufRead) -> Result<Vec<u8>, ParseError> {
//         let mut body = Vec::new();

//         loop {
//             let mut size_buf = String::new();
//             stream
//                 .read_line(&mut size_buf)
//                 .map_err(|_| ParseError::ChunkError)?;
//             let size =
//                 usize::from_str_radix(size_buf.trim(), 16).map_err(|_| ParseError::ChunkError)?;

//             if size == 0 {
//                 break;
//             }

//             let mut data_chunk = vec![0; size];
//             stream
//                 .read_exact(&mut data_chunk)
//                 .map_err(|_| ParseError::ChunkError)?;
//             body.extend_from_slice(&data_chunk);

//             let mut crlf = vec![0; 2];
//             stream
//                 .read_exact(&mut crlf)
//                 .map_err(|_| ParseError::ChunkError)?;

// use std::collections::HashMap;
//             // Check if CRLF is correct
//             if &crlf != b"\r\n" {
//                 return Err(ParseError::ChunkError);
//             }
//         }
//         Ok(body)
//     }
//     pub fn get_cookie(&self,name: &str)->Option<String>{
//         if let Some( cookie) = self.headers.get("Cookie"){
//             for elem in cookie.split(';'){
//                 let kk: Vec<&str>=elem.split('=').collect();
//                 if kk.len()==2 && kk[0]==name{
//                     return Some(kk[1].to_owned())
//                 }
//             }
//         }
//         None
//     }
// }
use std::collections::HashMap;
use std::{str, fmt};
use serde_json::Value;
use std::error::Error;
use std::io::Read;
use uuid::Uuid;
use std::fs::File;
use std::io::Write;
use std::path::Path;
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    // pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<HttpBody>,
}
#[derive(Debug)]
pub enum HttpBody {
    Json(Value),
    Form(HashMap<String, String>),
    Text(String),
    Multipart(Vec<MultipartPart>),
    // Add other body types as needed
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
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ParseError {}
// impl From<std::io::Error> for ParseError {
//     fn from(err: std::io::Error) -> Self {
//         ParseError::IoError(err)
//     }
// }
// impl From<std::str::Utf8Error> for ParseError {
//     fn from(_: std::str::Utf8Error) -> Self {
//         ParseError::Utf8Error
//     }
// }
impl HttpRequest {
    pub fn parse(head: &str, request_body: &[u8]) -> Result<HttpRequest, ParseError> {
        let mut lines = head.lines();
        let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        let path = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        // If HTTP version is needed, it can be processed here
        // let version = parts.next().ok_or(ParseError::InvalidRequestLine)?.to_string();
        // Parse headers
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
        // Determine the content type
        let content_type = headers.get("content-type").map(|ct| ct.to_lowercase());
        
        // Parse body based on method and content type
        let body = match method.as_str() {
            "GET" => None, // Typically no body in GET requests
            "POST" => {
                if let Some(content_type) = &content_type {
                    if content_type.contains("multipart/form-data") {
                        // Extract boundary parameter
                        let boundary = Self::parse_boundary(content_type).ok_or(ParseError::MultipartParseError)?;
                        let multipart_body = Self::parse_body_multipart(request_body, &boundary, "/test/")?;
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
            },
            _ => None, // Handle other methods if needed
        };
        Ok(HttpRequest {
            method,
            path,
            headers,
            body,
        })
    }
    fn parse_body_by_type(body: &[u8], content_type: &str) -> Result<HttpBody, ParseError> {
        match content_type {
            "application/json" => Self::parse_body_json(body),
            "application/x-www-form-urlencoded" => Self::parse_body_form(body),
            "text/plain" | "text/html" => Self::parse_body_text(body),
            _ => Err(ParseError::UnsupportedMediaType),
        }
    }
    pub fn parse_body_chunked(body: &[u8]) -> Result<Vec<u8>, ParseError> {
        let mut stream = body;
        let mut body = Vec::new();
        
        loop {
            let mut size_buf = String::new();
            let mut reader = &mut stream;
            reader.read_to_string(&mut size_buf).map_err(|_| ParseError::ChunkedBodyParseError)?;
            let size = usize::from_str_radix(size_buf.trim(), 16).map_err(|_| ParseError::ChunkedBodyParseError)?;
            if size == 0 {
                break;
            }
            let mut data_chunk = vec![0; size];
            reader.read_exact(&mut data_chunk).map_err(|_| ParseError::ChunkedBodyParseError)?;
            body.extend_from_slice(&data_chunk);
            let mut crlf = [0; 2];
            reader.read_exact(&mut crlf).map_err(|_| ParseError::ChunkedBodyParseError)?;
            if &crlf != b"\r\n" {
                return Err(ParseError::ChunkedBodyParseError);
            }
        }
        Ok(body)
    }
    fn parse_body_json(body: &[u8]) -> Result<HttpBody, ParseError> {
        let json = serde_json::from_slice(body).map_err(|_| ParseError::UnsupportedMediaType)?;
        Ok(HttpBody::Json(json))
    }
    fn parse_body_form(body: &[u8]) -> Result<HttpBody, ParseError> {
        // let form_data = serde_urlencoded::from_bytes(body).map_err(|_| ParseError::UnsupportedMediaType)?;
        Ok(HttpBody::Form(HashMap::new()))
    }
    fn parse_body_text(body: &[u8]) -> Result<HttpBody, ParseError> {
        let text = String::from_utf8(body.to_vec()).map_err(|_| ParseError::UnsupportedMediaType)?;
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
            
            let headers_end = body[start..].windows(2).position(|window| window == b"\r\n\r\n").ok_or(ParseError::MultipartParseError)? + start;
            let headers_raw = &body[start..headers_end];
            let headers = Self::parse_headers(headers_raw)?;
            
            start = headers_end + 4;
            
            println!("\n<<<HERE>>> {:?}", headers);
            let part_end = body[start..].windows(boundary.len()).position(|window| window == boundary.as_bytes()).unwrap_or(body.len() - start) + start;
            
            let part_body = body[start..part_end].to_vec();
            
            parts.push(MultipartPart { headers: headers.clone(), body: part_body.clone() });
            
            if let Some(content_disposition) = headers.get("Content-Disposition") {
                if content_disposition.contains("filename=") {
                    upload_file(headers.clone(), part_body, upload_directory).map_err(|e| {
                        // Convert Box<dyn Error> to ParseError
                        ParseError::FileUploadError(e.to_string())
                    })?;
                }
            }
            
            start = part_end;
        }
        
        
        
        Ok(parts)
    }
    
    
    fn parse_headers(raw_headers: &[u8]) -> Result<HashMap<String, String>, ParseError> {
        let mut headers = HashMap::new();
        let headers_str = str::from_utf8(raw_headers).map_err(|_| ParseError::MultipartParseError)?;
        for line in headers_str.lines() {
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap().trim().to_string();
            let value = parts.next().ok_or(ParseError::MultipartParseError)?.trim().to_string();
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
// fn parse_multipart(body: &[u8], headers: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
//     let content_type = headers.get("content-type").ok_or("Missing Content-Type header")?;
//     let boundary = content_type.split(';')
//     .find_map(|part| {
//             let part = part.trim();
//             if part.starts_with("boundary=") {
//                 Some(&part["boundary=".len()..])
//             } else {
//                 None
//             }
//         })
//         .ok_or("Missing boundary in Content-Type header")?;
        
//     let boundary_str = format!("--{}", boundary);
//     let parts: Vec<&str> = str::from_utf8(body)?.split(&boundary_str).collect();
    
//     for part in parts {
//         if part.trim().is_empty() {
//             continue;
//         }
        
//         let mut lines = part.lines();
//         let mut part_headers = HashMap::new();
//         println!("rrrequest {:?}", lines);
//         while let Some(line) = lines.next() {
//             if line.is_empty() {
//                 break;
//             }
//             let mut header_parts = line.splitn(2, ':');
//             let key = header_parts.next().unwrap().trim().to_string();
//             let value = header_parts.next().unwrap_or("").trim().to_string();
//             part_headers.insert(key, value);
//         }
        
//         let body: Vec<u8> = lines.collect::<Vec<&str>>().join("\r\n").as_bytes().to_vec();
//         // upload_file(part_headers, body, "./test".to_string())?;
//     }
//     Ok(())
// }
fn upload_file(headers: HashMap<String, String>, body: Vec<u8>, directory: &str) -> Result<(), ParseError> {
    let content_disposition = headers.get("Content-Disposition")
        .ok_or(ParseError::MissingContentDispositionHeader)?;
    
    let filename = content_disposition.split(';')
        .find_map(|part| {
            let part = part.trim();
            if part.starts_with("filename=") {
                Some(part["filename=".len()..].trim_matches('"').to_string())
            } else {
                None
            }
        })
        .ok_or(ParseError::MissingFilename)?;
    
    let file_path = Path::new(directory).join(filename);
    
    let mut file = File::create(&file_path).map_err(|_| ParseError::FileCreationFailed)?;
    file.write_all(&body).map_err(|_| ParseError::FileWriteFailed)?;
    
    println!("File uploaded: {:?}", file_path);
    Ok(())
}