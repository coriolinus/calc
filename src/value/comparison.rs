use std::cmp::Ordering;

use crate::{value::dispatch_operation, Value};

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut left = *self;
        let mut right = *other;
        left.match_orders(&mut right);

        match (left, right) {
            (Value::UnsignedInt(l), Value::UnsignedInt(r)) => l.cmp(&r),
            (Value::UnsignedBigInt(l), Value::UnsignedBigInt(r)) => l.cmp(&r),
            (Value::SignedInt(l), Value::SignedInt(r)) => l.cmp(&r),
            (Value::SignedBigInt(l), Value::SignedBigInt(r)) => l.cmp(&r),
            (Value::Float(l), Value::Float(r)) => l.total_cmp(&r),
            _ => unreachable!("both sides have equal orders because we did `match_orders`"),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        let mut left = *self;
        let mut right = *other;
        dispatch_operation!(left, right, l, |r| l == r)
    }
}

impl Eq for Value {}

impl Value {
    /// Perform a strict equality comparison: this is equal if the values have equal value and order _without promotion_.
    pub fn strict_eq(self, other: Self) -> bool {
        match (self, other) {
            (Value::UnsignedInt(l), Value::UnsignedInt(r)) => l == r,
            (Value::UnsignedBigInt(l), Value::UnsignedBigInt(r)) => l == r,
            (Value::SignedInt(l), Value::SignedInt(r)) => l == r,
            (Value::SignedBigInt(l), Value::SignedBigInt(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            _ => false,
        }
    }

    /// Compute a strict ordering: this orders first by the [Order][super::Order], then by value only if the orders match
    pub fn strict_cmp(self, other: Self) -> Ordering {
        self.order()
            .cmp(&other.order())
            .then_with(|| self.cmp(&other))
    }
}

#[cfg(test)]
mod ordering_tests {
    use super::*;
    use rstest::rstest;

    // ---------- EQUALITY ----------
    #[rstest]
    fn equality_across_variants(
        #[values(42_u64, 42_u128, 42_i64, 42_i128, 42.0_f64)] left: impl Into<Value>,
    ) {
        let left = left.into();
        let right: Value = 42_u64.into();
        // All representations of 42 should be equal
        assert_eq!(left, right);
    }

    #[test]
    fn float_total_cmp_nan_behavior() {
        let nan1: Value = f64::NAN.into();
        let nan2: Value = f64::NAN.into();
        // NaN is not equal to NaN under PartialEq
        assert_ne!(nan1, nan2);
        // But ordering is defined via total_cmp
        assert_eq!(nan1.partial_cmp(&nan2), Some(Ordering::Equal));
    }

    // ---------- ORDERING ----------
    #[rstest]
    fn ordering_across_variants(
        #[values(1_u64, 1_u128, 1_i64, 1_i128, 1.0_f64)] one: impl Into<Value>,
        #[values(2_u64, 2_u128, 2_i64, 2_i128, 2.0_f64)] two: impl Into<Value>,
    ) {
        let one = one.into();
        let two = two.into();
        // All representations of 1 < 2
        assert!(one < two);
    }

    #[test]
    fn float_ordering_total_cmp() {
        let neg_zero: Value = (-0.0_f64).into();
        let pos_zero: Value = 0.0_f64.into();
        // total_cmp distinguishes -0.0 and +0.0 ordering
        assert_eq!(neg_zero.cmp(&pos_zero), f64::total_cmp(&-0.0, &0.0));
    }

    // ---------- STRICT EQUALITY ----------
    #[test]
    fn strict_eq_same_variant_same_value() {
        let a: Value = 42_u64.into();
        let b: Value = 42_u64.into();
        assert!(a.strict_eq(b));
    }

    #[test]
    fn strict_eq_different_variants_same_value() {
        let a: Value = 42_u64.into();
        let b: Value = 42_u128.into();
        // Same numeric value, but different variants
        assert!(!a.strict_eq(b));
    }

    // ---------- STRICT ORDERING ----------
    #[test]
    fn strict_cmp_same_variant() {
        let a: Value = 10_i64.into();
        let b: Value = 20_i64.into();
        assert_eq!(a.strict_cmp(b), Ordering::Less);
    }

    #[test]
    fn strict_cmp_different_variants() {
        let a: Value = 10_u64.into();
        let b: Value = 10_u128.into();
        // Different variants: strict_cmp should order by variant discriminant first
        assert_eq!(a.strict_cmp(b), Ordering::Less);
    }

    // ---------- EDGE CASES ----------
    #[test]
    fn equality_large_values_promote() {
        let a: Value = u64::MAX.into();
        let b: Value = (u64::MAX as u128).into();
        assert_eq!(a, b);
    }

    #[test]
    fn ordering_signed_vs_unsigned() {
        let a: Value = (-1_i64).into();
        let b: Value = 1_u64.into();
        assert!(a < b);
    }

    #[test]
    fn ordering_float_vs_int() {
        let a: Value = 3.5_f64.into();
        let b: Value = 4_i64.into();
        assert!(a < b);
    }
}
