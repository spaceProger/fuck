use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub fn run(item: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["🌭", " 🌭", "  🌭", "   🌭", "  🌭", " 🌭"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.set_message(format!("Анализ объекта '{}' через нейросеть Jian-Yang...", item));
    for _ in 0..20 {
        pb.tick();
        sleep(Duration::from_millis(100));
    }
    pb.finish_and_clear();

    let is_hotdog = rand::thread_rng().gen_bool(0.1);
    if is_hotdog {
        println!("{}", format!("✅ HOTDOG! (Даже если это '{}')", item).green().bold());
        println!("{}", "Jian-Yang гордится тобой.".yellow());
    } else {
        println!("{}", "❌ NOT HOTDOG.".red().bold());
        println!("{}", format!("Объект '{}' отвергнут.", item).red());
    }
}
