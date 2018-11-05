#![feature(plugin, proc_macro_hygiene)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

mod js;
use maud::{DOCTYPE, html, Markup};
use rocket::Request;

#[get("/")]
fn index() -> Markup {
    template_base("Home", html! {
        style {
            "*{margin:0;padding:0;}"
            "body{background:#222;}"
            "canvas{position:absolute;border:0 solid;box-shadow:inset 0 0 80px #4162a9;transform:translateZ(0);width:100%;height:100%;}"
        }
        canvas width="1000" height="1000" {}
        script type="text/javascript" src="particles.js" {}
    })
}

#[get("/particles.js")]
fn script() -> &'static str {
    js::PARTICLES
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
                title { (title) }
            }
            body { (markup) }
        }
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, script])
        .catch(catchers![not_found])
}

fn main() {
    rocket().launch();
}
