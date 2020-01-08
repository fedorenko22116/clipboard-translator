#[derive(Debug, Clone)]
pub enum TranslationError {
    Connection,
    Directory,
}

impl std::error::Error for TranslationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TranslationError::Connection => "Cannot connect to the service host",
                TranslationError::Directory => "Cannot create a cache dir",
            }
        )
    }
}
