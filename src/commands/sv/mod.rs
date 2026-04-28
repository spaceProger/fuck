use crate::config::SvConfig;
use clap::Subcommand;

mod pied_piper;
mod not_hotdog;
mod hooli;
mod tabs_spaces;
mod nucleus;
mod erlich;
mod standup;
mod valuation;

#[derive(Subcommand)]
pub enum SvCommands {
    /// Сжимает текст алгоритмом Pied Piper (Middle-Out)
    PiedPiper {
        #[arg(default_value = "Hello World")]
        text: String,
    },
    /// Определяет, является ли объект хот-догом
    NotHotdog {
        #[arg(default_value = "banana")]
        item: String,
    },
    /// Генерирует корпоративную цитату Гэвина Белсона
    Hooli,
    /// Решает вечный спор табов и пробелов
    TabsSpaces,
    /// Рассчитывает время для удовлетворения аудитории
    Nucleus {
        #[arg(short, long, default_value_t = 500)]
        people: u32,
        #[arg(short, long, default_value_t = 2.5)]
        speed: f64,
    },
    /// Монолог Эрлиха Бэкмана
    Erlich,
    /// Генерирует standup отчёт в стиле Silicon Valley
    Standup,
    /// Считает оценку стартапа
    Valuation,
}

pub fn run(cmd: SvCommands, cfg: &SvConfig) {
    match cmd {
        SvCommands::PiedPiper { text } => pied_piper::run(&text),
        SvCommands::NotHotdog { item } => not_hotdog::run(&item),
        SvCommands::Hooli => hooli::run(&cfg.hooli_quotes),
        SvCommands::TabsSpaces => tabs_spaces::run(),
        SvCommands::Nucleus { people, speed } => nucleus::run(people, speed),
        SvCommands::Erlich => erlich::run(&cfg.erlich_openers, &cfg.erlich_middles, &cfg.erlich_claims),
        SvCommands::Standup => standup::run(&cfg.standup_yesterdays, &cfg.standup_todays, &cfg.standup_blockers),
        SvCommands::Valuation => valuation::run(),
    }
}
