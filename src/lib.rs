mod fps;

use std::cell::RefCell;

use sdl2::event::Event;

thread_local! {
        static ENGINE: RefCell<Option<Engine>> = RefCell::new(None);
}

pub type ApplicationResult = Result<bool, Box<dyn std::error::Error>>;

pub trait Application {
    fn on_create(&mut self) -> ApplicationResult {
        Ok(true)
    }

    fn on_update(&mut self, _elapsed_time: f64) -> ApplicationResult {
        Ok(true)
    }
}

struct Engine {
    pub sdl: sdl2::Sdl,
    #[allow(dead_code)]
    video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub events: sdl2::EventPump,
}

impl Engine {
    fn new(title: &str, width: u32, height: u32, present_vsync: bool) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let mut canvas = video
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .accelerated();
        if present_vsync {
            canvas = canvas.present_vsync();
        }
        let canvas = canvas.build().map_err(|e| e.to_string())?;
        let events = sdl.event_pump()?;
        Ok(Self {
            sdl,
            video,
            canvas,
            events,
        })
    }
}

pub fn start<A: Application>(
    app: &mut A,
    title: &str,
    width: u32,
    height: u32,
    present_vsync: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    ENGINE.with(|e| {
        use fps::FpsCounter;

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
