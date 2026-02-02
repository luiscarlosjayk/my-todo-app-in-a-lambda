use maud::{Markup, Render, html};

#[derive(Debug)]
pub struct Script {
    defer: bool,
    url: String,
}

impl Script {
    pub fn new_deferred(url: &str) -> Self {
        Self {
            defer: true,
            url: url.to_string(),
        }
    }
    
    pub fn new(url: &str, defer: bool) -> Self {
        Self {
            defer,
            url: url.to_string(),
        }
    }
}

impl Render for Script {
    fn render(&self) -> Markup {
        html! {
            script src=(self.url) defer=(self.defer) {}
        }
    }
}
