use anyhow::{bail, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn scaffold_rust_project(directory: PathBuf, name: &str) -> Result<()> {
    if let Ok(true) = directory.try_exists() {
        bail!(
            "Directory {} already exists. Please delete it before proceeding.",
            directory.to_string_lossy()
        );
    }

    fs::create_dir(directory.clone())?;
    eprintln!("Created directory: {}", directory.to_string_lossy());

    let output = Command::new("cargo")
        .args(["init", &directory.to_string_lossy()])
        .args(["--name", name])
        .output()?;

    eprintln!(
        "command logs:\n {}",
        String::from_utf8_lossy(&output.stderr)
    );

    Ok(())
}
