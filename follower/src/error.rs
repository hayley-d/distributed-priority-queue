use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

/// Custom Error type for API error responses.
#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    /// Database Error occurs when there is an error with the database.
    DatabaseError(String),
    /// Internal Server Error occurs when there is any other error not related to the database.
    InternalServerError(String),
    /// Empty heap error occurs when the heap is empty
    EmptyHeapError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::DatabaseError(s) => write!(f, "Database Error: {}", s),
            ApiError::InternalServerError(s) => write!(f, "Internal Server Error: {}", s),
            ApiError::EmptyHeapError => write!(f, "Empty Heap Error"),
        }
    }
}

impl std::error::Error for ApiError {}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let message = format!("{:?}", self);
        let status = match self {
            ApiError::DatabaseError(_) => Status::InternalServerError,
            ApiError::InternalServerError(_) => Status::InternalServerError,
            ApiError::EmptyHeapError => Status::InternalServerError,
        };

        Response::build()
            .status(status)
            .header(ContentType::Plain)
            .sized_body(message.len(), Cursor::new(message))
            .ok()
    }
}
