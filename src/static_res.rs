use std::ffi::OsStr;
use std::io::Cursor;
use std::path::Path;

use actix_web::{App, Error, http, HttpRequest, HttpResponse, Responder, server};

pub struct StaticResource {
    data: Vec<u8>,
//    content_type: ContentType,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

impl StaticResource {
    pub fn new(path: &str) -> Option<Self> {
//        let content_type = Path::new(path)
//            .extension()
//            .and_then(OsStr::to_str)
//            .and_then(|ext| ContentType::from_extension(ext))
//            .unwrap_or(ContentType::Binary);

        FILES.get(format!("data/{}", path).as_str()).ok()
            .map(|data| StaticResource {
                data: Vec::from(data),
//                content_type,
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
//            .content_type("application/json")
            .body(self.data))
    }
}
