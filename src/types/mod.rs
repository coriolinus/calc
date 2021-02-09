use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

mod f64;

#[derive(Debug, thiserror::Error)]
pub enum ArithmeticError {
    #[error("overflow")]
    Overflow,
    #[error("underflow")]
    Underflow,
    #[error("attempt to divide by 0")]
    DivideBy0,
}

#[derive(Debug, thiserror::Error)]
pub enum BasicError<T: std::fmt::Debug> {
    #[error(transparent)]
    Arithmetic(#[from] ArithmeticError),
    #[error("operation `{0}` not implemented for {}", std::any::type_name::<T>())]
    NotImplemented(&'static str, std::marker::PhantomData<T>),
}

pub(crate) fn not_implemented<T: std::fmt::Debug>(
    symbol: &'static str,
) -> Result<T, BasicError<T>> {
    Err(BasicError::NotImplemented(symbol, std::marker::PhantomData))
}

/// A trait indicating that this type is suitable for usage in this program.
///
/// Every type used here has to have basic arithmetic operations defined, but the rest of its
/// behaviors may or may not be defined. Attempts to evaluate an operation which returns `None`
/// will result in an "unimplemented" error message bubbling up to the user.
pub trait Calcable:
    Clone
    + FromStr
    + std::fmt::Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
{
    type Err;

    const E: Option<Self>;
    const PI: Option<Self>;

    /// Parse a binary input without decimals.
    ///
    /// Should succeed with or without a leading `0b`.
    fn parse_binary(s: &str) -> Option<Self>;

    /// Parse an octal input without decimals.
    ///
    /// Should succeed with or without a leading `0o`.
    fn parse_octal(s: &str) -> Option<Self>;

    /// Parse an octal input without decimals.
    ///
    /// Should succeed with or without a leading `0o`.
    fn parse_hex(s: &str) -> Option<Self>;

    /// Instantiate an instance of `Self` from an `f32`.
    ///
    /// This should be possible with minimal loss for most reasonable types.
    fn from_f32(f: f32) -> Option<Self>;

    /// Negate this value.
    fn neg(self) -> Option<Self>;

    /// Bitwise not this value.
    fn not(self) -> Option<Self>;

    /// Add this value and another, returning an error on overflow.
    fn add(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Subtract another value from this, returning an error on underflow.
    fn sub(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Multiply this value and another, returning an error on overflow.
    fn mul(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Divide this value by another, returning an error on divide by zero.
    fn div(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Divide this value by another, flooring the result to the next lowest integer.
    fn trunc_div(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Raise this value by another.
    fn pow(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Compute the arithmetic remainder of this value and another.
    fn rem(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Compute this value left-shifted by `other` bits.
    fn shl(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Compute this value right-shifted by `other` bits.
    fn shr(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Compute this value left-shifted by `other` bits, wrapping the bits around.
    fn wrapping_shl(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Compute this value right-shifted by `other` bits, wrapping the bits around.
    fn wrapping_shr(self, other: Self) -> Result<Self, <Self as Calcable>::Err>;

    /// Compute this value bitwise anded with another.
    fn bit_and(self, other: Self) -> Option<Self>;

    /// Compute this value bitwise or'd with another.
    fn bit_or(self, other: Self) -> Option<Self>;

    /// Compute this value bitwise xor'd with another.
    fn bit_xor(self, other: Self) -> Option<Self>;

    /// Compute the absolute value of this value.
    fn abs(self) -> Option<Self>;

    /// Compute the smallest integer greater than or equal to self.
    fn ceil(self) -> Option<Self>;

    /// Compute the greatest integer less than or equal to self.
    fn floor(self) -> Option<Self>;

    /// Round self to the nearest integer; halfway cases away from 0.0.
    fn round(self) -> Option<Self>;

    /// Compute the sine of self.
    fn sin(self) -> Option<Self>;

    /// Compute the cosine of self.
    fn cos(self) -> Option<Self>;

    /// Compute the tangent of self.
    fn tan(self) -> Option<Self>;

    /// Compute the hyperbolic sine of self.
    fn sinh(self) -> Option<Self>;

    /// Compute the hyperbolic cosine of self.
    fn cosh(self) -> Option<Self>;

    /// Compute the hyperbolic tangent of self.
    fn tanh(self) -> Option<Self>;

    /// Compute the arcsine of self.
    fn asin(self) -> Option<Self>;

    /// Compute the arccosine of self.
    fn acos(self) -> Option<Self>;

    /// Compute the arctangent of self.
    fn atan(self) -> Option<Self>;

    /// Compute the inverse hyperbolic sine of self.
    fn asinh(self) -> Option<Self>;

    /// Compute the inverse hyperbolic cosine of self.
    fn acosh(self) -> Option<Self>;

    /// Compute the inverse hyperbolic tangent of self.
    fn atanh(self) -> Option<Self>;

    /// Convert self as degrees to radians.
    fn rad(self) -> Option<Self> {
        Some(Self::PI? / Self::from_f32(180.0)? * self)
    }

    /// Convert self as radians to degrees.
    fn deg(self) -> Option<Self> {
        Some(Self::from_f32(180.0)? / Self::PI? * self)
    }

    /// Determine the square root of self.
    fn sqrt(self) -> Option<Self>;

    /// Determine the cube root of self.
    fn cbrt(self) -> Option<Self>;

    /// Determine the base-10 logarithm of self.
    fn log(self) -> Option<Self> {
        Some(self.ln()? / Self::from_f32(10.0)?.ln()?)
    }

    /// Determine the base-2 logarithm of self.
    fn lg(self) -> Option<Self> {
        Some(self.ln()? / Self::from_f32(2.0)?.ln()?)
    }

    /// Determine the base-`e` (natural) logarithm of self.
    fn ln(self) -> Option<Self>;

    /// Determine `e**self`
    fn exp(self) -> Option<Self>;
}

/// Strip underscores from the input string
pub(crate) fn strip_underscore(s: &str) -> String {
    let mut input = String::with_capacity(s.len());
    input.extend(s.chars().filter(|&c| c != '_'));
    input
}

/// Parse the given string into a `Calcable` instance.
///
/// This implementation strips out underscores.
pub fn parse<N: Calcable>(s: &str) -> Result<N, <N as FromStr>::Err> {
    let mut input = String::with_capacity(s.len());
    input.extend(s.chars().filter(|&c| c != '_'));
    input.parse()
}
