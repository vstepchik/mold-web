use std::str::FromStr;

use actix_web::HttpRequest;

pub fn is_night_theme(req: &HttpRequest) -> bool {
    const DEFAULT: bool = false;

    req.cookie("night")
        .map(|cookie| bool::from_str(cookie.value()).unwrap_or(DEFAULT))
        .unwrap_or(DEFAULT)
}
