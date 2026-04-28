use crate::config::OffConfig;
use crate::print::{self, typewrite};
use colored::*;
use rand::seq::SliceRandom;

pub fn run(cfg: &OffConfig, rude: bool) {
    let list = if rude { &cfg.rude } else { &cfg.polite };
    let msg = list.choose(&mut rand::thread_rng())
        .map(String::as_str).unwrap_or("иди отдохни");

    print!("🚪 ");
    if rude {
        typewrite(msg, print::RED);
        println!("{}", "(Шучу... или нет?)".yellow());
    } else {
        typewrite(msg, print::BLUE);
    }
}
