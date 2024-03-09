use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    pattern: String,

    path: std::path::PathBuf,
}

#[derive(Debug)]
struct FileErr(String);

fn main() -> Result<()> {
    let args = Args::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
