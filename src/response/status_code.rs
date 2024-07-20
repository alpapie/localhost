#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    PayloadTooLarge,
    InternalServerError,
    TooManyRediection,
    // Add other status codes as needed
}
use HttpStatus::*;
impl HttpStatus {
    pub fn code(&self) -> u16 {
        match self {
            Ok => 200,
            BadRequest => 400,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            PayloadTooLarge => 413,
            InternalServerError => 500,
            TooManyRediection=>310
        }
    }

    pub fn reason_phrase(&self) -> &'static str {
        match self {
            Ok => "OK",
            BadRequest => "Bad Request",
            Forbidden => "Forbidden",
            NotFound => "Not Found",
            MethodNotAllowed => "Method Not Allowed",
            PayloadTooLarge => "Payload Too Large",
            InternalServerError => "Internal Server Error",
            TooManyRediection=>"Too Many Redirects"
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.code(), self.reason_phrase())
    }

    pub fn from_code(code: u16) ->HttpStatus {
        match code {
            200 => Ok,
            400 => BadRequest,
            403 => Forbidden,
            404 => NotFound,
            405 => MethodNotAllowed,
            413 => PayloadTooLarge,
            500 => InternalServerError,
            310 => TooManyRediection,
            _=>NotFound
        }
    }
}