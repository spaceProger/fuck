use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

/// Печатает строку посимвольно с заданным ANSI-цветом.
pub fn typewrite(text: &str, color_code: &str) {
    let reset = "\x1b[0m";
    for ch in text.chars() {
        print!("{}{}{}", color_code, ch, reset);
        io::stdout().flush().unwrap();
        let delay = if matches!(ch, ',' | '.' | '!' | '?' | ':') {
            Duration::from_millis(60)
        } else {
            Duration::from_millis(18)
        };
        sleep(delay);
    }
    println!();
}

// Цветовые константы для удобства
pub const GREEN:   &str = "\x1b[92m";
pub const YELLOW:  &str = "\x1b[93m";
pub const RED:     &str = "\x1b[91m";
pub const BLUE:    &str = "\x1b[94m";
pub const MAGENTA: &str = "\x1b[95m";
pub const CYAN:    &str = "\x1b[96m";
pub const BOLD:    &str = "\x1b[1m";
pub const WHITE:   &str = "\x1b[97m";
pub const RESET:   &str = "\x1b[0m";
