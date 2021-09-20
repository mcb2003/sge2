use std::path::Path;

use sdl2::render::Texture as SdlTexture;

use crate::{Color, Point, Rect, Surface, TextureValueError, ENGINE, NOT_INIT};

#[cfg(not(feature = "image"))]
mod error;
#[cfg(not(feature = "image"))]
use error::LoadTextureError;

/// A (potentially hardware accelerated) renderable texture.
pub struct Texture(Option<SdlTexture>);

impl Texture {
    /// Load a T`exture` from an existing [`Surface`][crate::Surface].
    pub fn from_surface(surface: &Surface) -> Result<Self, TextureValueError> {
        ENGINE.with(|e| {
            let engine = e.get().expect(NOT_INIT).borrow();
            surface
                .0
                .as_texture(&engine.texture_creator)
                .map(|t| Self(Some(t)))
        })
    }

    /// Load a `Texture` from an image file.
    #[cfg(feature = "image")]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        use sdl2::image::LoadTexture;

        ENGINE.with(move |e| {
            let engine = e.get().expect(NOT_INIT).borrow();
            engine
                .texture_creator
                .load_texture(file_path)
                .map(|t| Self(Some(t)))
        })
    }

    /// Load a `Texture` from a BMP file.
    #[cfg(not(feature = "image"))]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, LoadTextureError> {
        use sdl2::surface::Surface;
        use LoadTextureError as E;
        let surface = Surface::load_bmp(file_path).map_err(|e| E::LoadError(e))?;

        ENGINE.with(move |e| {
            let mut engine = e.get().expect(NOT_INIT).borrow_mut();
            surface
                .as_texture(&mut engine.texture_creator)
                .map(|t| Self(Some(t)))
                .map_err(|e| E::ValueError(e))
        })
    }

    /// Draw the portion of a `Texture` given by `src` to `dst` on the canvas. If `src` is `None`,
    /// draws the entire `Texture`. If `dst` is `None`, draws the `Texture` to fill the entire
    /// canvas.
    pub fn draw<R1, R2>(&self, src: R1, dst: R2) -> Result<(), String>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
    {
        let texture = self.0.as_ref().unwrap();

        ENGINE.with(|e| {
            let mut engine = e.get().expect(NOT_INIT).borrow_mut();
            engine.canvas.copy(texture, src, dst)
        })
    }

    /// Draw the portion of a `Texture` given by `src` to `dst` on the canvas, rotated around
    /// `center` by `angle`, and optionally flipped horizontally or vertically. If `src` is `None`,
    /// draws the entire `Texture`. If `dst` is `None`, draws the `Texture` to fill the entire
    /// canvas.
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
        let texture = self.0.as_ref().unwrap();

        ENGINE.with(|e| {
            let mut engine = e.get().expect(NOT_INIT).borrow_mut();
            engine.canvas.copy_ex(
                texture,
                src,
                dst,
                angle,
                center,
                flip_horizontal,
                flip_vertical,
            )
        })
    }

    /// Returns the size of the `Texture` in pixels.
    pub fn size(&self) -> Point {
        let query = self.0.as_ref().unwrap().query();
        Point::new(query.width as i32, query.height as i32)
    }

    /// Returns a [`Color`][crate::Color] representing the color and alpha mod of the `Texture`.
    pub fn mod_(&self) -> Color {
        let texture = self.0.as_ref().unwrap();
        let (r, g, b) = texture.color_mod();
        let a = texture.alpha_mod();
        Color::RGBA(r, g, b, a)
    }

    /// Sets the color and alpha mod of the `Texture`.
    pub fn set_mod<C: Into<Color>>(&mut self, mod_: C) {
        let (r, g, b, a) = mod_.into().rgba();
        let texture = self.0.as_mut().unwrap();
        texture.set_color_mod(r, g, b);
        texture.set_alpha_mod(a);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        ENGINE.with(|e| {
            if e.get().is_some() {
                // Safety: TextureCreator definitely exists and is the same object, so this is safe.
                unsafe {
                    self.0.take().unwrap().destroy();
                }
            }
        })
    }
}

    /// Draw the portion of a `Texture` given by `src` to `dst` on the canvas. If `src` is `None`,
    /// draws the entire `Texture`. If `dst` is `None`, draws the `Texture` to fill the entire
    /// canvas.
pub fn draw_texture<R1, R2>(texture: &Texture, src: R1, dst: R2) -> Result<(), String>
where
    R1: Into<Option<Rect>>,
    R2: Into<Option<Rect>>,
{
    texture.draw(src, dst)
}

    /// Draw the portion of a `Texture` given by `src` to `dst` on the canvas, rotated around
    /// `center` by `angle`, and optionally flipped horizontally or vertically. If `src` is `None`,
    /// draws the entire `Texture`. If `dst` is `None`, draws the `Texture` to fill the entire
    /// canvas.
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
    texture.draw_ex(src, dst, angle, center, flip_horizontal, flip_vertical)
}
