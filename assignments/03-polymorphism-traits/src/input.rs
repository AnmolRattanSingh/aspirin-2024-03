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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{self, BufRead, Cursor, Write};
    use std::path::Path;

    #[test]
    fn test_file_reader_with_input_reader_trait() {
        // Create a temporary file with test data
        let temp_file_path = "test_temp_file.txt";
        let mut temp_file = File::create(temp_file_path).expect("Failed to create temp file");
        writeln!(temp_file, "line 1").expect("Failed to write to temp file");
        writeln!(temp_file, "line 2").expect("Failed to write to temp file");
        writeln!(temp_file, "line 3").expect("Failed to write to temp file");

        // Use FileReader via InputReader trait
        let file_reader =
            FileReader::new(Path::new(temp_file_path)).expect("Failed to open temp file");
        let boxed_reader: Box<dyn InputReader> = Box::new(file_reader);
        let mut lines = boxed_reader
            .lines()
            .expect("Failed to read lines from file");

        assert_eq!(lines.next().unwrap().unwrap(), "line 1");
        assert_eq!(lines.next().unwrap().unwrap(), "line 2");
        assert_eq!(lines.next().unwrap().unwrap(), "line 3");

        // Clean up the temporary file
        fs::remove_file(temp_file_path).expect("Failed to delete temp file");
    }

    #[test]
    fn test_stdin_reader_with_input_reader_trait() {
        // I'm not really sure how to mock stdin. Please enter "line 1" and "line 2" in the terminal.

        // Uncomment the lines below to run the test interactively 🙏🏼
        /*
        println!("Please enter two lines of input:");
        let stdin_reader = StdinReader::new();
        let boxed_reader: Box<dyn InputReader> = Box::new(stdin_reader);
        let mut lines = boxed_reader
            .lines()
            .expect("Failed to read lines from stdin");

        let line1 = lines.next().unwrap().unwrap();
        let line2 = lines.next().unwrap().unwrap();

        println!("Read lines: '{}' and '{}'", line1, line2);

        // You can add assertions based on expected input if desired.
        assert_eq!(line1, "line 1");
        assert_eq!(line2, "line 2");
        */
    }
}
