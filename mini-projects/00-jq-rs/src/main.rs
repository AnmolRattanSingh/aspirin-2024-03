use anyhow::Result;
use clap::Parser;
use std::io::stdout;
use std::path::PathBuf;

mod input;
mod filter;

use input::InputReader;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'c', long)]
    compact_output: bool,

    #[clap(short = 'S', long)]
    sort_keys: bool,

    #[clap(short = 'C', long)]
    color_output: bool,

    #[clap(short = 'M', long)]
    monochrome_output: bool,

    #[clap(long)]
    indent: Option<u8>,

    filter: String,

    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    // Create a boxed input reader depending on whether a file is provided
    let reader: Box<dyn InputReader> = if let Some(file) = &args.file {
        Box::new(input::FileReader::new(file)?)
    } else {
        Box::new(input::StdinReader::new())
    };
    // Iterate through the lines from the reader
    let mut stdout_writer = stdout(); // Get a mutable reference to stdout for writing output

    let json = reader.json()?;
    println!("{}", json);

    Ok(())
}
