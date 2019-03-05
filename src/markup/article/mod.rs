use std::fmt;

use maud::{html, Markup};
use phf::phf_map;

use crate::markup::base::template_base;

mod about_mold_web;

pub static ARTICLES: phf::Map<&'static str, &'static Article> = phf_map! {
    "about-mold-web" => &about_mold_web::ABOUT_MOLD_WEB,
};

pub struct Article<'a> {
    pub title: &'a str,
    pub date: Date,
    pub keywords: &'a [&'static str],
    pub summary: &'static (Fn() -> Markup + Sync),
    pub body: &'static (Fn() -> Markup + Sync),
}

impl<'a> Article<'a> {
    pub fn render(&self, is_night: bool) -> Markup {
        template_base(is_night, self.title,
                      Some(html! {
            meta name="description" content=((self.summary)());
            meta name="keywords" content=(self.keywords.join(","));
        }),
                      html! {
            span.date { (self.date) }
            h1 { (self.title) }
            ((self.body)())
        })
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
