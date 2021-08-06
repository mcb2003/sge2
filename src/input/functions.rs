use super::{Button, MouseButton, Scancode};
use crate::{Point, ENGINE, NOT_INIT};

pub fn key(key: Scancode) -> Button {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.input.keyboard[key]
    })
}

pub fn mouse_button(button: MouseButton) -> Button {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.input.mouse.buttons[button]
    })
}

pub fn mouse_pos() -> Point {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        Point::new(engine.input.mouse.x, engine.input.mouse.y)
    })
}
