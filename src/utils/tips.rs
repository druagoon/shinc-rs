use colored::Color;

use super::formatter::Formatter;

#[allow(dead_code)]
pub fn title(text: &str) {
    println!("{}", Formatter::new().headline(text).color(Color::Green));
}

#[allow(dead_code)]
pub fn h1(text: &str) {
    println!("{}", Formatter::new().headline(text).color(Color::Blue));
}

#[allow(dead_code)]
pub fn h2(text: &str) {
    println!("{}", Formatter::new().indent("  ").headline(text).color(Color::Blue));
}

#[allow(dead_code)]
pub fn debug(message: &str) {
    eprintln!("{}", Formatter::new().headline(message).color(Color::Magenta));
}

#[allow(dead_code)]
pub fn warning(message: &str) {
    eprintln!("{}", Formatter::new().warning("Warning").text(message));
}

#[allow(dead_code)]
pub fn error(message: &str) {
    eprintln!("{}", Formatter::new().error("Error").text(message));
}

#[allow(dead_code)]
pub fn die(message: &str) {
    error(message);
    std::process::exit(1);
}
