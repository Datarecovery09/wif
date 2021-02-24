use std::fmt::Display;

use tide::StatusCode;

#[derive(Debug)]
pub struct WifError {
    pub status: StatusCode,
    pub message: String
}
impl Display for WifError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} - {}", self.status, self.message)
    }
}
impl std::error::Error for WifError {}


impl WifError {
    pub fn not_found(m: String) -> Self {
        WifError {
            status: StatusCode::NotFound,
            message: m
        }
    }
    pub fn bad_request(m: String) -> Self {
        WifError {
            status: StatusCode::BadRequest,
            message: m
        }
    }
    pub fn internal_error(m: String) -> Self {
        WifError {
            status: StatusCode::InternalServerError,
            message: m
        }
    }
}
