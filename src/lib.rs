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

    fn on_update(&mut self) -> ApplicationResult {
        Ok(true)
    }
}

struct Engine {
    #[allow(dead_code)]
    sdl: sdl2::Sdl,
    #[allow(dead_code)]
    video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub events: sdl2::EventPump,
}

impl Engine {
    fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let canvas = video
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;
        let events = sdl.event_pump()?;
        Ok(Self { sdl, video, canvas, events })
    }
}

pub fn start<A: Application>(app: &mut A, title: &str, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    ENGINE.with(|e| {
        {
        let mut engine = e.try_borrow_mut().expect("An engine is already running in this thread");
        *engine = Some(Engine::new(title, width, height)?);
    }
        if app.on_create()? {
            while app.on_update()? {
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
