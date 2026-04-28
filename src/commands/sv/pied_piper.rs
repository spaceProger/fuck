use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub fn run(text: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Запуск алгоритма сжатия Pied Piper...");
    for _ in 0..25 {
        pb.tick();
        sleep(Duration::from_millis(60));
    }
    pb.finish_and_clear();

    let compressed: String = text
        .chars()
        .filter(|c| !"aeiouAEIOU".contains(*c))
        .collect();

    let efficiency = rand::thread_rng().gen_range(40..=99);

    println!("{}", "🎵 Алгоритм Pied Piper (Middle-Out)".cyan().bold());
    println!("{}", "─".repeat(40).dimmed());
    println!("  {} {} байт", "Исходный размер:".yellow(), text.len());
    println!("  {} {} байт", "Сжатый размер:".green(), compressed.len());
    println!("  {} {}%", "Эффективность:".bold().green(), efficiency);
    println!("  {} '{}'", "Результат:".magenta(), compressed.bold());
    println!("{}", "─".repeat(40).dimmed());
    println!("{}", "💡 Ричард Хендрикс одобряет этот результат.".blue());
}
