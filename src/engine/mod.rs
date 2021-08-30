use crate::{
    Builder,
    input::{InputState, KeyboardState, MouseState},
    Fullscreen,
};

mod error;
pub use error::EngineBuildError;

pub(crate) struct Engine {
    pub sdl: sdl2::Sdl,
    #[allow(dead_code)]
    video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub events: sdl2::EventPump,
    pub input: InputState,
    #[cfg(feature = "gfx")]
    pub anti_alias: bool,
}

impl Engine {
    pub fn new(
        builder: Builder,
    ) -> Result<Self, EngineBuildError> {
        use EngineBuildError as E;

        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let mut canvas = video.window(builder.title, builder.width, builder.height);

        match builder.fullscreen {
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
            .map_err(|e| E::WindowBuildError(e))?
            .into_canvas()
            .accelerated();

        if builder.present_vsync {
            canvas = canvas.present_vsync();
        }

        let mut canvas = canvas.build().map_err(|e| E::CanvasBuildError(e))?;
        canvas.set_scale(builder.scale.0, builder.scale.1)?;
        let texture_creator = canvas.texture_creator();

        let events = sdl.event_pump()?;
        let input = InputState {
            keyboard: KeyboardState::new(events.keyboard_state().scancodes()),
            mouse: MouseState::new(events.mouse_state()),
        };

        Ok(Self {
            sdl,
            video,
            canvas,
            texture_creator,
            events,
            input,
            #[cfg(feature = "gfx")]
            anti_alias: builder.anti_alias,
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
