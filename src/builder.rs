use std::{cell::RefCell, error::Error};

use sdl2::event::Event;

use crate::{Application, Engine, Fullscreen, ENGINE, NOT_INIT};

/// A type used to construct and start an engine from an [`Application`][crate::Application].
#[must_use = "Builders do nothing unless an Application is started with them"]
pub struct Builder<'a> {
    pub(crate) title: &'a str,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) present_vsync: bool,
    pub(crate) show_fps: bool,
    pub(crate) fullscreen: Fullscreen,
    pub(crate) scale: (f32, f32),
    #[cfg(feature = "gfx")]
    pub(crate) anti_alias: bool,
}

impl<'a> Builder<'a> {
    /// Create a new `Builder` with the required parameters.
    pub fn new(title: &'a str, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
            present_vsync: true,
            show_fps: true,
            fullscreen: Fullscreen::Off,
            scale: (1.0, 1.0),
            #[cfg(feature = "gfx")]
            anti_alias: false,
        }
    }

    // Set whether vsync is used. If set to true, the update-rate of the application will be
    // limited to the refresh rate of the user's display.
    pub fn present_vsync(mut self, val: bool) -> Self {
        self.present_vsync = val;
        self
    }

    /// Set whether the FPS is calculated and shown in the Window's title-bar. Regardless of this
    /// setting, the elapsed time between frames will still be calculated and passed to
    /// [`Application::on_update`][crate::Application::on_update].
    pub fn show_fps(mut self, val: bool) -> Self {
        self.show_fps = val;
        self
    }

    /// Set whether the application should start in fullscreen.
    ///
    /// You can either pass a bool, indicating whether you want fullscreen or not, or a variant of
    /// [`Fullscreen`][crate::Fullscreen] if, for example, you want to use desktop mode.
    pub fn fullscreen(mut self, val: impl Into<Fullscreen>) -> Self {
        self.fullscreen = val.into();
        self
    }

    /// Set the engine scale. All drawn objects will be scaled by this amount in each axis.
    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> Self {
        self.scale.0 = scale_x;
        self.scale.1 = scale_y;
        self
    }

    /// Set whether drawn shapes are anti-aliased. Doing so smooths the edges of objects, but is
    /// more CPU intensive.
    #[cfg(feature = "gfx")]
    pub fn anti_alias(mut self, val: bool) -> Self {
        self.anti_alias = val;
        self
    }

    // Start the engine, using the settings from this `Builder`, and running the passed
    // [`Application`][crate::Application].
    pub fn start<A: Application>(self, app: &mut A) -> Result<(), Box<dyn Error>> {
        let title = self.title;
        let show_fps = self.show_fps;

        ENGINE.with(|e| {
            use crate::fps::FpsCounter;

            let mut fps_counter: FpsCounter;
            {
                let new = Engine::new(self)?;
                fps_counter = FpsCounter::new(new.sdl.timer()?);
                if e.set(RefCell::new(new)).is_err() {
                    panic!("An engine was already started in this thread");
                }
            }
            if app.on_create()? {
                loop {
                    let elapsed_time = fps_counter.update(show_fps);
                    {
                        let mut engine = e.get().expect(NOT_INIT).borrow_mut();

                        if show_fps && fps_counter.time_acc() >= 1.0 {
                            let fps = fps_counter.fps();
                            let title = format!("{} ({:.0} FPS)", title, fps.round());

                            // This fails silently on error
                            engine.canvas.window_mut().set_title(&title).ok();

                            fps_counter.reset_average();
                        }

                        engine.update();

                        for event in engine.events.poll_iter() {
                            if !app.on_event(&event)? {
                                // The app didn't handle the event
                                if let Event::Quit { .. } = event {
                                    return Ok(()); // Quit
                                }
                            }
                        }
                    }

                    if !app.on_update(elapsed_time)? {
                        return Ok(()); // Application wanted to quit
                    }
                }
            }
            Ok(())
        })
    }
}
