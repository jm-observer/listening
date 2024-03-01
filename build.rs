#![allow(unused_imports)]
use std::process::Command;

fn main() -> anyhow::Result<()> {
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    let len = if output.stdout.len() > 7 {
        7
    } else {
        output.stdout.len()
    };
    println!(
        "cargo:rustc-env=GIT_COMMIT={}",
        String::from_utf8_lossy(&output.stdout[..len])
    );
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;
    println!("cargo:rerun-if-changed=.git/refs/heads");
    println!(
        "cargo:rustc-env=GIT_BRANCH={}",
        String::from_utf8_lossy(output.stdout.as_slice())
    );
    println!(
        "cargo:rustc-env=BUILD_DATE_TIME={}",
        chrono::Local::now().format("%Y-%m-%d %H:%M")
    );

    tauri_build::build();
    Ok(())
}
