use super::executor::ExecutorResultContext;
use notify_rust::{Notification, NotificationHandle, Error};

pub fn notify(context: ExecutorResultContext) -> Result<NotificationHandle, Error> {
    Notification::new()
        .summary("Translator")
        .subtitle(&context.service)
        .body(&context.text)
        .icon("info")
        .show()
}
