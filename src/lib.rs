use std::cell::RefCell;

thread_local! {
        static ENGINE: RefCell<Option<Engine>> = RefCell::new(None);
}

struct Engine {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
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
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self { sdl, video, canvas })
    }
}

pub fn start(title: &str, width: u32, height: u32) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.try_borrow_mut().expect("An engine is already running in this thread");
        *engine = Some(Engine::new(title, width, height)?);
        Ok(())
    })
}
