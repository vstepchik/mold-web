use crate::cookies::is_night_theme;
use actix_web::HttpRequest;
use maud::{html, Markup};

use crate::markup::base::template_base;
use crate::markup::ARTICLES;

const ARTICLE_URL_PREFIX: &str = "/a";

pub async fn index(req: HttpRequest) -> Markup {
    let head_markup = html! {
        meta name="description" content="A personal blog about procedural content generation and process simulation.";
        meta name="keywords" content="vitalatron,vstepchik,personal blog,blog,procedural content generation,procedural content,generator,simulation,process simulation,python,kotlin,rust,mold,mildew,mushroom";
    };

    let content_markup = html! {
        @for (id, article) in ARTICLES.entries().rev() {
            div.article {
                h3 { a href={ (ARTICLE_URL_PREFIX) "/" (id) } { (article.title) } }
                p {
                    ((article.summary)())
                    span.date { (article.date) }
                }
            }
        }
    };

    template_base(is_night_theme(&req), "Mildew", Some(head_markup), content_markup)
}
