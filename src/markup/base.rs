use maud::{DOCTYPE, html, Markup};

const FAVICON_URL: &str = "/s/favicon.svg";
const CSS_URL: &str = "/s/style.css";
const FONT_CSS_URL: &str = "https://fonts.googleapis.com/css?family=Open+Sans";

const LOGO_URL: &str = FAVICON_URL;
const THEME_ICON_URL: &str = "/s/day-and-night.svg";

pub fn template_base(is_night: bool, title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html.night[is_night] lang="en" {
            head {
                meta charset="utf-8";
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
                img#logo src=(LOGO_URL);
            }
            img#themeIcon src=(THEME_ICON_URL)
                onclick="document.cookie = \"night=\" + \
                document.documentElement.classList.toggle(\"night\") + \"; path=/; \
                expires=Fri, 31 Dec 9999 23:59:59 GMT\"";
        }
    }
}
