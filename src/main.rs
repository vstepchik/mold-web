#![feature(plugin, proc_macro_hygiene)]
#![plugin(rocket_codegen, phf_macros)]

extern crate includedir;
extern crate maud;
extern crate phf;
extern crate rocket;

use crate::markup::ARTICLES;
use maud::Markup;
use rocket::http::Cookies;
use rocket::Request;
use rocket::response::Stream;
use std::env;
use std::io::Cursor;

mod markup;
mod cookies;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

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
fn static_res(file: String) -> Option<Stream<Cursor<Vec<u8>>>> {
    FILES.get(format!("data/{}", file).as_str()).ok()
        .map(|data| Stream::from(Cursor::new(Vec::from(data.into_owned()))))
}

#[catch(404)]
fn not_found(req: &Request) -> Markup {
    markup::e404(req)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![static_res, index, article])
        .catch(catchers![not_found])
}

fn main() {
    FILES.set_passthrough(env::var_os("PASSTHROUGH").is_some());

    for name in FILES.file_names() {
        println!("Found: {}", name);
    }

    rocket().launch();
}
