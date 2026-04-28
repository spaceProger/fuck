use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use std::thread::sleep;
use std::time::Duration;

pub fn run(strengths: &[String], improvements: &[String]) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message("Генерирую performance review...");
    for _ in 0..30 {
        pb.tick();
        sleep(Duration::from_millis(60));
    }
    pb.finish_and_clear();

    let ratings = [
        ("⭐⭐⭐⭐⭐", "Превышает ожидания",      "green"),
        ("⭐⭐⭐⭐",   "Соответствует ожиданиям", "yellow"),
        ("⭐⭐⭐",     "Частично соответствует",  "yellow"),
        ("⭐⭐",       "Требует улучшения",        "red"),
    ];
    let mut rng = rand::thread_rng();
    let (stars, label, color) = ratings.choose(&mut rng).unwrap();
    let strength   = strengths.choose(&mut rng).map(String::as_str).unwrap_or("существует");
    let improvement = improvements.choose(&mut rng).map(String::as_str).unwrap_or("всё остальное");

    let rating_str = format!("{} — {}", stars, label);
    let colored_rating = match *color {
        "green" => rating_str.green().bold().to_string(),
        "red"   => rating_str.red().bold().to_string(),
        _       => rating_str.yellow().bold().to_string(),
    };

    println!("{}", "📊 PERFORMANCE REVIEW".bold().blue());
    println!("{}", "─".repeat(50).dimmed());
    println!("  {} {}", "Оценка:".white(),          colored_rating);
    println!("  {} {}", "Сильные стороны:".green(), strength.white());
    println!("  {} {}", "Зоны роста:".yellow(),     improvement.white());
    println!("{}", "─".repeat(50).dimmed());
    println!("{}", "Повышение зарплаты: 0%. Зато грамота.".red());
}
