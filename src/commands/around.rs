use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    println!("{}", "🔄 Запуск процесса 'Fuck Around'...".cyan());
    println!("{}", "Пожалуйста, подождите... (это займёт вечность)".yellow());

    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} [{bar:40.cyan/blue}] {pos}% | {msg}")
            .unwrap()
            .progress_chars("█▓░"),
    );

    let messages = [
        "думаю...", "сомневаюсь...", "откатываюсь...", "переосмысляю...",
        "игнорирую проблему...", "делаю вид что работаю...",
    ];
    let mut rng = rand::thread_rng();
    let mut pos: i64 = 0;

    while pos < 100 {
        let msg = messages[rng.gen_range(0..messages.len())];
        pb.set_message(msg);

        if rng.gen_bool(0.15) {
            let back = rng.gen_range(3..8).min(pos);
            pos -= back;
            pb.set_position(pos.max(0) as u64);
            sleep(Duration::from_millis(300));
        } else {
            pos += 1;
            pb.set_position(pos as u64);
            sleep(Duration::from_millis(60));
        }
    }

    pb.finish_with_message("готово?");
    println!();
    println!("{}", "✨ Готово! Ничего не произошло.".green());
}
