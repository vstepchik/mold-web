use static_files::NpmBuild;
use std::fs;

const STATIC_OUTPUT_DIR: &str = "frontend/dist/bundle";

fn main() -> std::io::Result<()> {
    fs::create_dir_all(STATIC_OUTPUT_DIR)?;
    NpmBuild::new("frontend")
        .executable("yarn")
        .install()?
        .run("build")?
        .target(STATIC_OUTPUT_DIR)
        .change_detection()
        .to_resource_dir()
        .build()
}
