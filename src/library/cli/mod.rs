mod entry;

use clap::ArgMatches;
pub use entry::EntryBuilder;

pub enum Command {
    Translate(String, Option<String>, Option<String>, bool, bool, bool),
}

impl<'a> From<(&str, Option<&ArgMatches<'a>>)> for Command {
    fn from(s: (&str, Option<&ArgMatches<'a>>)) -> Self {
        match s.0 {
            "translate" => Command::Translate(
                s.1.unwrap().value_of("primary-lang").unwrap().to_string(),
                s.1.unwrap()
                    .value_of("secondary-lang")
                    .map(|a| a.to_owned()),
                s.1.unwrap().value_of("translator").map(|a| a.to_owned()),
                s.1.unwrap().is_present("not-notify"),
                s.1.unwrap().is_present("selected"),
                s.1.unwrap().is_present("copy"),
            ),
            _ => panic!("Undefined command given"),
        }
    }
}
