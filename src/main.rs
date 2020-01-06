#[macro_use]
extern crate clap;

mod library;

use crate::library::{Command, EntryBuilder, Executor, Type};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    EntryBuilder::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .build()
        .start(|command| match command {
            Command::Translate(pl, sl, translator, not_notify, selected) => {
                Executor::new(&Type::from(translator.to_owned()))
                    .show_translation(&pl, &sl, &translator, &not_notify, &selected)
                    .map(|r| {
                        println!("{}", r.text);
                        ()
                    })
                    .map_err(|err| err.into())
            }
        })
}
