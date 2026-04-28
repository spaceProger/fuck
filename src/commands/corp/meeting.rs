use colored::*;
use rand::seq::SliceRandom;

pub fn run(topics: &[String], attendees: &[String], outcomes: &[String]) {
    let mut rng = rand::thread_rng();
    let topic    = topics.choose(&mut rng).map(String::as_str).unwrap_or("Синергия");
    let attendee = attendees.choose(&mut rng).map(String::as_str).unwrap_or("все");
    let outcome  = outcomes.choose(&mut rng).map(String::as_str).unwrap_or("ничего");

    println!("{}", "📅 ПОВЕСТКА МИТИНГА".bold().blue());
    println!("{}", "─".repeat(50).dimmed());
    println!("  {} {}", "Тема:".yellow(),                topic.white());
    println!("  {} {}", "Участники:".cyan(),             attendee.white());
    println!("  {} 60 минут (могло быть письмом за 2 минуты)", "Длительность:".red());
    println!("  {} {}", "Ожидаемый результат:".green(),  outcome.white());
    println!("{}", "─".repeat(50).dimmed());
    println!("{}", "⚠️  Этот митинг мог быть письмом.".yellow().bold());
}
