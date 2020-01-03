use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub p_lang: String,
    pub s_lang: String,
    pub service: String,
}

impl Config {
    pub fn serialize() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    pub fn deserialize() -> Result<Self, Box<dyn Error>> {
        unimplemented!()
    }

    pub fn set_primary_lang<T: Into<String>>(&mut self, lang: T) -> &mut Self {
        unimplemented!()
    }

    pub fn set_secondary_lang<T: Into<String>>(&mut self, lang: T) -> &mut Self {
        unimplemented!()
    }

    pub fn set_translator<T: Into<String>>(&mut self, service: T) -> &mut Self {
        unimplemented!()
    }
}
