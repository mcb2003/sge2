pub struct Engine {
    pub sdl: sdl2::Sdl,
    #[allow(dead_code)]
    video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub events: sdl2::EventPump,
}

impl Engine {
    pub fn new(title: &str, width: u32, height: u32, present_vsync: bool) -> Result<Self, String> {
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


