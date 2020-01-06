use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    error::Error,
    process::{Command, Stdio},
};

pub trait Storage {
    fn get(&self) -> Result<String, Box<dyn Error>>;
}

pub struct Clipboard;

impl Storage for Clipboard {
    fn get(&self) -> Result<String, Box<dyn Error>> {
        let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
        context.get_contents()
    }
}

pub struct Selected;

impl Storage for Selected {
    fn get(&self) -> Result<String, Box<dyn Error>> {
        Command::new("xsel")
            .arg("-o")
            .stdout(Stdio::null())
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .map_err(|err| "'xsel' library is not installed".into())
    }
}

pub fn get_storage(use_selected: &bool) -> Box<dyn Storage> {
    if *use_selected {
        Box::new(Selected)
    } else {
        Box::new(Clipboard)
    }
}
