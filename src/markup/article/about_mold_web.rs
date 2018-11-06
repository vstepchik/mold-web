use crate::markup::base::template_base;
use maud::{html, Markup};
use super::Article;

pub struct AboutMoldWeb;

impl Article for AboutMoldWeb {
    fn render(&self, is_night: bool) -> Markup {
        template_base(is_night, "About mold-web", html! {
            p { "Sample article." }
        })
    }
}
