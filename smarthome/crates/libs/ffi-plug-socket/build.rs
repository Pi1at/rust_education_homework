use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join(format!("target/{profile}")).display()
    );
    println!("cargo:rustc-link-lib=plug_socket");
}
