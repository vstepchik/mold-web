use crate::markup::base::template_base;
use maud::Markup;
use phf;

mod about_mold_web;

pub static ARTICLES: phf::Map<&'static str, &'static Article> = phf_map! {
    "about-mold-web" => &about_mold_web::AboutMoldWeb,
};

pub trait Article where Self: Sync {
    fn title(&self) -> &str;
    fn body(&self) -> Markup;
    fn render(&self, is_night: bool) -> Markup {
        template_base(is_night, self.title(), self.body())
    }
}
