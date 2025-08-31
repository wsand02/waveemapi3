use core::fmt;
use hound::Error as HoundError;
use mp3lame_encoder::{BuildError, EncodeError};
use rocket::{http::Status, response::Responder, serde::json::Json, tokio::task::JoinError};

use crate::api::DefaultErrorResp;

#[derive(Debug)]
pub enum ObaError {
    EncoderError(EncodeError),
    BuildError(BuildError),
    HoundError(HoundError),
    IoError(std::io::Error),
    JoinError(JoinError),
}

impl fmt::Display for ObaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObaError::EncoderError(e) => write!(f, "Encoder error: {}", e),
            ObaError::BuildError(e) => write!(f, "Build error: {}", e),
            ObaError::HoundError(e) => write!(f, "Wav error: {}", e),
            ObaError::IoError(e) => write!(f, "IO error: {}", e),
            ObaError::JoinError(e) => write!(f, "Join error: {}", e),
        }
    }
}

impl From<JoinError> for ObaError {
    fn from(value: JoinError) -> Self {
        ObaError::JoinError(value)
    }
}

impl From<std::io::Error> for ObaError {
    fn from(value: std::io::Error) -> Self {
        ObaError::IoError(value)
    }
}

impl From<EncodeError> for ObaError {
    fn from(value: EncodeError) -> Self {
        ObaError::EncoderError(value)
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ObaError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            ObaError::HoundError(_) => Status::BadRequest,
            ObaError::IoError(_) => Status::InternalServerError,
            ObaError::BuildError(_) => Status::BadRequest,
            _ => Status::InternalServerError,
        };
        let error_resp = DefaultErrorResp {
            error: self.to_string(),
        };
        Json(error_resp).respond_to(request).map(|mut response| {
            response.set_status(status);
            response
        })
    }
}
