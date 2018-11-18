use std::ffi::OsStr;
use std::io::Cursor;
use std::path::Path;

use actix_web::{App, Error, http, HttpRequest, HttpResponse, Responder, server};
use mime_guess::guess_mime_type;
use mime_guess::Mime;

pub struct StaticResource {
    data: Vec<u8>,
    mime: Mime,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

impl StaticResource {
    pub fn new(path: &str) -> Option<Self> {
        let mime = guess_mime_type(Path::new(path));

        FILES.get(format!("data/{}", path).as_str()).ok()
            .map(|data| StaticResource {
                data: Vec::from(data),
                mime,
            })
    }
}

impl Responder for StaticResource {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _: &HttpRequest<S>) -> Result<HttpResponse, Error> {
//        Response::build()
//            .sized_body(Cursor::new(self.data))
//            .header(self.content_type)
//            .raw_header("Cache-Control", "public, max-age=604800")
//            .ok();
        Ok(HttpResponse::Ok()
            .content_type(self.mime.as_ref())
            .body(self.data))
    }
}
