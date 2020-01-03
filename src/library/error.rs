use super::trans::TranslationError;
use std::{error::Error, fmt::Debug};

#[derive(Debug, Clone)]
pub enum ExecutorError {
    Translation(TranslationError),
    Clipboard(String),
    Notifier(String),
}

impl std::error::Error for ExecutorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExecutorError::Translation(err) => err.to_owned().to_string(),
                ExecutorError::Clipboard(err) => err.to_string(),
                ExecutorError::Notifier(err) => err.to_string(),
            }
        )
    }
}
