use std::{error::Error, fmt};

use sdl2::{IntegerOrSdlError, video::WindowBuildError};

#[derive(Debug)]
pub enum EngineBuildError {
    CanvasBuildError(IntegerOrSdlError),
    WindowBuildError(WindowBuildError),
    SdlError(String),
}

impl From<String> for EngineBuildError {
    fn from(e: String) -> Self {
        Self::SdlError(e)
    }
}

impl fmt::Display for EngineBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanvasBuildError(e) => write!(f, "Error constructing canvas: {}", e),
            Self::WindowBuildError(e) => write!(f, "Error constructing window: {}", e),
Self::SdlError(s) => write!(f, "SDL error: {}", s),
        }
    }
}

impl Error for EngineBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CanvasBuildError(e) => Some(e),
            Self::WindowBuildError(e) => Some(e),
            Self::SdlError(_) => None,
        }
    }
}
