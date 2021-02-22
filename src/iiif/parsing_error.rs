use super::super::ResponderWrapper;

#[derive(Debug)]
pub enum ErrorReturnType {
    BadRequest(String),
    NotFound(String),
    InternalError(String),
    // NotImplemented(String)
}

impl ErrorReturnType {
    pub fn get_response(&self) -> ResponderWrapper {
        match self {
            ErrorReturnType::BadRequest(s) => ResponderWrapper::bad_request(&s),
            ErrorReturnType::InternalError(s) => ResponderWrapper::server_error(&s),
            ErrorReturnType::NotFound(s) => ResponderWrapper::not_found(&s),
            // ErrorReturnType::NotImplemented(s) => ResponderWrapper::not_implemented(&s)
        }
    }
}
