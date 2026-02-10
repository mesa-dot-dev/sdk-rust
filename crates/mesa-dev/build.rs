#![allow(missing_docs)]

use std::process::Command;

fn main() {
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            // "rustc 1.82.0 (..." -> "1.82.0"
            s.split_whitespace().nth(1).map(String::from)
        })
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=MESA_RUSTC_VERSION={rustc_version}");
}
