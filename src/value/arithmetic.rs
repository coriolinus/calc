use std::ops;

use super::dispatch_operation;
use crate::Value;

impl<Rhs> ops::AddAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        dispatch_operation!(self, rhs, n, |rhs| *n += rhs);
    }
}

impl<Rhs> ops::Add<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;
    fn add(mut self, rhs: Rhs) -> Value {
        let mut rhs = rhs.into();
        dispatch_operation!(&mut self, rhs, n, |rhs| {
            *n += rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::SubAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        if rhs > *self {
            self.promote_to_signed();
        }
        dispatch_operation!(self, rhs, n, |rhs| *n -= rhs);
    }
}

impl<Rhs> ops::Sub<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn sub(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        if rhs > self {
            self.promote_to_signed();
        }
        dispatch_operation!(&mut self, rhs, n, |rhs| {
            *n -= rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::MulAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        dispatch_operation!(self, rhs, n, |rhs| *n *= rhs);
    }
}

impl<Rhs> ops::Mul<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn mul(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(&mut self, rhs, n, |rhs| {
            *n *= rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::DivAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn div_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        dispatch_operation!(self, rhs, n, |rhs| *n /= rhs);
    }
}

impl<Rhs> ops::Div<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn div(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(&mut self, rhs, n, |rhs| {
            *n /= rhs;
            (*n).into()
        })
    }
}

impl<Rhs> ops::RemAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn rem_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        dispatch_operation!(self, rhs, n, |rhs| *n %= rhs);
    }
}

impl<Rhs> ops::Rem<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn rem(mut self, rhs: Rhs) -> Self::Output {
        let mut rhs = rhs.into();
        dispatch_operation!(&mut self, rhs, n, |rhs| {
            *n %= rhs;
            (*n).into()
        })
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(mut self) -> Self::Output {
        self.promote_to_signed();
        match self {
            Value::UnsignedInt(_) | Value::UnsignedBigInt(_) => {
                unreachable!("we have already promoted out of unsigned territory")
            }
            Value::SignedInt(n) => (-n).into(),
            Value::SignedBigInt(n) => (-n).into(),
            Value::Float(n) => (-n).into(),
        }
    }
}
