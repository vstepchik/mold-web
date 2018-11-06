use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::ffi::OsStr;
use std::io::Cursor;
use std::path::Path;

pub struct StaticResource {
    data: Vec<u8>,
    content_type: ContentType,
}

include!(concat!(env!("OUT_DIR"), "/data.rs"));

impl StaticResource {
    pub fn new(path: &str) -> Option<Self> {
        let content_type = Path::new(path)
            .extension()
            .and_then(OsStr::to_str)
            .and_then(|ext| ContentType::from_extension(ext))
            .unwrap_or(ContentType::Binary);

        FILES.get(format!("data/{}", path).as_str()).ok()
            .map(|data| StaticResource {
                data: Vec::from(data),
                content_type,
            })
    }
}

impl<'r> Responder<'r> for StaticResource {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(self.data))
            .header(self.content_type)
            .raw_header("Cache-Control", "public, max-age=604800")
            .ok()
    }
}
