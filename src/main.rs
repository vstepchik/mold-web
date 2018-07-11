#![feature(plugin, proc_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

use maud::{DOCTYPE, html, Markup};
use rocket::Request;
use rocket::response::Redirect;

#[get("/")]
fn index() -> Markup {
    template_base("Home", html! {
        h3 "Home"
        p "sweet home"
    })
}

#[get("/index.html")]
fn index_html() -> Redirect {
    Redirect::to("/")
}

#[get("/hello/<name>")]
fn get(name: String) -> Markup {
    let items: Vec<String> = vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect();

    template_base("Demo", html! {
        h1 { "Hi " (name.as_str()) }
        h3 "Here are your items:"
        ul @for item in &items {li (item)}
        p {
            "Try going to "
            a href="/hello/YourName" "/hello/YourName";
        }
    })
}

#[error(404)]
fn not_found(req: &Request) -> Markup {
    template_base("404", html! {
        h1 "404: Hey! There's nothing here."
        "The page at " (req.uri().as_str()) " does not exist!"
    })
}

fn template_base(title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title (title)
            }
            body (markup)
        }
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
