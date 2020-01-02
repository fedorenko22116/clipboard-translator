mod google;
mod error;

pub use google::GoogleTranslator;
pub use error::TranslationError;

pub type TranslateResult = Result<Translated, TranslationError>;

pub trait Translator {
    fn translate(&self, text: &String) -> TranslateResult;
    fn get_name(&self) -> String;
}

pub struct Translated {
    pub text: String,
    pub lang: Lang
}

impl Translated {
    pub fn new (text: &String, lang: &Lang) -> Self {
        Translated {
            text: text.to_owned(),
            lang: lang.to_owned()
        }
    }
}

#[derive(Clone)]
pub enum Lang {
    Custom(String)
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lang::Custom(text) => text.to_lowercase()
            }
        )
    }
}
