use maud::{Markup, Render, html};
use super::Notification;

#[derive(Debug)]
pub struct DefaultNotification {
    pub message: String,
}

impl DefaultNotification {
    pub fn new(message: impl Into<String>) -> Self {
            DefaultNotification { message: message.into() }
        }
}

impl Notification for DefaultNotification {}

impl Render for DefaultNotification {
    fn render(&self) -> Markup {
        html! {
            li {
                span {
                    (self.message)
                }
            }
        }
    }
}
