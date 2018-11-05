extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("FILES")
        .dir("data", Compression::Gzip)
        .build("data.rs")
        .unwrap();
}
