use std::ops;

use super::{dispatch_operation, Error, Result};
use crate::Value;

impl Value {
    /// Compute this value left-shifted by `other` bits, wrapping the bits around.
    pub fn rotate_left(self, shift: impl Into<Value>) -> Result {
        let shift = shift.into().as_u32()?;
        match self {
            Value::UnsignedInt(n) => Ok(n.rotate_left(shift).into()),
            Value::UnsignedBigInt(n) => Ok(n.rotate_left(shift).into()),
            Value::SignedInt(n) => Ok(n.rotate_left(shift).into()),
            Value::SignedBigInt(n) => Ok(n.rotate_left(shift).into()),
            Value::Float(_) => Err(Error::ImproperlyFloat),
        }
    }

    /// Compute this value right-shifted by `other` bits, wrapping the bits around.
    pub fn rotate_right(self, shift: impl Into<Value>) -> Result {
        let shift = shift.into().as_u32()?;
        match self {
            Value::UnsignedInt(n) => Ok(n.rotate_right(shift).into()),
            Value::UnsignedBigInt(n) => Ok(n.rotate_right(shift).into()),
            Value::SignedInt(n) => Ok(n.rotate_right(shift).into()),
            Value::SignedBigInt(n) => Ok(n.rotate_right(shift).into()),
            Value::Float(_) => Err(Error::ImproperlyFloat),
        }
    }
}

impl<Rhs> ops::Shl<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Result;

    fn shl(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(INTS: &mut self, rhs, n, |rhs| {
            *n <<= rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::Shr<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Result;

    fn shr(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(INTS: &mut self, rhs, n, |rhs| {
            *n >>= rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::BitAnd<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Result;

    fn bitand(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(INTS: &mut self, rhs, n, |rhs| {
            *n &= rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::BitOr<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Result;

    fn bitor(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(INTS: &mut self, rhs, n, |rhs| {
            *n |= rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::BitXor<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Result;

    fn bitxor(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(INTS: &mut self, rhs, n, |rhs| {
            *n ^= rhs;
            (*n).into()
        })
    }
}

impl ops::Not for Value {
    type Output = Result;

    fn not(self) -> Self::Output {
        match self {
            Value::UnsignedInt(n) => Ok((!n).into()),
            Value::UnsignedBigInt(n) => Ok((!n).into()),
            Value::SignedInt(n) => Ok((!n).into()),
            Value::SignedBigInt(n) => Ok((!n).into()),
            Value::Float(_) => Err(Error::ImproperlyFloat),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Order;
    use rstest::rstest;

    // ---------- SHIFT LEFT ----------
    #[rstest]
    fn shl_integers(
        #[values(1_u64, 1_u128, 1_i64, 1_i128)] left: impl Into<Value>,
        #[values(1, 2, 3)] shift: u32,
    ) {
        let left = left.into();
        let result = (left << shift).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn shl_float_is_error() {
        let left: Value = 1.5_f64.into();
        let result = left << 1_u32;
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- SHIFT RIGHT ----------
    #[rstest]
    fn shr_integers(
        #[values(8_u64, 8_u128, 8_i64, 8_i128)] left: impl Into<Value>,
        #[values(1, 2)] shift: u32,
    ) {
        let left = left.into();
        let result = (left >> shift).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn shr_float_is_error() {
        let left: Value = 8.0_f64.into();
        let result = left >> 1_u32;
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- BITWISE AND ----------
    #[rstest]
    fn bitand_integers(
        #[values(0b1010_u64, 0b1010_u128, 0b1010_i64, 0b1010_i128)] left: impl Into<Value>,
        #[values(0b1100_u64, 0b1100_u128, 0b1100_i64, 0b1100_i128)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = (left & right).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn bitand_float_is_error() {
        let left: Value = 1.0_f64.into();
        let right: Value = 2_u64.into();
        let result = left & right;
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- BITWISE OR ----------
    #[rstest]
    fn bitor_integers(
        #[values(0b1010_u64, 0b1010_u128, 0b1010_i64, 0b1010_i128)] left: impl Into<Value>,
        #[values(0b0101_u64, 0b0101_u128, 0b0101_i64, 0b0101_i128)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = (left | right).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn bitor_float_is_error() {
        let left: Value = 1.0_f64.into();
        let right: Value = 2_u64.into();
        let result = left | right;
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- BITWISE XOR ----------
    #[rstest]
    fn bitxor_integers(
        #[values(0b1010_u64, 0b1010_u128, 0b1010_i64, 0b1010_i128)] left: impl Into<Value>,
        #[values(0b1100_u64, 0b1100_u128, 0b1100_i64, 0b1100_i128)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = (left ^ right).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn bitxor_float_is_error() {
        let left: Value = 1.0_f64.into();
        let right: Value = 2_u64.into();
        let result = left ^ right;
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- BITWISE NOT ----------
    #[rstest]
    fn not_integers(#[values(0_u64, 0_u128, 0_i64, 0_i128)] val: impl Into<Value>) {
        let val = val.into();
        let result = (!val).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn not_float_is_error() {
        let val: Value = 1.0_f64.into();
        let result = !val;
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- ROTATE LEFT ----------
    #[rstest]
    fn rotate_left_integers(
        #[values(0b0001_u64, 0b0001_u128, 0b0001_i64, 0b0001_i128)] val: impl Into<Value>,
        #[values(1, 2, 63)] shift: u32,
    ) {
        let val = val.into();
        let result = val.rotate_left(shift).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn rotate_left_float_is_error() {
        let val: Value = 1.0_f64.into();
        let result = val.rotate_left(1);
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- ROTATE RIGHT ----------
    #[rstest]
    fn rotate_right_integers(
        #[values(0b1000_u64, 0b1000_u128, 0b1000_i64, 0b1000_i128)] val: impl Into<Value>,
        #[values(1, 2, 63)] shift: u32,
    ) {
        let val = val.into();
        let result = val.rotate_right(shift).unwrap();
        assert_ne!(result.order(), Order::Float);
    }

    #[test]
    fn rotate_right_float_is_error() {
        let val: Value = 1.0_f64.into();
        let result = val.rotate_right(1);
        assert!(matches!(result, Err(Error::ImproperlyFloat)));
    }

    // ---------- CROSS-TYPE BITWISE ----------
    #[test]
    fn u64_and_u128_promotes_to_u128() {
        let left: Value = 0b1010_u64.into();
        let right: Value = 0b1100_u128.into();
        let result = (left & right).unwrap();
        assert!(matches!(result, Value::UnsignedBigInt(_)));
    }

    #[test]
    fn i64_or_i128_promotes_to_i128() {
        let left: Value = 0b1010_i64.into();
        let right: Value = 0b0101_i128.into();
        let result = (left | right).unwrap();
        assert!(matches!(result, Value::SignedBigInt(_)));
    }

    #[test]
    fn u64_xor_i64_promotes_to_signed() {
        let left: Value = 0b1111_u64.into();
        let right: Value = (-1_i64).into();
        let result = (left ^ right).unwrap();
        assert!(matches!(result, Value::SignedInt(_)));
    }
}
