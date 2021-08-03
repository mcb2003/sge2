use std::error::Error;

use sge::prelude::*;

const CYCLE_SPEED: f32 = 130.0;
const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;

struct App {
    col: f32,
    flipper: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            col: 255.0,
            flipper: true,
        }
    }
}

impl sge::Application for App {
    fn on_update(&mut self, elapsed_time: f64) -> sge::ApplicationResult {
        // If we're at the bounds for a colour value, change direction
        if self.col <= 0.0 || self.col >= 255.0 {
            self.flipper = !self.flipper;
        }
        // Fill the screen with the current colour
        sge::set_draw_color(Color::RGB(self.col as u8, 0, 255 - self.col as u8));
        sge::clear();
        // Change the colour
        if !self.flipper {
            self.col -= CYCLE_SPEED * elapsed_time as f32;
        } else {
            self.col += CYCLE_SPEED * elapsed_time as f32;
        }
        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    sge::Builder::new("Color Cycle", SCREEN_WIDTH, SCREEN_HEIGHT).start(&mut app)
}
