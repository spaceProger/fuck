use crate::config::CorpConfig;
use clap::Subcommand;

mod meeting;
mod email;
mod review;

#[derive(Subcommand)]
pub enum CorpCommands {
    /// Генерирует повестку митинга, который мог быть письмом
    Meeting,
    /// Генерирует корпоративный email с максимумом buzzwords
    Email,
    /// Симулирует performance review
    Review,
}

pub fn run(cmd: CorpCommands, cfg: &CorpConfig) {
    match cmd {
        CorpCommands::Meeting => meeting::run(&cfg.meeting_topics, &cfg.meeting_attendees, &cfg.meeting_outcomes),
        CorpCommands::Email   => email::run(&cfg.email_subjects, &cfg.email_greetings, &cfg.email_bodies, &cfg.email_closings),
        CorpCommands::Review  => review::run(&cfg.review_strengths, &cfg.review_improvements),
    }
}
