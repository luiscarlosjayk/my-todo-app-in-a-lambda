use crate::html::pages;
use actix_web::{Result, get};
use maud::{Markup, Render};

#[get("/")]
pub async fn handler() -> Result<Markup> {
    let page = pages::home::Home;
    Ok(page.render())
}
