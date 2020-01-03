mod entry;

pub use entry::EntryBuilder;

pub enum Command {
    Translate,
    SetPrimaryLanguage,
    SetSecondaryLanguage,
    SetTranslator,
    Info,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "translate" => Command::Translate,
            "set-primary-lang" => Command::SetPrimaryLanguage,
            "set-secondary-lang" => Command::SetSecondaryLanguage,
            "set-translator" => Command::SetTranslator,
            "info" => Command::Info,
            _ => panic!("Undefined command given")
        }
    }
}
