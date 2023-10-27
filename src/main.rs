use anyhow::{bail, Result};
use std::path::PathBuf;
use std::{fs, process::Command};

use clap::{Parser, ValueEnum};

/// A CLI to scaffold a new project in multiple languages
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct CliOptions {
    /// Language to use for the project
    #[arg(value_enum, long, short)]
    language: Language,

    /// Project Name, defaults to directory name
    #[arg(long, short)]
    name: Option<String>,

    directory: PathBuf,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
enum Language {
    Rust,
    Cpp,
    Go,
    Haskell,
    Typescript,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Rust => "Rust",
            Language::Cpp => "C++",
            Language::Go => "Go",
            Language::Haskell => "Haskell",
            Language::Typescript => "Typescript",
        }
        .into()
    }
}

fn main() -> Result<()> {
    let args = CliOptions::parse();

    process_options(args)?;

    Ok(())
}

pub fn process_options(opts: CliOptions) -> Result<()> {
    let project_name = opts
        .name
        .unwrap_or(opts.directory.to_string_lossy().to_string());

    match opts.language {
        Language::Rust => scaffold_rust_project(opts.directory, &project_name)?,
        _ => unimplemented!(),
    }

    eprintln!(
        "Created {} project: {}",
        opts.language.to_string(),
        project_name
    );

    Ok(())
}

fn scaffold_rust_project(directory: PathBuf, name: &str) -> Result<()> {
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
