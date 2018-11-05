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

mod js;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

#[get("/")]
fn index() -> Markup {
    template_base("Home", html! {
        style {
            "*{margin:0;padding:0;}"
            "body{background:#222;}"
            "canvas{position:absolute;border:0 solid;box-shadow:inset 0 0 80px #4162a9;transform:translateZ(0);width:100%;height:100%;}"
        }
        canvas width="1000" height="1000" {}
        script type="text/javascript" src="/s/particles.js" {}
    })
}

#[get("/s/particles.js")]
fn particles() -> &'static str {
    js::PARTICLES
}

#[get("/s/favicon.svg")]
fn favicon() -> Stream<Cursor<Vec<u8>>> {
    let mut data = FILES.get("data/favicon.svg").expect("Favicon not found").into_owned();
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
                link rel="shortcut icon" href="/s/favicon.svg" type="image/x-icon";
                title { (title) }
            }
            body { (markup) }
        }
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![favicon, index, particles])
        .catch(catchers![not_found])
}

fn main() {
    FILES.set_passthrough(env::var_os("PASSTHROUGH").is_some());

    for name in FILES.file_names() {
        println!("Found: {}", name);
    }

    rocket().launch();
}
