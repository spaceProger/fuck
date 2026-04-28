use crate::config::FindOutConfig;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

pub fn run(cfg: &FindOutConfig) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["🔍", "🔎", "🔍", "🔎"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    pb.set_message("Вы решили узнать последствия...");
    for _ in 0..15 {
        pb.tick();
        sleep(Duration::from_millis(100));
    }
    pb.finish_and_clear();

    println!("{}", "🔍 Вы решили узнать последствия...".magenta());
    sleep(Duration::from_millis(600));
    for line in &cfg.consequences {
        println!("{}", line.red());
        sleep(Duration::from_millis(400));
    }
}
