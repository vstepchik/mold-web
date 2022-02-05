use std::str::FromStr;

use actix_web::{HttpMessage, HttpRequest};

pub fn is_night_theme(req: &HttpRequest) -> bool {
    req.cookie("night")
        .and_then(|cookie| bool::from_str(cookie.value()).ok())
        .unwrap_or(false)
}
