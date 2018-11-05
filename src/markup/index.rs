use crate::markup::base::template_base;
use maud::{html, Markup};

pub fn index() -> Markup {
    template_base("Home", html! {
        p { "Hello" }
    })
}
