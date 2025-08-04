use core::fmt;
use hound::Error as HoundError;
use mp3lame_encoder::{BuildError, EncodeError, Id3TagError};

#[derive(Debug)]
pub enum Mp3Error {
    EncoderError(EncodeError),
    BuildError(BuildError),
    Id3TagError(Id3TagError),
}

impl fmt::Display for Mp3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mp3Error::EncoderError(e) => write!(f, "Encoder error: {}", e),
            Mp3Error::BuildError(e) => write!(f, "Build error: {}", e),
            Mp3Error::Id3TagError(e) => write!(f, "Id3Tag error: {:?}", e),
        }
    }
}

#[derive(Debug)]
pub enum WavError {
    HoundError(HoundError),
}

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WavError::HoundError(e) => write!(f, "Wav error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
    InvalidServerSetup,
}
