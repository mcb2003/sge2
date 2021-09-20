use std::path::Path;

use sdl2::{pixels::PixelFormatEnum, surface::Surface as SdlSurface};

use crate::{Color, Texture, TextureValueError, ENGINE, NOT_INIT};

/// A helper to get the window's default pixel format.
fn default_pixel_format() -> PixelFormatEnum {
    ENGINE.with(|e| {
        e.get()
            .expect(NOT_INIT)
            .borrow()
            .canvas
            .default_pixel_format()
    })
}

/// A 2D array of pixels in a given format.
pub struct Surface<'a>(pub(crate) SdlSurface<'a>);

impl Surface<'_> {
    /// Create a new surface with the given `width` and `height`, and the same pixel format as the
    /// canvas, allowing for efficient blit operations.
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let format = default_pixel_format();
        SdlSurface::new(width, height, format).map(Self)
    }

    /// Load a `Surface` from an image file. The image data will automatically be converted to
    /// match the canvas' pixel format if possible.
    #[cfg(feature = "image")]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        use sdl2::image::LoadSurface;
        let mut surf = SdlSurface::from_file(file_path)?;
        if let Ok(new_surf) = surf.convert_format(default_pixel_format()) {
            surf = new_surf;
        }
        Ok(Self(surf))
    }

    /// Load a `Surface` from a BMP file. The image data will automatically be converted to
    /// match the canvas' pixel format if possible.
    #[cfg(not(feature = "image"))]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        let mut surf = SdlSurface::load_bmp(file_path)?;
        if let Ok(new_surf) = surf.convert_format(default_pixel_format()) {
            surf = new_surf;
        }
        Ok(Self(surf))
    }

    /// Returns the size, in pixels, of the `Surface`.
    pub fn size(&self) -> (u32, u32) {
        self.0.size()
    }

    /// Returns a [`Color`][crate::Color] representing the color and alpha mod of the `Surface`.
    pub fn mod_(&self) -> Color {
        let (r, g, b) = self.0.color_mod().rgb();
        let a = self.0.alpha_mod();
        Color::RGBA(r, g, b, a)
    }

    /// Sets the color and alpha mod of the `Surface`.
    pub fn set_mod<C: Into<Color>>(&mut self, mod_: C) {
        let mod_ = mod_.into();
        let (_, _, _, a) = mod_.rgba();
        self.0.set_color_mod(mod_);
        self.0.set_alpha_mod(a);
    }

    /// Loads this surface as a new [`Texture`][crate::Texture], which may be hardware accelerated
    /// and thus faster to render.
    pub fn as_texture(&self) -> Result<Texture, TextureValueError> {
        Texture::from_surface(self)
    }
}
