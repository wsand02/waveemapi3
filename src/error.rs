use core::fmt;
use hound::Error as HoundError;
use mp3lame_encoder::{BuildError, EncodeError};

#[derive(Debug)]
pub enum ObaError {
    EncoderError(EncodeError),
    BuildError(BuildError),
    HoundError(HoundError),
    IoError(std::io::Error),
}

impl fmt::Display for ObaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObaError::EncoderError(e) => write!(f, "Encoder error: {}", e),
            ObaError::BuildError(e) => write!(f, "Build error: {}", e),
            ObaError::HoundError(e) => write!(f, "Wav error: {}", e),
            ObaError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}
