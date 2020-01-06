#[cfg(not(target_os = "windows"))]
use notify_rust::{Notification, NotificationHint};
use std::error::Error;

const TRANSLATOR_TITLE: &str = "Clipboard Translator";

#[cfg(target_os = "windows")]
pub fn notify<T, S, U>(service: T, lang: S, body: U) -> Result<(), Box<dyn Error>>
where
    T: Into<String>,
    S: Into<String>,
    U: Into<String>,
{
    unimplemented!()
}

#[cfg(not(target_os = "windows"))]
pub fn notify<T, S, U>(service: T, lang: S, body: U) -> Result<(), Box<dyn Error>>
where
    T: Into<String>,
    S: Into<String>,
    U: Into<String>,
{
    let body = body.into();

    Notification::new()
        .summary(TRANSLATOR_TITLE)
        .subtitle(&format!("{} ({})", service.into(), lang.into()))
        .body(&body)
        .icon("Info")
        .hint(NotificationHint::Category(body))
        .hint(NotificationHint::Resident(true))
        .show()
        .map_err(|err| err.into())
        .map(|r| ())
}
