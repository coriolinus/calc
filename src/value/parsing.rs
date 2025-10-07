use std::str::FromStr;

use noisy_float::types::N64;

use super::Result;
use crate::{ParseValueError, Value};

/// Strip underscores and leading bit markers from the input string
fn clean_input(s: &str, leading: &str) -> String {
    let mut input = String::with_capacity(s.len());
    input.extend(s.chars().filter(|&c| c != '_'));
    input.trim_start_matches(leading).to_owned()
}

impl Value {
    /// Parses an integer from a string slice with digits in a given base.
    ///
    /// The string is expected to be an optional `+` or `-` sign followed by only digits.
    /// Leading and trailing non-digit characters (including whitespace) represent an error.
    /// Underscores (which are accepted in Rust literals) also represent an error.
    ///
    /// This function panics if `radix` is not in `2..=36`.
    pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseValueError> {
        u64::from_str_radix(src, radix)
            .map(Self::UnsignedInt)
            .or_else(|_| u128::from_str_radix(src, radix).map(Self::UnsignedBigInt))
            .or_else(|_| i64::from_str_radix(src, radix).map(Self::SignedInt))
            .or_else(|_| i128::from_str_radix(src, radix).map(Self::SignedBigInt)).map_err(|_| ParseValueError::Radix(src.to_owned(), radix))
    }

    /// Parse a binary input without decimals.
    ///
    /// Should succeed with or without a leading `0b`.
    pub fn parse_binary(s: &str) -> Result<Self, ParseValueError> {
        Value::from_str_radix(&clean_input(s, "0b"), 2)
    }

    /// Parse an octal input without decimals.
    ///
    /// Should succeed with or without a leading `0o`.
    pub fn parse_octal(s: &str) -> Result<Self, ParseValueError> {
        Value::from_str_radix(&clean_input(s, "0o"), 8)
    }

    /// Parse a decimal input which may or may not contain a decimal point.
    ///
    /// Should succeed with or without a leading `0d`.
    pub fn parse_decimal(s: &str) -> Result<Self, ParseValueError> {
        Value::from_str_radix(&clean_input(s, "0d"), 10)
    }

    /// Parse an octal input without decimals.
    ///
    /// Should succeed with or without a leading `0o`.
    pub fn parse_hex(s: &str) -> Result<Self, ParseValueError> {
        Value::from_str_radix(&clean_input(s, "0x"), 16)
    }
}

impl FromStr for Value {
    type Err = ParseValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map(Self::UnsignedInt)
            .or_else(|_| s.parse::<u128>().map(Self::UnsignedBigInt))
            .or_else(|_| s.parse::<i64>().map(Self::SignedInt))
            .or_else(|_| s.parse::<i128>().map(Self::SignedBigInt))
            .or_else(|_| s.parse::<f64>().map(N64::new).map(Self::Float)).map_err(|_| ParseValueError::Simple(s.to_owned()))
    }
}
