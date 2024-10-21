use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;

mod filter;
mod input;
mod output;
mod parse;

use filter::Filter;
use input::InputReader;
use output::{write_output, OutputOptions};
use parse::Parse;

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

    // Create a boxed input reader depending on whether a file is provided
    let reader: Box<dyn InputReader> = if let Some(file) = &args.file {
        Box::new(input::FileReader::new(file)?)
    } else {
        Box::new(input::StdinReader::new())
    };

    // Read the JSON input
    let json = reader.json()?;

    // Create a Parse instance
    let parser = Parse::new();

    // Parse the filter string into filters
    let filters = parser.parse(&args.filter)?;

    // Apply the filters to the JSON
    let filter_instance = Filter::new();
    let mut current_values = vec![json];
    for filter_fn in &filters {
        current_values = filter_fn.apply(&filter_instance, current_values)?;
    }

    // Set up output options
    let output_options = OutputOptions {
        compact: args.compact_output,
        color_output: args.color_output,
        monochrome_output: args.monochrome_output,
        sort_keys: args.sort_keys,
        indent: args.indent.unwrap_or(2) as usize,
    };

    println!("{}", args.sort_keys);

    // Output the results
    let mut stdout_writer = io::stdout();

    for value in current_values {
        write_output(&mut stdout_writer, &value, &output_options)?;
        if !output_options.compact {
            writeln!(stdout_writer)?;
        }
    }
    Ok(())
}
