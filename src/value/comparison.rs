use std::cmp::Ordering;

use crate::{value::dispatch_operation, Value};

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut left = *self;
        let mut right = *other;
        dispatch_operation!(left, right, l, |r| l.cmp(&r))
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
