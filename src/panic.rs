use sdl2::messagebox::{show_simple_message_box, MessageBoxFlag};

use crate::ENGINE;

/// Installs a panic hook that will display a message box if the program panics. If the message box
/// could not be displayed,falls back to printing to standard output.
pub fn install_hook() {
    std::panic::set_hook(Box::new(|info| {
        let thread = std::thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");
        let msg = format!("Thread '{}' {}", thread_name, info);
        ENGINE
            .try_with(|e| {
                let engine = e.get().and_then(|e| e.try_borrow().ok());
                if let Some(e) = engine {
                    // Use the parent window
                    show_simple_message_box(
                        MessageBoxFlag::ERROR,
                        "Application Panicked",
                        &msg,
                        e.canvas.window(),
                    )
                } else {
                    // We can't get access to the parent window, don't use one
                    show_simple_message_box(
                        MessageBoxFlag::ERROR,
                        "Application Panicked",
                        &msg,
                        None,
                    )
                }
            })
            .map(|r| {
                r.or_else(|_| {
                    // We can't get access to the parent window, don't use one
                    show_simple_message_box(
                        MessageBoxFlag::ERROR,
                        "Application Panicked",
                        &msg,
                        None,
                    )
                })
            })
            .ok();
        // Also print to stderr
        eprintln!("{}", msg);
    }));
}
