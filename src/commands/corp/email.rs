use colored::*;
use rand::seq::SliceRandom;

pub fn run(subjects: &[String], greetings: &[String], bodies: &[String], closings: &[String]) {
    let mut rng = rand::thread_rng();
    let subject  = subjects.choose(&mut rng).map(String::as_str).unwrap_or("Важно");
    let greeting = greetings.choose(&mut rng).map(String::as_str).unwrap_or("Команда,");
    let body     = bodies.choose(&mut rng).map(String::as_str).unwrap_or("Синергия.");
    let closing  = closings.choose(&mut rng).map(String::as_str).unwrap_or("С уважением,");

    println!("{}", "📧 КОРПОРАТИВНЫЙ EMAIL".bold().blue());
    println!("{}", "─".repeat(55).dimmed());
    println!("  {} {}", "Тема:".yellow(), subject.white().bold());
    println!();
    println!("  {}", greeting.cyan());
    println!();
    println!("  {}", body.white());
    println!();
    println!("  {}", closing.cyan());
    println!("  {}", "Менеджер Менеджеров, Senior VP of Synergy".dimmed());
    println!("{}", "─".repeat(55).dimmed());
}
