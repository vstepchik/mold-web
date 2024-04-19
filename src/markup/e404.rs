use actix_web::{HttpRequest, HttpResponse, http::header::ContentType};
use maud::html;

use crate::cookies::is_night_theme;
use crate::markup::base::template_base;

pub async fn e404(req: HttpRequest) -> HttpResponse {
    let content_markup = html! {
        h1 { "404: Hey! There's nothing here." }
        "The page at " samp { (req.uri()) } " does not exist!"
    };
    let markup = template_base(is_night_theme(&req), "404", None, content_markup);
    HttpResponse::NotFound()
        .content_type(ContentType::html())
        .body(markup.into_string())
}
