use maud::{DOCTYPE, html, Markup};

const FAVICON_URL: &'static str = "/favicon.svg";
const CSS_URL: &'static str = "/style.css";
const FONT_CSS_URL: &'static str = "https://fonts.googleapis.com/css?family=Open+Sans";

pub fn template_base(title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
//                link rel="preload" href=(FAVICON_URL) as="image";
//                link rel="preload" href=(FONT_CSS_URL) as="style";
//                link rel="preload" href=(CSS_URL) as="style";
                link rel="stylesheet" href=(CSS_URL);
                link rel="stylesheet" href=(FONT_CSS_URL);
                link rel="shortcut icon" href=(FAVICON_URL) type="image/x-icon";
                title { (title) }
            }
            body { (markup) }
        }
    }
}
