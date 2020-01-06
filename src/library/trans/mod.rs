mod service;

use crate::library::trans::service::Translated;
pub use crate::library::trans::service::{Lang, Type};
pub use service::{GoogleTranslator, TranslationError};

pub type TranslateResult = Result<Translated, TranslationError>;

#[derive(Clone)]
pub struct Translator {
    _type: Type,
}

impl Translator {
    pub fn new(t: &Type) -> Self {
        Translator {
            _type: t.to_owned(),
        }
    }

    pub fn translate<T: Into<String>>(
        &self,
        text: T,
        s_lang: &Lang,
        t_lang: &Lang,
    ) -> TranslateResult {
        self._type
            .get_translator()
            .translate(&text.into(), &s_lang, &t_lang)
    }
}
