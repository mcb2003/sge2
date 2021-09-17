use std::{cell::RefCell, error::Error};

use sdl2::event::Event;

use crate::{Application, Engine, Fullscreen, ENGINE, NOT_INIT};

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

    pub fn present_vsync(mut self, val: bool) -> Self {
        self.present_vsync = val;
        self
    }

    pub fn show_fps(mut self, val: bool) -> Self {
        self.show_fps = val;
        self
    }

    pub fn fullscreen(mut self, val: impl Into<Fullscreen>) -> Self {
        self.fullscreen = val.into();
        self
    }

    pub fn scale(mut self, scale_x: f32, scale_y: f32) -> Self {
        self.scale.0 = scale_x;
        self.scale.1 = scale_y;
        self
    }

    #[cfg(feature = "gfx")]
    pub fn anti_alias(mut self, val: bool) -> Self {
        self.anti_alias = val;
        self
    }

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
                            // The app didn't handle the event
                            if !app.on_event(&event)? {
                                if let Event::Quit { .. } = event {
                                    return Ok(());
                                }
                            }
                        }
                    }

                    if !app.on_update(elapsed_time)? {
                        return Ok(());
                    }
                }
            }
            Ok(())
        })
    }
}
