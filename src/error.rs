use core::fmt;

use rocket::{http::Status, response::Responder, serde::json::Json};

use crate::api::DefaultErrorResp;

#[derive(Debug)]
pub enum WaveemapiError {
    Encoder(mp3lame_encoder::EncodeError),
    Build(mp3lame_encoder::BuildError),
    Hound(hound::Error),
    Io(std::io::Error),
    Join(rocket::tokio::task::JoinError),
    Id3Tag(mp3lame_encoder::Id3TagError),
}

impl fmt::Display for WaveemapiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WaveemapiError::Encoder(e) => write!(f, "Encoder error: {}", e),
            WaveemapiError::Build(e) => write!(f, "Build error: {}", e),
            WaveemapiError::Hound(e) => write!(f, "Wav error: {}", e),
            WaveemapiError::Io(e) => write!(f, "IO error: {}", e),
            WaveemapiError::Join(e) => write!(f, "Join error: {}", e),
            WaveemapiError::Id3Tag(_) => write!(f, "ID3 tag error"),
        }
    }
}

impl From<rocket::tokio::task::JoinError> for WaveemapiError {
    fn from(value: rocket::tokio::task::JoinError) -> Self {
        WaveemapiError::Join(value)
    }
}

impl From<std::io::Error> for WaveemapiError {
    fn from(value: std::io::Error) -> Self {
        WaveemapiError::Io(value)
    }
}

impl From<mp3lame_encoder::EncodeError> for WaveemapiError {
    fn from(value: mp3lame_encoder::EncodeError) -> Self {
        WaveemapiError::Encoder(value)
    }
}

impl From<hound::Error> for WaveemapiError {
    fn from(value: hound::Error) -> Self {
        WaveemapiError::Hound(value)
    }
}

impl From<mp3lame_encoder::Id3TagError> for WaveemapiError {
    fn from(value: mp3lame_encoder::Id3TagError) -> Self {
        WaveemapiError::Id3Tag(value)
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for WaveemapiError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            WaveemapiError::Hound(_) => Status::BadRequest,
            WaveemapiError::Io(_) => Status::InternalServerError,
            WaveemapiError::Build(_) => Status::BadRequest,
            _ => Status::InternalServerError,
        };
        let message = match self {
            WaveemapiError::Encoder(_) => "Failed to encode MP3".to_string(),
            WaveemapiError::Hound(_) => "Invalid WAV file".to_string(),
            WaveemapiError::Io(_) => "Internal server error".to_string(),
            WaveemapiError::Build(_) => "Failed to build encoder".to_string(),
            _ => "An error occurred".to_string(),
        };
        let error_resp = DefaultErrorResp { error: message };
        Json(error_resp).respond_to(request).map(|mut response| {
            response.set_status(status);
            response
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    #[test]
    fn test_from_io_error() {
        // keep this for now to make adding more tests easier later on.
        let err = io::Error::other("fail");
        let we = WaveemapiError::from(err);
        match we {
            WaveemapiError::Io(_) => {}
            _ => panic!("Wrong variant"),
        }
    }
}
