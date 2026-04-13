use anyhow::{Context, Result};
use arboard::Clipboard;
use clap::Parser;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
struct Args {
    file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = if let Some(file) = args.file {
        fs::read_to_string(&file).context("Failed to read file")
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read stdin")?;
        Ok(buffer)
    }?;

    let mut clipboard = Clipboard::new().context("Failed to connect to clipboard")?;
    clipboard
        .set_text(&content)
        .context("Failed to copy to clipboard")?;
    // println!("Copied {} chars to system clipboard", content.len());
    Ok(())
}
