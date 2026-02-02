use crate::html::{
    components::nav::{Nav, NavItem},
    templates::Board,
};
use maud::{Markup, Render, html};

#[derive(Debug)]
pub struct Home;

impl Render for Home {
    fn render(&self) -> Markup {
        let title = "Home".to_string();
        let header = html! {
            div { "Header" }
        };
        let main = html! {
            div { "Main content" }
        };
        let footer = html! {
            div { "Footer" }
        };

        let home_nav_item = NavItem::new("/".to_string(), "Home".to_string());
        let list_boards_nav_item = NavItem::new("/board".to_string(), "Boards".to_string());
        let new_board_nav_item =
            NavItem::new("/board/create".to_string(), "Create board".to_string());
        let nav = Nav::default()
            .with_items(vec![
                home_nav_item,
                list_boards_nav_item,
                new_board_nav_item,
            ])
            .with_active(0)
            .with_heading("Menu");
        let board_page = Board::default()
            .with_title(title)
            .with_header(header)
            .with_nav(nav.render())
            .with_main(main)
            .with_footer(footer);

        board_page.render()
    }
}
