use actix_web::middleware::{Compress, Logger};
use actix_web::{App, HttpServer};
use clap::Parser;

mod config;
mod cookies;
mod markup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let cfg = config::Config::parse();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(config::configuration)
    })
    .bind((cfg.address, cfg.http_port))?
    .run()
    .await
}

#[cfg(test)]
mod integration_tests {
    use actix_web::http::StatusCode;
    use actix_web::{http::header, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().configure(config::configuration)).await;

        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get(header::CONTENT_TYPE).unwrap(), "text/html; charset=utf-8");
    }

    #[actix_web::test]
    async fn test_robots_get() {
        let app = test::init_service(App::new().configure(config::configuration)).await;

        let req = test::TestRequest::default().uri("/robots.txt").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get(header::CONTENT_TYPE).unwrap(), "text/plain");
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains("User-agent:"));
        assert!(body_str.contains("Allow:"));
        assert!(body_str.contains("Disallow:"));
    }

    #[actix_web::test]
    async fn test_favicon_get() {
        let app = test::init_service(App::new().configure(config::configuration)).await;

        let req = test::TestRequest::default().uri("/favicon.ico").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get(header::CONTENT_TYPE).unwrap(), "image/x-icon");
        let body = test::read_body(resp).await;
        let file = std::fs::read("frontend/static/favicon.ico").unwrap();
        assert_eq!(body, file);
    }

    #[actix_web::test]
    async fn test_not_found_get() {
        let app = test::init_service(App::new().configure(config::configuration)).await;

        let req = test::TestRequest::default().uri("/unknown-resource").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        assert_eq!(resp.headers().get(header::CONTENT_TYPE).unwrap(), "text/html; charset=utf-8");
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.to_lowercase().contains("404"));
    }

    #[actix_web::test]
    async fn test_unresolfed_article_returns_not_found() {
        let app = test::init_service(App::new().configure(config::configuration)).await;

        let req = test::TestRequest::default().uri("/a/unknown-article.html").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        assert_eq!(resp.headers().get(header::CONTENT_TYPE).unwrap(), "text/html; charset=utf-8");
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.to_lowercase().contains("404"));
    }
}
