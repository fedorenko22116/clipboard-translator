#[macro_use]
extern crate clap;

mod library;

use crate::library::{EntryBuilder, Executor, Type, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    EntryBuilder::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .build()
        .start(|command| match command {
            Command::Translate => Executor::new(&Type::Google)
                .show_translation()
                .map(|_r| ())
                .map_err(|err| err.into()),
            Command::SetPrimaryLanguage => unimplemented!(),
            Command::SetSecondaryLanguage => unimplemented!(),
            Command::SetTranslator => unimplemented!(),
            Command::Info => unimplemented!()
        })
}
