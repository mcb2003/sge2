use std::error::Error;

struct App;

impl sge::Application for App {
    fn on_update(&mut self, elapsed_time: f64) -> sge::ApplicationResult {
        println!("FPS: {}", 1.0 / elapsed_time);
        Ok(true)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App;
    sge::start(&mut app, "Test App", 640, 480)
}
