use std::path::Path;

use sdl2::{pixels::PixelFormatEnum, surface::Surface as SdlSurface};

use crate::{Color, Texture, ENGINE, NOT_INIT, TextureValueError};

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

pub struct Surface<'a>(pub(crate) SdlSurface<'a>);

impl Surface<'_> {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        let format = default_pixel_format();
        SdlSurface::new(width, height, format).map(|s| Self(s))
    }

    #[cfg(feature = "image")]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        use sdl2::image::LoadSurface;
        let mut surf = SdlSurface::from_file(file_path)?;
        if let Ok(new_surf) = surf.convert_format(default_pixel_format()) {
            surf = new_surf;
        }
        Ok(Self(surf))
    }

    #[cfg(not(feature = "image"))]
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        let mut surf = SdlSurface::load_bmp(file_path)?;
        if let Ok(new_surf) = surf.convert_format(default_pixel_format()) {
            surf = new_surf;
        }
        Ok(Self(surf))
    }

    pub fn size(&self) -> (u32, u32) {
        self.0.size()
    }

    pub fn mod_(&self) -> Color {
        let (r, g, b) = self.0.color_mod().rgb();
        let a = self.0.alpha_mod();
        Color::RGBA(r, g, b, a)
    }

    pub fn set_mod<C: Into<Color>>(&mut self, mod_: C) {
        let mod_ = mod_.into();
        let (_, _, _, a) = mod_.rgba();
        self.0.set_color_mod(mod_);
        self.0.set_alpha_mod(a);
    }

    pub fn as_texture(&self) -> Result<Texture, TextureValueError> {
        Texture::from_surface(self)
    }
}