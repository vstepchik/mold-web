use crate::markup::base::template_base;
use maud::{html, Markup};
use rocket::Request;

pub fn e404(req: &Request) -> Markup {
    template_base("404", html! {
        h1 { "404: Hey! There's nothing here." }
        "The page at " (req.uri().as_str()) " does not exist!"
    })
}
