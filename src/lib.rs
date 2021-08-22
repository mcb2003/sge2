mod builder;
pub use builder::Builder;
mod draw;
pub use draw::*;
mod engine;
use engine::Engine;
mod fps;
mod fullscreen;
pub use fullscreen::Fullscreen;
#[cfg(feature = "gfx")]
mod gfx;
#[cfg(feature = "gfx")]
pub use gfx::*;
pub mod input;
pub use input::functions::*;
mod surface;
pub use surface::*;
mod texture;
pub use texture::*;

use std::cell::RefCell;

pub use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use once_cell::unsync::OnceCell;

thread_local! {
        static ENGINE: OnceCell<RefCell<Engine>> = OnceCell::new();
}

/// Panic message
const NOT_INIT: &str = "No engine is initialised in this thread";

pub type ApplicationResult = Result<bool, Box<dyn std::error::Error>>;

pub trait Application {
    fn on_create(&mut self) -> ApplicationResult {
        Ok(true)
    }

    fn on_update(&mut self, _elapsed_time: f64) -> ApplicationResult {
        Ok(true)
    }
}

impl<F> Application for F
where
    F: FnMut(f64) -> ApplicationResult,
{
    fn on_update(&mut self, elapsed_time: f64) -> ApplicationResult {
        self(elapsed_time)
    }
}

pub mod prelude {
    pub use crate::{
        input::{MouseButton, Scancode},
        Color, Point, Rect, Texture,
    };
}
