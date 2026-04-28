use crate::config::ItConfig;
use colored::*;
use rand::seq::SliceRandom;

pub fn run(cfg: &ItConfig) {
    let excuse = cfg.excuses.choose(&mut rand::thread_rng())
        .map(String::as_str).unwrap_or("всё сломалось само");
    println!("{}", format!("💼 Оправдание дня: \"{}\"", excuse).green());
    println!("{}", "Теперь можешь смело идти пить чай.".yellow());
}
