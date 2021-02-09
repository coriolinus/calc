use super::{not_implemented, ArithmeticError, BasicError, Calcable};

impl Calcable for f64 {
    type Err = BasicError<f64>;

    const E: Option<Self> = Some(std::f64::consts::E);
    const PI: Option<Self> = Some(std::f64::consts::PI);

    fn parse_binary(_s: &str) -> Option<Self> {
        None
    }

    fn parse_octal(_s: &str) -> Option<Self> {
        None
    }

    fn parse_hex(_s: &str) -> Option<Self> {
        None
    }

    fn from_f32(f: f32) -> Option<Self> {
        Some(f as Self)
    }

    fn neg(self) -> Option<Self> {
        Some(-self)
    }

    fn not(self) -> Option<Self> {
        None
    }

    fn add(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self + other)
    }

    fn sub(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self - other)
    }

    fn mul(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self * other)
    }

    fn div(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        if other == 0.0 {
            Err(ArithmeticError::DivideBy0.into())
        } else {
            Ok(self / other)
        }
    }

    fn trunc_div(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        self.div(other).map(|quot| quot.floor())
    }

    fn pow(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        Ok(self.powf(other))
    }

    fn rem(self, other: Self) -> Result<Self, <Self as Calcable>::Err> {
        if other == 0.0 {
            Err(ArithmeticError::DivideBy0.into())
        } else {
            Ok(self % other)
        }
    }

    fn shl(self, _other: Self) -> Result<Self, <Self as Calcable>::Err> {
        not_implemented("<<")
    }

    fn shr(self, _other: Self) -> Result<Self, <Self as Calcable>::Err> {
        not_implemented(">>")
    }

    fn wrapping_shl(self, _other: Self) -> Result<Self, <Self as Calcable>::Err> {
        not_implemented("<<<")
    }

    fn wrapping_shr(self, _other: Self) -> Result<Self, <Self as Calcable>::Err> {
        not_implemented(">>>")
    }

    fn bit_and(self, _other: Self) -> Option<Self> {
        None
    }

    fn bit_or(self, _other: Self) -> Option<Self> {
        None
    }

    fn bit_xor(self, _other: Self) -> Option<Self> {
        None
    }

    fn abs(self) -> Option<Self> {
        Some(self.abs())
    }

    fn ceil(self) -> Option<Self> {
        Some(self.ceil())
    }

    fn floor(self) -> Option<Self> {
        Some(self.floor())
    }

    fn round(self) -> Option<Self> {
        Some(self.round())
    }

    fn sin(self) -> Option<Self> {
        Some(self.sin())
    }

    fn cos(self) -> Option<Self> {
        Some(self.cos())
    }

    fn tan(self) -> Option<Self> {
        Some(self.tan())
    }

    fn sinh(self) -> Option<Self> {
        Some(self.sinh())
    }

    fn cosh(self) -> Option<Self> {
        Some(self.cosh())
    }

    fn tanh(self) -> Option<Self> {
        Some(self.tanh())
    }

    fn asin(self) -> Option<Self> {
        Some(self.asin())
    }

    fn acos(self) -> Option<Self> {
        Some(self.acos())
    }

    fn atan(self) -> Option<Self> {
        Some(self.atan())
    }

    fn asinh(self) -> Option<Self> {
        Some(self.asinh())
    }

    fn acosh(self) -> Option<Self> {
        Some(self.acosh())
    }

    fn atanh(self) -> Option<Self> {
        Some(self.atanh())
    }

    fn sqrt(self) -> Option<Self> {
        Some(self.sqrt())
    }

    fn cbrt(self) -> Option<Self> {
        Some(self.cbrt())
    }

    fn log(self) -> Option<Self> {
        Some(self.log10())
    }

    fn lg(self) -> Option<Self> {
        Some(self.log2())
    }

    fn ln(self) -> Option<Self> {
        Some(self.ln())
    }

    fn exp(self) -> Option<Self> {
        Some(self.exp())
    }
}
