use std::convert::TryInto;

use sdl2::gfx::primitives::DrawRenderer;

use crate::{with_engine, with_engine_mut, Color, Point};

const CHAR_X_BOUNDS: &str = "Character x coordinate out of bounds, must fit in an i16";
const CHAR_Y_BOUNDS: &str = "Character y coordinate out of bounds, must fit in an i16";
const CIRCLE_X_BOUNDS: &str = "Circle x coordinate out of bounds, must fit in an i16";
const CIRCLE_Y_BOUNDS: &str = "Circle y coordinate out of bounds, must fit in an i16";
const ELLIPSE_RX_BOUNDS: &str = "Ellipse horizontal radius out of bounds, must fit in an i16";
const ELLIPSE_RY_BOUNDS: &str = "Ellipse vertical radius out of bounds, must fit in an i16";
const ELLIPSE_X_BOUNDS: &str = "Ellipse x coordinate out of bounds, must fit in an i16";
const ELLIPSE_Y_BOUNDS: &str = "Ellipse y coordinate out of bounds, must fit in an i16";
const LINE_X_BOUNDS: &str = "Line x coordinate out of bounds, must fit in an i16";
const LINE_Y_BOUNDS: &str = "Line y coordinate out of bounds, must fit in an i16";
const STRING_X_BOUNDS: &str = "String x coordinate out of bounds, must fit in an i16";
const STRING_Y_BOUNDS: &str = "String y coordinate out of bounds, must fit in an i16";
const TRIANGLE_X_BOUNDS: &str = "Triangle x coordinate out of bounds, must fit in an i16";
const TRIANGLE_Y_BOUNDS: &str = "Triangle y coordinate out of bounds, must fit in an i16";

fn to_xy<P: Into<Point>>(p: P, x_bounds: &'static str, y_bounds: &'static str) -> (i16, i16) {
    let p = p.into();
    let x = p.x().try_into().expect(x_bounds);
    let y = p.y().try_into().expect(y_bounds);
    (x, y)
}

/// Returns whether shapes are currently being anti-aliased when drawn.
pub fn anti_aliased() -> bool {
    with_engine(|e| e.anti_alias)
}

/// Draw a single character at `pos` in the specified color. The built-in font (from SDL_gfx) is
/// used.
pub fn draw_char<P, C>(pos: P, character: char, color: C) -> Result<(), String>
where
    P: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (x, y) = to_xy(pos, CHAR_X_BOUNDS, CHAR_Y_BOUNDS);

        engine.canvas.character(x, y, character, color.into())
    })
}

/// Draw a circle outline with the given center, radius and in the specified color.
pub fn draw_circle<P, C>(center: P, radius: i16, color: C) -> Result<(), String>
where
    P: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (x, y) = to_xy(center, CIRCLE_X_BOUNDS, CIRCLE_Y_BOUNDS);

        let func = if engine.anti_alias {
            DrawRenderer::aa_circle
        } else {
            DrawRenderer::circle
        };

        func(&engine.canvas, x, y, radius, color.into())
    })
}

/// Draw an ellipse outline with the given center and radii, and in the specified color.
pub fn draw_ellipse<P, R, C>(center: P, radii: R, color: C) -> Result<(), String>
where
    P: Into<Point>,
    R: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (x, y) = to_xy(center, ELLIPSE_X_BOUNDS, ELLIPSE_Y_BOUNDS);
        let (rx, ry) = to_xy(radii, ELLIPSE_RX_BOUNDS, ELLIPSE_RY_BOUNDS);

        let func = if engine.anti_alias {
            DrawRenderer::aa_ellipse
        } else {
            DrawRenderer::ellipse
        };

        func(&engine.canvas, x, y, rx, ry, color.into())
    })
}

/// Draw a straight line between `start` and `end`, with the specified color.
pub fn draw_line<P1, P2, C>(start: P1, end: P2, color: C) -> Result<(), String>
where
    P1: Into<Point>,
    P2: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (start_x, start_y) = to_xy(start, LINE_X_BOUNDS, LINE_Y_BOUNDS);
        let (end_x, end_y) = to_xy(end, LINE_X_BOUNDS, LINE_Y_BOUNDS);

        let func = if engine.anti_alias {
            DrawRenderer::aa_line
        } else {
            DrawRenderer::line
        };

        func(&engine.canvas, start_x, start_y, end_x, end_y, color.into())
    })
}

/// Draw a text string at `pos` in the specified color. The built-in font (from SDL_gfx) is used.
pub fn draw_string<P, C>(pos: P, string: &str, color: C) -> Result<(), String>
where
    P: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (x, y) = to_xy(pos, STRING_X_BOUNDS, STRING_Y_BOUNDS);

        engine.canvas.string(x, y, string, color.into())
    })
}

/// Draw a triangle outline from `a` to `b` to `c` and back to `a`, in the specified color.
pub fn draw_triangle<P1, P2, P3, C>(a: P1, b: P2, c: P3, color: C) -> Result<(), String>
where
    P1: Into<Point>,
    P2: Into<Point>,
    P3: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (ax, ay) = to_xy(a, TRIANGLE_X_BOUNDS, TRIANGLE_Y_BOUNDS);
        let (bx, by) = to_xy(b, TRIANGLE_X_BOUNDS, TRIANGLE_Y_BOUNDS);
        let (cx, cy) = to_xy(c, TRIANGLE_X_BOUNDS, TRIANGLE_Y_BOUNDS);

        let func = if engine.anti_alias {
            DrawRenderer::aa_trigon
        } else {
            DrawRenderer::trigon
        };

        func(&engine.canvas, ax, ay, bx, by, cx, cy, color.into())
    })
}

/// Draw a filled circle with the given center, radius and in the specified color.
pub fn fill_circle<P, C>(center: P, radius: i16, color: C) -> Result<(), String>
where
    P: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (x, y) = to_xy(center, CIRCLE_X_BOUNDS, CIRCLE_Y_BOUNDS);

        engine.canvas.filled_circle(x, y, radius, color.into())
    })
}

/// Draw a filled ellipse with the given center and radii, and in the specified color.
pub fn fill_ellipse<P, R, C>(center: P, radii: R, color: C) -> Result<(), String>
where
    P: Into<Point>,
    R: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (x, y) = to_xy(center, ELLIPSE_X_BOUNDS, ELLIPSE_Y_BOUNDS);
        let (rx, ry) = to_xy(radii, ELLIPSE_RX_BOUNDS, ELLIPSE_RY_BOUNDS);

        engine.canvas.filled_ellipse(x, y, rx, ry, color.into())
    })
}

/// Draw a filled triangle from `a` to `b` to `c` and back to `a`, in the specified color.
pub fn fill_triangle<P1, P2, P3, C>(a: P1, b: P2, c: P3, color: C) -> Result<(), String>
where
    P1: Into<Point>,
    P2: Into<Point>,
    P3: Into<Point>,
    C: Into<Color>,
{
    with_engine_mut(|engine| {
        let (ax, ay) = to_xy(a, TRIANGLE_X_BOUNDS, TRIANGLE_Y_BOUNDS);
        let (bx, by) = to_xy(b, TRIANGLE_X_BOUNDS, TRIANGLE_Y_BOUNDS);
        let (cx, cy) = to_xy(c, TRIANGLE_X_BOUNDS, TRIANGLE_Y_BOUNDS);

        engine
            .canvas
            .filled_trigon(ax, ay, bx, by, cx, cy, color.into())
    })
}

/// Sets if shapes should be anti-aliased when drawn. This smooths the edges of shapes, but is more
/// CPU intensive.
pub fn set_anti_alias(anti_alias: bool) {
    with_engine_mut(|engine| {
        engine.anti_alias = anti_alias;
    })
}
