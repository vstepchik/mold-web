#![feature(plugin, proc_macro_hygiene)]
#![plugin(rocket_codegen)]

extern crate includedir;
extern crate maud;
extern crate phf;
extern crate rocket;

use maud::{DOCTYPE, html, Markup};
use rocket::Request;
use rocket::response::Stream;
use std::env;
use std::io::Cursor;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

#[get("/")]
fn index() -> Markup {
    template_base("Home", html! {
        p { "Hello" }
    })
}

#[get("/s/favicon.svg")]
fn favicon() -> Stream<Cursor<Vec<u8>>> {
    let data = FILES.get("data/favicon.svg").expect("Favicon not found").into_owned();
    Stream::from(Cursor::new(Vec::from(data)))
}

#[get("/s/style.css")]
fn style() -> Stream<Cursor<Vec<u8>>> {
    let data = FILES.get("data/style.css").expect("Stylesheet not found").into_owned();
    Stream::from(Cursor::new(Vec::from(data)))
}

#[catch(404)]
fn not_found(req: &Request) -> Markup {
    template_base("404", html! {
        h1 { "404: Hey! There's nothing here." }
        "The page at " (req.uri().as_str()) " does not exist!"
    })
}

fn template_base(title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                link rel="preload" href="/s/favicon.svg" as="image";
                link rel="preload" href="/s/style.css" as="style";
                link rel="stylesheet" href="/s/style.css";
                link rel="shortcut icon" href="/s/favicon.svg" type="image/x-icon";
                title { (title) }
            }
            body { (markup) }
        }
    }
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
