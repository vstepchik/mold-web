use maud::{DOCTYPE, html, Markup};

const FAVICON_URL: &'static str = "/s/favicon.svg";
const CSS_URL: &'static str = "/s/style.css";
const FONT_CSS_URL: &'static str = "https://fonts.googleapis.com/css?family=Open+Sans";

const LOGO_URL: &'static str = FAVICON_URL;
const THEME_ICON_URL: &'static str = "/s/day-and-night.svg";

pub fn template_base(title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
//                link rel="preload" href=(FAVICON_URL) as="image";
//                link rel="preload" href=(FONT_CSS_URL) as="style";
//                link rel="preload" href=(CSS_URL) as="style";
                link rel="shortcut icon" href=(FAVICON_URL) type="image/x-icon";
                link rel="stylesheet" href=(CSS_URL);
                link rel="stylesheet" href=(FONT_CSS_URL);
                title { (title) }
            }
            body { (header()) (markup) }
        }
    }
}

fn header() -> Markup {
    html! {
        div id="header" {
            a href=("/") {
                img id="logo" src=(LOGO_URL);
            }
            a href=("/") {
                img id="themeIcon" src=(THEME_ICON_URL);
            }
        }
    }
}
