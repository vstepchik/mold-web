#![feature(plugin, proc_macro_hygiene)]
#![plugin(rocket_codegen, phf_macros)]

extern crate includedir;
extern crate maud;
extern crate phf;
extern crate rocket;

use maud::Markup;
use rocket::http::Cookies;
use rocket::Request;

use crate::markup::ARTICLES;
use crate::static_res::StaticResource;

mod markup;
mod cookies;
mod static_res;


#[get("/")]
fn index(cookies: Cookies) -> Markup {
    let is_night = cookies::is_night_theme(cookies);
    markup::index(is_night)
}

#[get("/a/<id>")]
fn article(id: String, cookies: Cookies) -> Option<Markup> {
    let is_night = cookies::is_night_theme(cookies);
    ARTICLES.get(id.as_str()).map(|a| a.render(is_night))
}

#[get("/s/<file>")]
fn static_res(file: String) -> Option<StaticResource> {
    StaticResource::new(file.as_str())
}

#[get("/robots.txt")]
fn robots() -> &'static str {
    "User-agent: *\nDisallow:\nAllow: /\n"
}

#[get("/favicon.ico")]
fn favicon() -> Option<StaticResource> {
    StaticResource::new("favicon.ico")
}

#[catch(404)]
fn not_found(req: &Request) -> Markup {
    markup::e404(req)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![static_res, robots, favicon, index, article])
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
