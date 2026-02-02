use maud::{Markup, Render, html};

#[derive(Debug, Clone)]
pub struct NavItem {
    href: String,
    text: String,
}

impl NavItem {
    pub fn new(href: String, text: String) -> Self {
        NavItem { href, text }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Nav {
    items: Vec<NavItem>,
    current_active: Option<u8>,
    heading: Option<String>,
}

impl Nav {
    pub fn new(items: Vec<NavItem>, current_active: Option<u8>, heading: Option<String>) -> Self {
        Nav {
            items,
            current_active,
            heading,
        }
    }

    pub fn set_active(&mut self, current_active: u8) -> &mut Self {
        self.current_active = Some(current_active);
        self
    }

    pub fn set_heading(&mut self, heading: &str) -> &mut Self {
        self.heading = Some(heading.to_string());
        self
    }

    pub fn set_current_to_none(&mut self) -> &mut Self {
        self.current_active = None;
        self
    }

    pub fn set_items(&mut self, items: Vec<NavItem>) -> &mut Self {
        self.items = items;
        self
    }

    pub fn with_items(mut self, items: Vec<NavItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_active(mut self, current_active: u8) -> Self {
        self.current_active = Some(current_active);
        self
    }

    pub fn add_item(&mut self, item: NavItem) -> &mut Self {
        self.items.push(item);
        self
    }

    pub fn with_heading(mut self, heading: &str) -> Self {
        self.heading = Some(heading.to_string());
        self
    }   
}

impl Render for Nav {
    fn render(&self) -> Markup {
        html! {
            div id="sidebar" {
                nav id="sidebar__nav" {
                    @if self.items.len() > 0 {
                        ul {
                            @if let Some(heading) = &self.heading {
                                li .menu-heading {
                                    (heading)
                                }
                            }
                            @for (i, item) in self.items.iter().enumerate() {
                                li {
                                    a href=[Some(&item.href)] .pointer .active[self.current_active == Some(i as u8)] {
                                        (item.text)
                                    }
                                }
                            }
                        }
                    } @else {
                        div {
                            "No items found in nav"
                        }
                    }
                }
            }
        }
    }
}
