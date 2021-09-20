use sdl2::render::BlendMode;

use crate::{Color, Point, Rect, ENGINE, NOT_INIT};

/// Get the current blend mode.
pub fn blend_mode() -> BlendMode {
    ENGINE.with(|e| {
        let engine = e.get().expect(NOT_INIT).borrow();
        engine.canvas.blend_mode()
    })
}

/// Clear the canvas to the specified color.
pub fn clear<C: Into<Color>>(color: C) {
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.clear()
    })
}

/// Get the clipping rectangle currently in use (if any).
pub fn clip_rect() -> Option<Rect> {
    ENGINE.with(|e| {
        let engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.clip_rect()
    })
}

/// Draw a straight line between `start` and `end`, with the specified color.
#[cfg(not(feature = "gfx"))]
pub fn draw_line<P1, P2, C>(start: P1, end: P2, color: C) -> Result<(), String>
where
    P1: Into<Point>,
    P2: Into<Point>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_line(start, end)
    })
}

// Draw a series of lines in the specified color.
pub fn draw_lines<'a, P, C>(points: P, color: C) -> Result<(), String>
where
    P: Into<&'a [Point]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_lines(points)
    })
}

/// Draw a single pixel at `point` with the specified color.
pub fn draw_point<P, C>(point: P, color: C) -> Result<(), String>
where
    P: Into<Point>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_point(point)
    })
}

/// Draw a series of points in the specified color.
pub fn draw_points<'a, P, C>(points: P, color: C) -> Result<(), String>
where
    P: Into<&'a [Point]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_points(points)
    })
}

/// Draw a rectangle outline denoted by `rect` in the specified color.
pub fn draw_rect<R, C>(rect: R, color: C) -> Result<(), String>
where
    R: Into<Rect>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_rect(rect.into())
    })
}

/// Draw a series of rectangle outlines in the specified color.
pub fn draw_rects<'a, R, C>(rects: R, color: C) -> Result<(), String>
where
    R: Into<&'a [Rect]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.draw_rects(rects.into())
    })
}

/// Draw a filled rectangle denoted by `rect` in the specified color.
pub fn fill_rect<R, C>(rect: R, color: C) -> Result<(), String>
where
    R: Into<Rect>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.fill_rect(rect.into())
    })
}

/// Draw a series of filled rectangles in the specified color.
pub fn fill_rects<'a, R, C>(rects: R, color: C) -> Result<(), String>
where
    R: Into<&'a [Rect]>,
    C: Into<Color>,
{
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_draw_color(color);
        engine.canvas.fill_rects(rects.into())
    })
}

/// Set the blend mode for alpha blending.
pub fn set_blend_mode(blend: BlendMode) {
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_blend_mode(blend)
    })
}

/// Set or clear the current clipping rectangle.
pub fn set_clip_rect<R: Into<Option<Rect>>>(rect: R) {
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_clip_rect(rect)
    })
}

/// Set or clear the current viewport. Any draw calls will be confined to this viewport until it is reset.
pub fn set_viewport<R: Into<Option<Rect>>>(rect: R) {
    ENGINE.with(|e| {
        let mut engine = e.get().expect(NOT_INIT).borrow_mut();
        engine.canvas.set_viewport(rect)
    })
}

/// Get the current viewport (if any).
pub fn viewport() -> Rect {
    ENGINE.with(|e| {
        let engine = e.get().expect(NOT_INIT).borrow();
        engine.canvas.viewport()
    })
}
