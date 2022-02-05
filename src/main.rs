use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use actix_web_static_files::ResourceFiles;
use std::env;

mod cookies;
mod markup;

const DISABLE_HTTPS_VAR: &str = "DISABLE_HTTPS";
const PORT_VAR: &str = "PORT";
// const CERT_LOCATION_VAR: &str = "TLS_CERT";
// const KEY_LOCATION_VAR: &str = "TLS_KEY";
// const CA_LOCATION_VAR: &str = "TLS_CA";
// const ACME_KEY_PATH_VAR: &str = "ACME_KEY_PATH";

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let disable_https = env::var_os(DISABLE_HTTPS_VAR)
        .and_then(|v| v.to_string_lossy().to_ascii_lowercase().parse::<bool>().ok())
        .unwrap_or(false);
    let port = env::var_os(PORT_VAR)
        .map(|v| {
            v.to_string_lossy()
                .parse::<u16>()
                .unwrap_or_else(|_| panic!("Invalid port: {:?}", v))
        })
        .unwrap_or(8080);

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .wrap(Logger::default())
            .wrap(RedirectSchemeBuilder::new().enable(!disable_https).build()) // redirect HTTP -> HTTPS
            .wrap(Compress::default())
            .route("/", web::get().to(greet))
            .route("/name/{name}", web::get().to(greet))
            .service(ResourceFiles::new("/s", generated).do_not_resolve_defaults())
            .default_service(web::route().to(markup::e404))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
