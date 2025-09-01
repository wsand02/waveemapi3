use core::fmt;
use hound::Error as HoundError;
use mp3lame_encoder::{BuildError, EncodeError};
use rocket::{http::Status, response::Responder, serde::json::Json, tokio::task::JoinError};

use crate::api::DefaultErrorResp;

#[derive(Debug)]
pub enum WaveemapiError {
    EncoderError(EncodeError),
    BuildError(BuildError),
    HoundError(HoundError),
    IoError(std::io::Error),
    JoinError(JoinError),
}

impl fmt::Display for WaveemapiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WaveemapiError::EncoderError(e) => write!(f, "Encoder error: {}", e),
            WaveemapiError::BuildError(e) => write!(f, "Build error: {}", e),
            WaveemapiError::HoundError(e) => write!(f, "Wav error: {}", e),
            WaveemapiError::IoError(e) => write!(f, "IO error: {}", e),
            WaveemapiError::JoinError(e) => write!(f, "Join error: {}", e),
        }
    }
}

impl From<JoinError> for WaveemapiError {
    fn from(value: JoinError) -> Self {
        WaveemapiError::JoinError(value)
    }
}

impl From<std::io::Error> for WaveemapiError {
    fn from(value: std::io::Error) -> Self {
        WaveemapiError::IoError(value)
    }
}

impl From<EncodeError> for WaveemapiError {
    fn from(value: EncodeError) -> Self {
        WaveemapiError::EncoderError(value)
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for WaveemapiError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            WaveemapiError::HoundError(_) => Status::BadRequest,
            WaveemapiError::IoError(_) => Status::InternalServerError,
            WaveemapiError::BuildError(_) => Status::BadRequest,
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
