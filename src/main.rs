#![feature(plugin, proc_macro_hygiene)]
#![plugin(phf_macros)]

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate includedir;
extern crate maud;
extern crate mime_guess;
extern crate phf;
extern crate rustls;

use std::fs::File;
use std::io::BufReader;

use actix_web::{App, http, HttpRequest, HttpResponse, Result, server};
use actix_web::http::header;
use actix_web::middleware::{ErrorHandlers, Logger, Response};
use bytes::Bytes;
use maud::Markup;
use rustls::{NoClientAuth, ServerConfig};
use rustls::internal::pemfile::{certs, rsa_private_keys};

use crate::markup::ARTICLES;
use crate::static_res::StaticResource;

mod markup;
mod cookies;
mod static_res;


fn index(req: &HttpRequest) -> Markup {
    let is_night = cookies::is_night_theme(req);
    markup::index(is_night)
}

fn article(req: &HttpRequest) -> Option<Markup> {
    let id: String = req.match_info().query("id").unwrap_or(String::new());
    let is_night = cookies::is_night_theme(req);
    ARTICLES.get(id.as_str()).map(|a| a.render(is_night))
}

fn static_res(req: &HttpRequest) -> Option<StaticResource> {
    let file: String = req.match_info().query("file").unwrap_or(String::new());
    StaticResource::new(file.as_str())
}

fn robots(_req: &HttpRequest) -> &'static str {
    "User-agent: *\nDisallow:\nAllow: /\n"
}

fn favicon(_req: &HttpRequest) -> Option<StaticResource> {
    StaticResource::new("favicon.ico")
}

fn not_found(req: &HttpRequest, resp: HttpResponse) -> Result<Response> {
    let body = Bytes::from(markup::e404(req).into_string());
    let mut builder = resp.into_builder();
    builder.header(http::header::CONTENT_TYPE, header::ContentType::html());
    Ok(Response::Done(builder.body(body)))
}

fn create_app() -> App {
    App::new()
        .middleware(Logger::default())
        .middleware(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, not_found))
        .resource("/", |r| r.f(index))
        .resource("/s/{file}", |r| r.f(static_res))
        .resource("/robots.txt", |r| r.f(robots))
        .resource("/favicon.ico", |r| r.f(favicon))
        .resource("/a/{id}", |r| r.f(article))
}

fn main() {
    const LOG_ENV_VAR: &str = "RUST_LOG";
    let socket = "0.0.0.0:8443";

    if std::env::var_os(LOG_ENV_VAR).is_none() {
        std::env::set_var(LOG_ENV_VAR, "info");
    }
    env_logger::init();

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").expect("cert.pem not found"));
    let key_file = &mut BufReader::new(File::open("key.pem").expect("key.pem not found"));
    let cert_chain = certs(cert_file).expect("failed to read cert file");
    let mut keys = rsa_private_keys(key_file).expect("failed to read keys file");
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // actix acceptor
    let acceptor = server::RustlsAcceptor::with_flags(
        config,
        server::ServerFlags::HTTP1 | server::ServerFlags::HTTP2,
    );

    server::new(|| create_app())
        .bind_width(socket, move || acceptor.clone())
        .expect("Unable to bind socket")
        .keep_alive(10)
        .run();
}
