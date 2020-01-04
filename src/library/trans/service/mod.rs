mod error;
mod google;

use super::TranslateResult;
pub use error::TranslationError;
pub use google::GoogleTranslator;

pub trait TranslatorService {
    fn translate(&self, text: &String, lang: &Lang) -> TranslateResult;
}

pub struct Translated {
    pub text: String,
    pub lang: Lang,
}

impl Translated {
    pub fn new(text: &String, lang: &Lang) -> Self {
        Translated {
            text: text.to_owned(),
            lang: lang.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Google,
}

impl Type {
    pub(super) fn get_translator(&self) -> Box<dyn TranslatorService> {
        Box::new(match self {
            Type::Google => GoogleTranslator,
        })
    }
}

impl<T: Into<String>> From<Option<T>> for Type {
    fn from(t: Option<T>) -> Self {
        if let None = t {
            return Type::Google;
        }

        let t = t.unwrap().into();

        match t.as_str() {
            "Google" => Type::Google,
            _ => panic!("Unimplemented translator type"),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Google => "Google",
            }
        )
    }
}

#[derive(Clone)]
pub enum Lang {
    Custom(String),
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lang::Custom(text) => text.to_lowercase(),
            }
        )
    }
}
