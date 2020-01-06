mod error;
mod google;
mod mymemory;

use super::TranslateResult;
use crate::library::trans::service::mymemory::MyMemoryTranslator;
pub use error::TranslationError;
use fake_useragent::{Browsers, UserAgentsBuilder};
pub use google::GoogleTranslator;

pub trait TranslatorService {
    fn translate(&self, text: &String, s_lang: &Lang, t_lang: &Lang) -> TranslateResult;
    fn get_ua(&self) -> String {
        UserAgentsBuilder::new()
            .cache(false)
            .dir("/tmp")
            .thread(20)
            .set_browsers(Browsers::new().set_chrome().set_firefox().set_safari())
            .build()
            .random()
            .to_string()
    }
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
    MyMemory,
}

impl Type {
    pub(super) fn get_translator(&self) -> Box<dyn TranslatorService> {
        if let Type::MyMemory = self {
            return Box::new(MyMemoryTranslator);
        }

        Box::new(GoogleTranslator)
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
            "MyMemory" => Type::MyMemory,
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
                Type::MyMemory => "MyMemory",
            }
        )
    }
}

#[derive(Clone)]
pub enum Lang {
    Custom(String),
    Auto,
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lang::Custom(text) => text.to_lowercase(),
                Lang::Auto => "auto".to_string(),
            }
        )
    }
}
