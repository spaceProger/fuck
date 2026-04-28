use crate::print::{self, typewrite};
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
        ("⭐⭐⭐⭐⭐", "Превышает ожидания",      print::GREEN),
        ("⭐⭐⭐⭐",   "Соответствует ожиданиям", print::YELLOW),
        ("⭐⭐⭐",     "Частично соответствует",  print::YELLOW),
        ("⭐⭐",       "Требует улучшения",        print::RED),
    ];
    let mut rng = rand::thread_rng();
    let (stars, label, color) = ratings.choose(&mut rng).unwrap();
    let strength    = strengths.choose(&mut rng).map(String::as_str).unwrap_or("существует");
    let improvement = improvements.choose(&mut rng).map(String::as_str).unwrap_or("всё остальное");

    println!("{}", "📊 PERFORMANCE REVIEW".bold().blue());
    println!("{}", "─".repeat(50).dimmed());
    print!("  {} ", "Оценка:".white());
    typewrite(&format!("{} — {}", stars, label), color);
    print!("  {} ", "Сильные стороны:".green());
    typewrite(strength, print::WHITE);
    print!("  {} ", "Зоны роста:".yellow());
    typewrite(improvement, print::WHITE);
    println!("{}", "─".repeat(50).dimmed());
    typewrite("Повышение зарплаты: 0%. Зато грамота.", print::RED);
}
