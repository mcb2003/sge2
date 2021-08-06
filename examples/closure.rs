//! An implementation of the color_cycle example using a closure instead of a struct that
//! implements sge::Application.

use std::error::Error;

use sge::prelude::*;

const CYCLE_SPEED: f32 = 130.0;
const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;

fn main() -> Result<(), Box<dyn Error>> {
    let mut col = 0.0;
    let mut flipper = false;

    sge::Builder::new("Color Cycle", SCREEN_WIDTH, SCREEN_HEIGHT).start(&mut |elapsed_time| {
        // If we're at the bounds for a colour value, change direction
        if col <= 0.0 || col >= 255.0 {
            flipper = !flipper;
        }
        // Fill the screen with the current colour
        sge::clear(Color::RGB(col as u8, 0, 255 - col as u8));
        // Change the colour
        if !flipper {
            col -= CYCLE_SPEED * elapsed_time as f32;
        } else {
            col += CYCLE_SPEED * elapsed_time as f32;
        }
        Ok(true)
    })
}
