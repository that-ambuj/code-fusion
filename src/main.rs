use anyhow::Result;
use clap::Parser;

use code_fusion::cli::{process_options, CliOptions};

fn main() -> Result<()> {
    let args = CliOptions::parse();

    process_options(args)?;

    Ok(())
}
