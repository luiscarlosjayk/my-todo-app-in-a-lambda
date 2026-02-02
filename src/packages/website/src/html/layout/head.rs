use crate::html::layout::Script;
use maud::{html, Markup, Render, DOCTYPE};

#[derive(Debug)]
pub struct Head {
    pub title: String,
    pub scripts: Vec<Script>,
}

impl Default for Head {
    fn default() -> Self {
        let title = Self::default_title();
        let scripts = Self::default_scripts();
        Self {
            title: title.to_string(),
            scripts,
        }
    }
}

impl Head {
    fn default_title() -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn default_scripts() -> Vec<Script> {
        let alpine_ajax_script = Script::new_deferred(
            "https://cdn.jsdelivr.net/npm/@imacrayon/alpine-ajax@0.12.6/dist/cdn.min.js",
        );
        let app_script = Script::new_deferred("/assets/input.iife.js");
        let alpine_core_script =
            Script::new_deferred("https://cdn.jsdelivr.net/npm/alpinejs@3.15.2/dist/cdn.min.js");
        vec![alpine_ajax_script, app_script, alpine_core_script]
    }

    pub fn default_with_title(title: Option<String>) -> Self {
        let title = title.map_or_else(Self::default_title, |title| title);
        let scripts = Self::default_scripts();
        Self { title, scripts }
    }
}

impl Render for Head {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    link rel="icon" type="image/x-icon" href="/assets/favicon.ico" title="favicon";
                    link rel="stylesheet" href="/assets/styles.css";
                    style {
                        "/* https://alpinejs.dev/directives/cloak */"
                        "[x-cloak] { display: none !important; }"
                    };

                    // Scripts
                    @for script in &self.scripts {
                        (script)
                    }

                    title { (&self.title) }
                }
            }
        }
    }
}
