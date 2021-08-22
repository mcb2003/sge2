use std::{error::Error, fmt};

use crate::TextureValueError;

#[derive(Debug)]
pub enum LoadTextureError {
    LoadError(String),
    ValueError(TextureValueError),
}

impl fmt::Display for LoadTextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadError(s) => write!(f, "Error loading texture: {}", s),
            Self::ValueError(e) => write!(f, "Value error: {}", e),
        }
    }
}

impl Error for LoadTextureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LoadError(_) => None,
            Self::ValueError(e) => Some(e),
        }
    }
}
