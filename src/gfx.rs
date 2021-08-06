use std::convert::TryInto;

use sdl2::gfx::primitives::DrawRenderer;

use crate::{Point, ENGINE, NOT_INIT};

const CIRCLE_X_BOUNDS: &str = "Circle x coordinate out of bounds, must fit in an i16";
const CIRCLE_Y_BOUNDS: &str = "Circle y coordinate out of bounds, must fit in an i16";
const LINE_X_BOUNDS: &str = "Line x coordinate out of bounds, must fit in an i16";
const LINE_Y_BOUNDS: &str = "Line y coordinate out of bounds, must fit in an i16";

fn to_xy<P: Into<Point>>(p: P, x_bounds: &'static str, y_bounds: &'static str) -> (i16, i16) {
    let p = p.into();
    let x = p.x().try_into().expect(x_bounds);
    let y = p.y().try_into().expect(y_bounds);
    (x, y)
}

pub fn anti_aliased() -> bool {
    ENGINE.with(|e| {
        let engine = e.borrow();
        let engine = engine.as_ref().expect(NOT_INIT);
        engine.anti_alias
    })
}

pub fn draw_circle<P: Into<Point>>(center: P, radius: i16) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);

        let (x, y) = to_xy(center, CIRCLE_X_BOUNDS, CIRCLE_Y_BOUNDS);

        let func = if engine.anti_alias {
            DrawRenderer::aa_circle
        } else {
            DrawRenderer::circle
        };

        func(&engine.canvas, x, y, radius, engine.canvas.draw_color())
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

        let (start_x, start_y) = to_xy(start, LINE_X_BOUNDS, LINE_Y_BOUNDS);
        let (end_x, end_y) = to_xy(end, LINE_X_BOUNDS, LINE_Y_BOUNDS);

        let func = if engine.anti_alias {
            DrawRenderer::aa_line
        } else {
            DrawRenderer::line
        };

        func(
            &engine.canvas,
            start_x,
            start_y,
            end_x,
            end_y,
            engine.canvas.draw_color(),
        )
    })
}

pub fn fill_circle<P: Into<Point>>(center: P, radius: i16) -> Result<(), String> {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);

let (x, y) = to_xy(center, CIRCLE_X_BOUNDS, CIRCLE_Y_BOUNDS);

        engine
            .canvas
            .filled_circle(x, y, radius, engine.canvas.draw_color())
    })
}

pub fn set_anti_alias(anti_alias: bool) {
    ENGINE.with(|e| {
        let mut engine = e.borrow_mut();
        let engine = engine.as_mut().expect(NOT_INIT);
        engine.anti_alias = anti_alias;
    })
}
