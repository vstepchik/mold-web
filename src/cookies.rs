use rocket::http::Cookies;
use std::str::FromStr;

pub fn is_night_theme(cookies: Cookies) -> bool {
    const DEFAULT: bool = false;

    cookies.get("night")
        .map(|cookie| bool::from_str(cookie.value()).unwrap_or(DEFAULT))
        .unwrap_or(DEFAULT)
}
