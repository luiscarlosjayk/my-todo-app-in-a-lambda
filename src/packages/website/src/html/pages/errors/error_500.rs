use maud::{Render, html, Markup};
use crate::html:: {
    templates::errors::Error500Template,
};

#[derive(Debug)]
pub struct Error500Page {
    pub message: Option<String>,
}

impl Render for Error500Page {
    fn render(&self) -> Markup {
        let title = "InternalError".to_string();
        let header = html! {
            div { "Header" }
        };
        let main = html! {
            "Oops. Esto no debió ocurrir."
        };
        let footer = html! {
            div { "Footer" }
        };
        let error_page = Error500Template::default()
            .with_title(title)
            .with_header(header)
            .with_footer(footer)
            .with_main(main);
        
        error_page.render()
    }
}
