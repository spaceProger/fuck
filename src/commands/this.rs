use crate::config::ThisConfig;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub fn run(cfg: &ThisConfig) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );
    pb.set_message("Анализирую проект...");
    for _ in 0..30 {
        pb.tick();
        sleep(Duration::from_millis(60));
    }
    pb.finish_and_clear();

    let entries = fs::read_dir(".").map(|rd| rd.flatten().count()).unwrap_or(0);
    let has_cargo  = fs::metadata("Cargo.toml").is_ok();
    let has_node   = fs::metadata("package.json").is_ok();
    let has_python = fs::metadata("requirements.txt").is_ok() || fs::metadata("pyproject.toml").is_ok();
    let has_git    = fs::metadata(".git").is_ok();
    let has_docker = fs::metadata("Dockerfile").is_ok();

    println!("{}", "🔬 ДИАГНОЗ ПРОЕКТА".bold().yellow());
    println!("{}", "─".repeat(40).dimmed());

    let lang = if has_cargo { &cfg.lang_rust }
        else if has_node   { &cfg.lang_node }
        else if has_python { &cfg.lang_python }
        else               { &cfg.lang_unknown };
    println!("{}", lang.cyan());
    println!("{}", format!("📁 Файлов в корне: {}", entries).white());

    if has_git {
        println!("{}", "✅ Git есть. Хотя бы это.".green());
    } else {
        println!("{}", "❌ Git отсутствует. Ты живёшь опасно.".red());
    }
    if has_docker {
        println!("{}", "🐳 Dockerfile найден. Контейнеризация — это не решение твоих проблем.".blue());
    }

    println!("{}", "─".repeat(40).dimmed());
    let verdict = cfg.verdicts.choose(&mut rand::thread_rng())
        .map(String::as_str).unwrap_or("диагноз неизвестен");
    println!("{}", format!("🩺 Вердикт: {}", verdict).yellow());
}
