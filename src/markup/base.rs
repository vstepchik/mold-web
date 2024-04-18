use maud::{html, Markup, DOCTYPE};

const FAVICON_URL: &str = "/favicon.ico";
const CSS_URL: &str = "/main.css";

const LOGO_URL: &str = "/logo.svg";
const THEME_ICON_URL: &str = "/day-and-night.svg";
const JUMP_TO_TOP_ICON_URL: &str = "/top.svg";

pub fn template_base(is_night: bool, title: &str, head: Option<Markup>, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html#top.night[is_night] lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="shortcut icon" href=(FAVICON_URL) type="image/x-icon";
                link rel="stylesheet" href=(CSS_URL);
                title { (title) }
                @if let Some(head) = head {
                    (head)
                }
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
        header {
            a href=("/") {
                img#logo src=(LOGO_URL) alt="Logo" width="0" height="0";
            }
            img#theme-icon src=(THEME_ICON_URL) alt="Switch theme" width="0" height="0"
                onclick="document.cookie = \"night=\" + \
                document.documentElement.classList.toggle(\"night\") + \"; path=/; \
                expires=Fri, 31 Dec 9999 23:59:59 GMT\"";
        }
        div#jump-to-top {
            a href="#top" { img width="0" height="0" src=(JUMP_TO_TOP_ICON_URL) alt="Jump to top"; }
        }
    }
}
