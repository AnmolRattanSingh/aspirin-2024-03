use serde_json::{to_writer_pretty, to_writer, Value};
use std::io::{Write, Result};

// write normal and pretty output to stdout
pub fn normal_output<W: Write>(writer: &mut W, value: &Value) -> Result<()> {
    to_writer(writer, value)?;
    Ok(())
}

pub fn pretty_output<W: Write>(writer: &mut W, value: &Value) -> Result<()> {
    to_writer_pretty(writer, value)?;
    Ok(())
}
