use std::ops::{self, AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use super::dispatch_operation;
use crate::Value;

// shim for the missing method on f64
trait CheckedMaths:
    Sized
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Div<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Rem<Output = Self>
{
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;
}

impl CheckedMaths for f64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(self + rhs)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(self - rhs)
    }

    fn checked_div(self, rhs: Self) -> Option<Self> {
        Some(self / rhs)
    }

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        Some(self * rhs)
    }

    fn checked_rem(self, rhs: Self) -> Option<Self> {
        Some(self % rhs)
    }
}

impl<Rhs> ops::AddAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_add(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n + rhs))
            });
    }
}

impl<Rhs> ops::Add<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;
    fn add(mut self, rhs: Rhs) -> Value {
        self.add_assign(rhs);
        self
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
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_sub(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n - rhs))
            })
    }
}

impl<Rhs> ops::Sub<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn sub(mut self, rhs: Rhs) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<Rhs> ops::MulAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_mul(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n * rhs))
            });
    }
}

impl<Rhs> ops::Mul<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn mul(mut self, rhs: Rhs) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<Rhs> ops::DivAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn div_assign(&mut self, rhs: Rhs) {
        self.promote_to_float();
        let mut rhs = rhs.into();
        rhs.promote_to_float();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_div(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n / rhs))
            });
    }
}

impl<Rhs> ops::Div<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn div(mut self, rhs: Rhs) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<Rhs> ops::RemAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn rem_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_rem(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n % rhs))
            });
    }
}

impl<Rhs> ops::Rem<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn rem(mut self, rhs: Rhs) -> Self::Output {
        self.rem_assign(rhs);
        self
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
            // these integers cannot represent the negative of their minima
            Value::SignedInt(n) if n == i64::MIN => {
                self.promote();
                self.neg()
            }
            Value::SignedBigInt(n) if n == i128::MIN => {
                self.promote();
                self.neg()
            }
            // everything else is simple negation
            Value::SignedInt(n) => (-n).into(),
            Value::SignedBigInt(n) => (-n).into(),
            Value::Float(n) => (-n).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Order;
    use rstest::rstest;

    // ---------- ADD ----------
    #[rstest]
    fn add(
        #[values(10_u64, 20_u128, -30_i64, -40_i128, 1.5_f64)] left: impl Into<Value>,
        #[values(1_u64, 2_u128, -3_i64, -4_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let expect_order = left.order().max(right.order());

        let result = left + right;
        assert_eq!(result.order(), expect_order);
    }

    #[rstest]
    fn add_assign(
        #[values(10_u64, 20_u128, 30_i64, 40_i128, 50_f64)] left: impl Into<Value>,
        #[values(1_u64, 2_u128, 3_i64, 4_i128, 5_f64)] right: impl Into<Value>,
    ) {
        let mut left = left.into();
        let right = right.into();
        let expect_order = left.order().max(right.order());

        left += right;
        assert_eq!(left.order(), expect_order);
    }

    // ---------- SUB ----------
    #[rstest]
    fn sub(
        #[values(10_u64, 20_u128, 30_i64, 40_i128, 5.5_f64)] left: impl Into<Value>,
        #[values(1_u64, 2_u128, 3_i64, 4_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        // subtraction always produces a signed order
        let expect_order = left.order().max(right.order());

        let result = left - right;
        assert_eq!(result.order(), expect_order);
    }

    #[rstest]
    fn sub_assign(
        #[values(10_u64, 20_u128, 30_i64, 40_i128, 5.5_f64)] left: impl Into<Value>,
        #[values(1_u64, 2_u128, 3_i64, 4_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let mut left = left.into();
        let right = right.into();
        // subtraction always produces a signed order
        let expect_order = left.order().max(right.order());

        left -= right;
        assert_eq!(left.order(), expect_order);
    }

    // subtraction will auto-promote to a signed type to avoid underflow
    #[rstest]
    fn sub_underflow(
        #[values(1_u64, 2_u128, 3_i64, 4_i128, 2.5_f64)] left: impl Into<Value>,
        #[values(10_u64, 20_u128, 30_i64, 40_i128, 5.5_f64)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        // subtraction always produces a signed order
        let expect_order = left.order().max(right.order()).max(Order::SignedInt);

        let result = left - right;
        assert_eq!(result.order(), expect_order);
    }

    #[rstest]
    fn sub_assign_underflow(
        #[values(1_u64, 2_u128, 3_i64, 4_i128, 2.5_f64)] left: impl Into<Value>,
        #[values(10_u64, 20_u128, 30_i64, 40_i128, 5.5_f64)] right: impl Into<Value>,
    ) {
        let mut left = left.into();
        let right = right.into();
        // subtraction always produces a signed order
        let expect_order = left.order().max(right.order()).max(Order::SignedInt);

        left -= right;
        assert_eq!(left.order(), expect_order);
    }

    // ---------- MUL ----------
    #[rstest]
    fn mul(
        #[values(2_u64, 3_u128, -4_i64, -5_i128, 1.5_f64)] left: impl Into<Value>,
        #[values(2_u64, 3_u128, -4_i64, -5_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let expect_order = left.order().max(right.order());

        let result = left * right;
        assert_eq!(result.order(), expect_order);
    }

    #[rstest]
    fn mul_assign(
        #[values(2_u64, 3_u128, -4_i64, -5_i128, 1.5_f64)] left: impl Into<Value>,
        #[values(2_u64, 3_u128, -4_i64, -5_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let mut left = left.into();
        let right = right.into();
        let expect_order = left.order().max(right.order());

        left *= right;
        assert_eq!(left.order(), expect_order);
    }

    // ---------- DIV ----------
    #[rstest]
    fn div(
        #[values(10_u64, 20_u128, -30_i64, -40_i128, 5.5_f64)] left: impl Into<Value>,
        #[values(1_u64, 2_u128, -3_i64, -4_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        // division always produces a float
        let expect_order = Order::Float;

        let result = left / right;
        assert_eq!(result.order(), expect_order);
    }

    #[test]
    fn div_by_zero_produces_infinity() {
        let left: Value = 10_u64.into();
        let right: Value = 0_u64.into();
        let result = left / right;
        assert_eq!(result, f64::INFINITY.into());
    }

    #[rstest]
    fn div_assign(
        #[values(10_u64, 20_u128, -30_i64, -40_i128, 5.5_f64)] left: impl Into<Value>,
        #[values(1_u64, 2_u128, -3_i64, -4_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let mut left = left.into();
        let right = right.into();
        // division always produces a float
        let expect_order = Order::Float;

        left /= right;
        assert_eq!(left.order(), expect_order);
    }

    // ---------- REM ----------
    #[rstest]
    fn rem(
        #[values(10_u64, 20_u128, -30_i64, -40_i128, 5.5_f64)] left: impl Into<Value>,
        #[values(3_u64, 4_u128, -5_i64, -6_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let expect_order = left.order().max(right.order());

        let result = left % right;
        assert_eq!(result.order(), expect_order);
    }

    #[test]
    #[should_panic]
    fn rem_by_zero_panics() {
        let left: Value = 10_i64.into();
        let right: Value = 0_i64.into();
        let _ = left % right;
    }

    #[rstest]
    fn rem_assign(
        #[values(10_u64, 20_u128, -30_i64, -40_i128, 5.5_f64)] left: impl Into<Value>,
        #[values(3_u64, 4_u128, -5_i64, -6_i128, 2.5_f64)] right: impl Into<Value>,
    ) {
        let mut left = left.into();
        let right = right.into();
        let expect_order = left.order().max(right.order());

        left %= right;
        assert_eq!(left.order(), expect_order);
    }

    // ---------- NEG ----------
    #[rstest]
    fn neg(#[values(10_u64, 20_u128, -30_i64, -40_i128, 5.5_f64)] val: impl Into<Value>) {
        let val = val.into();
        let result = -val;

        assert!(
            result.order() >= Order::SignedInt,
            "negation should always yield a signed order"
        );
    }

    // ---------- EDGE CASES ----------
    #[test]
    fn add_large_unsigned_promotes_to_u128() {
        let left: Value = u64::MAX.into();
        let right: Value = 1_u64.into();
        let result = left + right;
        eprintln!("{result:?}");
        assert!(matches!(result, Value::UnsignedBigInt(_)));
    }

    #[test]
    fn sub_underflow_promotes_to_signed() {
        let left: Value = 0_u64.into();
        let right: Value = 1_u64.into();
        let result = left - right;
        assert!(matches!(
            result,
            Value::SignedInt(_) | Value::SignedBigInt(_)
        ));
        assert_eq!(result, (-1_i64).into());
    }

    #[rstest]
    #[case::i64_overflows_to_i128(0_i64, i64::MIN, Order::SignedBigInt, 9223372036854775808_i128)]
    #[case::i128_overflows_to_float(0_i128, i128::MIN, Order::Float, 1.7014118346046923e38)]
    fn sub_overflow_promotes_to_signed_or_float(
        #[case] left: impl Into<Value>,
        #[case] right: impl Into<Value>,
        #[case] expected_order: Order,
        #[case] expected_value: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = left - right;

        assert_eq!(
            result.order(),
            expected_order,
            "sub overflow should promote this to {expected_order:?} : got = {result:?}"
        );
        let expected_value = expected_value.into();
        assert_eq!(
            result, expected_value,
            "sub overflow value should be {expected_value:?} : got = {result:?}",
        );
    }

    #[rstest]
    #[case::i64_overflows_to_i128(
        i64::MAX,
        i64::MAX,
        Order::SignedBigInt,
        85070591730234615847396907784232501249_i128
    )]
    #[case::i128_overflows_to_float(i128::MAX, i128::MAX, Order::Float, 2.894802230932905e76)]
    fn mul_overflow_promotes_to_signed_or_float(
        #[case] left: impl Into<Value>,
        #[case] right: impl Into<Value>,
        #[case] expected_order: Order,
        #[case] expected_value: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = left * right;

        assert_eq!(
            result.order(),
            expected_order,
            "mul overflow should promote this to {expected_order:?} : got = {result:?}"
        );
        let expected_value = expected_value.into();
        assert_eq!(
            result, expected_value,
            "mul overflow value should be {expected_value:?} : got = {result:?}",
        );
    }

    #[rstest]
    #[case::i64_overflows_to_float(i64::MIN, -1_i64, 9.223372036854776e18)]
    #[case::i128_overflows_to_float(i128::MIN, -1_i128, 1.7014118346046923e38)]
    fn div_overflow_promotes_to_float(
        #[case] left: impl Into<Value>,
        #[case] right: impl Into<Value>,
        #[case] expected_value: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = left / right;

        // Div always produces a float
        let expected_order = Order::Float;

        assert_eq!(
            result.order(),
            expected_order,
            "div overflow should promote this to {expected_order:?} : got = {result:?}"
        );
        let expected_value = expected_value.into();
        assert_eq!(
            result, expected_value,
            "div overflow value should be {expected_value:?} : got = {result:?}",
        );
    }

    #[rstest]
    #[case::i64_overflows_to_i128(i64::MIN, -1_i64, Order::SignedBigInt, 0_u128)]
    #[case::i128_overflows_to_float(i128::MIN, -1_i128, Order::Float, -0.0)]
    fn rem_overflow_promotes_to_signed_or_float(
        #[case] left: impl Into<Value>,
        #[case] right: impl Into<Value>,
        #[case] expected_order: Order,
        #[case] expected_value: impl Into<Value>,
    ) {
        let left = left.into();
        let right = right.into();
        let result = left % right;

        assert_eq!(
            result.order(),
            expected_order,
            "rem overflow should promote this to {expected_order:?} : got = {result:?}"
        );
        let expected_value = expected_value.into();
        assert_eq!(
            result, expected_value,
            "rem overflow value should be {expected_value:?} : got = {result:?}",
        );
    }

    // ---------- INFINITY PROPAGATION ----------
    #[test]
    fn inf_plus_finite_is_inf() {
        let inf: Value = (f64::INFINITY).into();
        let finite: Value = 42_u64.into();
        let result = inf + finite;
        assert!(matches!(result, Value::Float(f) if f.is_infinite()));
    }

    #[test]
    fn inf_times_zero_is_nan_like() {
        let inf: Value = (f64::INFINITY).into();
        let zero: Value = 0_u64.into();
        let result = inf * zero;
        assert!(matches!(result, Value::Float(f) if f.is_nan()));
    }

    #[test]
    fn inf_div_inf_is_nan_like() {
        let inf: Value = (f64::INFINITY).into();
        let result = inf / inf;
        assert!(matches!(result, Value::Float(f) if f.is_nan()));
    }

    // ---------- SIGNED ZERO ----------
    #[test]
    fn zero_div_negative_one_is_negative_zero() {
        let zero: Value = 0.0_f64.into();
        let neg_one: Value = (-1_i64).into();
        let result = zero / neg_one;
        assert!(matches!(result, Value::Float(f) if f == 0.0 && f.is_sign_negative()));
    }

    // ---------- OVERFLOW BOUNDARIES ----------
    #[test]
    fn u64_max_plus_one_promotes_to_u128() {
        let left: Value = u64::MAX.into();
        let right: Value = 1_u64.into();
        let result = left + right;
        assert!(matches!(result, Value::UnsignedBigInt(_)));
    }

    #[test]
    fn i64_max_plus_one_promotes_to_i128() {
        let left: Value = i64::MAX.into();
        let right: Value = 1_i64.into();
        let result = left + right;
        assert!(matches!(result, Value::SignedBigInt(_)));
    }

    #[test]
    fn i128_min_neg_promotes() {
        let val: Value = i128::MIN.into();
        let result = -val;
        assert!(matches!(result, Value::Float(_)));
    }

    // ---------- CROSS-TYPE INTERACTIONS ----------
    #[test]
    fn unsigned_plus_negative_signed_promotes_to_signed() {
        let left: Value = u64::MAX.into();
        let right: Value = (-1_i64).into();
        let result = left + right;
        assert_eq!(result, Value::SignedBigInt((u64::MAX - 1) as _));
    }

    #[test]
    fn float_and_int_promotes_to_float() {
        let left: Value = 10_u64.into();
        let right: Value = 2.5_f64.into();
        let result = left + right;
        assert!(matches!(result, Value::Float(_)));
    }

    #[test]
    fn float_and_bigint_promotes_to_float() {
        let left: Value = u128::MAX.into();
        let right: Value = 1.5_f64.into();
        let result = left * right;
        assert!(matches!(result, Value::Float(_)));
    }
}
