use std::env;
use std::net::Ipv4Addr;

use actix_web::web;
use actix_web_static_files::ResourceFiles;
use clap::Parser;

use crate::markup;

/// A Mold-Web server.
#[derive(Debug, Copy, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// IP-address to allow connections from
    #[arg(short, long, default_value_t = Config::default().address)]
    pub address: Ipv4Addr,

    /// Network port to listen for HTTP connections
    #[arg(short = 'p', long, default_value_t = Config::default().http_port)]
    pub http_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: Ipv4Addr::UNSPECIFIED,
            http_port: 8080,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn configuration(cfg: &mut web::ServiceConfig) {
    let generated = generate();

    cfg.route("/", web::get().to(markup::index))
        .service(
            web::resource("/a/{article_id}.html")
                .route(web::get().to(markup::article)),
        )
        .service(
            ResourceFiles::new("/", generated)
                .skip_handler_when_not_found()
                .do_not_resolve_defaults(),
        )
        .default_service(web::get().to(markup::e404_handler));
}
