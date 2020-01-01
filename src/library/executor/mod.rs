mod trans;
mod t_type;
mod error;

use trans::{Translator, TranslateResult};
use trans::GoogleTranslator;
pub use t_type::Type;
use std::borrow::Borrow;
pub use error::ExecutorError;
use clipboard::{ClipboardProvider, ClipboardContext};
use notify_rust::{Notification, NotificationHandle, Error};

pub type ExecutorResult<T> = Result<T, error::ExecutorError>;

pub struct ExecutorResultContext {
    pub text: String,
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
        let translator = self.selected_type.get_translator();
        let source_text = self.get_clipboard_text()?;
        let context = translator
            .translate(&source_text)
            .map(|text| ExecutorResultContext {
                service: translator.get_name(),
                source_text,
                text
            })
            .map_err(|err| ExecutorError::Translation(err))?;

        self.notify(context)
    }

    fn get_clipboard_text(&self) -> ExecutorResult<String> {
        let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
        context
            .get_contents()
            .map_err(|err| ExecutorError::Clipboard(err.to_string()))
    }

    fn notify(&self, context: ExecutorResultContext) -> ExecutorResult<ExecutorResultContext> {
        Notification::new()
            .summary("Translator")
            .subtitle(&context.service)
            .body(&context.text)
            .icon("info")
            .show()
            .map_err(|err| ExecutorError::Notifier(err.to_string()))
            .map(|r| context)
    }
}
