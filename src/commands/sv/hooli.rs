use colored::*;
use rand::seq::SliceRandom;

pub fn run(quotes: &[String]) {
    let quote = quotes.choose(&mut rand::thread_rng())
        .map(String::as_str).unwrap_or("Инновации — это наш путь.");
    println!("{}", "🏢 Hooli Corporate Statement:".blue().bold());
    println!("{}", format!("\"{}\"", quote).bold());
    println!("{}", "— Gavin Belson".cyan());
}
