use serde::Serialize;
use actix_web::{HttpResponse, error::ResponseError};

#[derive(Debug, Fail, Serialize)]
pub enum ServerError {
    #[fail(display="{} Bad Request", _0)]
    InsertFailure(String),
    #[fail(display="{} Not found", _0)]
    ObjectNotFound(String),
    #[fail(display="{} Error Retrieving from DB", _0)]
    ErrorRetrievingData(String),
    #[fail(display="{} TokenCreationError", _0)]
    TokenCreationError(String),
    #[fail(display="{} BadQueryParameters", _0)]
    BadQueryParameters(String),
}

#[derive(Serialize)]
struct ErrorMessage {
    cause: String,
    message: String
}

impl ErrorMessage {

    fn new(cause: &str, message: &str) -> Self {
        ErrorMessage {
            cause: cause.to_string(),
            message: message.to_string()
        }
    }
}


impl ResponseError for ServerError {

    fn error_response(&self) -> HttpResponse {
        match *self {
            ServerError::InsertFailure(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("InsertFailure", message)),
            ServerError::ObjectNotFound(ref message) => HttpResponse::NotFound()
                .json(ErrorMessage::new("ObjectNotFound", message)),
            ServerError::ErrorRetrievingData(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("ErrorRetrievingData", message)),
            ServerError::TokenCreationError(ref message) => HttpResponse::InternalServerError()
                .json(ErrorMessage::new("TokenCreationError", message)),
            ServerError::BadQueryParameters(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("BadQueryParameters", message))
        }
    }
}
