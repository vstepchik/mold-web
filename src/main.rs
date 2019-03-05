#![feature(proc_macro_hygiene)]

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate includedir;
extern crate maud;
extern crate mime_guess;
extern crate phf;

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use actix_web::{App, http, HttpRequest, HttpResponse, Result, server};
use actix_web::http::header;
use actix_web::middleware::{ErrorHandlers, Logger, Response};
use actix_web::middleware::DefaultHeaders;
use bytes::Bytes;
use maud::Markup;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::markup::ARTICLES;
use crate::static_res::StaticResource;

mod markup;
mod cookies;
mod static_res;
mod middleware;

const LOG_ENV_VAR: &str = "RUST_LOG";
const CERT_LOCATION_VAR: &str = "TLS_CERT";
const KEY_LOCATION_VAR: &str = "TLS_KEY";
const ACME_KEY_PATH_VAR: &str = "ACME_KEY_PATH";


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

fn env_default(env_var: &str, default: &str) -> String {
    env::var_os(env_var).unwrap_or_else(|| OsString::from(default))
        .into_string().unwrap_or_else(|_| default.to_owned())
}

fn create_app() -> App {
    App::new()
        .middleware(Logger::default())
        .middleware(DefaultHeaders::new()
            .header(
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security
                http::header::STRICT_TRANSPORT_SECURITY,
                "max-age=63072000; includeSubDomains; preload",
            )
        )
        .middleware(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, not_found))
        .resource("/", |r| r.f(index))
        .resource("/s/{file}", |r| r.f(static_res))
        .resource("/robots.txt", |r| r.f(robots))
        .resource("/favicon.ico", |r| r.f(favicon))
        .resource("/a/{id}", |r| r.f(article))
}

fn create_redirect_app() -> App {
    App::new()
        .middleware(Logger::default())
        .middleware(middleware::AcmeChallengeResponder)
        .middleware(middleware::RedirectToHttps)
}

fn main() {
    if env::var_os(LOG_ENV_VAR).is_none() {
        env::set_var(LOG_ENV_VAR, "info");
    }
    env_logger::init();

    let sys = actix::System::new("mold-web");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(env_default(KEY_LOCATION_VAR, "key.pem"), SslFiletype::PEM)
        .expect("Key not loaded");
    builder.set_certificate_chain_file(env_default(CERT_LOCATION_VAR, "cert.pem"))
        .expect("Certificate not loaded");

    let socket = "0.0.0.0:8443";
    server::new(|| create_app())
        .bind_ssl(socket, builder).expect(format!("Unable to bind socket {:?}", socket).as_str())
        .keep_alive(10)
        .start();

    server::new(|| create_redirect_app())
        .bind("0.0.0.0:8080").unwrap()
        .workers(1)
        .start();

    let _ = sys.run();
}
