use crate::print::{self, typewrite};
use colored::*;
use rand::seq::SliceRandom;

pub fn run(yesterdays: &[String], todays: &[String], blockers: &[String]) {
    let mut rng = rand::thread_rng();
    let y = yesterdays.choose(&mut rng).map(String::as_str).unwrap_or("ничего");
    let t = todays.choose(&mut rng).map(String::as_str).unwrap_or("тоже ничего");
    let b = blockers.choose(&mut rng).map(String::as_str).unwrap_or("всё");

    println!("{}", "📋 STANDUP ОТЧЁТ (Silicon Valley Edition)".bold().cyan());
    println!("{}", "─".repeat(45).dimmed());
    print!("  {} ", "Вчера:".yellow());   typewrite(y, print::WHITE);
    print!("  {} ", "Сегодня:".green());  typewrite(t, print::WHITE);
    print!("  {} ", "Блокеры:".red());    typewrite(b, print::WHITE);
    println!("{}", "─".repeat(45).dimmed());
    typewrite("Этот митинг мог быть письмом.", print::RESET);
}
