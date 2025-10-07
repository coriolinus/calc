use std::ops;

use super::{dispatch_operation, Error, Result};
use crate::Value;

impl Value {
    /// Compute this value left-shifted by `other` bits, wrapping the bits around.
    pub fn rotate_left(mut self, right: impl Into<Value>) -> Result {
        let mut right = right.into();
        dispatch_operation!(INTS: &mut self, right, n, |rhs| {
            *n <<= rhs;
            (*n).into()
        })
    }

    /// Compute this value right-shifted by `other` bits, wrapping the bits around.
    pub fn rotate_right(mut self, right: impl Into<Value>) -> Result {
        let mut right = right.into();
        dispatch_operation!(INTS: &mut self, right, n, |rhs| {
            *n <<= rhs;
            (*n).into()
        })
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
