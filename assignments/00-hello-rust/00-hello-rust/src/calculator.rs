use std::io;
use std::num::ParseIntError;

enum BitwiseOp {
    And,
    Or,
    Xor,
}

// A function to display the operation name
fn display_op(op: &BitwiseOp) -> String {
    match op {
        BitwiseOp::And => "AND".to_string(),
        BitwiseOp::Or => "OR".to_string(),
        BitwiseOp::Xor => "XOR".to_string(),
    }
}

// Function to determine the bitwise operation from the input
fn determine_op(s: &str) -> Result<BitwiseOp, String> {
    match s.to_lowercase().as_str() {
        "&" | "and" => Ok(BitwiseOp::And),
        "|" | "or" => Ok(BitwiseOp::Or),
        "^" | "xor" => Ok(BitwiseOp::Xor),
        _ => Err(format!("Invalid operator: {}", s)),
    }
}

// Enum to handle different bases
#[derive(PartialEq, Debug)]
enum Base {
    Decimal,
    Hexadecimal,
    Binary,
}

// Function to detect base from input string
fn detect_base(input: &str) -> Base {
    if input.starts_with("0x") {
        Base::Hexadecimal
    } else if input.starts_with("0b") {
        Base::Binary
    } else {
        Base::Decimal
    }
}

// Function to parse numbers in different bases
fn parse_number(input: &str) -> Result<u32, ParseIntError> {
    match detect_base(input) {
        Base::Hexadecimal => u32::from_str_radix(&input[2..], 16),
        Base::Binary => u32::from_str_radix(&input[2..], 2),
        Base::Decimal => input.parse(),
    }
}

// Function to apply the bitwise operation
fn apply_operation(left: u32, right: u32, op: BitwiseOp) -> u32 {
    match op {
        BitwiseOp::And => left & right,
        BitwiseOp::Or => left | right,
        BitwiseOp::Xor => left ^ right,
    }
}

// Main function to take input and perform the operation
pub fn run() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operator = String::new();

    println!("Enter the first number (in decimal, binary, or hex format):");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");

    println!("Enter the second number (in decimal, binary, or hex format):");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");

    println!("Enter the operation (&, |, ^, AND, OR, XOR):");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read input");

    // Trim the input to remove any extra whitespace
    let input1 = input1.trim();
    let input2 = input2.trim();
    let operator = operator.trim();

    // Parse the two numbers
    let left = match parse_number(input1) {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing first number: {}", e);
            return;
        }
    };

    let right = match parse_number(input2) {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing second number: {}", e);
            return;
        }
    };

    // Parse the operator
    let op = match determine_op(operator) {
        Ok(op) => op,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // Apply the operation and print the result
    let result = apply_operation(left, right, op);
    println!(
        "The result of {} {} {} is: {}",
        input1, operator, input2, result
    );
}

#[cfg(test)]
mod tests {
    use crate::calculator::{apply_operation, detect_base, parse_number, Base, BitwiseOp};

    #[test]
    fn test_apply_operation() {
        // Basic tests
        assert_eq!(apply_operation(5, 3, BitwiseOp::And), 1);
        assert_eq!(apply_operation(5, 3, BitwiseOp::Or), 7);
        assert_eq!(apply_operation(5, 3, BitwiseOp::Xor), 6);

        // Edge cases
        assert_eq!(apply_operation(0, 0, BitwiseOp::And), 0);
        assert_eq!(apply_operation(0, 0, BitwiseOp::Or), 0);
        assert_eq!(apply_operation(0, 0, BitwiseOp::Xor), 0);

        assert_eq!(apply_operation(0xFFFFFFFF, 0, BitwiseOp::And), 0);
        assert_eq!(apply_operation(0xFFFFFFFF, 0, BitwiseOp::Or), 0xFFFFFFFF);
        assert_eq!(apply_operation(0xFFFFFFFF, 0, BitwiseOp::Xor), 0xFFFFFFFF);
    }

    #[test]
    fn test_parse_number() {
        // Decimal numbers
        assert_eq!(parse_number("10").unwrap(), 10);
        assert_eq!(parse_number("255").unwrap(), 255);

        // Hexadecimal numbers
        assert_eq!(parse_number("0xFF").unwrap(), 255);
        assert_eq!(parse_number("0x0").unwrap(), 0);

        // Binary numbers
        assert_eq!(parse_number("0b1010").unwrap(), 10);
        assert_eq!(parse_number("0b1111").unwrap(), 15);

        // Test invalid inputs
        assert!(parse_number("invalid").is_err());
        assert!(parse_number("0xG").is_err());
        assert!(parse_number("0b2101").is_err());
    }

    #[test]
    fn test_detect_base() {
        // Test decimal, binary, and hexadecimal bases
        assert_eq!(detect_base("10"), Base::Decimal);
        assert_eq!(detect_base("0x1A"), Base::Hexadecimal);
        assert_eq!(detect_base("0b1010"), Base::Binary);
    }
}
