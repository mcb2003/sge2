use super::{Button, MouseButton, Scancode};
use crate::{Point, ENGINE, NOT_INIT};

pub fn key(key: Scancode) -> Button {
    ENGINE.with(|e| {
        let engine = e.get().expect(NOT_INIT).borrow();
        engine.input.keyboard[key]
    })
}

pub fn mouse_button(button: MouseButton) -> Button {
    ENGINE.with(|e| {
        let engine = e.get().expect(NOT_INIT).borrow();
        engine.input.mouse.buttons[button]
    })
}

pub fn mouse_pos() -> Point {
    ENGINE.with(|e| {
        let engine = e.get().expect(NOT_INIT).borrow();
        Point::new(engine.input.mouse.x, engine.input.mouse.y)
    })
}
