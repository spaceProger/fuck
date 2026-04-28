use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

pub fn run(people: u32, speed: f64) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["🧮", " 🧮", "  🧮", " 🧮"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.set_message("Запуск калькулятора Nucleus...");
    for _ in 0..20 {
        pb.tick();
        sleep(Duration::from_millis(80));
    }
    pb.finish_and_clear();

    let total_minutes = people as f64 * speed;
    let hours = total_minutes / 60.0;
    let days = hours / 24.0;

    println!("{}", "🧮 РЕЗУЛЬТАТЫ РАСЧЁТА NUCLEUS".bold().magenta());
    println!("{}", "─".repeat(40).dimmed());
    println!("  {} {} чел.", "Аудитория:".cyan(), people);
    println!("  {} {} мин/чел", "Скорость:".cyan(), speed);
    println!("{}", "─".repeat(40).dimmed());
    println!("  {} {:.1} минут", "⏳ Общее время:".yellow(), total_minutes);
    println!("  {} {:.2} часов", "🕒 Это примерно:".yellow(), hours);
    println!("  {} {:.2} дней", "📅 Или:".yellow(), days);
    println!("{}", "─".repeat(40).dimmed());

    if days > 1.0 {
        println!("{}", "⚠️  Вам понадобится команда помощников.".red());
        println!("{}", "💡 Совет: Наймите Джин-Янга. Он быстрый.".green());
    } else {
        println!("{}", "✅ Вы успеете до обеда!".green());
    }
    println!("{}", "\n\"It's not magic, it's talent and sweat.\" — Richard Hendricks".blue());
}
