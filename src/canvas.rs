use sdl2::pixels::Color;

use crate::{ENGINE, NOT_INIT};

pub fn draw_color() -> Color {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine
            .as_mut()
            .expect(NOT_INIT);
        engine.canvas.draw_color()
    })
}

pub fn set_draw_color<C: Into<Color>>(color: C) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine
            .as_mut()
            .expect(NOT_INIT);
        engine.canvas.set_draw_color(color)
    })
}

pub fn clear() {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine
            .as_mut()
            .expect(NOT_INIT);
        engine.canvas.clear()
    })
}
