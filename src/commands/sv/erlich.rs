use colored::*;
use rand::seq::SliceRandom;

pub fn run(openers: &[String], middles: &[String], claims: &[String]) {
    let mut rng = rand::thread_rng();
    let opener = openers.choose(&mut rng).map(String::as_str).unwrap_or("Послушайте");
    let middle = middles.choose(&mut rng).map(String::as_str).unwrap_or("и скажу вам");
    let claim  = claims.choose(&mut rng).map(String::as_str).unwrap_or("что я лучший.");

    println!("{}", "🧔 МОНОЛОГ ЭРЛИХА БЭКМАНА".bold().yellow());
    println!("{}", "─".repeat(50).dimmed());
    println!("  \"{} {}, {}\"", opener, middle, claim);
    println!("{}", "─".repeat(50).dimmed());
    println!("{}", "— Erlich Bachman, CEO of Aviato™".cyan());
}
