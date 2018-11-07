use crate::markup::ARTICLES;
use crate::markup::base::template_base;
use maud::{html, Markup};

const ARTICLE_URL_PREFIX: &'static str = "/a";

pub fn index(is_night: bool) -> Markup {
    template_base(is_night, "Home", html! {
        h1 { "Hello" }
        @for (id, article) in ARTICLES.entries().rev() {
            div.article {
                h3 { a href={ (ARTICLE_URL_PREFIX) "/" (id) } { (article.title()) } }
                p {
                    span.date { (article.date()) }
                    (article.summary())
                }
            }
        }
    })
}
