use std::path::Path;

use sdl2::render::Texture as SdlTexture;

use crate::{Color, Point, Rect, ENGINE, NOT_INIT};

pub struct Texture(SdlTexture);

impl Texture {
    #[cfg(feature = "image")]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        use sdl2::image::LoadTexture;

        ENGINE.with(move |e| {
            let engine = e.get().expect(NOT_INIT).borrow();
            engine
                .texture_creator
                .load_texture(file_path)
                .map(|t| Self(t))
        })
    }

    #[cfg(not(feature = "image"))]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        use sdl2::surface::Surface;
        let surface = Surface::load_bmp(file_path)?;

        ENGINE.with(move |e| {
            let mut engine = e.get().expect(NOT_INIT).borrow_mut();
            surface
                .as_texture(&mut engine.texture_creator)
                .map(|t| Self(t))
                .map_err(|e| e.to_string())
        })
    }

    pub fn draw<R1, R2>(&self, src: R1, dst: R2) -> Result<(), String>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
    {
        draw_texture(self, src, dst)
    }

    pub fn draw_ex<R1, R2, P>(
        &self,
        src: R1,
        dst: R2,
        angle: f64,
        center: P,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> Result<(), String>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
        P: Into<Option<Point>>,
    {
        draw_texture_ex(
            self,
            src,
            dst,
            angle,
            center,
            flip_horizontal,
            flip_vertical,
        )
    }

    pub fn size(&self) -> Point {
        let query = self.0.query();
        Point::new(query.width as i32, query.height as i32)
    }

    pub fn mod_(&self) -> Color {
        let (r, g, b) = self.0.color_mod();
        let a = self.0.alpha_mod();
        Color::RGBA(r, g, b, a)
    }

    pub fn set_mod<C: Into<Color>>(&mut self, mod_: C) {
        let (r, g, b, a) = mod_.into().rgba();
        self.0.set_color_mod(r, g, b);
        self.0.set_alpha_mod(a);
    }
}

pub fn draw_texture<R1, R2>(texture: &Texture, src: R1, dst: R2) -> Result<(), String>
where
    R1: Into<Option<Rect>>,
    R2: Into<Option<Rect>>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.copy(&texture.0, src, dst)
    })
}

pub fn draw_texture_ex<R1, R2, P>(
    texture: &Texture,
    src: R1,
    dst: R2,
    angle: f64,
    center: P,
    flip_horizontal: bool,
    flip_vertical: bool,
) -> Result<(), String>
where
    R1: Into<Option<Rect>>,
    R2: Into<Option<Rect>>,
    P: Into<Option<Point>>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.copy_ex(
            &texture.0,
            src,
            dst,
            angle,
            center,
            flip_horizontal,
            flip_vertical,
        )
    })
}
