#![allow(missing_docs)]

use std::process::Command;

fn target_rustc_version() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            // "rustc 1.82.0 (..." -> "1.82.0"
            s.split_whitespace().nth(1).map(String::from)
        })
        .ok_or("Failed to get rustc version")?;

    println!("cargo:rustc-env=MESA_RUSTC_VERSION={rustc_version}");

    Ok(())
}

fn target_protos() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/vcs.proto");
    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(&["proto/vcs.proto"], &["proto/"])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    target_rustc_version()?;
    target_protos()?;
    Ok(())
}
