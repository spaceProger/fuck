use crate::config::ItConfig;
use crate::print::{self, typewrite};
use colored::*;
use rand::seq::SliceRandom;

pub fn run(cfg: &ItConfig) {
    let excuse = cfg.excuses.choose(&mut rand::thread_rng())
        .map(String::as_str).unwrap_or("всё сломалось само");
    print!("{}", "💼 Оправдание дня: ".green());
    typewrite(&format!("\"{}\"", excuse), print::GREEN);
    println!("{}", "Теперь можешь смело идти пить чай.".yellow());
}
