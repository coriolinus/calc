mod arithmetic;
mod bitwise;
mod error;
mod format;
mod numeric;
mod parsing;

use std::{cmp::Ordering, f64};

use noisy_float::types::N64;
use num_traits::ToPrimitive as _;

pub use error::{ArithmeticError, Error, ParseValueError};

macro_rules! dispatch_operation {
    ($lhs:expr, $rhs:expr, $n:ident, $op:expr) => {{
        $lhs.match_orders(&mut $rhs);
        debug_assert_eq!(
            $lhs.order(),
            $rhs.order(),
            "orders must match after match_orders"
        );

        match $lhs {
            Value::UnsignedInt(n) => {
                let rhs = u64::try_from($rhs).expect("orders must match");
                let $n = n;
                $op(rhs)
            }
            Value::UnsignedBigInt(n) => {
                let rhs = u128::try_from($rhs).expect("orders must match");
                let $n = n;
                $op(rhs)
            }
            Value::SignedInt(n) => {
                let rhs = i64::try_from($rhs).expect("orders must match");
                let $n = n;
                $op(rhs)
            }
            Value::SignedBigInt(n) => {
                let rhs = i128::try_from($rhs).expect("orders must match");
                let $n = n;
                $op(rhs)
            }
            Value::Float(n) => {
                let rhs = ::noisy_float::types::N64::try_from($rhs).expect("orders must match");
                let $n = n;
                $op(rhs)
            }
        }
    }};
    (INTS: $lhs:expr, $rhs:expr, $n:ident, $op:expr) => {{
        $lhs.match_orders(&mut $rhs);
        debug_assert_eq!(
            $lhs.order(),
            $rhs.order(),
            "orders must match after match_orders"
        );

        match $lhs {
            Value::UnsignedInt(n) => {
                let rhs = u64::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::UnsignedBigInt(n) => {
                let rhs = u128::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::SignedInt(n) => {
                let rhs = i64::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::SignedBigInt(n) => {
                let rhs = i128::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::Float(_) => Err(Error::ImproperlyFloat),
        }
    }};
}

pub(crate) use dispatch_operation;

/// A numeric value.
///
/// Every calculation is parsed calculated as a common value type.
/// This type can be concretely represented by one of a number of memory formats.
///
/// ## Value Orders
///
/// The **order** of a value is jargon for its in-memory representation. The
/// orders currently available are:
///
/// 1. `u64`
/// 1. `u128`
/// 1. `i64`
/// 1. `i128`
/// 1. `f64`
///
/// Note that in general, lower orders have a narrower scope and higher orders
/// have a broader scope. This enables us to promote values to higher compatible orders
/// as necessary.
///
/// ### Promotion
///
/// It will sometimes be necessary to promote a value. The next order after a promotion
/// depends on the value in question. It follows these rules:
///
/// - `u64` values are unconditionally promoted to `u128` as that conversion is infallible
/// - `u128` values are promoted to the next order in sequence which can represent the type,
///   according to whether or not it fits inside the type bounds.
///
///   I.e. the value `u64::MAX` would be promoted to `i128`, skipping `i64`, as it could
///   not be losslessly converted.
///   The value `i128::MAX + 1` would be promoted to `f64`, _even though this will lose
///   precision_, because `f64` can better approximate that overflow than `i128` could.
/// - `i64` values are unconditionally promoted to `i128` as that conversion is infallible.
/// - `i128` values are unconditionally promoted to `f64` as that type can better approximate
///   very large values.
/// - `f64` values remain `f64`.
///
/// ## Parsing Rules
///
/// When parsing a value, each order is checked in sequence.
/// The first value type which parses without error is used.
///
/// ## Math Rules
///
/// When computing an expression, for each pair of values, this algorithm is applied:
///
/// - the lower-order of the pair is promoted
/// - if the two orders are still not equal, the previous step is repeated
/// - once the two orders are equal, math is performed as normal.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumDiscriminants,
    derive_more::From,
    derive_more::TryInto,
)]
#[strum_discriminants(derive(PartialOrd, Ord))]
#[strum_discriminants(name(Order))]
pub enum Value {
    UnsignedInt(u64),
    UnsignedBigInt(u128),
    SignedInt(i64),
    SignedBigInt(i128),
    Float(N64),
}

pub(crate) type Result<T = Value, E = Error> = std::result::Result<T, E>;

impl Value {
    pub const PI: Self = Self::Float(N64::unchecked_new(f64::consts::PI));
    pub const E: Self = Self::Float(N64::unchecked_new(f64::consts::E));

    pub(crate) fn order(&self) -> Order {
        Order::from(*self)
    }

    pub(crate) fn promote(&mut self) {
        *self = match *self {
            Value::UnsignedInt(n) => Self::UnsignedBigInt(n as _),
            Value::UnsignedBigInt(n) => {
                const SI_MAX: u128 = i64::MAX as _;
                const SBI_MIN: u128 = SI_MAX + 1;
                const SBI_MAX: u128 = i128::MAX as _;

                match n {
                    0..=SI_MAX => Self::SignedInt(n as _),
                    SBI_MIN..=SBI_MAX => Self::SignedBigInt(n as _),
                    _ => Self::Float(
                        n.to_f64()
                            .expect("all u128 convert to f64")
                            .try_into()
                            .expect("all f64 from u128 are numbers"),
                    ),
                }
            }
            Value::SignedInt(n) => Self::SignedBigInt(n as _),
            Value::SignedBigInt(n) => Self::Float(
                n.to_f64()
                    .expect("all i128 convert to f64")
                    .try_into()
                    .expect("all f64 from i128 are numbers"),
            ),
            Value::Float(n) => Self::Float(n),
        }
    }

    pub(crate) fn match_orders(&mut self, other: &mut Self) {
        while self.order() != other.order() {
            match self.order().cmp(&other.order()) {
                Ordering::Equal => unreachable!("orders already known not to be equal"),
                Ordering::Less => self.promote(),
                Ordering::Greater => other.promote(),
            }
        }
    }
}
