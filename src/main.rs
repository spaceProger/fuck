mod commands;
mod config;
mod print;

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(
    name = "fuck",
    about = "🖕 Утилита для управления фрустрацией и хаосом",
    long_about = None,
    disable_help_subcommand = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Генерирует профессиональное оправдание безделья
    It,
    /// Предлагает пользователю прекратить страдания
    Off {
        #[arg(short, long, help = "Быть более грубым")]
        rude: bool,
    },
    /// Симулирует катастрофический сбой системы
    Up,
    /// Бесполезный прогресс-бар, который тратит твоё время
    Around,
    /// Find out what happens when you fuck around
    #[command(name = "find-out")]
    FindOut,
    /// Анализирует текущий проект и ставит диагноз
    This,
    /// Философски объясняет, почему всё сломалось
    Why,
    /// Счётчик фрустрации с достижениями
    Count,
    /// Команды, вдохновлённые сериалом «Кремниевая долина»
    Sv {
        #[command(subcommand)]
        command: commands::sv::SvCommands,
    },
    /// Корпоративный булшит-генератор
    Corp {
        #[command(subcommand)]
        command: commands::corp::CorpCommands,
    },
}

fn main() {
    let cli = Cli::parse();
    let cfg = config::load();

    match cli.command {
        None => {
            println!("{}", "🖕 Добро пожаловать в FUCK CLI!".bold().magenta());
            println!("{}", "Напиши 'fuck --help', чтобы узнать, как всё испортить.".cyan());
        }
        Some(Commands::It) => commands::it::run(&cfg.it),
        Some(Commands::Off { rude }) => commands::off::run(&cfg.off, rude),
        Some(Commands::Up) => commands::up::run(&cfg.up),
        Some(Commands::Around) => commands::around::run(),
        Some(Commands::FindOut) => commands::find_out::run(&cfg.find_out),
        Some(Commands::This) => commands::this::run(&cfg.this),
        Some(Commands::Why) => commands::why::run(&cfg.why),
        Some(Commands::Count) => commands::count::run(),
        Some(Commands::Sv { command }) => commands::sv::run(command, &cfg.sv),
        Some(Commands::Corp { command }) => commands::corp::run(command, &cfg.corp),
    }
}
