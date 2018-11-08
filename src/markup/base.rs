use maud::{DOCTYPE, html, Markup};

const FAVICON_URL: &str = "/favicon.ico";
const CSS_URL: &str = "/s/style.css";
const FONT_CSS_URL: &str = "https://fonts.googleapis.com/css?family=Open+Sans";

const LOGO_URL: &str = "/s/logo.svg";
const THEME_ICON_URL: &str = "/s/day-and-night.svg";
const JUMP_TO_TOP_ICON_URL: &str = "/s/top.svg";

pub fn template_base(is_night: bool, title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html#top.night[is_night] lang="en" {
            head {
                meta charset="utf-8";
                link rel="shortcut icon" href=(FAVICON_URL) type="image/x-icon";
                link rel="stylesheet" href=(FONT_CSS_URL);
                link rel="stylesheet" href=(CSS_URL);
                title { (title) }
            }
            body {
                (header())
                (markup)
            }
        }
    }
}

fn header() -> Markup {
    html! {
        div#header {
            a href=("/") {
                img#logo src=(LOGO_URL) width="0" height="0";
            }
            img#themeIcon src=(THEME_ICON_URL) width="0" height="0"
                onclick="document.cookie = \"night=\" + \
                document.documentElement.classList.toggle(\"night\") + \"; path=/; \
                expires=Fri, 31 Dec 9999 23:59:59 GMT\"";
        }
        div#jumpToTop {
            a href="#top" { img width="0" height="0" src=(JUMP_TO_TOP_ICON_URL); }
        }
    }
}
