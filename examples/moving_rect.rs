use std::error::Error;

use sge::prelude::*;

const MOVEMENT_SPEED: f64 = 200.0;
const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;
const RECT_SIZE: u32 = 100;

struct App {
    x: f64,
    y: f64,
}

impl App {
    pub fn new() -> Self {
        Self { x: 10.0, y: 10.0 }
    }
}

impl sge::Application for App {
    fn on_update(&mut self, elapsed_time: f64) -> sge::ApplicationResult {
        // Move the rectangle with the keyboard
        if sge::key(Scancode::Up).held {
            self.y = (self.y - MOVEMENT_SPEED * elapsed_time).max(0.0);
        } else if sge::key(Scancode::Down).held {
            self.y =
                (self.y + MOVEMENT_SPEED * elapsed_time).min((SCREEN_HEIGHT - RECT_SIZE) as f64);
        }
        if sge::key(Scancode::Left).held {
            self.x = (self.x - MOVEMENT_SPEED * elapsed_time).max(0.0);
        } else if sge::key(Scancode::Right).held {
            self.x =
                (self.x + MOVEMENT_SPEED * elapsed_time).min((SCREEN_WIDTH - RECT_SIZE) as f64);
        }
        // Move the rectangle with the mouse
        if sge::mouse_button(MouseButton::Left).held {
            let pos = sge::mouse_pos();
            self.x = pos.x() as f64;
            self.y = pos.y() as f64;
        }
        // Fill the screen
        sge::set_draw_color(Color::BLACK);
        sge::clear();
        sge::set_draw_color(Color::GRAY);
        sge::fill_rect(Rect::new(
            self.x as i32,
            self.y as i32,
            RECT_SIZE,
            RECT_SIZE,
        ))?;
        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    sge::Builder::new("Moving Rectangle", SCREEN_WIDTH, SCREEN_HEIGHT).start(&mut app)
}
