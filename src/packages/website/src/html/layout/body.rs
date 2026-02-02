use maud::{Markup, html};

pub fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)
        }
    }
}
