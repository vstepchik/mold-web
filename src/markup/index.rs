use crate::markup::ARTICLES;
use crate::markup::base::template_base;
use maud::{html, Markup};

const ARTICLE_URL_PREFIX: &'static str = "/a";

pub fn index(is_night: bool) -> Markup {
    template_base(is_night, "Home", html! {
        p { "Hello" }
        ul {
            @for (id, _article) in ARTICLES.entries().rev() {
                li {
                    a href={ (ARTICLE_URL_PREFIX) "/" (id) } { (id) }
                }
            }
        }
    })
}
