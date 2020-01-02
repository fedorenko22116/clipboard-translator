use super::ExecutorResultContext;
use super::ExecutorResult;
use super::ExecutorError;
use notify_rust::{Notification, NotificationHint};

const TRANSLATOR_TITLE: &str = "Clipboard Translator";

#[cfg(target_os = "windows")]
pub fn notify(context: &ExecutorResultContext) -> ExecutorResult<()> {
    notifica::notify(TRANSLATOR_TITLE, &context.text);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn notify(context: &ExecutorResultContext) -> ExecutorResult<()> {
    Notification::new()
        .summary(TRANSLATOR_TITLE)
        .subtitle(&format!("{} ({})", context.service, context.lang))
        .body(&context.text)
        .icon("Info")
        .hint(NotificationHint::Category("translation".to_owned()))
        .hint(NotificationHint::Resident(true))
        .show()
        .map_err(|err| ExecutorError::Notifier(err.to_string()))
        .map(|r| ())
}