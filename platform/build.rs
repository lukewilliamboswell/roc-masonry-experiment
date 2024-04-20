use std::{env, path::Path, process::Command};

fn main() {

    // Rebuild if app.roc changes
    println!("cargo:rerun-if-changed=../app.roc");

    // Build roc app into a static library
    match Command::new("roc")
        .args(&["build", "--no-link", "../app.roc"])
        .status()
    {
        Ok(status) if status.success() => {
            eprintln!("Successfully built app into a static library");
        }
        Ok(..) | Err(..) => {
            panic!("Failed to build app into a static library");
        }
    }

    // Package static library into an archive
    match Command::new("ar")
    .args(&["rcs","libapp.a", "../app.o"])
    .status() {
        Ok(status) if status.success() => {
            eprintln!("Successfully packaged app into an archive");
        }
        Ok(..) | Err(..) => {
            panic!("Failed to package app into an archive");
        }
    }

    // Search for archive in current directory
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}", Path::new(&dir).display());
    
    // Link against the archive
    println!("cargo:rustc-link-lib=static=app");
}
