use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["💰", " 💰", "  💰", "   💰", "  💰", " 💰"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.set_message("Считаем оценку стартапа...");
    for _ in 0..25 {
        pb.tick();
        sleep(Duration::from_millis(80));
    }
    pb.finish_and_clear();

    let mut rng = rand::thread_rng();
    let buzzwords: u64 = rng.gen_range(10..=100);
    let hype: u64 = rng.gen_range(1..=10);
    let revenue: u64 = rng.gen_range(0..=5); // почти всегда 0
    let valuation = if revenue == 0 {
        buzzwords * hype * 1_000_000
    } else {
        buzzwords * hype * 1_000_000 / revenue
    };

    println!("{}", "📈 ОЦЕНКА СТАРТАПА".bold().green());
    println!("{}", "─".repeat(40).dimmed());
    println!("  {} {}", "Buzzwords в питчдеке:".yellow(), buzzwords);
    println!("  {} {}x", "Уровень хайпа:".cyan(), hype);
    println!("  {} ${}k", "Реальная выручка:".red(), revenue);
    println!("{}", "─".repeat(40).dimmed());
    println!("  {} ${}", "💎 Оценка:".bold().green(), format_valuation(valuation));
    println!("{}", "─".repeat(40).dimmed());

    if revenue == 0 {
        println!("{}", "🚀 Выручки нет — значит, потенциал безграничен!".yellow());
    }
    println!("{}", "\"We're not a company, we're a movement.\"".blue().italic());
}

fn format_valuation(v: u64) -> String {
    if v >= 1_000_000_000 {
        format!("{:.1}B", v as f64 / 1_000_000_000.0)
    } else if v >= 1_000_000 {
        format!("{:.1}M", v as f64 / 1_000_000.0)
    } else {
        format!("{}", v)
    }
}
