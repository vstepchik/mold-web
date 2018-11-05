use maud::{DOCTYPE, html, Markup};

pub fn template_base(title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                link rel="preload" href="/favicon.svg" as="image";
                link rel="preload" href="/style.css" as="style";
                link rel="stylesheet" href="/style.css";
                link rel="shortcut icon" href="/favicon.svg" type="image/x-icon";
                title { (title) }
            }
            body { (markup) }
        }
    }
}
