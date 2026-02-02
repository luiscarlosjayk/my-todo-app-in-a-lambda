use crate::html::components::notifications::{DefaultNotification, Notification};
use maud::{Markup, Render, html};
use std::fmt::Debug;

pub struct NotificationList {
    pub notifications: Vec<Box<dyn Notification>>,
}

impl NotificationList {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }
    
    pub fn new_with_default_notification(message: impl Into<String>) -> Self {
        let notification = DefaultNotification::new(message);
        let mut noficiation_list = Self::new();
        noficiation_list.add_notification_owned(notification);
        noficiation_list
    }

    pub fn add_notification(&mut self, notification: Box<dyn Notification>) -> &mut Self {
        self.notifications.push(notification);
        self
    }

    pub fn add_notification_owned(&mut self, notification: impl Notification + 'static) -> &mut Self {
        self.notifications.push(Box::new(notification));
        self
    }
}

impl Debug for NotificationList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationList")
            .field("notifications", &self.notifications)
            .finish()
    }
}

impl Render for NotificationList {
    fn render(&self) -> Markup {
        html! {
            ul
                id="notifications"
                x-sync
                x-merge="prepend"
                role="status" {
                    @for notification in &self.notifications {
                        li {
                            (notification)
                        }
                    }
                }
        }
    }
}
