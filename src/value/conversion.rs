use crate::Value;

macro_rules! impl_from {
    ($t:ty => $variant:ident) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::$variant(value as _)
            }
        }
    };
}

impl_from!(u8 => UnsignedInt);
impl_from!(u16 => UnsignedInt);
impl_from!(u32 => UnsignedInt);

impl_from!(i8 => SignedInt);
impl_from!(i16 => SignedInt);
impl_from!(i32 => SignedInt);
