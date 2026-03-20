// src/input.rs
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
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

// tests/input_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::{BufReader, Cursor, Read};

    // Implement a TestFileReader that uses an in-memory buffer
    struct TestFileReader<R: Read> {
        reader: BufReader<R>,
    }

    impl<R: Read> InputReader for TestFileReader<R> {
        fn json(self: Box<Self>) -> Result<Value, InputError> {
            let value = serde_json::from_reader(self.reader)?;
            Ok(value)
        }
    }

    #[test]
    fn test_file_reader_with_valid_json() {
        // Simulate file content using Cursor
        let json_content = r#"{"key": "value", "number": 42}"#;
        let cursor = Cursor::new(json_content.as_bytes());

        // Create a TestFileReader
        let test_reader = TestFileReader {
            reader: BufReader::new(cursor),
        };

        let value = Box::new(test_reader).json().expect("Failed to read JSON");

        // Verify the JSON content
        let expected = json!({"key": "value", "number": 42});
        assert_eq!(value, expected);
    }

    #[test]
    fn test_file_reader_with_invalid_json() {
        // Simulate file content with invalid JSON
        let json_content = r#"{"key": "value", "number": 42"#; // Missing closing brace
        let cursor = Cursor::new(json_content.as_bytes());

        let test_reader = TestFileReader {
            reader: BufReader::new(cursor),
        };

        let result = Box::new(test_reader).json();

        // Verify that an error is returned
        assert!(result.is_err());
        if let Err(InputError::Json(_)) = result {
            // Expected Json error
        } else {
            panic!("Expected Json error");
        }
    }

    #[test]
    fn test_file_reader_with_non_json_content() {
        // Simulate file content with non-JSON data
        let json_content = "This is not JSON";
        let cursor = Cursor::new(json_content.as_bytes());

        let test_reader = TestFileReader {
            reader: BufReader::new(cursor),
        };

        let result = Box::new(test_reader).json();

        // Verify that an error is returned
        assert!(result.is_err());
        if let Err(InputError::Json(_)) = result {
            // Expected Json error
        } else {
            panic!("Expected Json error");
        }
    }

    #[test]
    fn test_stdin_reader_with_valid_json() {
        // Simulate stdin using Cursor
        let json_content = r#"{"key": "value", "number": 42}"#;
        let cursor = Cursor::new(json_content.as_bytes());

        struct TestStdinReader<R: Read> {
            reader: BufReader<R>,
        }

        impl<R: Read + 'static> InputReader for TestStdinReader<R> {
            fn json(self: Box<Self>) -> Result<Value, InputError> {
                let value = serde_json::from_reader(self.reader)?;
                Ok(value)
            }
        }

        let test_reader = TestStdinReader {
            reader: BufReader::new(cursor),
        };

        let value = Box::new(test_reader).json().expect("Failed to read JSON");

        // Verify the JSON content
        let expected = json!({"key": "value", "number": 42});
        assert_eq!(value, expected);
    }

    #[test]
    fn test_stdin_reader_with_invalid_json() {
        // Simulate stdin with invalid JSON
        let json_content = r#"{"key": "value", "number": 42"#; // Missing closing brace
        let cursor = Cursor::new(json_content.as_bytes());

        struct TestStdinReader<R: Read> {
            reader: BufReader<R>,
        }

        impl<R: Read + 'static> InputReader for TestStdinReader<R> {
            fn json(self: Box<Self>) -> Result<Value, InputError> {
                let value = serde_json::from_reader(self.reader)?;
                Ok(value)
            }
        }

        let test_reader = TestStdinReader {
            reader: BufReader::new(cursor),
        };

        let result = Box::new(test_reader).json();

        // Verify that an error is returned
        assert!(result.is_err());
        if let Err(InputError::Json(_)) = result {
            // Expected Json error
        } else {
            panic!("Expected Json error");
        }
    }
}
