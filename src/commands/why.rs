use crate::config::WhyConfig;
use colored::*;
use rand::seq::SliceRandom;

pub fn run(cfg: &WhyConfig) {
    let mut rng = rand::thread_rng();
    let reason  = cfg.reasons.choose(&mut rng).map(String::as_str).unwrap_or("неизвестно");
    let culprit = cfg.culprits.choose(&mut rng).map(String::as_str).unwrap_or("кто-то");
    let solution = cfg.solutions.choose(&mut rng).map(String::as_str).unwrap_or("помолиться");

    println!("{}", "🤔 ФИЛОСОФСКИЙ АНАЛИЗ ПОЛОМКИ".bold().magenta());
    println!("{}", "─".repeat(40).dimmed());
    println!("  {} {}", "Причина:".yellow(),  reason.white());
    println!("  {} {}", "Виновник:".red(),    culprit.white());
    println!("  {} {}", "Решение:".green(),   solution.white());
    println!("{}", "─".repeat(40).dimmed());
    println!("{}", "Вселенная не обязана быть понятной.".dimmed().italic());
}
