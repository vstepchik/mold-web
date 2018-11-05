#![feature(plugin, proc_macro_hygiene)]
#![plugin(rocket_codegen)]

extern crate includedir;
extern crate maud;
extern crate phf;
extern crate rocket;

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

#[get("/favicon.svg")]
fn favicon() -> Stream<Cursor<Vec<u8>>> {
    let data = FILES.get("data/favicon.svg").expect("Favicon not found").into_owned();
    Stream::from(Cursor::new(Vec::from(data)))
}

#[get("/style.css")]
fn style() -> Stream<Cursor<Vec<u8>>> {
    let data = FILES.get("data/style.css").expect("Stylesheet not found").into_owned();
    Stream::from(Cursor::new(Vec::from(data)))
}

#[catch(404)]
fn not_found(req: &Request) -> Markup {
    markup::e404(req)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![favicon, style, index])
        .catch(catchers![not_found])
}

fn main() {
    FILES.set_passthrough(env::var_os("PASSTHROUGH").is_some());

    for name in FILES.file_names() {
        println!("Found: {}", name);
    }

    rocket().launch();
}
