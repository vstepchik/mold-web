use crate::markup::base::template_base;
use maud::{html, Markup};
use super::Article;

pub struct AboutMoldWeb;

impl Article for AboutMoldWeb {
    fn render(&self) -> Markup {
        template_base("About mold-web", html! {
            p { "Sample article." }
        })
    }
}
