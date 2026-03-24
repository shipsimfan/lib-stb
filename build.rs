use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").expect("TARGET not set");
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // Check for platform
    let (platform, library_name) = if target.contains("windows-msvc") {
        ("windows-msvc", "stb.lib")
    } else if target.contains("linux-gnu") {
        ("linux-gnu", "libstb.a")
    } else {
        panic!("building for an unsupported operating system");
    };

    let architecture = if target.starts_with("x86_64-") {
        "x86-64"
    } else if target.starts_with("aarch64-") {
        "aarch64"
    } else {
        panic!("building for an unsupported architecture");
    };

    // Build directory
    let lib_dir = manifest_dir.join("deps").join(platform).join(architecture);
    if !lib_dir.exists() {
        panic!("missing stb library folder at \"{}\"", lib_dir.display());
    }

    // Tell cargo to use it for libraries
    println!("cargo:rerun-if-changed={}", lib_dir.display());
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // Copy the .dll to the target directory
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let bin_dir = out_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("unexpected OUT_DIR layout");

    let lib_src = lib_dir.join(library_name);
    let lib_dst = bin_dir.join(library_name);

    if lib_src.exists() {
        std::fs::copy(&lib_src, &lib_dst).ok();
    } else {
        println!(
            "cargo:warning={} not found at {}",
            library_name,
            lib_src.display()
        );
    }
}
