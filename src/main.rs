use crate::library::{Executor, Type, ExecutorError};
use std::error::Error;

mod library;

fn main() -> Result<(), ExecutorError> {
    Executor::new(&Type::Google)
        .show_translation()
        .map(|res| ())
}
