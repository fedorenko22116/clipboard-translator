use super::Translator;
use super::error::TranslationError;

pub struct GoogleTranslator;

impl Translator for GoogleTranslator {
    fn translate(&self, text: &String) -> Result<String, TranslationError> {
        unimplemented!()
    }

    fn get_name(&self) -> String {
        "Google".to_string()
    }
}
