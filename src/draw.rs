use sdl2::render::{BlendMode, Texture};

use crate::{Color, Point, Rect, ENGINE, NOT_INIT};

pub fn blend_mode() -> BlendMode {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.canvas.blend_mode()
    })
}

pub fn clear<C: Into<Color>>(color: C) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
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

#[cfg(not(feature = "gfx"))]
pub fn draw_line<P1, P2, C>(start: P1, end: P2, color: C) -> Result<(), String>
where
    P1: Into<Point>,
    P2: Into<Point>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_line(start, end)
    })
}

pub fn draw_lines<'a, P, C>(points: P, color: C) -> Result<(), String>
where
    P: Into<&'a [Point]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_lines(points)
    })
}

pub fn draw_point<P, C>(point: P, color: C) -> Result<(), String>
where
    P: Into<Point>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_point(point)
    })
}

pub fn draw_points<'a, P, C>(points: P, color: C) -> Result<(), String>
where
    P: Into<&'a [Point]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_points(points)
    })
}

pub fn draw_rect<R, C>(rect: R, color: C) -> Result<(), String>
where
    R: Into<Rect>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_rect(rect.into())
    })
}

pub fn draw_rects<'a, R, C>(rects: R, color: C) -> Result<(), String>
where
    R: Into<&'a [Rect]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_rects(rects.into())
    })
}

pub fn draw_texture<R1, R2>(texture: &Texture, src: R1, dst: R2) -> Result<(), String>
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

pub fn fill_rect<R, C>(rect: R, color: C) -> Result<(), String>
where
    R: Into<Rect>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.fill_rect(rect.into())
    })
}

pub fn fill_rects<'a, R, C>(rects: R, color: C) -> Result<(), String>
where
    R: Into<&'a [Rect]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.canvas.set_draw_color(color);
        engine.canvas.fill_rects(rects.into())
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
