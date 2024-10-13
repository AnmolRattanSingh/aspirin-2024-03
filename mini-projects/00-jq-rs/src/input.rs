// src/input.rs
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

pub trait InputReader {
    fn json(self: Box<Self>) -> Result<Value, InputError>;
}

pub struct FileReader {
    reader: BufReader<File>,
}

impl FileReader {
    pub fn new(path: &Path) -> Result<FileReader, InputError> {
        let file = File::open(path)?;
        Ok(FileReader {
            reader: BufReader::new(file),
        })
    }
}

impl InputReader for FileReader {
    fn json(self: Box<Self>) -> Result<Value, InputError> {
        let value = serde_json::from_reader(self.reader)?;
        Ok(value)
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
    fn json(self: Box<Self>) -> Result<Value, InputError> {
        let value = serde_json::from_reader(self.reader)?;
        Ok(value)
    }
}
