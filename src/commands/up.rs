use crate::config::UpConfig;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

pub fn run(cfg: &UpConfig) {
    println!("{}", "⚠️  Инициализация протокола 'FUCK UP'...".red().bold());
    sleep(Duration::from_millis(800));

    for step in &cfg.steps {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
                .template("{spinner:.red} {msg}")
                .unwrap(),
        );
        pb.set_message(step.clone());
        for _ in 0..20 {
            pb.tick();
            sleep(Duration::from_millis(60));
        }
        pb.finish_with_message(format!("{} {}", "[FAIL]".red().bold(), step.red()));
    }

    sleep(Duration::from_millis(400));
    println!();
    println!("{}", r"
  ██████╗  ██████╗  ██████╗ ███╗   ███╗██╗
  ██╔══██╗██╔═══██╗██╔═══██╗████╗ ████║██║
  ██████╔╝██║   ██║██║   ██║██╔████╔██║██║
  ██╔══██╗██║   ██║██║   ██║██║╚██╔╝██║╚═╝
  ██████╔╝╚██████╔╝╚██████╔╝██║ ╚═╝ ██║██╗
  ╚═════╝  ╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝".red().bold());
    println!();
    println!("{}", "💥 СИСТЕМА УНИЧТОЖЕНА!".red().bold());
    println!("{}", "(На самом деле нет. Но ты же испугался, да?)".green());
}
