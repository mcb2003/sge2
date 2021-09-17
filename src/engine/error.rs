use std::{error::Error, fmt};

use sdl2::{video::WindowBuildError, IntegerOrSdlError};

#[derive(Debug)]
pub enum EngineBuildError {
    Canvas(IntegerOrSdlError),
    Window(WindowBuildError),
    Sdl(String),
}

impl From<String> for EngineBuildError {
    fn from(e: String) -> Self {
        Self::Sdl(e)
    }
}

impl fmt::Display for EngineBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canvas(e) => write!(f, "Error constructing canvas: {}", e),
            Self::Window(e) => write!(f, "Error constructing window: {}", e),
            Self::Sdl(s) => write!(f, "SDL error: {}", s),
        }
    }
}

impl Error for EngineBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Canvas(e) => Some(e),
            Self::Window(e) => Some(e),
            Self::Sdl(_) => None,
        }
    }
}
