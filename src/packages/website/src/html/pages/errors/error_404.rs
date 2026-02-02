use crate::html::templates::errors::Error404Template;
use maud::{Markup, Render, html};

#[derive(Debug)]
pub struct Error404Page {
    pub message: Option<String>,
}

impl Render for Error404Page {
    fn render(&self) -> Markup {
        let title = "NotFound".to_string();
        let header = html! {
            div { "Header" }
        };
        let main = html! {
            "Not found"
        };
        let footer = html! {
            div { "Footer" }
        };
        let error_page = Error404Template::default()
            .with_title(title)
            .with_header(header)
            .with_footer(footer)
            .with_main(main);

        error_page.render()
    }
}
