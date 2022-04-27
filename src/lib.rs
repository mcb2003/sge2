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
pub mod panic;
mod surface;
pub use surface::*;
mod texture;
pub use texture::*;

use std::cell::RefCell;

pub use sdl2::{
    self,
    event::{Event, EventType},
    pixels::Color,
    rect::{Point, Rect},
    render::TextureValueError,
};

use once_cell::unsync::OnceCell;

thread_local! {
        static ENGINE: OnceCell<RefCell<Engine>> = OnceCell::new();
}

/// Panic message
const NOT_INIT: &str = "No engine is initialised in this thread";

/// The return type of most functions in the [`Application`] trait.
///
/// * Returning `Ok(true` continues the application.
/// * Returning `Ok(false)` aborts the application.
/// * Returning `Err(e)` aborts the application, and propogates the error back up to the
///   [`Builder::start`][crate::Builder::start] function.
pub type ApplicationResult = Result<bool, Box<dyn std::error::Error>>;

/// An application using this framework.
pub trait Application {
    /// Called once when the engine is started. You can perform one-time initialisation in this
    /// function, such as loading assets or drawing static parts of the screen.
    fn on_create(&mut self) -> ApplicationResult {
        Ok(true)
    }

    /// Called once per frame.
    ///
    /// `elapsed_time` is the duration (in seconds) since the last frame. This can be used to keep
    ///   time-sensative routines, such as animation, running at a constant speed.
    fn on_update(&mut self, _elapsed_time: f64) -> ApplicationResult {
        Ok(true)
    }

    /// Called for each [`Event`] every frame.
    ///
    /// You shouldn't call any other SGE functions from this handler. Doing so will panic because
    /// the engine is already borrowed. This can be used to process any type of SDL event, such as
    /// text input.
    fn on_event(&mut self, _event: &Event) -> ApplicationResult {
        Ok(false)
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

/// Commonly used types.
pub mod prelude {
    pub use crate::{
        input::{MouseButton, Scancode},
        Color, Point, Rect, Texture,
    };
}
