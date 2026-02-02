use crate::html::templates::errors::DefaultErrorTemplate;
use maud::{Markup, Render, html};

#[derive(Debug)]
pub struct ErrorDefaultPage {
    pub title: Option<String>,
    pub message: Option<String>,
}

impl Render for ErrorDefaultPage {
    fn render(&self) -> Markup {
        let title = self.title.clone().unwrap_or("Error".to_string());
        let header = html! {
            div { "Header" }
        };
        let message = self.message.as_ref().map_or_else(|| "Error".to_string(), |m| m.to_string());
        let main = html! {
            (message)
        };
        let footer = html! {
            div { "Footer" }
        };
        let error_page = DefaultErrorTemplate::default()
            .with_title(title)
            .with_header(header)
            .with_footer(footer)
            .with_main(main);

        error_page.render()
    }
}
