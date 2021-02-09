use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref OUTPUT_FORMAT_RE: Regex = Regex::new(
        r"(?P<pad_0>0)?(?P<pad_width>[1-9][0-9]*)?(?P<base>[bodx])?(?P<separator>[n_,s])?(?P<group_size>\d+)?"
    ).unwrap();
}

/// A numeric base
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl Default for Base {
    fn default() -> Self {
        Self::Decimal
    }
}

/// Separator between character groups in the output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separator {
    None,
    Underscore,
    Comma,
    Space,
}

impl Default for Separator {
    fn default() -> Self {
        Self::None
    }
}

/// Output format
///
/// This controls the displayed representation of the number.
pub struct OutputFormat {
    pub pad_0: bool,
    pub pad_width: usize,
    pub base: Base,
    pub separator: Separator,
    pub group_size: usize,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self {
            pad_0: false,
            pad_width: Default::default(),
            base: Default::default(),
            separator: Default::default(),
            group_size: 3,
        }
    }
}

impl FromStr for OutputFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = OUTPUT_FORMAT_RE.captures(s).ok_or(())?;
        let mut format = Self::default();

        format.pad_0 =  captures.name("pad_0").is_some();
        if let Some(pad_width) = captures.name("pad_width") {
            format.pad_width = pad_width.as_str().parse().map_err(|_| ())?;
        }
        if let Some(base) = captures.name("base") {
            format.base = match base.as_str() {
                "b" => Base::Binary,
                "o" => Base::Octal,
                "d" => Base::Decimal,
                "x" => Base::Hexadecimal,
                _ => unreachable!("regex ensures we don't hit this branch"),
            };
        }
        if let Some(separator) = captures.name("separator") {
            format.separator = match separator.as_str() {
                "n" => Separator::None,
                "_" => Separator::Underscore,
                "," => Separator::Comma,
                "s" => Separator::Space,
                _ => unreachable!("regex ensures we don't hit this branch"),
            };
        }
        if let Some(group_size) = captures.name("group_size") {
            format.group_size = group_size.as_str().parse().map_err(|_| ())?;
        }

        Ok(format)
    }
}
