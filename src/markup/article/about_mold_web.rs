use maud::{html, Markup};

use crate::markup::article::{Article, Date};
use crate::markup::icons::{icon, KOTLIN, PYTHON, RUST};

pub const ABOUT_MOLD_WEB: Article = Article {
    title: "About this site",
    date: Date { year: 2018, month: 11, day: 12 },
    summary: &summary,
    body: &body,
};

fn summary() -> Markup {
    html! {
        "The reason I started this project and some details about its implementation."
    }
}

fn body() -> Markup {
    html! {
            p {
                "So, the idea to start the project came into my head while I was on a vacation "
                "in Amsterdam. What a wonderful city! That magical and unreal combination of "
                "beautiful old architecture, high-tech and nature! \u{1f498}"
            }
            p {
                "I've long been interested in procedural content generation, especially map "
                "generation and simulation, mostly as theorist. But this time I was playing with "
                "the actual implementation of the map generation with python " (icon(PYTHON))
                " bla " (icon(KOTLIN)) (icon(PYTHON)) (icon(RUST))
            }
            p {
                "Blogging."
            }
            p {
                "Exotics."
            }
            p {
                "Rust. Rocket."
            }
            p {
                "Repository."
            }
            p {
                "Implementation. Bundled everything."
            }
            p {
                "Size optimization."
            }
            p {
                "Results."
            }
        }
}
