// src/input.rs
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

pub trait InputReader {
    fn lines(self: Box<Self>) -> io::Result<Lines<Box<dyn BufRead>>>;
}

pub struct FileReader {
    reader: BufReader<File>,
}

impl FileReader {
    pub fn new(path: &Path) -> io::Result<FileReader> {
        let file = File::open(path)?;
        Ok(FileReader {
            reader: BufReader::new(file),
        })
    }
}

impl InputReader for FileReader {
    fn lines(self: Box<Self>) -> io::Result<Lines<Box<dyn BufRead>>> {
        Ok((Box::new(self.reader) as Box<dyn BufRead>).lines())
    }
}

pub struct StdinReader {
    reader: BufReader<io::Stdin>,
}

impl StdinReader {
    pub fn new() -> StdinReader {
        StdinReader {
            reader: BufReader::new(io::stdin()),
        }
    }
}

impl InputReader for StdinReader {
    fn lines(self: Box<Self>) -> io::Result<Lines<Box<dyn BufRead>>> {
        Ok((Box::new(self.reader) as Box<dyn BufRead>).lines())
    }
}
