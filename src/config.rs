use std::collections::HashMap;
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
            address: Ipv4Addr::LOCALHOST,
            http_port: 8080,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn configuration(cfg: &mut web::ServiceConfig) {
    let generated = generate();
    let mut files_at_root: HashMap<&str, static_files::Resource> = HashMap::new();
    let robots = generated.get("robots.txt").unwrap();
    let robots = static_files::resource::new_resource(robots.data, robots.modified, robots.mime_type);
    let favicon = generated.get("favicon.ico").unwrap();
    let favicon = static_files::resource::new_resource(favicon.data, favicon.modified, favicon.mime_type);
    files_at_root.insert("robots.txt", robots);
    files_at_root.insert("favicon.ico", favicon); // todo: fix it the proper way

    cfg.route("/", web::get().to(markup::index))
        .route("/a/{article_id}", web::get().to(markup::article))
        .service(ResourceFiles::new("/s", generated).do_not_resolve_defaults())
        .service(ResourceFiles::new("/", files_at_root).do_not_resolve_defaults())
        .default_service(web::route().to(markup::e404));
}
