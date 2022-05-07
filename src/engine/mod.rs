use crate::{
    input::{InputState, KeyboardState, MouseState},
    Builder, Fullscreen,
};

mod error;
pub use error::EngineBuildError;

pub struct Engine {
    pub(crate) sdl: sdl2::Sdl,
    pub(crate) video: sdl2::VideoSubsystem,
    pub(crate) canvas: sdl2::render::WindowCanvas,
    pub(crate) texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub(crate) events: sdl2::EventPump,
    pub(crate) input: InputState,
    #[cfg(feature = "gfx")]
    pub(crate) anti_alias: bool,
}

impl Engine {
    pub(crate) fn new(builder: Builder) -> Result<Self, EngineBuildError> {
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
            .map_err(E::Window)?
            .into_canvas()
            .accelerated();

        if builder.present_vsync {
            canvas = canvas.present_vsync();
        }

        let mut canvas = canvas.build().map_err(E::Canvas)?;
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

    /// Obtain an immutable reference to this engine's [SDL context][sdl2::Sdl].
    ///
    /// This allows you to easily extend SGE by giving you access to the underlying sdl types.
    #[inline]
    pub fn sdl(&self) -> &sdl2::Sdl {
        &self.sdl
    }

    /// Obtain an immutable reference to this engine's [`sdl2::VideoSubsystem`].
    ///
    /// This allows you to easily extend SGE by giving you access to the underlying sdl types.
    #[inline]
    pub fn video(&self) -> &sdl2::VideoSubsystem {
        &self.video
    }

    pub fn update(&mut self) {
        self.canvas.present();
        self.input
            .keyboard
            .update(self.events.keyboard_state().scancodes());
        self.input.mouse.update(self.events.mouse_state());
    }
}
