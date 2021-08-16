use std::error::Error;

use sge::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    sge::Builder::new("Hello World", 640, 480).start(&mut |_elapsed_time| {
        sge::clear(Color::BLACK);
        sge::draw_string(Point::new(80, 80), "Some test text", Color::WHITE)?;
        Ok(true)
    })
}
