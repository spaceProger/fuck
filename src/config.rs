use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {    #[serde(default)] pub it: ItConfig,
    #[serde(default)] pub off: OffConfig,
    #[serde(default)] pub up: UpConfig,
    #[serde(default)] pub find_out: FindOutConfig,
    #[serde(default)] pub why: WhyConfig,
    #[serde(default)] pub this: ThisConfig,
    #[serde(default)] pub sv: SvConfig,
    #[serde(default)] pub corp: CorpConfig,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ItConfig {
    #[serde(default)] pub excuses: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OffConfig {
    #[serde(default)] pub polite: Vec<String>,
    #[serde(default)] pub rude: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UpConfig {
    #[serde(default)] pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FindOutConfig {
    #[serde(default)] pub consequences: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct WhyConfig {
    #[serde(default)] pub reasons: Vec<String>,
    #[serde(default)] pub culprits: Vec<String>,
    #[serde(default)] pub solutions: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ThisConfig {
    #[serde(default)] pub verdicts: Vec<String>,
    #[serde(default)] pub lang_rust: String,
    #[serde(default)] pub lang_node: String,
    #[serde(default)] pub lang_python: String,
    #[serde(default)] pub lang_unknown: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SvConfig {
    #[serde(default)] pub hooli_quotes: Vec<String>,
    #[serde(default)] pub erlich_openers: Vec<String>,
    #[serde(default)] pub erlich_middles: Vec<String>,
    #[serde(default)] pub erlich_claims: Vec<String>,
    #[serde(default)] pub standup_yesterdays: Vec<String>,
    #[serde(default)] pub standup_todays: Vec<String>,
    #[serde(default)] pub standup_blockers: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CorpConfig {
    #[serde(default)] pub meeting_topics: Vec<String>,
    #[serde(default)] pub meeting_attendees: Vec<String>,
    #[serde(default)] pub meeting_outcomes: Vec<String>,
    #[serde(default)] pub email_subjects: Vec<String>,
    #[serde(default)] pub email_greetings: Vec<String>,
    #[serde(default)] pub email_bodies: Vec<String>,
    #[serde(default)] pub email_closings: Vec<String>,
    #[serde(default)] pub review_strengths: Vec<String>,
    #[serde(default)] pub review_improvements: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            it: ItConfig {
                excuses: vec![
                    "У меня был митинг, который длился вечность.".into(),
                    "Деплой упал из-за лунной фазы.".into(),
                    "Я ждал, пока остынет кофе.".into(),
                    "Баг был в продакшене, я просто наблюдал.".into(),
                    "Git сказал 'no'.".into(),
                    "Я думал о смысле жизни вместо кода.".into(),
                    "Сервер ушёл в отпуск без предупреждения.".into(),
                    "Это не баг, это фича, которую никто не просил.".into(),
                    "Компилятор был не в настроении.".into(),
                    "Я изучал документацию. Она не помогла.".into(),
                ],
            },
            off: OffConfig {
                polite: vec![
                    "Пожалуйста, отойдите от клавиатуры.".into(),
                    "Ваше присутствие здесь больше не требуется.".into(),
                    "Сделайте перерыв, вы выглядите уставшим.".into(),
                    "Мир подождёт, идите отдохните.".into(),
                ],
                rude: vec![
                    "Иди нахер отсюда.".into(),
                    "Закрой ноут и иди гуляй.".into(),
                    "Твой код никому не нужен, иди спать.".into(),
                    "Выключи компьютер, пока он не взорвался.".into(),
                ],
            },
            up: UpConfig {
                steps: vec![
                    "Удаление важных файлов".into(),
                    "Шифрование диска ключом '1234'".into(),
                    "Отправка истории браузера начальнику".into(),
                    "Перегрев процессора до 1000°C".into(),
                    "Замена всех пробелов на табы".into(),
                    "Форматирование prod базы данных".into(),
                    "Публикация .env в открытый репозиторий".into(),
                ],
            },
            find_out: FindOutConfig {
                consequences: vec![
                    "📉 Ваш социальный рейтинг снижен на 50 пунктов.".into(),
                    "🐱 Кот теперь смотрит на вас с осуждением.".into(),
                    "☕ Кофе в офисе закончился именно из-за вас.".into(),
                    "🌍 Где-то в мире упал ещё один прод.".into(),
                ],
            },
            why: WhyConfig {
                reasons: vec![
                    "ретроградный Меркурий".into(),
                    "неправильно расставленные запятые".into(),
                    "квантовая неопределённость в продакшене".into(),
                    "кто-то мёрджнул в пятницу вечером".into(),
                    "баг существовал ещё до Big Bang".into(),
                    "разработчик думал, что это очевидно".into(),
                    "документация врала".into(),
                    "npm audit fix --force".into(),
                ],
                culprits: vec![
                    "стажёр".into(),
                    "ты сам три месяца назад".into(),
                    "неизвестный коммит без описания".into(),
                    "библиотека с 2 звёздами на GitHub".into(),
                    "Stack Overflow ответ 2011 года".into(),
                    "ChatGPT".into(),
                    "senior разработчик, который уже уволился".into(),
                ],
                solutions: vec![
                    "выключить и включить снова".into(),
                    "добавить ещё один слой абстракции".into(),
                    "переписать на Rust".into(),
                    "сделать вид, что это фича".into(),
                    "создать тикет с приоритетом P4".into(),
                    "помолиться компилятору".into(),
                    "откатить деплой и уйти в отпуск".into(),
                ],
            },
            this: ThisConfig {
                verdicts: vec![
                    "Проект выглядит как последствие дедлайна.".into(),
                    "Архитектура напоминает джунгли после урагана.".into(),
                    "Код написан с любовью. К себе, не к читателю.".into(),
                    "Технический долг достиг астрономических значений.".into(),
                    "Это либо гениально, либо катастрофа. Скорее второе.".into(),
                ],
                lang_rust: "🦀 Rust — ты мазохист, но уважаемый".into(),
                lang_node: "📦 Node.js — node_modules весит больше твоей души".into(),
                lang_python: "🐍 Python — пространство имён в порядке, совесть нет".into(),
                lang_unknown: "❓ Неизвестный язык — это смело".into(),
            },
            sv: SvConfig {
                hooli_quotes: vec![
                    "Мы делаем мир лучше через платформы изменения масштаба.".into(),
                    "Hooli Search — потому что Google слишком сложен.".into(),
                    "Я не сплю. Я медитирую с открытыми глазами.".into(),
                    "Наш новый продукт изменит способ, которым вы дышите.".into(),
                    "Box 3? Нет, мы используем Box 4. Он более квадратный.".into(),
                    "Добро пожаловать в Hooli. Здесь мы семья. Дисфункциональная, но семья.".into(),
                    "Инновации — это не то, что ты делаешь. Это то, что ты говоришь, что делаешь.".into(),
                ],
                erlich_openers: vec![
                    "Позвольте мне быть с вами честным".into(),
                    "Я построил три компании с нуля".into(),
                    "Когда я был в Burning Man в 2009-м".into(),
                    "Как человек, который изменил индустрию".into(),
                    "Я не хвастаюсь, просто констатирую факты".into(),
                ],
                erlich_middles: vec![
                    "и могу сказать вам с полной уверенностью".into(),
                    "после многолетнего опыта в этой сфере".into(),
                    "имея за плечами несколько успешных экзитов".into(),
                    "будучи визионером и лидером мнений".into(),
                ],
                erlich_claims: vec![
                    "что ваш стартап — это следующий единорог.".into(),
                    "что я уже делал это лучше пять лет назад.".into(),
                    "что Aviato изменила правила игры навсегда.".into(),
                    "что инкубатор — это не просто дом, это образ жизни.".into(),
                    "что я мог бы купить вашу компанию, но не буду.".into(),
                ],
                standup_yesterdays: vec![
                    "думал о масштабировании".into(),
                    "рефакторил архитектуру в голове".into(),
                    "изучал конкурентов (смотрел Netflix)".into(),
                    "синхронизировал видение с командой".into(),
                    "оптимизировал процессы (пил кофе)".into(),
                ],
                standup_todays: vec![
                    "буду думать о масштабировании".into(),
                    "продолжу синергизировать пайплайны".into(),
                    "займусь дизрапшном индустрии".into(),
                    "буду итерировать над MVP".into(),
                    "проведу глубокий анализ рынка".into(),
                ],
                standup_blockers: vec![
                    "всё".into(),
                    "реальность не соответствует питчдеку".into(),
                    "инвесторы задают неудобные вопросы".into(),
                    "продакшн снова упал".into(),
                    "команда не разделяет моё видение".into(),
                    "технический долг размером с Техас".into(),
                ],
            },
            corp: CorpConfig {
                meeting_topics: vec![
                    "Синхронизация по синхронизации прошлой синхронизации".into(),
                    "Обсуждение формата будущих обсуждений".into(),
                    "Выравнивание стратегического выравнивания".into(),
                    "Ретроспектива ретроспективы Q3".into(),
                    "Митинг по поводу слишком большого количества митингов".into(),
                ],
                meeting_attendees: vec![
                    "все из отдела, включая тех, кто не знает зачем".into(),
                    "14 человек, из которых 12 будут молчать".into(),
                    "вся команда + менеджер менеджера менеджера".into(),
                ],
                meeting_outcomes: vec![
                    "Запланировать следующий митинг".into(),
                    "Создать документ о создании документов".into(),
                    "Делегировать делегирование".into(),
                    "Договориться договориться позже".into(),
                ],
                email_subjects: vec![
                    "Re: Re: Re: Fw: Синергия наших синергий".into(),
                    "ACTION REQUIRED: Выравнивание по выравниванию".into(),
                    "FYI: Важное обновление (не срочно, но срочно)".into(),
                ],
                email_greetings: vec![
                    "Команда,".into(),
                    "Коллеги,".into(),
                    "Все заинтересованные стороны,".into(),
                ],
                email_bodies: vec![
                    "Хочу проактивно синхронизироваться по нашим стратегическим инициативам и убедиться, что мы движемся в одном направлении с точки зрения выравнивания наших KPI.".into(),
                    "В продолжение нашего последнего митинга хочу задействовать все заинтересованные стороны для холистического переосмысления нашего подхода к дизрапшну.".into(),
                    "Давайте возьмём это оффлайн и синхронизируемся асинхронно, чтобы обеспечить максимальную пропускную способность нашего пайплайна.".into(),
                ],
                email_closings: vec![
                    "С уважением к нашей общей синергии,".into(),
                    "В духе непрерывного улучшения,".into(),
                    "Двигаясь вперёд вместе,".into(),
                ],
                review_strengths: vec![
                    "синергизирует командные процессы".into(),
                    "проактивно выравнивает стратегические инициативы".into(),
                    "демонстрирует холистический подход к задачам".into(),
                    "эффективно использует корпоративный жаргон".into(),
                ],
                review_improvements: vec![
                    "необходимо улучшить пропускную способность пайплайна".into(),
                    "следует больше дизраптить устаревшие парадигмы".into(),
                    "рекомендуется усилить кросс-функциональную синергию".into(),
                    "нужно активнее двигать иглу в правильном направлении".into(),
                ],
            },
        }
    }
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config").join("fuck").join("config.toml")
}

pub fn load() -> Config {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let parsed: Config = toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("⚠️  Ошибка парсинга конфига: {}. Используется дефолтный.", e);
            return Config::default();
        });
        merge_with_defaults(parsed)
    } else {
        let cfg = Config::default();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).ok();
        }
        fs::write(&path, toml::to_string_pretty(&cfg).unwrap()).ok();
        eprintln!("✨ Создан конфиг: {}", path.display());
        cfg
    }
}

/// Заполняет пустые поля значениями из дефолтного конфига.
fn merge_with_defaults(mut cfg: Config) -> Config {
    let d = Config::default();
    macro_rules! fill_vec {
        ($field:expr, $default:expr) => {
            if $field.is_empty() { $field = $default; }
        };
    }
    macro_rules! fill_str {
        ($field:expr, $default:expr) => {
            if $field.is_empty() { $field = $default; }
        };
    }

    fill_vec!(cfg.it.excuses,                  d.it.excuses);
    fill_vec!(cfg.off.polite,                   d.off.polite);
    fill_vec!(cfg.off.rude,                     d.off.rude);
    fill_vec!(cfg.up.steps,                     d.up.steps);
    fill_vec!(cfg.find_out.consequences,        d.find_out.consequences);
    fill_vec!(cfg.why.reasons,                  d.why.reasons);
    fill_vec!(cfg.why.culprits,                 d.why.culprits);
    fill_vec!(cfg.why.solutions,                d.why.solutions);
    fill_vec!(cfg.this.verdicts,                d.this.verdicts);
    fill_str!(cfg.this.lang_rust,               d.this.lang_rust);
    fill_str!(cfg.this.lang_node,               d.this.lang_node);
    fill_str!(cfg.this.lang_python,             d.this.lang_python);
    fill_str!(cfg.this.lang_unknown,            d.this.lang_unknown);
    fill_vec!(cfg.sv.hooli_quotes,              d.sv.hooli_quotes);
    fill_vec!(cfg.sv.erlich_openers,            d.sv.erlich_openers);
    fill_vec!(cfg.sv.erlich_middles,            d.sv.erlich_middles);
    fill_vec!(cfg.sv.erlich_claims,             d.sv.erlich_claims);
    fill_vec!(cfg.sv.standup_yesterdays,        d.sv.standup_yesterdays);
    fill_vec!(cfg.sv.standup_todays,            d.sv.standup_todays);
    fill_vec!(cfg.sv.standup_blockers,          d.sv.standup_blockers);
    fill_vec!(cfg.corp.meeting_topics,          d.corp.meeting_topics);
    fill_vec!(cfg.corp.meeting_attendees,       d.corp.meeting_attendees);
    fill_vec!(cfg.corp.meeting_outcomes,        d.corp.meeting_outcomes);
    fill_vec!(cfg.corp.email_subjects,          d.corp.email_subjects);
    fill_vec!(cfg.corp.email_greetings,         d.corp.email_greetings);
    fill_vec!(cfg.corp.email_bodies,            d.corp.email_bodies);
    fill_vec!(cfg.corp.email_closings,          d.corp.email_closings);
    fill_vec!(cfg.corp.review_strengths,        d.corp.review_strengths);
    fill_vec!(cfg.corp.review_improvements,     d.corp.review_improvements);
    cfg
}
