use crate::config::FindOutConfig;
use crate::print::{self, typewrite};
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

    typewrite("🔍 Вы решили узнать последствия...", print::MAGENTA);
    sleep(Duration::from_millis(400));
    for line in &cfg.consequences {
        typewrite(line, print::RED);
        sleep(Duration::from_millis(200));
    }
}
