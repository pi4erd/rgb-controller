use std::fmt::Display;

use openrgb::OpenRGBError;

#[derive(Debug)]
pub enum RunError {
    OpenRGBError {
        message: String,
        error: OpenRGBError,
    },
}

impl Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenRGBError { message, error } => {
                write!(f, "OpenRGB: {message}: {error}")
            }
        }
    }
}

impl std::error::Error for RunError {}
