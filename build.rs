use std::path::PathBuf;

fn main() {
    let zombie_dir = PathBuf::from("./zombie").canonicalize().expect("k");
    cc::Build::new()
        .file(zombie_dir.join("src/ffi.cpp"))
        .include(zombie_dir.join("include"))
        .cargo_metadata(true)
        .cpp(true)
        .flag("-std=c++17")
        .compile("zombie");
}
