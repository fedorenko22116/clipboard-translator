use super::Command;
use clap::{App, AppSettings};
use std::{collections::HashMap, error::Error};

#[derive(Clone)]
pub struct EntryBuilder {
    parts: HashMap<String, String>,
}

impl EntryBuilder {
    pub fn new() -> Self {
        EntryBuilder {
            parts: HashMap::new(),
        }
    }

    pub fn name<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.parts
            .insert("name".to_string(), text.into())
            .unwrap_or_default();
        self
    }

    pub fn version<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.parts
            .insert("version".to_string(), text.into())
            .unwrap_or_default();
        self
    }

    pub fn author<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.parts
            .insert("author".to_string(), text.into())
            .unwrap_or_default();
        self
    }

    pub fn build(&self) -> Entry {
        Entry {
            name: self.parts.get("name").map(|r| r.to_owned()),
            author: self.parts.get("author").map(|r| r.to_owned()),
            version: self.parts.get("version").map(|r| r.to_owned()),
        }
    }
}

pub struct Entry {
    name: Option<String>,
    author: Option<String>,
    version: Option<String>,
}

impl Entry {
    pub fn start(
        &self,
        matcher: fn(Command) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), Box<dyn Error>> {
        let package_name = self
            .name
            .to_owned()
            .unwrap_or("translator".to_string())
            .to_owned();
        let authors = self
            .author
            .to_owned()
            .unwrap_or("anonymous".to_string())
            .to_owned();
        let version = self
            .version
            .to_owned()
            .unwrap_or("undefined".to_string())
            .to_owned();

        let yaml = load_yaml!("./config.yml");
        let matches = App::from_yaml(yaml)
            .setting(AppSettings::ArgRequiredElseHelp)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .name(package_name)
            .author(authors.as_str())
            .version(version.as_str())
            .get_matches();

        if let Some(command) = matches.subcommand_name() {
            matcher(Command::from(matches.subcommand()))?;
        }

        Ok(())
    }
}
