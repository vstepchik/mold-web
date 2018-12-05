use maud::{html, Markup};

use crate::markup::ARTICLES;
use crate::markup::base::template_base;

const ARTICLE_URL_PREFIX: &'static str = "/a";

pub fn index(is_night: bool) -> Markup {
    template_base(
        is_night, "Mildew",
        Some(html! {
            meta name="description" content="A personal blog about procedural content generation and process simulation.";
            meta name="keywords" content="vitalatron,vstepchik,personal blog,blog,procedural content generation,procedural content,generator,simulation,process simulation,python,kotlin,rust,mold,mildew,mushroom";
        }),
        html! {
            @for (id, article) in ARTICLES.entries().rev() {
                div.article {
                    h3 { a href={ (ARTICLE_URL_PREFIX) "/" (id) } { (article.title) } }
                    p {
                        ((article.summary)())
                        span.date { (article.date) }
                    }
                }
            }
        })
}
