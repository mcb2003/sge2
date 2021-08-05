#[cfg(feature = "gfx")]
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::{BlendMode, Texture};

use crate::{
    input::{Button, MouseButton, Scancode},
    Color, Point, Rect, ENGINE, NOT_INIT,
};

pub fn blend_mode() -> BlendMode {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.canvas.blend_mode()
    })
}

pub fn clear() {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.clear()
    })
}

pub fn clip_rect() -> Option<Rect> {
    ENGINE.with(|e| {
        let engine = e.borrow_mut();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.canvas.clip_rect()
    })
}

#[cfg(feature = "gfx")]
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

pub fn draw_color() -> Color {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.canvas.draw_color()
    })
}

pub fn draw_line<P1, P2>(start: P1, end: P2) -> Result<(), String>
where
    P1: Into<Point>,
    P2: Into<Point>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.draw_line(start, end)
    })
}

pub fn draw_lines<'a, P: Into<&'a [Point]>>(points: P) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.draw_lines(points)
    })
}

pub fn draw_point<P: Into<Point>>(point: P) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.draw_point(point)
    })
}

pub fn draw_points<'a, P: Into<&'a [Point]>>(points: P) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.draw_points(points)
    })
}

pub fn draw_rect<R: Into<Rect>>(rect: R) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.draw_rect(rect.into())
    })
}

pub fn draw_rects<'a, R: Into<&'a [Rect]>>(rects: R) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.draw_rects(rects.into())
    })
}

pub fn draw_texture<R1, R2>(texture: &Texture<'_>, src: R1, dst: R2) -> Result<(), String>
where
    R1: Into<Option<Rect>>,
    R2: Into<Option<Rect>>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.copy(texture, src, dst)
    })
}

pub fn draw_texture_ex<R1, R2, P>(
    texture: &Texture<'_>,
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
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
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

#[cfg(feature = "gfx")]
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

pub fn fill_rect<R: Into<Rect>>(rect: R) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.fill_rect(rect.into())
    })
}

pub fn fill_rects<'a, R: Into<&'a [Rect]>>(rects: R) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.fill_rects(rects.into())
    })
}

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

pub fn set_blend_mode(blend: BlendMode) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_blend_mode(blend)
    })
}

pub fn set_clip_rect<R: Into<Option<Rect>>>(rect: R) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_clip_rect(rect)
    })
}

pub fn set_draw_color<C: Into<Color>>(color: C) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color)
    })
}

pub fn set_viewport<R: Into<Option<Rect>>>(rect: R) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_viewport(rect)
    })
}

pub fn viewport() -> Rect {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.canvas.viewport()
    })
}
