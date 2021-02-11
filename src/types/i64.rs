use super::{clean_input, ArithmeticError, BasicError, Calcable};

pub type Error = BasicError<i64, std::num::ParseIntError>;
pub type Result = std::result::Result<i64, Error>;

fn as_u32(n: i64) -> std::result::Result<u32, Error> {
    if n > (u32::MAX as i64) {
        Err(Error::Arithmetic(ArithmeticError::Overflow))
    } else if n < 0 {
        Err(Error::Arithmetic(ArithmeticError::Underflow))
    } else {
        Ok(n as u32)
    }
}

impl Calcable for i64 {
    type Err = Error;

    const E: Option<Self> = None;
    const PI: Option<Self> = None;

    fn parse_binary(s: &str) -> Result {
        i64::from_str_radix(&clean_input(s, "0b"), 2).map_err(Error::Parse)
    }

    fn parse_octal(s: &str) -> Result {
        i64::from_str_radix(&clean_input(s, "0o"), 8).map_err(Error::Parse)
    }

    fn parse_decimal(s: &str) -> Result {
        i64::from_str_radix(&clean_input(s, "0d"), 10).map_err(Error::Parse)
    }

    fn parse_hex(s: &str) -> Result {
        i64::from_str_radix(&clean_input(s, "0x"), 16).map_err(Error::Parse)
    }

    fn from_f32(f: f32) -> Option<Self> {
        Some(f as Self)
    }

    fn neg(self) -> Option<Self> {
        Some(-self)
    }

    fn not(self) -> Option<Self> {
        Some(!self)
    }

    fn add(self, other: Self) -> Result {
        self.checked_add(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Overflow))
    }

    fn sub(self, other: Self) -> Result {
        self.checked_sub(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Underflow))
    }

    fn mul(self, other: Self) -> Result {
        self.checked_mul(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Overflow))
    }

    fn div(self, other: Self) -> Result {
        self.checked_div(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::DivideBy0))
    }

    fn trunc_div(self, other: Self) -> Result {
        self.div(other)
    }

    fn pow(self, other: Self) -> Result {
        let other = as_u32(other)?;
        self.checked_pow(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Overflow))
    }

    fn rem(self, other: Self) -> Result {
        self.checked_rem(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::DivideBy0))
    }

    fn shl(self, other: Self) -> Result {
        Ok(self << other)
    }

    fn shr(self, other: Self) -> Result {
        Ok(self >> other)
    }

    fn rotate_left(self, other: Self) -> Result {
        Ok(self.rotate_left(as_u32(other)?))
    }

    fn rotate_right(self, other: Self) -> Result {
        Ok(self.rotate_right(as_u32(other)?))
    }

    fn bit_and(self, other: Self) -> Option<Self> {
        Some(self & other)
    }

    fn bit_or(self, other: Self) -> Option<Self> {
        Some(self | other)
    }

    fn bit_xor(self, other: Self) -> Option<Self> {
        Some(self ^ other)
    }

    fn abs(self) -> Option<Self> {
        Some(self.abs())
    }

    fn ceil(self) -> Option<Self> {
        Some(self)
    }

    fn floor(self) -> Option<Self> {
        Some(self)
    }

    fn round(self) -> Option<Self> {
        Some(self)
    }

    fn sin(self) -> Option<Self> {
        None
    }

    fn cos(self) -> Option<Self> {
        None
    }

    fn tan(self) -> Option<Self> {
        None
    }

    fn sinh(self) -> Option<Self> {
        None
    }

    fn cosh(self) -> Option<Self> {
        None
    }

    fn tanh(self) -> Option<Self> {
        None
    }

    fn asin(self) -> Option<Self> {
        None
    }

    fn acos(self) -> Option<Self> {
        None
    }

    fn atan(self) -> Option<Self> {
        None
    }

    fn asinh(self) -> Option<Self> {
        None
    }

    fn acosh(self) -> Option<Self> {
        None
    }

    fn atanh(self) -> Option<Self> {
        None
    }

    fn sqrt(self) -> Option<Self> {
        None
    }

    fn cbrt(self) -> Option<Self> {
        None
    }

    fn ln(self) -> Option<Self> {
        None
    }
}
