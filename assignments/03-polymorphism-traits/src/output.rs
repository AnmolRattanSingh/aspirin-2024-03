// src/output.rs
use colored::*;
use std::io::{self, Write};

pub trait Output {
    fn write_line<W: Write>(&self, writer: &mut W, line: &str, needle: &str) -> io::Result<()>;
}

pub struct PlainOutput;

impl PlainOutput {
    pub fn new() -> Self {
        PlainOutput
    }
}

impl Output for PlainOutput {
    fn write_line<W: Write>(&self, writer: &mut W, line: &str, _needle: &str) -> io::Result<()> {
        writeln!(writer, "{}", line)
    }
}

pub struct ColoredOutput {
    color: Color,
}

impl ColoredOutput {
    pub fn new(color: Color) -> Self {
        ColoredOutput { color }
    }
}

impl Output for ColoredOutput {
    fn write_line<W: Write>(&self, writer: &mut W, line: &str, needle: &str) -> io::Result<()> {
        let colored_line = line.replace(needle, &needle.color(self.color).to_string());
        writeln!(writer, "{}", colored_line)
    }
}

// Define an enum to hold either a PlainOutput or a ColoredOutput
pub enum OutputMode {
    Plain(PlainOutput),
    Colored(ColoredOutput),
}

impl OutputMode {
    pub fn new_plain() -> Self {
        OutputMode::Plain(PlainOutput::new())
    }

    pub fn new_colored(color: Color) -> Self {
        OutputMode::Colored(ColoredOutput::new(color))
    }
}

impl Output for OutputMode {
    fn write_line<W: Write>(&self, writer: &mut W, line: &str, needle: &str) -> io::Result<()> {
        match self {
            OutputMode::Plain(plain) => plain.write_line(writer, line, needle),
            OutputMode::Colored(colored) => colored.write_line(writer, line, needle),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;

    #[test]
    fn test_plain_output() {
        let output = PlainOutput::new();
        let mut buffer = Vec::new();
        let line = "This is a test line.";
        output.write_line(&mut buffer, line, "test").unwrap();
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "This is a test line.\n");
    }

    #[test]
    fn test_colored_output() {
        let output = ColoredOutput::new(Color::Red);
        let mut buffer = Vec::new();
        let line = "This is a test line.";
        let needle = "test";
        output.write_line(&mut buffer, line, needle).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = format!("This is a {} line.\n", needle.color(Color::Red).to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_output_mode_plain() {
        let output_mode = OutputMode::new_plain();
        let mut buffer = Vec::new();
        let line = "This is a test line.";
        output_mode.write_line(&mut buffer, line, "test").unwrap();
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "This is a test line.\n");
    }

    #[test]
    fn test_output_mode_colored() {
        let output_mode = OutputMode::new_colored(Color::Green);
        let mut buffer = Vec::new();
        let line = "This is a test line.";
        let needle = "test";
        output_mode.write_line(&mut buffer, line, needle).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = format!(
            "This is a {} line.\n",
            needle.color(Color::Green).to_string()
        );
        assert_eq!(result, expected);
    }
}
