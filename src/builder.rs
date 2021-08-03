use std::error::Error;

use sdl2::event::Event;

use crate::{Application, ENGINE, Engine};

pub struct Builder<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    present_vsync: bool,
}

impl<'a> Builder<'a> {
    pub fn new(title: &'a str, width: u32, height: u32) -> Self {
        Self { title, width, height, present_vsync: true }
    }

    pub fn present_vsync(mut self, val: bool) -> Self {
        self.present_vsync = val;
        self
    }

    pub fn start<A: Application>(self, app: &mut A) -> Result<(), Box<dyn Error>> {
        // Destructure `self`
        let Self { title, width, height, present_vsync} = self;

    ENGINE.with(|e| {
        use crate::fps::FpsCounter;

        let mut fps_counter: FpsCounter;
        {
            let mut engine = e
                .try_borrow_mut()
                .expect("An engine is already running in this thread");
            let new = Engine::new(title, width, height, present_vsync)?;
            fps_counter = FpsCounter::new(new.sdl.timer()?);
            *engine = Some(new);
        }
        if app.on_create()? {
            loop {
                let elapsed_time = fps_counter.update(true);
                if fps_counter.time_acc() >= 1.0 {
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
                engine.canvas.present();

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
