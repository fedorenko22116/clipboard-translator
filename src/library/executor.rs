use crate::library::{error, notifier, trans};

use crate::library::storage::get_storage;
use clipboard::{ClipboardContext, ClipboardProvider};
pub use error::ExecutorError;
use std::borrow::Borrow;
use trans::{GoogleTranslator, Lang, TranslateResult, Translator, Type};

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

    pub fn execute_translate(
        &self,
        pl: &String,
        sl: &Option<String>,
        service: &Option<String>,
        not_notify: &bool,
        selected: &bool,
        copy: &bool,
    ) -> ExecutorResult<ExecutorResultContext> {
        let source_text = self.get_text(selected)?;
        let translator = Translator::new(&self.selected_type);

        let context = translator
            .translate(
                &source_text,
                &Lang::Custom(sl.to_owned().unwrap_or("auto".to_string())),
                &Lang::Custom(pl.to_owned()),
            )
            .map(|t| ExecutorResultContext {
                service: self.selected_type.to_string(),
                lang: t.lang.to_string(),
                text: t.text,
                source_text,
            })
            .map_err(|err| {
                notifier::notify(
                    "Error",
                    &self.selected_type.to_string(),
                    "Something went wrong!",
                )
                .unwrap_or_default();

                ExecutorError::Translation(err)
            })?;

        if context.lang.to_string().eq(pl) {
            if let Some(sl) = sl {
                return self.execute_translate(sl, &None, &service, &not_notify, &selected, &copy);
            }
        }

        if !*not_notify {
            notifier::notify(&context.service, &context.lang, &context.text)
                .map_err(|err| ExecutorError::Notifier(err.to_string()))?;
        }

        if *copy {
            let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
            clipboard.set_contents(context.text.to_owned());
        }

        Ok(context)
    }

    fn get_text(&self, use_selected: &bool) -> ExecutorResult<String> {
        let mut storage = get_storage(use_selected);
        storage
            .get()
            .map_err(|err| ExecutorError::Clipboard(err.to_string()))
    }
}
