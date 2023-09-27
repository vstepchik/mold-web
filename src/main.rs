use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use actix_web_static_files::ResourceFiles;
use std::collections::HashMap;
use std::env;

mod cookies;
mod markup;

const PORT_VAR: &str = "PORT";

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port = env::var_os(PORT_VAR)
        .map(|v| {
            v.to_string_lossy()
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Invalid port: {:?}", v))
        })
        .unwrap_or(8080);

    HttpServer::new(move || {
        let generated = generate();
        let mut files_at_root: HashMap<&str, static_files::Resource> = HashMap::new();
        let robots = generated.get("robots.txt").unwrap();
        let robots = static_files::resource::new_resource(robots.data, robots.modified, robots.mime_type);
        let favicon = generated.get("favicon.ico").unwrap();
        let favicon = static_files::resource::new_resource(favicon.data, favicon.modified, favicon.mime_type);
        files_at_root.insert("robots.txt", robots);
        files_at_root.insert("favicon.ico", favicon); // todo: fix it the proper way
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .route("/", web::get().to(markup::index))
            .route("/a/{article_id}", web::get().to(markup::article))
            .service(ResourceFiles::new("/s", generated).do_not_resolve_defaults())
            .service(ResourceFiles::new("/", files_at_root).do_not_resolve_defaults())
            .default_service(web::route().to(markup::e404))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
