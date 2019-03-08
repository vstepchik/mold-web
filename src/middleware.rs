use std::fs::File;
use std::io::{BufRead, BufReader};

use actix_web::{http, HttpRequest, HttpResponse, Result};
use actix_web::http::StatusCode;
use actix_web::http::Uri;
use actix_web::http::uri::{PathAndQuery, Scheme};
use actix_web::middleware::{Middleware, Started};

use crate::ACME_KEY_PATH_VAR;
use crate::env_default;

pub struct RedirectToHttps;

pub struct AcmeChallengeResponder;

impl<S> Middleware<S> for RedirectToHttps {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let hostname = Self::get_hostname(req)
            .ok_or(actix_web::error::ErrorBadRequest("HTTP Host header is required"))?;
        let redirect_uri = Self::create_redirect_url(hostname.as_str(), req.uri())?;

        Ok(Started::Response(
            HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
                .header(http::header::LOCATION, redirect_uri.to_string())
                .finish(),
        ))
    }
}

impl RedirectToHttps {
    fn get_hostname<S>(req: &HttpRequest<S>) -> Option<String> {
        let con_info = req.connection_info();
        con_info.host()
            .split(':').next()
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
    }

    fn create_redirect_url(host: &str, uri: &Uri) -> Result<Uri> {
        let pq = PathAndQuery::from_static("/");
        let pq = uri.path_and_query().unwrap_or(&pq);
        Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(host)
            .path_and_query(pq.as_str())
            .build()
            .map_err(http::Error::into)
    }
}

impl<S> Middleware<S> for AcmeChallengeResponder {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let acme_token: Option<&str> = req.uri().path()
            .rsplitn(2, "/.well-known/acme-challenge/").next();

        let started = acme_token
            .and_then(Self::acme_check)
            .map(|validation| Started::Response(
                HttpResponse::build(StatusCode::OK).body(validation)
            ))
            .unwrap_or(Started::Done);

        Ok(started)
    }
}

impl AcmeChallengeResponder {
    fn acme_check(acme_token: &str) -> Option<String> {
        let acme_file = File::open(env_default(ACME_KEY_PATH_VAR, "acme.txt")).ok()?;

        let mut lines: Vec<String> = BufReader::new(acme_file).lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.trim().is_empty())
            .take(2)
            .collect();

        if lines.len() < 2 || lines[0].ne(acme_token) {
            return None;
        }

        Some(lines.remove(1))
    }
}
