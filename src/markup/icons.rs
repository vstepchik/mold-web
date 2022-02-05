use maud::{html, Markup};

pub struct Icon {
    href: &'static str,
    alt: &'static str,
    link: &'static str,
}

pub const KOTLIN: Icon = Icon {
    href: "kotlin.svg",
    alt: "Kotlin",
    link: "https://kotlinlang.org/",
};
pub const PYTHON: Icon = Icon {
    href: "python.svg",
    alt: "Python",
    link: "https://www.python.org/",
};
pub const RUST: Icon = Icon {
    href: "rust.svg",
    alt: "Rust",
    link: "https://www.rust-lang.org/",
};

pub fn icon(ico: Icon) -> Markup {
    html! {
        a href=(ico.link) target="_blank" rel="noopener noreferrer" {
            img.icon src={ "/s/" (ico.href) } alt=(ico.alt);
        }
    }
}
