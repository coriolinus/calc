use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

/// A trait indicating that this type is suitable for
pub trait Calcable:
    FromStr
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
{
}
