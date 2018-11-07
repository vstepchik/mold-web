use crate::markup::base::template_base;
use maud::Markup;
use phf;
use std::fmt;

mod about_mold_web;

pub static ARTICLES: phf::Map<&'static str, &'static Article> = phf_map! {
    "about-mold-web" => &about_mold_web::AboutMoldWeb,
};

pub trait Article where Self: Sync {
    fn title(&self) -> &str;
    fn date(&self) -> Date;
    fn summary(&self) -> Markup;
    fn body(&self) -> Markup;
    fn render(&self, is_night: bool) -> Markup {
        template_base(is_night, self.title(), self.body())
    }
}

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        assert!(year > 2000 && year <= 2100);
        assert!(month > 0 && month <= 12);
        assert!(day > 0 && day <= 31);
        Date { year, month, day }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
