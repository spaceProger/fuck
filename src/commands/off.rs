use crate::config::OffConfig;
use colored::*;
use rand::seq::SliceRandom;

pub fn run(cfg: &OffConfig, rude: bool) {
    let list = if rude { &cfg.rude } else { &cfg.polite };
    let msg = list.choose(&mut rand::thread_rng())
        .map(String::as_str).unwrap_or("иди отдохни");

    if rude {
        println!("🚪 {}", msg.red());
        println!("{}", "(Шучу... или нет?)".yellow());
    } else {
        println!("🚪 {}", msg.blue());
    }
}
