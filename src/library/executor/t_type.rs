use super::trans::{TranslateResult, Translator, GoogleTranslator};

#[derive(Debug, Clone)]
pub enum Type {
    Google,
}

impl Type {
    pub fn get_translator(&self) -> Box<dyn Translator> {
        Box::new(
            match self {
                Type::Google => GoogleTranslator,
            }
        )
    }
}