#![feature(plugin, proc_macro_hygiene)]
#![plugin(rocket_codegen, phf_macros)]

extern crate includedir;
extern crate maud;
extern crate phf;
extern crate rocket;

use crate::markup::ARTICLES;
use maud::Markup;
use rocket::Request;
use rocket::response::Stream;
use std::env;
use std::io::Cursor;

mod markup;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

#[get("/")]
fn index() -> Markup {
    markup::index()
}

#[get("/a/<id>")]
fn article(id: String) -> Option<Markup> {
    ARTICLES.get(id.as_str()).map(|a| a.render())
}

#[get("/s/<file>")]
fn static_res(file: String) -> Option<Stream<Cursor<Vec<u8>>>> {
    let file = format!("data/{}", file);
    FILES.get(file.as_str()).map(|data| Stream::from(Cursor::new(Vec::from(data.into_owned())))).ok()
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
