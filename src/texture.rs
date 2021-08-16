/*
use std::path::Path;

use sdl2::surface::Surface;

use crate::{Texture, ENGINE, NOT_INIT};

pub fn create_texture<P: AsRef<Path>>(file_path: P) -> Result<Texture<'static>, String> {
    let surface = Surface::load_bmp(file_path)?;

    ENGINE.with(move |e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        surface.as_texture(&mut engine.texture_creator).map_err(|e| e.to_string())
    })
}
*/
