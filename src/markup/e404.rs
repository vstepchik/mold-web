use actix_web::HttpRequest;
use maud::{html, Markup};

use crate::cookies::is_night_theme;
use crate::markup::base::template_base;

pub fn e404(req: &HttpRequest) -> Markup {
    template_base(is_night_theme(req), "404", None, html! {
        h1 { "404: Hey! There's nothing here." }
        "The page at " samp { (req.uri()) } " does not exist!"
    })
}
