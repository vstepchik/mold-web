use crate::markup::ARTICLES;
use crate::markup::base::template_base;
use maud::{html, Markup};

const ARTICLE_URL_PREFIX: &'static str = "/a";

pub fn index() -> Markup {
    template_base("Home", html! {
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
