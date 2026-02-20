use std::io::{self, Write};

use chroma_print::Color;

/// Display loading message
pub fn show_loading(message: Option<&str>) {
    print!(
        "{}{}{}",
        Color::Yellow.value(),
        message.unwrap_or("Loading..."),
        Color::Reset.value()
    );
    io::stdout().flush().unwrap();
}

/// Clear loading message
pub fn clear_loading() {
    print!("\r\x1b[2K"); // Clears the entire current line
    io::stdout().flush().unwrap();
}
