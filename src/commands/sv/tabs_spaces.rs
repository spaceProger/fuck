use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    println!("{}", "⚔️  Начинается битва: Табы против Пробелов...".red().bold());

    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  TABS    [{bar:20.green}] vs [{bar:20.blue}] SPACES")
            .unwrap()
            .progress_chars("█░░"),
    );
    let mut rng = rand::thread_rng();
    for i in 0..=100u64 {
        pb.set_position(i);
        sleep(Duration::from_millis(rng.gen_range(10..50)));
    }
    pb.finish_and_clear();
    sleep(Duration::from_millis(300));

    let winner = if rand::thread_rng().gen_bool(0.5) { "TABS" } else { "SPACES" };
    if winner == "TABS" {
        println!("{}", "🏆 Победили ТАБЫ!".green().bold());
        println!("{}", "Пробелы — это трата памяти. Ты уволен.".red());
    } else {
        println!("{}", "🏆 Победили ПРОБЕЛЫ!".green().bold());
        println!("{}", "Табы — это анархия. Твой код отвратителен.".red());
    }
    println!("{}", "Gilfoyle молча наблюдает за этим цирком.".magenta());
}
