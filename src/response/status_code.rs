#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    PayloadTooLarge,
    InternalServerError,
    // Add other status codes as needed
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::BadRequest => 400,
            HttpStatus::Forbidden => 403,
            HttpStatus::NotFound => 404,
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::PayloadTooLarge => 413,
            HttpStatus::InternalServerError => 500,
        }
    }

    pub fn reason_phrase(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.code(), self.reason_phrase())
    }

    pub fn from_code(code: u16) ->HttpStatus {
        match code {
            200 => HttpStatus::Ok,
            400 => HttpStatus::BadRequest,
            403 => HttpStatus::Forbidden,
            404 => HttpStatus::NotFound,
            405 => HttpStatus::MethodNotAllowed,
            413 => HttpStatus::PayloadTooLarge,
            500 => HttpStatus::InternalServerError,
            _=>HttpStatus::NotFound
        }
    }
}