use crate::print::{self, typewrite};
use colored::*;
use rand::seq::SliceRandom;

pub fn run(topics: &[String], attendees: &[String], outcomes: &[String]) {
    let mut rng = rand::thread_rng();
    let topic    = topics.choose(&mut rng).map(String::as_str).unwrap_or("Синергия");
    let attendee = attendees.choose(&mut rng).map(String::as_str).unwrap_or("все");
    let outcome  = outcomes.choose(&mut rng).map(String::as_str).unwrap_or("ничего");

    println!("{}", "📅 ПОВЕСТКА МИТИНГА".bold().blue());
    println!("{}", "─".repeat(50).dimmed());
    print!("  {} ", "Тема:".yellow());               typewrite(topic,    print::WHITE);
    print!("  {} ", "Участники:".cyan());             typewrite(attendee, print::WHITE);
    println!("  {} 60 минут (могло быть письмом за 2 минуты)", "Длительность:".red());
    print!("  {} ", "Ожидаемый результат:".green());  typewrite(outcome,  print::WHITE);
    println!("{}", "─".repeat(50).dimmed());
    typewrite("⚠️  Этот митинг мог быть письмом.", print::YELLOW);
}
