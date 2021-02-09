use super::{clean_input, ArithmeticError, BasicError, Calcable};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Basic(#[from] BasicError<i64>),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

fn as_u32(n: i64) -> Result<u32, Error> {
    if n > (u32::MAX as i64) {
        Err(Error::Basic(BasicError::Arithmetic(
            ArithmeticError::Overflow,
        )))
    } else if n < 0 {
        Err(Error::Basic(BasicError::Arithmetic(
            ArithmeticError::Underflow,
        )))
    } else {
        Ok(n as u32)
    }
}

impl Calcable for i64 {
    type Err = Error;

    const E: Option<Self> = None;
    const PI: Option<Self> = None;

    fn parse_binary(s: &str) -> Result<Self, <Self as Calcable>::Err> {
        i64::from_str_radix(&clean_input(s, "0b"), 2).map_err(Into::into)
    }

    fn parse_octal(s: &str) -> Result<Self, <Self as Calcable>::Err> {
        i64::from_str_radix(&clean_input(s, "0o"), 8).map_err(Into::into)
    }

    fn parse_hex(s: &str) -> Result<Self, <Self as Calcable>::Err> {
        i64::from_str_radix(&clean_input(s, "0x"), 16).map_err(Into::into)
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

    fn add(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        self.checked_add(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Overflow).into())
    }

    fn sub(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        self.checked_sub(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Underflow).into())
    }

    fn mul(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        self.checked_mul(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Overflow).into())
    }

    fn div(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        self.checked_div(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::DivideBy0).into())
    }

    fn trunc_div(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        self.div(other)
    }

    fn pow(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        let other = as_u32(other)?;
        self.checked_pow(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::Overflow).into())
    }

    fn rem(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        self.checked_rem(other)
            .ok_or(BasicError::Arithmetic(ArithmeticError::DivideBy0).into())
    }

    fn shl(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self << other)
    }

    fn shr(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self >> other)
    }

    fn wrapping_shl(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self.wrapping_shl(as_u32(other)?))
    }

    fn wrapping_shr(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self.wrapping_shr(as_u32(other)?))
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
