use actix_web::HttpRequest;
use actix_web::{http::header::ContentType, HttpResponse};
use maud::html;

use crate::cookies::is_night_theme;
use crate::markup::base::template_base;

pub async fn e404_handler(req: HttpRequest) -> HttpResponse {
    e404(req.uri().to_string().as_str(), is_night_theme(&req))
}

pub fn e404(resource: &str, is_night: bool) -> HttpResponse {
    let content_markup = html! {
        h1 { "Hey! There's nothing too look at." }
        p { small { "404 Not found" } }
        p { "The page at " samp { (resource) } " does not exist!" }
    };
    let markup = template_base(is_night, "404", None, content_markup);
    HttpResponse::NotFound()
        .content_type(ContentType::html())
        .body(markup.into_string())
}
