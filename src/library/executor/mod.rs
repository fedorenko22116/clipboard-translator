mod trans;
mod t_type;
mod error;
mod notifier;

use trans::{Translator, TranslateResult};
use trans::GoogleTranslator;
pub use t_type::Type;
use std::borrow::Borrow;
pub use error::ExecutorError;
use clipboard::{ClipboardProvider, ClipboardContext};
use notify_rust::{Notification, NotificationHandle, Error, NotificationHint};

pub type ExecutorResult<T> = Result<T, error::ExecutorError>;

pub struct ExecutorResultContext {
    pub text: String,
    pub lang: String,
    pub source_text: String,
    pub service: String,
}

pub struct Executor {
    selected_type: Type,
}

impl Executor {
    pub fn new(selected_type: &Type) -> Self {
        Executor {
            selected_type: selected_type.to_owned(),
        }
    }

    pub fn show_translation(&self) -> ExecutorResult<ExecutorResultContext> {
        let source_text = self.get_clipboard_text()?;
        let translator = self.selected_type.get_translator();
        let context = translator
            .translate(&source_text)
            .map(|t| ExecutorResultContext {
                service: translator.get_name(),
                lang: t.lang.to_string(),
                text: t.text,
                source_text,
            })
            .map_err(|err| ExecutorError::Translation(err))?;

        notifier::notify(&context);

        Ok(context)
    }

    fn get_clipboard_text(&self) -> ExecutorResult<String> {
        let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
        context
            .get_contents()
            .map_err(|err| ExecutorError::Clipboard(err.to_string()))
    }
}
