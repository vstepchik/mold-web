#![feature(plugin, proc_macro_hygiene)]
#![plugin(phf_macros)]

extern crate bytes;
extern crate includedir;
extern crate maud;
extern crate phf;

use actix_web::App;
use actix_web::http;
use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::middleware::ErrorHandlers;
//use crate::static_res::StaticResource;
use actix_web::middleware::Logger;
use actix_web::middleware::Response;
use actix_web::Result;
use actix_web::server;
use bytes::Bytes;
use maud::Markup;

use crate::markup::ARTICLES;

mod markup;
mod cookies;
//mod static_res;


fn index(req: &HttpRequest) -> Markup {
    let is_night = cookies::is_night_theme(req);
    markup::index(is_night)
}

fn article(req: &HttpRequest) -> Option<Markup> {
    let id: String = req.match_info().query("id").unwrap_or(String::new());
    let is_night = cookies::is_night_theme(req);
    ARTICLES.get(id.as_str()).map(|a| a.render(is_night))
}

//#[get("/s/<file>")]
//fn static_res(file: String) -> Option<StaticResource> {
//    StaticResource::new(file.as_str())
//}

fn robots(_req: &HttpRequest) -> &'static str {
    "User-agent: *\nDisallow:\nAllow: /\n"
}

//#[get("/favicon.ico")]
//fn favicon() -> Option<StaticResource> {
//    StaticResource::new("favicon.ico")
//}

fn not_found(req: &HttpRequest, resp: HttpResponse) -> Result<Response> {
    let body = Bytes::from(markup::e404(req).into_string());
    let mut builder = resp.into_builder();
    builder.header(http::header::CONTENT_TYPE, ContentType::html());
    Ok(Response::Done(builder.body(body)))
}

fn create_app() -> App {
    App::new()
        .middleware(Logger::default())
        .middleware(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, not_found))
        .resource("/", |r| r.f(index))
//        .resource("/s/{file}", |r| r.f(static_res))
        .resource("/robots.txt", |r| r.f(robots))
//        .resource("/favicon.ico", |r| r.f(favicon))
        .resource("/a/{id}", |r| r.f(article))
}

fn main() {
    server::new(|| create_app())
        .bind("127.0.0.1:8000")
        .expect("Unable to bind socket")
        .keep_alive(10)
        .run();
}
