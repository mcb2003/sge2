use crate::{
    input::{InputState, KeyboardState, MouseState},
    Fullscreen,
};

pub struct Engine {
    pub sdl: sdl2::Sdl,
    #[allow(dead_code)]
    video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub events: sdl2::EventPump,
    pub input: InputState,
}

impl Engine {
    pub fn new(
        title: &str,
        width: u32,
        height: u32,
        present_vsync: bool,
        fullscreen: Fullscreen,
    ) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let mut canvas = video.window(title, width, height);

        match fullscreen {
            Fullscreen::Off => {} // Do nothing
            Fullscreen::On => {
                canvas.fullscreen();
            }
            Fullscreen::Desktop => {
                canvas.fullscreen_desktop();
            }
        }

        let mut canvas = canvas
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
        let input = InputState {
            keyboard: KeyboardState::new(events.keyboard_state().scancodes()),
            mouse: MouseState::new(events.mouse_state()),
        };

        Ok(Self {
            sdl,
            video,
            canvas,
            events,
            input,
        })
    }

    pub fn update(&mut self) {
        self.canvas.present();
        self.input
            .keyboard
            .update(self.events.keyboard_state().scancodes());
        self.input.mouse.update(self.events.mouse_state());
    }
}
