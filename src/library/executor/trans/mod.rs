mod google;
mod error;

pub use google::GoogleTranslator;
pub use error::TranslationError;

pub type TranslateResult = Result<String, TranslationError>;

pub trait Translator {
    fn translate(&self, text: &String) -> TranslateResult;
    fn get_name(&self) -> String;
}
