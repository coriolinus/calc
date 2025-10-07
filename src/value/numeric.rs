use crate::Value;

use super::{ArithmeticError, Error, Result};

impl Value {
    pub(crate) fn as_u32(self) -> Result<u32> {
        match self {
            Value::UnsignedInt(n) => u32::try_from(n).map_err(|_| ArithmeticError::Overflow.into()),
            Value::UnsignedBigInt(n) => {
                u32::try_from(n).map_err(|_| ArithmeticError::Overflow.into())
            }
            Value::SignedInt(n) => u32::try_from(n).map_err(|_| ArithmeticError::Overflow.into()),
            Value::SignedBigInt(n) => {
                u32::try_from(n).map_err(|_| ArithmeticError::Overflow.into())
            }
            Value::Float(n) => {
                if n < 0.0 {
                    return Err(ArithmeticError::Overflow.into());
                }
                if n.fract() != 0.0 {
                    return Err(Error::ImproperlyFloat);
                }
                // a 64-bit integer has at least enough precision to capture the integer part of this number
                let n = n as u64;

                u32::try_from(n).map_err(|_| ArithmeticError::Overflow.into())
            }
        }
    }

    /// Divide this value by another, flooring the result to the next lowest integer.
    pub fn trunc_div(mut self, other: impl Into<Self>) -> Self {
        self /= other;
        if let Value::Float(n) = &mut self {
            *n = n.floor();
        }
        self.demote();
        self
    }

    /// Raise this value by another.
    pub fn pow(self, right: impl Into<Value>) -> Result {
        let right = right.into();
        match self {
            Value::UnsignedInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Value::UnsignedBigInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Value::SignedInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Value::SignedBigInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Value::Float(n) => {
                if let Value::Float(e) = right {
                    Ok(n.powf(e).into())
                } else {
                    let right = right
                        .as_u32()?
                        .try_into()
                        .map_err(|_| ArithmeticError::Overflow)?;
                    Ok(n.powi(right).into())
                }
            }
        }
    }

    /// Compute the absolute value of this value.
    pub fn abs(self) -> Value {
        match self {
            Value::UnsignedInt(n) => n.into(),
            Value::UnsignedBigInt(n) => n.into(),
            Value::SignedInt(n) => n.abs().into(),
            Value::SignedBigInt(n) => n.abs().into(),
            Value::Float(n) => n.abs().into(),
        }
    }

    /// Compute the smallest integer greater than or equal to self.
    pub fn ceil(self) -> Value {
        if let Value::Float(n) = self {
            let mut out = Value::from(n.ceil());
            out.demote();
            out
        } else {
            self
        }
    }

    /// Compute the greatest integer less than or equal to self.
    pub fn floor(self) -> Value {
        if let Value::Float(n) = self {
            let mut out = Value::from(n.floor());
            out.demote();
            out
        } else {
            self
        }
    }

    /// Round self to the nearest integer; halfway cases away from 0.0.
    pub fn round(self) -> Value {
        if let Value::Float(n) = self {
            let mut out = Value::from(n.round());
            out.demote();
            out
        } else {
            self
        }
    }

    /// Compute the sine of self.
    pub fn sin(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.sin();
        }
        self
    }

    /// Compute the cosine of self.
    pub fn cos(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.cos();
        }
        self
    }

    /// Compute the tangent of self.
    pub fn tan(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.tan();
        }
        self
    }

    /// Compute the hyperbolic sine of self.
    pub fn sinh(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.sinh();
        }
        self
    }

    /// Compute the hyperbolic cosine of self.
    pub fn cosh(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.cosh();
        }
        self
    }

    /// Compute the hyperbolic tangent of self.
    pub fn tanh(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.tanh();
        }
        self
    }

    /// Compute the arcsine of self.
    pub fn asin(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.asin();
        }
        self
    }

    /// Compute the arccosine of self.
    pub fn acos(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.acos();
        }
        self
    }

    /// Compute the arctangent of self.
    pub fn atan(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.atan();
        }
        self
    }

    /// Compute the inverse hyperbolic sine of self.
    pub fn asinh(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.asinh();
        }
        self
    }

    /// Compute the inverse hyperbolic cosine of self.
    pub fn acosh(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.acosh();
        }
        self
    }

    /// Compute the inverse hyperbolic tangent of self.
    pub fn atanh(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.atanh();
        }
        self
    }

    /// Convert self as degrees to radians.
    pub fn rad(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f *= std::f64::consts::PI / 180.0;
        }
        self
    }

    /// Convert self as radians to degrees.
    pub fn deg(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f *= 180.0 / std::f64::consts::PI;
        }
        self
    }

    /// Determine the square root of self.
    pub fn sqrt(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.sqrt();
        }
        self
    }

    /// Determine the cube root of self.
    pub fn cbrt(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.cbrt();
        }
        self
    }

    /// Determine the base-10 logarithm of self.
    pub fn log(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.log10();
        }
        self
    }

    /// Determine the base-2 logarithm of self
    pub fn lg(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.log2();
        }
        self
    }

    /// Determine the base-`e` (natural) logarithm of self.
    pub fn ln(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.ln();
        }
        self
    }

    /// Determine `e**self`
    pub fn exp(mut self) -> Value {
        {
            let f = self.promote_to_float();
            *f = f.exp();
        }
        self
    }
}

#[cfg(test)]
mod demotion_tests {
    use super::*;
    use crate::value::Order;
    use rstest::rstest;

    // ---------- TRUNC_DIV ----------
    #[rstest]
    fn trunc_div_integers(
        #[values(10_u64, 20_u128, -30_i64, -40_i128)] left: impl Into<Value>,
        #[values(2_u64, 5_u128, -3_i64, -4_i128)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = left.trunc_div(right);
        // Integral inputs remain integral
        assert_ne!(result.order(), Order::Float);
    }

    #[rstest]
    fn trunc_div_float_demotes(
        #[values(10.5_f64, -20.9_f64, 1.0_f64)] left: f64,
        #[values(2.0_f64, -3.0_f64)] right: f64,
    ) {
        let left: Value = left.into();
        let right: Value = right.into();
        let result = left.trunc_div(right);
        // Floats demote to an integer type
        assert_ne!(result.order(), Order::Float);
    }

    // ---------- CEIL ----------
    #[rstest]
    fn ceil_integers_remain_integral(
        #[values(10_u64, 20_u128, -30_i64, -40_i128)] val: impl Into<Value>,
    ) {
        let val = val.into();
        let result = val.ceil();
        assert_eq!(result.order(), val.order());
    }

    #[rstest]
    fn ceil_float_demotes(#[values(1.2_f64, -1.8_f64, 1000.0_f64)] val: f64) {
        let val: Value = val.into();
        let result = val.ceil();
        assert_ne!(result.order(), Order::Float);
    }

    // ---------- FLOOR ----------
    #[rstest]
    fn floor_integers_remain_integral(
        #[values(10_u64, 20_u128, -30_i64, -40_i128)] val: impl Into<Value>,
    ) {
        let val = val.into();
        let result = val.floor();
        assert_eq!(result.order(), val.order());
    }

    #[rstest]
    fn floor_float_demotes(#[values(1.2_f64, -1.8_f64, 1000.0_f64)] val: f64) {
        let val: Value = val.into();
        let result = val.floor();
        assert_ne!(result.order(), Order::Float);
    }

    // ---------- ROUND ----------
    #[rstest]
    fn round_integers_remain_integral(
        #[values(10_u64, 20_u128, -30_i64, -40_i128)] val: impl Into<Value>,
    ) {
        let val = val.into();
        let result = val.round();
        assert_eq!(result.order(), val.order());
    }

    #[rstest]
    fn round_float_demotes(#[values(1.2_f64, 1.5_f64, -1.8_f64, -2.5_f64)] val: f64) {
        let val: Value = val.into();
        let result = val.round();
        assert_ne!(result.order(), Order::Float);
    }

    // ---------- EDGE CASES ----------
    #[test]
    fn ceil_of_large_float_demotes_to_bigint() {
        let val: Value = (u64::MAX as f64 * 1.5).into();
        let result = val.ceil();
        // Should require promotion to a big integer type
        assert!(matches!(result, Value::UnsignedBigInt(_)));
    }

    #[test]
    fn floor_of_negative_float_demotes_to_signed() {
        let val: Value = (-123.45_f64).into();
        let result = val.floor();
        assert!(matches!(result, Value::SignedInt(_)));
    }

    // ---------- FLOAT ÷ INT ----------
    #[rstest]
    fn trunc_div_float_div_int_demotes(
        #[values(10.9_f64, -20.1_f64, 1.5_f64)] left: f64,
        #[values(2_u64, 3_i64, 4_u128, -5_i128)] right: impl Into<Value>,
    ) {
        let left: Value = left.into();
        let right = right.into();
        let result = left.trunc_div(right);
        // Floats should demote to an integer type
        assert_ne!(result.order(), Order::Float);
    }

    // ---------- INT ÷ FLOAT ----------
    #[rstest]
    fn trunc_div_int_div_float_demotes(
        #[values(10_u64, -20_i64, 30_u128, -40_i128)] left: impl Into<Value>,
        #[values(2.5_f64, -3.5_f64, 1000.0_f64)] right: f64,
    ) {
        let left = left.into();
        let right: Value = right.into();
        let result = left.trunc_div(right);
        // Floats should demote to an integer type
        assert_ne!(result.order(), Order::Float);
    }

    // ---------- EDGE CASES ----------
    #[test]
    fn trunc_div_large_float_by_int_promotes_to_bigint() {
        let left: Value = (u64::MAX as f64 * 3.0).into();
        let right: Value = 2_u64.into();
        let result = left.trunc_div(right);
        // Result should require a big integer type
        assert!(matches!(result, Value::UnsignedBigInt(_)));
    }

    #[test]
    fn trunc_div_middling_large_float_by_int_promotes_to_bigint() {
        let left: Value = (u64::MAX as f64 * 1.5).into();
        let right: Value = 2_u64.into();
        let result = left.trunc_div(right);
        // Result should require a big integer type
        assert!(matches!(result, Value::UnsignedInt(_)));
    }

    #[test]
    fn trunc_div_negative_float_by_positive_int_is_signed() {
        let left: Value = (-123.45_f64).into();
        let right: Value = 2_u64.into();
        let result = left.trunc_div(right);
        assert!(matches!(result, Value::SignedInt(_)));
    }

    #[test]
    fn trunc_div_positive_int_by_negative_float_is_signed() {
        let left: Value = 123_u64.into();
        let right: Value = (-2.5_f64).into();
        let result = left.trunc_div(right);
        assert!(matches!(result, Value::SignedInt(_)));
    }
}
