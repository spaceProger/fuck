use colored::*;
use std::fs;
use std::path::PathBuf;

fn count_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".fuck_count")
}

pub fn run() {
    let path = count_file();
    let count: u64 = fs::read_to_string(&path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
        + 1;

    fs::write(&path, count.to_string()).ok();

    println!("{}", format!("🖕 Ты запустил fuck {} раз(а).", count).bold().magenta());

    let achievement = match count {
        1 => Some(("🐣 Новичок", "Первый раз. Добро пожаловать в клуб.")),
        10 => Some(("🔥 Десятка", "10 запусков. Ты явно что-то сломал.")),
        25 => Some(("😤 Фрустрированный", "25 запусков. Всё ещё не работает?")),
        50 => Some(("💀 Ветеран страданий", "50 запусков. Обратись к психологу.")),
        100 => Some(("🏆 Легенда хаоса", "100 запусков. Ты наш герой.")),
        _ => None,
    };

    if let Some((title, desc)) = achievement {
        println!();
        println!("{}", "🎖️  НОВОЕ ДОСТИЖЕНИЕ!".bold().yellow());
        println!("  {} — {}", title.bold(), desc);
    }
}
