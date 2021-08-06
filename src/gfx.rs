use sdl2::gfx::primitives::DrawRenderer;

use crate::{Point, ENGINE, NOT_INIT};

pub fn draw_circle<P: Into<Point>>(center: P, radius: i16) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        let center = center.into();
        engine.canvas.circle(
            center.x() as i16,
            center.y() as i16,
            radius,
            engine.canvas.draw_color(),
        )
    })
}

pub fn fill_circle<P: Into<Point>>(center: P, radius: i16) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        let center = center.into();
        engine.canvas.filled_circle(
            center.x() as i16,
            center.y() as i16,
            radius,
            engine.canvas.draw_color(),
        )
    })
}
