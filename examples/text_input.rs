use std::error::Error;

use sge::{prelude::*, Event};

const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;

struct App {
    text: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
}

impl sge::Application for App {
    fn on_update(&mut self, elapsed_time: f64) -> sge::ApplicationResult {
        if sge::key(Scancode::Backspace).pressed {
            self.text.pop();
        }
        if sge::key(Scancode::Return).pressed {
            self.text.push('\n');
        }
        sge::clear(Color::BLACK);
        for (i, line) in self.text.lines().enumerate() {
            // Note: this doesn't wrap lines, they'll just be cut off
            sge::draw_string((0, 8 * i as i32), line, Color::WHITE);
        }
        Ok(true)
    }

    fn on_event(&mut self, event: &Event) -> sge::ApplicationResult {
        match event {
            Event::TextInput { ref text, .. } => {
                self.text.push_str(text);
                Ok(true) // We handled this event
            }
            _ => Ok(false), // Unhandled event
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    sge::Builder::new("Text Input", SCREEN_WIDTH, SCREEN_HEIGHT).start(&mut app)
}
