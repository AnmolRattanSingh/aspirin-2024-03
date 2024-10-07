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

    #[test]
    fn test_plain_output() {
        let output = OutputMode::new_plain();
        let mut writer = Vec::new();
        let line = "hello world";
        let needle = "hello";
        output.write_line(&mut writer, line, needle).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "hello world\n");
    }

    #[test]
    fn test_colored_output() {
        let output = OutputMode::new_colored(Color::Red);
        let mut writer = Vec::new();
        let line = "hello world";
        let needle = "hello";
        output.write_line(&mut writer, line, needle).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "\u{1b}[31mhello\u{1b}[0m world\n");
    }
}
