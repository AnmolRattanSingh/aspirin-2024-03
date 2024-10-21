use serde_json::Value;
use std::env;
use std::io::{Result, Write};

#[derive(Clone)]
pub struct OutputOptions {
    pub compact: bool,
    pub color_output: bool,
    pub monochrome_output: bool,
    pub sort_keys: bool,
    pub indent: usize,
}

pub fn write_output<W: Write>(
    writer: &mut W,
    value: &Value,
    options: &OutputOptions,
) -> Result<()> {
    let color_config = if options.color_output && !options.monochrome_output {
        Some(ColorConfig::from_env())
    } else {
        None
    };

    let mut formatter = Formatter::new(writer, options, color_config);
    formatter.format(value, 0)?;
    Ok(())
}

struct Formatter<'a, W: Write> {
    writer: &'a mut W,
    options: &'a OutputOptions,
    color_config: Option<ColorConfig>,
}

impl<'a, W: Write> Formatter<'a, W> {
    fn new(
        writer: &'a mut W,
        options: &'a OutputOptions,
        color_config: Option<ColorConfig>,
    ) -> Self {
        Formatter {
            writer,
            options,
            color_config,
        }
    }

    fn format(&mut self, value: &Value, depth: usize) -> Result<()> {
        match value {
            Value::Null => {
                let color = self.color_config.as_ref().map(|c| c.null.clone());
                self.write_colored("null", &color)?;
            }
            Value::Bool(b) => {
                let text = if *b { "true" } else { "false" };
                let color = if *b {
                    self.color_config.as_ref().map(|c| c.boolean_true.clone())
                } else {
                    self.color_config.as_ref().map(|c| c.boolean_false.clone())
                };
                self.write_colored(text, &color)?;
            }
            Value::Number(num) => {
                let color = self.color_config.as_ref().map(|c| c.number.clone());
                self.write_colored(&num.to_string(), &color)?;
            }
            Value::String(s) => {
                let quoted = format!("\"{}\"", s);
                let color = self.color_config.as_ref().map(|c| c.string.clone());
                self.write_colored(&quoted, &color)?;
            }
            Value::Array(arr) => {
                self.write_array(arr, depth)?;
            }
            Value::Object(map) => {
                self.write_object(map, depth)?;
            }
        }
        Ok(())
    }

    fn write_array(&mut self, arr: &[Value], depth: usize) -> Result<()> {
        let brackets_color = self.color_config.as_ref().map(|c| c.array.clone());
        self.write_colored("[", &brackets_color)?;

        if !arr.is_empty() {
            if !self.options.compact {
                self.writer.write_all(b"\n")?;
            }
            let new_depth = depth + 1;
            for (i, value) in arr.iter().enumerate() {
                if !self.options.compact {
                    self.write_indent(new_depth)?;
                }
                self.format(value, new_depth)?;
                if i != arr.len() - 1 {
                    self.write_colored(",", &brackets_color)?;
                }
                if !self.options.compact {
                    self.writer.write_all(b"\n")?;
                }
            }
            if !self.options.compact {
                self.write_indent(depth)?;
            }
        }

        self.write_colored("]", &brackets_color)?;
        Ok(())
    }

    fn write_object(&mut self, map: &serde_json::Map<String, Value>, depth: usize) -> Result<()> {
        let brackets_color = self.color_config.as_ref().map(|c| c.object.clone());
        self.write_colored("{", &brackets_color)?;

        if !map.is_empty() {
            if !self.options.compact {
                self.writer.write_all(b"\n")?;
            }
            let new_depth = depth + 1;

            // Collect entries and sort if necessary
            let mut entries: Vec<(&String, &Value)> = map.iter().collect();
            if self.options.sort_keys {
                entries.sort_by(|a, b| a.0.cmp(b.0));
            }

            for (i, (key, value)) in entries.iter().enumerate() {
                if !self.options.compact {
                    self.write_indent(new_depth)?;
                }
                let quoted_key = format!("\"{}\"", key);
                let key_color = self.color_config.as_ref().map(|c| c.key.clone());
                self.write_colored(&quoted_key, &key_color)?;
                self.write_colored(":", &brackets_color)?;
                if !self.options.compact {
                    self.writer.write_all(b" ")?;
                }
                self.format(value, new_depth)?;
                if i != entries.len() - 1 {
                    self.write_colored(",", &brackets_color)?;
                }
                if !self.options.compact {
                    self.writer.write_all(b"\n")?;
                }
            }
            if !self.options.compact {
                self.write_indent(depth)?;
            }
        }

        self.write_colored("}", &brackets_color)?;
        Ok(())
    }

    fn write_indent(&mut self, depth: usize) -> Result<()> {
        let indent = if self.options.indent <= 7 {
            self.options.indent
        } else {
            2
        };
        let spaces = " ".repeat(depth * indent);
        self.writer.write_all(spaces.as_bytes())?;
        Ok(())
    }

    fn write_colored(&mut self, text: &str, color: &Option<Color>) -> Result<()> {
        if let Some(color) = color {
            if !self.options.monochrome_output {
                self.writer.write_all(color.prefix.as_bytes())?;
            }
            self.writer.write_all(text.as_bytes())?;
            if !self.options.monochrome_output {
                self.writer.write_all(color.suffix.as_bytes())?;
            }
        } else {
            self.writer.write_all(text.as_bytes())?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct ColorConfig {
    null: Color,
    boolean_false: Color,
    boolean_true: Color,
    number: Color,
    string: Color,
    array: Color,
    object: Color,
    key: Color,
}

impl ColorConfig {
    fn from_env() -> Self {
        let default = "0;31:0;31:0;31:0;35:0;32:1;34:1;34:0;34";
        let jq_colors = env::var("JQ_COLORS").unwrap_or_else(|_| default.to_string());

        let entries: Vec<&str> = jq_colors.split(':').collect();
        let mut colors = vec![];

        for entry in entries {
            let parts: Vec<&str> = entry.split(';').collect();
            let format_code = parts.first().unwrap_or(&"0");
            let color_code = parts.get(1).unwrap_or(&"37"); // Default to white
            colors.push(Color::new(format_code, color_code));
        }

        ColorConfig {
            null: colors
                .first()
                .cloned()
                .unwrap_or_else(|| Color::new("0", "31")),
            boolean_false: colors
                .get(1)
                .cloned()
                .unwrap_or_else(|| Color::new("0", "31")),
            boolean_true: colors
                .get(2)
                .cloned()
                .unwrap_or_else(|| Color::new("0", "31")),
            number: colors
                .get(3)
                .cloned()
                .unwrap_or_else(|| Color::new("0", "35")),
            string: colors
                .get(4)
                .cloned()
                .unwrap_or_else(|| Color::new("0", "32")),
            array: colors
                .get(5)
                .cloned()
                .unwrap_or_else(|| Color::new("1", "34")),
            object: colors
                .get(6)
                .cloned()
                .unwrap_or_else(|| Color::new("1", "34")),
            key: colors
                .get(7)
                .cloned()
                .unwrap_or_else(|| Color::new("0", "34")),
        }
    }
}

#[derive(Clone)]
struct Color {
    prefix: String,
    suffix: String,
}

impl Color {
    fn new(format_code: &str, color_code: &str) -> Self {
        let prefix = format!("\x1b[{};{}m", format_code, color_code);
        let suffix = "\x1b[0m".to_string();
        Color { prefix, suffix }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::env;

    #[test]
    fn test_default_output() {
        let value = json!({"key": "value", "number": 42, "bool": true});
        let options = OutputOptions {
            compact: false,
            color_output: false,
            monochrome_output: false,
            sort_keys: false,
            indent: 2,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected = "{\n  \"key\": \"value\",\n  \"number\": 42,\n  \"bool\": true\n}";
        assert_eq!(output_str.trim(), expected);
    }

    #[test]
    fn test_compact_output() {
        let value = json!({"key": "value", "number": 42, "bool": true});
        let options = OutputOptions {
            compact: true,
            color_output: false,
            monochrome_output: false,
            sort_keys: false,
            indent: 2,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected = "{\"key\":\"value\",\"number\":42,\"bool\":true}";
        assert_eq!(output_str.trim(), expected);
    }

    #[test]
    fn test_custom_indent() {
        let value = json!({"key": "value", "number": 42, "bool": true});
        let options = OutputOptions {
            compact: false,
            color_output: false,
            monochrome_output: false,
            sort_keys: false,
            indent: 4,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected = "{\n    \"key\": \"value\",\n    \"number\": 42,\n    \"bool\": true\n}";
        assert_eq!(output_str.trim(), expected);
    }

    #[test]
    fn test_sort_keys() {
        let value = json!({"b": 1, "a": 2, "c": 3});
        let options = OutputOptions {
            compact: false,
            color_output: false,
            monochrome_output: false,
            sort_keys: true,
            indent: 2,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected = "{\n  \"a\": 2,\n  \"b\": 1,\n  \"c\": 3\n}";
        assert_eq!(output_str.trim(), expected);
    }

    #[test]
    fn test_color_output() {
        // Ensure JQ_COLORS is unset
        env::remove_var("JQ_COLORS");
        let value = json!({"key": "value", "number": 42, "bool": true, "null": null});
        let options = OutputOptions {
            compact: false,
            color_output: true,
            monochrome_output: false,
            sort_keys: false,
            indent: 2,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        // Expected output with ANSI color codes
        // Since the color codes are embedded in the output, we'll check for their presence
        assert!(output_str.contains("\x1b[0;34m\"key\"\x1b[0m")); // Key in blue
        assert!(output_str.contains("\x1b[0;32m\"value\"\x1b[0m")); // String in green
        assert!(output_str.contains("\x1b[0;35m42\x1b[0m")); // Number in magenta
        assert!(output_str.contains("\x1b[0;31mtrue\x1b[0m")); // Boolean true in red
        assert!(output_str.contains("\x1b[0;31mnull\x1b[0m")); // Null in red
    }

    #[test]
    fn test_monochrome_output() {
        let value = json!({"key": "value", "number": 42, "bool": false});
        let options = OutputOptions {
            compact: false,
            color_output: true,
            monochrome_output: true,
            sort_keys: false,
            indent: 2,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        // No ANSI color codes should be present
        assert!(!output_str.contains("\x1b["));
        let expected = "{\n  \"key\": \"value\",\n  \"number\": 42,\n  \"bool\": false\n}";
        assert_eq!(output_str.trim(), expected);
    }

    #[test]
    fn test_custom_jq_colors() {
        // Set custom JQ_COLORS
        env::set_var("JQ_COLORS", "1;33:1;34:1;35:1;36:1;37:1;31:1;32:1;34");
        let value = json!({"key": "value", "number": 42, "bool": false, "null": null});
        let options = OutputOptions {
            compact: false,
            color_output: true,
            monochrome_output: false,
            sort_keys: false,
            indent: 2,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        env::remove_var("JQ_COLORS"); // Clean up

        let output_str = String::from_utf8(output).unwrap();

        println!("Output:\n{}", output_str);

        // Check for custom color codes
        assert!(output_str.contains("\x1b[1;34m\"key\"\x1b[0m")); // Key in bold blue
        assert!(output_str.contains("\x1b[1;37m\"value\"\x1b[0m")); // String in bold white
        assert!(output_str.contains("\x1b[1;36m42\x1b[0m")); // Number in bold cyan
        assert!(output_str.contains("\x1b[1;34mfalse\x1b[0m")); // Boolean false in bold magenta
        assert!(output_str.contains("\x1b[1;33mnull\x1b[0m")); // Null in bold yellow
    }

    #[test]
    fn test_array_output() {
        let value = json!([1, 2, 3, {"a": true, "b": null}]);
        let options = OutputOptions {
            compact: false,
            color_output: false,
            monochrome_output: false,
            sort_keys: false,
            indent: 2,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected = "[\n  1,\n  2,\n  3,\n  {\n    \"a\": true,\n    \"b\": null\n  }\n]";
        assert_eq!(output_str.trim(), expected);
    }

    #[test]
    fn test_large_indent() {
        let value = json!({"key": "value"});
        let options = OutputOptions {
            compact: false,
            color_output: false,
            monochrome_output: false,
            sort_keys: false,
            indent: 8, // Exceeds the maximum of 7, should default to 2
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected = "{\n  \"key\": \"value\"\n}";
        assert_eq!(output_str.trim(), expected);
    }

    #[test]
    fn test_zero_indent() {
        let value = json!({"key": "value", "nested": {"inner": 123}});
        let options = OutputOptions {
            compact: false,
            color_output: false,
            monochrome_output: false,
            sort_keys: false,
            indent: 0,
        };
        let mut output = Vec::new();
        write_output(&mut output, &value, &options).unwrap();
        let output_str = String::from_utf8(output).unwrap();
        let expected = "{\n\"key\": \"value\",\n\"nested\": {\n\"inner\": 123\n}\n}";
        assert_eq!(output_str.trim(), expected);
    }
}
