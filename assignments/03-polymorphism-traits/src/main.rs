use anyhow::Result;
use clap::Parser;
use colored::Color;
use std::io::stdout;
use std::path::PathBuf;

mod input;
mod matcher;
mod output;

use input::InputReader;
use matcher::{LiteralMatcher, Matcher, RegexMatcher};
use output::{Output, OutputMode};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short = 'v', long)]
    invert_match: bool,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long)]
    color: Option<Color>,

    needle: String,

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

    // Create the appropriate matcher
    let matcher: Box<dyn Matcher> = if args.regex {
        Box::new(RegexMatcher::new(&args.needle, args.ignore_case)?)
    } else {
        Box::new(LiteralMatcher::new(args.needle.clone(), args.ignore_case))
    };

    // Create the appropriate output mode
    let output_mode = if let Some(color) = args.color {
        OutputMode::new_colored(color)
    } else {
        OutputMode::new_plain()
    };

    // Iterate through the lines from the reader
    let mut stdout_writer = stdout(); // Get a mutable reference to stdout for writing output

    for line_result in reader.lines()? {
        let line = line_result?;

        // Determine if the line matches or not based on the invert_match flag
        let is_match = matcher.is_match(&line);
        if (is_match && !args.invert_match) || (!is_match && args.invert_match) {
            output_mode.write_line(&mut stdout_writer, &line, &args.needle)?;
        }
    }

    Ok(())
}
