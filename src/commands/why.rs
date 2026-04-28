use crate::config::WhyConfig;
use crate::print::{self, typewrite};
use colored::*;
use rand::seq::SliceRandom;

pub fn run(cfg: &WhyConfig) {
    let mut rng = rand::thread_rng();
    let reason   = cfg.reasons.choose(&mut rng).map(String::as_str).unwrap_or("неизвестно");
    let culprit  = cfg.culprits.choose(&mut rng).map(String::as_str).unwrap_or("кто-то");
    let solution = cfg.solutions.choose(&mut rng).map(String::as_str).unwrap_or("помолиться");

    println!("{}", "🤔 ФИЛОСОФСКИЙ АНАЛИЗ ПОЛОМКИ".bold().magenta());
    println!("{}", "─".repeat(40).dimmed());
    print!("  {} ", "Причина:".yellow());  typewrite(reason,   print::WHITE);
    print!("  {} ", "Виновник:".red());    typewrite(culprit,  print::WHITE);
    print!("  {} ", "Решение:".green());   typewrite(solution, print::WHITE);
    println!("{}", "─".repeat(40).dimmed());
    typewrite("Вселенная не обязана быть понятной.", print::RESET);
}
