use std::error::Error;

use sdl2::event::Event;

use crate::{Application, Engine, Fullscreen, ENGINE};

pub struct Builder<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    present_vsync: bool,
    show_fps: bool,
    fullscreen: Fullscreen,
    #[cfg(feature = "gfx")]
    anti_alias: bool,
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

    #[cfg(feature = "gfx")]
    pub fn anti_alias(mut self, val: bool) -> Self {
        self.anti_alias = val;
        self
    }

    pub fn start<A: Application>(self, app: &mut A) -> Result<(), Box<dyn Error>> {
        // Destructure `self`
        let Self {
            title,
            width,
            height,
            present_vsync,
            show_fps,
            fullscreen,
            #[cfg(feature = "gfx")]
            anti_alias,
        } = self;

        ENGINE.with(|e| {
            use crate::fps::FpsCounter;

            let mut fps_counter: FpsCounter;
            {
                let mut engine = e
                    .try_borrow_mut()
                    .expect("An engine is already running in this thread");
                let new = Engine::new(
                    title,
                    width,
                    height,
                    present_vsync,
                    fullscreen,
                    #[cfg(feature = "gfx")]
                    anti_alias,
                )?;
                fps_counter = FpsCounter::new(new.sdl.timer()?);
                *engine = Some(new);
            }
            if app.on_create()? {
                loop {
                    let elapsed_time = fps_counter.update(show_fps);
                    if show_fps && fps_counter.time_acc() >= 1.0 {
                        let fps = fps_counter.fps();
                        let title = format!("{} ({:.0} FPS)", title, fps.round());

                        let mut engine = e.borrow_mut();
                        let engine = engine.as_mut().unwrap();
                        // This fails silently on error
                        engine.canvas.window_mut().set_title(&title).ok();

                        fps_counter.reset_average();
                    }

                    if !app.on_update(elapsed_time)? {
                        return Ok(());
                    }

                    let mut engine = e.borrow_mut();
                    let engine = engine.as_mut().unwrap();
                    engine.update();

                    for event in engine.events.poll_iter() {
                        if let Event::Quit { .. } = event {
                            return Ok(());
                        }
                    }
                }
            }
            Ok(())
        })
    }
}
