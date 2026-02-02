use crate::html::layout::Head;
use maud::{Markup, Render, html};

#[derive(Debug, Clone)]
pub struct Board {
    title: Option<String>,
    header: Option<Markup>,
    nav: Option<Markup>,
    main: Option<Markup>,
    footer: Option<Markup>,
}

impl Board {
    pub fn new(
        title: Option<String>,
        header: Option<Markup>,
        nav: Option<Markup>,
        main: Option<Markup>,
        footer: Option<Markup>,
    ) -> Self {
        Self {
            title,
            header,
            nav,
            main,
            footer,
        }
    }

    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn set_header(&mut self, header: Markup) -> &mut Self {
        self.header = Some(header);
        self
    }

    pub fn set_nav(&mut self, nav: Markup) -> &mut Self {
        self.nav = Some(nav);
        self
    }

    pub fn set_main(&mut self, main: Markup) -> &mut Self {
        self.main = Some(main);
        self
    }

    pub fn set_footer(&mut self, footer: Markup) -> &mut Self {
        self.footer = Some(footer);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_header(mut self, header: Markup) -> Self {
        self.header = Some(header);
        self
    }

    pub fn with_nav(mut self, nav: Markup) -> Self {
        self.nav = Some(nav);
        self
    }

    pub fn with_main(mut self, main: Markup) -> Self {
        self.main = Some(main);
        self
    }

    pub fn with_footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            title: None,
            header: None,
            nav: None,
            main: None,
            footer: None,
        }
    }
}

impl Render for Board {
    fn render(&self) -> Markup {
        let head = Head::default_with_title(self.title.clone());

        html! {
            (head)
            
            body {
                .page {
                    @if let Some(nav) = &self.nav {
                        nav.page_nav {
                            (nav)
                        }
                    }
                    @if let Some(header) = &self.header {
                        header.page_header {
                            (header)
                        }
                    }
                    @if let Some(main) = &self.main {
                        main.page_main {
                            (main)
                        }
                    }
                    @if let Some(footer) = &self.footer {
                        footer.page_footer {
                            (footer)
                        }
                    }
                }
            }
        }
    }
}
