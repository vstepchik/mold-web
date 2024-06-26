use actix_web::{HttpRequest, ResponseError};
use std::fmt;

use crate::cookies::is_night_theme;
use maud::{html, Markup};
use phf::phf_map;

use crate::error::UserError;
use crate::markup::base::template_base;

mod about_mold_web;
mod long_time_not_seen;

pub async fn article(req: HttpRequest) -> Result<Markup, impl ResponseError> {
    let article_id = req.match_info().get("article_id").unwrap();
    render_article(is_night_theme(&req), article_id).ok_or(UserError::ResourceNotFound { resource: req.uri().to_string(), is_night: is_night_theme(&req) })
}

fn render_article(is_night: bool, id: &str) -> Option<Markup> {
    ARTICLES.get(id).map(|article| article.render(is_night))
}

pub static ARTICLES: phf::Map<&'static str, &'static Article> = phf_map! {
    "about-mold-web" => &about_mold_web::ABOUT_MOLD_WEB,
    "long-time-not-seen" => &long_time_not_seen::LONG_TIME_NOT_SEEN,
};

pub struct Article<'a> {
    pub title: &'a str,
    pub date: Date,
    pub keywords: &'a [&'static str],
    pub summary: &'static (dyn Fn() -> Markup + Sync),
    pub body: &'static (dyn Fn() -> Markup + Sync),
}

impl<'a> Article<'a> {
    pub fn render(&self, is_night: bool) -> Markup {
        template_base(
            is_night,
            self.title,
            Some(html! {
                meta name="description" content=((self.summary)());
                meta name="keywords" content=(self.keywords.join(","));
            }),
            html! {
                article {
                    span.date { (self.date) }
                    h1 { (self.title) }
                    ((self.body)())
                }
            },
        )
    }
}

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
