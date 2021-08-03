use std::error::Error;

struct App;

impl sge::Application for App {}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App;
    sge::Builder::new("Test App", 640, 480).start(&mut app)
}
